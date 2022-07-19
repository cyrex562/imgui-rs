use crate::condition::Condition;
use crate::Context;
use crate::vectors::two_d::Vector2D;
use crate::window::Window;

/// This is called during NewFrame()->UpdateViewportsNewFrame() only.
/// Need to keep in sync with set_window_pos()
/// static void TranslateWindow(ImGuiWindow* window, const Vector2D& delta)
pub fn translate_window(window: &mut Window, delta: &Vector2D)
{
    window.pos += delta;
    window.clip_rect.Translate(delta);
    window.OuterRectClipped.Translate(delta);
    window.inner_rect.Translate(delta);
    window.dc.cursor_pos += delta;
    window.dc.cursor_start_pos += delta;
    window.dc.cursor_max_pos += delta;
    window.dc.ideal_max_pos += delta;
}

// void ImGui::set_window_pos(ImGuiWindow* window, const Vector2D& pos, ImGuiCond cond)
pub fn set_window_pos(g: &mut Context, window: &mut Window, pos: &Vector2D, condition: Condition)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.set_window_pos_allow_flags & cond) == 0)
        return;

    IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.set_window_pos_allow_flags &= ~(ImGuiCond_Once | Cond::FirstUseEver | ImGuiCond_Appearing);
    window.SetWindowPosVal = Vector2D::new(f32::MAX, f32::MAX);

    // Set
    const Vector2D old_pos = window.pos;
    window.pos = f32::floor(pos);
    Vector2D offset = window.pos - old_pos;
    if (offset.x == 0.0 && offset.y == 0.0)
        return;
    MarkIniSettingsDirty(window);
    // FIXME: share code with TranslateWindow(), need to confirm whether the 3 rect modified by TranslateWindow() are desirable here.
    window.dc.cursor_pos += offset;         // As we happen to move the window while it is being appended to (which is a bad idea - will smear) let's at least offset the cursor
    window.dc.cursor_max_pos += offset;      // And more importantly we need to offset CursorMaxPos/CursorStartPos this so content_size calculation doesn't get affected.
    window.dc.ideal_max_pos += offset;
    window.dc.cursor_start_pos += offset;
}

// void ImGui::set_window_pos(const Vector2D& pos, ImGuiCond cond)
pub fn set_window_pos_2(g: &mut Window, pos: &Vector2D, condition: Condition)
{
    ImGuiWindow* window = GetCurrentWindowRead();
    set_window_pos(window, pos, cond);
}

// void ImGui::set_window_pos(const char* name, const Vector2D& pos, ImGuiCond cond)
pub fn set_window_pos3(g: &mut Context, name: &str, pos: &Vector2D, condition: Condition)
{
    if (ImGuiWindow* window = FindWindowByName(name))
        set_window_pos(window, pos, cond);
}
