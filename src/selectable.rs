// flags for ImGui::selectable()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgselectableFlags
{
    None               = 0,
    DontClosePopups   ,   // Clicking this don't close parent popup window
    SpanAllColumns    ,   // selectable frame can span all columns (text will still fit in current column)
    AllowDoubleClick  ,   // Generate press events on double clicks too
    Disabled          ,   // Cannot be selected, display grayed out text
    AllowItemOverlap   = 1 << 4    // (WIP) Hit testing to allow subsequent widgets to overlap this one
}
