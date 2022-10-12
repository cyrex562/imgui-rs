use libc::c_int;
use crate::table_column_sort_specs::ImGuiTableColumnSortSpecs;

// Sorting specifications for a table (often handling sort specs for a single column, occasionally more)
// Obtained by calling TableGetSortSpecs().
// When 'SpecsDirty == true' you can sort your data. It will be true with sorting specs have changed since last call, or the first time.
// Make sure to set 'SpecsDirty = false' after sorting, else you may wastefully sort your data every frame!
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiTableSortSpecs {
    pub Specs: *const ImGuiTableColumnSortSpecs,
    // Pointer to sort spec array.
    pub SpecsCount: c_int,
    // Sort spec count. Most often 1. May be > 1 when ImGuiTableFlags_SortMulti is enabled. May be == 0 when ImGuiTableFlags_SortTristate is enabled.
    pub SpecsDirty: bool,     // Set to true when specs have changed since last time! Use this to sort again, then clear the flag.
}

impl ImGuiTableSortSpecs {
    //  ImGuiTableSortSpecs()       { memset(this, 0, sizeof(*this)); }
}
