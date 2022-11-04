use crate::color::ImGuiCol_PopupBg;
use crate::string_ops::ImFormatString;
use crate::tooltip_flags::{
    ImGuiTooltipFlags, ImGuiTooltipFlags_None, ImGuiTooltipFlags_OverridePreviousTooltip,
};
use crate::utils::is_not_null;
use crate::vec2::ImVec2;
use crate::window::find::FindWindowByName;
use crate::window::ops::{Begin, End};
use crate::window::props::{SetNextWindowBgAlpha, SetNextWindowPos};
use crate::window::window_flags::{
    ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_NoDocking,
    ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoResize,
    ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_None,
    ImGuiWindowFlags_Tooltip,
};
use crate::window::ImGuiWindow;
use crate::GImGui;
use libc::c_char;
use std::ptr::null_mut;

pub unsafe fn BeginTooltip() {
    BeginTooltipEx(ImGuiTooltipFlags_None, ImGuiWindowFlags_None);
}

pub unsafe fn BeginTooltipEx(
    mut tooltip_flags: ImGuiTooltipFlags,
    extra_window_flags: ImGuiWindowFlags,
) {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    if g.DragDropWithinSource || g.DragDropWithinTarget {
        // The default tooltip position is a little offset to give space to see the context menu (it's also clamped within the current viewport/monitor)
        // In the context of a dragging tooltip we try to reduce that offset and we enforce following the cursor.
        // Whatever we do we want to call SetNextWindowPos() to enforce a tooltip position and disable clipping the tooltip without our display area, like regular tooltip do.
        //let mut tooltip_pos: ImVec2 =  g.IO.MousePos - g.ActiveIdClickOffset - g.Style.WindowPadding;
        let tooltip_pos: ImVec2 = g.IO.MousePos
            + ImVec2::from_floats(16 * g.Style.MouseCursorScale, 8 * g.Style.MouseCursorScale);
        SetNextWindowPos(&tooltip_pos, 0, &Default::default());
        SetNextWindowBgAlpha(g.Style.Colors[ImGuiCol_PopupBg].w * 0.60);
        //PushStyleVar(ImGuiStyleVar_Alpha, g.Style.Alpha * 0.60); // This would be nice but e.g ColorButton with checkboard has issue with transparent colors :(
        tooltip_flags |= ImGuiTooltipFlags_OverridePreviousTooltip;
    }

    window_name: [c_char; 16];
    // ImFormatString(window_name, window_name.len(), "##Tooltip_{}", g.TooltipOverrideCount);
    if (tooltip_flags & ImGuiTooltipFlags_OverridePreviousTooltip) {
        let mut window: *mut ImGuiWindow = FindWindowByName(window_name);
        if (is_not_null(window)) {
            if window.Active {
                // Hide previous tooltip from being displayed. We can't easily "reset" the content of a window so we create a new one.
                window.Hidden = true;
                window.HiddenFramesCanSkipItems = 1; // FIXME: This may not be necessary?
                                                     // ImFormatString(window_name, window_name.len(), "##Tooltip_{}", + + g.TooltipOverrideCount);
            }
        }
    }
    flags: ImGuiWindowFlags = ImGuiWindowFlags_Tooltip
        | ImGuiWindowFlags_NoInputs
        | ImGuiWindowFlags_NoTitleBar
        | ImGuiWindowFlags_NoMove
        | ImGuiWindowFlags_NoResize
        | ImGuiWindowFlags_NoSavedSettings
        | ImGuiWindowFlags_AlwaysAutoResize
        | ImGuiWindowFlags_NoDocking;
    Begin(window_name, null_mut());
}

pub unsafe fn EndTooltip() {
    // IM_ASSERT(GetCurrentWindowRead()->Flags & ImGuiWindowFlags_Tooltip);   // Mismatched BeginTooltip()/EndTooltip() calls
    End();
}

// pub unsafe fn SetTooltipV(fmt: *const c_char, va_list args)
// {
//     BeginTooltipEx(ImGuiTooltipFlags_OverridePreviousTooltip, ImGuiWindowFlags_None);
//     TextV(fmt, args);
//     EndTooltip();
// }

// c_void SetTooltip(fmt: *const c_char, ...)
// {
//     va_list args;
//     va_start(args, fmt);
//     SetTooltipV(fmt, args);
//     va_end(args);
// }
