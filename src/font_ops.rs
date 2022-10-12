use libc::c_float;
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::GImGui;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window_ops::GetCurrentWindow;

// Important: this alone doesn't alter current ImDrawList state. This is called by PushFont/PopFont only.
pub unsafe fn SetCurrentFont(font: *mut ImFont) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(font && font.IsLoaded());    // Font Atlas not created. Did you call io.Fonts.GetTexDataAsRGBA32 / GetTexDataAsAlpha8 ?
    // IM_ASSERT(font.Scale > 0f32);
    g.Font = font;
    g.FontBaseSize = ImMax(1f32, g.IO.FontGlobalScale * g.Font.FontSize * g.Font.Scale);
    g.FontSize = if g.CurrentWindow {
        g.Currentwindow.CalcFontSize()
    } else { 0f32 };

    ImFontAtlas * atlas = g.Font.ContainerAtlas;
    g.DrawListSharedData.TexUvWhitePixel = atlas.TexUvWhitePixel;
    g.DrawListSharedData.TexUvLines = atlas.TexUvLines;
    g.DrawListSharedData.Font = g.Font;
    g.DrawListSharedData.FontSize = g.FontSize;
}

pub unsafe fn PushFont(font: *mut ImFont) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if !font {
        *font = GetDefaultFont();
    }
    SetCurrentFont(font);
    g.FontStack.push((*font).clone());
    g.Currentwindow.DrawList.PushTextureID(font.ContainerAtlas.TexID);
}

pub unsafe fn PopFont() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.Currentwindow.DrawList.PopTextureID();
    g.FontStack.pop_back();
    SetCurrentFont(&mut if g.FontStack.empty() { GetDefaultFont().clone() } else { g.FontStack.last().unwrap().clone() });
}


// ImFont* GetFont()
pub fn GetFont() -> *mut ImFont {
    return GimGui.Font;
}

// c_float GetFontSize()
pub fn GetFontSize() -> c_float {
    return GimGui.FontSize;
}

// ImVec2 GetFontTexUvWhitePixel()
pub fn GetFontTexUvWhitePixel() -> ImVec2 {
    return GimGui.DrawListSharedData.TexUvWhitePixel;
}

// c_void SetWindowFontScale(scale: c_float)
pub unsafe fn SetWindowFontScale(scale: c_float) {
    // IM_ASSERT(scale > 0f32);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    window.FontWindowScale = scale;
    g.FontSize = window.CalcFontSize();
    g.DrawListSharedData.FontSize = window.CalcFontSize();
}
