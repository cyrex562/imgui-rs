use std::collections::HashSet;

// flags for ImGui::TableSetupColumn()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgTableColumnFlags
{
    // Input configuration flags
    None                  = 0,
    Disabled             ,   // Overriding/master disable flag: hide column, won't show in context menu (unlike calling TableSetColumnEnabled() which manipulates the user accessible state)
    DefaultHide          ,   // Default as a hidden/disabled column.
    DefaultSort          ,   // Default as a sorting column.
    WidthStretch         ,   // column will stretch. Preferable with horizontal scrolling disabled (default if table sizing policy is _SizingStretchSame or _SizingStretchProp).
    WidthFixed           ,   // column will not stretch. Preferable with horizontal scrolling enabled (default if table sizing policy is _SizingFixedFit and table is resizable).
    NoResize             ,   // Disable manual resizing.
    NoReorder            ,   // Disable manual reordering this column, this will also prevent other columns from crossing over this column.
    NoHide               ,   // Disable ability to hide/disable this column.
    NoClip               ,   // Disable clipping for this column (all NoClip columns will render in a same draw command).
    NoSort               ,   // Disable ability to sort on this field (even if ImGuiTableFlags_Sortable is set on the table).
    NoSortAscending      ,  // Disable ability to sort in the ascending direction.
    NoSortDescending     ,  // Disable ability to sort in the descending direction.
    NoHeaderLabel        ,  // TableHeadersRow() will not submit label for this column. Convenient for some small columns. name will still appear in context menu.
    NoHeaderWidth        ,  // Disable header text width contribution to automatic column width.
    PreferSortAscending  ,  // Make the initial sort direction Ascending when first sorting on this column (default).
    PreferSortDescending ,  // Make the initial sort direction Descending when first sorting on this column.
    IndentEnable         ,  // Use current Indent value when entering cell (default for column 0).
    IndentDisable        ,  // Ignore current Indent value when entering cell (default for columns > 0). Indentation changes _within_ the cell will still be honored.

    // Output status flags, read-only via TableGetColumnFlags()
    IsEnabled            ,  // Status: is enabled == not hidden by user/api (referred to as "Hide" in _DefaultHide and _NoHide) flags.
    is_visible            ,  // Status: is visible == is enabled AND not clipped by scrolling.
    IsSorted             ,  // Status: is currently part of the sort specs
    IsHovered            ,  // Status: is hovered by mouse

    // [Internal] Combinations and masks
    // ImGuiTableColumnFlags_WidthMask_            = ImGuiTableColumnFlags_WidthStretch | ImGuiTableColumnFlags_WidthFixed,
    // ImGuiTableColumnFlags_IndentMask_           = ImGuiTableColumnFlags_IndentEnable | ImGuiTableColumnFlags_IndentDisable,
    // ImGuiTableColumnFlags_StatusMask_           = ImGuiTableColumnFlags_IsEnabled | ImGuiTableColumnFlags_is_visible | ImGuiTableColumnFlags_IsSorted | ImGuiTableColumnFlags_IsHovered,
    NoDirectResize_       = 1 << 30   // [Internal] Disable user resizing this column directly (it may however we resized indirectly from its left edge)

    // Obsolete names (will be removed soon)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     //ImGuiTableColumnFlags_WidthAuto           = ImGuiTableColumnFlags_WidthFixed | ImGuiTableColumnFlags_NoResize, // column will not stretch and keep resizing based on submitted contents.
// #endif
}


// pub const WidthMask_ : i32           = DimgTableColumnFlags::WidthStretch | DimgTableColumnFlags::WidthFixed;
pub const WIDTH_MASK: HashSet<DimgTableColumnFlags> = HashSet::from([
    DimgTableColumnFlags::WidthStretch, DimgTableColumnFlags::WidthFixed
]);

// pub const     IndentMask_ : i32          = DimgTableColumnFlags::IndentEnable | DimgTableColumnFlags::IndentDisable;
pub const INDENT_MASK: HashSet<DimgTableColumnFlags> = HashSet::from([
    DimgTableColumnFlags::IndentEnable, DimgTableColumnFlags::IndentDisable
]);

pub const STATUS_MASK: HashSet<DimgTableColumnFlags> = HashSet::from([
    DimgTableColumnFlags::IsEnabled, DimgTableColumnFlags::is_visible, DimgTableColumnFlags::IsSorted, DimgTableColumnFlags::IsHovered
]);
