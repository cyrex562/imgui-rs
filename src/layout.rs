// FIXME: this is in development, not exposed/functional as a generic feature yet.
// Horizontal/Vertical enums are fixed to 0/1 so they may be used to index ImVec2
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImGuiLayoutType
{
    Horizontal,
    Vertical
}
