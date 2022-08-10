// Extend
pub enum SliderFlags {
    Vertical,
    // Should this slider be orientated vertically?
    ReadOnly,
    None,
    AlwaysClamp,
    // Clamp value to min/max bounds when input manually with CTRL+Click. By default CTRL+Click allows going out of bounds.
    Logarithmic,
    // Make the widget logarithmic (linear otherwise). Consider using ImGuiSliderFlags_NoRoundToFormat with this if using a format-string with small amount of digits.
    NoRoundToFormat,
    // Disable rounding underlying value to match precision of the display format string (e.g. %.3 values are rounded to those 3 digits)
    NoInput,       // Disable CTRL+Click or Enter key allowing to input text directly into the widget
    // InvalidMask           = 0x7000000F    // [Internal] We treat using those bits as being potentially a 'float power' argument from the previous API that has got miscast to this enum, and will trigger an assert if needed.
}

