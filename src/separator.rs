pub enum DimgSeparatorFlags
{
    None                = 0,
    Horizontal         ,   // Axis default to current layout type, so generally Horizontal unless e.g. in a menu bar
    Vertical           ,
    SpanAllColumns      = 1 << 2
}
