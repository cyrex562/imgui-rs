// A cardinal direction
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DimgDirection {
    None,
    Left,
    Right,
    Up,
    Down,
}

impl Default for DimgDirection {
    fn default() -> Self {
        Self::None
    }
}

// A sorting direction
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgSortDirection
{
    None         = 0,
    Ascending    = 1,    // Ascending = 0->9, A->Z etc.
    Descending   = 2     // Descending = 9->0, Z->A etc.
}

impl Default for DimgSortDirection {
    fn default() -> Self {
        Self::None
    }
}
