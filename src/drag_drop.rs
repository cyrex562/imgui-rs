use std::borrow::BorrowMut;
use std::collections::HashSet;
use crate::color::StyleColor;
use crate::condition::Condition;
use crate::{Context, hash_string, INVALID_ID};
use crate::context::set_active_id_using_nav_and_keys;
use crate::id::set_active_id;
use crate::input::mouse::{is_mouse_dragging, is_mouse_hovering_rect};
use crate::input::MouseButton;
use crate::item::{item_hoverable, ItemStatusFlags};
use crate::orig_imgui_single_file::{Id32, Window};
use crate::payload::Payload;
use crate::rect::Rect;
use crate::style::color_u32_from_style_color;
use crate::types::Id32;
use crate::vectors::vector_2d::Vector2D;
use crate::window::layer::focus_window;
use crate::window::Window;

// pub const AcceptPeekOnly: i32               = DimgDragDropFlags::AcceptBeforeDelivery | DimgDragDropFlags::AcceptNoDrawDefaultRect;
pub const ACCEPT_PEEK_ONLY: HashSet<DragDropFlags> = HashSet::from([
    DragDropFlags::AcceptBeforeDelivery, DragDropFlags::AcceptNoDrawDefaultRect
]);

// Standard Drag and Drop payload types. You can define you own payload types using short strings. Types starting with '_' are defined by Dear ImGui.
pub const IMGUI_PAYLOAD_TYPE_COLOR_3F: String = String::from("_COL3F");

// float[3]: Standard type for colors, without alpha. User code may use this type.
pub const IMGUI_PAYLOAD_TYPE_COLOR_4F: String = String::from("_COL4F");

// flags for ImGui::begin_drag_drop_source(), ImGui::accept_drag_drop_payload()
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DragDropFlags {
    // begin_drag_drop_source() flags
    SourceNoPreviewTooltip,
    // By default, a successful call to begin_drag_drop_source opens a tooltip so you can display a preview or description of the source contents. This flag disable this behavior.
    SourceNoDisableHover,
    // By default, when dragging we clear data so that IsItemHovered() will return false, to avoid subsequent user code submitting tooltips. This flag disable this behavior so you can still call IsItemHovered() on the source item.
    SourceNoHoldToOpenOthers,
    // Disable the behavior that allows to open tree nodes and collapsing header by holding over them while dragging a source item.
    SourceAllowNullID,
    // Allow items such as Text(), Image() that have no unique identifier to be used as drag source, by manufacturing a temporary identifier based on their window-relative position. This is extremely unusual within the dear imgui ecosystem and so we made it explicit.
    SourceExtern,
    // External source (from outside of dear imgui), won't attempt to read current item/window info. Will always return true. Only one Extern source can be active simultaneously.
    SourceAutoExpirePayload,
    // Automatically expire the payload if the source cease to be submitted (otherwise payloads are persisting while being dragged)
    // accept_drag_drop_payload() flags
    AcceptBeforeDelivery,
    // accept_drag_drop_payload() will returns true even before the mouse button is released. You can then call is_delivery() to test if the payload needs to be delivered.
    AcceptNoDrawDefaultRect,
    // Do not draw the default highlight rectangle when hovering over target.
    AcceptNoPreviewTooltip,  // Request hiding the begin_drag_drop_source tooltip from the BeginDragDropTarget site.
    // AcceptPeekOnly               = AcceptBeforeDelivery | AcceptNoDrawDefaultRect  // For peeking ahead and inspecting the payload before delivery.
}

// bool IsDragDropActive()
pub fn is_drag_drop_active(g: &mut Context) -> bool {
    // ImGuiContext& g = *GImGui;
    return g.drag_drop_active;
}

// void clear_drag_drop()
pub fn clear_drag_drop(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    g.drag_drop_active = false;
    g.drag_drop_payload.Clear();
    g.drag_drop_accept_flags = DragDropFlags::None;
    g.drag_drop_accept_id_curr = 0;
    g.drag_drop_accept_id_prev = 0;
    g.drag_drop_accept_id_curr_rect_surface = f32::MAX;
    g.drag_drop_accept_fraame_count = -1;

    g.drag_drop_payload_buf_heap.clear();
    memset(&g.drag_drop_payload_buf_local, 0, sizeof(g.drag_drop_payload_buf_local));
}

// When this returns true you need to: a) call set_drag_drop_payload() exactly once, b) you may render the payload visual/description, c) call end_drag_drop_source()
// If the item has an identifier:
// - This assume/require the item to be activated (typically via ButtonBehavior).
// - Therefore if you want to use this with a mouse button other than left mouse button, it is up to the item itself to activate with another button.
// - We then pull and use the mouse button that was used to activate the item and use it to carry on the drag.
// If the item has no identifier:
// - Currently always assume left mouse button.
// bool begin_drag_drop_source(ImGuiDragDropFlags flags)
pub fn begin_drag_drop_source(g: &mut Context, flags: &HashSet<DragDropFlags>) -> bool {
    // ImGuiContext& g = *GImGui;
    Window * window = g.current_window;

    // FIXME-DRAGDROP: While in the common-most "drag from non-zero active id" case we can tell the mouse button,
    // in both SourceExtern and id==0 cases we may requires something else (explicit flags or some heuristic).
    let mut mouse_button = MouseButton::Left;

    let mut source_drag_active = false;
    let mut source_id = INVALID_ID;
    let mut source_parent_id = INVALID_ID;
    // if (!(flags & DragDropFlags::SourceExtern))
    if flags.contains(&DragDropFlags::SourceExtern) {
        source_id = g.last_item_data.id;
        if source_id != 0 {
            // Common path: items with id
            if g.active_id != source_id {
                return false;
            }
            if g.active_id_mouse_button != -1 {
                mouse_button = g.active_id_mouse_button;
            }
            if g.io.mouse_down[mouse_button] == false || window.skip_items {
                return false;
            }
            g.active_id_allow_overlap = false;
        } else {
            // Uncommon path: items without id
            if g.io.mouse_down[mouse_button] == false || window.skip_items {
                return false;
            }
            if (g.last_item_data.status_flags.contains(&ItemStatusFlags::HoveredRect)) == false && (g.active_id == 0 || g.active_id_window != window.id) {
                return false;
            }

            // If you want to use begin_drag_drop_source() on an item with no unique identifier for interaction, such as Text() or Image(), you need to:
            // A) Read the explanation below, B) Use the DragDropFlags::SourceAllowNullID flag.
            if !(flags.contains(&DragDropFlags::SourceAllowNullID)) {
                // IM_ASSERT(0);
                return false;
            }

            // Magic fallback to handle items with no assigned id, e.g. Text(), Image()
            // We build a throwaway id based on current id stack + relative AABB of items in window.
            // THE IDENTIFIER WON'T SURVIVE ANY REPOSITIONING/RESIZINGG OF THE WIDGET, so if your widget moves your dragging operation will be canceled.
            // We don't need to maintain/call clear_active_id() as releasing the button will early out this function and trigger !active_id_is_alive.
            // Rely on keeping other window->LastItemXXX fields intact.
            g.last_item_data.id = window.get_id_from_rectangle(&g.last_item_data.rect);
            source_id = g.last_item_data.id;
            keep_alive_id(source_id);
            let is_hovered = item_hoverable(g, g.last_item_data.Rect, source_id).unwrap();
            if is_hovered && g.io.mouse_clicked[mouse_button] {
                set_active_id(g, source_id, window);
                focus_window(g, window);
            }
            if g.active_id == source_id { // Allow the underlying widget to display/return hovered during the mouse release frame, else we would get a flicker.
                g.active_id_allow_overlap = is_hovered;
            }
        }
        if g.active_id != source_id {
            return false;
        }
        source_parent_id = window.id_stack.back();
        source_drag_active = is_mouse_dragging(g, mouse_button, 0.0);

        // Disable navigation and key inputs while dragging + cancel existing request if any
        set_active_id_using_nav_and_keys(g);
    } else {
        window = None;
        source_id = hash_string("#SourceExtern", 0);
        source_drag_active = true;
    }

    if source_drag_active {
        if !g.drag_drop_active {
            // IM_ASSERT(source_id != 0);
            clear_drag_drop(g);
            let payload = &mut g.drag_drop_payload;
            payload.source_id = source_id;
            payload.source_parent_id = source_parent_id;
            g.drag_drop_active = true;
            g.drag_drop_source_flags = flags.clone();
            g.drag_drop_mouse_button = mouse_button;
            if payload.source_id == g.active_id {
                g.active_id_no_clear_on_focus_loss = true;
            }
        }
        g.drag_drop_source_frame_count = g.frame_count;
        g.drag_drop_within_source = true;

        if !(flags.contains(&DragDropFlags::SourceNoPreviewTooltip)) {
            // Target can request the Source to not display its tooltip (we use a dedicated flag to make this request explicit)
            // We unfortunately can't just modify the source flags and skip the call to BeginTooltip, as caller may be emitting contents.
            BeginTooltip();
            if g.drag_drop_accept_id_prev != INVALID_ID && (g.drag_drop_accept_flags.contains(&DragDropFlags::AcceptNoPreviewTooltip)) {
                let tooltip_window = g.current_window_mut();
                tooltip_window.hidden = true;
                tooltip_window.skip_items = true;
                tooltip_window.hidden_frames_can_skip_items = 1;
            }
        }

        if !(flags.contains(&DragDropFlags::SourceNoDisableHover)) && !(flags.contains(&DragDropFlags::SourceExtern)) {
            g.last_item_data.status_flags.remove(&ItemStatusFlags::HoveredRect);// &= ~ItemStatusFlags::HoveredRect;
        }

        return true;
    }
    return false;
}

// void end_drag_drop_source()
pub fn end_drag_drop_source(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.drag_drop_active);
    // IM_ASSERT(g.drag_drop_within_source && "Not after a begin_drag_drop_source()?");

    if !(g.drag_drop_source_flags.contains(&DragDropFlags::SourceNoPreviewTooltip)) {
        EndTooltip();
    }

    // Discard the drag if have not called set_drag_drop_payload()
    if g.drag_drop_payload.data_frame_count == -1 {
        clear_drag_drop(g);
    }
    g.drag_drop_within_source = false;
}

// Use 'cond' to choose to submit payload on drag start or every frame
// bool set_drag_drop_payload(const char* type, const void* data, size_t data_size, ImGuiCond cond)
pub fn set_drag_drop_payload(g: &mut Context, payload_type: &str, data: &Window, data_size: usize, mut condition: Condition) -> bool {
    // ImGuiContext& g = *GImGui;
    let payload = &mut g.drag_drop_payload;
    if condition == Condition::None {
        condition = Condition::Always;
    }

    // IM_ASSERT(type != None);
    // IM_ASSERT(strlen(type) < IM_ARRAYSIZE(payload.dataType) && "Payload type can be at most 32 characters long");
    // IM_ASSERT((data != None && data_size > 0) || (data == None && data_size == 0));
    // IM_ASSERT(cond == Condition::Always || cond == ImGuiCond_Once);
    // IM_ASSERT(payload.source_id != 0);                               // Not called between begin_drag_drop_source() and end_drag_drop_source()

    if condition == Condition::Always || payload.data_frame_count == -1 {
        // Copy payload
        // ImStrncpy(payload.dataType, type, IM_ARRAYSIZE(payload.dataType));
        payload.data_type = payload_type.to_string();
        g.drag_drop_payload_buf_heap.resize(0, 0u8);
        if data_size > (g.drag_drop_payload_buf_local.len()) {
            // Store in heap
            g.drag_drop_payload_buf_heap.resize(data_size, 0u8);
            payload.data = g.drag_drop_payload_buf_heap.data;
            // memcpy(payload.data, data, data_size);
            payload.data = data.clone();
        } else if data_size > 0 {
            // Store locally
            // memset(&g.drag_drop_payload_buf_local, 0, sizeof(g.drag_drop_payload_buf_local));
            g.drag_drop_payload_buf_local.fill(0);
            payload.data = g.drag_drop_payload_buf_local.clone();
            // memcpy(payload.data, data, data_size);
            payload.data = data.clone();
        } else {
            payload.data = None;
        }
        payload.dataSize = data_size;
    }
    payload.data_frame_count = g.frame_count;

    // Return whether the payload has been accepted
    return (g.drag_drop_accept_fraame_count == g.frame_count) || (g.drag_drop_accept_fraame_count == g.frame_count - 1);
}

// bool begin_drag_drop_target_custom(const Rect& bb, Id32 id)
pub fn begin_drag_drop_target_custom(g: &mut Context, bb: &Rect, id: Id32) -> bool {
    // ImGuiContext& g = *GImGui;
    if !g.drag_drop_active {
        return false;
    }

    // Window* window = g.current_window;
    let window = g.current_window_mut();
    // Window* hovered_window = g.hovered_window_under_moving_window;
    let hovered_window = g.hovered_window_under_moving_window_mut();
    // if (hovered_window == None || window.root_window_dock_tree != hovered_window.root_window_dock_tree)
    if window.root_window_dock_tree_id != hovered_window.root_window_dock_tree_id {
        return false;
    }
    // IM_ASSERT(id != 0);
    if !is_mouse_hovering_rect(g, &bb.min, &bb.max, false) || (id == g.drag_drop_payload.source_id) {
        return false;
    }
    if window.skip_items {
        return false;
    }

    // IM_ASSERT(g.drag_drop_within_target == false);
    g.drag_drop_target_rect = bb.clone();
    g.drag_drop_target_id = id;
    g.drag_drop_within_target = true;
    return true;
}

// We don't use begin_drag_drop_target_custom() and duplicate its code because:
// 1) we use LastItemRectHoveredRect which handles items that pushes a temporarily clip rectangle in their code. Calling begin_drag_drop_target_custom(LastItemRect) would not handle them.
// 2) and it's faster. as this code may be very frequently called, we want to early out as fast as we can.
// Also note how the hovered_window test is positioned differently in both functions (in both functions we optimize for the cheapest early out case)
// bool BeginDragDropTarget()
pub fn begin_drag_drop_target(g: &mut Context) -> bool {
    // ImGuiContext& g = *GImGui;
    if !g.drag_drop_active {
        return false;
    }

    // Window* window = g.current_window;
    let window = g.current_window_mut();
    if !(g.last_item_data.status_flags.contains(&ItemStatusFlags::HoveredRect)) {
        return false;
    }
    // Window* hovered_window = g.hovered_window_under_moving_window;
    let hovered_window = g.hovered_window_under_moving_window_mut();
    if window.root_window_dock_tree_id != hovered_window.root_window_dock_tree_id || window.skip_items {
        return false;
    }

    let display_rect = if g.last_item_data.status_flags.contains(&ItemStatusFlags::HasDisplayRect) { g.last_item_data.display_rect.clone() } else { g.last_item_data.rect.clone() };
    let mut id = g.last_item_data.id;
    if id == 0 {
        id = window.get_id_from_rectangle(display_rect);
        keep_alive_id(id);
    }
    if g.drag_drop_payload.source_id == id {
        return false;
    }

    // IM_ASSERT(g.drag_drop_within_target == false);
    g.drag_drop_target_rect = display_rect.clone();
    g.drag_drop_target_id = id;
    g.drag_drop_within_target = true;
    return true;
}

// bool is_drag_drop_payload_being_accepted()
pub fn is_drag_drop_payload_being_accepted(g: &mut Context) -> bool {
    // ImGuiContext& g = *GImGui;
    return g.drag_drop_active && g.drag_drop_accept_id_prev != 0;
}

// const ImGuiPayload* accept_drag_drop_payload(const char* type, ImGuiDragDropFlags flags)
pub fn accept_drag_drop_payload(g: &mut Context, payload_type: &str, flags: &mut HashSet<DragDropFlags>) -> Option<&mut Payload> {
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    // ImGuiPayload& payload = g.drag_drop_payload;
    let payload = &mut g.drag_drop_payload;
    // IM_ASSERT(g.drag_drop_active);                        // Not called between BeginDragDropTarget() and end_drag_drop_target() ?
    // IM_ASSERT(payload.data_frame_count != -1);            // Forgot to call end_drag_drop_target() ?
    if payload_type != None && !payload.is_data_type(payload_type) {
        return None;
    }

    // Accept smallest drag target bounding box, this allows us to nest drag targets conveniently without ordering constraints.
    // NB: We currently accept None id as target. However, overlapping targets requires a unique id to function!
    let was_accepted_previously = (g.drag_drop_accept_id_prev == g.drag_drop_target_id);
    let r = &mut g.drag_drop_target_rect;
    let r_surface = r.width() * r.height();
    if r_surface <= g.drag_drop_accept_id_curr_rect_surface {
        g.drag_drop_accept_flags = flags.clone();
        g.drag_drop_accept_id_curr = g.drag_drop_target_id;
        g.drag_drop_accept_id_curr_rect_surface = r_surface;
    }

    // Render default drop visuals
    // FIXME-DRAGDROP: Settle on a proper default visuals for drop target.
    payload.preview = was_accepted_previously;
    *flags |= (g.drag_drop_source_flags.clone() & DragDropFlags::AcceptNoDrawDefaultRect); // Source can also inhibit the preview (useful for external sources that lives for 1 frame)
    if !(flags.contains(&DragDropFlags::AcceptNoDrawDefaultRect)) && payload.preview {
        window.draw_list.add_rect(&r.min - Vector2D::new(3.5, 3.5), &r.max + Vector2D::new(3.5, 3.5), color_u32_from_style_color(StyleColor::DragDropTarget, 0.0), 0.0, 0, 2.0);
    }

    g.drag_drop_accept_fraame_count = g.frame_count;
    payload.Delivery = was_accepted_previously && !IsMouseDown(g.drag_drop_mouse_button); // For extern drag sources affecting os window focus, it's easier to just test !IsMouseDown() instead of IsMouseReleased()
    if !payload.delivery && !(flags.contains(&DragDropFlags::AcceptBeforeDelivery)) {
        return None;
    }

    return Some(payload);
}

// const ImGuiPayload* GetDragDropPayload()
pub fn get_drag_drop_payload(g: &mut Context) -> Option<&mut Payload> {
    // ImGuiContext& g = *GImGui;
    // return g.drag_drop_active ? &g.drag_drop_payload : None;
    if g.drag_drop_active {
        Some(g.drag_drop_payload.borrow_mut())
    } else {
        None
    }
}

// We don't really use/need this now, but added it for the sake of consistency and because we might need it later.
// void end_drag_drop_target()
pub fn end_drag_drop_target(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.drag_drop_active);
    // IM_ASSERT(g.drag_drop_within_target);
    g.drag_drop_within_target = false;
}
