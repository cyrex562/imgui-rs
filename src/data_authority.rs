// Store the source authority (dock node vs window) of a field
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DataAuthority
{
    None,
    Auto,
    DockNode,
    Window
}

impl Default for DataAuthority {
    fn default() -> Self {
        Self::None
    }
}
