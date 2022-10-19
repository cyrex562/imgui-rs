
//-----------------------------------------------------------------------------
// [SECTION] PLATFORM DEPENDENT HELPERS
//-----------------------------------------------------------------------------

// #if defined(_WIN32) && !defined(IMGUI_DISABLE_WIN32_FUNCTIONS) && !defined(IMGUI_DISABLE_WIN32_DEFAULT_CLIPBOARD_FUNCTIONS)

// #ifdef _MSC_VER
// #pragma comment(lib, "user32")
// #pragma comment(lib, "kernel32")
// #endif

// Win32 clipboard implementation
// We use g.ClipboardHandlerData for temporary storage to ensure it is freed on Shutdown()
static GetClipboardTextFn_DefaultImpl: *const c_char
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.ClipboardHandlerData.clear();
    if (!::OpenClipboard(null_mut()))
        return null_mut();
    HANDLE wbuf_handle = ::GetClipboardData(CF_UNICODETEXT);
    if (wbuf_handle == null_mut())
    {
        ::CloseClipboard();
        return null_mut();
    }
    if (*const WCHAR wbuf_global = (*const WCHAR)::GlobalLock(wbuf_handle))
    {
        let buf_len: c_int = ::WideCharToMultiByte(CP_UTF8, 0, wbuf_global, -1, null_mut(), 0, null_mut(), null_mut());
        g.ClipboardHandlerData.resize(buf_len);
        ::WideCharToMultiByte(CP_UTF8, 0, wbuf_global, -1, g.ClipboardHandlerData.Data, buf_len, null_mut(), null_mut());
    }
    ::GlobalUnlock(wbuf_handle);
    ::CloseClipboard();
    return g.ClipboardHandlerData.Data;
}

pub unsafe fn SetClipboardTextFn_DefaultImpl(*mut c_void, text: *const c_char)
{
    if !::OpenClipboard(null_mut()) { return ; }
    let wbuf_length: c_int = ::MultiByteToWideChar(CP_UTF8, 0, text, -1, null_mut(), 0);
    HGLOBAL wbuf_handle = ::GlobalAlloc(GMEM_MOVEABLE, wbuf_length * sizeof(WCHAR));
    if (wbuf_handle == null_mut())
    {
        ::CloseClipboard();
        return;
    }
    WCHAR* wbuf_global = (WCHAR*)::GlobalLock(wbuf_handle);
    ::MultiByteToWideChar(CP_UTF8, 0, text, -1, wbuf_global, wbuf_length);
    ::GlobalUnlock(wbuf_handle);
    ::EmptyClipboard();
    if (::SetClipboardData(CF_UNICODETEXT, wbuf_handle) == null_mut())
        ::GlobalFree(wbuf_handle);
    ::CloseClipboard();
}

// #elif defined(__APPLE__) && TARGET_OS_OSX && defined(IMGUI_ENABLE_OSX_DEFAULT_CLIPBOARD_FUNCTIONS)

// #include <Carbon/Carbon.h>  // Use old API to avoid need for separate .mm file
static PasteboardRef main_clipboard = 0;

// OSX clipboard implementation
// If you enable this you will need to add '-framework ApplicationServices' to your linker command-line!
pub unsafe fn SetClipboardTextFn_DefaultImpl(*mut c_void, text: *const c_char)
{
    if (!main_clipboard)
        PasteboardCreate(kPasteboardClipboard, &main_clipboard);
    PasteboardClear(main_clipboard);
    CFDataRef cf_data = CFDataCreate(kCFAllocatorDefault, (*const UInt8)text, strlen(text));
    if (cf_data)
    {
        PasteboardPutItemFlavor(main_clipboard, (PasteboardItemID)1, CFSTR("public.utf8-plain-text"), cf_data, 0);
        CFRelease(cf_data);
    }
}

static GetClipboardTextFn_DefaultImpl: *const c_char
{
    if (!main_clipboard)
        PasteboardCreate(kPasteboardClipboard, &main_clipboard);
    PasteboardSynchronize(main_clipboard);

    ItemCount item_count = 0;
    PasteboardGetItemCount(main_clipboard, &item_count);
    for (ItemCount i = 0; i < item_count; i++)
    {
        PasteboardItemID item_id = 0;
        PasteboardGetItemIdentifier(main_clipboard, i + 1, &item_id);
        CFArrayRef flavor_type_array = 0;
        PasteboardCopyItemFlavors(main_clipboard, item_id, &flavor_type_array);
        for (CFIndex j = 0, nj = CFArrayGetCount(flavor_type_array); j < nj; j++)
        {
            CFDataRef cf_data;
            if (PasteboardCopyItemFlavorData(main_clipboard, item_id, CFSTR("public.utf8-plain-text"), &cf_data) == noErr)
            {
                let g = GImGui; // ImGuiContext& g = *GImGui;
                g.ClipboardHandlerData.clear();
                let length: c_int = CFDataGetLength(cf_data);
                g.ClipboardHandlerData.resize(length + 1);
                CFDataGetBytes(cf_data, CFRangeMake(0, length), (UInt8*)g.ClipboardHandlerData.Data);
                g.ClipboardHandlerData[length] = 0;
                CFRelease(cf_data);
                return g.ClipboardHandlerData.Data;
            }
        }
    }
    return null_mut();
}

// #else

// Local Dear ImGui-only clipboard implementation, if user hasn't defined better clipboard handlers.
static GetClipboardTextFn_DefaultImpl: *const c_char
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.ClipboardHandlerData.empty() ? null_mut() : g.ClipboardHandlerData.begin();
}

pub unsafe fn SetClipboardTextFn_DefaultImpl(*mut c_void, text: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.ClipboardHandlerData.clear();
    let mut  text_end: *const c_char = text + strlen(text);
    g.ClipboardHandlerData.resize((text_end - text) + 1);
    memcpy(&g.ClipboardHandlerData[0], text, (text_end - text));
    g.ClipboardHandlerData[(text_end - text)] = 0;
}

// #endif

// Win32 API IME support (for Asian languages, etc.)
// #if defined(_WIN32) && !defined(IMGUI_DISABLE_WIN32_FUNCTIONS) && !defined(IMGUI_DISABLE_WIN32_DEFAULT_IME_FUNCTIONS)

// #include <imm.h>
// #ifdef _MSC_VER
// #pragma comment(lib, "imm32")
// #endif

pub unsafe fn SetPlatformImeDataFn_DefaultImpl(viewport: *mut ImGuiViewport, ImGuiPlatformImeData* data)
{
    // Notify OS Input Method Editor of text input position
    HWND hwnd = (HWND)viewport.PlatformHandleRaw;
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    if hwnd == 0{
        hwnd = (HWND)GetIO().ImeWindowHandle;}
// #endif
    if hwnd == 0 { return ; }

    //::ImmAssociateContextEx(hwnd, NULL, data->WantVisible ? IACE_DEFAULT : 0);
    if (HIMC himc = ::ImmGetContext(hwnd))
    {
        COMPOSITIONFORM composition_form = {};
        composition_form.ptCurrentPos.x = (LONG)(data.InputPos.x - viewport.Pos.x);
        composition_form.ptCurrentPos.y = (LONG)(data.InputPos.y - viewport.Pos.y);
        composition_form.dwStyle = CFS_FORCE_POSITION;
        ::ImmSetCompositionWindow(himc, &composition_form);
        CANDIDATEFORM candidate_form = {};
        candidate_form.dwStyle = CFS_CANDIDATEPOS;
        candidate_form.ptCurrentPos.x = (LONG)(data.InputPos.x - viewport.Pos.x);
        candidate_form.ptCurrentPos.y = (LONG)(data.InputPos.y - viewport.Pos.y);
        ::ImmSetCandidateWindow(himc, &candidate_form);
        ::ImmReleaseContext(hwnd, himc);
    }
}

// #else

pub unsafe fn SetPlatformImeDataFn_DefaultImpl(ImGuiViewport*, ImGuiPlatformImeData*) {}

// #endif
