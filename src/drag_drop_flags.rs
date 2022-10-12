#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiDragDropFlags;     // -> enum ImGuiDragDropFlags_   // Flags: for BeginDragDropSource(), AcceptDragDropPayload()
pub type ImGuiDragDropFlags = c_int;

// Flags for BeginDragDropSource(), AcceptDragDropPayload()
// enum ImGuiDragDropFlags_
// {
pub const ImGuiDragDropFlags_None: ImGuiDragDropFlags = 0;
// BeginDragDropSource() flags
pub const ImGuiDragDropFlags_SourceNoPreviewTooltip: ImGuiDragDropFlags = 1 << 0;
// By default, a successful call to BeginDragDropSource opens a tooltip so you can display a preview or description of the source contents. This flag disable this behavior.
pub const ImGuiDragDropFlags_SourceNoDisableHover: ImGuiDragDropFlags = 1 << 1;
// By default, when dragging we clear data so that IsItemHovered() will return false, to avoid subsequent user code submitting tooltips. This flag disable this behavior so you can still call IsItemHovered() on the source item.
pub const ImGuiDragDropFlags_SourceNoHoldToOpenOthers: ImGuiDragDropFlags = 1 << 2;
// Disable the behavior that allows to open tree nodes and collapsing header by holding over them while dragging a source item.
pub const ImGuiDragDropFlags_SourceAllowNullID: ImGuiDragDropFlags = 1 << 3;
// Allow items such as Text(), Image() that have no unique identifier to be used as drag source, by manufacturing a temporary identifier based on their window-relative position. This is extremely unusual within the dear imgui ecosystem and so we made it explicit.
pub const ImGuiDragDropFlags_SourceExtern: ImGuiDragDropFlags = 1 << 4;
// External source (from outside of dear imgui), won't attempt to read current item/window info. Will always return true. Only one Extern source can be active simultaneously.
pub const ImGuiDragDropFlags_SourceAutoExpirePayload: ImGuiDragDropFlags = 1 << 5;
// Automatically expire the payload if the source cease to be submitted (otherwise payloads are persisting while being dragged)
// AcceptDragDropPayload() flags
pub const ImGuiDragDropFlags_AcceptBeforeDelivery: ImGuiDragDropFlags = 1 << 10;
// AcceptDragDropPayload() will returns true even before the mouse button is released. You can then call IsDelivery() to test if the payload needs to be delivered.
pub const ImGuiDragDropFlags_AcceptNoDrawDefaultRect: ImGuiDragDropFlags = 1 << 11;
// Do not draw the default highlight rectangle when hovering over target.
pub const ImGuiDragDropFlags_AcceptNoPreviewTooltip: ImGuiDragDropFlags = 1 << 12;
// Request hiding the BeginDragDropSource tooltip from the BeginDragDropTarget site.
pub const ImGuiDragDropFlags_AcceptPeekOnly: ImGuiDragDropFlags = ImGuiDragDropFlags_AcceptBeforeDelivery | ImGuiDragDropFlags_AcceptNoDrawDefaultRect; // For peeking ahead and inspecting the payload before delivery.
// };
