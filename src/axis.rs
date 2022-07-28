// x/Y enums are fixed to 0/1 so they may be used to index Vector2D
#[derive(Debug,Clone,Copy,Eq, PartialEq,Hash)]
pub enum Axis
{
    None,
    X,
    Y
}

impl Default for Axis {
    fn default() -> Self {
        Self::None
    }
}
