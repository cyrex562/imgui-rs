// flags for ImGui::TableNextRow()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum TableRowFlags
{
    None                         = 0,
    Headers                      = 1 << 0    // Identify header row (set default background color + width of its contents accounted differently for auto column width)
}
