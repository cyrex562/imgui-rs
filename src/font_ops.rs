
// Important: this alone doesn't alter current ImDrawList state. This is called by PushFont/PopFont only.
c_void SetCurrentFont(ImFont* font)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(font && font.IsLoaded());    // Font Atlas not created. Did you call io.Fonts.GetTexDataAsRGBA32 / GetTexDataAsAlpha8 ?
    // IM_ASSERT(font.Scale > 0f32);
    g.Font = font;
    g.FontBaseSize = ImMax(1f32, g.IO.FontGlobalScale * g.Font.FontSize * g.Font.Scale);
    g.FontSize = g.CurrentWindow ? g.Currentwindow.CalcFontSize() : 0f32;

    ImFontAtlas* atlas = g.Font.ContainerAtlas;
    g.DrawListSharedData.TexUvWhitePixel = atlas.TexUvWhitePixel;
    g.DrawListSharedData.TexUvLines = atlas.TexUvLines;
    g.DrawListSharedData.Font = g.Font;
    g.DrawListSharedData.FontSize = g.FontSize;
}

c_void PushFont(ImFont* font)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!font)
        font = GetDefaultFont();
    SetCurrentFont(font);
    g.FontStack.push(font);
    g.Currentwindow.DrawList.PushTextureID(font.ContainerAtlas.TexID);
}

c_void  PopFont()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.Currentwindow.DrawList.PopTextureID();
    g.FontStack.pop_back();
    SetCurrentFont(g.FontStack.empty() ? GetDefaultFont() : g.FontStack.last().unwrap());
}


ImFont* GetFont()
{
    return GimGui.Font;
}

c_float GetFontSize()
{
    return GimGui.FontSize;
}

ImVec2 GetFontTexUvWhitePixel()
{
    return GimGui.DrawListSharedData.TexUvWhitePixel;
}

c_void SetWindowFontScale(scale: c_float)
{
    // IM_ASSERT(scale > 0f32);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.FontWindowScale = scale;
    g.FontSize = g.DrawListSharedData.FontSize = window.CalcFontSize();
}
