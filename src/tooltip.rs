pub enum DimgTooltipFlags
{
    None = 0,
    OverridePreviousTooltip = 1 << 0      // Override will clear/ignore previously submitted tooltip (defaults to append)
}
