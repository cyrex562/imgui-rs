use std::collections::HashSet;

// pub const AcceptPeekOnly: i32               = DimgDragDropFlags::AcceptBeforeDelivery | DimgDragDropFlags::AcceptNoDrawDefaultRect;
pub const ACCEPT_PEEK_ONLY: HashSet<DimgDragDropFlags> = HashSet::from([
    DimgDragDropFlags::AcceptBeforeDelivery, DimgDragDropFlags::AcceptNoDrawDefaultRect
]);

// Standard Drag and Drop payload types. You can define you own payload types using short strings. Types starting with '_' are defined by Dear ImGui.
pub const IMGUI_PAYLOAD_TYPE_COLOR_3F: String =     String::from("_COL3F");

// float[3]: Standard type for colors, without alpha. User code may use this type.
pub const IMGUI_PAYLOAD_TYPE_COLOR_4F: String =     String::from("_COL4F");

// flags for ImGui::BeginDragDropSource(), ImGui::AcceptDragDropPayload()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgDragDropFlags
{
    None                         = 0,
    // BeginDragDropSource() flags
    SourceNoPreviewTooltip       = 1 << 0,   // By default, a successful call to BeginDragDropSource opens a tooltip so you can display a preview or description of the source contents. This flag disable this behavior.
    SourceNoDisableHover         = 1 << 1,   // By default, when dragging we clear data so that IsItemHovered() will return false, to avoid subsequent user code submitting tooltips. This flag disable this behavior so you can still call IsItemHovered() on the source item.
    SourceNoHoldToOpenOthers     = 1 << 2,   // Disable the behavior that allows to open tree nodes and collapsing header by holding over them while dragging a source item.
    SourceAllowNullID            = 1 << 3,   // Allow items such as Text(), Image() that have no unique identifier to be used as drag source, by manufacturing a temporary identifier based on their window-relative position. This is extremely unusual within the dear imgui ecosystem and so we made it explicit.
    SourceExtern                 = 1 << 4,   // External source (from outside of dear imgui), won't attempt to read current item/window info. Will always return true. Only one Extern source can be active simultaneously.
    SourceAutoExpirePayload      = 1 << 5,   // Automatically expire the payload if the source cease to be submitted (otherwise payloads are persisting while being dragged)
    // AcceptDragDropPayload() flags
    AcceptBeforeDelivery         = 1 << 10,  // AcceptDragDropPayload() will returns true even before the mouse button is released. You can then call is_delivery() to test if the payload needs to be delivered.
    AcceptNoDrawDefaultRect      = 1 << 11,  // Do not draw the default highlight rectangle when hovering over target.
    AcceptNoPreviewTooltip       = 1 << 12,  // Request hiding the BeginDragDropSource tooltip from the BeginDragDropTarget site.
    // AcceptPeekOnly               = AcceptBeforeDelivery | AcceptNoDrawDefaultRect  // For peeking ahead and inspecting the payload before delivery.
}
