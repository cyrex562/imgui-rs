// dear imgui: wrappers for C++ standard library (STL) types (std::string, etc.)
// This is also an example of how you may wrap your own similar types.

// Changelog:
// - v0.10: Initial version. Added InputText() / InputTextMultiline() calls with std::string

#pragma once

#include <string>

namespace ImGui
{
    // ImGui::InputText() with std::string
    // Because text input needs dynamic resizing, we need to setup a callback to grow the capacity
     bool  InputText(const char* label, std::string* str, ImGuiInputTextFlags flags = 0, ImGuiInputTextCallback callback = None, void* user_data = None);
     bool  InputTextMultiline(const char* label, std::string* str, const Vector2D& size = DimgVec2D::new(0, 0), ImGuiInputTextFlags flags = 0, ImGuiInputTextCallback callback = None, void* user_data = None);
     bool  InputTextWithHint(const char* label, const char* hint, std::string* str, ImGuiInputTextFlags flags = 0, ImGuiInputTextCallback callback = None, void* user_data = None);
}
