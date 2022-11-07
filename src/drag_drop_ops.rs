use crate::color::ImGuiCol_DragDropTarget;
use crate::condition::{ImGuiCond, ImGuiCond_Always};
use crate::data_type::ImGuiPayloadType;
use crate::drag_drop_flags::{
    ImGuiDragDropFlags, ImGuiDragDropFlags_AcceptBeforeDelivery,
    ImGuiDragDropFlags_AcceptNoDrawDefaultRect, ImGuiDragDropFlags_AcceptNoPreviewTooltip,
    ImGuiDragDropFlags_None, ImGuiDragDropFlags_SourceAllowNullID, ImGuiDragDropFlags_SourceExtern,
    ImGuiDragDropFlags_SourceNoDisableHover, ImGuiDragDropFlags_SourceNoPreviewTooltip,
};
use crate::id_ops::{KeepAliveID, SetActiveID};
use crate::input_ops::{IsMouseDown, IsMouseDragging, IsMouseHoveringRect};
use crate::item_ops::ItemHoverable;
use crate::item_status_flags::{
    ImGuiItemStatusFlags_HasDisplayRect, ImGuiItemStatusFlags_HoveredRect,
};
use crate::mouse_button::{ImGuiMouseButton, ImGuiMouseButton_Left};
use crate::payload::ImGuiPayload;
use crate::rect::ImRect;
use crate::string_ops::{str_to_const_c_char_ptr, ImStrncpy};
use crate::style_ops::GetColorU32;
use crate::tooltip_ops::{BeginTooltip, EndTooltip};
use crate::type_defs::ImguiHandle;
use crate::utils::flag_clear;
use crate::vec2::ImVec2;
use crate::window::focus::FocusWindow;
use crate::window::ImguiWindow;
use crate::{hash_string, GImGui};
use libc::{c_float, c_int, c_void, memcpy, memset, size_t};
use std::ptr::null_mut;

pub unsafe fn IsDragDropActive() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.DragDropActive;
}

pub unsafe fn ClearDragDrop() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.DragDropActive = false;
    g.DragDropPayload.Clear();
    g.DragDropAcceptFlags = ImGuiDragDropFlags_None;
    g.DragDropAcceptIdCurr = 0;
    g.DragDropAcceptIdPrev = 0;
    g.DragDropAcceptIdCurrRectSurface = f32::MAX;
    g.DragDropAcceptFrameCount = -1;

    g.DragDropPayloadBufHeap.clear();
    // TODO:
    // libc::memset(&mut g.DragDropPayloadBufLocal, 0, (g.DragDropPayloadBufLocal.len()));
}

// When this returns true you need to: a) call SetDragDropPayload() exactly once, b) you may render the payload visual/description, c) call EndDragDropSource()
// If the item has an identifier:
// - This assume/require the item to be activated (typically via ButtonBehavior).
// - Therefore if you want to use this with a mouse button other than left mouse button, it is up to the item itself to activate with another button.
// - We then pull and use the mouse button that was used to activate the item and use it to carry on the drag.
// If the item has no identifier:
// - Currently always assume left mouse button.
pub unsafe fn BeginDragDropSource(flags: ImGuiDragDropFlags) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();

    // FIXME-DRAGDROP: While in the common-most "drag from non-zero active id" case we can tell the mouse button,
    // in both SourceExtern and id==0 cases we may requires something else (explicit flags or some heuristic).
    let mut mouse_button: ImGuiMouseButton = ImGuiMouseButton_Left;

    let mut source_drag_active: bool = false;
    let mut source_id: ImguiHandle = 0;
    let mut source_parent_id: ImguiHandle = 0;
    if flag_clear(flags, ImGuiDragDropFlags_SourceExtern) {
        source_id = g.LastItemData.ID;
        if (source_id != 0) {
            // Common path: items with ID
            if (g.ActiveId != source_id) {
                return false;
            }
            if (g.ActiveIdMouseButton != -1) {
                mouse_button = g.ActiveIdMouseButton;
            }
            if (g.IO.MouseDown[mouse_button] == false || window.skip_items) {
                return false;
            }
            g.ActiveIdAllowOverlap = false;
        } else {
            // Uncommon path: items without ID
            if (g.IO.MouseDown[mouse_button] == false || window.skip_items) {
                return false;
            }
            if ((g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HoveredRect) == 0
                && (g.ActiveId == 0 || g.ActiveIdWindow != window))
            {
                return false;
            }

            // If you want to use BeginDragDropSource() on an item with no unique identifier for interaction, such as Text() or Image(), you need to:
            // A) Read the explanation below, B) Use the ImGuiDragDropFlags_SourceAllowNullID flag.
            if (flag_clear(flags, ImGuiDragDropFlags_SourceAllowNullID)) {
                // IM_ASSERT(0);
                return false;
            }

            // Magic fallback to handle items with no assigned ID, e.g. Text(), Image()
            // We build a throwaway ID based on current ID stack + relative AABB of items in window.
            // THE IDENTIFIER WON'T SURVIVE ANY REPOSITIONING/RESIZINGG OF THE WIDGET, so if your widget moves your dragging operation will be canceled.
            // We don't need to maintain/call ClearActiveID() as releasing the button will early out this function and trigger !ActiveIdIsAlive.
            // Rely on keeping other window.LastItemXXX fields intact.
            source_id = window.GetIDFromRectangle(&g.LastItemData.Rect);
            g.LastItemData.ID = window.GetIDFromRectangle(&g.LastItemData.Rect);
            KeepAliveID(g, source_id);
            let mut is_hovered: bool = ItemHoverable(&g.LastItemData.Rect, source_id);
            if (is_hovered && g.IO.MouseClicked[mouse_button]) {
                SetActiveID(g, source_id, window);
                FocusWindow(window);
            }
            if (g.ActiveId == source_id) {
                // Allow the underlying widget to display/return hovered during the mouse release frame, else we would get a flicker.
                g.ActiveIdAllowOverlap = is_hovered;
            }
        }
        if (g.ActiveId != source_id) {
            return false;
        }
        source_parent_id = window.IDStack.last().unwrap().clone();
        source_drag_active = IsMouseDragging(mouse_button, 0.0);

        // Disable navigation and key inputs while dragging + cancel existing request if any
        SetActiveIdUsingAllKeyboardKeys();
    } else {
        window = None;
        source_id = hash_string("#SourceExtern", 0);
        source_drag_active = true;
    }

    if source_drag_active {
        if !g.DragDropActive {
            // IM_ASSERT(source_id != 0);
            ClearDragDrop();
            let mut payload = &mut g.DragDropPayload;
            payload.SourceId = source_id;
            payload.SourceParentId = source_parent_id;
            g.DragDropActive = true;
            g.DragDropSourceFlags = flags;
            g.DragDropMouseButton = mouse_button;
            if payload.SourceId == g.ActiveId {
                g.ActiveIdNoClearOnFocusLoss = true;
            }
        }
        g.DragDropSourceFrameCount = g.FrameCount;
        g.DragDropWithinSource = true;

        if (flag_clear(flags, ImGuiDragDropFlags_SourceNoPreviewTooltip)) {
            // Target can request the Source to not display its tooltip (we use a dedicated flag to make this request explicit)
            // We unfortunately can't just modify the source flags and skip the call to BeginTooltip, as caller may be emitting contents.
            BeginTooltip();
            if (g.DragDropAcceptIdPrev
                && (g.DragDropAcceptFlags & ImGuiDragDropFlags_AcceptNoPreviewTooltip))
            {
                let mut tooltip_window: &mut ImguiWindow = g.CurrentWindow;
                tooltip_window.Hidden = true;
                tooltip_window.skip_items = true;
                tooltip_window.HiddenFramesCanSkipItems = 1;
            }
        }

        if (flag_clear(flags, ImGuiDragDropFlags_SourceNoDisableHover)
            && flag_clear(flags, ImGuiDragDropFlags_SourceExtern))
        {
            g.LastItemData.StatusFlags &= !ImGuiItemStatusFlags_HoveredRect;
        }

        return true;
    }
    return false;
}

pub unsafe fn EndDragDropSource() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
                    // IM_ASSERT(g.DragDropActive);
                    // IM_ASSERT(g.DragDropWithinSource && "Not after a BeginDragDropSource()?");

    if flag_clear(
        g.DragDropSourceFlags,
        ImGuiDragDropFlags_SourceNoPreviewTooltip,
    ) {
        EndTooltip();
    }

    // Discard the drag if have not called SetDragDropPayload()
    if g.DragDropPayload.DataFrameCount == -1 {
        ClearDragDrop();
    }
    g.DragDropWithinSource = false;
}

// Use 'cond' to choose to submit payload on drag start or every frame
pub unsafe fn SetDragDropPayload(
    payload_type: ImGuiPayloadType,
    data: &Vec<u8>,
    mut cond: ImGuiCond,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let payload = &mut g.DragDropPayload;
    if cond == 0 {
        cond = ImGuiCond_Always;
    }

    // IM_ASSERT(type != NULL);
    // IM_ASSERT(strlen(type) < IM_ARRAYSIZE(payload.DataType) && "Payload type can be at most 32 characters long");
    // IM_ASSERT((data != NULL && data_size > 0) || (data == NULL && data_size == 0));
    // IM_ASSERT(cond == ImGuiCond_Always || cond == ImGuiCond_Once);
    // IM_ASSERT(payload.SourceId != 0);                               // Not called between BeginDragDropSource() and EndDragDropSource()

    if cond == ImGuiCond_Always || payload.DataFrameCount == -1 {
        // Copy payload
        // ImStrncpy(payload.DataType.as_mut_ptr(), payload_type, payload.DataType.len());
        g.DragDropPayloadBufHeap.clear();
        if data.len() > g.DragDropPayloadBufLocal.len() {
            // Store in heap
            g.DragDropPayloadBufHeap.resize(data.len());
            payload.Data = g.DragDropPayloadBufHeap.Data;
            // libc::memcpy(payload.Data, data, data_size);
        } else if data_size > 0 {
            // Store locally
            // libc::memset(&mut g.DragDropPayloadBufLocal, 0, sizeof(g.DragDropPayloadBufLocal));
            payload.Data = g.DragDropPayloadBufLocal.clone();
            // libc::memcpy(payload.Data, data, data_size);
        } else {
            payload.Data.clear()
        }
        payload.DataSize = payload.Data.len()
    }
    payload.DataFrameCount = g.FrameCount;

    // Return whether the payload has been accepted
    return (g.DragDropAcceptFrameCount == g.FrameCount)
        || (g.DragDropAcceptFrameCount == g.FrameCount - 1);
}

pub unsafe fn BeginDragDropTargetCustom(bb: &ImRect, id: ImguiHandle) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if !g.DragDropActive {
        return false;
    }

    let mut window = g.current_window_mut().unwrap();
    let mut hovered_window: &mut ImguiWindow = g.HoveredWindowUnderMovingWindow;
    if hovered_window == None || window.RootWindowDockTree != hovered_window.RootWindowDockTree {
        return false;
    }
    // IM_ASSERT(id != 0);
    if !IsMouseHoveringRect(&bb.min, &bb.max, false) || (id == g.DragDropPayload.SourceId) {
        return false;
    }
    if window.skip_items {
        return false;
    }

    // IM_ASSERT(g.DragDropWithinTarget == false);
    g.DragDropTargetRect = bb.clone();
    g.DragDropTargetId = id;
    g.DragDropWithinTarget = true;
    return true;
}

// We don't use BeginDragDropTargetCustom() and duplicate its code because:
// 1) we use LastItemRectHoveredRect which handles items that pushes a temporarily clip rectangle in their code. Calling BeginDragDropTargetCustom(LastItemRect) would not handle them.
// 2) and it's faster. as this code may be very frequently called, we want to early out as fast as we can.
// Also note how the HoveredWindow test is positioned differently in both functions (in both functions we optimize for the cheapest early out case)
pub unsafe fn BeginDragDropTarget() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.DragDropActive) {
        return false;
    }

    let mut window = g.current_window_mut().unwrap();
    if (!(g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HoveredRect)) {
        return false;
    }
    let mut hovered_window: &mut ImguiWindow = g.HoveredWindowUnderMovingWindow;
    if (hovered_window == None
        || window.RootWindowDockTree != hovered_window.RootWindowDockTree
        || window.skip_items)
    {
        return false;
    }

    let display_rect: &ImRect = if g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HasDisplayRect
    {
        &g.LastItemData.DisplayRect
    } else {
        &g.LastItemData.Rect
    };
    let mut id: ImguiHandle = g.LastItemData.ID;
    if (id == 0) {
        id = window.GetIDFromRectangle(display_rect);
        KeepAliveID(g, id);
    }
    if (g.DragDropPayload.SourceId == id) {
        return false;
    }

    // IM_ASSERT(g.DragDropWithinTarget == false);
    g.DragDropTargetRect = display_rect.clone();
    g.DragDropTargetId = id;
    g.DragDropWithinTarget = true;
    return true;
}

pub unsafe fn IsDragDropPayloadBeingAccepted() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.DragDropActive && g.DragDropAcceptIdPrev != 0;
}

pub unsafe fn AcceptDragDropPayload(
    payload_type: &str,
    mut flags: ImGuiDragDropFlags,
) -> *const ImGuiPayload {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    let payload = &mut g.DragDropPayload;
    // IM_ASSERT(g.DragDropActive);                        // Not called between BeginDragDropTarget() and EndDragDropTarget() ?
    // IM_ASSERT(payload.DataFrameCount != -1);            // Forgot to call EndDragDropTarget() ?
    if payload_type != None && !payload.IsDataType(payload_type) {
        return None;
    }

    // Accept smallest drag target bounding box, this allows us to nest drag targets conveniently without ordering constraints.
    // NB: We currently accept NULL id as target. However, overlapping targets requires a unique ID to function!
    let was_accepted_previously: bool = (g.DragDropAcceptIdPrev == g.DragDropTargetId);
    let mut r = g.DragDropTargetRect;
    let r_surface: c_float = r.GetWidth() * r.GetHeight();
    if r_surface <= g.DragDropAcceptIdCurrRectSurface {
        g.DragDropAcceptFlags = flags;
        g.DragDropAcceptIdCurr = g.DragDropTargetId;
        g.DragDropAcceptIdCurrRectSurface = r_surface;
    }

    // Render default drop visuals
    // FIXME-DRAGDROP: Settle on a proper default visuals for drop target.
    payload.Preview = was_accepted_previously;
    flags |= (g.DragDropSourceFlags & ImGuiDragDropFlags_AcceptNoDrawDefaultRect); // Source can also inhibit the preview (useful for external sources that lives for 1 frame)
    if flag_clear(flags, ImGuiDragDropFlags_AcceptNoDrawDefaultRect) && payload.Preview {
        window.DrawList.AddRect(
            r.Min - ImVec2::from_floats(3.5, 3.5),
            r.Max + ImVec2::from_floats(3.5, 3.5),
            GetColorU32(ImGuiCol_DragDropTarget, 0.0),
            0.0,
        );
    }
    g.DragDropAcceptFrameCount = g.FrameCount;
    payload.Delivery = was_accepted_previously && !IsMouseDown(g.DragDropMouseButton); // For extern drag sources affecting os window focus, it's easier to just test !IsMouseDown() instead of IsMouseReleased()
    if !payload.Delivery && flag_clear(flags, ImGuiDragDropFlags_AcceptBeforeDelivery) {
        return None;
    }

    return payload;
}

pub unsafe fn GetDragDropPayload() -> *const ImGuiPayload {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return if g.DragDropActive {
        &g.DragDropPayload
    } else {
        None
    };
}

// We don't really use/need this now, but added it for the sake of consistency and because we might need it later.
pub unsafe fn EndDragDropTarget() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
                    // IM_ASSERT(g.DragDropActive);
                    // IM_ASSERT(g.DragDropWithinTarget);
    g.DragDropWithinTarget = false;
}
