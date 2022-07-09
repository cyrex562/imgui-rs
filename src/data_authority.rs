// Store the source authority (dock node vs window) of a field
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgDataAuthority
{
    None,
    Auto,
    DockNode,
    Window
}

impl Default for DimgDataAuthority {
    fn default() -> Self {
        Self::None
    }
}
