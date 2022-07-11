// x/Y enums are fixed to 0/1 so they may be used to index Vector2D
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgAxis
{
    None = -1,
    X = 0,
    Y = 1
}

impl Default for DimgAxis {
    fn default() -> Self {
        Self::None
    }
}
