use std::os::raw::c_char;
use crate::context::Context;
use crate::imgui_context::ImGuiContext;

// void ImGui::SetClipboardText(const char* text)
pub fn SetClipboardText(g: *mut ImGuiContext, text: *const c_char) {
    // ImGuiContext& g = *GImGui;
    if g.io.SetClipboardTextFn {
        g.io.SetClipboardTextFn(g.io.ClipboardUserData, text);
    }
}

// Win32 clipboard implementation
// We use g.clipboard_handler_data for temporary storage to ensure it is freed on Shutdown()
// static const char* GetClipboardTextFn_DefaultImpl(void*)
pub fn get_clipboard_text_fn_dflt_impl(ctx: &mut Context) -> String {
    todo!()
    // ImGuiContext& g = *GImGui;
    // ctx.clipboard_handler_data.clear();
    // TODO: winapi
    // if (!::OpenClipboard(NULL)) {
    //     return NULL;
    // }
    // HANDLE wbuf_handle = ::GetClipboardData(CF_UNICODETEXT);
    // if (wbuf_handle == NULL)
    // {
    //     ::CloseClipboard();
    //     return NULL;
    // }
    // if (const WCHAR* wbuf_global = (const WCHAR*)::GlobalLock(wbuf_handle))
    // {
    //     int buf_len = ::WideCharToMultiByte(CP_UTF8, 0, wbuf_global, -1, NULL, 0, NULL, NULL);
    //     g.ClipboardHandlerData.resize(buf_len);
    //     ::WideCharToMultiByte(CP_UTF8, 0, wbuf_global, -1, g.ClipboardHandlerData.Data, buf_len, NULL, NULL);
    // }
    // ::GlobalUnlock(wbuf_handle);
    // ::CloseClipboard();
    // return g.ClipboardHandlerData.Data;
}

// static void SetClipboardTextFn_DefaultImpl(void*, const char* text)
pub fn set_clipboard_text_fn_dflt_impl(text: &String) {
    todo!()
    // if (!::OpenClipboard(NULL))
    //     return;
    // const int wbuf_length = ::MultiByteToWideChar(CP_UTF8, 0, text, -1, NULL, 0);
    // HGLOBAL wbuf_handle = ::GlobalAlloc(GMEM_MOVEABLE, wbuf_length * sizeof(WCHAR));
    // if (wbuf_handle == NULL)
    // {
    //     ::CloseClipboard();
    //     return;
    // }
    // WCHAR* wbuf_global = (WCHAR*)::GlobalLock(wbuf_handle);
    // ::MultiByteToWideChar(CP_UTF8, 0, text, -1, wbuf_global, wbuf_length);
    // ::GlobalUnlock(wbuf_handle);
    // ::EmptyClipboard();
    // if (::SetClipboardData(CF_UNICODETEXT, wbuf_handle) == NULL)
    //     ::GlobalFree(wbuf_handle);
    // ::CloseClipboard();
    // TODO
}


//-----------------------------------------------------------------------------
// [SECTION] PLATFORM DEPENDENT HELPERS
//-----------------------------------------------------------------------------

// #if defined(_WIN32) && !defined(IMGUI_DISABLE_WIN32_FUNCTIONS) && !defined(IMGUI_DISABLE_WIN32_DEFAULT_CLIPBOARD_FUNCTIONS)
//
// #ifdef _MSC_VER
// #pragma comment(lib, "user32")
// #pragma comment(lib, "kernel32")
// #endif

// #elif defined(__APPLE__) && TARGET_OS_OSX && defined(IMGUI_ENABLE_OSX_DEFAULT_CLIPBOARD_FUNCTIONS)
//
// #include <Carbon/Carbon.h>  // Use old API to avoid need for separate .mm file
// static PasteboardRef main_clipboard = 0;

// OSX clipboard implementation
// If you enable this you will need to add '-framework ApplicationServices' to your linker command-line!
// static void SetClipboardTextFn_DefaultImpl(void*, const char* text)
// {
//     if (!main_clipboard)
//         PasteboardCreate(kPasteboardClipboard, &main_clipboard);
//     PasteboardClear(main_clipboard);
//     CFDataRef cf_data = CFDataCreate(kCFAllocatorDefault, (const UInt8*)text, strlen(text));
//     if (cf_data)
//     {
//         PasteboardPutItemFlavor(main_clipboard, (PasteboardItemID)1, CFSTR("public.utf8-plain-text"), cf_data, 0);
//         CFRelease(cf_data);
//     }
// }

// static const char* GetClipboardTextFn_DefaultImpl(void*)
// {
//     if (!main_clipboard)
//         PasteboardCreate(kPasteboardClipboard, &main_clipboard);
//     PasteboardSynchronize(main_clipboard);
//
//     ItemCount item_count = 0;
//     PasteboardGetItemCount(main_clipboard, &item_count);
//     for (ItemCount i = 0; i < item_count; i += 1)
//     {
//         PasteboardItemID item_id = 0;
//         PasteboardGetItemIdentifier(main_clipboard, i + 1, &item_id);
//         CFArrayRef flavor_type_array = 0;
//         PasteboardCopyItemFlavors(main_clipboard, item_id, &flavor_type_array);
//         for (CFIndex j = 0, nj = CFArrayGetCount(flavor_type_array); j < nj; j += 1)
//         {
//             CFDataRef cf_data;
//             if (PasteboardCopyItemFlavorData(main_clipboard, item_id, CFSTR("public.utf8-plain-text"), &cf_data) == noErr)
//             {
//                 ImGuiContext& g = *GImGui;
//                 g.ClipboardHandlerData.clear();
//                 int length = CFDataGetLength(cf_data);
//                 g.ClipboardHandlerData.resize(length + 1);
//                 CFDataGetBytes(cf_data, CFRangeMake(0, length), (UInt8*)g.ClipboardHandlerData.Data);
//                 g.ClipboardHandlerData[length] = 0;
//                 CFRelease(cf_data);
//                 return g.ClipboardHandlerData.Data;
//             }
//         }
//     }
//     return NULL;
// }

// #else
//
// // Local Dear ImGui-only clipboard implementation, if user hasn't defined better clipboard handlers.
// static const char* GetClipboardTextFn_DefaultImpl(void*)
// {
//     ImGuiContext& g = *GImGui;
//     return g.ClipboardHandlerData.empty() ? NULL : g.ClipboardHandlerData.begin();
// }
//
// static void SetClipboardTextFn_DefaultImpl(void*, const char* text)
// {
//     ImGuiContext& g = *GImGui;
//     g.ClipboardHandlerData.clear();
//     const char* text_end = text + strlen(text);
//     g.ClipboardHandlerData.resize((text_end - text) + 1);
//     memcpy(&g.ClipboardHandlerData[0], text, (text_end - text));
//     g.ClipboardHandlerData[(text_end - text)] = 0;
// }
//
// #endif
