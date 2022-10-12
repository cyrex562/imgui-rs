#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiColorEditFlags;    // -> enum ImGuiColorEditFlags_  // Flags: for ColorEdit4(), ColorPicker4() etc.
pub type ImGuiColorEditFlags = c_int;


// Flags for ColorEdit3() / ColorEdit4() / ColorPicker3() / ColorPicker4() / ColorButton()
// enum ImGuiColorEditFlags_
// {
pub const ImGuiColorEditFlags_None: ImGuiColorEditFlags            = 0;
pub const ImGuiColorEditFlags_NoAlpha: ImGuiColorEditFlags         = 1 << 1;   //              // ColorEdit, ColorPicker, ColorButton: ignore Alpha component (will only read 3 components from the input pointer).
pub const ImGuiColorEditFlags_NoPicker: ImGuiColorEditFlags        = 1 << 2;   //              // ColorEdit: disable picker when clicking on color square.
pub const ImGuiColorEditFlags_NoOptions: ImGuiColorEditFlags       = 1 << 3;   //              // ColorEdit: disable toggling options menu when right-clicking on inputs/small preview.
pub const ImGuiColorEditFlags_NoSmallPreview: ImGuiColorEditFlags  = 1 << 4;   //              // ColorEdit, ColorPicker: disable color square preview next to the inputs. (e.g. to show only the inputs)
pub const ImGuiColorEditFlags_NoInputs: ImGuiColorEditFlags        = 1 << 5;   //              // ColorEdit, ColorPicker: disable inputs sliders/text widgets (e.g. to show only the small preview color square).
pub const ImGuiColorEditFlags_NoTooltip: ImGuiColorEditFlags       = 1 << 6;   //              // ColorEdit, ColorPicker, ColorButton: disable tooltip when hovering the preview.
pub const ImGuiColorEditFlags_NoLabel: ImGuiColorEditFlags         = 1 << 7;   //              // ColorEdit, ColorPicker: disable display of inline text label (the label is still forwarded to the tooltip and picker).
pub const ImGuiColorEditFlags_NoSidePreview: ImGuiColorEditFlags   = 1 << 8;   //              // ColorPicker: disable bigger color preview on right side of the picker, use small color square preview instead.
pub const ImGuiColorEditFlags_NoDragDrop: ImGuiColorEditFlags      = 1 << 9;   //              // ColorEdit: disable drag and drop target. ColorButton: disable drag and drop source.
pub const ImGuiColorEditFlags_NoBorder: ImGuiColorEditFlags        = 1 << 10;  //              // ColorButton: disable border (which is enforced by default)

    // User Options (right-click on widget to change some of them).
pub const ImGuiColorEditFlags_AlphaBar: ImGuiColorEditFlags        = 1 << 16;  //              // ColorEdit, ColorPicker: show vertical alpha bar/gradient in picker.
pub const ImGuiColorEditFlags_AlphaPreview: ImGuiColorEditFlags    = 1 << 17;  //              // ColorEdit, ColorPicker, ColorButton: display preview as a transparent color over a checkerboard, instead of opaque.
pub const ImGuiColorEditFlags_AlphaPreviewHalf: ImGuiColorEditFlags= 1 << 18;  //              // ColorEdit, ColorPicker, ColorButton: display half opaque / half checkerboard, instead of opaque.
pub const ImGuiColorEditFlags_HDR: ImGuiColorEditFlags             = 1 << 19;  //              // (WIP) ColorEdit: Currently only disable 0.0..1.0 limits in RGBA edition (note: you probably want to use ImGuiColorEditFlags_Float flag as well).
pub const ImGuiColorEditFlags_DisplayRGB: ImGuiColorEditFlags      = 1 << 20;  // [Display]    // ColorEdit: override _display_ type among RGB/HSV/Hex. ColorPicker: select any combination using one or more of RGB/HSV/Hex.
pub const ImGuiColorEditFlags_DisplayHSV: ImGuiColorEditFlags      = 1 << 21;  // [Display]    // "
pub const ImGuiColorEditFlags_DisplayHex: ImGuiColorEditFlags      = 1 << 22;  // [Display]    // "
pub const ImGuiColorEditFlags_Uint8: ImGuiColorEditFlags           = 1 << 23;  // [DataType]   // ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0..255.
pub const ImGuiColorEditFlags_Float: ImGuiColorEditFlags           = 1 << 24;  // [DataType]   // ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0.0..1.0 floats instead of 0..255 integers. No round-trip of value via integers.
pub const ImGuiColorEditFlags_PickerHueBar: ImGuiColorEditFlags    = 1 << 25;  // [Picker]     // ColorPicker: bar for Hue, rectangle for Sat/Value.
pub const ImGuiColorEditFlags_PickerHueWheel: ImGuiColorEditFlags  = 1 << 26;  // [Picker]     // ColorPicker: wheel for Hue, triangle for Sat/Value.
pub const ImGuiColorEditFlags_InputRGB: ImGuiColorEditFlags        = 1 << 27;  // [Input]      // ColorEdit, ColorPicker: input and output data in RGB format.
pub const ImGuiColorEditFlags_InputHSV: ImGuiColorEditFlags        = 1 << 28;  // [Input]      // ColorEdit, ColorPicker: input and output data in HSV format.

    // Defaults Options. You can set application defaults using SetColorEditOptions(). The intent is that you probably don't want to
    // override them in most of your calls. Let the user choose via the option menu and/or call SetColorEditOptions() once during startup.
pub const ImGuiColorEditFlags_DefaultOptions_: ImGuiColorEditFlags = ImGuiColorEditFlags_Uint8 | ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_InputRGB | ImGuiColorEditFlags_PickerHueBar;

    // [Internal] Masks
pub const ImGuiColorEditFlags_DisplayMask_: ImGuiColorEditFlags    = ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_DisplayHSV | ImGuiColorEditFlags_DisplayHex;
pub const ImGuiColorEditFlags_DataTypeMask_: ImGuiColorEditFlags   = ImGuiColorEditFlags_Uint8 | ImGuiColorEditFlags_Float;
pub const ImGuiColorEditFlags_PickerMask_: ImGuiColorEditFlags     = ImGuiColorEditFlags_PickerHueWheel | ImGuiColorEditFlags_PickerHueBar;
pub const ImGuiColorEditFlags_InputMask_: ImGuiColorEditFlags      = ImGuiColorEditFlags_InputRGB | ImGuiColorEditFlags_InputHSV;

    // Obsolete names (will be removed)
    // ImGuiColorEditFlags_RGB = ImGuiColorEditFlags_DisplayRGB, ImGuiColorEditFlags_HSV = ImGuiColorEditFlags_DisplayHSV, ImGuiColorEditFlags_HEX = ImGuiColorEditFlags_DisplayHex  // [renamed in 1.69]
// };
