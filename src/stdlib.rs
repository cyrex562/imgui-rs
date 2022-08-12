// dear imgui: wrappers for C++ standard library (STL) types (std::string, etc.)
// This is also an example of how you may wrap your own similar types.

// Changelog:
// - v0.10: Initial version. Added InputText() / InputTextMultiline() calls with std::string

#include "dock_style_color"

#include "imgui_stdlib.h"

struct InputTextCallback_UserData
{
    std::string*            Str;
    ImGuiInputTextCallback  ChainCallback;
    void*                   ChainCallbackUserData;
};

static int InputTextCallback(ImGuiInputTextCallbackData* data)
{
    InputTextCallback_UserData* user_data = (InputTextCallback_UserData*)data.user_data;
    if (data.EventFlag == InputTextFlags_CallbackResize)
    {
        // Resize string callback
        // If for some reason we refuse the new length (BufTextLen) and/or capacity (BufSize) we need to set them back to what we want.
        std::string* str = user_data.Str;
        // IM_ASSERT(data.Buf == str.c_str());
        str.resize(data.BufTextLen);
        data.Buf = (char*)str.c_str();
    }
    else if (user_data.ChainCallback)
    {
        // Forward to user callback, if any
        data.user_data = user_data.ChainCallbackUserData;
        return user_data.ChainCallback(data);
    }
    return 0;
}

bool ImGui::InputText(const char* label, std::string* str, InputTextFlags flags, ImGuiInputTextCallback callback, void* user_data)
{
    // IM_ASSERT((flags & ImGuiInputTextFlags_CallbackResize) == 0);
    flags |= InputTextFlags_CallbackResize;

    InputTextCallback_UserData cb_user_data;
    cb_user_data.Str = str;
    cb_user_data.ChainCallback = callback;
    cb_user_data.ChainCallbackUserData = user_data;
    return InputText(label, (char*)str.c_str(), str.capacity() + 1, flags, InputTextCallback, &cb_user_data);
}

bool ImGui::InputTextMultiline(const char* label, std::string* str, const Vector2D& size, InputTextFlags flags, ImGuiInputTextCallback callback, void* user_data)
{
    // IM_ASSERT((flags & ImGuiInputTextFlags_CallbackResize) == 0);
    flags |= InputTextFlags_CallbackResize;

    InputTextCallback_UserData cb_user_data;
    cb_user_data.Str = str;
    cb_user_data.ChainCallback = callback;
    cb_user_data.ChainCallbackUserData = user_data;
    return InputTextMultiline(label, (char*)str.c_str(), str.capacity() + 1, size, flags, InputTextCallback, &cb_user_data);
}

bool ImGui::InputTextWithHint(const char* label, const char* hint, std::string* str, InputTextFlags flags, ImGuiInputTextCallback callback, void* user_data)
{
    // IM_ASSERT((flags & ImGuiInputTextFlags_CallbackResize) == 0);
    flags |= InputTextFlags_CallbackResize;

    InputTextCallback_UserData cb_user_data;
    cb_user_data.Str = str;
    cb_user_data.ChainCallback = callback;
    cb_user_data.ChainCallbackUserData = user_data;
    return InputTextWithHint(label, hint, (char*)str.c_str(), str.capacity() + 1, flags, InputTextCallback, &cb_user_data);
}
