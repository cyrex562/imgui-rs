
ImFontConfig::ImFontConfig()
{
    memset(this, 0, sizeof(*this));
    FontDataOwnedByAtlas = true;
    OversampleH = 3; // FIXME: 2 may be a better default?
    OversampleV = 1;
    GlyphMaxAdvanceX = f32::MAX;
    RasterizerMultiply = 1.0;
    EllipsisChar = (ImWchar)-1;
}
