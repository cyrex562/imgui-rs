use std::collections::HashSet;
use crate::color::make_color_32;
use crate::condition::Cond;
use crate::config::{ConfigFlags, IMGUI_DEBUG_INI_SETINGS};
use crate::context::{Context, ContextHook, ContextHookType};
use crate::data_authority::DataAuthority::Window;
use crate::dock_node::{dock_node_get_root_node, DockNode};
use crate::draw_data::DrawData;
use crate::draw_list::{DrawList, get_foreground_draw_list, get_viewport_draw_list};
use crate::font_atlas::FontAtlas;
use crate::types::{Id32, INVALID_ID};
use crate::globals::GImGui;
use crate::id::set_active_id;
use crate::item::ItemStatusFlags;
use crate::rect::Rect;
use crate::utils::{remove_hash_set_val, set_hash_set};
use crate::vectors::Vector2D;
use crate::viewport::{Viewport, ViewportFlags};
use crate::window::{HoveredFlags, is_window_content_hoverable, ItemFlags, start_mouse_moving_window, Window, WindowFlags};


//-------------------------------------------------------------------------
// [SECTION] FORWARD DECLARATIONS
//-------------------------------------------------------------------------

// static void             set_current_window(ImGuiWindow* window);
// static void             FindHoveredWindow();
// static ImGuiWindow*     CreateNewWindow(const char* name, ImGuiWindowFlags flags);
// static Vector2D           CalcNextScrollFromScrollTargetAndClamp(ImGuiWindow* window);
//
// static void             AddDrawListToDrawData(ImVector<ImDrawList*>* out_list, ImDrawList* draw_list);
// static void             AddWindowToSortBuffer(ImVector<ImGuiWindow*>* out_sorted_windows, ImGuiWindow* window);
//
// // Settings
// static void             WindowSettingsHandler_ClearAll(ImGuiContext*, ImGuiSettingsHandler*);
// static void*            WindowSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, const char* name);
// static void             WindowSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, void* entry, const char* line);
// static void             WindowSettingsHandler_ApplyAll(ImGuiContext*, ImGuiSettingsHandler*);
// static void             WindowSettingsHandler_WriteAll(ImGuiContext*, ImGuiSettingsHandler*, ImGuiTextBuffer* buf);
//
// // Platform Dependents default implementation for io functions
// static const char*      GetClipboardTextFn_DefaultImpl(void* user_data);
// static void             SetClipboardTextFn_DefaultImpl(void* user_data, const char* text);
// static void             SetPlatformImeDataFn_DefaultImpl(ImGuiViewport* viewport, ImGuiPlatformImeData* data);

// namespace ImGui
// {
// // Navigation
// static void             NavUpdate();
// static void             NavUpdateWindowing();
// static void             NavUpdateWindowingOverlay();
// static void             NavUpdateCancelRequest();
// static void             NavUpdateCreateMoveRequest();
// static void             NavUpdateCreateTabbingRequest();
// static float            NavUpdatePageUpPageDown();
// static inline void      NavUpdateAnyRequestFlag();
// static void             NavUpdateCreateWrappingRequest();
// static void             NavEndFrame();
// static bool             NavScoreItem(ImGuiNavItemData* result);
// static void             NavApplyItemToResult(ImGuiNavItemData* result);
// static void             NavProcessItem();
// static void             NavProcessItemForTabbingRequest(ImGuiID id);
// static Vector2D           NavCalcPreferredRefPos();
// static void             NavSaveLastChildNavWindowIntoParent(ImGuiWindow* nav_window);
// static ImGuiWindow*     NavRestoreLastChildNavWindow(ImGuiWindow* window);
// static void             NavRestoreLayer(ImGuiNavLayer layer);
// static void             NavRestoreHighlightAfterMove();
// static int              FindWindowFocusIndex(ImGuiWindow* window);
//
// // Error Checking and Debug Tools
// static void             ErrorCheckNewFrameSanityChecks();
// static void             ErrorCheckEndFrameSanityChecks();
// static void             UpdateDebugToolItemPicker();
// static void             UpdateDebugToolStackQueries();
//
// // Misc
// static void             UpdateSettings();
// static void             UpdateKeyboardInputs();
// static void             UpdateMouseInputs();
// static void             UpdateMouseWheel();
// static bool             UpdateWindowManualResize(ImGuiWindow* window, const Vector2D& size_auto_fit, int* border_held, int resize_grip_count, ImU32 resize_grip_col[4], const ImRect& visibility_rect);
// static void             RenderWindowOuterBorders(ImGuiWindow* window);
// static void             RenderWindowDecorations(ImGuiWindow* window, const ImRect& title_bar_rect, bool title_bar_is_highlight, bool handle_borders_and_resize_grips, int resize_grip_count, const ImU32 resize_grip_col[4], float resize_grip_draw_size);
// static void             RenderWindowTitleBarContents(ImGuiWindow* window, const ImRect& title_bar_rect, const char* name, bool* p_open);
// static void             RenderDimmedBackgroundBehindWindow(ImGuiWindow* window, ImU32 col);
// static void             RenderDimmedBackgrounds();
// static ImGuiWindow*     FindBlockingModal(ImGuiWindow* window);
//
// // viewports
// const ImGuiID           IMGUI_VIEWPORT_DEFAULT_ID = 0x11111111; // Using an arbitrary constant instead of e.g. ImHashStr("ViewportDefault", 0); so it's easier to spot in the debugger. The exact value doesn't matter.
// static ImGuiViewportP*  AddUpdateViewport(ImGuiWindow* window, ImGuiID id, const Vector2D& platform_pos, const Vector2D& size, ImGuiViewportFlags flags);
// static void             DestroyViewport(ImGuiViewportP* viewport);
// static void             UpdateViewportsNewFrame();
// static void             UpdateViewportsEndFrame();
// static void             WindowSelectViewport(ImGuiWindow* window);
// static void             WindowSyncOwnedViewport(ImGuiWindow* window, ImGuiWindow* parent_window_in_stack);
// static bool             update_try_merge_window_into_host_viewport(ImGuiWindow* window, ImGuiViewportP* host_viewport);
// static bool             UpdateTryMergeWindowIntoHostViewports(ImGuiWindow* window);
// static bool             GetWindowAlwaysWantOwnViewport(ImGuiWindow* window);
// static int              FindPlatformMonitorForPos(const Vector2D& pos);
// static int              FindPlatformMonitorForRect(const ImRect& r);
// static void             UpdateViewportPlatformMonitor(ImGuiViewportP* viewport);
//
// }

//-----------------------------------------------------------------------------
// [SECTION] CONTEXT AND MEMORY ALLOCATORS
//-----------------------------------------------------------------------------

// DLL users:
// - Heaps and globals are not shared across DLL boundaries!
// - You will need to call SetCurrentContext() + SetAllocatorFunctions() for each static/DLL boundary you are calling from.
// - Same applies for hot-reloading mechanisms that are reliant on reloading DLL (note that many hot-reloading mechanisms work without DLL).
// - Using Dear ImGui via a shared library is not recommended, because of function call overhead and because we don't guarantee backward nor forward ABI compatibility.
// - Confused? In a debugger: add GImGui to your watch window and notice how its value changes depending on your current location (which DLL boundary you are in).
// #endif

// TODO
// Memory Allocator functions. Use SetAllocatorFunctions() to change them.
// - You probably don't want to modify that mid-program, and if you use global/static e.g. ImVector<> instances you may need to keep them accessible during program destruction.
// - DLL users: read comments above.
// #ifndef IMGUI_DISABLE_DEFAULT_ALLOCATORS
// static void*   MallocWrapper(size_t size, void* user_data)    { IM_UNUSED(user_data); return malloc(size); }
// static void    FreeWrapper(void* ptr, void* user_data)        { IM_UNUSED(user_data); free(ptr); }
// #else
// static void*   MallocWrapper(size_t size, void* user_data)    { IM_UNUSED(user_data); IM_UNUSED(size); IM_ASSERT(0); return NULL; }
// static void    FreeWrapper(void* ptr, void* user_data)        { IM_UNUSED(user_data); IM_UNUSED(ptr); IM_ASSERT(0); }
// #endif
// static ImGuiMemAllocFunc    GImAllocatorAllocFunc = MallocWrapper;
// static ImGuiMemFreeFunc     GImAllocatorFreeFunc = FreeWrapper;
// static void*                GImAllocatorUserData = NULL;

//-----------------------------------------------------------------------------
// [SECTION] USER FACING STRUCTURES (ImGuiStyle, ImGuiIO)
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] MISC HELPERS/UTILITIES (Geometry functions)
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] MISC HELPERS/UTILITIES (String, Format, Hash functions)
//-----------------------------------------------------------------------------
//
// // Consider using _stricmp/_strnicmp under windows or strcasecmp/strncasecmp. We don't actually use either ImStricmp/ImStrnicmp in the codebase any more.
// int ImStricmp(const char* str1, const char* str2)
// {
//     int d;
//     while ((d = toupper(*str2) - toupper(*str1)) == 0 && *str1) { str1++; str2++; }
//     return d;
// }
//
// int ImStrnicmp(const char* str1, const char* str2, size_t count)
// {
//     int d = 0;
//     while (count > 0 && (d = toupper(*str2) - toupper(*str1)) == 0 && *str1) { str1++; str2++; count--; }
//     return d;
// }
//
// void ImStrncpy(char* dst, const char* src, size_t count)
// {
//     if (count < 1)
//         return;
//     if (count > 1)
//         strncpy(dst, src, count - 1);
//     dst[count - 1] = 0;
// }
//
// char* ImStrdup(const char* str)
// {
//     size_t len = strlen(str);
//     void* buf = IM_ALLOC(len + 1);
//     return (char*)memcpy(buf, (const void*)str, len + 1);
// }
//
// char* ImStrdupcpy(char* dst, size_t* p_dst_size, const char* src)
// {
//     size_t dst_buf_size = p_dst_size ? *p_dst_size : strlen(dst) + 1;
//     size_t src_size = strlen(src) + 1;
//     if (dst_buf_size < src_size)
//     {
//         IM_FREE(dst);
//         dst = (char*)IM_ALLOC(src_size);
//         if (p_dst_size)
//             *p_dst_size = src_size;
//     }
//     return (char*)memcpy(dst, (const void*)src, src_size);
// }
//
// const char* ImStrchrRange(const char* str, const char* str_end, char c)
// {
//     const char* p = (const char*)memchr(str, c, str_end - str);
//     return p;
// }
//
// int ImStrlenW(const ImWchar* str)
// {
//     //return wcslen((const wchar_t*)str);  // FIXME-OPT: Could use this when wchar_t are 16-bit
//     int n = 0;
//     while (*str++) n++;
//     return n;
// }
//

//
// const ImWchar* ImStrbolW(const ImWchar* buf_mid_line, const ImWchar* buf_begin) // find beginning-of-line
// {
//     while (buf_mid_line > buf_begin && buf_mid_line[-1] != '\n')
//         buf_mid_line--;
//     return buf_mid_line;
// }
//
// const char* ImStristr(const char* haystack, const char* haystack_end, const char* needle, const char* needle_end)
// {
//     if (!needle_end)
//         needle_end = needle + strlen(needle);
//
//     const char un0 = (char)toupper(*needle);
//     while ((!haystack_end && *haystack) || (haystack_end && haystack < haystack_end))
//     {
//         if (toupper(*haystack) == un0)
//         {
//             const char* b = needle + 1;
//             for (const char* a = haystack + 1; b < needle_end; a++, b++)
//                 if (toupper(*a) != toupper(*b))
//                     break;
//             if (b == needle_end)
//                 return haystack;
//         }
//         haystack++;
//     }
//     return NULL;
// }
//
// // Trim str by offsetting contents when there's leading data + writing a \0 at the trailing position. We use this in situation where the cost is negligible.
// void ImStrTrimBlanks(char* buf)
// {
//     char* p = buf;
//     while (p[0] == ' ' || p[0] == '\t')     // Leading blanks
//         p++;
//     char* p_start = p;
//     while (*p != 0)                         // Find end of string
//         p++;
//     while (p > p_start && (p[-1] == ' ' || p[-1] == '\t'))  // Trailing blanks
//         p--;
//     if (p_start != buf)                     // Copy memory if we had leading blanks
//         memmove(buf, p_start, p - p_start);
//     buf[p - p_start] = 0;                   // Zero terminate
// }
//
// const char* ImStrSkipBlank(const char* str)
// {
//     while (str[0] == ' ' || str[0] == '\t')
//         str++;
//     return str;
// }
//
// // A) MSVC version appears to return -1 on overflow, whereas glibc appears to return total count (which may be >= buf_size).
// // Ideally we would test for only one of those limits at runtime depending on the behavior the vsnprintf(), but trying to deduct it at compile time sounds like a pandora can of worm.
// // B) When buf==NULL vsnprintf() will return the output size.
// #ifndef IMGUI_DISABLE_DEFAULT_FORMAT_FUNCTIONS
//
// // We support stb_sprintf which is much faster (see: https://github.com/nothings/stb/blob/master/stb_sprintf.h)
// // You may set IMGUI_USE_STB_SPRINTF to use our default wrapper, or set IMGUI_DISABLE_DEFAULT_FORMAT_FUNCTIONS
// // and setup the wrapper yourself. (FIXME-OPT: Some of our high-level operations such as ImGuiTextBuffer::appendfv() are
// // designed using two-passes worst case, which probably could be improved using the stbsp_vsprintfcb() function.)
// #ifdef IMGUI_USE_STB_SPRINTF
// #define STB_SPRINTF_IMPLEMENTATION
// #ifdef IMGUI_STB_SPRINTF_FILENAME
// #include IMGUI_STB_SPRINTF_FILENAME
// #else
// #include "stb_sprintf.h"
// #endif
// #endif
//
// #if defined(_MSC_VER) && !defined(vsnprintf)
// #define vsnprintf _vsnprintf
// #endif
//
// int ImFormatString(char* buf, size_t buf_size, const char* fmt, ...)
// {
//     va_list args;
//     va_start(args, fmt);
// #ifdef IMGUI_USE_STB_SPRINTF
//     int w = stbsp_vsnprintf(buf, buf_size, fmt, args);
// #else
//     int w = vsnprintf(buf, buf_size, fmt, args);
// #endif
//     va_end(args);
//     if (buf == NULL)
//         return w;
//     if (w == -1 || w >= buf_size)
//         w = buf_size - 1;
//     buf[w] = 0;
//     return w;
// }
//
// int ImFormatStringV(char* buf, size_t buf_size, const char* fmt, va_list args)
// {
// #ifdef IMGUI_USE_STB_SPRINTF
//     int w = stbsp_vsnprintf(buf, buf_size, fmt, args);
// #else
//     int w = vsnprintf(buf, buf_size, fmt, args);
// #endif
//     if (buf == NULL)
//         return w;
//     if (w == -1 || w >= buf_size)
//         w = buf_size - 1;
//     buf[w] = 0;
//     return w;
// }
// #endif // #ifdef IMGUI_DISABLE_DEFAULT_FORMAT_FUNCTIONS
//
// void ImFormatStringToTempBuffer(const char** out_buf, const char** out_buf_end, const char* fmt, ...)
// {
//     ImGuiContext& g = *GImGui;
//     va_list args;
//     va_start(args, fmt);
//     int buf_len = ImFormatStringV(g.temp_buffer.data, g.temp_buffer.size, fmt, args);
//     *out_buf = g.temp_buffer.data;
//     if (out_buf_end) { *out_buf_end = g.temp_buffer.data + buf_len; }
//     va_end(args);
// }
//
// void ImFormatStringToTempBufferV(const char** out_buf, const char** out_buf_end, const char* fmt, va_list args)
// {
//     ImGuiContext& g = *GImGui;
//     int buf_len = ImFormatStringV(g.temp_buffer.data, g.temp_buffer.size, fmt, args);
//     *out_buf = g.temp_buffer.data;
//     if (out_buf_end) { *out_buf_end = g.temp_buffer.data + buf_len; }
// }


//-----------------------------------------------------------------------------
// [SECTION] MISC HELPERS/UTILITIES (File functions)
//-----------------------------------------------------------------------------

// Default file functions
// #ifndef IMGUI_DISABLE_DEFAULT_FILE_FUNCTIONS

//-----------------------------------------------------------------------------
// [SECTION] MISC HELPERS/UTILITIES (ImText* functions)
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] MISC HELPERS/UTILITIES (Color functions)
// Note: The Convert functions are early design which are not consistent with other API.
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] ImGuiStorage
// Helper: Key->value storage
//-----------------------------------------------------------------------------


//-----------------------------------------------------------------------------
// [SECTION] ImGuiTextFilter
//-----------------------------------------------------------------------------


// Helper: Parse and apply text filters. In format "aaaaa[,bbbb][,ccccc]"
// ImGuiTextFilter::ImGuiTextFilter(const char* default_filter) //-V1077
// {
//     InputBuf[0] = 0;
//     CountGrep = 0;
//     if (default_filter)
//     {
//         ImStrncpy(InputBuf, default_filter, IM_ARRAYSIZE(InputBuf));
//         build();
//     }
// }

// bool ImGuiTextFilter::Draw(const char* label, float width)
// {
//     if (width != 0.0)
//         ImGui::SetNextItemWidth(width);
//     bool value_changed = ImGui::InputText(label, InputBuf, IM_ARRAYSIZE(InputBuf));
//     if (value_changed)
//         build();
//     return value_changed;
// }

//-----------------------------------------------------------------------------
// [SECTION] ImGuiTextBuffer
//-----------------------------------------------------------------------------

// On some platform vsnprintf() takes va_list by reference and modifies it.
// va_copy is the 'correct' way to copy a va_list but Visual Studio prior to 2013 doesn't have it.
// #ifndef va_copy
// #if defined(__GNUC__) || defined(__clang__)
// #define va_copy(dest, src) __builtin_va_copy(dest, src)
// #else
// #define va_copy(dest, src) (dest = src)
// #endif
// #endif

//-----------------------------------------------------------------------------
// [SECTION] ImGuiListClipper
// This is currently not as flexible/powerful as it should be and really confusing/spaghetti, mostly because we changed
// the API mid-way through development and support two ways to using the clipper, needs some rework (see TODO)
//-----------------------------------------------------------------------------

//
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
// // Legacy helper to calculate coarse clipping of large list of evenly sized items.
// // This legacy API is not ideal because it assume we will return a single contiguous rectangle.
// // Prefer using ImGuiListClipper which can returns non-contiguous ranges.
// void ImGui::CalcListClipping(int items_count, float items_height, int* out_items_display_start, int* out_items_display_end)
// {
//     ImGuiContext& g = *GImGui;
//     ImGuiWindow* window = g.current_window;
//     if (g.log_enabled)
//     {
//         // If logging is active, do not perform any clipping
//         *out_items_display_start = 0;
//         *out_items_display_end = items_count;
//         return;
//     }
//     if (GetSkipItemForListClipping())
//     {
//         *out_items_display_start = *out_items_display_end = 0;
//         return;
//     }
//
//     // We create the union of the clip_rect and the scoring rect which at worst should be 1 page away from clip_rect
//     // We don't include g.nav_id's rectangle in there (unless g.nav_just_moved_to_id is set) because the rectangle enlargement can get costly.
//     ImRect rect = window->clip_rect;
//     if (g.nav_move_scoring_items)
//         rect.Add(g.nav_scoring_no_clip_rect);
//     if (g.nav_just_moved_to_id && window->nav_last_ids[0] == g.nav_just_moved_to_id)
//         rect.Add(WindowRectRelToAbs(window, window->nav_rect_rel[0])); // Could store and use NavJustMovedToRectRel
//
//     const Vector2D pos = window->dc.CursorPos;
//     int start = ((rect.min.y - pos.y) / items_height);
//     int end = ((rect.max.y - pos.y) / items_height);
//
//     // When performing a navigation request, ensure we have one item extra in the direction we are moving to
//     // FIXME: Verify this works with tabbing
//     const bool is_nav_request = (g.nav_move_scoring_items && g.nav_window && g.nav_window->root_window_for_nav == window->root_window_for_nav);
//     if (is_nav_request && g.nav_move_clip_dir == ImGuiDir_Up)
//         start--;
//     if (is_nav_request && g.nav_move_clip_dir == ImGuiDir_Down)
//         end += 1;
//
//     start = ImClamp(start, 0, items_count);
//     end = ImClamp(end + 1, start, items_count);
//     *out_items_display_start = start;
//     *out_items_display_end = end;
// }
// #endif

//-----------------------------------------------------------------------------
// [SECTION] STYLING
//-----------------------------------------------------------------------------


//-----------------------------------------------------------------------------
// [SECTION] RENDER HELPERS
// Some of those (internal) functions are currently quite a legacy mess - their signature and behavior will change,
// we need a nicer separation between low-level functions and high-level functions relying on the ImGui context.
// Also see imgui_draw.cpp for some more which have been reworked to not rely on ImGui:: context.
//-----------------------------------------------------------------------------


//-----------------------------------------------------------------------------
// [SECTION] MAIN CODE (most of the code! lots of stuff, needs tidying up!)
//-----------------------------------------------------------------------------

// IM_ALLOC() == ImGui::MemAlloc()
// void* ImGui::MemAlloc(size_t size)
// {
//     if (ImGuiContext* ctx = GImGui)
//         ctx->IO.MetricsActiveAllocations += 1;
// {
//     return GImGui;
// }

// void ImGui::SetCurrentContext(ImGuiContext* ctx)
// {
// #ifdef IMGUI_SET_CURRENT_CONTEXT_FUNC
//     IMGUI_SET_CURRENT_CONTEXT_FUNC(ctx); // For custom thread-based hackery you may want to have control over this.
// #else
//     GImGui = ctx;
// #endif
// }

// void ImGui::SetAllocatorFunctions(ImGuiMemAllocFunc alloc_func, ImGuiMemFreeFunc free_func, void* user_data)
// {
//     GImAllocatorAllocFunc = alloc_func;
//     GImAllocatorFreeFunc = free_func;
//     GImAllocatorUserData = user_data;
// }

// This is provided to facilitate copying allocators from one static/DLL boundary to another (e.g. retrieve default allocator of your executable address space)
// void ImGui::GetAllocatorFunctions(ImGuiMemAllocFunc* p_alloc_func, ImGuiMemFreeFunc* p_free_func, void** p_user_data)
// {
//     *p_alloc_func = GImAllocatorAllocFunc;
//     *p_free_func = GImAllocatorFreeFunc;
//     *p_user_data = GImAllocatorUserData;
// }

// ImGuiContext* ImGui::CreateContext(ImFontAtlas* shared_font_atlas)
// pub fn create_context(ctx: Option<&mut Context>, shared_font_atlas: &FontAtlas) -> Context
// {
//     // ImGuiContext* prev_ctx = GetCurrentContext();
//     // ImGuiContext* ctx = IM_NEW(ImGuiContext)(shared_font_atlas);
//
//     SetCurrentContext(ctx);
//     Initialize();
//     if (prev_ctx != NULL)
//         SetCurrentContext(prev_ctx); // Restore previous context if any, else keep new one.
//     return ctx;
// }

// void ImGui::DestroyContext(ImGuiContext* ctx)
// pub fn destroy_context(ctx: &mut Context)
// {
//     ImGuiContext* prev_ctx = GetCurrentContext();
//     if (ctx == NULL) //-V1051
//         ctx = prev_ctx;
//     SetCurrentContext(ctx);
//     Shutdown();
//     SetCurrentContext((prev_ctx != ctx) ? prev_ctx : NULL);
//     IM_DELETE(ctx);
// }

// ImGuiIO& ImGui::GetIO()
// {
//     IM_ASSERT(GImGui != NULL && "No current context. Did you call ImGui::CreateContext() and ImGui::SetCurrentContext() ?");
//     return GImGui->IO;
// }

// ImGuiPlatformIO& ImGui::GetPlatformIO()
// {
//     IM_ASSERT(GImGui != NULL && "No current context. Did you call ImGui::CreateContext() or ImGui::SetCurrentContext()?");
//     return GImGui->PlatformIO;
// }

// double ImGui::GetTime()
// {
//     return GImGui->Time;
// }

// int ImGui::GetFrameCount()
// {
//     return GImGui->FrameCount;
// }

// ImDrawListSharedData* ImGui::GetDrawListSharedData()
// {
//     return &GImGui->DrawListSharedData;
// }

// Initiate moving window when clicking on empty space or title bar.
// Handle left-click and right-click focus.
// void ImGui::UpdateMouseMovingWindowEndFrame()
pub fn update_mouse_moving_window_end_frame(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    if g.active_id != INVALID_ID || g.hovered_id != INVALID_ID {
        return;
    }

    // Unless we just made a window/popup appear
    // if (g.nav_window && g.nav_window.appearing) {
    //     return;
    // }
    if g.nav_window != INVALID_ID {
        let win = g.get_window(g.nav_window).unwrap();
        if win.appearing {
            return;
        }
    }

    // Click on empty space to focus window and start moving
    // (after we're done with all our widgets, so e.g. clicking on docking tab-bar which have set hovered_id already and not get us here!)
    if g.io.mouse_clicked[0]
    {
        // Handle the edge case of a popup being closed while clicking in its empty space.
        // If we try to focus it, focus_window() > ClosePopupsOverWindow() will accidentally close any parent popups because they are not linked together any more.
        // ImGuiWindow* root_window = g.hovered_window ? g.hovered_window->RootWindow : NULL;
        let root_window = if g.hovered_window_id != INVALID_ID {
            let hov_win = g.get_window(g.hovered_window_id).unwrap();
            Some(g.get_window(hov_win.root_window).unwrap())
        } else {
            None
        };
        // const bool is_closed_popup = root_window && (root_window.Flags & ImGuiWindowFlags_Popup) && !IsPopupOpen(root_window.PopupId, ImGuiPopupFlags_AnyPopupLevel);
        let is_closed_popup: bool = if root_window.is_some() {
            let root_win = root_window.unwrap();
            if root_win.flags.contains(&WindowFlags::Popup) {
                if is_popup_open(root_win.popup_id, PopupFlags::AnyPopupLevel) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        // if (root_window != NULL && !is_closed_popup)
        if root_window != INVALID_ID && is_closed_popup == false
        {
            start_mouse_moving_window(g, g.hovered_window); //-V595

            // Cancel moving if clicked outside of title bar
            if (g.io.ConfigWindowsMoveFromTitleBarOnly)
                if (!(root_window.Flags & ImGuiWindowFlags_NoTitleBar) || root_window.DockIsActive)
                    if (!root_window.TitleBarRect().Contains(g.io.MouseClickedPos[0]))
                        g.moving_window = NULL;

            // Cancel moving if clicked over an item which was disabled or inhibited by popups (note that we know hovered_id == 0 already)
            if (g.HoveredIdDisabled)
                g.moving_window = NULL;
        }
        else if (root_window == NULL && g.nav_window != NULL && GetTopMostPopupModal() == NULL)
        {
            // Clicking on void disable focus
            focus_window(NULL);
        }
    }

    // With right mouse button we close popups without changing focus based on where the mouse is aimed
    // Instead, focus will be restored to the window under the bottom-most closed popup.
    // (The left mouse button path calls focus_window on the hovered window, which will lead NewFrame->ClosePopupsOverWindow to trigger)
    if (g.io.mouse_clicked[1])
    {
        // Find the top-most window between hovered_window and the top-most Modal Window.
        // This is where we can trim the popup stack.
        ImGuiWindow* modal = GetTopMostPopupModal();
        bool hovered_window_above_modal = g.hovered_window && (modal == NULL || IsWindowAbove(g.hovered_window, modal));
        ClosePopupsOverWindow(hovered_window_above_modal ? g.hovered_window : modal, true);
    }
}

// This is called during NewFrame()->UpdateViewportsNewFrame() only.
// Need to keep in sync with set_window_pos()
static void TranslateWindow(ImGuiWindow* window, const Vector2D& delta)
{
    window.Pos += delta;
    window.ClipRect.Translate(delta);
    window.OuterRectClipped.Translate(delta);
    window.InnerRect.Translate(delta);
    window.DC.CursorPos += delta;
    window.DC.CursorStartPos += delta;
    window.DC.CursorMaxPos += delta;
    window.DC.IdealMaxPos += delta;
}

static void ScaleWindow(ImGuiWindow* window, float scale)
{
    Vector2D origin = window.viewport.pos;
    window.Pos = ImFloor((window.Pos - origin) * scale + origin);
    window.Size = ImFloor(window.Size * scale);
    window.SizeFull = ImFloor(window.SizeFull * scale);
    window.ContentSize = ImFloor(window.ContentSize * scale);
}

static bool IsWindowActiveAndVisible(ImGuiWindow* window)
{
    return (window.Active) && (!window.Hidden);
}

static void ImGui::UpdateKeyboardInputs()
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;

    // Import legacy keys or verify they are not used
#ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    if (io.BackendUsingLegacyKeyArrays == 0)
    {
        // Backend used new io.add_key_event() API: Good! Verify that old arrays are never written to externally.
        for (int n = 0; n < ImGuiKey_LegacyNativeKey_END; n += 1)
            IM_ASSERT((io.KeysDown[n] == false || IsKeyDown(n)) && "Backend needs to either only use io.add_key_event(), either only fill legacy io.KeysDown[] + io.KeyMap[]. Not both!");
    }
    else
    {
        if (g.FrameCount == 0)
            for (int n = ImGuiKey_LegacyNativeKey_BEGIN; n < ImGuiKey_LegacyNativeKey_END; n += 1)
                IM_ASSERT(g.io.KeyMap[n] == -1 && "Backend is not allowed to write to io.KeyMap[0..511]!");

        // build reverse KeyMap (Named -> Legacy)
        for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n += 1)
            if (io.KeyMap[n] != -1)
            {
                IM_ASSERT(IsLegacyKey((ImGuiKey)io.KeyMap[n]));
                io.KeyMap[io.KeyMap[n]] = n;
            }

        // Import legacy keys into new ones
        for (int n = ImGuiKey_LegacyNativeKey_BEGIN; n < ImGuiKey_LegacyNativeKey_END; n += 1)
            if (io.KeysDown[n] || io.BackendUsingLegacyKeyArrays == 1)
            {
                const ImGuiKey key = (ImGuiKey)(io.KeyMap[n] != -1 ? io.KeyMap[n] : n);
                IM_ASSERT(io.KeyMap[n] == -1 || IsNamedKey(key));
                io.KeysData[key].Down = io.KeysDown[n];
                if (key != n)
                    io.KeysDown[key] = io.KeysDown[n]; // Allow legacy code using io.KeysDown[GetKeyIndex()] with old backends
                io.BackendUsingLegacyKeyArrays = 1;
            }
        if (io.BackendUsingLegacyKeyArrays == 1)
        {
            io.KeysData[ImGuiKey_ModCtrl].Down = io.KeyCtrl;
            io.KeysData[ImGuiKey_ModShift].Down = io.KeyShift;
            io.KeysData[ImGuiKey_ModAlt].Down = io.KeyAlt;
            io.KeysData[ImGuiKey_ModSuper].Down = io.KeySuper;
        }
    }


    // Synchronize io.key_mods with individual modifiers io.KeyXXX bools
    io.KeyMods = GetMergedModFlags();

    // clear gamepad data if disabled
    if ((io.BackendFlags & ImGuiBackendFlags_HasGamepad) == 0)
        for (int i = ImGuiKey_Gamepad_BEGIN; i < ImGuiKey_Gamepad_END; i += 1)
        {
            io.KeysData[i - ImGuiKey_KeysData_OFFSET].Down = false;
            io.KeysData[i - ImGuiKey_KeysData_OFFSET].AnalogValue = 0.0;
        }

    // Update keys
    for (int i = 0; i < IM_ARRAYSIZE(io.KeysData); i += 1)
    {
        ImGuiKeyData* key_data = &io.KeysData[i];
        key_data->DownDurationPrev = key_data->DownDuration;
        key_data->DownDuration = key_data->Down ? (key_data->DownDuration < 0.0 ? 0.0 : key_data->DownDuration + io.DeltaTime) : -1.0;
    }
}

static void ImGui::UpdateMouseInputs()
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;

    // Round mouse position to avoid spreading non-rounded position (e.g. UpdateManualResize doesn't support them well)
    if (is_mouse_pos_valid(&io.MousePos))
        io.MousePos = g.MouseLastValidPos = ImFloorSigned(io.MousePos);

    // If mouse just appeared or disappeared (usually denoted by -FLT_MAX components) we cancel out movement in mouse_delta
    if (is_mouse_pos_valid(&io.MousePos) && is_mouse_pos_valid(&io.MousePosPrev))
        io.MouseDelta = io.MousePos - io.MousePosPrev;
    else
        io.MouseDelta = DimgVec2D::new(0.0, 0.0);

    // If mouse moved we re-enable mouse hovering in case it was disabled by gamepad/keyboard. In theory should use a >0.0 threshold but would need to reset in everywhere we set this to true.
    if (io.MouseDelta.x != 0.0 || io.MouseDelta.y != 0.0)
        g.NavDisableMouseHover = false;

    io.MousePosPrev = io.MousePos;
    for (int i = 0; i < IM_ARRAYSIZE(io.mouse_down); i += 1)
    {
        io.mouse_clicked[i] = io.mouse_down[i] && io.MouseDownDuration[i] < 0.0;
        io.MouseClickedCount[i] = 0; // Will be filled below
        io.MouseReleased[i] = !io.mouse_down[i] && io.MouseDownDuration[i] >= 0.0;
        io.MouseDownDurationPrev[i] = io.MouseDownDuration[i];
        io.MouseDownDuration[i] = io.mouse_down[i] ? (io.MouseDownDuration[i] < 0.0 ? 0.0 : io.MouseDownDuration[i] + io.DeltaTime) : -1.0;
        if (io.mouse_clicked[i])
        {
            bool is_repeated_click = false;
            if ((float)(g.Time - io.MouseClickedTime[i]) < io.MouseDoubleClickTime)
            {
                Vector2D delta_from_click_pos = is_mouse_pos_valid(&io.MousePos) ? (io.MousePos - io.MouseClickedPos[i]) : DimgVec2D::new(0.0, 0.0);
                if (ImLengthSqr(delta_from_click_pos) < io.MouseDoubleClickMaxDist * io.MouseDoubleClickMaxDist)
                    is_repeated_click = true;
            }
            if (is_repeated_click)
                io.MouseClickedLastCount[i] += 1;
            else
                io.MouseClickedLastCount[i] = 1;
            io.MouseClickedTime[i] = g.Time;
            io.MouseClickedPos[i] = io.MousePos;
            io.MouseClickedCount[i] = io.MouseClickedLastCount[i];
            io.MouseDragMaxDistanceAbs[i] = DimgVec2D::new(0.0, 0.0);
            io.MouseDragMaxDistanceSqr[i] = 0.0;
        }
        else if (io.mouse_down[i])
        {
            // Maintain the maximum distance we reaching from the initial click position, which is used with dragging threshold
            Vector2D delta_from_click_pos = is_mouse_pos_valid(&io.MousePos) ? (io.MousePos - io.MouseClickedPos[i]) : DimgVec2D::new(0.0, 0.0);
            io.MouseDragMaxDistanceSqr[i] = ImMax(io.MouseDragMaxDistanceSqr[i], ImLengthSqr(delta_from_click_pos));
            io.MouseDragMaxDistanceAbs[i].x = ImMax(io.MouseDragMaxDistanceAbs[i].x, delta_from_click_pos.x < 0.0 ? -delta_from_click_pos.x : delta_from_click_pos.x);
            io.MouseDragMaxDistanceAbs[i].y = ImMax(io.MouseDragMaxDistanceAbs[i].y, delta_from_click_pos.y < 0.0 ? -delta_from_click_pos.y : delta_from_click_pos.y);
        }

        // We provide io.mouse_double_clicked[] as a legacy service
        io.MouseDoubleClicked[i] = (io.MouseClickedCount[i] == 2);

        // Clicking any mouse button reactivate mouse hovering which may have been deactivated by gamepad/keyboard navigation
        if (io.mouse_clicked[i])
            g.NavDisableMouseHover = false;
    }
}

static void StartLockWheelingWindow(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    if (g.WheelingWindow == window)
        return;
    g.WheelingWindow = window;
    g.WheelingWindowRefMousePos = g.io.MousePos;
    g.WheelingWindowTimer = WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER;
}

void ImGui::UpdateMouseWheel()
{
    ImGuiContext& g = *GImGui;

    // Reset the locked window if we move the mouse or after the timer elapses
    if (g.WheelingWindow != NULL)
    {
        g.WheelingWindowTimer -= g.io.DeltaTime;
        if (is_mouse_pos_valid() && ImLengthSqr(g.io.MousePos - g.WheelingWindowRefMousePos) > g.io.MouseDragThreshold * g.io.MouseDragThreshold)
            g.WheelingWindowTimer = 0.0;
        if (g.WheelingWindowTimer <= 0.0)
        {
            g.WheelingWindow = NULL;
            g.WheelingWindowTimer = 0.0;
        }
    }

    float wheel_x = g.io.MouseWheelH;
    float wheel_y = g.io.MouseWheel;
    if (wheel_x == 0.0 && wheel_y == 0.0)
        return;

    if ((g.active_id != 0 && g.ActiveIdUsingMouseWheel) || (g.HoveredIdPreviousFrame != 0 && g.HoveredIdPreviousFrameUsingMouseWheel))
        return;

    ImGuiWindow* window = g.WheelingWindow ? g.WheelingWindow : g.hovered_window;
    if (!window || window.Collapsed)
        return;

    // Zoom / scale window
    // FIXME-OBSOLETE: This is an old feature, it still works but pretty much nobody is using it and may be best redesigned.
    if (wheel_y != 0.0 && g.io.KeyCtrl && g.io.FontAllowUserScaling)
    {
        StartLockWheelingWindow(window);
        const float new_font_scale = ImClamp(window.FontWindowScale + g.io.MouseWheel * 0.10, 0.50, 2.50);
        const float scale = new_font_scale / window.FontWindowScale;
        window.FontWindowScale = new_font_scale;
        if (window == window.RootWindow)
        {
            const Vector2D offset = window.Size * (1.0 - scale) * (g.io.MousePos - window.Pos) / window.Size;
            set_window_pos(window, window.Pos + offset, 0);
            window.Size = ImFloor(window.Size * scale);
            window.SizeFull = ImFloor(window.SizeFull * scale);
        }
        return;
    }

    // Mouse wheel scrolling
    // If a child window has the ImGuiWindowFlags_NoScrollWithMouse flag, we give a chance to scroll its parent
    if (g.io.KeyCtrl)
        return;

    // As a standard behavior holding SHIFT while using Vertical Mouse Wheel triggers Horizontal scroll instead
    // (we avoid doing it on OSX as it the OS input layer handles this already)
    const bool swap_axis = g.io.KeyShift && !g.io.ConfigMacOSXBehaviors;
    if (swap_axis)
    {
        wheel_x = wheel_y;
        wheel_y = 0.0;
    }

    // Vertical Mouse Wheel scrolling
    if (wheel_y != 0.0)
    {
        StartLockWheelingWindow(window);
        while ((window.Flags & ImGuiWindowFlags_ChildWindow) && ((window.ScrollMax.y == 0.0) || ((window.Flags & ImGuiWindowFlags_NoScrollWithMouse) && !(window.Flags & ImGuiWindowFlags_NoMouseInputs))))
            window = window.ParentWindow;
        if (!(window.Flags & ImGuiWindowFlags_NoScrollWithMouse) && !(window.Flags & ImGuiWindowFlags_NoMouseInputs))
        {
            float max_step = window.InnerRect.GetHeight() * 0.67;
            float scroll_step = ImFloor(ImMin(5 * window.CalcFontSize(), max_step));
            SetScrollY(window, window.Scroll.y - wheel_y * scroll_step);
        }
    }

    // Horizontal Mouse Wheel scrolling, or Vertical Mouse Wheel w/ Shift held
    if (wheel_x != 0.0)
    {
        StartLockWheelingWindow(window);
        while ((window.Flags & ImGuiWindowFlags_ChildWindow) && ((window.ScrollMax.x == 0.0) || ((window.Flags & ImGuiWindowFlags_NoScrollWithMouse) && !(window.Flags & ImGuiWindowFlags_NoMouseInputs))))
            window = window.ParentWindow;
        if (!(window.Flags & ImGuiWindowFlags_NoScrollWithMouse) && !(window.Flags & ImGuiWindowFlags_NoMouseInputs))
        {
            float max_step = window.InnerRect.GetWidth() * 0.67;
            float scroll_step = ImFloor(ImMin(2 * window.CalcFontSize(), max_step));
            SetScrollX(window, window.Scroll.x - wheel_x * scroll_step);
        }
    }
}

// The reason this is exposed in imgui_internal.h is: on touch-based system that don't have hovering, we want to dispatch inputs to the right target (imgui vs imgui+app)
void ImGui::UpdateHoveredWindowAndCaptureFlags()
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;
    g.WindowsHoverPadding = ImMax(g.Style.TouchExtraPadding, DimgVec2D::new(WINDOWS_HOVER_PADDING, WINDOWS_HOVER_PADDING));

    // Find the window hovered by mouse:
    // - Child windows can extend beyond the limit of their parent so we need to derive HoveredRootWindow from hovered_window.
    // - When moving a window we can skip the search, which also conveniently bypasses the fact that window->WindowRectClipped is lagging as this point of the frame.
    // - We also support the moved window toggling the NoInputs flag after moving has started in order to be able to detect windows below it, which is useful for e.g. docking mechanisms.
    bool clear_hovered_windows = false;
    FindHoveredWindow();
    IM_ASSERT(g.hovered_window == NULL || g.hovered_window == g.moving_window || g.hovered_window->Viewport == g.mouse_viewport);

    // Modal windows prevents mouse from hovering behind them.
    ImGuiWindow* modal_window = GetTopMostPopupModal();
    if (modal_window && g.hovered_window && !IsWindowWithinBeginStackOf(g.hovered_window->RootWindow, modal_window)) // FIXME-MERGE: root_window_dock_tree ?
        clear_hovered_windows = true;

    // Disabled mouse?
    if (io.ConfigFlags & ImGuiConfigFlags_NoMouse)
        clear_hovered_windows = true;

    // We track click ownership. When clicked outside of a window the click is owned by the application and
    // won't report hovering nor request capture even while dragging over our windows afterward.
    const bool has_open_popup = (g.OpenPopupStack.Size > 0);
    const bool has_open_modal = (modal_window != NULL);
    int mouse_earliest_down = -1;
    bool mouse_any_down = false;
    for (int i = 0; i < IM_ARRAYSIZE(io.mouse_down); i += 1)
    {
        if (io.mouse_clicked[i])
        {
            io.MouseDownOwned[i] = (g.hovered_window != NULL) || has_open_popup;
            io.MouseDownOwnedUnlessPopupClose[i] = (g.hovered_window != NULL) || has_open_modal;
        }
        mouse_any_down |= io.mouse_down[i];
        if (io.mouse_down[i])
            if (mouse_earliest_down == -1 || io.MouseClickedTime[i] < io.MouseClickedTime[mouse_earliest_down])
                mouse_earliest_down = i;
    }
    const bool mouse_avail = (mouse_earliest_down == -1) || io.MouseDownOwned[mouse_earliest_down];
    const bool mouse_avail_unless_popup_close = (mouse_earliest_down == -1) || io.MouseDownOwnedUnlessPopupClose[mouse_earliest_down];

    // If mouse was first clicked outside of ImGui bounds we also cancel out hovering.
    // FIXME: For patterns of drag and drop across OS windows, we may need to rework/remove this test (first committed 311c0ca9 on 2015/02)
    const bool mouse_dragging_extern_payload = g.DragDropActive && (g.DragDropSourceFlags & ImGuiDragDropFlags_SourceExtern) != 0;
    if (!mouse_avail && !mouse_dragging_extern_payload)
        clear_hovered_windows = true;

    if (clear_hovered_windows)
        g.hovered_window = g.HoveredWindowUnderMovingWindow = NULL;

    // Update io.want_capture_mouse for the user application (true = dispatch mouse info to Dear ImGui only, false = dispatch mouse to Dear ImGui + underlying app)
    // Update io.WantCaptureMouseAllowPopupClose (experimental) to give a chance for app to react to popup closure with a drag
    if (g.WantCaptureMouseNextFrame != -1)
    {
        io.WantCaptureMouse = io.WantCaptureMouseUnlessPopupClose = (g.WantCaptureMouseNextFrame != 0);
    }
    else
    {
        io.WantCaptureMouse = (mouse_avail && (g.hovered_window != NULL || mouse_any_down)) || has_open_popup;
        io.WantCaptureMouseUnlessPopupClose = (mouse_avail_unless_popup_close && (g.hovered_window != NULL || mouse_any_down)) || has_open_modal;
    }

    // Update io.want_capture_keyboard for the user application (true = dispatch keyboard info to Dear ImGui only, false = dispatch keyboard info to Dear ImGui + underlying app)
    if (g.WantCaptureKeyboardNextFrame != -1)
        io.WantCaptureKeyboard = (g.WantCaptureKeyboardNextFrame != 0);
    else
        io.WantCaptureKeyboard = (g.active_id != 0) || (modal_window != NULL);
    if (io.NavActive && (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) && !(io.ConfigFlags & ImGuiConfigFlags_NavNoCaptureKeyboard))
        io.WantCaptureKeyboard = true;

    // Update io.want_text_input flag, this is to allow systems without a keyboard (e.g. mobile, hand-held) to show a software keyboard if possible
    io.WantTextInput = (g.WantTextInputNextFrame != -1) ? (g.WantTextInputNextFrame != 0) : false;
}

// [Internal] Do not use directly (can read io.key_mods instead)
ImGuiModFlags ImGui::GetMergedModFlags()
{
    ImGuiContext& g = *GImGui;
    ImGuiModFlags key_mods = ImGuiModFlags_None;
    if (g.io.KeyCtrl)   { key_mods |= ImGuiModFlags_Ctrl; }
    if (g.io.KeyShift)  { key_mods |= ImGuiModFlags_Shift; }
    if (g.io.KeyAlt)    { key_mods |= ImGuiModFlags_Alt; }
    if (g.io.KeySuper)  { key_mods |= ImGuiModFlags_Super; }
    return key_mods;
}

void ImGui::NewFrame()
{
    IM_ASSERT(GImGui != NULL && "No current context. Did you call ImGui::CreateContext() and ImGui::SetCurrentContext() ?");
    ImGuiContext& g = *GImGui;

    // Remove pending delete hooks before frame start.
    // This deferred removal avoid issues of removal while iterating the hook vector
    for (int n = g.Hooks.Size - 1; n >= 0; n--)
        if (g.Hooks[n].Type == ImGuiContextHookType_PendingRemoval_)
            g.Hooks.erase(&g.Hooks[n]);

    CallContextHooks(&g, ImGuiContextHookType_NewFramePre);

    // Check and assert for various common io and Configuration mistakes
    g.ConfigFlagsLastFrame = g.config_flags_curr_frame;
    ErrorCheckNewFrameSanityChecks();
    g.config_flags_curr_frame = g.io.ConfigFlags;

    // Load settings on first frame, save settings when modified (after a delay)
    UpdateSettings();

    g.Time += g.io.DeltaTime;
    g.WithinFrameScope = true;
    g.FrameCount += 1;
    g.TooltipOverrideCount = 0;
    g.WindowsActiveCount = 0;
    g.MenusIdSubmittedThisFrame.resize(0);

    // Calculate frame-rate for the user, as a purely luxurious feature
    g.FramerateSecPerFrameAccum += g.io.DeltaTime - g.FramerateSecPerFrame[g.FramerateSecPerFrameIdx];
    g.FramerateSecPerFrame[g.FramerateSecPerFrameIdx] = g.io.DeltaTime;
    g.FramerateSecPerFrameIdx = (g.FramerateSecPerFrameIdx + 1) % IM_ARRAYSIZE(g.FramerateSecPerFrame);
    g.FramerateSecPerFrameCount = ImMin(g.FramerateSecPerFrameCount + 1, IM_ARRAYSIZE(g.FramerateSecPerFrame));
    g.io.Framerate = (g.FramerateSecPerFrameAccum > 0.0) ? (1.0 / (g.FramerateSecPerFrameAccum / (float)g.FramerateSecPerFrameCount)) : FLT_MAX;

    UpdateViewportsNewFrame();

    // Setup current font and draw list shared data
    // FIXME-VIEWPORT: the concept of a single ClipRectFullscreen is not ideal!
    g.io.Fonts->Locked = true;
    SetCurrentFont(GetDefaultFont());
    IM_ASSERT(g.Font->IsLoaded());
    ImRect virtual_space(FLT_MAX, FLT_MAX, -FLT_MAX, -FLT_MAX);
    for (int n = 0; n < g.Viewports.Size; n += 1)
        virtual_space.Add(g.Viewports[n]->GetMainRect());
    g.DrawListSharedData.ClipRectFullscreen = virtual_space.ToVec4();
    g.DrawListSharedData.CurveTessellationTol = g.Style.CurveTessellationTol;
    g.DrawListSharedData.SetCircleTessellationMaxError(g.Style.CircleTessellationMaxError);
    g.DrawListSharedData.InitialFlags = ImDrawListFlags_None;
    if (g.Style.AntiAliasedLines)
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AntiAliasedLines;
    if (g.Style.AntiAliasedLinesUseTex && !(g.Font->ContainerAtlas.flags & ImFontAtlasFlags_NoBakedLines))
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AntiAliasedLinesUseTex;
    if (g.Style.AntiAliasedFill)
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AntiAliasedFill;
    if (g.io.BackendFlags & ImGuiBackendFlags_RendererHasVtxOffset)
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AllowVtxOffset;

    // Mark rendering data as invalid to prevent user who may have a handle on it to use it.
    for (int n = 0; n < g.Viewports.Size; n += 1)
    {
        ImGuiViewportP* viewport = g.Viewports[n];
        viewport->DrawData = NULL;
        viewport->DrawDataP.Clear();
    }

    // Drag and drop keep the source id alive so even if the source disappear our state is consistent
    if (g.DragDropActive && g.DragDropPayload.SourceId == g.active_id)
        keep_alive_id(g.DragDropPayload.SourceId);

    // Update hovered_id data
    if (!g.HoveredIdPreviousFrame)
        g.HoveredIdTimer = 0.0;
    if (!g.HoveredIdPreviousFrame || (g.hovered_id && g.active_id == g.hovered_id))
        g.HoveredIdNotActiveTimer = 0.0;
    if (g.hovered_id)
        g.HoveredIdTimer += g.io.DeltaTime;
    if (g.hovered_id && g.active_id != g.hovered_id)
        g.HoveredIdNotActiveTimer += g.io.DeltaTime;
    g.HoveredIdPreviousFrame = g.hovered_id;
    g.HoveredIdPreviousFrameUsingMouseWheel = g.HoveredIdUsingMouseWheel;
    g.hovered_id = 0;
    g.HoveredIdAllowOverlap = false;
    g.HoveredIdUsingMouseWheel = false;
    g.HoveredIdDisabled = false;

    // clear ActiveID if the item is not alive anymore.
    // In 1.87, the common most call to keep_alive_id() was moved from GetID() to ItemAdd().
    // As a result, custom widget using ButtonBehavior() _without_ ItemAdd() need to call keep_alive_id() themselves.
    if (g.active_id != 0 && g.ActiveIdIsAlive != g.active_id && g.ActiveIdPreviousFrame == g.active_id)
    {
        IMGUI_DEBUG_LOG_ACTIVEID("NewFrame(): ClearActiveID() because it isn't marked alive anymore!\n");
        clear_active_id();
    }

    // Update active_id data (clear reference to active widget if the widget isn't alive anymore)
    if (g.active_id)
        g.ActiveIdTimer += g.io.DeltaTime;
    g.LastActiveIdTimer += g.io.DeltaTime;
    g.ActiveIdPreviousFrame = g.active_id;
    g.ActiveIdPreviousFrameWindow = g.active_id_window;
    g.ActiveIdPreviousFrameHasBeenEditedBefore = g.ActiveIdHasBeenEditedBefore;
    g.ActiveIdIsAlive = 0;
    g.ActiveIdHasBeenEditedThisFrame = false;
    g.ActiveIdPreviousFrameIsAlive = false;
    g.ActiveIdIsJustActivated = false;
    if (g.TempInputId != 0 && g.active_id != g.TempInputId)
        g.TempInputId = 0;
    if (g.active_id == 0)
    {
        g.ActiveIdUsingNavDirMask = 0x00;
        g.ActiveIdUsingNavInputMask = 0x00;
        g.ActiveIdUsingKeyInputMask.ClearAllBits();
    }

    // Drag and drop
    g.DragDropAcceptIdPrev = g.DragDropAcceptIdCurr;
    g.DragDropAcceptIdCurr = 0;
    g.DragDropAcceptIdCurrRectSurface = FLT_MAX;
    g.DragDropWithinSource = false;
    g.DragDropWithinTarget = false;
    g.DragDropHoldJustPressedId = 0;

    // Close popups on focus lost (currently wip/opt-in)
    //if (g.io.app_focus_lost)
    //    ClosePopupsExceptModals();

    // Process input queue (trickle as many events as possible)
    g.InputEventsTrail.resize(0);
    UpdateInputEvents(g.io.ConfigInputTrickleEventQueue);

    // Update keyboard input state
    UpdateKeyboardInputs();

    //IM_ASSERT(g.io.key_ctrl == IsKeyDown(ImGuiKey_LeftCtrl) || IsKeyDown(ImGuiKey_RightCtrl));
    //IM_ASSERT(g.io.key_shift == IsKeyDown(ImGuiKey_LeftShift) || IsKeyDown(ImGuiKey_RightShift));
    //IM_ASSERT(g.io.key_alt == IsKeyDown(ImGuiKey_LeftAlt) || IsKeyDown(ImGuiKey_RightAlt));
    //IM_ASSERT(g.io.key_super == IsKeyDown(ImGuiKey_LeftSuper) || IsKeyDown(ImGuiKey_RightSuper));

    // Update gamepad/keyboard navigation
    NavUpdate();

    // Update mouse input state
    UpdateMouseInputs();

    // Undocking
    // (needs to be before UpdateMouseMovingWindowNewFrame so the window is already offset and following the mouse on the detaching frame)
    DockContextNewFrameUpdateUndocking(&g);

    // Find hovered window
    // (needs to be before UpdateMouseMovingWindowNewFrame so we fill g.hovered_window_under_moving_window on the mouse release frame)
    UpdateHoveredWindowAndCaptureFlags();

    // Handle user moving window with mouse (at the beginning of the frame to avoid input lag or sheering)
    UpdateMouseMovingWindowNewFrame();

    // Background darkening/whitening
    if (GetTopMostPopupModal() != NULL || (g.NavWindowingTarget != NULL && g.NavWindowingHighlightAlpha > 0.0))
        g.DimBgRatio = ImMin(g.DimBgRatio + g.io.DeltaTime * 6.0, 1.0);
    else
        g.DimBgRatio = ImMax(g.DimBgRatio - g.io.DeltaTime * 10.0, 0.0);

    g.MouseCursor = ImGuiMouseCursor_Arrow;
    g.WantCaptureMouseNextFrame = g.WantCaptureKeyboardNextFrame = g.WantTextInputNextFrame = -1;

    // Platform IME data: reset for the frame
    g.PlatformImeDataPrev = g.PlatformImeData;
    g.PlatformImeData.WantVisible = false;

    // Mouse wheel scrolling, scale
    UpdateMouseWheel();

    // Mark all windows as not visible and compact unused memory.
    IM_ASSERT(g.WindowsFocusOrder.Size <= g.Windows.Size);
    const float memory_compact_start_time = (g.GcCompactAll || g.io.ConfigMemoryCompactTimer < 0.0) ? FLT_MAX : (float)g.Time - g.io.ConfigMemoryCompactTimer;
    for (int i = 0; i != g.Windows.Size; i += 1)
    {
        ImGuiWindow* window = g.Windows[i];
        window.WasActive = window.Active;
        window.BeginCount = 0;
        window.Active = false;
        window.WriteAccessed = false;

        // Garbage collect transient buffers of recently unused windows
        if (!window.WasActive && !window.MemoryCompacted && window.LastTimeActive < memory_compact_start_time)
            GcCompactTransientWindowBuffers(window);
    }

    // Garbage collect transient buffers of recently unused tables
    for (int i = 0; i < g.TablesLastTimeActive.Size; i += 1)
        if (g.TablesLastTimeActive[i] >= 0.0 && g.TablesLastTimeActive[i] < memory_compact_start_time)
            TableGcCompactTransientBuffers(g.Tables.GetByIndex(i));
    for (int i = 0; i < g.TablesTempData.Size; i += 1)
        if (g.TablesTempData[i].LastTimeActive >= 0.0 && g.TablesTempData[i].LastTimeActive < memory_compact_start_time)
            TableGcCompactTransientBuffers(&g.TablesTempData[i]);
    if (g.GcCompactAll)
        GcCompactTransientMiscBuffers();
    g.GcCompactAll = false;

    // Closing the focused window restore focus to the first active root window in descending z-order
    if (g.nav_window && !g.nav_window->WasActive)
        FocusTopMostWindowUnderOne(NULL, NULL);

    // No window should be open at the beginning of the frame.
    // But in order to allow the user to call NewFrame() multiple times without calling Render(), we are doing an explicit clear.
    g.CurrentWindowStack.resize(0);
    g.BeginPopupStack.resize(0);
    g.ItemFlagsStack.resize(0);
    g.ItemFlagsStack.push_back(ImGuiItemFlags_None);
    g.GroupStack.resize(0);

    // Docking
    DockContextNewFrameUpdateDocking(&g);

    // [DEBUG] Update debug features
    UpdateDebugToolItemPicker();
    UpdateDebugToolStackQueries();

    // Create implicit/fallback window - which we will only render it if the user has added something to it.
    // We don't use "Debug" to avoid colliding with user trying to create a "Debug" window with custom flags.
    // This fallback is particularly important as it avoid ImGui:: calls from crashing.
    g.WithinFrameScopeWithImplicitWindow = true;
    SetNextWindowSize(DimgVec2D::new(400, 400), ImGuiCond_FirstUseEver);
    Begin("Debug##Default");
    IM_ASSERT(g.CurrentWindow->IsFallbackWindow == true);

    CallContextHooks(&g, ImGuiContextHookType_NewFramePost);
}

void ImGui::Initialize()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(!g.Initialized && !g.SettingsLoaded);

    // Add .ini handle for ImGuiWindow type
    {
        ImGuiSettingsHandler ini_handler;
        ini_handler.TypeName = "Window";
        ini_handler.TypeHash = ImHashStr("Window");
        ini_handler.ClearAllFn = WindowSettingsHandler_ClearAll;
        ini_handler.ReadOpenFn = WindowSettingsHandler_ReadOpen;
        ini_handler.ReadLineFn = WindowSettingsHandler_ReadLine;
        ini_handler.ApplyAllFn = WindowSettingsHandler_ApplyAll;
        ini_handler.WriteAllFn = WindowSettingsHandler_WriteAll;
        AddSettingsHandler(&ini_handler);
    }

    // Add .ini handle for ImGuiTable type
    TableSettingsAddSettingsHandler();

    // Create default viewport
    ImGuiViewportP* viewport = IM_NEW(ImGuiViewportP)();
    viewport->ID = IMGUI_VIEWPORT_DEFAULT_ID;
    viewport->Idx = 0;
    viewport->PlatformWindowCreated = true;
    viewport.flags = ImGuiViewportFlags_OwnedByApp;
    g.Viewports.push_back(viewport);
    g.TempBuffer.resize(1024 * 3 + 1, 0);
    g.PlatformIO.Viewports.push_back(g.Viewports[0]);

#ifdef IMGUI_HAS_DOCK
    // Initialize Docking
    DockContextInitialize(&g);


    g.Initialized = true;
}

// This function is merely here to free heap allocations.
void ImGui::Shutdown()
{
    // The fonts atlas can be used prior to calling NewFrame(), so we clear it even if g.Initialized is FALSE (which would happen if we never called NewFrame)
    ImGuiContext& g = *GImGui;
    if (g.io.Fonts && g.FontAtlasOwnedByContext)
    {
        g.io.Fonts->Locked = false;
        IM_DELETE(g.io.Fonts);
    }
    g.io.Fonts = NULL;

    // Cleanup of other data are conditional on actually having initialized Dear ImGui.
    if (!g.Initialized)
        return;

    // Save settings (unless we haven't attempted to load them: CreateContext/DestroyContext without a call to NewFrame shouldn't save an empty file)
    if (g.SettingsLoaded && g.io.IniFilename != NULL)
        SaveIniSettingsToDisk(g.io.IniFilename);

    // Destroy platform windows
    DestroyPlatformWindows();

    // Shutdown extensions
    DockContextShutdown(&g);

    CallContextHooks(&g, ImGuiContextHookType_Shutdown);

    // clear everything else
    g.Windows.clear_delete();
    g.WindowsFocusOrder.clear();
    g.WindowsTempSortBuffer.clear();
    g.CurrentWindow = NULL;
    g.CurrentWindowStack.clear();
    g.WindowsById.Clear();
    g.nav_window = NULL;
    g.hovered_window = g.HoveredWindowUnderMovingWindow = NULL;
    g.active_id_window = g.ActiveIdPreviousFrameWindow = NULL;
    g.moving_window = NULL;
    g.ColorStack.clear();
    g.StyleVarStack.clear();
    g.FontStack.clear();
    g.OpenPopupStack.clear();
    g.BeginPopupStack.clear();

    g.CurrentViewport = g.mouse_viewport = g.MouseLastHoveredViewport = NULL;
    g.Viewports.clear_delete();

    g.TabBars.Clear();
    g.CurrentTabBarStack.clear();
    g.ShrinkWidthBuffer.clear();

    g.ClipperTempData.clear_destruct();

    g.Tables.Clear();
    g.TablesTempData.clear_destruct();
    g.DrawChannelsTempMergeBuffer.clear();

    g.ClipboardHandlerData.clear();
    g.MenusIdSubmittedThisFrame.clear();
    g.InputTextState.ClearFreeMemory();

    g.SettingsWindows.clear();
    g.SettingsHandlers.clear();

    if (g.LogFile)
    {
#ifndef IMGUI_DISABLE_TTY_FUNCTIONS
        if (g.LogFile != stdout)

            ImFileClose(g.LogFile);
        g.LogFile = NULL;
    }
    g.LogBuffer.clear();
    g.DebugLogBuf.clear();

    g.Initialized = false;
}

// FIXME: Add a more explicit sort order in the window structure.
static int IMGUI_CDECL ChildWindowComparer(const void* lhs, const void* rhs)
{
    const ImGuiWindow* const a = *(const ImGuiWindow* const *)lhs;
    const ImGuiWindow* const b = *(const ImGuiWindow* const *)rhs;
    if (int d = (a.flags & ImGuiWindowFlags_Popup) - (b.flags & ImGuiWindowFlags_Popup))
        return d;
    if (int d = (a.flags & ImGuiWindowFlags_Tooltip) - (b.flags & ImGuiWindowFlags_Tooltip))
        return d;
    return (a->BeginOrderWithinParent - b->BeginOrderWithinParent);
}

static void AddWindowToSortBuffer(ImVector<ImGuiWindow*>* out_sorted_windows, ImGuiWindow* window)
{
    out_sorted_windows->push_back(window);
    if (window.Active)
    {
        int count = window.DC.ChildWindows.Size;
        ImQsort(window.DC.ChildWindows.Data, count, sizeof(ImGuiWindow*), ChildWindowComparer);
        for (int i = 0; i < count; i += 1)
        {
            ImGuiWindow* child = window.DC.ChildWindows[i];
            if (child->Active)
                AddWindowToSortBuffer(out_sorted_windows, child);
        }
    }
}

static void AddDrawListToDrawData(ImVector<ImDrawList*>* out_list, ImDrawList* draw_list)
{
    if (draw_list->CmdBuffer.Size == 0)
        return;
    if (draw_list->CmdBuffer.Size == 1 && draw_list->CmdBuffer[0].ElemCount == 0 && draw_list->CmdBuffer[0].UserCallback == NULL)
        return;

    // Draw list sanity check. Detect mismatch between PrimReserve() calls and incrementing _VtxCurrentIdx, _VtxWritePtr etc.
    // May trigger for you if you are using PrimXXX functions incorrectly.
    IM_ASSERT(draw_list->VtxBuffer.Size == 0 || draw_list->_VtxWritePtr == draw_list->VtxBuffer.Data + draw_list->VtxBuffer.Size);
    IM_ASSERT(draw_list->IdxBuffer.Size == 0 || draw_list->_IdxWritePtr == draw_list->IdxBuffer.Data + draw_list->IdxBuffer.Size);
    if (!(draw_list.flags & ImDrawListFlags_AllowVtxOffset))
        IM_ASSERT(draw_list->_VtxCurrentIdx == draw_list->VtxBuffer.Size);

    // Check that draw_list doesn't use more vertices than indexable (default ImDrawIdx = unsigned short = 2 bytes = 64K vertices per ImDrawList = per window)
    // If this assert triggers because you are drawing lots of stuff manually:
    // - First, make sure you are coarse clipping yourself and not trying to draw many things outside visible bounds.
    //   Be mindful that the ImDrawList API doesn't filter vertices. Use the Metrics/Debugger window to inspect draw list contents.
    // - If you want large meshes with more than 64K vertices, you can either:
    //   (A) Handle the ImDrawCmd::vtx_offset value in your renderer backend, and set 'io.backend_flags |= ImGuiBackendFlags_RendererHasVtxOffset'.
    //       Most example backends already support this from 1.71. Pre-1.71 backends won't.
    //       Some graphics API such as GL ES 1/2 don't have a way to offset the starting vertex so it is not supported for them.
    //   (B) Or handle 32-bit indices in your renderer backend, and uncomment '#define ImDrawIdx unsigned int' line in imconfig.h.
    //       Most example backends already support this. For example, the OpenGL example code detect index size at compile-time:
    //         glDrawElements(GL_TRIANGLES, (GLsizei)pcmd->elem_count, sizeof(ImDrawIdx) == 2 ? GL_UNSIGNED_SHORT : GL_UNSIGNED_INT, idx_buffer_offset);
    //       Your own engine or render API may use different parameters or function calls to specify index sizes.
    //       2 and 4 bytes indices are generally supported by most graphics API.
    // - If for some reason neither of those solutions works for you, a workaround is to call BeginChild()/EndChild() before reaching
    //   the 64K limit to split your draw commands in multiple draw lists.
    if (sizeof(ImDrawIdx) == 2)
        IM_ASSERT(draw_list->_VtxCurrentIdx < (1 << 16) && "Too many vertices in ImDrawList using 16-bit indices. Read comment above");

    out_list->push_back(draw_list);
}

static void AddWindowToDrawData(ImGuiWindow* window, int layer)
{
    ImGuiContext& g = *GImGui;
    ImGuiViewportP* viewport = window.viewport;
    g.io.MetricsRenderWindows += 1;
    if (window.Flags & ImGuiWindowFlags_DockNodeHost)
        window.DrawList->ChannelsMerge();
    AddDrawListToDrawData(&viewport->DrawDataBuilder.Layers[layer], window.DrawList);
    for (int i = 0; i < window.DC.ChildWindows.Size; i += 1)
    {
        ImGuiWindow* child = window.DC.ChildWindows[i];
        if (IsWindowActiveAndVisible(child)) // Clipped children may have been marked not active
            AddWindowToDrawData(child, layer);
    }
}

static inline int GetWindowDisplayLayer(ImGuiWindow* window)
{
    return (window.Flags & ImGuiWindowFlags_Tooltip) ? 1 : 0;
}

// Layer is locked for the root window, however child windows may use a different viewport (e.g. extruding menu)
static inline void AddRootWindowToDrawData(ImGuiWindow* window)
{
    AddWindowToDrawData(window, GetWindowDisplayLayer(window));
}

void ImDrawDataBuilder::FlattenIntoSingleLayer()
{
    int n = Layers[0].Size;
    int size = n;
    for (int i = 1; i < IM_ARRAYSIZE(Layers); i += 1)
        size += Layers[i].Size;
    Layers[0].resize(size);
    for (int layer_n = 1; layer_n < IM_ARRAYSIZE(Layers); layer_n += 1)
    {
        ImVector<ImDrawList*>& layer = Layers[layer_n];
        if (layer.empty())
            continue;
        memcpy(&Layers[0][n], &layer[0], layer.Size * sizeof(ImDrawList*));
        n += layer.Size;
        layer.resize(0);
    }
}

static void SetupViewportDrawData(ImGuiViewportP* viewport, ImVector<ImDrawList*>* draw_lists)
{
    // When minimized, we report draw_data->display_size as zero to be consistent with non-viewport mode,
    // and to allow applications/backends to easily skip rendering.
    // FIXME: Note that we however do NOT attempt to report "zero drawlist / vertices" into the ImDrawData structure.
    // This is because the work has been done already, and its wasted! We should fix that and add optimizations for
    // it earlier in the pipeline, rather than pretend to hide the data at the end of the pipeline.
    const bool is_minimized = (viewport.flags & ImGuiViewportFlags_Minimized) != 0;

    ImGuiIO& io = ImGui::GetIO();
    ImDrawData* draw_data = &viewport->DrawDataP;
    viewport->DrawData = draw_data; // Make publicly accessible
    draw_data->Valid = true;
    draw_data->CmdLists = (draw_lists->Size > 0) ? draw_lists->Data : NULL;
    draw_data->CmdListsCount = draw_lists->Size;
    draw_data->TotalVtxCount = draw_data->TotalIdxCount = 0;
    draw_data->DisplayPos = viewport.pos;
    draw_data->DisplaySize = is_minimized ? DimgVec2D::new(0.0, 0.0) : viewport->Size;
    draw_data->FramebufferScale = io.DisplayFramebufferScale; // FIXME-VIEWPORT: This may vary on a per-monitor/viewport basis?
    draw_data->OwnerViewport = viewport;
    for (int n = 0; n < draw_lists->Size; n += 1)
    {
        ImDrawList* draw_list = draw_lists->Data[n];
        draw_list->_PopUnusedDrawCmd();
        draw_data->TotalVtxCount += draw_list->VtxBuffer.Size;
        draw_data->TotalIdxCount += draw_list->IdxBuffer.Size;
    }
}

// Push a clipping rectangle for both ImGui logic (hit-testing etc.) and low-level ImDrawList rendering.
// - When using this function it is sane to ensure that float are perfectly rounded to integer values,
//   so that e.g. (max.x-min.x) in user's render produce correct result.
// - If the code here changes, may need to update code of functions like NextColumn() and PushColumnClipRect():
//   some frequently called functions which to modify both channels and clipping simultaneously tend to use the
//   more specialized SetWindowClipRectBeforeSetChannel() to avoid extraneous updates of underlying ImDrawCmds.
void ImGui::PushClipRect(const Vector2D& clip_rect_min, const Vector2D& clip_rect_max, bool intersect_with_current_clip_rect)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DrawList->PushClipRect(clip_rect_min, clip_rect_max, intersect_with_current_clip_rect);
    window.ClipRect = window.DrawList->_ClipRectStack.back();
}

void ImGui::PopClipRect()
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DrawList->PopClipRect();
    window.ClipRect = window.DrawList->_ClipRectStack.back();
}

static ImGuiWindow* FindFrontMostVisibleChildWindow(ImGuiWindow* window)
{
    for (int n = window.DC.ChildWindows.Size - 1; n >= 0; n--)
        if (IsWindowActiveAndVisible(window.DC.ChildWindows[n]))
            return FindFrontMostVisibleChildWindow(window.DC.ChildWindows[n]);
    return window;
}

static void ImGui::RenderDimmedBackgroundBehindWindow(ImGuiWindow* window, ImU32 col)
{
    if ((col & IM_COL32_A_MASK) == 0)
        return;

    ImGuiViewportP* viewport = window.viewport;
    ImRect viewport_rect = viewport->GetMainRect();

    // Draw behind window by moving the draw command at the FRONT of the draw list
    {
        // We've already called AddWindowToDrawData() which called draw_list->ChannelsMerge() on DockNodeHost windows,
        // and draw list have been trimmed already, hence the explicit recreation of a draw command if missing.
        // FIXME: This is creating complication, might be simpler if we could inject a drawlist in drawdata at a given position and not attempt to manipulate ImDrawCmd order.
        ImDrawList* draw_list = window.RootWindowDockTree->DrawList;
        if (draw_list->CmdBuffer.Size == 0)
            draw_list->AddDrawCmd();
        draw_list->PushClipRect(viewport_rect.Min - DimgVec2D::new(1, 1), viewport_rect.Max + DimgVec2D::new(1, 1), false); // Ensure ImDrawCmd are not merged
        draw_list->AddRectFilled(viewport_rect.Min, viewport_rect.Max, col);
        ImDrawCmd cmd = draw_list->CmdBuffer.back();
        IM_ASSERT(cmd.ElemCount == 6);
        draw_list->CmdBuffer.pop_back();
        draw_list->CmdBuffer.push_front(cmd);
        draw_list->PopClipRect();
        draw_list->AddDrawCmd(); // We need to create a command as cmd_buffer.back().idx_offset won't be correct if we append to same command.
    }

    // Draw over sibling docking nodes in a same docking tree
    if (window.RootWindow->DockIsActive)
    {
        ImDrawList* draw_list = FindFrontMostVisibleChildWindow(window.RootWindowDockTree)->DrawList;
        if (draw_list->CmdBuffer.Size == 0)
            draw_list->AddDrawCmd();
        draw_list->PushClipRect(viewport_rect.Min, viewport_rect.Max, false);
        RenderRectFilledWithHole(draw_list, window.RootWindowDockTree->Rect(), window.RootWindow->Rect(), col, 0.0);// window->root_window_dock_tree->window_rounding);
        draw_list->PopClipRect();
    }
}

ImGuiWindow* ImGui::FindBottomMostVisibleWindowWithinBeginStack(ImGuiWindow* parent_window)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* bottom_most_visible_window = parent_window;
    for (int i = FindWindowDisplayIndex(parent_window); i >= 0; i--)
    {
        ImGuiWindow* window = g.Windows[i];
        if (window.Flags & ImGuiWindowFlags_ChildWindow)
            continue;
        if (!IsWindowWithinBeginStackOf(window, parent_window))
            break;
        if (IsWindowActiveAndVisible(window) && GetWindowDisplayLayer(window) <= GetWindowDisplayLayer(parent_window))
            bottom_most_visible_window = window;
    }
    return bottom_most_visible_window;
}

static void ImGui::RenderDimmedBackgrounds()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* modal_window = GetTopMostAndVisiblePopupModal();
    if (g.DimBgRatio <= 0.0 && g.NavWindowingHighlightAlpha <= 0.0)
        return;
    const bool dim_bg_for_modal = (modal_window != NULL);
    const bool dim_bg_for_window_list = (g.NavWindowingTargetAnim != NULL && g.NavWindowingTargetAnim->Active);
    if (!dim_bg_for_modal && !dim_bg_for_window_list)
        return;

    ImGuiViewport* viewports_already_dimmed[2] = { NULL, NULL };
    if (dim_bg_for_modal)
    {
        // Draw dimming behind modal or a begin stack child, whichever comes first in draw order.
        ImGuiWindow* dim_behind_window = FindBottomMostVisibleWindowWithinBeginStack(modal_window);
        RenderDimmedBackgroundBehindWindow(dim_behind_window, GetColorU32(ImGuiCol_ModalWindowDimBg, g.DimBgRatio));
        viewports_already_dimmed[0] = modal_window.viewport;
    }
    else if (dim_bg_for_window_list)
    {
        // Draw dimming behind CTRL+Tab target window and behind CTRL+Tab UI window
        RenderDimmedBackgroundBehindWindow(g.NavWindowingTargetAnim, GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio));
        if (g.NavWindowingListWindow != NULL && g.NavWindowingListWindow->Viewport && g.NavWindowingListWindow->Viewport != g.NavWindowingTargetAnim->Viewport)
            RenderDimmedBackgroundBehindWindow(g.NavWindowingListWindow, GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio));
        viewports_already_dimmed[0] = g.NavWindowingTargetAnim->Viewport;
        viewports_already_dimmed[1] = g.NavWindowingListWindow ? g.NavWindowingListWindow->Viewport : NULL;

        // Draw border around CTRL+Tab target window
        ImGuiWindow* window = g.NavWindowingTargetAnim;
        ImGuiViewport* viewport = window.viewport;
        float distance = g.FontSize;
        ImRect bb = window.Rect();
        bb.Expand(distance);
        if (bb.GetWidth() >= viewport->Size.x && bb.GetHeight() >= viewport->Size.y)
            bb.Expand(-distance - 1.0); // If a window fits the entire viewport, adjust its highlight inward
        if (window.DrawList->CmdBuffer.Size == 0)
            window.DrawList->AddDrawCmd();
        window.DrawList->PushClipRect(viewport.pos, viewport.pos + viewport->Size);
        window.DrawList->AddRect(bb.Min, bb.Max, GetColorU32(ImGuiCol_NavWindowingHighlight, g.NavWindowingHighlightAlpha), window.WindowRounding, 0, 3.0);
        window.DrawList->PopClipRect();
    }

    // Draw dimming background on _other_ viewports than the ones our windows are in
    for (int viewport_n = 0; viewport_n < g.Viewports.Size; viewport_n += 1)
    {
        ImGuiViewportP* viewport = g.Viewports[viewport_n];
        if (viewport == viewports_already_dimmed[0] || viewport == viewports_already_dimmed[1])
            continue;
        if (modal_window && viewport->Window && IsWindowAbove(viewport->Window, modal_window))
            continue;
        ImDrawList* draw_list = GetForegroundDrawList(viewport);
        const ImU32 dim_bg_col = GetColorU32(dim_bg_for_modal ? ImGuiCol_ModalWindowDimBg : ImGuiCol_NavWindowingDimBg, g.DimBgRatio);
        draw_list->AddRectFilled(viewport.pos, viewport.pos + viewport->Size, dim_bg_col);
    }
}

// This is normally called by Render(). You may want to call it directly if you want to avoid calling Render() but the gain will be very minimal.
void ImGui::EndFrame()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.Initialized);

    // Don't process EndFrame() multiple times.
    if (g.FrameCountEnded == g.FrameCount)
        return;
    IM_ASSERT(g.WithinFrameScope && "Forgot to call ImGui::NewFrame()?");

    CallContextHooks(&g, ImGuiContextHookType_EndFramePre);

    ErrorCheckEndFrameSanityChecks();

    // Notify Platform/OS when our Input Method Editor cursor has moved (e.g. CJK inputs using Microsoft IME)
    if (g.io.SetPlatformImeDataFn && memcmp(&g.PlatformImeData, &g.PlatformImeDataPrev, sizeof(ImGuiPlatformImeData)) != 0)
    {
        ImGuiViewport* viewport = FindViewportByID(g.PlatformImeViewport);
        g.io.SetPlatformImeDataFn(viewport ? viewport : GetMainViewport(), &g.PlatformImeData);
    }

    // Hide implicit/fallback "Debug" window if it hasn't been used
    g.WithinFrameScopeWithImplicitWindow = false;
    if (g.CurrentWindow && !g.CurrentWindow->WriteAccessed)
        g.CurrentWindow->Active = false;
    End();

    // Update navigation: CTRL+Tab, wrap-around requests
    NavEndFrame();

    // Update docking
    DockContextEndFrame(&g);

    SetCurrentViewport(NULL, NULL);

    // Drag and Drop: Elapse payload (if delivered, or if source stops being submitted)
    if (g.DragDropActive)
    {
        bool is_delivered = g.DragDropPayload.Delivery;
        bool is_elapsed = (g.DragDropPayload.DataFrameCount + 1 < g.FrameCount) && ((g.DragDropSourceFlags & ImGuiDragDropFlags_SourceAutoExpirePayload) || !IsMouseDown(g.DragDropMouseButton));
        if (is_delivered || is_elapsed)
            ClearDragDrop();
    }

    // Drag and Drop: Fallback for source tooltip. This is not ideal but better than nothing.
    if (g.DragDropActive && g.DragDropSourceFrameCount < g.FrameCount && !(g.DragDropSourceFlags & ImGuiDragDropFlags_SourceNoPreviewTooltip))
    {
        g.DragDropWithinSource = true;
        SetTooltip("...");
        g.DragDropWithinSource = false;
    }

    // End frame
    g.WithinFrameScope = false;
    g.FrameCountEnded = g.FrameCount;

    // Initiate moving window + handle left-click and right-click focus
    UpdateMouseMovingWindowEndFrame();

    // Update user-facing viewport list (g.viewports -> g.platform_io.viewports after filtering out some)
    UpdateViewportsEndFrame();

    // Sort the window list so that all child windows are after their parent
    // We cannot do that on focus_window() because children may not exist yet
    g.WindowsTempSortBuffer.resize(0);
    g.WindowsTempSortBuffer.reserve(g.Windows.Size);
    for (int i = 0; i != g.Windows.Size; i += 1)
    {
        ImGuiWindow* window = g.Windows[i];
        if (window.Active && (window.Flags & ImGuiWindowFlags_ChildWindow))       // if a child is active its parent will add it
            continue;
        AddWindowToSortBuffer(&g.WindowsTempSortBuffer, window);
    }

    // This usually assert if there is a mismatch between the ImGuiWindowFlags_ChildWindow / ParentWindow values and dc.ChildWindows[] in parents, aka we've done something wrong.
    IM_ASSERT(g.Windows.Size == g.WindowsTempSortBuffer.Size);
    g.Windows.swap(g.WindowsTempSortBuffer);
    g.io.MetricsActiveWindows = g.WindowsActiveCount;

    // Unlock font atlas
    g.io.Fonts->Locked = false;

    // clear Input data for next frame
    g.io.MouseWheel = g.io.MouseWheelH = 0.0;
    g.io.InputQueueCharacters.resize(0);
    memset(g.io.NavInputs, 0, sizeof(g.io.NavInputs));

    CallContextHooks(&g, ImGuiContextHookType_EndFramePost);
}

// Prepare the data for rendering so you can call GetDrawData()
// (As with anything within the ImGui:: namspace this doesn't touch your GPU or graphics API at all:
// it is the role of the ImGui_ImplXXXX_RenderDrawData() function provided by the renderer backend)
void ImGui::Render()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.Initialized);

    if (g.FrameCountEnded != g.FrameCount)
        EndFrame();
    const bool first_render_of_frame = (g.FrameCountRendered != g.FrameCount);
    g.FrameCountRendered = g.FrameCount;
    g.io.MetricsRenderWindows = 0;

    CallContextHooks(&g, ImGuiContextHookType_RenderPre);

    // Add background ImDrawList (for each active viewport)
    for (int n = 0; n != g.Viewports.Size; n += 1)
    {
        ImGuiViewportP* viewport = g.Viewports[n];
        viewport->DrawDataBuilder.Clear();
        if (viewport->DrawLists[0] != NULL)
            AddDrawListToDrawData(&viewport->DrawDataBuilder.Layers[0], GetBackgroundDrawList(viewport));
    }

    // Add ImDrawList to render
    ImGuiWindow* windows_to_render_top_most[2];
    windows_to_render_top_most[0] = (g.NavWindowingTarget && !(g.NavWindowingTarget.flags & ImGuiWindowFlags_NoBringToFrontOnFocus)) ? g.NavWindowingTarget->RootWindowDockTree : NULL;
    windows_to_render_top_most[1] = (g.NavWindowingTarget ? g.NavWindowingListWindow : NULL);
    for (int n = 0; n != g.Windows.Size; n += 1)
    {
        ImGuiWindow* window = g.Windows[n];
        IM_MSVC_WARNING_SUPPRESS(6011); // Static Analysis false positive "warning C6011: Dereferencing NULL pointer 'window'"
        if (IsWindowActiveAndVisible(window) && (window.Flags & ImGuiWindowFlags_ChildWindow) == 0 && window != windows_to_render_top_most[0] && window != windows_to_render_top_most[1])
            AddRootWindowToDrawData(window);
    }
    for (int n = 0; n < IM_ARRAYSIZE(windows_to_render_top_most); n += 1)
        if (windows_to_render_top_most[n] && IsWindowActiveAndVisible(windows_to_render_top_most[n])) // nav_windowing_target is always temporarily displayed as the top-most window
            AddRootWindowToDrawData(windows_to_render_top_most[n]);

    // Draw modal/window whitening backgrounds
    if (first_render_of_frame)
        RenderDimmedBackgrounds();

    // Draw software mouse cursor if requested by io.mouse_draw_cursor flag
    if (g.io.MouseDrawCursor && first_render_of_frame && g.MouseCursor != ImGuiMouseCursor_None)
        RenderMouseCursor(g.io.MousePos, g.Style.MouseCursorScale, g.MouseCursor, IM_COL32_WHITE, IM_COL32_BLACK, IM_COL32(0, 0, 0, 48));

    // Setup ImDrawData structures for end-user
    g.io.MetricsRenderVertices = g.io.MetricsRenderIndices = 0;
    for (int n = 0; n < g.Viewports.Size; n += 1)
    {
        ImGuiViewportP* viewport = g.Viewports[n];
        viewport->DrawDataBuilder.FlattenIntoSingleLayer();

        // Add foreground ImDrawList (for each active viewport)
        if (viewport->DrawLists[1] != NULL)
            AddDrawListToDrawData(&viewport->DrawDataBuilder.Layers[0], GetForegroundDrawList(viewport));

        SetupViewportDrawData(viewport, &viewport->DrawDataBuilder.Layers[0]);
        ImDrawData* draw_data = viewport->DrawData;
        g.io.MetricsRenderVertices += draw_data->TotalVtxCount;
        g.io.MetricsRenderIndices += draw_data->TotalIdxCount;
    }

    CallContextHooks(&g, ImGuiContextHookType_RenderPost);
}

// Calculate text size. Text can be multi-line. Optionally ignore text after a ## marker.
// CalcTextSize("") should return Vector2D(0.0, g.font_size)
Vector2D ImGui::CalcTextSize(const char* text, const char* text_end, bool hide_text_after_double_hash, float wrap_width)
{
    ImGuiContext& g = *GImGui;

    const char* text_display_end;
    if (hide_text_after_double_hash)
        text_display_end = FindRenderedTextEnd(text, text_end);      // Hide anything after a '##' string
    else
        text_display_end = text_end;

    ImFont* font = g.Font;
    const float font_size = g.FontSize;
    if (text == text_display_end)
        return DimgVec2D::new(0.0, font_size);
    Vector2D text_size = font->CalcTextSizeA(font_size, FLT_MAX, wrap_width, text, text_display_end, NULL);

    // Round
    // FIXME: This has been here since Dec 2015 (7b0bf230) but down the line we want this out.
    // FIXME: Investigate using ceilf or e.g.
    // - https://git.musl-libc.org/cgit/musl/tree/src/math/ceilf.c
    // - https://embarkstudios.github.io/rust-gpu/api/src/libm/math/ceilf.rs.html
    text_size.x = IM_FLOOR(text_size.x + 0.99999);

    return text_size;
}

// Find window given position, search front-to-back
// FIXME: Note that we have an inconsequential lag here: outer_rect_clipped is updated in Begin(), so windows moved programmatically
// with set_window_pos() and not SetNextWindowPos() will have that rectangle lagging by a frame at the time FindHoveredWindow() is
// called, aka before the next Begin(). Moving window isn't affected.
static void FindHoveredWindow()
{
    ImGuiContext& g = *GImGui;

    // Special handling for the window being moved: Ignore the mouse viewport check (because it may reset/lose its viewport during the undocking frame)
    ImGuiViewportP* moving_window_viewport = g.moving_window ? g.moving_window->Viewport : NULL;
    if (g.moving_window)
        g.moving_window->Viewport = g.mouse_viewport;

    ImGuiWindow* hovered_window = NULL;
    ImGuiWindow* hovered_window_ignoring_moving_window = NULL;
    if (g.moving_window && !(g.moving_window.flags & ImGuiWindowFlags_NoMouseInputs))
        hovered_window = g.moving_window;

    Vector2D padding_regular = g.Style.TouchExtraPadding;
    Vector2D padding_for_resize = g.io.ConfigWindowsResizeFromEdges ? g.WindowsHoverPadding : padding_regular;
    for (int i = g.Windows.Size - 1; i >= 0; i--)
    {
        ImGuiWindow* window = g.Windows[i];
        IM_MSVC_WARNING_SUPPRESS(28182); // [Static Analyzer] Dereferencing NULL pointer.
        if (!window.Active || window.Hidden)
            continue;
        if (window.Flags & ImGuiWindowFlags_NoMouseInputs)
            continue;
        IM_ASSERT(window.viewport);
        if (window.viewport != g.mouse_viewport)
            continue;

        // Using the clipped AABB, a child window will typically be clipped by its parent (not always)
        ImRect bb(window.OuterRectClipped);
        if (window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_AlwaysAutoResize))
            bb.Expand(padding_regular);
        else
            bb.Expand(padding_for_resize);
        if (!bb.Contains(g.io.MousePos))
            continue;

        // Support for one rectangular hole in any given window
        // FIXME: Consider generalizing hit-testing override (with more generic data, callback, etc.) (#1512)
        if (window.HitTestHoleSize.x != 0)
        {
            Vector2D hole_pos(window.Pos.x + (float)window.HitTestHoleOffset.x, window.Pos.y + (float)window.HitTestHoleOffset.y);
            Vector2D hole_size((float)window.HitTestHoleSize.x, (float)window.HitTestHoleSize.y);
            if (ImRect(hole_pos, hole_pos + hole_size).Contains(g.io.MousePos))
                continue;
        }

        if (hovered_window == NULL)
            hovered_window = window;
        IM_MSVC_WARNING_SUPPRESS(28182); // [Static Analyzer] Dereferencing NULL pointer.
        if (hovered_window_ignoring_moving_window == NULL && (!g.moving_window || window.RootWindowDockTree != g.moving_window->RootWindowDockTree))
            hovered_window_ignoring_moving_window = window;
        if (hovered_window && hovered_window_ignoring_moving_window)
            break;
    }

    g.hovered_window = hovered_window;
    g.HoveredWindowUnderMovingWindow = hovered_window_ignoring_moving_window;

    if (g.moving_window)
        g.moving_window->Viewport = moving_window_viewport;
}

bool ImGui::IsItemActive()
{
    ImGuiContext& g = *GImGui;
    if (g.active_id)
        return g.active_id == g.last_item_data.ID;
    return false;
}

bool ImGui::IsItemActivated()
{
    ImGuiContext& g = *GImGui;
    if (g.active_id)
        if (g.active_id == g.last_item_data.ID && g.ActiveIdPreviousFrame != g.last_item_data.ID)
            return true;
    return false;
}

bool ImGui::IsItemDeactivated()
{
    ImGuiContext& g = *GImGui;
    if (g.last_item_data.StatusFlags & ImGuiItemStatusFlags_HasDeactivated)
        return (g.last_item_data.StatusFlags & ImGuiItemStatusFlags_Deactivated) != 0;
    return (g.ActiveIdPreviousFrame == g.last_item_data.ID && g.ActiveIdPreviousFrame != 0 && g.active_id != g.last_item_data.ID);
}

bool ImGui::IsItemDeactivatedAfterEdit()
{
    ImGuiContext& g = *GImGui;
    return IsItemDeactivated() && (g.ActiveIdPreviousFrameHasBeenEditedBefore || (g.active_id == 0 && g.ActiveIdHasBeenEditedBefore));
}

// == GetItemID() == GetFocusID()
bool ImGui::IsItemFocused()
{
    ImGuiContext& g = *GImGui;
    if (g.NavId != g.last_item_data.ID || g.NavId == 0)
        return false;

    // Special handling for the dummy item after Begin() which represent the title bar or tab.
    // When the window is collapsed (skip_items==true) that last item will never be overwritten so we need to detect the case.
    ImGuiWindow* window = g.CurrentWindow;
    if (g.last_item_data.ID == window.ID && window.WriteAccessed)
        return false;

    return true;
}

// Important: this can be useful but it is NOT equivalent to the behavior of e.g.Button()!
// Most widgets have specific reactions based on mouse-up/down state, mouse position etc.
bool ImGui::IsItemClicked(ImGuiMouseButton mouse_button)
{
    return IsMouseClicked(mouse_button) && IsItemHovered(ImGuiHoveredFlags_None);
}

bool ImGui::IsItemToggledOpen()
{
    ImGuiContext& g = *GImGui;
    return (g.last_item_data.StatusFlags & ImGuiItemStatusFlags_ToggledOpen) ? true : false;
}

bool ImGui::IsItemToggledSelection()
{
    ImGuiContext& g = *GImGui;
    return (g.last_item_data.StatusFlags & ImGuiItemStatusFlags_ToggledSelection) ? true : false;
}

bool ImGui::IsAnyItemHovered()
{
    ImGuiContext& g = *GImGui;
    return g.hovered_id != 0 || g.HoveredIdPreviousFrame != 0;
}

bool ImGui::IsAnyItemActive()
{
    ImGuiContext& g = *GImGui;
    return g.active_id != 0;
}

bool ImGui::IsAnyItemFocused()
{
    ImGuiContext& g = *GImGui;
    return g.NavId != 0 && !g.NavDisableHighlight;
}

bool ImGui::IsItemVisible()
{
    ImGuiContext& g = *GImGui;
    return g.CurrentWindow->ClipRect.Overlaps(g.last_item_data.Rect);
}

bool ImGui::IsItemEdited()
{
    ImGuiContext& g = *GImGui;
    return (g.last_item_data.StatusFlags & ImGuiItemStatusFlags_Edited) != 0;
}

// Allow last item to be overlapped by a subsequent item. Both may be activated during the same frame before the later one takes priority.
// FIXME: Although this is exposed, its interaction and ideal idiom with using ImGuiButtonFlags_AllowItemOverlap flag are extremely confusing, need rework.
void ImGui::SetItemAllowOverlap()
{
    ImGuiContext& g = *GImGui;
    ImGuiID id = g.last_item_data.ID;
    if (g.hovered_id == id)
        g.HoveredIdAllowOverlap = true;
    if (g.active_id == id)
        g.ActiveIdAllowOverlap = true;
}

void ImGui::SetItemUsingMouseWheel()
{
    ImGuiContext& g = *GImGui;
    ImGuiID id = g.last_item_data.ID;
    if (g.hovered_id == id)
        g.HoveredIdUsingMouseWheel = true;
    if (g.active_id == id)
        g.ActiveIdUsingMouseWheel = true;
}

void ImGui::SetActiveIdUsingNavAndKeys()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.active_id != 0);
    g.ActiveIdUsingNavDirMask = ~0;
    g.ActiveIdUsingNavInputMask = ~0;
    g.ActiveIdUsingKeyInputMask.SetAllBits();
    NavMoveRequestCancel();
}

Vector2D ImGui::GetItemRectMin()
{
    ImGuiContext& g = *GImGui;
    return g.last_item_data.Rect.Min;
}

Vector2D ImGui::GetItemRectMax()
{
    ImGuiContext& g = *GImGui;
    return g.last_item_data.Rect.Max;
}

Vector2D ImGui::GetItemRectSize()
{
    ImGuiContext& g = *GImGui;
    return g.last_item_data.Rect.GetSize();
}

bool ImGui::BeginChildEx(const char* name, ImGuiID id, const Vector2D& size_arg, bool border, ImGuiWindowFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* parent_window = g.CurrentWindow;

    flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoDocking;
    flags |= (parent_window.Flags & ImGuiWindowFlags_NoMove);  // Inherit the NoMove flag

    // size
    const Vector2D content_avail = GetContentRegionAvail();
    Vector2D size = ImFloor(size_arg);
    const int auto_fit_axises = ((size.x == 0.0) ? (1 << ImGuiAxis_X) : 0x00) | ((size.y == 0.0) ? (1 << ImGuiAxis_Y) : 0x00);
    if (size.x <= 0.0)
        size.x = ImMax(content_avail.x + size.x, 4.0); // Arbitrary minimum child size (0.0 causing too much issues)
    if (size.y <= 0.0)
        size.y = ImMax(content_avail.y + size.y, 4.0);
    SetNextWindowSize(size);

    // build up name. If you need to append to a same child from multiple location in the id stack, use BeginChild(ImGuiID id) with a stable value.
    const char* temp_window_name;
    if (name)
        ImFormatStringToTempBuffer(&temp_window_name, NULL, "%s/%s_%08X", parent_window.Name, name, id);
    else
        ImFormatStringToTempBuffer(&temp_window_name, NULL, "%s/%08X", parent_window.Name, id);

    const float backup_border_size = g.Style.ChildBorderSize;
    if (!border)
        g.Style.ChildBorderSize = 0.0;
    bool ret = Begin(temp_window_name, NULL, flags);
    g.Style.ChildBorderSize = backup_border_size;

    ImGuiWindow* child_window = g.CurrentWindow;
    child_window.ChildId = id;
    child_window.AutoFitChildAxises = (ImS8)auto_fit_axises;

    // Set the cursor to handle case where the user called SetNextWindowPos()+BeginChild() manually.
    // While this is not really documented/defined, it seems that the expected thing to do.
    if (child_window.BeginCount == 1)
        parent_window.DC.CursorPos = child_window.Pos;

    // Process navigation-in immediately so NavInit can run on first frame
    if (g.NavActivateId == id && !(flags & ImGuiWindowFlags_NavFlattened) && (child_window.DC.NavLayersActiveMask != 0 || child_window.DC.NavHasScroll))
    {
        focus_window(child_window);
        NavInitWindow(child_window, false);
        SetActiveID(id + 1, child_window); // Steal active_id with another arbitrary id so that key-press won't activate child item
        g.ActiveIdSource = ImGuiInputSource_Nav;
    }
    return ret;
}

bool ImGui::BeginChild(const char* str_id, const Vector2D& size_arg, bool border, ImGuiWindowFlags extra_flags)
{
    ImGuiWindow* window = GetCurrentWindow();
    return BeginChildEx(str_id, window.GetID(str_id), size_arg, border, extra_flags);
}

bool ImGui::BeginChild(ImGuiID id, const Vector2D& size_arg, bool border, ImGuiWindowFlags extra_flags)
{
    IM_ASSERT(id != 0);
    return BeginChildEx(NULL, id, size_arg, border, extra_flags);
}

void ImGui::EndChild()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;

    IM_ASSERT(g.WithinEndChild == false);
    IM_ASSERT(window.Flags & ImGuiWindowFlags_ChildWindow);   // Mismatched BeginChild()/EndChild() calls

    g.WithinEndChild = true;
    if (window.BeginCount > 1)
    {
        End();
    }
    else
    {
        Vector2D sz = window.Size;
        if (window.AutoFitChildAxises & (1 << ImGuiAxis_X)) // Arbitrary minimum zero-ish child size of 4.0 causes less trouble than a 0.0
            sz.x = ImMax(4.0, sz.x);
        if (window.AutoFitChildAxises & (1 << ImGuiAxis_Y))
            sz.y = ImMax(4.0, sz.y);
        End();

        ImGuiWindow* parent_window = g.CurrentWindow;
        ImRect bb(parent_window.DC.CursorPos, parent_window.DC.CursorPos + sz);
        ItemSize(sz);
        if ((window.DC.NavLayersActiveMask != 0 || window.DC.NavHasScroll) && !(window.Flags & ImGuiWindowFlags_NavFlattened))
        {
            ItemAdd(bb, window.ChildId);
            RenderNavHighlight(bb, window.ChildId);

            // When browsing a window that has no activable items (scroll only) we keep a highlight on the child (pass g.nav_id to trick into always displaying)
            if (window.DC.NavLayersActiveMask == 0 && window == g.nav_window)
                RenderNavHighlight(ImRect(bb.Min - DimgVec2D::new(2, 2), bb.Max + DimgVec2D::new(2, 2)), g.NavId, ImGuiNavHighlightFlags_TypeThin);
        }
        else
        {
            // Not navigable into
            ItemAdd(bb, 0);
        }
        if (g.hovered_window == window)
            g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_HoveredWindow;
    }
    g.WithinEndChild = false;
    g.LogLinePosY = -FLT_MAX; // To enforce a carriage return
}

// Helper to create a child window / scrolling region that looks like a normal widget frame.
bool ImGui::BeginChildFrame(ImGuiID id, const Vector2D& size, ImGuiWindowFlags extra_flags)
{
    ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    PushStyleColor(ImGuiCol_ChildBg, style.Colors[ImGuiCol_FrameBg]);
    PushStyleVar(ImGuiStyleVar_ChildRounding, style.FrameRounding);
    PushStyleVar(ImGuiStyleVar_ChildBorderSize, style.FrameBorderSize);
    PushStyleVar(ImGuiStyleVar_WindowPadding, style.FramePadding);
    bool ret = BeginChild(id, size, true, ImGuiWindowFlags_NoMove | ImGuiWindowFlags_AlwaysUseWindowPadding | extra_flags);
    PopStyleVar(3);
    PopStyleColor();
    return ret;
}

void ImGui::EndChildFrame()
{
    EndChild();
}

static void SetWindowConditionAllowFlags(ImGuiWindow* window, ImGuiCond flags, bool enabled)
{
    window.SetWindowPosAllowFlags       = enabled ? (window.SetWindowPosAllowFlags       | flags) : (window.SetWindowPosAllowFlags       & ~flags);
    window.SetWindowSizeAllowFlags      = enabled ? (window.SetWindowSizeAllowFlags      | flags) : (window.SetWindowSizeAllowFlags      & ~flags);
    window.SetWindowCollapsedAllowFlags = enabled ? (window.SetWindowCollapsedAllowFlags | flags) : (window.SetWindowCollapsedAllowFlags & ~flags);
    window.SetWindowDockAllowFlags      = enabled ? (window.SetWindowDockAllowFlags      | flags) : (window.SetWindowDockAllowFlags      & ~flags);
}

ImGuiWindow* ImGui::FindWindowByID(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    return (ImGuiWindow*)g.WindowsById.GetVoidPtr(id);
}

ImGuiWindow* ImGui::FindWindowByName(const char* name)
{
    ImGuiID id = ImHashStr(name);
    return FindWindowByID(id);
}

static void ApplyWindowSettings(ImGuiWindow* window, ImGuiWindowSettings* settings)
{
    const ImGuiViewport* main_viewport = ImGui::GetMainViewport();
    window.ViewportPos = main_viewport.pos;
    if (settings->ViewportId)
    {
        window.ViewportId = settings->ViewportId;
        window.ViewportPos = DimgVec2D::new(settings->ViewportPos.x, settings->ViewportPos.y);
    }
    window.Pos = ImFloor(DimgVec2D::new(settings.pos.x + window.ViewportPos.x, settings.pos.y + window.ViewportPos.y));
    if (settings->Size.x > 0 && settings->Size.y > 0)
        window.Size = window.SizeFull = ImFloor(DimgVec2D::new(settings->Size.x, settings->Size.y));
    window.Collapsed = settings->Collapsed;
    window.DockId = settings->DockId;
    window.DockOrder = settings->DockOrder;
}

static void UpdateWindowInFocusOrderList(ImGuiWindow* window, bool just_created, ImGuiWindowFlags new_flags)
{
    ImGuiContext& g = *GImGui;

    const bool new_is_explicit_child = (new_flags & ImGuiWindowFlags_ChildWindow) != 0;
    const bool child_flag_changed = new_is_explicit_child != window.IsExplicitChild;
    if ((just_created || child_flag_changed) && !new_is_explicit_child)
    {
        IM_ASSERT(!g.WindowsFocusOrder.contains(window));
        g.WindowsFocusOrder.push_back(window);
        window.FocusOrder = (short)(g.WindowsFocusOrder.Size - 1);
    }
    else if (!just_created && child_flag_changed && new_is_explicit_child)
    {
        IM_ASSERT(g.WindowsFocusOrder[window.FocusOrder] == window);
        for (int n = window.FocusOrder + 1; n < g.WindowsFocusOrder.Size; n += 1)
            g.WindowsFocusOrder[n]->FocusOrder--;
        g.WindowsFocusOrder.erase(g.WindowsFocusOrder.Data + window.FocusOrder);
        window.FocusOrder = -1;
    }
    window.IsExplicitChild = new_is_explicit_child;
}

static ImGuiWindow* CreateNewWindow(const char* name, ImGuiWindowFlags flags)
{
    ImGuiContext& g = *GImGui;
    //IMGUI_DEBUG_LOG("CreateNewWindow '%s', flags = 0x%08X\n", name, flags);

    // Create window the first time
    ImGuiWindow* window = IM_NEW(ImGuiWindow)(&g, name);
    window.Flags = flags;
    g.WindowsById.SetVoidPtr(window.ID, window);

    // Default/arbitrary window position. Use SetNextWindowPos() with the appropriate condition flag to change the initial position of a window.
    const ImGuiViewport* main_viewport = ImGui::GetMainViewport();
    window.Pos = main_viewport.pos + DimgVec2D::new(60, 60);
    window.ViewportPos = main_viewport.pos;

    // User can disable loading and saving of settings. Tooltip and child windows also don't store settings.
    if (!(flags & ImGuiWindowFlags_NoSavedSettings))
        if (ImGuiWindowSettings* settings = ImGui::FindWindowSettings(window.ID))
        {
            // Retrieve settings from .ini file
            window.SettingsOffset = g.SettingsWindows.offset_from_ptr(settings);
            SetWindowConditionAllowFlags(window, ImGuiCond_FirstUseEver, false);
            ApplyWindowSettings(window, settings);
        }
    window.DC.CursorStartPos = window.DC.CursorMaxPos = window.DC.IdealMaxPos = window.Pos; // So first call to CalcWindowContentSizes() doesn't return crazy values

    if ((flags & ImGuiWindowFlags_AlwaysAutoResize) != 0)
    {
        window.AutoFitFramesX = window.AutoFitFramesY = 2;
        window.AutoFitOnlyGrows = false;
    }
    else
    {
        if (window.Size.x <= 0.0)
            window.AutoFitFramesX = 2;
        if (window.Size.y <= 0.0)
            window.AutoFitFramesY = 2;
        window.AutoFitOnlyGrows = (window.AutoFitFramesX > 0) || (window.AutoFitFramesY > 0);
    }

    if (flags & ImGuiWindowFlags_NoBringToFrontOnFocus)
        g.Windows.push_front(window); // Quite slow but rare and only once
    else
        g.Windows.push_back(window);
    UpdateWindowInFocusOrderList(window, true, window.Flags);

    return window;
}

static ImGuiWindow* GetWindowForTitleDisplay(ImGuiWindow* window)
{
    return window.DockNodeAsHost ? window.DockNodeAsHost->VisibleWindow : window;
}

static ImGuiWindow* GetWindowForTitleAndMenuHeight(ImGuiWindow* window)
{
    return (window.DockNodeAsHost && window.DockNodeAsHost->VisibleWindow) ? window.DockNodeAsHost->VisibleWindow : window;
}

static Vector2D CalcWindowSizeAfterConstraint(ImGuiWindow* window, const Vector2D& size_desired)
{
    ImGuiContext& g = *GImGui;
    Vector2D new_size = size_desired;
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSizeConstraint)
    {
        // Using -1,-1 on either x/Y axis to preserve the current size.
        ImRect cr = g.NextWindowData.SizeConstraintRect;
        new_size.x = (cr.Min.x >= 0 && cr.Max.x >= 0) ? ImClamp(new_size.x, cr.Min.x, cr.Max.x) : window.SizeFull.x;
        new_size.y = (cr.Min.y >= 0 && cr.Max.y >= 0) ? ImClamp(new_size.y, cr.Min.y, cr.Max.y) : window.SizeFull.y;
        if (g.NextWindowData.SizeCallback)
        {
            ImGuiSizeCallbackData data;
            data.UserData = g.NextWindowData.SizeCallbackUserData;
            data.Pos = window.Pos;
            data.CurrentSize = window.SizeFull;
            data.DesiredSize = new_size;
            g.NextWindowData.SizeCallback(&data);
            new_size = data.DesiredSize;
        }
        new_size.x = IM_FLOOR(new_size.x);
        new_size.y = IM_FLOOR(new_size.y);
    }

    // Minimum size
    if (!(window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_AlwaysAutoResize)))
    {
        ImGuiWindow* window_for_height = GetWindowForTitleAndMenuHeight(window);
        const float decoration_up_height = window_for_height->TitleBarHeight() + window_for_height->MenuBarHeight();
        new_size = ImMax(new_size, g.Style.WindowMinSize);
        new_size.y = ImMax(new_size.y, decoration_up_height + ImMax(0.0, g.Style.WindowRounding - 1.0)); // Reduce artifacts with very small windows
    }
    return new_size;
}

static void CalcWindowContentSizes(ImGuiWindow* window, Vector2D* content_size_current, Vector2D* content_size_ideal)
{
    bool preserve_old_content_sizes = false;
    if (window.Collapsed && window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0)
        preserve_old_content_sizes = true;
    else if (window.Hidden && window.HiddenFramesCannotSkipItems == 0 && window.HiddenFramesCanSkipItems > 0)
        preserve_old_content_sizes = true;
    if (preserve_old_content_sizes)
    {
        *content_size_current = window.ContentSize;
        *content_size_ideal = window.ContentSizeIdeal;
        return;
    }

    content_size_current->x = (window.ContentSizeExplicit.x != 0.0) ? window.ContentSizeExplicit.x : IM_FLOOR(window.DC.CursorMaxPos.x - window.DC.CursorStartPos.x);
    content_size_current->y = (window.ContentSizeExplicit.y != 0.0) ? window.ContentSizeExplicit.y : IM_FLOOR(window.DC.CursorMaxPos.y - window.DC.CursorStartPos.y);
    content_size_ideal->x = (window.ContentSizeExplicit.x != 0.0) ? window.ContentSizeExplicit.x : IM_FLOOR(ImMax(window.DC.CursorMaxPos.x, window.DC.IdealMaxPos.x) - window.DC.CursorStartPos.x);
    content_size_ideal->y = (window.ContentSizeExplicit.y != 0.0) ? window.ContentSizeExplicit.y : IM_FLOOR(ImMax(window.DC.CursorMaxPos.y, window.DC.IdealMaxPos.y) - window.DC.CursorStartPos.y);
}

static Vector2D CalcWindowAutoFitSize(ImGuiWindow* window, const Vector2D& size_contents)
{
    ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;
    const float decoration_up_height = window.TitleBarHeight() + window.MenuBarHeight();
    Vector2D size_pad = window.WindowPadding * 2.0;
    Vector2D size_desired = size_contents + size_pad + DimgVec2D::new(0.0, decoration_up_height);
    if (window.Flags & ImGuiWindowFlags_Tooltip)
    {
        // Tooltip always resize
        return size_desired;
    }
    else
    {
        // Maximum window size is determined by the viewport size or monitor size
        const bool is_popup = (window.Flags & ImGuiWindowFlags_Popup) != 0;
        const bool is_menu = (window.Flags & ImGuiWindowFlags_ChildMenu) != 0;
        Vector2D size_min = style.WindowMinSize;
        if (is_popup || is_menu) // Popups and menus bypass style.WindowMinSize by default, but we give then a non-zero minimum size to facilitate understanding problematic cases (e.g. empty popups)
            size_min = ImMin(size_min, DimgVec2D::new(4.0, 4.0));

        // FIXME-VIEWPORT-WORKAREA: May want to use GetWorkSize() instead of size depending on the type of windows?
        Vector2D avail_size = window.viewport->Size;
        if (window.viewport_owned)
            avail_size = DimgVec2D::new(FLT_MAX, FLT_MAX);
        const int monitor_idx = window.ViewportAllowPlatformMonitorExtend;
        if (monitor_idx >= 0 && monitor_idx < g.PlatformIO.Monitors.Size)
            avail_size = g.PlatformIO.Monitors[monitor_idx].WorkSize;
        Vector2D size_auto_fit = ImClamp(size_desired, size_min, ImMax(size_min, avail_size - style.DisplaySafeAreaPadding * 2.0));

        // When the window cannot fit all contents (either because of constraints, either because screen is too small),
        // we are growing the size on the other axis to compensate for expected scrollbar. FIXME: Might turn bigger than ViewportSize-window_padding.
        Vector2D size_auto_fit_after_constraint = CalcWindowSizeAfterConstraint(window, size_auto_fit);
        bool will_have_scrollbar_x = (size_auto_fit_after_constraint.x - size_pad.x - 0.0                 < size_contents.x && !(window.Flags & ImGuiWindowFlags_NoScrollbar) && (window.Flags & ImGuiWindowFlags_HorizontalScrollbar)) || (window.Flags & ImGuiWindowFlags_AlwaysHorizontalScrollbar);
        bool will_have_scrollbar_y = (size_auto_fit_after_constraint.y - size_pad.y - decoration_up_height < size_contents.y && !(window.Flags & ImGuiWindowFlags_NoScrollbar)) || (window.Flags & ImGuiWindowFlags_AlwaysVerticalScrollbar);
        if (will_have_scrollbar_x)
            size_auto_fit.y += style.ScrollbarSize;
        if (will_have_scrollbar_y)
            size_auto_fit.x += style.ScrollbarSize;
        return size_auto_fit;
    }
}

Vector2D ImGui::CalcWindowNextAutoFitSize(ImGuiWindow* window)
{
    Vector2D size_contents_current;
    Vector2D size_contents_ideal;
    CalcWindowContentSizes(window, &size_contents_current, &size_contents_ideal);
    Vector2D size_auto_fit = CalcWindowAutoFitSize(window, size_contents_ideal);
    Vector2D size_final = CalcWindowSizeAfterConstraint(window, size_auto_fit);
    return size_final;
}

static ImGuiColor GetWindowBgColorIdx(ImGuiWindow* window)
{
    if (window.Flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup))
        return ImGuiCol_PopupBg;
    if ((window.Flags & ImGuiWindowFlags_ChildWindow) && !window.DockIsActive)
        return ImGuiCol_ChildBg;
    return ImGuiCol_WindowBg;
}

static void CalcResizePosSizeFromAnyCorner(ImGuiWindow* window, const Vector2D& corner_target, const Vector2D& corner_norm, Vector2D* out_pos, Vector2D* out_size)
{
    Vector2D pos_min = ImLerp(corner_target, window.Pos, corner_norm);                // Expected window upper-left
    Vector2D pos_max = ImLerp(window.Pos + window.Size, corner_target, corner_norm); // Expected window lower-right
    Vector2D size_expected = pos_max - pos_min;
    Vector2D size_constrained = CalcWindowSizeAfterConstraint(window, size_expected);
    *out_pos = pos_min;
    if (corner_norm.x == 0.0)
        out_pos->x -= (size_constrained.x - size_expected.x);
    if (corner_norm.y == 0.0)
        out_pos->y -= (size_constrained.y - size_expected.y);
    *out_size = size_constrained;
}

// data for resizing from corner
struct ImGuiResizeGripDef
{
    Vector2D  CornerPosN;
    Vector2D  InnerDir;
    int     AngleMin12, AngleMax12;
};
static const ImGuiResizeGripDef resize_grip_def[4] =
{
    { DimgVec2D::new(1, 1), DimgVec2D::new(-1, -1), 0, 3 },  // Lower-right
    { DimgVec2D::new(0, 1), DimgVec2D::new(+1, -1), 3, 6 },  // Lower-left
    { DimgVec2D::new(0, 0), DimgVec2D::new(+1, +1), 6, 9 },  // Upper-left (Unused)
    { DimgVec2D::new(1, 0), DimgVec2D::new(-1, +1), 9, 12 }  // Upper-right (Unused)
};

// data for resizing from borders
struct ImGuiResizeBorderDef
{
    Vector2D InnerDir;
    Vector2D SegmentN1, SegmentN2;
    float  OuterAngle;
};
static const ImGuiResizeBorderDef resize_border_def[4] =
{
    { DimgVec2D::new(+1, 0), DimgVec2D::new(0, 1), DimgVec2D::new(0, 0), IM_PI * 1.00 }, // Left
    { DimgVec2D::new(-1, 0), DimgVec2D::new(1, 0), DimgVec2D::new(1, 1), IM_PI * 0.00 }, // Right
    { DimgVec2D::new(0, +1), DimgVec2D::new(0, 0), DimgVec2D::new(1, 0), IM_PI * 1.50 }, // Up
    { DimgVec2D::new(0, -1), DimgVec2D::new(1, 1), DimgVec2D::new(0, 1), IM_PI * 0.50 }  // down
};

static ImRect GetResizeBorderRect(ImGuiWindow* window, int border_n, float perp_padding, float thickness)
{
    ImRect rect = window.Rect();
    if (thickness == 0.0)
        rect.Max -= DimgVec2D::new(1, 1);
    if (border_n == ImGuiDir_Left)  { return ImRect(rect.Min.x - thickness,    rect.Min.y + perp_padding, rect.Min.x + thickness,    rect.Max.y - perp_padding); }
    if (border_n == ImGuiDir_Right) { return ImRect(rect.Max.x - thickness,    rect.Min.y + perp_padding, rect.Max.x + thickness,    rect.Max.y - perp_padding); }
    if (border_n == ImGuiDir_Up)    { return ImRect(rect.Min.x + perp_padding, rect.Min.y - thickness,    rect.Max.x - perp_padding, rect.Min.y + thickness);    }
    if (border_n == ImGuiDir_Down)  { return ImRect(rect.Min.x + perp_padding, rect.Max.y - thickness,    rect.Max.x - perp_padding, rect.Max.y + thickness);    }
    IM_ASSERT(0);
    return ImRect();
}

// 0..3: corners (Lower-right, Lower-left, Unused, Unused)
ImGuiID ImGui::GetWindowResizeCornerID(ImGuiWindow* window, int n)
{
    IM_ASSERT(n >= 0 && n < 4);
    ImGuiID id = window.DockIsActive ? window.DockNode->HostWindow->ID : window.ID;
    id = ImHashStr("#RESIZE", 0, id);
    id = ImHashData(&n, sizeof, id);
    return id;
}

// Borders (Left, Right, Up, down)
ImGuiID ImGui::GetWindowResizeBorderID(ImGuiWindow* window, ImGuiDir dir)
{
    IM_ASSERT(dir >= 0 && dir < 4);
    int n = dir + 4;
    ImGuiID id = window.DockIsActive ? window.DockNode->HostWindow->ID : window.ID;
    id = ImHashStr("#RESIZE", 0, id);
    id = ImHashData(&n, sizeof, id);
    return id;
}

// Handle resize for: Resize Grips, Borders, Gamepad
// Return true when using auto-fit (double click on resize grip)
static bool ImGui::UpdateWindowManualResize(ImGuiWindow* window, const Vector2D& size_auto_fit, int* border_held, int resize_grip_count, ImU32 resize_grip_col[4], const ImRect& visibility_rect)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindowFlags flags = window.Flags;

    if ((flags & ImGuiWindowFlags_NoResize) || (flags & ImGuiWindowFlags_AlwaysAutoResize) || window.AutoFitFramesX > 0 || window.AutoFitFramesY > 0)
        return false;
    if (window.WasActive == false) // Early out to avoid running this code for e.g. an hidden implicit/fallback Debug window.
        return false;

    bool ret_auto_fit = false;
    const int resize_border_count = g.io.ConfigWindowsResizeFromEdges ? 4 : 0;
    const float grip_draw_size = IM_FLOOR(ImMax(g.FontSize * 1.35, window.WindowRounding + 1.0 + g.FontSize * 0.2));
    const float grip_hover_inner_size = IM_FLOOR(grip_draw_size * 0.75);
    const float grip_hover_outer_size = g.io.ConfigWindowsResizeFromEdges ? WINDOWS_HOVER_PADDING : 0.0;

    Vector2D pos_target(FLT_MAX, FLT_MAX);
    Vector2D size_target(FLT_MAX, FLT_MAX);

    // Clip mouse interaction rectangles within the viewport rectangle (in practice the narrowing is going to happen most of the time).
    // - Not narrowing would mostly benefit the situation where OS windows _without_ decoration have a threshold for hovering when outside their limits.
    //   This is however not the case with current backends under Win32, but a custom borderless window implementation would benefit from it.
    // - When decoration are enabled we typically benefit from that distance, but then our resize elements would be conflicting with OS resize elements, so we also narrow.
    // - Note that we are unable to tell if the platform setup allows hovering with a distance threshold (on Win32, decorated window have such threshold).
    // We only clip interaction so we overwrite window->clip_rect, cannot call push_clip_rect() yet as draw_list is not yet setup.
    const bool clip_with_viewport_rect = !(g.io.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport) || (g.io.MouseHoveredViewport != window.ViewportId) || !(window.viewport.flags & ImGuiViewportFlags_NoDecoration);
    if (clip_with_viewport_rect)
        window.ClipRect = window.viewport->GetMainRect();

    // Resize grips and borders are on layer 1
    window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;

    // Manual resize grips
    PushID("#RESIZE");
    for (int resize_grip_n = 0; resize_grip_n < resize_grip_count; resize_grip_n += 1)
    {
        const ImGuiResizeGripDef& def = resize_grip_def[resize_grip_n];
        const Vector2D corner = ImLerp(window.Pos, window.Pos + window.Size, def.CornerPosN);

        // Using the FlattenChilds button flag we make the resize button accessible even if we are hovering over a child window
        bool hovered, held;
        ImRect resize_rect(corner - def.InnerDir * grip_hover_outer_size, corner + def.InnerDir * grip_hover_inner_size);
        if (resize_rect.Min.x > resize_rect.Max.x) ImSwap(resize_rect.Min.x, resize_rect.Max.x);
        if (resize_rect.Min.y > resize_rect.Max.y) ImSwap(resize_rect.Min.y, resize_rect.Max.y);
        ImGuiID resize_grip_id = window.GetID(resize_grip_n); // == GetWindowResizeCornerID()
        keep_alive_id(resize_grip_id);
        ButtonBehavior(resize_rect, resize_grip_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_NoNavFocus);
        //GetForegroundDrawList(window)->add_rect(resize_rect.min, resize_rect.max, IM_COL32(255, 255, 0, 255));
        if (hovered || held)
            g.MouseCursor = (resize_grip_n & 1) ? ImGuiMouseCursor_ResizeNESW : ImGuiMouseCursor_ResizeNWSE;

        if (held && g.io.MouseClickedCount[0] == 2 && resize_grip_n == 0)
        {
            // Manual auto-fit when double-clicking
            size_target = CalcWindowSizeAfterConstraint(window, size_auto_fit);
            ret_auto_fit = true;
            clear_active_id();
        }
        else if (held)
        {
            // Resize from any of the four corners
            // We don't use an incremental mouse_delta but rather compute an absolute target size based on mouse position
            Vector2D clamp_min = DimgVec2D::new(def.CornerPosN.x == 1.0 ? visibility_rect.Min.x : -FLT_MAX, def.CornerPosN.y == 1.0 ? visibility_rect.Min.y : -FLT_MAX);
            Vector2D clamp_max = DimgVec2D::new(def.CornerPosN.x == 0.0 ? visibility_rect.Max.x : +FLT_MAX, def.CornerPosN.y == 0.0 ? visibility_rect.Max.y : +FLT_MAX);
            Vector2D corner_target = g.io.MousePos - g.ActiveIdClickOffset + ImLerp(def.InnerDir * grip_hover_outer_size, def.InnerDir * -grip_hover_inner_size, def.CornerPosN); // Corner of the window corresponding to our corner grip
            corner_target = ImClamp(corner_target, clamp_min, clamp_max);
            CalcResizePosSizeFromAnyCorner(window, corner_target, def.CornerPosN, &pos_target, &size_target);
        }

        // Only lower-left grip is visible before hovering/activating
        if (resize_grip_n == 0 || held || hovered)
            resize_grip_col[resize_grip_n] = GetColorU32(held ? ImGuiCol_ResizeGripActive : hovered ? ImGuiCol_ResizeGripHovered : ImGuiCol_ResizeGrip);
    }
    for (int border_n = 0; border_n < resize_border_count; border_n += 1)
    {
        const ImGuiResizeBorderDef& def = resize_border_def[border_n];
        const ImGuiAxis axis = (border_n == ImGuiDir_Left || border_n == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;

        bool hovered, held;
        ImRect border_rect = GetResizeBorderRect(window, border_n, grip_hover_inner_size, WINDOWS_HOVER_PADDING);
        ImGuiID border_id = window.GetID(border_n + 4); // == GetWindowResizeBorderID()
        keep_alive_id(border_id);
        ButtonBehavior(border_rect, border_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_NoNavFocus);
        //GetForegroundDrawLists(window)->add_rect(border_rect.min, border_rect.max, IM_COL32(255, 255, 0, 255));
        if ((hovered && g.HoveredIdTimer > WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER) || held)
        {
            g.MouseCursor = (axis == ImGuiAxis_X) ? ImGuiMouseCursor_ResizeEW : ImGuiMouseCursor_ResizeNS;
            if (held)
                *border_held = border_n;
        }
        if (held)
        {
            Vector2D clamp_min(border_n == ImGuiDir_Right ? visibility_rect.Min.x : -FLT_MAX, border_n == ImGuiDir_Down ? visibility_rect.Min.y : -FLT_MAX);
            Vector2D clamp_max(border_n == ImGuiDir_Left  ? visibility_rect.Max.x : +FLT_MAX, border_n == ImGuiDir_Up   ? visibility_rect.Max.y : +FLT_MAX);
            Vector2D border_target = window.Pos;
            border_target[axis] = g.io.MousePos[axis] - g.ActiveIdClickOffset[axis] + WINDOWS_HOVER_PADDING;
            border_target = ImClamp(border_target, clamp_min, clamp_max);
            CalcResizePosSizeFromAnyCorner(window, border_target, ImMin(def.SegmentN1, def.SegmentN2), &pos_target, &size_target);
        }
    }
    PopID();

    // Restore nav layer
    window.DC.NavLayerCurrent = ImGuiNavLayer_Main;

    // Navigation resize (keyboard/gamepad)
    if (g.NavWindowingTarget && g.NavWindowingTarget->RootWindowDockTree == window)
    {
        Vector2D nav_resize_delta;
        if (g.NavInputSource == ImGuiInputSource_Keyboard && g.io.KeyShift)
            nav_resize_delta = GetNavInputAmount2d(ImGuiNavDirSourceFlags_RawKeyboard, ImGuiNavReadMode_Down);
        if (g.NavInputSource == ImGuiInputSource_Gamepad)
            nav_resize_delta = GetNavInputAmount2d(ImGuiNavDirSourceFlags_PadDPad, ImGuiNavReadMode_Down);
        if (nav_resize_delta.x != 0.0 || nav_resize_delta.y != 0.0)
        {
            const float NAV_RESIZE_SPEED = 600.0;
            nav_resize_delta *= ImFloor(NAV_RESIZE_SPEED * g.io.DeltaTime * ImMin(g.io.DisplayFramebufferScale.x, g.io.DisplayFramebufferScale.y));
            nav_resize_delta = ImMax(nav_resize_delta, visibility_rect.Min - window.Pos - window.Size);
            g.NavWindowingToggleLayer = false;
            g.NavDisableMouseHover = true;
            resize_grip_col[0] = GetColorU32(ImGuiCol_ResizeGripActive);
            // FIXME-NAV: Should store and accumulate into a separate size buffer to handle sizing constraints properly, right now a constraint will make us stuck.
            size_target = CalcWindowSizeAfterConstraint(window, window.SizeFull + nav_resize_delta);
        }
    }

    // Apply back modified position/size to window
    if (size_target.x != FLT_MAX)
    {
        window.SizeFull = size_target;
        MarkIniSettingsDirty(window);
    }
    if (pos_target.x != FLT_MAX)
    {
        window.Pos = ImFloor(pos_target);
        MarkIniSettingsDirty(window);
    }

    window.Size = window.SizeFull;
    return ret_auto_fit;
}

static inline void ClampWindowRect(ImGuiWindow* window, const ImRect& visibility_rect)
{
    ImGuiContext& g = *GImGui;
    Vector2D size_for_clamping = window.Size;
    if (g.io.ConfigWindowsMoveFromTitleBarOnly && (!(window.Flags & ImGuiWindowFlags_NoTitleBar) || window.DockNodeAsHost))
        size_for_clamping.y = ImGui::GetFrameHeight(); // Not using window->TitleBarHeight() as dock_node_as_host will report 0.0 here.
    window.Pos = ImClamp(window.Pos, visibility_rect.Min - size_for_clamping, visibility_rect.Max);
}

static void ImGui::RenderWindowOuterBorders(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    float rounding = window.WindowRounding;
    float border_size = window.WindowBorderSize;
    if (border_size > 0.0 && !(window.Flags & ImGuiWindowFlags_NoBackground))
        window.DrawList->AddRect(window.Pos, window.Pos + window.Size, GetColorU32(ImGuiCol_Border), rounding, 0, border_size);

    int border_held = window.ResizeBorderHeld;
    if (border_held != -1)
    {
        const ImGuiResizeBorderDef& def = resize_border_def[border_held];
        ImRect border_r = GetResizeBorderRect(window, border_held, rounding, 0.0);
        window.DrawList->PathArcTo(ImLerp(border_r.Min, border_r.Max, def.SegmentN1) + DimgVec2D::new(0.5, 0.5) + def.InnerDir * rounding, rounding, def.OuterAngle - IM_PI * 0.25, def.OuterAngle);
        window.DrawList->PathArcTo(ImLerp(border_r.Min, border_r.Max, def.SegmentN2) + DimgVec2D::new(0.5, 0.5) + def.InnerDir * rounding, rounding, def.OuterAngle, def.OuterAngle + IM_PI * 0.25);
        window.DrawList->PathStroke(GetColorU32(ImGuiCol_SeparatorActive), 0, ImMax(2.0, border_size)); // Thicker than usual
    }
    if (g.Style.FrameBorderSize > 0 && !(window.Flags & ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive)
    {
        float y = window.Pos.y + window.TitleBarHeight() - 1;
        window.DrawList->AddLine(DimgVec2D::new(window.Pos.x + border_size, y), DimgVec2D::new(window.Pos.x + window.Size.x - border_size, y), GetColorU32(ImGuiCol_Border), g.Style.FrameBorderSize);
    }
}

// Draw background and borders
// Draw and handle scrollbars
void ImGui::RenderWindowDecorations(ImGuiWindow* window, const ImRect& title_bar_rect, bool title_bar_is_highlight, bool handle_borders_and_resize_grips, int resize_grip_count, const ImU32 resize_grip_col[4], float resize_grip_draw_size)
{
    ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;
    ImGuiWindowFlags flags = window.Flags;

    // Ensure that ScrollBar doesn't read last frame's skip_items
    IM_ASSERT(window.BeginCount == 0);
    window.SkipItems = false;

    // Draw window + handle manual resize
    // As we highlight the title bar when want_focus is set, multiple reappearing windows will have have their title bar highlighted on their reappearing frame.
    const float window_rounding = window.WindowRounding;
    const float window_border_size = window.WindowBorderSize;
    if (window.Collapsed)
    {
        // Title bar only
        float backup_border_size = style.FrameBorderSize;
        g.Style.FrameBorderSize = window.WindowBorderSize;
        ImU32 title_bar_col = GetColorU32((title_bar_is_highlight && !g.NavDisableHighlight) ? ImGuiCol_TitleBgActive : ImGuiCol_TitleBgCollapsed);
        RenderFrame(title_bar_rect.Min, title_bar_rect.Max, title_bar_col, true, window_rounding);
        g.Style.FrameBorderSize = backup_border_size;
    }
    else
    {
        // Window background
        if (!(flags & ImGuiWindowFlags_NoBackground))
        {
            bool is_docking_transparent_payload = false;
            if (g.DragDropActive && (g.FrameCount - g.DragDropAcceptFrameCount) <= 1 && g.io.ConfigDockingTransparentPayload)
                if (g.DragDropPayload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) && *(ImGuiWindow**)g.DragDropPayload.Data == window)
                    is_docking_transparent_payload = true;

            ImU32 bg_col = GetColorU32(GetWindowBgColorIdx(window));
            if (window.viewport_owned)
            {
                // No alpha
                bg_col = (bg_col | IM_COL32_A_MASK);
                if (is_docking_transparent_payload)
                    window.viewport->Alpha *= DOCKING_TRANSPARENT_PAYLOAD_ALPHA;
            }
            else
            {
                // Adjust alpha. For docking
                bool override_alpha = false;
                float alpha = 1.0;
                if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasBgAlpha)
                {
                    alpha = g.NextWindowData.BgAlphaVal;
                    override_alpha = true;
                }
                if (is_docking_transparent_payload)
                {
                    alpha *= DOCKING_TRANSPARENT_PAYLOAD_ALPHA; // FIXME-DOCK: Should that be an override?
                    override_alpha = true;
                }
                if (override_alpha)
                    bg_col = (bg_col & ~IM_COL32_A_MASK) | (IM_F32_TO_INT8_SAT(alpha) << IM_COL32_A_SHIFT);
            }

            // Render, for docked windows and host windows we ensure bg goes before decorations
            ImDrawList* bg_draw_list = window.DockIsActive ? window.DockNode->HostWindow->DrawList : window.DrawList;
            if (window.DockIsActive || (flags & ImGuiWindowFlags_DockNodeHost))
                bg_draw_list->ChannelsSetCurrent(0);
            if (window.DockIsActive)
                window.DockNode->LastBgColor = bg_col;

            bg_draw_list->AddRectFilled(window.Pos + DimgVec2D::new(0, window.TitleBarHeight()), window.Pos + window.Size, bg_col, window_rounding, (flags & ImGuiWindowFlags_NoTitleBar) ? 0 : ImDrawFlags_RoundCornersBottom);

            if (window.DockIsActive || (flags & ImGuiWindowFlags_DockNodeHost))
                bg_draw_list->ChannelsSetCurrent(1);
        }
        if (window.DockIsActive)
            window.DockNode->IsBgDrawnThisFrame = true;

        // Title bar
        // (when docked, dock_node are drawing their own title bar. Individual windows however do NOT set the _NoTitleBar flag,
        // in order for their pos/size to be matching their undocking state.)
        if (!(flags & ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive)
        {
            ImU32 title_bar_col = GetColorU32(title_bar_is_highlight ? ImGuiCol_TitleBgActive : ImGuiCol_TitleBg);
            window.DrawList->AddRectFilled(title_bar_rect.Min, title_bar_rect.Max, title_bar_col, window_rounding, ImDrawFlags_RoundCornersTop);
        }

        // Menu bar
        if (flags & ImGuiWindowFlags_MenuBar)
        {
            ImRect menu_bar_rect = window.MenuBarRect();
            menu_bar_rect.ClipWith(window.Rect());  // Soft clipping, in particular child window don't have minimum size covering the menu bar so this is useful for them.
            window.DrawList->AddRectFilled(menu_bar_rect.Min + DimgVec2D::new(window_border_size, 0), menu_bar_rect.Max - DimgVec2D::new(window_border_size, 0), GetColorU32(ImGuiCol_MenuBarBg), (flags & ImGuiWindowFlags_NoTitleBar) ? window_rounding : 0.0, ImDrawFlags_RoundCornersTop);
            if (style.FrameBorderSize > 0.0 && menu_bar_rect.Max.y < window.Pos.y + window.Size.y)
                window.DrawList->AddLine(menu_bar_rect.GetBL(), menu_bar_rect.GetBR(), GetColorU32(ImGuiCol_Border), style.FrameBorderSize);
        }

        // Docking: Unhide tab bar (small triangle in the corner), drag from small triangle to quickly undock
        ImGuiDockNode* node = window.DockNode;
        if (window.DockIsActive && node->IsHiddenTabBar() && !node->IsNoTabBar())
        {
            float unhide_sz_draw = ImFloor(g.FontSize * 0.70);
            float unhide_sz_hit = ImFloor(g.FontSize * 0.55);
            Vector2D p = node.pos;
            ImRect r(p, p + DimgVec2D::new(unhide_sz_hit, unhide_sz_hit));
            ImGuiID unhide_id = window.GetID("#UNHIDE");
            keep_alive_id(unhide_id);
            bool hovered, held;
            if (ButtonBehavior(r, unhide_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren))
                node->WantHiddenTabBarToggle = true;
            else if (held && IsMouseDragging(0))
                StartMouseMovingWindowOrNode(window, node, true);

            // FIXME-DOCK: Ideally we'd use ImGuiCol_TitleBgActive/ImGuiCol_TitleBg here, but neither is guaranteed to be visible enough at this sort of size..
            ImU32 col = GetColorU32(((held && hovered) || (node->IsFocused && !hovered)) ? ImGuiCol_ButtonActive : hovered ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
            window.DrawList->AddTriangleFilled(p, p + DimgVec2D::new(unhide_sz_draw, 0.0), p + DimgVec2D::new(0.0, unhide_sz_draw), col);
        }

        // Scrollbars
        if (window.ScrollbarX)
            Scrollbar(ImGuiAxis_X);
        if (window.ScrollbarY)
            Scrollbar(ImGuiAxis_Y);

        // Render resize grips (after their input handling so we don't have a frame of latency)
        if (handle_borders_and_resize_grips && !(flags & ImGuiWindowFlags_NoResize))
        {
            for (int resize_grip_n = 0; resize_grip_n < resize_grip_count; resize_grip_n += 1)
            {
                const ImGuiResizeGripDef& grip = resize_grip_def[resize_grip_n];
                const Vector2D corner = ImLerp(window.Pos, window.Pos + window.Size, grip.CornerPosN);
                window.DrawList->PathLineTo(corner + grip.InnerDir * ((resize_grip_n & 1) ? DimgVec2D::new(window_border_size, resize_grip_draw_size) : DimgVec2D::new(resize_grip_draw_size, window_border_size)));
                window.DrawList->PathLineTo(corner + grip.InnerDir * ((resize_grip_n & 1) ? DimgVec2D::new(resize_grip_draw_size, window_border_size) : DimgVec2D::new(window_border_size, resize_grip_draw_size)));
                window.DrawList->PathArcToFast(DimgVec2D::new(corner.x + grip.InnerDir.x * (window_rounding + window_border_size), corner.y + grip.InnerDir.y * (window_rounding + window_border_size)), window_rounding, grip.AngleMin12, grip.AngleMax12);
                window.DrawList->PathFillConvex(resize_grip_col[resize_grip_n]);
            }
        }

        // Borders (for dock node host they will be rendered over after the tab bar)
        if (handle_borders_and_resize_grips && !window.DockNodeAsHost)
            RenderWindowOuterBorders(window);
    }
}

// Render title text, collapse button, close button
// When inside a dock node, this is handled in DockNodeCalcTabBarLayout() instead.
void ImGui::RenderWindowTitleBarContents(ImGuiWindow* window, const ImRect& title_bar_rect, const char* name, bool* p_open)
{
    ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;
    ImGuiWindowFlags flags = window.Flags;

    const bool has_close_button = (p_open != NULL);
    const bool has_collapse_button = !(flags & ImGuiWindowFlags_NoCollapse) && (style.WindowMenuButtonPosition != ImGuiDir_None);

    // Close & Collapse button are on the Menu nav_layer and don't default focus (unless there's nothing else on that layer)
    // FIXME-NAV: Might want (or not?) to set the equivalent of ImGuiButtonFlags_NoNavFocus so that mouse clicks on standard title bar items don't necessarily set nav/keyboard ref?
    const ImGuiItemFlags item_flags_backup = g.CurrentItemFlags;
    g.CurrentItemFlags |= ImGuiItemFlags_NoNavDefaultFocus;
    window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;

    // Layout buttons
    // FIXME: Would be nice to generalize the subtleties expressed here into reusable code.
    float pad_l = style.FramePadding.x;
    float pad_r = style.FramePadding.x;
    float button_sz = g.FontSize;
    Vector2D close_button_pos;
    Vector2D collapse_button_pos;
    if (has_close_button)
    {
        pad_r += button_sz;
        close_button_pos = DimgVec2D::new(title_bar_rect.Max.x - pad_r - style.FramePadding.x, title_bar_rect.Min.y);
    }
    if (has_collapse_button && style.WindowMenuButtonPosition == ImGuiDir_Right)
    {
        pad_r += button_sz;
        collapse_button_pos = DimgVec2D::new(title_bar_rect.Max.x - pad_r - style.FramePadding.x, title_bar_rect.Min.y);
    }
    if (has_collapse_button && style.WindowMenuButtonPosition == ImGuiDir_Left)
    {
        collapse_button_pos = DimgVec2D::new(title_bar_rect.Min.x + pad_l - style.FramePadding.x, title_bar_rect.Min.y);
        pad_l += button_sz;
    }

    // Collapse button (submitting first so it gets priority when choosing a navigation init fallback)
    if (has_collapse_button)
        if (CollapseButton(window.GetID("#COLLAPSE"), collapse_button_pos, NULL))
            window.WantCollapseToggle = true; // Defer actual collapsing to next frame as we are too far in the Begin() function

    // Close button
    if (has_close_button)
        if (CloseButton(window.GetID("#CLOSE"), close_button_pos))
            *p_open = false;

    window.DC.NavLayerCurrent = ImGuiNavLayer_Main;
    g.CurrentItemFlags = item_flags_backup;

    // Title bar text (with: horizontal alignment, avoiding collapse/close button, optional "unsaved document" marker)
    // FIXME: Refactor text alignment facilities along with render_text helpers, this is WAY too much messy code..
    const float marker_size_x = (flags & ImGuiWindowFlags_UnsavedDocument) ? button_sz * 0.80 : 0.0;
    const Vector2D text_size = CalcTextSize(name, NULL, true) + DimgVec2D::new(marker_size_x, 0.0);

    // As a nice touch we try to ensure that centered title text doesn't get affected by visibility of Close/Collapse button,
    // while uncentered title text will still reach edges correctly.
    if (pad_l > style.FramePadding.x)
        pad_l += g.Style.ItemInnerSpacing.x;
    if (pad_r > style.FramePadding.x)
        pad_r += g.Style.ItemInnerSpacing.x;
    if (style.WindowTitleAlign.x > 0.0 && style.WindowTitleAlign.x < 1.0)
    {
        float centerness = ImSaturate(1.0 - ImFabs(style.WindowTitleAlign.x - 0.5) * 2.0); // 0.0 on either edges, 1.0 on center
        float pad_extend = ImMin(ImMax(pad_l, pad_r), title_bar_rect.GetWidth() - pad_l - pad_r - text_size.x);
        pad_l = ImMax(pad_l, pad_extend * centerness);
        pad_r = ImMax(pad_r, pad_extend * centerness);
    }

    ImRect layout_r(title_bar_rect.Min.x + pad_l, title_bar_rect.Min.y, title_bar_rect.Max.x - pad_r, title_bar_rect.Max.y);
    ImRect clip_r(layout_r.Min.x, layout_r.Min.y, ImMin(layout_r.Max.x + g.Style.ItemInnerSpacing.x, title_bar_rect.Max.x), layout_r.Max.y);
    if (flags & ImGuiWindowFlags_UnsavedDocument)
    {
        Vector2D marker_pos;
        marker_pos.x = ImClamp(layout_r.Min.x + (layout_r.GetWidth() - text_size.x) * style.WindowTitleAlign.x + text_size.x, layout_r.Min.x, layout_r.Max.x);
        marker_pos.y = (layout_r.Min.y + layout_r.Max.y) * 0.5;
        if (marker_pos.x > layout_r.Min.x)
        {
            RenderBullet(window.DrawList, marker_pos, GetColorU32(ImGuiCol_Text));
            clip_r.Max.x = ImMin(clip_r.Max.x, marker_pos.x - (marker_size_x * 0.5));
        }
    }
    //if (g.io.key_shift) window->draw_list->add_rect(layout_r.min, layout_r.max, IM_COL32(255, 128, 0, 255)); // [DEBUG]
    //if (g.io.key_ctrl) window->draw_list->add_rect(clip_r.min, clip_r.max, IM_COL32(255, 128, 0, 255)); // [DEBUG]
    RenderTextClipped(layout_r.Min, layout_r.Max, name, NULL, &text_size, style.WindowTitleAlign, &clip_r);
}

void ImGui::UpdateWindowParentAndRootLinks(ImGuiWindow* window, ImGuiWindowFlags flags, ImGuiWindow* parent_window)
{
    window.ParentWindow = parent_window;
    window.RootWindow = window.RootWindowPopupTree = window.RootWindowDockTree = window.RootWindowForTitleBarHighlight = window.RootWindowForNav = window;
    if (parent_window && (flags & ImGuiWindowFlags_ChildWindow) && !(flags & ImGuiWindowFlags_Tooltip))
    {
        window.RootWindowDockTree = parent_window.RootWindowDockTree;
        if (!window.DockIsActive && !(parent_window.Flags & ImGuiWindowFlags_DockNodeHost))
            window.RootWindow = parent_window.RootWindow;
    }
    if (parent_window && (flags & ImGuiWindowFlags_Popup))
        window.RootWindowPopupTree = parent_window.RootWindowPopupTree;
    if (parent_window && !(flags & ImGuiWindowFlags_Modal) && (flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_Popup))) // FIXME: simply use _NoTitleBar ?
        window.RootWindowForTitleBarHighlight = parent_window.RootWindowForTitleBarHighlight;
    while (window.RootWindowForNav.flags & ImGuiWindowFlags_NavFlattened)
    {
        IM_ASSERT(window.RootWindowForNav->ParentWindow != NULL);
        window.RootWindowForNav = window.RootWindowForNav->ParentWindow;
    }
}

// When a modal popup is open, newly created windows that want focus (i.e. are not popups and do not specify ImGuiWindowFlags_NoFocusOnAppearing)
// should be positioned behind that modal window, unless the window was created inside the modal begin-stack.
// In case of multiple stacked modals newly created window honors begin stack order and does not go below its own modal parent.
// - Window             // FindBlockingModal() returns Modal1
//   - Window           //                  .. returns Modal1
//   - Modal1           //                  .. returns Modal2
//      - Window        //                  .. returns Modal2
//          - Window    //                  .. returns Modal2
//          - Modal2    //                  .. returns Modal2
static ImGuiWindow* ImGui::FindBlockingModal(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size <= 0)
        return NULL;

    // Find a modal that has common parent with specified window. Specified window should be positioned behind that modal.
    for (int i = g.OpenPopupStack.Size - 1; i >= 0; i--)
    {
        ImGuiWindow* popup_window = g.OpenPopupStack.Data[i].Window;
        if (popup_window == NULL || !(popup_window.Flags & ImGuiWindowFlags_Modal))
            continue;
        if (!popup_window.Active && !popup_window.WasActive)      // Check was_active, because this code may run before popup renders on current frame, also check active to handle newly created windows.
            continue;
        if (IsWindowWithinBeginStackOf(window, popup_window))       // Window is rendered over last modal, no render order change needed.
            break;
        for (ImGuiWindow* parent = popup_window.ParentWindowInBeginStack->RootWindow; parent != NULL; parent = parent->ParentWindowInBeginStack->RootWindow)
            if (IsWindowWithinBeginStackOf(window, parent))
                return popup_window;                                // Place window above its begin stack parent.
    }
    return NULL;
}

// Push a new Dear ImGui window to add widgets to.
// - A default window called "Debug" is automatically stacked at the beginning of every frame so you can use widgets without explicitly calling a Begin/End pair.
// - Begin/End can be called multiple times during the frame with the same window name to append content.
// - The window name is used as a unique identifier to preserve window information across frames (and save rudimentary information to the .ini file).
//   You can use the "##" or "###" markers to use the same label with different id, or same id with different label. See documentation at the top of this file.
// - Return false when window is collapsed, so you can early out in your code. You always need to call ImGui::End() even if false is returned.
// - Passing 'bool* p_open' displays a Close button on the upper-right corner of the window, the pointed value will be set to false when the button is pressed.
bool ImGui::Begin(const char* name, bool* p_open, ImGuiWindowFlags flags)
{
    ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    IM_ASSERT(name != NULL && name[0] != '\0');     // Window name required
    IM_ASSERT(g.WithinFrameScope);                  // Forgot to call ImGui::NewFrame()
    IM_ASSERT(g.FrameCountEnded != g.FrameCount);   // Called ImGui::Render() or ImGui::EndFrame() and haven't called ImGui::NewFrame() again yet

    // Find or create
    ImGuiWindow* window = FindWindowByName(name);
    const bool window_just_created = (window == NULL);
    if (window_just_created)
        window = CreateNewWindow(name, flags);
    else
        UpdateWindowInFocusOrderList(window, window_just_created, flags);

    // Automatically disable manual moving/resizing when NoInputs is set
    if ((flags & ImGuiWindowFlags_NoInputs) == ImGuiWindowFlags_NoInputs)
        flags |= ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoResize;

    if (flags & ImGuiWindowFlags_NavFlattened)
        IM_ASSERT(flags & ImGuiWindowFlags_ChildWindow);

    const int current_frame = g.FrameCount;
    const bool first_begin_of_the_frame = (window.LastFrameActive != current_frame);
    window.IsFallbackWindow = (g.CurrentWindowStack.Size == 0 && g.WithinFrameScopeWithImplicitWindow);

    // Update the appearing flag (note: the BeginDocked() path may also set this to true later)
    bool window_just_activated_by_user = (window.LastFrameActive < current_frame - 1); // Not using !was_active because the implicit "Debug" window would always toggle off->on
    if (flags & ImGuiWindowFlags_Popup)
    {
        ImGuiPopupData& popup_ref = g.OpenPopupStack[g.BeginPopupStack.Size];
        window_just_activated_by_user |= (window.PopupId != popup_ref.PopupId); // We recycle popups so treat window as activated if popup id changed
        window_just_activated_by_user |= (window != popup_ref.Window);
    }

    // Update flags, last_frame_active, BeginOrderXXX fields
    const bool window_was_appearing = window.Appearing;
    if (first_begin_of_the_frame)
    {
        window.Appearing = window_just_activated_by_user;
        if (window.Appearing)
            SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, true);

        window.FlagsPreviousFrame = window.Flags;
        window.Flags = (ImGuiWindowFlags)flags;
        window.LastFrameActive = current_frame;
        window.LastTimeActive = (float)g.Time;
        window.BeginOrderWithinParent = 0;
        window.BeginOrderWithinContext = (short)(g.WindowsActiveCount += 1);
    }
    else
    {
        flags = window.Flags;
    }

    // Docking
    // (NB: during the frame dock nodes are created, it is possible that (window->dock_is_active == false) even though (window->dock_node->windows.size > 1)
    IM_ASSERT(window.DockNode == NULL || window.DockNodeAsHost == NULL); // Cannot be both
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasDock)
        SetWindowDock(window, g.NextWindowData.DockId, g.NextWindowData.DockCond);
    if (first_begin_of_the_frame)
    {
        bool has_dock_node = (window.DockId != 0 || window.DockNode != NULL);
        bool new_auto_dock_node = !has_dock_node && GetWindowAlwaysWantOwnTabBar(window);
        bool dock_node_was_visible = window.DockNodeIsVisible;
        bool dock_tab_was_visible = window.DockTabIsVisible;
        if (has_dock_node || new_auto_dock_node)
        {
            BeginDocked(window, p_open);
            flags = window.Flags;
            if (window.DockIsActive)
            {
                IM_ASSERT(window.DockNode != NULL);
                g.NextWindowData.Flags &= ~ImGuiNextWindowDataFlags_HasSizeConstraint; // Docking currently override constraints
            }

            // Amend the appearing flag
            if (window.DockTabIsVisible && !dock_tab_was_visible && dock_node_was_visible && !window.Appearing && !window_was_appearing)
            {
                window.Appearing = true;
                SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, true);
            }
        }
        else
        {
            window.DockIsActive = window.DockNodeIsVisible = window.DockTabIsVisible = false;
        }
    }

    // Parent window is latched only on the first call to Begin() of the frame, so further append-calls can be done from a different window stack
    ImGuiWindow* parent_window_in_stack = (window.DockIsActive && window.DockNode->HostWindow) ? window.DockNode->HostWindow : g.CurrentWindowStack.empty() ? NULL : g.CurrentWindowStack.back().Window;
    ImGuiWindow* parent_window = first_begin_of_the_frame ? ((flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_Popup)) ? parent_window_in_stack : NULL) : window.ParentWindow;
    IM_ASSERT(parent_window != NULL || !(flags & ImGuiWindowFlags_ChildWindow));

    // We allow window memory to be compacted so recreate the base stack when needed.
    if (window.IDStack.Size == 0)
        window.IDStack.push_back(window.ID);

    // Add to stack
    // We intentionally set g.current_window to NULL to prevent usage until when the viewport is set, then will call set_current_window()
    g.CurrentWindow = window;
    ImGuiWindowStackData window_stack_data;
    window_stack_data.Window = window;
    window_stack_data.ParentLastItemDataBackup = g.last_item_data;
    window_stack_data.StackSizesOnBegin.SetToCurrentState();
    g.CurrentWindowStack.push_back(window_stack_data);
    g.CurrentWindow = NULL;
    if (flags & ImGuiWindowFlags_ChildMenu)
        g.BeginMenuCount += 1;

    if (flags & ImGuiWindowFlags_Popup)
    {
        ImGuiPopupData& popup_ref = g.OpenPopupStack[g.BeginPopupStack.Size];
        popup_ref.Window = window;
        popup_ref.ParentNavLayer = parent_window_in_stack->DC.NavLayerCurrent;
        g.BeginPopupStack.push_back(popup_ref);
        window.PopupId = popup_ref.PopupId;
    }

    // Update ->root_window and others pointers (before any possible call to focus_window)
    if (first_begin_of_the_frame)
    {
        UpdateWindowParentAndRootLinks(window, flags, parent_window);
        window.ParentWindowInBeginStack = parent_window_in_stack;
    }

    // Process SetNextWindow***() calls
    // (FIXME: Consider splitting the HasXXX flags into x/Y components
    bool window_pos_set_by_api = false;
    bool window_size_x_set_by_api = false, window_size_y_set_by_api = false;
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasPos)
    {
        window_pos_set_by_api = (window.SetWindowPosAllowFlags & g.NextWindowData.PosCond) != 0;
        if (window_pos_set_by_api && ImLengthSqr(g.NextWindowData.PosPivotVal) > 0.00001)
        {
            // May be processed on the next frame if this is our first frame and we are measuring size
            // FIXME: Look into removing the branch so everything can go through this same code path for consistency.
            window.SetWindowPosVal = g.NextWindowData.PosVal;
            window.SetWindowPosPivot = g.NextWindowData.PosPivotVal;
            window.SetWindowPosAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);
        }
        else
        {
            set_window_pos(window, g.NextWindowData.PosVal, g.NextWindowData.PosCond);
        }
    }
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize)
    {
        window_size_x_set_by_api = (window.SetWindowSizeAllowFlags & g.NextWindowData.SizeCond) != 0 && (g.NextWindowData.SizeVal.x > 0.0);
        window_size_y_set_by_api = (window.SetWindowSizeAllowFlags & g.NextWindowData.SizeCond) != 0 && (g.NextWindowData.SizeVal.y > 0.0);
        SetWindowSize(window, g.NextWindowData.SizeVal, g.NextWindowData.SizeCond);
    }
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasScroll)
    {
        if (g.NextWindowData.ScrollVal.x >= 0.0)
        {
            window.ScrollTarget.x = g.NextWindowData.ScrollVal.x;
            window.ScrollTargetCenterRatio.x = 0.0;
        }
        if (g.NextWindowData.ScrollVal.y >= 0.0)
        {
            window.ScrollTarget.y = g.NextWindowData.ScrollVal.y;
            window.ScrollTargetCenterRatio.y = 0.0;
        }
    }
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasContentSize)
        window.ContentSizeExplicit = g.NextWindowData.ContentSizeVal;
    else if (first_begin_of_the_frame)
        window.ContentSizeExplicit = DimgVec2D::new(0.0, 0.0);
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasWindowClass)
        window.WindowClass = g.NextWindowData.WindowClass;
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasCollapsed)
        SetWindowCollapsed(window, g.NextWindowData.CollapsedVal, g.NextWindowData.CollapsedCond);
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasFocus)
        focus_window(window);
    if (window.Appearing)
        SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, false);

    // When reusing window again multiple times a frame, just append content (don't need to setup again)
    if (first_begin_of_the_frame)
    {
        // Initialize
        const bool window_is_child_tooltip = (flags & ImGuiWindowFlags_ChildWindow) && (flags & ImGuiWindowFlags_Tooltip); // FIXME-WIP: Undocumented behavior of Child+Tooltip for pinned tooltip (#1345)
        const bool window_just_appearing_after_hidden_for_resize = (window.HiddenFramesCannotSkipItems > 0);
        window.Active = true;
        window.HasCloseButton = (p_open != NULL);
        window.ClipRect = Vector4D(-FLT_MAX, -FLT_MAX, +FLT_MAX, +FLT_MAX);
        window.IDStack.resize(1);
        window.DrawList->_ResetForNewFrame();
        window.DC.CurrentTableIdx = -1;
        if (flags & ImGuiWindowFlags_DockNodeHost)
        {
            window.DrawList->ChannelsSplit(2);
            window.DrawList->ChannelsSetCurrent(1); // Render decorations on channel 1 as we will render the backgrounds manually later
        }

        // Restore buffer capacity when woken from a compacted state, to avoid
        if (window.MemoryCompacted)
            GcAwakeTransientWindowBuffers(window);

        // Update stored window name when it changes (which can _only_ happen with the "###" operator, so the id would stay unchanged).
        // The title bar always display the 'name' parameter, so we only update the string storage if it needs to be visible to the end-user elsewhere.
        bool window_title_visible_elsewhere = false;
        if ((window.viewport && window.viewport->Window == window) || (window.DockIsActive))
            window_title_visible_elsewhere = true;
        else if (g.NavWindowingListWindow != NULL && (window.Flags & ImGuiWindowFlags_NoNavFocus) == 0)   // Window titles visible when using CTRL+TAB
            window_title_visible_elsewhere = true;
        if (window_title_visible_elsewhere && !window_just_created && strcmp(name, window.Name) != 0)
        {
            size_t buf_len = window.NameBufLen;
            window.Name = ImStrdupcpy(window.Name, &buf_len, name);
            window.NameBufLen = buf_len;
        }

        // UPDATE CONTENTS SIZE, UPDATE HIDDEN STATUS

        // Update contents size from last frame for auto-fitting (or use explicit size)
        CalcWindowContentSizes(window, &window.ContentSize, &window.ContentSizeIdeal);

        // FIXME: These flags are decremented before they are used. This means that in order to have these fields produce their intended behaviors
        // for one frame we must set them to at least 2, which is counter-intuitive. hidden_frames_cannot_skip_items is a more complicated case because
        // it has a single usage before this code block and may be set below before it is finally checked.
        if (window.HiddenFramesCanSkipItems > 0)
            window.HiddenFramesCanSkipItems--;
        if (window.HiddenFramesCannotSkipItems > 0)
            window.HiddenFramesCannotSkipItems--;
        if (window.HiddenFramesForRenderOnly > 0)
            window.HiddenFramesForRenderOnly--;

        // Hide new windows for one frame until they calculate their size
        if (window_just_created && (!window_size_x_set_by_api || !window_size_y_set_by_api))
            window.HiddenFramesCannotSkipItems = 1;

        // Hide popup/tooltip window when re-opening while we measure size (because we recycle the windows)
        // We reset size/content_size for reappearing popups/tooltips early in this function, so further code won't be tempted to use the old size.
        if (window_just_activated_by_user && (flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) != 0)
        {
            window.HiddenFramesCannotSkipItems = 1;
            if (flags & ImGuiWindowFlags_AlwaysAutoResize)
            {
                if (!window_size_x_set_by_api)
                    window.Size.x = window.SizeFull.x = 0.f;
                if (!window_size_y_set_by_api)
                    window.Size.y = window.SizeFull.y = 0.f;
                window.ContentSize = window.ContentSizeIdeal = DimgVec2D::new(0.f, 0.f);
            }
        }

        // SELECT VIEWPORT
        // We need to do this before using any style/font sizes, as viewport with a different DPI may affect font sizes.

        WindowSelectViewport(window);
        SetCurrentViewport(window, window.viewport);
        window.FontDpiScale = (g.io.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleFonts) ? window.viewport->DpiScale : 1.0;
        SetCurrentWindow(window);
        flags = window.Flags;

        // LOCK BORDER SIZE AND PADDING FOR THE FRAME (so that altering them doesn't cause inconsistencies)
        // We read style data after the call to UpdateSelectWindowViewport() which might be swapping the style.

        if (flags & ImGuiWindowFlags_ChildWindow)
            window.WindowBorderSize = style.ChildBorderSize;
        else
            window.WindowBorderSize = ((flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && !(flags & ImGuiWindowFlags_Modal)) ? style.PopupBorderSize : style.WindowBorderSize;
        if (!window.DockIsActive && (flags & ImGuiWindowFlags_ChildWindow) && !(flags & (ImGuiWindowFlags_AlwaysUseWindowPadding | ImGuiWindowFlags_Popup)) && window.WindowBorderSize == 0.0)
            window.WindowPadding = DimgVec2D::new(0.0, (flags & ImGuiWindowFlags_MenuBar) ? style.WindowPadding.y : 0.0);
        else
            window.WindowPadding = style.WindowPadding;

        // Lock menu offset so size calculation can use it as menu-bar windows need a minimum size.
        window.DC.MenuBarOffset.x = ImMax(ImMax(window.WindowPadding.x, style.ItemSpacing.x), g.NextWindowData.MenuBarOffsetMinVal.x);
        window.DC.MenuBarOffset.y = g.NextWindowData.MenuBarOffsetMinVal.y;

        // Collapse window by double-clicking on title bar
        // At this point we don't have a clipping rectangle setup yet, so we can use the title bar area for hit detection and drawing
        if (!(flags & ImGuiWindowFlags_NoTitleBar) && !(flags & ImGuiWindowFlags_NoCollapse) && !window.DockIsActive)
        {
            // We don't use a regular button+id to test for double-click on title bar (mostly due to legacy reason, could be fixed), so verify that we don't have items over the title bar.
            ImRect title_bar_rect = window.TitleBarRect();
            if (g.hovered_window == window && g.hovered_id == 0 && g.HoveredIdPreviousFrame == 0 && IsMouseHoveringRect(title_bar_rect.Min, title_bar_rect.Max) && g.io.MouseClickedCount[0] == 2)
                window.WantCollapseToggle = true;
            if (window.WantCollapseToggle)
            {
                window.Collapsed = !window.Collapsed;
                MarkIniSettingsDirty(window);
            }
        }
        else
        {
            window.Collapsed = false;
        }
        window.WantCollapseToggle = false;

        // SIZE

        // Calculate auto-fit size, handle automatic resize
        const Vector2D size_auto_fit = CalcWindowAutoFitSize(window, window.ContentSizeIdeal);
        bool use_current_size_for_scrollbar_x = window_just_created;
        bool use_current_size_for_scrollbar_y = window_just_created;
        if ((flags & ImGuiWindowFlags_AlwaysAutoResize) && !window.Collapsed)
        {
            // Using SetNextWindowSize() overrides ImGuiWindowFlags_AlwaysAutoResize, so it can be used on tooltips/popups, etc.
            if (!window_size_x_set_by_api)
            {
                window.SizeFull.x = size_auto_fit.x;
                use_current_size_for_scrollbar_x = true;
            }
            if (!window_size_y_set_by_api)
            {
                window.SizeFull.y = size_auto_fit.y;
                use_current_size_for_scrollbar_y = true;
            }
        }
        else if (window.AutoFitFramesX > 0 || window.AutoFitFramesY > 0)
        {
            // Auto-fit may only grow window during the first few frames
            // We still process initial auto-fit on collapsed windows to get a window width, but otherwise don't honor ImGuiWindowFlags_AlwaysAutoResize when collapsed.
            if (!window_size_x_set_by_api && window.AutoFitFramesX > 0)
            {
                window.SizeFull.x = window.AutoFitOnlyGrows ? ImMax(window.SizeFull.x, size_auto_fit.x) : size_auto_fit.x;
                use_current_size_for_scrollbar_x = true;
            }
            if (!window_size_y_set_by_api && window.AutoFitFramesY > 0)
            {
                window.SizeFull.y = window.AutoFitOnlyGrows ? ImMax(window.SizeFull.y, size_auto_fit.y) : size_auto_fit.y;
                use_current_size_for_scrollbar_y = true;
            }
            if (!window.Collapsed)
                MarkIniSettingsDirty(window);
        }

        // Apply minimum/maximum window size constraints and final size
        window.SizeFull = CalcWindowSizeAfterConstraint(window, window.SizeFull);
        window.Size = window.Collapsed && !(flags & ImGuiWindowFlags_ChildWindow) ? window.TitleBarRect().GetSize() : window.SizeFull;

        // Decoration size
        const float decoration_up_height = window.TitleBarHeight() + window.MenuBarHeight();

        // POSITION

        // Popup latch its initial position, will position itself when it appears next frame
        if (window_just_activated_by_user)
        {
            window.AutoPosLastDirection = ImGuiDir_None;
            if ((flags & ImGuiWindowFlags_Popup) != 0 && !(flags & ImGuiWindowFlags_Modal) && !window_pos_set_by_api) // FIXME: BeginPopup() could use SetNextWindowPos()
                window.Pos = g.BeginPopupStack.back().OpenPopupPos;
        }

        // Position child window
        if (flags & ImGuiWindowFlags_ChildWindow)
        {
            IM_ASSERT(parent_window && parent_window.Active);
            window.BeginOrderWithinParent = (short)parent_window.DC.ChildWindows.Size;
            parent_window.DC.ChildWindows.push_back(window);
            if (!(flags & ImGuiWindowFlags_Popup) && !window_pos_set_by_api && !window_is_child_tooltip)
                window.Pos = parent_window.DC.CursorPos;
        }

        const bool window_pos_with_pivot = (window.SetWindowPosVal.x != FLT_MAX && window.HiddenFramesCannotSkipItems == 0);
        if (window_pos_with_pivot)
            set_window_pos(window, window.SetWindowPosVal - window.Size * window.SetWindowPosPivot, 0); // Position given a pivot (e.g. for centering)
        else if ((flags & ImGuiWindowFlags_ChildMenu) != 0)
            window.Pos = FindBestWindowPosForPopup(window);
        else if ((flags & ImGuiWindowFlags_Popup) != 0 && !window_pos_set_by_api && window_just_appearing_after_hidden_for_resize)
            window.Pos = FindBestWindowPosForPopup(window);
        else if ((flags & ImGuiWindowFlags_Tooltip) != 0 && !window_pos_set_by_api && !window_is_child_tooltip)
            window.Pos = FindBestWindowPosForPopup(window);

        // Late create viewport if we don't fit within our current host viewport.
        if (window.ViewportAllowPlatformMonitorExtend >= 0 && !window.viewport_owned && !(window.viewport.flags & ImGuiViewportFlags_Minimized))
            if (!window.viewport->GetMainRect().Contains(window.Rect()))
            {
                // This is based on the assumption that the DPI will be known ahead (same as the DPI of the selection done in UpdateSelectWindowViewport)
                //ImGuiViewport* old_viewport = window->viewport;
                window.viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_NoFocusOnAppearing);

                // FIXME-DPI
                //IM_ASSERT(old_viewport->dpi_scale == window->viewport->dpi_scale); // FIXME-DPI: Something went wrong
                SetCurrentViewport(window, window.viewport);
                window.FontDpiScale = (g.io.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleFonts) ? window.viewport->DpiScale : 1.0;
                SetCurrentWindow(window);
            }

        if (window.viewport_owned)
            WindowSyncOwnedViewport(window, parent_window_in_stack);

        // Calculate the range of allowed position for that window (to be movable and visible past safe area padding)
        // When clamping to stay visible, we will enforce that window->pos stays inside of visibility_rect.
        ImRect viewport_rect(window.viewport->GetMainRect());
        ImRect viewport_work_rect(window.viewport->GetWorkRect());
        Vector2D visibility_padding = ImMax(style.DisplayWindowPadding, style.DisplaySafeAreaPadding);
        ImRect visibility_rect(viewport_work_rect.Min + visibility_padding, viewport_work_rect.Max - visibility_padding);

        // Clamp position/size so window stays visible within its viewport or monitor
        // Ignore zero-sized display explicitly to avoid losing positions if a window manager reports zero-sized window when initializing or minimizing.
        // FIXME: Similar to code in GetWindowAllowedExtentRect()
        if (!window_pos_set_by_api && !(flags & ImGuiWindowFlags_ChildWindow) && window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0)
        {
            if (!window.viewport_owned && viewport_rect.GetWidth() > 0 && viewport_rect.GetHeight() > 0.0)
            {
                ClampWindowRect(window, visibility_rect);
            }
            else if (window.viewport_owned && g.PlatformIO.Monitors.Size > 0)
            {
                // Lost windows (e.g. a monitor disconnected) will naturally moved to the fallback/dummy monitor aka the main viewport.
                const ImGuiPlatformMonitor* monitor = GetViewportPlatformMonitor(window.viewport);
                visibility_rect.Min = monitor->WorkPos + visibility_padding;
                visibility_rect.Max = monitor->WorkPos + monitor->WorkSize - visibility_padding;
                ClampWindowRect(window, visibility_rect);
            }
        }
        window.Pos = ImFloor(window.Pos);

        // Lock window rounding for the frame (so that altering them doesn't cause inconsistencies)
        // Large values tend to lead to variety of artifacts and are not recommended.
        if (window.viewport_owned || window.DockIsActive)
            window.WindowRounding = 0.0;
        else
            window.WindowRounding = (flags & ImGuiWindowFlags_ChildWindow) ? style.ChildRounding : ((flags & ImGuiWindowFlags_Popup) && !(flags & ImGuiWindowFlags_Modal)) ? style.PopupRounding : style.WindowRounding;

        // For windows with title bar or menu bar, we clamp to FrameHeight(font_size + FramePadding.y * 2.0) to completely hide artifacts.
        //if ((window->flags & ImGuiWindowFlags_MenuBar) || !(window->flags & ImGuiWindowFlags_NoTitleBar))
        //    window->window_rounding = ImMin(window->window_rounding, g.font_size + style.FramePadding.y * 2.0);

        // Apply window focus (new and reactivated windows are moved to front)
        bool want_focus = false;
        if (window_just_activated_by_user && !(flags & ImGuiWindowFlags_NoFocusOnAppearing))
        {
            if (flags & ImGuiWindowFlags_Popup)
                want_focus = true;
            else if ((window.DockIsActive || (flags & ImGuiWindowFlags_ChildWindow) == 0) && !(flags & ImGuiWindowFlags_Tooltip))
                want_focus = true;

            ImGuiWindow* modal = GetTopMostPopupModal();
            if (modal != NULL && !IsWindowWithinBeginStackOf(window, modal))
            {
                // Avoid focusing a window that is created outside of active modal. This will prevent active modal from being closed.
                // Since window is not focused it would reappear at the same display position like the last time it was visible.
                // In case of completely new windows it would go to the top (over current modal), but input to such window would still be blocked by modal.
                // Position window behind a modal that is not a begin-parent of this window.
                want_focus = false;
                if (window == window.RootWindow)
                {
                    ImGuiWindow* blocking_modal = FindBlockingModal(window);
                    IM_ASSERT(blocking_modal != NULL);
                    BringWindowToDisplayBehind(window, blocking_modal);
                }
            }
        }

        // [Test Engine] Register whole window in the item system
#ifdef IMGUI_ENABLE_TEST_ENGINE
        if (g.TestEngineHookItems)
        {
            IM_ASSERT(window.IDStack.Size == 1);
            window.IDStack.Size = 0;
            IMGUI_TEST_ENGINE_ITEM_ADD(window.Rect(), window.ID);
            IMGUI_TEST_ENGINE_ITEM_INFO(window.ID, window.Name, (g.hovered_window == window) ? ImGuiItemStatusFlags_HoveredRect : 0);
            window.IDStack.Size = 1;
        }


        // Decide if we are going to handle borders and resize grips
        const bool handle_borders_and_resize_grips = (window.DockNodeAsHost || !window.DockIsActive);

        // Handle manual resize: Resize Grips, Borders, Gamepad
        int border_held = -1;
        ImU32 resize_grip_col[4] = {};
        const int resize_grip_count = g.io.ConfigWindowsResizeFromEdges ? 2 : 1; // Allow resize from lower-left if we have the mouse cursor feedback for it.
        const float resize_grip_draw_size = IM_FLOOR(ImMax(g.FontSize * 1.10, window.WindowRounding + 1.0 + g.FontSize * 0.2));
        if (handle_borders_and_resize_grips && !window.Collapsed)
            if (UpdateWindowManualResize(window, size_auto_fit, &border_held, resize_grip_count, &resize_grip_col[0], visibility_rect))
                use_current_size_for_scrollbar_x = use_current_size_for_scrollbar_y = true;
        window.ResizeBorderHeld = (signed char)border_held;

        // Synchronize window --> viewport again and one last time (clamping and manual resize may have affected either)
        if (window.viewport_owned)
        {
            if (!window.viewport->PlatformRequestMove)
                window.viewport.pos = window.Pos;
            if (!window.viewport->PlatformRequestResize)
                window.viewport->Size = window.Size;
            window.viewport.update_work_rect();
            viewport_rect = window.viewport->GetMainRect();
        }

        // Save last known viewport position within the window itself (so it can be saved in .ini file and restored)
        window.ViewportPos = window.viewport.pos;

        // SCROLLBAR VISIBILITY

        // Update scrollbar visibility (based on the size that was effective during last frame or the auto-resized size).
        if (!window.Collapsed)
        {
            // When reading the current size we need to read it after size constraints have been applied.
            // When we use inner_rect here we are intentionally reading last frame size, same for scrollbar_sizes values before we set them again.
            Vector2D avail_size_from_current_frame = DimgVec2D::new(window.SizeFull.x, window.SizeFull.y - decoration_up_height);
            Vector2D avail_size_from_last_frame = window.InnerRect.GetSize() + window.ScrollbarSizes;
            Vector2D needed_size_from_last_frame = window_just_created ? DimgVec2D::new(0, 0) : window.ContentSize + window.WindowPadding * 2.0;
            float size_x_for_scrollbars = use_current_size_for_scrollbar_x ? avail_size_from_current_frame.x : avail_size_from_last_frame.x;
            float size_y_for_scrollbars = use_current_size_for_scrollbar_y ? avail_size_from_current_frame.y : avail_size_from_last_frame.y;
            //bool scrollbar_y_from_last_frame = window->scrollbar_y; // FIXME: May want to use that in the scrollbar_x expression? How many pros vs cons?
            window.ScrollbarY = (flags & ImGuiWindowFlags_AlwaysVerticalScrollbar) || ((needed_size_from_last_frame.y > size_y_for_scrollbars) && !(flags & ImGuiWindowFlags_NoScrollbar));
            window.ScrollbarX = (flags & ImGuiWindowFlags_AlwaysHorizontalScrollbar) || ((needed_size_from_last_frame.x > size_x_for_scrollbars - (window.ScrollbarY ? style.ScrollbarSize : 0.0)) && !(flags & ImGuiWindowFlags_NoScrollbar) && (flags & ImGuiWindowFlags_HorizontalScrollbar));
            if (window.ScrollbarX && !window.ScrollbarY)
                window.ScrollbarY = (needed_size_from_last_frame.y > size_y_for_scrollbars) && !(flags & ImGuiWindowFlags_NoScrollbar);
            window.ScrollbarSizes = DimgVec2D::new(window.ScrollbarY ? style.ScrollbarSize : 0.0, window.ScrollbarX ? style.ScrollbarSize : 0.0);
        }

        // UPDATE RECTANGLES (1- THOSE NOT AFFECTED BY SCROLLING)
        // Update various regions. Variables they depends on should be set above in this function.
        // We set this up after processing the resize grip so that our rectangles doesn't lag by a frame.

        // Outer rectangle
        // Not affected by window border size. Used by:
        // - FindHoveredWindow() (w/ extra padding when border resize is enabled)
        // - Begin() initial clipping rect for drawing window background and borders.
        // - Begin() clipping whole child
        const ImRect host_rect = ((flags & ImGuiWindowFlags_ChildWindow) && !(flags & ImGuiWindowFlags_Popup) && !window_is_child_tooltip) ? parent_window.ClipRect : viewport_rect;
        const ImRect outer_rect = window.Rect();
        const ImRect title_bar_rect = window.TitleBarRect();
        window.OuterRectClipped = outer_rect;
        if (window.DockIsActive)
            window.OuterRectClipped.Min.y += window.TitleBarHeight();
        window.OuterRectClipped.ClipWith(host_rect);

        // Inner rectangle
        // Not affected by window border size. Used by:
        // - inner_clip_rect
        // - ScrollToRectEx()
        // - NavUpdatePageUpPageDown()
        // - Scrollbar()
        window.InnerRect.Min.x = window.Pos.x;
        window.InnerRect.Min.y = window.Pos.y + decoration_up_height;
        window.InnerRect.Max.x = window.Pos.x + window.Size.x - window.ScrollbarSizes.x;
        window.InnerRect.Max.y = window.Pos.y + window.Size.y - window.ScrollbarSizes.y;

        // Inner clipping rectangle.
        // Will extend a little bit outside the normal work region.
        // This is to allow e.g. Selectable or CollapsingHeader or some separators to cover that space.
        // Force round operator last to ensure that e.g. (max.x-min.x) in user's render code produce correct result.
        // Note that if our window is collapsed we will end up with an inverted (~null) clipping rectangle which is the correct behavior.
        // Affected by window/frame border size. Used by:
        // - Begin() initial clip rect
        float top_border_size = (((flags & ImGuiWindowFlags_MenuBar) || !(flags & ImGuiWindowFlags_NoTitleBar)) ? style.FrameBorderSize : window.WindowBorderSize);
        window.InnerClipRect.Min.x = ImFloor(0.5 + window.InnerRect.Min.x + ImMax(ImFloor(window.WindowPadding.x * 0.5), window.WindowBorderSize));
        window.InnerClipRect.Min.y = ImFloor(0.5 + window.InnerRect.Min.y + top_border_size);
        window.InnerClipRect.Max.x = ImFloor(0.5 + window.InnerRect.Max.x - ImMax(ImFloor(window.WindowPadding.x * 0.5), window.WindowBorderSize));
        window.InnerClipRect.Max.y = ImFloor(0.5 + window.InnerRect.Max.y - window.WindowBorderSize);
        window.InnerClipRect.ClipWithFull(host_rect);

        // Default item width. Make it proportional to window size if window manually resizes
        if (window.Size.x > 0.0 && !(flags & ImGuiWindowFlags_Tooltip) && !(flags & ImGuiWindowFlags_AlwaysAutoResize))
            window.ItemWidthDefault = ImFloor(window.Size.x * 0.65);
        else
            window.ItemWidthDefault = ImFloor(g.FontSize * 16.0);

        // SCROLLING

        // Lock down maximum scrolling
        // The value of scroll_max are ahead from scrollbar_x/scrollbar_y which is intentionally using inner_rect from previous rect in order to accommodate
        // for right/bottom aligned items without creating a scrollbar.
        window.ScrollMax.x = ImMax(0.0, window.ContentSize.x + window.WindowPadding.x * 2.0 - window.InnerRect.GetWidth());
        window.ScrollMax.y = ImMax(0.0, window.ContentSize.y + window.WindowPadding.y * 2.0 - window.InnerRect.GetHeight());

        // Apply scrolling
        window.Scroll = CalcNextScrollFromScrollTargetAndClamp(window);
        window.ScrollTarget = DimgVec2D::new(FLT_MAX, FLT_MAX);

        // DRAWING

        // Setup draw list and outer clipping rectangle
        IM_ASSERT(window.DrawList->CmdBuffer.Size == 1 && window.DrawList->CmdBuffer[0].ElemCount == 0);
        window.DrawList->PushTextureID(g.Font->ContainerAtlas->TexID);
        PushClipRect(host_rect.Min, host_rect.Max, false);

        // Child windows can render their decoration (bg color, border, scrollbars, etc.) within their parent to save a draw call (since 1.71)
        // When using overlapping child windows, this will break the assumption that child z-order is mapped to submission order.
        // FIXME: User code may rely on explicit sorting of overlapping child window and would need to disable this somehow. Please get in contact if you are affected (github #4493)
        const bool is_undocked_or_docked_visible = !window.DockIsActive || window.DockTabIsVisible;
        if (is_undocked_or_docked_visible)
        {
            bool render_decorations_in_parent = false;
            if ((flags & ImGuiWindowFlags_ChildWindow) && !(flags & ImGuiWindowFlags_Popup) && !window_is_child_tooltip)
            {
                // - We test overlap with the previous child window only (testing all would end up being O(log N) not a good investment here)
                // - We disable this when the parent window has zero vertices, which is a common pattern leading to laying out multiple overlapping childs
                ImGuiWindow* previous_child = parent_window.DC.ChildWindows.Size >= 2 ? parent_window.DC.ChildWindows[parent_window.DC.ChildWindows.Size - 2] : NULL;
                bool previous_child_overlapping = previous_child ? previous_child->Rect().Overlaps(window.Rect()) : false;
                bool parent_is_empty = parent_window.DrawList->VtxBuffer.Size > 0;
                if (window.DrawList->CmdBuffer.back().ElemCount == 0 && parent_is_empty && !previous_child_overlapping)
                    render_decorations_in_parent = true;
            }
            if (render_decorations_in_parent)
                window.DrawList = parent_window.DrawList;

            // Handle title bar, scrollbar, resize grips and resize borders
            const ImGuiWindow* window_to_highlight = g.NavWindowingTarget ? g.NavWindowingTarget : g.nav_window;
            const bool title_bar_is_highlight = want_focus || (window_to_highlight && (window.RootWindowForTitleBarHighlight == window_to_highlight->RootWindowForTitleBarHighlight || (window.DockNode && window.DockNode == window_to_highlight->DockNode)));
            RenderWindowDecorations(window, title_bar_rect, title_bar_is_highlight, handle_borders_and_resize_grips, resize_grip_count, resize_grip_col, resize_grip_draw_size);

            if (render_decorations_in_parent)
                window.DrawList = &window.DrawListInst;
        }

        // UPDATE RECTANGLES (2- THOSE AFFECTED BY SCROLLING)

        // Work rectangle.
        // Affected by window padding and border size. Used by:
        // - Columns() for right-most edge
        // - TreeNode(), CollapsingHeader() for right-most edge
        // - BeginTabBar() for right-most edge
        const bool allow_scrollbar_x = !(flags & ImGuiWindowFlags_NoScrollbar) && (flags & ImGuiWindowFlags_HorizontalScrollbar);
        const bool allow_scrollbar_y = !(flags & ImGuiWindowFlags_NoScrollbar);
        const float work_rect_size_x = (window.ContentSizeExplicit.x != 0.0 ? window.ContentSizeExplicit.x : ImMax(allow_scrollbar_x ? window.ContentSize.x : 0.0, window.Size.x - window.WindowPadding.x * 2.0 - window.ScrollbarSizes.x));
        const float work_rect_size_y = (window.ContentSizeExplicit.y != 0.0 ? window.ContentSizeExplicit.y : ImMax(allow_scrollbar_y ? window.ContentSize.y : 0.0, window.Size.y - window.WindowPadding.y * 2.0 - decoration_up_height - window.ScrollbarSizes.y));
        window.WorkRect.Min.x = ImFloor(window.InnerRect.Min.x - window.Scroll.x + ImMax(window.WindowPadding.x, window.WindowBorderSize));
        window.WorkRect.Min.y = ImFloor(window.InnerRect.Min.y - window.Scroll.y + ImMax(window.WindowPadding.y, window.WindowBorderSize));
        window.WorkRect.Max.x = window.WorkRect.Min.x + work_rect_size_x;
        window.WorkRect.Max.y = window.WorkRect.Min.y + work_rect_size_y;
        window.ParentWorkRect = window.WorkRect;

        // [LEGACY] Content Region
        // FIXME-OBSOLETE: window->content_region_rect.max is currently very misleading / partly faulty, but some BeginChild() patterns relies on it.
        // Used by:
        // - Mouse wheel scrolling + many other things
        window.ContentRegionRect.Min.x = window.Pos.x - window.Scroll.x + window.WindowPadding.x;
        window.ContentRegionRect.Min.y = window.Pos.y - window.Scroll.y + window.WindowPadding.y + decoration_up_height;
        window.ContentRegionRect.Max.x = window.ContentRegionRect.Min.x + (window.ContentSizeExplicit.x != 0.0 ? window.ContentSizeExplicit.x : (window.Size.x - window.WindowPadding.x * 2.0 - window.ScrollbarSizes.x));
        window.ContentRegionRect.Max.y = window.ContentRegionRect.Min.y + (window.ContentSizeExplicit.y != 0.0 ? window.ContentSizeExplicit.y : (window.Size.y - window.WindowPadding.y * 2.0 - decoration_up_height - window.ScrollbarSizes.y));

        // Setup drawing context
        // (NB: That term "drawing context / dc" lost its meaning a long time ago. Initially was meant to hold transient data only. Nowadays difference between window-> and window->dc-> is dubious.)
        window.DC.Indent.x = 0.0 + window.WindowPadding.x - window.Scroll.x;
        window.DC.GroupOffset.x = 0.0;
        window.DC.ColumnsOffset.x = 0.0;

        // Record the loss of precision of CursorStartPos which can happen due to really large scrolling amount.
        // This is used by clipper to compensate and fix the most common use case of large scroll area. Easy and cheap, next best thing compared to switching everything to double or ImU64.
        double start_pos_highp_x = (double)window.Pos.x + window.WindowPadding.x - (double)window.Scroll.x + window.DC.ColumnsOffset.x;
        double start_pos_highp_y = (double)window.Pos.y + window.WindowPadding.y - (double)window.Scroll.y + decoration_up_height;
        window.DC.CursorStartPos  = DimgVec2D::new((float)start_pos_highp_x, (float)start_pos_highp_y);
        window.DC.CursorStartPosLossyness = DimgVec2D::new((float)(start_pos_highp_x - window.DC.CursorStartPos.x), (float)(start_pos_highp_y - window.DC.CursorStartPos.y));
        window.DC.CursorPos = window.DC.CursorStartPos;
        window.DC.CursorPosPrevLine = window.DC.CursorPos;
        window.DC.CursorMaxPos = window.DC.CursorStartPos;
        window.DC.IdealMaxPos = window.DC.CursorStartPos;
        window.DC.CurrLineSize = window.DC.PrevLineSize = DimgVec2D::new(0.0, 0.0);
        window.DC.CurrLineTextBaseOffset = window.DC.PrevLineTextBaseOffset = 0.0;
        window.DC.IsSameLine = false;

        window.DC.NavLayerCurrent = ImGuiNavLayer_Main;
        window.DC.NavLayersActiveMask = window.DC.NavLayersActiveMaskNext;
        window.DC.NavHideHighlightOneFrame = false;
        window.DC.NavHasScroll = (window.ScrollMax.y > 0.0);

        window.DC.MenuBarAppending = false;
        window.DC.MenuColumns.Update(style.ItemSpacing.x, window_just_activated_by_user);
        window.DC.TreeDepth = 0;
        window.DC.TreeJumpToParentOnPopMask = 0x00;
        window.DC.ChildWindows.resize(0);
        window.DC.StateStorage = &window.StateStorage;
        window.DC.CurrentColumns = NULL;
        window.DC.LayoutType = ImGuiLayoutType_Vertical;
        window.DC.ParentLayoutType = parent_window ? parent_window.DC.LayoutType : ImGuiLayoutType_Vertical;

        window.DC.ItemWidth = window.ItemWidthDefault;
        window.DC.TextWrapPos = -1.0; // disabled
        window.DC.ItemWidthStack.resize(0);
        window.DC.TextWrapPosStack.resize(0);

        if (window.AutoFitFramesX > 0)
            window.AutoFitFramesX--;
        if (window.AutoFitFramesY > 0)
            window.AutoFitFramesY--;

        // Apply focus (we need to call focus_window() AFTER setting dc.CursorStartPos so our initial navigation reference rectangle can start around there)
        if (want_focus)
        {
            focus_window(window);
            NavInitWindow(window, false); // <-- this is in the way for us to be able to defer and sort reappearing focus_window() calls
        }

        // Close requested by platform window
        if (p_open != NULL && window.viewport->PlatformRequestClose && window.viewport != GetMainViewport())
        {
            if (!window.DockIsActive || window.DockTabIsVisible)
            {
                window.viewport->PlatformRequestClose = false;
                g.NavWindowingToggleLayer = false; // Assume user mapped platform_request_close on ALT-F4 so we disable ALT for menu toggle. False positive not an issue.
                IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Window '%s' platform_request_close\n", window.Name);
                *p_open = false;
            }
        }

        // Title bar
        if (!(flags & ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive)
            RenderWindowTitleBarContents(window, ImRect(title_bar_rect.Min.x + window.WindowBorderSize, title_bar_rect.Min.y, title_bar_rect.Max.x - window.WindowBorderSize, title_bar_rect.Max.y), name, p_open);

        // clear hit test shape every frame
        window.HitTestHoleSize.x = window.HitTestHoleSize.y = 0;

        // Pressing CTRL+C while holding on a window copy its content to the clipboard
        // This works but 1. doesn't handle multiple Begin/End pairs, 2. recursing into another Begin/End pair - so we need to work that out and add better logging scope.
        // Maybe we can support CTRL+C on every element?
        /*
        //if (g.nav_window == window && g.active_id == 0)
        if (g.active_id == window->move_id)
            if (g.io.key_ctrl && IsKeyPressedMap(ImGuiKey_C))
                LogToClipboard();
        */

        if (g.io.ConfigFlags & ImGuiConfigFlags_DockingEnable)
        {
            // Docking: Dragging a dockable window (or any of its child) turns it into a drag and drop source.
            // We need to do this _before_ we overwrite window->dc.LastItemId below because BeginDockableDragDropSource() also overwrites it.
            if ((g.moving_window == window) && (g.io.ConfigDockingWithShift == g.io.KeyShift))
                if ((window.RootWindowDockTree.flags & ImGuiWindowFlags_NoDocking) == 0)
                    BeginDockableDragDropSource(window);

            // Docking: Any dockable window can act as a target. For dock node hosts we call BeginDockableDragDropTarget() in DockNodeUpdate() instead.
            if (g.DragDropActive && !(flags & ImGuiWindowFlags_NoDocking))
                if (g.moving_window == NULL || g.moving_window->RootWindowDockTree != window)
                    if ((window == window.RootWindowDockTree) && !(window.Flags & ImGuiWindowFlags_DockNodeHost))
                        BeginDockableDragDropTarget(window);
        }

        // We fill last item data based on Title Bar/Tab, in order for IsItemHovered() and IsItemActive() to be usable after Begin().
        // This is useful to allow creating context menus on title bar only, etc.
        if (window.DockIsActive)
            SetLastItemData(window.MoveId, g.CurrentItemFlags, window.DockTabItemStatusFlags, window.DockTabItemRect);
        else
            SetLastItemData(window.MoveId, g.CurrentItemFlags, IsMouseHoveringRect(title_bar_rect.Min, title_bar_rect.Max, false) ? ImGuiItemStatusFlags_HoveredRect : 0, title_bar_rect);

        // [Test Engine] Register title bar / tab
        if (!(window.Flags & ImGuiWindowFlags_NoTitleBar))
            IMGUI_TEST_ENGINE_ITEM_ADD(g.last_item_data.Rect, g.last_item_data.ID);
    }
    else
    {
        // Append
        SetCurrentViewport(window, window.viewport);
        SetCurrentWindow(window);
    }

    // Pull/inherit current state
    window.DC.NavFocusScopeIdCurrent = (flags & ImGuiWindowFlags_ChildWindow) ? parent_window.DC.NavFocusScopeIdCurrent : window.GetID("#FOCUSSCOPE"); // Inherit from parent only // -V595

    if (!(flags & ImGuiWindowFlags_DockNodeHost))
        PushClipRect(window.InnerClipRect.Min, window.InnerClipRect.Max, true);

    // clear 'accessed' flag last thing (After push_clip_rect which will set the flag. We want the flag to stay false when the default "Debug" window is unused)
    window.WriteAccessed = false;
    window.BeginCount += 1;
    g.NextWindowData.ClearFlags();

    // Update visibility
    if (first_begin_of_the_frame)
    {
        // When we are about to select this tab (which will only be visible on the _next frame_), flag it with a non-zero hidden_frames_cannot_skip_items.
        // This will have the important effect of actually returning true in Begin() and not setting skip_items, allowing an earlier submission of the window contents.
        // This is analogous to regular windows being hidden from one frame.
        // It is especially important as e.g. nested tab_bars would otherwise generate flicker in the form of one empty frame, or focus requests won't be processed.
        if (window.DockIsActive && !window.DockTabIsVisible)
        {
            if (window.LastFrameJustFocused == g.FrameCount)
                window.HiddenFramesCannotSkipItems = 1;
            else
                window.HiddenFramesCanSkipItems = 1;
        }

        if (flags & ImGuiWindowFlags_ChildWindow)
        {
            // Child window can be out of sight and have "negative" clip windows.
            // Mark them as collapsed so commands are skipped earlier (we can't manually collapse them because they have no title bar).
            IM_ASSERT((flags& ImGuiWindowFlags_NoTitleBar) != 0 || (window.DockIsActive));
            if (!(flags & ImGuiWindowFlags_AlwaysAutoResize) && window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0) // FIXME: Doesn't make sense for ChildWindow??
            {
                const bool nav_request = (flags & ImGuiWindowFlags_NavFlattened) && (g.NavAnyRequest && g.nav_window && g.nav_window->RootWindowForNav == window.RootWindowForNav);
                if (!g.LogEnabled && !nav_request)
                    if (window.OuterRectClipped.Min.x >= window.OuterRectClipped.Max.x || window.OuterRectClipped.Min.y >= window.OuterRectClipped.Max.y)
                        window.HiddenFramesCanSkipItems = 1;
            }

            // Hide along with parent or if parent is collapsed
            if (parent_window && (parent_window.Collapsed || parent_window.HiddenFramesCanSkipItems > 0))
                window.HiddenFramesCanSkipItems = 1;
            if (parent_window && (parent_window.Collapsed || parent_window.HiddenFramesCannotSkipItems > 0))
                window.HiddenFramesCannotSkipItems = 1;
        }

        // Don't render if style alpha is 0.0 at the time of Begin(). This is arbitrary and inconsistent but has been there for a long while (may remove at some point)
        if (style.Alpha <= 0.0)
            window.HiddenFramesCanSkipItems = 1;

        // Update the hidden flag
        bool hidden_regular = (window.HiddenFramesCanSkipItems > 0) || (window.HiddenFramesCannotSkipItems > 0);
        window.Hidden = hidden_regular || (window.HiddenFramesForRenderOnly > 0);

        // Disable inputs for requested number of frames
        if (window.DisableInputsFrames > 0)
        {
            window.DisableInputsFrames--;
            window.Flags |= ImGuiWindowFlags_NoInputs;
        }

        // Update the skip_items flag, used to early out of all items functions (no layout required)
        bool skip_items = false;
        if (window.Collapsed || !window.Active || hidden_regular)
            if (window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0 && window.HiddenFramesCannotSkipItems <= 0)
                skip_items = true;
        window.SkipItems = skip_items;

        // Only clear NavLayersActiveMaskNext when marked as visible, so a CTRL+Tab back can use a safe value.
        if (!window.SkipItems)
            window.DC.NavLayersActiveMaskNext = 0x00;

        // Sanity check: there are two spots which can set appearing = true
        // - when 'window_just_activated_by_user' is set -> hidden_frames_cannot_skip_items is set -> skip_items always false
        // - in BeginDocked() path when DockNodeIsVisible == dock_tab_is_visible == true -> hidden _should_ be all zero // FIXME: Not formally proven, hence the assert.
        if (window.SkipItems && !window.Appearing)
            IM_ASSERT(window.Appearing == false); // Please report on GitHub if this triggers: https://github.com/ocornut/imgui/issues/4177
    }

    return !window.SkipItems;
}

void ImGui::End()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;

    // Error checking: verify that user hasn't called End() too many times!
    if (g.CurrentWindowStack.Size <= 1 && g.WithinFrameScopeWithImplicitWindow)
    {
        IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size > 1, "Calling End() too many times!");
        return;
    }
    IM_ASSERT(g.CurrentWindowStack.Size > 0);

    // Error checking: verify that user doesn't directly call End() on a child window.
    if ((window.Flags & ImGuiWindowFlags_ChildWindow) && !(window.Flags & ImGuiWindowFlags_DockNodeHost) && !window.DockIsActive)
        IM_ASSERT_USER_ERROR(g.WithinEndChild, "Must call EndChild() and not End()!");

    // Close anything that is open
    if (window.DC.CurrentColumns)
        EndColumns();
    if (!(window.Flags & ImGuiWindowFlags_DockNodeHost))   // Pop inner window clip rectangle
        PopClipRect();

    // Stop logging
    if (!(window.Flags & ImGuiWindowFlags_ChildWindow))    // FIXME: add more options for scope of logging
        LogFinish();

    // Docking: report contents sizes to parent to allow for auto-resize
    if (window.DockNode && window.DockTabIsVisible)
        if (ImGuiWindow* host_window = window.DockNode->HostWindow)         // FIXME-DOCK
            host_window.DC.CursorMaxPos = window.DC.CursorMaxPos + window.WindowPadding - host_window.WindowPadding;

    // Pop from window stack
    g.last_item_data = g.CurrentWindowStack.back().ParentLastItemDataBackup;
    if (window.Flags & ImGuiWindowFlags_ChildMenu)
        g.BeginMenuCount--;
    if (window.Flags & ImGuiWindowFlags_Popup)
        g.BeginPopupStack.pop_back();
    g.CurrentWindowStack.back().StackSizesOnBegin.CompareWithCurrentState();
    g.CurrentWindowStack.pop_back();
    SetCurrentWindow(g.CurrentWindowStack.Size == 0 ? NULL : g.CurrentWindowStack.back().Window);
    if (g.CurrentWindow)
        SetCurrentViewport(g.CurrentWindow, g.CurrentWindow->Viewport);
}

void ImGui::BringWindowToFocusFront(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(window == window.RootWindow);

    const int cur_order = window.FocusOrder;
    IM_ASSERT(g.WindowsFocusOrder[cur_order] == window);
    if (g.WindowsFocusOrder.back() == window)
        return;

    const int new_order = g.WindowsFocusOrder.Size - 1;
    for (int n = cur_order; n < new_order; n += 1)
    {
        g.WindowsFocusOrder[n] = g.WindowsFocusOrder[n + 1];
        g.WindowsFocusOrder[n]->FocusOrder--;
        IM_ASSERT(g.WindowsFocusOrder[n]->FocusOrder == n);
    }
    g.WindowsFocusOrder[new_order] = window;
    window.FocusOrder = (short)new_order;
}

void ImGui::BringWindowToDisplayFront(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* current_front_window = g.Windows.back();
    if (current_front_window == window || current_front_window.RootWindowDockTree == window) // Cheap early out (could be better)
        return;
    for (int i = g.Windows.Size - 2; i >= 0; i--) // We can ignore the top-most window
        if (g.Windows[i] == window)
        {
            memmove(&g.Windows[i], &g.Windows[i + 1], (g.Windows.Size - i - 1) * sizeof(ImGuiWindow*));
            g.Windows[g.Windows.Size - 1] = window;
            break;
        }
}

void ImGui::BringWindowToDisplayBack(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    if (g.Windows[0] == window)
        return;
    for (int i = 0; i < g.Windows.Size; i += 1)
        if (g.Windows[i] == window)
        {
            memmove(&g.Windows[1], &g.Windows[0], i * sizeof(ImGuiWindow*));
            g.Windows[0] = window;
            break;
        }
}

void ImGui::BringWindowToDisplayBehind(ImGuiWindow* window, ImGuiWindow* behind_window)
{
    IM_ASSERT(window != NULL && behind_window != NULL);
    ImGuiContext& g = *GImGui;
    window = window.RootWindow;
    behind_window = behind_window.RootWindow;
    int pos_wnd = FindWindowDisplayIndex(window);
    int pos_beh = FindWindowDisplayIndex(behind_window);
    if (pos_wnd < pos_beh)
    {
        size_t copy_bytes = (pos_beh - pos_wnd - 1) * sizeof(ImGuiWindow*);
        memmove(&g.Windows.Data[pos_wnd], &g.Windows.Data[pos_wnd + 1], copy_bytes);
        g.Windows[pos_beh - 1] = window;
    }
    else
    {
        size_t copy_bytes = (pos_wnd - pos_beh) * sizeof(ImGuiWindow*);
        memmove(&g.Windows.Data[pos_beh + 1], &g.Windows.Data[pos_beh], copy_bytes);
        g.Windows[pos_beh] = window;
    }
}

int ImGui::FindWindowDisplayIndex(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    return g.Windows.index_from_ptr(g.Windows.find(window));
}

// Moving window to front of display and set focus (which happens to be back of our sorted list)
void ImGui::focus_window(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;

    if (g.nav_window != window)
    {
        SetNavWindow(window);
        if (window && g.NavDisableMouseHover)
            g.NavMousePosDirty = true;
        g.NavId = window ? window.NavLastIds[0] : 0; // Restore nav_id
        g.NavLayer = ImGuiNavLayer_Main;
        g.NavFocusScopeId = 0;
        g.NavIdIsAlive = false;
    }

    // Close popups if any
    ClosePopupsOverWindow(window, false);

    // Move the root window to the top of the pile
    IM_ASSERT(window == NULL || window.RootWindowDockTree != NULL);
    ImGuiWindow* focus_front_window = window ? window.RootWindow : NULL;
    ImGuiWindow* display_front_window = window ? window.RootWindowDockTree : NULL;
    ImGuiDockNode* dock_node = window ? window.DockNode : NULL;
    bool active_id_window_is_dock_node_host = (g.active_id_window && dock_node && dock_node->HostWindow == g.active_id_window);

    // Steal active widgets. Some of the cases it triggers includes:
    // - Focus a window while an InputText in another window is active, if focus happens before the old InputText can run.
    // - When using Nav to activate menu items (due to timing of activating on press->new window appears->losing active_id)
    // - Using dock host items (tab, collapse button) can trigger this before we redirect the active_id_window toward the child window.
    if (g.active_id != 0 && g.active_id_window && g.active_id_window->RootWindow != focus_front_window)
        if (!g.ActiveIdNoClearOnFocusLoss && !active_id_window_is_dock_node_host)
            clear_active_id();

    // Passing NULL allow to disable keyboard focus
    if (!window)
        return;
    window.LastFrameJustFocused = g.FrameCount;

    // Select in dock node
    if (dock_node && dock_node->TabBar)
        dock_node->TabBar->SelectedTabId = dock_node->TabBar->NextSelectedTabId = window.TabId;

    // Bring to front
    BringWindowToFocusFront(focus_front_window);
    if (((window.Flags | focus_front_window.Flags | display_front_window.Flags) & ImGuiWindowFlags_NoBringToFrontOnFocus) == 0)
        BringWindowToDisplayFront(display_front_window);
}

void ImGui::FocusTopMostWindowUnderOne(ImGuiWindow* under_this_window, ImGuiWindow* ignore_window)
{
    ImGuiContext& g = *GImGui;
    int start_idx = g.WindowsFocusOrder.Size - 1;
    if (under_this_window != NULL)
    {
        // Aim at root window behind us, if we are in a child window that's our own root (see #4640)
        int offset = -1;
        while (under_this_window.Flags & ImGuiWindowFlags_ChildWindow)
        {
            under_this_window = under_this_window.ParentWindow;
            offset = 0;
        }
        start_idx = FindWindowFocusIndex(under_this_window) + offset;
    }
    for (int i = start_idx; i >= 0; i--)
    {
        // We may later decide to test for different NoXXXInputs based on the active navigation input (mouse vs nav) but that may feel more confusing to the user.
        ImGuiWindow* window = g.WindowsFocusOrder[i];
        IM_ASSERT(window == window.RootWindow);
        if (window != ignore_window && window.WasActive)
            if ((window.Flags & (ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs)) != (ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs))
            {
                // FIXME-DOCK: This is failing (lagging by one frame) for docked windows.
                // If A and B are docked into window and B disappear, at the NewFrame() call site window->nav_last_child_nav_window will still point to B.
                // We might leverage the tab order implicitly stored in window->dock_node_as_host->tab_bar (essentially the 'most_recently_selected_tab' code in tab bar will do that but on next update)
                // to tell which is the "previous" window. Or we may leverage 'LastFrameFocused/last_frame_just_focused' and have this function handle child window itself?
                ImGuiWindow* focus_window = NavRestoreLastChildNavWindow(window);
                focus_window(focus_window);
                return;
            }
    }
    focus_window(NULL);
}

// Important: this alone doesn't alter current ImDrawList state. This is called by PushFont/PopFont only.
void ImGui::SetCurrentFont(ImFont* font)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(font && font->IsLoaded());    // font Atlas not created. Did you call io.fonts->GetTexDataAsRGBA32 / GetTexDataAsAlpha8 ?
    IM_ASSERT(font->Scale > 0.0);
    g.Font = font;
    g.FontBaseSize = ImMax(1.0, g.io.FontGlobalScale * g.Font->FontSize * g.Font->Scale);
    g.FontSize = g.CurrentWindow ? g.CurrentWindow->CalcFontSize() : 0.0;

    ImFontAtlas* atlas = g.Font->ContainerAtlas;
    g.DrawListSharedData.TexUvWhitePixel = atlas->TexUvWhitePixel;
    g.DrawListSharedData.TexUvLines = atlas->TexUvLines;
    g.DrawListSharedData.Font = g.Font;
    g.DrawListSharedData.FontSize = g.FontSize;
}

void ImGui::PushFont(ImFont* font)
{
    ImGuiContext& g = *GImGui;
    if (!font)
        font = GetDefaultFont();
    SetCurrentFont(font);
    g.FontStack.push_back(font);
    g.CurrentWindow->DrawList->PushTextureID(font->ContainerAtlas->TexID);
}

void  ImGui::PopFont()
{
    ImGuiContext& g = *GImGui;
    g.CurrentWindow->DrawList->PopTextureID();
    g.FontStack.pop_back();
    SetCurrentFont(g.FontStack.empty() ? GetDefaultFont() : g.FontStack.back());
}

void ImGui::PushItemFlag(ImGuiItemFlags option, bool enabled)
{
    ImGuiContext& g = *GImGui;
    ImGuiItemFlags item_flags = g.CurrentItemFlags;
    IM_ASSERT(item_flags == g.ItemFlagsStack.back());
    if (enabled)
        item_flags |= option;
    else
        item_flags &= ~option;
    g.CurrentItemFlags = item_flags;
    g.ItemFlagsStack.push_back(item_flags);
}

void ImGui::PopItemFlag()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.ItemFlagsStack.Size > 1); // Too many calls to PopItemFlag() - we always leave a 0 at the bottom of the stack.
    g.ItemFlagsStack.pop_back();
    g.CurrentItemFlags = g.ItemFlagsStack.back();
}

// BeginDisabled()/EndDisabled()
// - Those can be nested but it cannot be used to enable an already disabled section (a single BeginDisabled(true) in the stack is enough to keep everything disabled)
// - Visually this is currently altering alpha, but it is expected that in a future styling system this would work differently.
// - Feedback welcome at https://github.com/ocornut/imgui/issues/211
// - BeginDisabled(false) essentially does nothing useful but is provided to facilitate use of boolean expressions. If you can avoid calling BeginDisabled(False)/EndDisabled() best to avoid it.
// - Optimized shortcuts instead of PushStyleVar() + PushItemFlag()
void ImGui::BeginDisabled(bool disabled)
{
    ImGuiContext& g = *GImGui;
    bool was_disabled = (g.CurrentItemFlags & ImGuiItemFlags_Disabled) != 0;
    if (!was_disabled && disabled)
    {
        g.DisabledAlphaBackup = g.Style.Alpha;
        g.Style.Alpha *= g.Style.DisabledAlpha; // PushStyleVar(ImGuiStyleVar_Alpha, g.style.Alpha * g.style.DisabledAlpha);
    }
    if (was_disabled || disabled)
        g.CurrentItemFlags |= ImGuiItemFlags_Disabled;
    g.ItemFlagsStack.push_back(g.CurrentItemFlags);
    g.DisabledStackSize += 1;
}

void ImGui::EndDisabled()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.DisabledStackSize > 0);
    g.DisabledStackSize--;
    bool was_disabled = (g.CurrentItemFlags & ImGuiItemFlags_Disabled) != 0;
    //PopItemFlag();
    g.ItemFlagsStack.pop_back();
    g.CurrentItemFlags = g.ItemFlagsStack.back();
    if (was_disabled && (g.CurrentItemFlags & ImGuiItemFlags_Disabled) == 0)
        g.Style.Alpha = g.DisabledAlphaBackup; //PopStyleVar();
}

// FIXME: Look into renaming this once we have settled the new Focus/Activation/TabStop system.
void ImGui::PushAllowKeyboardFocus(bool allow_keyboard_focus)
{
    PushItemFlag(ImGuiItemFlags_NoTabStop, !allow_keyboard_focus);
}

void ImGui::PopAllowKeyboardFocus()
{
    PopItemFlag();
}

void ImGui::PushButtonRepeat(bool repeat)
{
    PushItemFlag(ImGuiItemFlags_ButtonRepeat, repeat);
}

void ImGui::PopButtonRepeat()
{
    PopItemFlag();
}

void ImGui::PushTextWrapPos(float wrap_pos_x)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.TextWrapPosStack.push_back(window.DC.TextWrapPos);
    window.DC.TextWrapPos = wrap_pos_x;
}

void ImGui::PopTextWrapPos()
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.TextWrapPos = window.DC.TextWrapPosStack.back();
    window.DC.TextWrapPosStack.pop_back();
}

static ImGuiWindow* GetCombinedRootWindow(ImGuiWindow* window, bool popup_hierarchy, bool dock_hierarchy)
{
    ImGuiWindow* last_window = NULL;
    while (last_window != window)
    {
        last_window = window;
        window = window.RootWindow;
        if (popup_hierarchy)
            window = window.RootWindowPopupTree;
		if (dock_hierarchy)
			window = window.RootWindowDockTree;
	}
    return window;
}

bool ImGui::IsWindowChildOf(ImGuiWindow* window, ImGuiWindow* potential_parent, bool popup_hierarchy, bool dock_hierarchy)
{
    ImGuiWindow* window_root = GetCombinedRootWindow(window, popup_hierarchy, dock_hierarchy);
    if (window_root == potential_parent)
        return true;
    while (window != NULL)
    {
        if (window == potential_parent)
            return true;
        if (window == window_root) // end of chain
            return false;
        window = window.ParentWindow;
    }
    return false;
}

bool ImGui::IsWindowWithinBeginStackOf(ImGuiWindow* window, ImGuiWindow* potential_parent)
{
    if (window.RootWindow == potential_parent)
        return true;
    while (window != NULL)
    {
        if (window == potential_parent)
            return true;
        window = window.ParentWindowInBeginStack;
    }
    return false;
}

bool ImGui::IsWindowAbove(ImGuiWindow* potential_above, ImGuiWindow* potential_below)
{
    ImGuiContext& g = *GImGui;

    // It would be saner to ensure that display layer is always reflected in the g.windows[] order, which would likely requires altering all manipulations of that array
    const int display_layer_delta = GetWindowDisplayLayer(potential_above) - GetWindowDisplayLayer(potential_below);
    if (display_layer_delta != 0)
        return display_layer_delta > 0;

    for (int i = g.Windows.Size - 1; i >= 0; i--)
    {
        ImGuiWindow* candidate_window = g.Windows[i];
        if (candidate_window == potential_above)
            return true;
        if (candidate_window == potential_below)
            return false;
    }
    return false;
}

bool ImGui::IsWindowHovered(ImGuiHoveredFlags flags)
{
    IM_ASSERT((flags & (ImGuiHoveredFlags_AllowWhenOverlapped | ImGuiHoveredFlags_AllowWhenDisabled)) == 0);   // flags not supported by this function
    ImGuiContext& g = *GImGui;
    ImGuiWindow* ref_window = g.hovered_window;
    ImGuiWindow* cur_window = g.CurrentWindow;
    if (ref_window == NULL)
        return false;

    if ((flags & ImGuiHoveredFlags_AnyWindow) == 0)
    {
        IM_ASSERT(cur_window); // Not inside a Begin()/End()
        const bool popup_hierarchy = (flags & ImGuiHoveredFlags_NoPopupHierarchy) == 0;
        const bool dock_hierarchy = (flags & ImGuiHoveredFlags_DockHierarchy) != 0;
        if (flags & ImGuiHoveredFlags_RootWindow)
            cur_window = GetCombinedRootWindow(cur_window, popup_hierarchy, dock_hierarchy);

        bool result;
        if (flags & ImGuiHoveredFlags_ChildWindows)
            result = IsWindowChildOf(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
        else
            result = (ref_window == cur_window);
        if (!result)
            return false;
    }

    if (!IsWindowContentHoverable(ref_window, flags))
        return false;
    if (!(flags & ImGuiHoveredFlags_AllowWhenBlockedByActiveItem))
        if (g.active_id != 0 && !g.ActiveIdAllowOverlap && g.active_id != ref_window.MoveId)
            return false;
    return true;
}

bool ImGui::IsWindowFocused(ImGuiFocusedFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* ref_window = g.nav_window;
    ImGuiWindow* cur_window = g.CurrentWindow;

    if (ref_window == NULL)
        return false;
    if (flags & ImGuiFocusedFlags_AnyWindow)
        return true;

    IM_ASSERT(cur_window); // Not inside a Begin()/End()
    const bool popup_hierarchy = (flags & ImGuiFocusedFlags_NoPopupHierarchy) == 0;
    const bool dock_hierarchy = (flags & ImGuiFocusedFlags_DockHierarchy) != 0;
    if (flags & ImGuiHoveredFlags_RootWindow)
        cur_window = GetCombinedRootWindow(cur_window, popup_hierarchy, dock_hierarchy);

    if (flags & ImGuiHoveredFlags_ChildWindows)
        return IsWindowChildOf(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
    else
        return (ref_window == cur_window);
}

ImGuiID ImGui::GetWindowDockID()
{
    ImGuiContext& g = *GImGui;
    return g.CurrentWindow->DockId;
}

bool ImGui::IsWindowDocked()
{
    ImGuiContext& g = *GImGui;
    return g.CurrentWindow->DockIsActive;
}

// Can we focus this window with CTRL+TAB (or PadMenu + PadFocusPrev/PadFocusNext)
// Note that NoNavFocus makes the window not reachable with CTRL+TAB but it can still be focused with mouse or programmatically.
// If you want a window to never be focused, you may use the e.g. NoInputs flag.
bool ImGui::IsWindowNavFocusable(ImGuiWindow* window)
{
    return window.WasActive && window == window.RootWindow && !(window.Flags & ImGuiWindowFlags_NoNavFocus);
}

float ImGui::GetWindowWidth()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.Size.x;
}

float ImGui::GetWindowHeight()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.Size.y;
}

Vector2D ImGui::GetWindowPos()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    return window.Pos;
}

void ImGui::set_window_pos(ImGuiWindow* window, const Vector2D& pos, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowPosAllowFlags & cond) == 0)
        return;

    IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowPosAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);
    window.SetWindowPosVal = DimgVec2D::new(FLT_MAX, FLT_MAX);

    // Set
    const Vector2D old_pos = window.Pos;
    window.Pos = ImFloor(pos);
    Vector2D offset = window.Pos - old_pos;
    if (offset.x == 0.0 && offset.y == 0.0)
        return;
    MarkIniSettingsDirty(window);
    // FIXME: share code with TranslateWindow(), need to confirm whether the 3 rect modified by TranslateWindow() are desirable here.
    window.DC.CursorPos += offset;         // As we happen to move the window while it is being appended to (which is a bad idea - will smear) let's at least offset the cursor
    window.DC.CursorMaxPos += offset;      // And more importantly we need to offset CursorMaxPos/CursorStartPos this so content_size calculation doesn't get affected.
    window.DC.IdealMaxPos += offset;
    window.DC.CursorStartPos += offset;
}

void ImGui::set_window_pos(const Vector2D& pos, ImGuiCond cond)
{
    ImGuiWindow* window = GetCurrentWindowRead();
    set_window_pos(window, pos, cond);
}

void ImGui::set_window_pos(const char* name, const Vector2D& pos, ImGuiCond cond)
{
    if (ImGuiWindow* window = FindWindowByName(name))
        set_window_pos(window, pos, cond);
}

Vector2D ImGui::GetWindowSize()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.Size;
}

void ImGui::SetWindowSize(ImGuiWindow* window, const Vector2D& size, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowSizeAllowFlags & cond) == 0)
        return;

    IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowSizeAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    Vector2D old_size = window.SizeFull;
    window.AutoFitFramesX = (size.x <= 0.0) ? 2 : 0;
    window.AutoFitFramesY = (size.y <= 0.0) ? 2 : 0;
    if (size.x <= 0.0)
        window.AutoFitOnlyGrows = false;
    else
        window.SizeFull.x = IM_FLOOR(size.x);
    if (size.y <= 0.0)
        window.AutoFitOnlyGrows = false;
    else
        window.SizeFull.y = IM_FLOOR(size.y);
    if (old_size.x != window.SizeFull.x || old_size.y != window.SizeFull.y)
        MarkIniSettingsDirty(window);
}

void ImGui::SetWindowSize(const Vector2D& size, ImGuiCond cond)
{
    SetWindowSize(GImGui->CurrentWindow, size, cond);
}

void ImGui::SetWindowSize(const char* name, const Vector2D& size, ImGuiCond cond)
{
    if (ImGuiWindow* window = FindWindowByName(name))
        SetWindowSize(window, size, cond);
}

void ImGui::SetWindowCollapsed(ImGuiWindow* window, bool collapsed, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowCollapsedAllowFlags & cond) == 0)
        return;
    window.SetWindowCollapsedAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    window.Collapsed = collapsed;
}

void ImGui::SetWindowHitTestHole(ImGuiWindow* window, const Vector2D& pos, const Vector2D& size)
{
    IM_ASSERT(window.HitTestHoleSize.x == 0);     // We don't support multiple holes/hit test filters
    window.HitTestHoleSize = Vector2Dih(size);
    window.HitTestHoleOffset = Vector2Dih(pos - window.Pos);
}

void ImGui::SetWindowCollapsed(bool collapsed, ImGuiCond cond)
{
    SetWindowCollapsed(GImGui->CurrentWindow, collapsed, cond);
}

bool ImGui::IsWindowCollapsed()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.Collapsed;
}

bool ImGui::IsWindowAppearing()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.Appearing;
}

void ImGui::SetWindowCollapsed(const char* name, bool collapsed, ImGuiCond cond)
{
    if (ImGuiWindow* window = FindWindowByName(name))
        SetWindowCollapsed(window, collapsed, cond);
}

void ImGui::SetWindowFocus()
{
    focus_window(GImGui->CurrentWindow);
}

void ImGui::SetWindowFocus(const char* name)
{
    if (name)
    {
        if (ImGuiWindow* window = FindWindowByName(name))
            focus_window(window);
    }
    else
    {
        focus_window(NULL);
    }
}

void ImGui::SetNextWindowPos(const Vector2D& pos, ImGuiCond cond, const Vector2D& pivot)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasPos;
    g.NextWindowData.PosVal = pos;
    g.NextWindowData.PosPivotVal = pivot;
    g.NextWindowData.PosCond = cond ? cond : Cond::Always;
    g.NextWindowData.PosUndock = true;
}

void ImGui::SetNextWindowSize(const Vector2D& size, ImGuiCond cond)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasSize;
    g.NextWindowData.SizeVal = size;
    g.NextWindowData.SizeCond = cond ? cond : Cond::Always;
}

void ImGui::SetNextWindowSizeConstraints(const Vector2D& size_min, const Vector2D& size_max, ImGuiSizeCallback custom_callback, void* custom_callback_user_data)
{
    ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasSizeConstraint;
    g.NextWindowData.SizeConstraintRect = ImRect(size_min, size_max);
    g.NextWindowData.SizeCallback = custom_callback;
    g.NextWindowData.SizeCallbackUserData = custom_callback_user_data;
}

// Content size = inner scrollable rectangle, padded with window_padding.
// SetNextWindowContentSize(Vector2D(100,100) + ImGuiWindowFlags_AlwaysAutoResize will always allow submitting a 100x100 item.
void ImGui::SetNextWindowContentSize(const Vector2D& size)
{
    ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasContentSize;
    g.NextWindowData.ContentSizeVal = ImFloor(size);
}

void ImGui::SetNextWindowScroll(const Vector2D& scroll)
{
    ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasScroll;
    g.NextWindowData.ScrollVal = scroll;
}

void ImGui::SetNextWindowCollapsed(bool collapsed, ImGuiCond cond)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasCollapsed;
    g.NextWindowData.CollapsedVal = collapsed;
    g.NextWindowData.CollapsedCond = cond ? cond : Cond::Always;
}

void ImGui::SetNextWindowFocus()
{
    ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasFocus;
}

void ImGui::SetNextWindowBgAlpha(float alpha)
{
    ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasBgAlpha;
    g.NextWindowData.BgAlphaVal = alpha;
}

void ImGui::SetNextWindowViewport(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasViewport;
    g.NextWindowData.ViewportId = id;
}

void ImGui::SetNextWindowDockID(ImGuiID id, ImGuiCond cond)
{
    ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasDock;
    g.NextWindowData.DockCond = cond ? cond : Cond::Always;
    g.NextWindowData.DockId = id;
}

void ImGui::SetNextWindowClass(const ImGuiWindowClass* window_class)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT((window_class->ViewportFlagsOverrideSet & window_class->ViewportFlagsOverrideClear) == 0); // Cannot set both set and clear for the same bit
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasWindowClass;
    g.NextWindowData.WindowClass = *window_class;
}

ImDrawList* ImGui::GetWindowDrawList()
{
    ImGuiWindow* window = GetCurrentWindow();
    return window.DrawList;
}

float ImGui::GetWindowDpiScale()
{
    ImGuiContext& g = *GImGui;
    return g.CurrentDpiScale;
}

ImGuiViewport* ImGui::GetWindowViewport()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.CurrentViewport != NULL && g.CurrentViewport == g.CurrentWindow->Viewport);
    return g.CurrentViewport;
}

ImFont* ImGui::GetFont()
{
    return GImGui->Font;
}

float ImGui::GetFontSize()
{
    return GImGui->FontSize;
}

Vector2D ImGui::GetFontTexUvWhitePixel()
{
    return GImGui->DrawListSharedData.TexUvWhitePixel;
}

void ImGui::SetWindowFontScale(float scale)
{
    IM_ASSERT(scale > 0.0);
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = GetCurrentWindow();
    window.FontWindowScale = scale;
    g.FontSize = g.DrawListSharedData.FontSize = window.CalcFontSize();
}

void ImGui::ActivateItem(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    g.NavNextActivateId = id;
    g.NavNextActivateFlags = ImGuiActivateFlags_None;
}

void ImGui::PushFocusScope(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    g.FocusScopeStack.push_back(window.DC.NavFocusScopeIdCurrent);
    window.DC.NavFocusScopeIdCurrent = id;
}

void ImGui::PopFocusScope()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    IM_ASSERT(g.FocusScopeStack.Size > 0); // Too many PopFocusScope() ?
    window.DC.NavFocusScopeIdCurrent = g.FocusScopeStack.back();
    g.FocusScopeStack.pop_back();
}

// Note: this will likely be called ActivateItem() once we rework our Focus/Activation system!
void ImGui::SetKeyboardFocusHere(int offset)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    IM_ASSERT(offset >= -1);    // -1 is allowed but not below
    IMGUI_DEBUG_LOG_ACTIVEID("SetKeyboardFocusHere(%d) in window \"%s\"\n", offset, window.Name);

    // It makes sense in the vast majority of cases to never interrupt a drag and drop.
    // When we refactor this function into ActivateItem() we may want to make this an option.
    // moving_window is protected from most user inputs using SetActiveIdUsingNavAndKeys(), but
    // is also automatically dropped in the event g.active_id is stolen.
    if (g.DragDropActive || g.moving_window != NULL)
    {
        IMGUI_DEBUG_LOG_ACTIVEID("SetKeyboardFocusHere() ignored while drag_drop_active!\n");
        return;
    }

    SetNavWindow(window);

    ImGuiScrollFlags scroll_flags = window.Appearing ? ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleEdgeY;
    NavMoveRequestSubmit(ImGuiDir_None, offset < 0 ? ImGuiDir_Up : ImGuiDir_Down, ImGuiNavMoveFlags_Tabbing | ImGuiNavMoveFlags_FocusApi, scroll_flags); // FIXME-NAV: Once we refactor tabbing, add LegacyApi flag to not activate non-inputable.
    if (offset == -1)
    {
        NavMoveRequestResolveWithLastItem(&g.NavMoveResultLocal);
    }
    else
    {
        g.NavTabbingDir = 1;
        g.NavTabbingCounter = offset + 1;
    }
}

void ImGui::SetItemDefaultFocus()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    if (!window.Appearing)
        return;
    if (g.nav_window != window.RootWindowForNav || (!g.NavInitRequest && g.NavInitResultId == 0) || g.NavLayer != window.DC.NavLayerCurrent)
        return;

    g.NavInitRequest = false;
    g.NavInitResultId = g.last_item_data.ID;
    g.NavInitResultRectRel = WindowRectAbsToRel(window, g.last_item_data.Rect);
    NavUpdateAnyRequestFlag();

    // scroll could be done in NavInitRequestApplyResult() via a opt-in flag (we however don't want regular init requests to scroll)
    if (!IsItemVisible())
        ScrollToRectEx(window, g.last_item_data.Rect, ImGuiScrollFlags_None);
}

void ImGui::SetStateStorage(ImGuiStorage* tree)
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    window.DC.StateStorage = tree ? tree : &window.StateStorage;
}

ImGuiStorage* ImGui::GetStateStorage()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.DC.StateStorage;
}

void ImGui::PushID(const char* str_id)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    ImGuiID id = window.GetID(str_id);
    window.IDStack.push_back(id);
}

void ImGui::PushID(const char* str_id_begin, const char* str_id_end)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    ImGuiID id = window.GetID(str_id_begin, str_id_end);
    window.IDStack.push_back(id);
}

void ImGui::PushID(const void* ptr_id)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    ImGuiID id = window.GetID(ptr_id);
    window.IDStack.push_back(id);
}

void ImGui::PushID(int int_id)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    ImGuiID id = window.GetID(int_id);
    window.IDStack.push_back(id);
}

// Push a given id value ignoring the id stack as a seed.
void ImGui::PushOverrideID(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    if (g.DebugHookIdInfo == id)
        DebugHookIdInfo(id, ImGuiDataType_ID, NULL, NULL);
    window.IDStack.push_back(id);
}

// Helper to avoid a common series of PushOverrideID -> GetID() -> PopID() call
// (note that when using this pattern, test_engine's "Stack Tool" will tend to not display the intermediate stack level.
//  for that to work we would need to do PushOverrideID() -> ItemAdd() -> PopID() which would alter widget code a little more)
ImGuiID ImGui::GetIDWithSeed(const char* str, const char* str_end, ImGuiID seed)
{
    ImGuiID id = ImHashStr(str, str_end ? (str_end - str) : 0, seed);
    ImGuiContext& g = *GImGui;
    if (g.DebugHookIdInfo == id)
        DebugHookIdInfo(id, ImGuiDataType_String, str, str_end);
    return id;
}

void ImGui::PopID()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    IM_ASSERT(window.IDStack.Size > 1); // Too many PopID(), or could be popping in a wrong/different window?
    window.IDStack.pop_back();
}

ImGuiID ImGui::GetID(const char* str_id)
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.GetID(str_id);
}

ImGuiID ImGui::GetID(const char* str_id_begin, const char* str_id_end)
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.GetID(str_id_begin, str_id_end);
}

ImGuiID ImGui::GetID(const void* ptr_id)
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.GetID(ptr_id);
}

bool ImGui::IsRectVisible(const Vector2D& size)
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.ClipRect.Overlaps(ImRect(window.DC.CursorPos, window.DC.CursorPos + size));
}

bool ImGui::IsRectVisible(const Vector2D& rect_min, const Vector2D& rect_max)
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.ClipRect.Overlaps(ImRect(rect_min, rect_max));
}


//-----------------------------------------------------------------------------
// [SECTION] INPUTS
//-----------------------------------------------------------------------------

ImGuiKeyData* ImGui::GetKeyData(ImGuiKey key)
{
    ImGuiContext& g = *GImGui;
    int index;
#ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    IM_ASSERT(key >= ImGuiKey_LegacyNativeKey_BEGIN && key < ImGuiKey_NamedKey_END);
    if (IsLegacyKey(key))
        index = (g.io.KeyMap[key] != -1) ? g.io.KeyMap[key] : key; // Remap native->imgui or imgui->native
    else
        index = key;
#else
    IM_ASSERT(IsNamedKey(key) && "Support for user key indices was dropped in favor of ImGuiKey. Please update backend & user code.");
    index = key - ImGuiKey_NamedKey_BEGIN;

    return &g.io.KeysData[index];
}

#ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
int ImGui::GetKeyIndex(ImGuiKey key)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(IsNamedKey(key));
    const ImGuiKeyData* key_data = GetKeyData(key);
    return (key_data - g.io.KeysData);
}


// Those names a provided for debugging purpose and are not meant to be saved persistently not compared.
static const char* const GKeyNames[] =
{
    "Tab", "LeftArrow", "RightArrow", "UpArrow", "DownArrow", "PageUp", "PageDown",
    "Home", "End", "Insert", "Delete", "Backspace", "Space", "Enter", "Escape",
    "LeftCtrl", "LeftShift", "LeftAlt", "LeftSuper", "RightCtrl", "RightShift", "RightAlt", "RightSuper", "Menu",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H",
    "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "x", "Y", "Z",
    "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
    "Apostrophe", "Comma", "Minus", "Period", "Slash", "Semicolon", "Equal", "LeftBracket",
    "Backslash", "RightBracket", "GraveAccent", "CapsLock", "ScrollLock", "NumLock", "PrintScreen",
    "Pause", "Keypad0", "Keypad1", "Keypad2", "Keypad3", "Keypad4", "Keypad5", "Keypad6",
    "Keypad7", "Keypad8", "Keypad9", "KeypadDecimal", "KeypadDivide", "KeypadMultiply",
    "KeypadSubtract", "KeypadAdd", "KeypadEnter", "KeypadEqual",
    "GamepadStart", "GamepadBack", "GamepadFaceUp", "GamepadFaceDown", "GamepadFaceLeft", "GamepadFaceRight",
    "GamepadDpadUp", "GamepadDpadDown", "GamepadDpadLeft", "GamepadDpadRight",
    "GamepadL1", "GamepadR1", "GamepadL2", "GamepadR2", "GamepadL3", "GamepadR3",
    "GamepadLStickUp", "GamepadLStickDown", "GamepadLStickLeft", "GamepadLStickRight",
    "GamepadRStickUp", "GamepadRStickDown", "GamepadRStickLeft", "GamepadRStickRight",
    "ModCtrl", "ModShift", "ModAlt", "ModSuper"
};
IM_STATIC_ASSERT(ImGuiKey_NamedKey_COUNT == IM_ARRAYSIZE(GKeyNames));

const char* ImGui::GetKeyName(ImGuiKey key)
{
#ifdef IMGUI_DISABLE_OBSOLETE_KEYIO
    IM_ASSERT((IsNamedKey(key) || key == ImGuiKey_None) && "Support for user key indices was dropped in favor of ImGuiKey. Please update backend and user code.");
#else
    if (IsLegacyKey(key))
    {
        ImGuiIO& io = GetIO();
        if (io.KeyMap[key] == -1)
            return "N/A";
        IM_ASSERT(IsNamedKey((ImGuiKey)io.KeyMap[key]));
        key = (ImGuiKey)io.KeyMap[key];
    }

    if (key == ImGuiKey_None)
        return "None";
    if (!IsNamedKey(key))
        return "Unknown";

    return GKeyNames[key - ImGuiKey_NamedKey_BEGIN];
}

// t0 = previous time (e.g.: g.time - g.io.delta_time)
// t1 = current time (e.g.: g.time)
// An event is triggered at:
//  t = 0.0     t = repeat_delay,    t = repeat_delay + repeat_rate*N
int ImGui::CalcTypematicRepeatAmount(float t0, float t1, float repeat_delay, float repeat_rate)
{
    if (t1 == 0.0)
        return 1;
    if (t0 >= t1)
        return 0;
    if (repeat_rate <= 0.0)
        return (t0 < repeat_delay) && (t1 >= repeat_delay);
    const int count_t0 = (t0 < repeat_delay) ? -1 : ((t0 - repeat_delay) / repeat_rate);
    const int count_t1 = (t1 < repeat_delay) ? -1 : ((t1 - repeat_delay) / repeat_rate);
    const int count = count_t1 - count_t0;
    return count;
}

int ImGui::GetKeyPressedAmount(ImGuiKey key, float repeat_delay, float repeat_rate)
{
    ImGuiContext& g = *GImGui;
    const ImGuiKeyData* key_data = GetKeyData(key);
    const float t = key_data->DownDuration;
    return CalcTypematicRepeatAmount(t - g.io.DeltaTime, t, repeat_delay, repeat_rate);
}

// Note that Dear ImGui doesn't know the meaning/semantic of ImGuiKey from 0..511: they are legacy native keycodes.
// Consider transitioning from 'IsKeyDown(MY_ENGINE_KEY_A)' (<1.87) to IsKeyDown(ImGuiKey_A) (>= 1.87)
bool ImGui::IsKeyDown(ImGuiKey key)
{
    const ImGuiKeyData* key_data = GetKeyData(key);
    if (!key_data->Down)
        return false;
    return true;
}

bool ImGui::IsKeyPressed(ImGuiKey key, bool repeat)
{
    ImGuiContext& g = *GImGui;
    const ImGuiKeyData* key_data = GetKeyData(key);
    const float t = key_data->DownDuration;
    if (t < 0.0)
        return false;
    const bool pressed = (t == 0.0) || (repeat && t > g.io.KeyRepeatDelay && GetKeyPressedAmount(key, g.io.KeyRepeatDelay, g.io.KeyRepeatRate) > 0);
    if (!pressed)
        return false;
    return true;
}

bool ImGui::IsKeyReleased(ImGuiKey key)
{
    const ImGuiKeyData* key_data = GetKeyData(key);
    if (key_data->DownDurationPrev < 0.0 || key_data->Down)
        return false;
    return true;
}

bool ImGui::IsMouseDown(ImGuiMouseButton button)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    return g.io.mouse_down[button];
}

bool ImGui::IsMouseClicked(ImGuiMouseButton button, bool repeat)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    const float t = g.io.MouseDownDuration[button];
    if (t == 0.0)
        return true;
    if (repeat && t > g.io.KeyRepeatDelay)
        return CalcTypematicRepeatAmount(t - g.io.DeltaTime, t, g.io.KeyRepeatDelay, g.io.KeyRepeatRate) > 0;
    return false;
}

bool ImGui::IsMouseReleased(ImGuiMouseButton button)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    return g.io.MouseReleased[button];
}

bool ImGui::IsMouseDoubleClicked(ImGuiMouseButton button)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    return g.io.MouseClickedCount[button] == 2;
}

int ImGui::GetMouseClickedCount(ImGuiMouseButton button)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    return g.io.MouseClickedCount[button];
}

// Return if a mouse click/drag went past the given threshold. valid to call during the mouse_released frame.
// [Internal] This doesn't test if the button is pressed
bool ImGui::IsMouseDragPastThreshold(ImGuiMouseButton button, float lock_threshold)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    if (lock_threshold < 0.0)
        lock_threshold = g.io.MouseDragThreshold;
    return g.io.MouseDragMaxDistanceSqr[button] >= lock_threshold * lock_threshold;
}

bool ImGui::IsMouseDragging(ImGuiMouseButton button, float lock_threshold)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    if (!g.io.mouse_down[button])
        return false;
    return IsMouseDragPastThreshold(button, lock_threshold);
}

Vector2D ImGui::GetMousePos()
{
    ImGuiContext& g = *GImGui;
    return g.io.MousePos;
}

// NB: prefer to call right after BeginPopup(). At the time Selectable/MenuItem is activated, the popup is already closed!
Vector2D ImGui::GetMousePosOnOpeningCurrentPopup()
{
    ImGuiContext& g = *GImGui;
    if (g.BeginPopupStack.Size > 0)
        return g.OpenPopupStack[g.BeginPopupStack.Size - 1].OpenMousePos;
    return g.io.MousePos;
}

// We typically use Vector2D(-FLT_MAX,-FLT_MAX) to denote an invalid mouse position.
bool ImGui::is_mouse_pos_valid(const Vector2D* mouse_pos)
{
    // The assert is only to silence a false-positive in XCode Static Analysis.
    // Because GImGui is not dereferenced in every code path, the static analyzer assume that it may be NULL (which it doesn't for other functions).
    IM_ASSERT(GImGui != NULL);
    const float MOUSE_INVALID = -256000.0;
    Vector2D p = mouse_pos ? *mouse_pos : GImGui->IO.MousePos;
    return p.x >= MOUSE_INVALID && p.y >= MOUSE_INVALID;
}

// [WILL OBSOLETE] This was designed for backends, but prefer having backend maintain a mask of held mouse buttons, because upcoming input queue system will make this invalid.
bool ImGui::IsAnyMouseDown()
{
    ImGuiContext& g = *GImGui;
    for (int n = 0; n < IM_ARRAYSIZE(g.io.mouse_down); n += 1)
        if (g.io.mouse_down[n])
            return true;
    return false;
}

// Return the delta from the initial clicking position while the mouse button is clicked or was just released.
// This is locked and return 0.0 until the mouse moves past a distance threshold at least once.
// NB: This is only valid if is_mouse_pos_valid(). backends in theory should always keep mouse position valid when dragging even outside the client window.
Vector2D ImGui::GetMouseDragDelta(ImGuiMouseButton button, float lock_threshold)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    if (lock_threshold < 0.0)
        lock_threshold = g.io.MouseDragThreshold;
    if (g.io.mouse_down[button] || g.io.MouseReleased[button])
        if (g.io.MouseDragMaxDistanceSqr[button] >= lock_threshold * lock_threshold)
            if (is_mouse_pos_valid(&g.io.MousePos) && is_mouse_pos_valid(&g.io.MouseClickedPos[button]))
                return g.io.MousePos - g.io.MouseClickedPos[button];
    return DimgVec2D::new(0.0, 0.0);
}

void ImGui::ResetMouseDragDelta(ImGuiMouseButton button)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.io.mouse_down));
    // NB: We don't need to reset g.io.mouse_drag_max_distance_sqr
    g.io.MouseClickedPos[button] = g.io.MousePos;
}

ImGuiMouseCursor ImGui::GetMouseCursor()
{
    ImGuiContext& g = *GImGui;
    return g.MouseCursor;
}

void ImGui::SetMouseCursor(ImGuiMouseCursor cursor_type)
{
    ImGuiContext& g = *GImGui;
    g.MouseCursor = cursor_type;
}

void ImGui::SetNextFrameWantCaptureKeyboard(bool want_capture_keyboard)
{
    ImGuiContext& g = *GImGui;
    g.WantCaptureKeyboardNextFrame = want_capture_keyboard ? 1 : 0;
}

void ImGui::SetNextFrameWantCaptureMouse(bool want_capture_mouse)
{
    ImGuiContext& g = *GImGui;
    g.WantCaptureMouseNextFrame = want_capture_mouse ? 1 : 0;
}

#ifndef IMGUI_DISABLE_DEBUG_TOOLS
static const char* GetInputSourceName(ImGuiInputSource source)
{
    const char* input_source_names[] = { "None", "Mouse", "Keyboard", "Gamepad", "Nav", "Clipboard" };
    IM_ASSERT(IM_ARRAYSIZE(input_source_names) == ImGuiInputSource_COUNT && source >= 0 && source < ImGuiInputSource_COUNT);
    return input_source_names[source];
}


/*static void DebugPrintInputEvent(const char* prefix, const ImGuiInputEvent* e)
{
    if (e->Type == ImGuiInputEventType_MousePos)    { IMGUI_DEBUG_LOG_IO("%s: mouse_pos (%.1 %.1)\n", prefix, e->mouse_pos.PosX, e->mouse_pos.PosY); return; }
    if (e->Type == ImGuiInputEventType_MouseButton) { IMGUI_DEBUG_LOG_IO("%s: MouseButton %d %s\n", prefix, e->MouseButton.Button, e->MouseButton.down ? "down" : "Up"); return; }
    if (e->Type == ImGuiInputEventType_MouseWheel)  { IMGUI_DEBUG_LOG_IO("%s: mouse_wheel (%.1 %.1)\n", prefix, e->mouse_wheel.WheelX, e->mouse_wheel.WheelY); return; }
    if (e->Type == ImGuiInputEventType_Key)         { IMGUI_DEBUG_LOG_IO("%s: Key \"%s\" %s\n", prefix, ImGui::GetKeyName(e->Key.Key), e->Key.down ? "down" : "Up"); return; }
    if (e->Type == ImGuiInputEventType_Text)        { IMGUI_DEBUG_LOG_IO("%s: Text: %c (U+%08X)\n", prefix, e->Text.Char, e->Text.Char); return; }
    if (e->Type == ImGuiInputEventType_Focus)       { IMGUI_DEBUG_LOG_IO("%s: AppFocused %d\n", prefix, e->AppFocused.Focused); return; }
}*/

// Process input queue
// We always call this with the value of 'bool g.io.config_input_trickle_event_queue'.
// - trickle_fast_inputs = false : process all events, turn into flattened input state (e.g. successive down/up/down/up will be lost)
// - trickle_fast_inputs = true  : process as many events as possible (successive down/up/down/up will be trickled over several frames so nothing is lost) (new feature in 1.87)
void ImGui::UpdateInputEvents(bool trickle_fast_inputs)
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;

    // Only trickle chars<>key when working with InputText()
    // FIXME: InputText() could parse event trail?
    // FIXME: Could specialize chars<>keys trickling rules for control keys (those not typically associated to characters)
    const bool trickle_interleaved_keys_and_text = (trickle_fast_inputs && g.WantTextInputNextFrame == 1);

    bool mouse_moved = false, mouse_wheeled = false, key_changed = false, text_inputted = false;
    int  mouse_button_changed = 0x00;
    ImBitArray<ImGuiKey_KeysData_SIZE> key_changed_mask;

    int event_n = 0;
    for (; event_n < g.InputEventsQueue.Size; event_n += 1)
    {
        const ImGuiInputEvent* e = &g.InputEventsQueue[event_n];
        if (e->Type == ImGuiInputEventType_MousePos)
        {
            Vector2D event_pos(e->MousePos.PosX, e->MousePos.PosY);
            if (is_mouse_pos_valid(&event_pos))
                event_pos = DimgVec2D::new(ImFloorSigned(event_pos.x), ImFloorSigned(event_pos.y)); // Apply same flooring as UpdateMouseInputs()
            if (io.MousePos.x != event_pos.x || io.MousePos.y != event_pos.y)
            {
                // Trickling Rule: Stop processing queued events if we already handled a mouse button change
                if (trickle_fast_inputs && (mouse_button_changed != 0 || mouse_wheeled || key_changed || text_inputted))
                    break;
                io.MousePos = event_pos;
                mouse_moved = true;
            }
        }
        else if (e->Type == ImGuiInputEventType_MouseButton)
        {
            const ImGuiMouseButton button = e->MouseButton.Button;
            IM_ASSERT(button >= 0 && button < ImGuiMouseButton_COUNT);
            if (io.mouse_down[button] != e->MouseButton.Down)
            {
                // Trickling Rule: Stop processing queued events if we got multiple action on the same button
                if (trickle_fast_inputs && ((mouse_button_changed & (1 << button)) || mouse_wheeled))
                    break;
                io.mouse_down[button] = e->MouseButton.Down;
                mouse_button_changed |= (1 << button);
            }
        }
        else if (e->Type == ImGuiInputEventType_MouseWheel)
        {
            if (e->MouseWheel.WheelX != 0.0 || e->MouseWheel.WheelY != 0.0)
            {
                // Trickling Rule: Stop processing queued events if we got multiple action on the event
                if (trickle_fast_inputs && (mouse_moved || mouse_button_changed != 0))
                    break;
                io.MouseWheelH += e->MouseWheel.WheelX;
                io.MouseWheel += e->MouseWheel.WheelY;
                mouse_wheeled = true;
            }
        }
        else if (e->Type == ImGuiInputEventType_MouseViewport)
        {
            io.MouseHoveredViewport = e->mouse_viewport.HoveredViewportID;
        }
        else if (e->Type == ImGuiInputEventType_Key)
        {
            ImGuiKey key = e->Key.Key;
            IM_ASSERT(key != ImGuiKey_None);
            const int keydata_index = (key - ImGuiKey_KeysData_OFFSET);
            ImGuiKeyData* keydata = &io.KeysData[keydata_index];
            if (keydata->Down != e->Key.Down || keydata->AnalogValue != e->Key.AnalogValue)
            {
                // Trickling Rule: Stop processing queued events if we got multiple action on the same button
                if (trickle_fast_inputs && keydata->Down != e->Key.Down && (key_changed_mask.TestBit(keydata_index) || text_inputted || mouse_button_changed != 0))
                    break;
                keydata->Down = e->Key.Down;
                keydata->AnalogValue = e->Key.AnalogValue;
                key_changed = true;
                key_changed_mask.SetBit(keydata_index);

                if (key == ImGuiKey_ModCtrl || key == ImGuiKey_ModShift || key == ImGuiKey_ModAlt || key == ImGuiKey_ModSuper)
                {
                    if (key == ImGuiKey_ModCtrl) { io.KeyCtrl = keydata->Down; }
                    if (key == ImGuiKey_ModShift) { io.KeyShift = keydata->Down; }
                    if (key == ImGuiKey_ModAlt) { io.KeyAlt = keydata->Down; }
                    if (key == ImGuiKey_ModSuper) { io.KeySuper = keydata->Down; }
                    io.KeyMods = GetMergedModFlags();
                }

                // Allow legacy code using io.KeysDown[GetKeyIndex()] with new backends
#ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
                io.KeysDown[key] = keydata->Down;
                if (io.KeyMap[key] != -1)
                    io.KeysDown[io.KeyMap[key]] = keydata->Down;

            }
        }
        else if (e->Type == ImGuiInputEventType_Text)
        {
            // Trickling Rule: Stop processing queued events if keys/mouse have been interacted with
            if (trickle_fast_inputs && ((key_changed && trickle_interleaved_keys_and_text) || mouse_button_changed != 0 || mouse_moved || mouse_wheeled))
                break;
            unsigned int c = e->Text.Char;
            io.InputQueueCharacters.push_back(c <= IM_UNICODE_CODEPOINT_MAX ? (ImWchar)c : IM_UNICODE_CODEPOINT_INVALID);
            if (trickle_interleaved_keys_and_text)
                text_inputted = true;
        }
        else if (e->Type == ImGuiInputEventType_Focus)
        {
            // We intentionally overwrite this and process lower, in order to give a chance
            // to multi-viewports backends to queue add_focus_event(false) + add_focus_event(true) in same frame.
            io.AppFocusLost = !e->AppFocused.Focused;
        }
        else
        {
            IM_ASSERT(0 && "Unknown event!");
        }
    }

    // Record trail (for domain-specific applications wanting to access a precise trail)
    //if (event_n != 0) IMGUI_DEBUG_LOG_IO("Processed: %d / Remaining: %d\n", event_n, g.input_events_queue.size - event_n);
    for (int n = 0; n < event_n; n += 1)
        g.InputEventsTrail.push_back(g.InputEventsQueue[n]);

    // [DEBUG]
    /*if (event_n != 0)
        for (int n = 0; n < g.input_events_queue.size; n++)
            DebugPrintInputEvent(n < event_n ? "Processed" : "Remaining", &g.input_events_queue[n]);*/

    // Remaining events will be processed on the next frame
    if (event_n == g.InputEventsQueue.Size)
        g.InputEventsQueue.resize(0);
    else
        g.InputEventsQueue.erase(g.InputEventsQueue.Data, g.InputEventsQueue.Data + event_n);

    // clear buttons state when focus is lost
    // (this is useful so e.g. releasing Alt after focus loss on Alt-Tab doesn't trigger the Alt menu toggle)
    if (g.io.AppFocusLost)
    {
        g.io.ClearInputKeys();
        g.io.AppFocusLost = false;
    }
}


//-----------------------------------------------------------------------------
// [SECTION] ERROR CHECKING
//-----------------------------------------------------------------------------

// Helper function to verify ABI compatibility between caller code and compiled version of Dear ImGui.
// Verify that the type sizes are matching between the calling file's compilation unit and imgui.cpp's compilation unit
// If this triggers you have an issue:
// - Most commonly: mismatched headers and compiled code version.
// - Or: mismatched configuration #define, compilation settings, packing pragma etc.
//   The configuration settings mentioned in imconfig.h must be set for all compilation units involved with Dear ImGui,
//   which is way it is required you put them in your imconfig file (and not just before including imgui.h).
//   Otherwise it is possible that different compilation units would see different structure layout
bool ImGui::DebugCheckVersionAndDataLayout(const char* version, size_t sz_io, size_t sz_style, size_t sz_vec2, size_t sz_vec4, size_t sz_vert, size_t sz_idx)
{
    bool error = false;
    if (strcmp(version, IMGUI_VERSION) != 0) { error = true; IM_ASSERT(strcmp(version, IMGUI_VERSION) == 0 && "Mismatched version string!"); }
    if (sz_io != sizeof(ImGuiIO)) { error = true; IM_ASSERT(sz_io == sizeof(ImGuiIO) && "Mismatched struct layout!"); }
    if (sz_style != sizeof(ImGuiStyle)) { error = true; IM_ASSERT(sz_style == sizeof(ImGuiStyle) && "Mismatched struct layout!"); }
    if (sz_vec2 != sizeof(Vector2D)) { error = true; IM_ASSERT(sz_vec2 == sizeof(Vector2D) && "Mismatched struct layout!"); }
    if (sz_vec4 != sizeof(Vector4D)) { error = true; IM_ASSERT(sz_vec4 == sizeof(Vector4D) && "Mismatched struct layout!"); }
    if (sz_vert != sizeof(ImDrawVert)) { error = true; IM_ASSERT(sz_vert == sizeof(ImDrawVert) && "Mismatched struct layout!"); }
    if (sz_idx != sizeof(ImDrawIdx)) { error = true; IM_ASSERT(sz_idx == sizeof(ImDrawIdx) && "Mismatched struct layout!"); }
    return !error;
}

static void ImGui::ErrorCheckNewFrameSanityChecks()
{
    ImGuiContext& g = *GImGui;

    // Check user IM_ASSERT macro
    // (IF YOU GET A WARNING OR COMPILE ERROR HERE: it means your assert macro is incorrectly defined!
    //  If your macro uses multiple statements, it NEEDS to be surrounded by a 'do { ... } while (0)' block.
    //  This is a common C/C++ idiom to allow multiple statements macros to be used in control flow blocks.)
    // #define IM_ASSERT(EXPR)   if (SomeCode(EXPR)) SomeMoreCode();                    // Wrong!
    // #define IM_ASSERT(EXPR)   do { if (SomeCode(EXPR)) SomeMoreCode(); } while (0)   // Correct!
    if (true) IM_ASSERT(1); else IM_ASSERT(0);

    // Check user data
    // (We pass an error message in the assert expression to make it visible to programmers who are not using a debugger, as most assert handlers display their argument)
    IM_ASSERT(g.Initialized);
    IM_ASSERT((g.io.DeltaTime > 0.0 || g.FrameCount == 0)              && "Need a positive delta_time!");
    IM_ASSERT((g.FrameCount == 0 || g.FrameCountEnded == g.FrameCount)  && "Forgot to call Render() or EndFrame() at the end of the previous frame?");
    IM_ASSERT(g.io.DisplaySize.x >= 0.0 && g.io.DisplaySize.y >= 0.0  && "Invalid display_size value!");
    IM_ASSERT(g.io.Fonts->IsBuilt()                                     && "font Atlas not built! Make sure you called ImGui_ImplXXXX_NewFrame() function for renderer backend, which should call io.fonts->GetTexDataAsRGBA32() / GetTexDataAsAlpha8()");
    IM_ASSERT(g.Style.CurveTessellationTol > 0.0                       && "Invalid style setting!");
    IM_ASSERT(g.Style.CircleTessellationMaxError > 0.0                 && "Invalid style setting!");
    IM_ASSERT(g.Style.Alpha >= 0.0 && g.Style.Alpha <= 1.0            && "Invalid style setting!"); // Allows us to avoid a few clamps in color computations
    IM_ASSERT(g.Style.WindowMinSize.x >= 1.0 && g.Style.WindowMinSize.y >= 1.0 && "Invalid style setting.");
    IM_ASSERT(g.Style.WindowMenuButtonPosition == ImGuiDir_None || g.Style.WindowMenuButtonPosition == ImGuiDir_Left || g.Style.WindowMenuButtonPosition == ImGuiDir_Right);
    IM_ASSERT(g.Style.ColorButtonPosition == ImGuiDir_Left || g.Style.ColorButtonPosition == ImGuiDir_Right);
#ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_COUNT; n += 1)
        IM_ASSERT(g.io.KeyMap[n] >= -1 && g.io.KeyMap[n] < ImGuiKey_LegacyNativeKey_END && "io.KeyMap[] contains an out of bound value (need to be 0..511, or -1 for unmapped key)");

    // Check: required key mapping (we intentionally do NOT check all keys to not pressure user into setting up everything, but Space is required and was only added in 1.60 WIP)
    if ((g.io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) && g.io.BackendUsingLegacyKeyArrays == 1)
        IM_ASSERT(g.io.KeyMap[ImGuiKey_Space] != -1 && "ImGuiKey_Space is not mapped, required for keyboard navigation.");


    // Check: the io.config_windows_resize_from_edges option requires backend to honor mouse cursor changes and set the ImGuiBackendFlags_HasMouseCursors flag accordingly.
    if (g.io.ConfigWindowsResizeFromEdges && !(g.io.BackendFlags & ImGuiBackendFlags_HasMouseCursors))
        g.io.ConfigWindowsResizeFromEdges = false;

    // Perform simple check: error if Docking or viewport are enabled _exactly_ on frame 1 (instead of frame 0 or later), which is a common error leading to loss of .ini data.
    if (g.FrameCount == 1 && (g.io.ConfigFlags & ImGuiConfigFlags_DockingEnable) && (g.ConfigFlagsLastFrame & ImGuiConfigFlags_DockingEnable) == 0)
        IM_ASSERT(0 && "Please set DockingEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");
    if (g.FrameCount == 1 && (g.io.ConfigFlags & ConfigFlags::ViewportsEnable) && (g.ConfigFlagsLastFrame & ConfigFlags::ViewportsEnable) == 0)
        IM_ASSERT(0 && "Please set ViewportsEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");

    // Perform simple checks: multi-viewport and platform windows support
    if (g.io.ConfigFlags & ConfigFlags::ViewportsEnable)
    {
        if ((g.io.BackendFlags & ImGuiBackendFlags_PlatformHasViewports) && (g.io.BackendFlags & ImGuiBackendFlags_RendererHasViewports))
        {
            IM_ASSERT((g.FrameCount == 0 || g.FrameCount == g.FrameCountPlatformEnded) && "Forgot to call UpdatePlatformWindows() in main loop after EndFrame()? Check examples/ applications for reference.");
            IM_ASSERT(g.PlatformIO.Platform_CreateWindow  != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.PlatformIO.Platform_DestroyWindow != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.PlatformIO.Platform_GetWindowPos  != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.PlatformIO.Platform_SetWindowPos  != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.PlatformIO.Platform_GetWindowSize != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.PlatformIO.Platform_SetWindowSize != NULL && "Platform init didn't install handlers?");
            IM_ASSERT(g.PlatformIO.Monitors.Size > 0 && "Platform init didn't setup Monitors list?");
            IM_ASSERT((g.Viewports[0]->PlatformUserData != NULL || g.Viewports[0]->PlatformHandle != NULL) && "Platform init didn't setup main viewport.");
            if (g.io.ConfigDockingTransparentPayload && (g.io.ConfigFlags & ImGuiConfigFlags_DockingEnable))
                IM_ASSERT(g.PlatformIO.Platform_SetWindowAlpha != NULL && "Platform_SetWindowAlpha handler is required to use io.ConfigDockingTransparent!");
        }
        else
        {
            // Disable feature, our backends do not support it
            g.io.ConfigFlags &= ~ConfigFlags::ViewportsEnable;
        }

        // Perform simple checks on platform monitor data + compute a total bounding box for quick early outs
        for (int monitor_n = 0; monitor_n < g.PlatformIO.Monitors.Size; monitor_n += 1)
        {
            ImGuiPlatformMonitor& mon = g.PlatformIO.Monitors[monitor_n];
            IM_UNUSED(mon);
            IM_ASSERT(mon.MainSize.x > 0.0 && mon.MainSize.y > 0.0 && "Monitor main bounds not setup properly.");
            IM_ASSERT(ImRect(mon.MainPos, mon.MainPos + mon.MainSize).Contains(ImRect(mon.WorkPos, mon.WorkPos + mon.WorkSize)) && "Monitor work bounds not setup properly. If you don't have work area information, just copy MainPos/MainSize into them.");
            IM_ASSERT(mon.DpiScale != 0.0);
        }
    }
}

static void ImGui::ErrorCheckEndFrameSanityChecks()
{
    ImGuiContext& g = *GImGui;

    // Verify that io.KeyXXX fields haven't been tampered with. Key mods should not be modified between NewFrame() and EndFrame()
    // One possible reason leading to this assert is that your backends update inputs _AFTER_ NewFrame().
    // It is known that when some modal native windows called mid-frame takes focus away, some backends such as GLFW will
    // send key release events mid-frame. This would normally trigger this assertion and lead to sheared inputs.
    // We silently accommodate for this case by ignoring/ the case where all io.KeyXXX modifiers were released (aka key_mod_flags == 0),
    // while still correctly asserting on mid-frame key press events.
    const ImGuiModFlags key_mods = GetMergedModFlags();
    IM_ASSERT((key_mods == 0 || g.io.KeyMods == key_mods) && "Mismatching io.key_ctrl/io.key_shift/io.key_alt/io.key_super vs io.key_mods");
    IM_UNUSED(key_mods);

    // [EXPERIMENTAL] Recover from errors: You may call this yourself before EndFrame().
    //ErrorCheckEndFrameRecover();

    // Report when there is a mismatch of Begin/BeginChild vs End/EndChild calls. Important: Remember that the Begin/BeginChild API requires you
    // to always call End/EndChild even if Begin/BeginChild returns false! (this is unfortunately inconsistent with most other Begin* API).
    if (g.CurrentWindowStack.Size != 1)
    {
        if (g.CurrentWindowStack.Size > 1)
        {
            IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you forget to call End/EndChild?");
            while (g.CurrentWindowStack.Size > 1)
                End();
        }
        else
        {
            IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you call End/EndChild too much?");
        }
    }

    IM_ASSERT_USER_ERROR(g.GroupStack.Size == 0, "Missing EndGroup call!");
}

// Experimental recovery from incorrect usage of BeginXXX/EndXXX/PushXXX/PopXXX calls.
// Must be called during or before EndFrame().
// This is generally flawed as we are not necessarily End/Popping things in the right order.
// FIXME: Can't recover from inside BeginTabItem/EndTabItem yet.
// FIXME: Can't recover from interleaved BeginTabBar/Begin
void    ImGui::ErrorCheckEndFrameRecover(ImGuiErrorLogCallback log_callback, void* user_data)
{
    // PVS-Studio V1044 is "Loop break conditions do not depend on the number of iterations"
    ImGuiContext& g = *GImGui;
    while (g.CurrentWindowStack.Size > 0) //-V1044
    {
        ErrorCheckEndWindowRecover(log_callback, user_data);
        ImGuiWindow* window = g.CurrentWindow;
        if (g.CurrentWindowStack.Size == 1)
        {
            IM_ASSERT(window.IsFallbackWindow);
            break;
        }
        if (window.Flags & ImGuiWindowFlags_ChildWindow)
        {
            if (log_callback) log_callback(user_data, "Recovered from missing EndChild() for '%s'", window.Name);
            EndChild();
        }
        else
        {
            if (log_callback) log_callback(user_data, "Recovered from missing End() for '%s'", window.Name);
            End();
        }
    }
}

// Must be called before End()/EndChild()
void    ImGui::ErrorCheckEndWindowRecover(ImGuiErrorLogCallback log_callback, void* user_data)
{
    ImGuiContext& g = *GImGui;
    while (g.CurrentTable && (g.CurrentTable->OuterWindow == g.CurrentWindow || g.CurrentTable->InnerWindow == g.CurrentWindow))
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndTable() in '%s'", g.CurrentTable->OuterWindow->Name);
        EndTable();
    }

    ImGuiWindow* window = g.CurrentWindow;
    ImGuiStackSizes* stack_sizes = &g.CurrentWindowStack.back().StackSizesOnBegin;
    IM_ASSERT(window != NULL);
    while (g.CurrentTabBar != NULL) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndTabBar() in '%s'", window.Name);
        EndTabBar();
    }
    while (window.DC.TreeDepth > 0)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing TreePop() in '%s'", window.Name);
        TreePop();
    }
    while (g.GroupStack.Size > stack_sizes->SizeOfGroupStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndGroup() in '%s'", window.Name);
        EndGroup();
    }
    while (window.IDStack.Size > 1)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopID() in '%s'", window.Name);
        PopID();
    }
    while (g.DisabledStackSize > stack_sizes->SizeOfDisabledStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndDisabled() in '%s'", window.Name);
        EndDisabled();
    }
    while (g.ColorStack.Size > stack_sizes->SizeOfColorStack)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopStyleColor() in '%s' for ImGuiCol_%s", window.Name, GetStyleColorName(g.ColorStack.back().Col));
        PopStyleColor();
    }
    while (g.ItemFlagsStack.Size > stack_sizes->SizeOfItemFlagsStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopItemFlag() in '%s'", window.Name);
        PopItemFlag();
    }
    while (g.StyleVarStack.Size > stack_sizes->SizeOfStyleVarStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopStyleVar() in '%s'", window.Name);
        PopStyleVar();
    }
    while (g.FocusScopeStack.Size > stack_sizes->SizeOfFocusScopeStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopFocusScope() in '%s'", window.Name);
        PopFocusScope();
    }
}

// Save current stack sizes for later compare
void ImGuiStackSizes::SetToCurrentState()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    SizeOfIDStack = (short)window.IDStack.Size;
    SizeOfColorStack = (short)g.ColorStack.Size;
    SizeOfStyleVarStack = (short)g.StyleVarStack.Size;
    SizeOfFontStack = (short)g.FontStack.Size;
    SizeOfFocusScopeStack = (short)g.FocusScopeStack.Size;
    SizeOfGroupStack = (short)g.GroupStack.Size;
    SizeOfItemFlagsStack = (short)g.ItemFlagsStack.Size;
    SizeOfBeginPopupStack = (short)g.BeginPopupStack.Size;
    SizeOfDisabledStack = (short)g.DisabledStackSize;
}

// Compare to detect usage errors
void ImGuiStackSizes::CompareWithCurrentState()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    IM_UNUSED(window);

    // Window stacks
    // NOT checking: dc.ItemWidth, dc.TextWrapPos (per window) to allow user to conveniently push once and not pop (they are cleared on Begin)
    IM_ASSERT(SizeOfIDStack         == window.IDStack.Size     && "PushID/PopID or TreeNode/TreePop Mismatch!");

    // Global stacks
    // For color, style and font stacks there is an incentive to use Push/Begin/Pop/.../End patterns, so we relax our checks a little to allow them.
    IM_ASSERT(SizeOfGroupStack      == g.GroupStack.Size        && "BeginGroup/EndGroup Mismatch!");
    IM_ASSERT(SizeOfBeginPopupStack == g.BeginPopupStack.Size   && "BeginPopup/EndPopup or BeginMenu/EndMenu Mismatch!");
    IM_ASSERT(SizeOfDisabledStack   == g.DisabledStackSize      && "BeginDisabled/EndDisabled Mismatch!");
    IM_ASSERT(SizeOfItemFlagsStack  >= g.ItemFlagsStack.Size    && "PushItemFlag/PopItemFlag Mismatch!");
    IM_ASSERT(SizeOfColorStack      >= g.ColorStack.Size        && "PushStyleColor/PopStyleColor Mismatch!");
    IM_ASSERT(SizeOfStyleVarStack   >= g.StyleVarStack.Size     && "PushStyleVar/PopStyleVar Mismatch!");
    IM_ASSERT(SizeOfFontStack       >= g.FontStack.Size         && "PushFont/PopFont Mismatch!");
    IM_ASSERT(SizeOfFocusScopeStack == g.FocusScopeStack.Size   && "PushFocusScope/PopFocusScope Mismatch!");
}


//-----------------------------------------------------------------------------
// [SECTION] LAYOUT
//-----------------------------------------------------------------------------
// - ItemSize()
// - ItemAdd()
// - SameLine()
// - GetCursorScreenPos()
// - SetCursorScreenPos()
// - GetCursorPos(), GetCursorPosX(), GetCursorPosY()
// - SetCursorPos(), SetCursorPosX(), SetCursorPosY()
// - GetCursorStartPos()
// - Indent()
// - Unindent()
// - SetNextItemWidth()
// - PushItemWidth()
// - PushMultiItemsWidths()
// - PopItemWidth()
// - CalcItemWidth()
// - CalcItemSize()
// - GetTextLineHeight()
// - GetTextLineHeightWithSpacing()
// - GetFrameHeight()
// - GetFrameHeightWithSpacing()
// - GetContentRegionMax()
// - GetContentRegionMaxAbs() [Internal]
// - GetContentRegionAvail(),
// - GetWindowContentRegionMin(), GetWindowContentRegionMax()
// - BeginGroup()
// - EndGroup()
// Also see in imgui_widgets: tab bars, and in imgui_tables: tables, columns.
//-----------------------------------------------------------------------------

// Advance cursor given item size for layout.
// Register minimum needed size so it can extend the bounding box used for auto-fit calculation.
// See comments in ItemAdd() about how/why the size provided to ItemSize() vs ItemAdd() may often different.
void ImGui::ItemSize(const Vector2D& size, float text_baseline_y)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    if (window.SkipItems)
        return;

    // We increase the height in this function to accommodate for baseline offset.
    // In theory we should be offsetting the starting position (window->dc.CursorPos), that will be the topic of a larger refactor,
    // but since ItemSize() is not yet an API that moves the cursor (to handle e.g. wrapping) enlarging the height has the same effect.
    const float offset_to_match_baseline_y = (text_baseline_y >= 0) ? ImMax(0.0, window.DC.CurrLineTextBaseOffset - text_baseline_y) : 0.0;

    const float line_y1 = window.DC.IsSameLine ? window.DC.CursorPosPrevLine.y : window.DC.CursorPos.y;
    const float line_height = ImMax(window.DC.CurrLineSize.y, /*ImMax(*/window.DC.CursorPos.y - line_y1/*, 0.0)*/ + size.y + offset_to_match_baseline_y);

    // Always align ourselves on pixel boundaries
    //if (g.io.key_alt) window->draw_list->add_rect(window->dc.CursorPos, window->dc.CursorPos + Vector2D(size.x, line_height), IM_COL32(255,0,0,200)); // [DEBUG]
    window.DC.CursorPosPrevLine.x = window.DC.CursorPos.x + size.x;
    window.DC.CursorPosPrevLine.y = line_y1;
    window.DC.CursorPos.x = IM_FLOOR(window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x);    // Next line
    window.DC.CursorPos.y = IM_FLOOR(line_y1 + line_height + g.Style.ItemSpacing.y);                    // Next line
    window.DC.CursorMaxPos.x = ImMax(window.DC.CursorMaxPos.x, window.DC.CursorPosPrevLine.x);
    window.DC.CursorMaxPos.y = ImMax(window.DC.CursorMaxPos.y, window.DC.CursorPos.y - g.Style.ItemSpacing.y);
    //if (g.io.key_alt) window->draw_list->add_circle(window->dc.CursorMaxPos, 3.0, IM_COL32(255,0,0,255), 4); // [DEBUG]

    window.DC.PrevLineSize.y = line_height;
    window.DC.CurrLineSize.y = 0.0;
    window.DC.PrevLineTextBaseOffset = ImMax(window.DC.CurrLineTextBaseOffset, text_baseline_y);
    window.DC.CurrLineTextBaseOffset = 0.0;
    window.DC.IsSameLine = false;

    // Horizontal layout mode
    if (window.DC.LayoutType == ImGuiLayoutType_Horizontal)
        SameLine();
}

// Declare item bounding box for clipping and interaction.
// Note that the size can be different than the one provided to ItemSize(). Typically, widgets that spread over available surface
// declare their minimum size requirement to ItemSize() and provide a larger region to ItemAdd() which is used drawing/interaction.
bool ImGui::ItemAdd(const ImRect& bb, ImGuiID id, const ImRect* nav_bb_arg, ImGuiItemFlags extra_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;

    // Set item data
    // (display_rect is left untouched, made valid when ImGuiItemStatusFlags_HasDisplayRect is set)
    g.last_item_data.ID = id;
    g.last_item_data.Rect = bb;
    g.last_item_data.NavRect = nav_bb_arg ? *nav_bb_arg : bb;
    g.last_item_data.InFlags = g.CurrentItemFlags | extra_flags;
    g.last_item_data.StatusFlags = ImGuiItemStatusFlags_None;

    // Directional navigation processing
    if (id != 0)
    {
        keep_alive_id(id);

        // Runs prior to clipping early-out
        //  (a) So that nav_init_request can be honored, for newly opened windows to select a default widget
        //  (b) So that we can scroll up/down past clipped items. This adds a small O(N) cost to regular navigation requests
        //      unfortunately, but it is still limited to one window. It may not scale very well for windows with ten of
        //      thousands of item, but at least NavMoveRequest is only set on user interaction, aka maximum once a frame.
        //      We could early out with "if (is_clipped && !g.nav_init_request) return false;" but when we wouldn't be able
        //      to reach unclipped widgets. This would work if user had explicit scrolling control (e.g. mapped on a stick).
        // We intentionally don't check if g.nav_window != NULL because g.nav_any_request should only be set when it is non null.
        // If we crash on a NULL g.nav_window we need to fix the bug elsewhere.
        window.DC.NavLayersActiveMaskNext |= (1 << window.DC.NavLayerCurrent);
        if (g.NavId == id || g.NavAnyRequest)
            if (g.nav_window->RootWindowForNav == window.RootWindowForNav)
                if (window == g.nav_window || ((window.Flags | g.nav_window.flags) & ImGuiWindowFlags_NavFlattened))
                    NavProcessItem();

        // [DEBUG] People keep stumbling on this problem and using "" as identifier in the root of a window instead of "##something".
        // Empty identifier are valid and useful in a small amount of cases, but 99.9% of the time you want to use "##something".
        // READ THE FAQ: https://dearimgui.org/faq
        IM_ASSERT(id != window.ID && "Cannot have an empty id at the root of a window. If you need an empty label, use ## and read the FAQ about how the id Stack works!");

        // [DEBUG] Item Picker tool, when enabling the "extended" version we perform the check in ItemAdd()
#ifdef IMGUI_DEBUG_TOOL_ITEM_PICKER_EX
        if (id == g.DebugItemPickerBreakId)
        {
            IM_DEBUG_BREAK();
            g.DebugItemPickerBreakId = 0;
        }

    }
    g.NextItemData.Flags = ImGuiNextItemDataFlags_None;

#ifdef IMGUI_ENABLE_TEST_ENGINE
    if (id != 0)
        IMGUI_TEST_ENGINE_ITEM_ADD(nav_bb_arg ? *nav_bb_arg : bb, id);


    // Clipping test
    const bool is_clipped = IsClippedEx(bb, id);
    if (is_clipped)
        return false;
    //if (g.io.key_alt) window->draw_list->add_rect(bb.min, bb.max, IM_COL32(255,255,0,120)); // [DEBUG]

    // We need to calculate this now to take account of the current clipping rectangle (as items like Selectable may change them)
    if (IsMouseHoveringRect(bb.Min, bb.Max))
        g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_HoveredRect;
    return true;
}

// Gets back to previous line and continue with horizontal layout
//      offset_from_start_x == 0 : follow right after previous item
//      offset_from_start_x != 0 : align to specified x position (relative to window/group left)
//      spacing_w < 0            : use default spacing if pos_x == 0, no spacing if pos_x != 0
//      spacing_w >= 0           : enforce spacing amount
void ImGui::SameLine(float offset_from_start_x, float spacing_w)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    if (window.SkipItems)
        return;

    if (offset_from_start_x != 0.0)
    {
        if (spacing_w < 0.0)
            spacing_w = 0.0;
        window.DC.CursorPos.x = window.Pos.x - window.Scroll.x + offset_from_start_x + spacing_w + window.DC.GroupOffset.x + window.DC.ColumnsOffset.x;
        window.DC.CursorPos.y = window.DC.CursorPosPrevLine.y;
    }
    else
    {
        if (spacing_w < 0.0)
            spacing_w = g.Style.ItemSpacing.x;
        window.DC.CursorPos.x = window.DC.CursorPosPrevLine.x + spacing_w;
        window.DC.CursorPos.y = window.DC.CursorPosPrevLine.y;
    }
    window.DC.CurrLineSize = window.DC.PrevLineSize;
    window.DC.CurrLineTextBaseOffset = window.DC.PrevLineTextBaseOffset;
    window.DC.IsSameLine = true;
}

Vector2D ImGui::GetCursorScreenPos()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.DC.CursorPos;
}

void ImGui::SetCursorScreenPos(const Vector2D& pos)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.CursorPos = pos;
    window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, window.DC.CursorPos);
}

// User generally sees positions in window coordinates. Internally we store CursorPos in absolute screen coordinates because it is more convenient.
// Conversion happens as we pass the value to user, but it makes our naming convention confusing because GetCursorPos() == (dc.CursorPos - window.pos). May want to rename 'dc.CursorPos'.
Vector2D ImGui::GetCursorPos()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.DC.CursorPos - window.Pos + window.Scroll;
}

float ImGui::GetCursorPosX()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.DC.CursorPos.x - window.Pos.x + window.Scroll.x;
}

float ImGui::GetCursorPosY()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.DC.CursorPos.y - window.Pos.y + window.Scroll.y;
}

void ImGui::SetCursorPos(const Vector2D& local_pos)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.CursorPos = window.Pos - window.Scroll + local_pos;
    window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, window.DC.CursorPos);
}

void ImGui::SetCursorPosX(float x)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.CursorPos.x = window.Pos.x - window.Scroll.x + x;
    window.DC.CursorMaxPos.x = ImMax(window.DC.CursorMaxPos.x, window.DC.CursorPos.x);
}

void ImGui::SetCursorPosY(float y)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.CursorPos.y = window.Pos.y - window.Scroll.y + y;
    window.DC.CursorMaxPos.y = ImMax(window.DC.CursorMaxPos.y, window.DC.CursorPos.y);
}

Vector2D ImGui::GetCursorStartPos()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.DC.CursorStartPos - window.Pos;
}

void ImGui::Indent(float indent_w)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.Indent.x += (indent_w != 0.0) ? indent_w : g.Style.IndentSpacing;
    window.DC.CursorPos.x = window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x;
}

void ImGui::Unindent(float indent_w)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.Indent.x -= (indent_w != 0.0) ? indent_w : g.Style.IndentSpacing;
    window.DC.CursorPos.x = window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x;
}

// Affect large frame+labels widgets only.
//void ImGui::SetNextItemWidth(float item_width)
pub fn SetNextItemWidth(item_width: f32)
{
    // ImGuiContext& g = *GImGui;
    GImGui.NextItemData.Flags |= ImGuiNextItemDataFlags::ImGuiNextItemDataFlags_HasWidth;
    GImGui.NextItemData.Width = item_width;
}

// FIXME: Remove the == 0.0 behavior?
void ImGui::PushItemWidth(float item_width)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    window.DC.ItemWidthStack.push_back(window.DC.ItemWidth); // Backup current width
    window.DC.ItemWidth = (item_width == 0.0 ? window.ItemWidthDefault : item_width);
    g.NextItemData.Flags &= ~ImGuiNextItemDataFlags_HasWidth;
}

void ImGui::PushMultiItemsWidths(int components, float w_full)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    const ImGuiStyle& style = g.Style;
    const float w_item_one  = ImMax(1.0, IM_FLOOR((w_full - (style.ItemInnerSpacing.x) * (components - 1)) / (float)components));
    const float w_item_last = ImMax(1.0, IM_FLOOR(w_full - (w_item_one + style.ItemInnerSpacing.x) * (components - 1)));
    window.DC.ItemWidthStack.push_back(window.DC.ItemWidth); // Backup current width
    window.DC.ItemWidthStack.push_back(w_item_last);
    for (int i = 0; i < components - 2; i += 1)
        window.DC.ItemWidthStack.push_back(w_item_one);
    window.DC.ItemWidth = (components == 1) ? w_item_last : w_item_one;
    g.NextItemData.Flags &= ~ImGuiNextItemDataFlags_HasWidth;
}

void ImGui::PopItemWidth()
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.ItemWidth = window.DC.ItemWidthStack.back();
    window.DC.ItemWidthStack.pop_back();
}

// Calculate default item width given value passed to PushItemWidth() or SetNextItemWidth().
// The SetNextItemWidth() data is generally cleared/consumed by ItemAdd() or next_item_data.ClearFlags()
float ImGui::CalcItemWidth()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    float w;
    if (g.NextItemData.Flags & ImGuiNextItemDataFlags_HasWidth)
        w = g.NextItemData.Width;
    else
        w = window.DC.ItemWidth;
    if (w < 0.0)
    {
        float region_max_x = GetContentRegionMaxAbs().x;
        w = ImMax(1.0, region_max_x - window.DC.CursorPos.x + w);
    }
    w = IM_FLOOR(w);
    return w;
}

// [Internal] Calculate full item size given user provided 'size' parameter and default width/height. Default width is often == CalcItemWidth().
// Those two functions CalcItemWidth vs CalcItemSize are awkwardly named because they are not fully symmetrical.
// Note that only CalcItemWidth() is publicly exposed.
// The 4.0 here may be changed to match CalcItemWidth() and/or BeginChild() (right now we have a mismatch which is harmless but undesirable)
Vector2D ImGui::CalcItemSize(Vector2D size, float default_w, float default_h)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;

    Vector2D region_max;
    if (size.x < 0.0 || size.y < 0.0)
        region_max = GetContentRegionMaxAbs();

    if (size.x == 0.0)
        size.x = default_w;
    else if (size.x < 0.0)
        size.x = ImMax(4.0, region_max.x - window.DC.CursorPos.x + size.x);

    if (size.y == 0.0)
        size.y = default_h;
    else if (size.y < 0.0)
        size.y = ImMax(4.0, region_max.y - window.DC.CursorPos.y + size.y);

    return size;
}

float ImGui::GetTextLineHeight()
{
    ImGuiContext& g = *GImGui;
    return g.FontSize;
}

float ImGui::GetTextLineHeightWithSpacing()
{
    ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.ItemSpacing.y;
}

float ImGui::GetFrameHeight()
{
    ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.FramePadding.y * 2.0;
}

float ImGui::GetFrameHeightWithSpacing()
{
    ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.FramePadding.y * 2.0 + g.Style.ItemSpacing.y;
}

// FIXME: All the Contents Region function are messy or misleading. WE WILL AIM TO OBSOLETE ALL OF THEM WITH A NEW "WORK RECT" API. Thanks for your patience!

// FIXME: This is in window space (not screen space!).
Vector2D ImGui::GetContentRegionMax()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    Vector2D mx = window.ContentRegionRect.Max - window.Pos;
    if (window.DC.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.Max.x - window.Pos.x;
    return mx;
}

// [Internal] Absolute coordinate. Saner. This is not exposed until we finishing refactoring work rect features.
Vector2D ImGui::GetContentRegionMaxAbs()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    Vector2D mx = window.ContentRegionRect.Max;
    if (window.DC.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.Max.x;
    return mx;
}

Vector2D ImGui::GetContentRegionAvail()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return GetContentRegionMaxAbs() - window.DC.CursorPos;
}

// In window space (not screen space!)
Vector2D ImGui::GetWindowContentRegionMin()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.ContentRegionRect.Min - window.Pos;
}

Vector2D ImGui::GetWindowContentRegionMax()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.ContentRegionRect.Max - window.Pos;
}

// Lock horizontal starting position + capture group bounding box into one "item" (so you can use IsItemHovered() or layout primitives such as SameLine() on whole group, etc.)
// Groups are currently a mishmash of functionalities which should perhaps be clarified and separated.
// FIXME-OPT: Could we safely early out on ->skip_items?
void ImGui::BeginGroup()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;

    g.GroupStack.resize(g.GroupStack.Size + 1);
    ImGuiGroupData& group_data = g.GroupStack.back();
    group_data.WindowID = window.ID;
    group_data.BackupCursorPos = window.DC.CursorPos;
    group_data.BackupCursorMaxPos = window.DC.CursorMaxPos;
    group_data.BackupIndent = window.DC.Indent;
    group_data.BackupGroupOffset = window.DC.GroupOffset;
    group_data.BackupCurrLineSize = window.DC.CurrLineSize;
    group_data.BackupCurrLineTextBaseOffset = window.DC.CurrLineTextBaseOffset;
    group_data.BackupActiveIdIsAlive = g.ActiveIdIsAlive;
    group_data.BackupHoveredIdIsAlive = g.hovered_id != 0;
    group_data.BackupActiveIdPreviousFrameIsAlive = g.ActiveIdPreviousFrameIsAlive;
    group_data.EmitItem = true;

    window.DC.GroupOffset.x = window.DC.CursorPos.x - window.Pos.x - window.DC.ColumnsOffset.x;
    window.DC.Indent = window.DC.GroupOffset;
    window.DC.CursorMaxPos = window.DC.CursorPos;
    window.DC.CurrLineSize = DimgVec2D::new(0.0, 0.0);
    if (g.LogEnabled)
        g.LogLinePosY = -FLT_MAX; // To enforce a carriage return
}

void ImGui::EndGroup()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    IM_ASSERT(g.GroupStack.Size > 0); // Mismatched BeginGroup()/EndGroup() calls

    ImGuiGroupData& group_data = g.GroupStack.back();
    IM_ASSERT(group_data.WindowID == window.ID); // EndGroup() in wrong window?

    ImRect group_bb(group_data.BackupCursorPos, ImMax(window.DC.CursorMaxPos, group_data.BackupCursorPos));

    window.DC.CursorPos = group_data.BackupCursorPos;
    window.DC.CursorMaxPos = ImMax(group_data.BackupCursorMaxPos, window.DC.CursorMaxPos);
    window.DC.Indent = group_data.BackupIndent;
    window.DC.GroupOffset = group_data.BackupGroupOffset;
    window.DC.CurrLineSize = group_data.BackupCurrLineSize;
    window.DC.CurrLineTextBaseOffset = group_data.BackupCurrLineTextBaseOffset;
    if (g.LogEnabled)
        g.LogLinePosY = -FLT_MAX; // To enforce a carriage return

    if (!group_data.EmitItem)
    {
        g.GroupStack.pop_back();
        return;
    }

    window.DC.CurrLineTextBaseOffset = ImMax(window.DC.PrevLineTextBaseOffset, group_data.BackupCurrLineTextBaseOffset);      // FIXME: Incorrect, we should grab the base offset from the *first line* of the group but it is hard to obtain now.
    ItemSize(group_bb.GetSize());
    ItemAdd(group_bb, 0, NULL, ImGuiItemFlags_NoTabStop);

    // If the current active_id was declared within the boundary of our group, we copy it to LastItemId so IsItemActive(), IsItemDeactivated() etc. will be functional on the entire group.
    // It would be be neater if we replaced window.dc.LastItemId by e.g. 'bool LastItemIsActive', but would put a little more burden on individual widgets.
    // Also if you grep for LastItemId you'll notice it is only used in that context.
    // (The two tests not the same because active_id_is_alive is an id itself, in order to be able to handle active_id being overwritten during the frame.)
    const bool group_contains_curr_active_id = (group_data.BackupActiveIdIsAlive != g.active_id) && (g.ActiveIdIsAlive == g.active_id) && g.active_id;
    const bool group_contains_prev_active_id = (group_data.BackupActiveIdPreviousFrameIsAlive == false) && (g.ActiveIdPreviousFrameIsAlive == true);
    if (group_contains_curr_active_id)
        g.last_item_data.ID = g.active_id;
    else if (group_contains_prev_active_id)
        g.last_item_data.ID = g.ActiveIdPreviousFrame;
    g.last_item_data.Rect = group_bb;

    // Forward Hovered flag
    const bool group_contains_curr_hovered_id = (group_data.BackupHoveredIdIsAlive == false) && g.hovered_id != 0;
    if (group_contains_curr_hovered_id)
        g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_HoveredWindow;

    // Forward Edited flag
    if (group_contains_curr_active_id && g.ActiveIdHasBeenEditedThisFrame)
        g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_Edited;

    // Forward Deactivated flag
    g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_HasDeactivated;
    if (group_contains_prev_active_id && g.active_id != g.ActiveIdPreviousFrame)
        g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_Deactivated;

    g.GroupStack.pop_back();
    //window->draw_list->add_rect(group_bb.min, group_bb.max, IM_COL32(255,0,255,255));   // [Debug]
}


//-----------------------------------------------------------------------------
// [SECTION] SCROLLING
//-----------------------------------------------------------------------------

// Helper to snap on edges when aiming at an item very close to the edge,
// So the difference between window_padding and ItemSpacing will be in the visible area after scrolling.
// When we refactor the scrolling API this may be configurable with a flag?
// Note that the effect for this won't be visible on x axis with default style settings as window_padding.x == ItemSpacing.x by default.
static float CalcScrollEdgeSnap(float target, float snap_min, float snap_max, float snap_threshold, float center_ratio)
{
    if (target <= snap_min + snap_threshold)
        return ImLerp(snap_min, target, center_ratio);
    if (target >= snap_max - snap_threshold)
        return ImLerp(target, snap_max, center_ratio);
    return target;
}

static Vector2D CalcNextScrollFromScrollTargetAndClamp(ImGuiWindow* window)
{
    Vector2D scroll = window.Scroll;
    if (window.ScrollTarget.x < FLT_MAX)
    {
        float decoration_total_width = window.ScrollbarSizes.x;
        float center_x_ratio = window.ScrollTargetCenterRatio.x;
        float scroll_target_x = window.ScrollTarget.x;
        if (window.ScrollTargetEdgeSnapDist.x > 0.0)
        {
            float snap_x_min = 0.0;
            float snap_x_max = window.ScrollMax.x + window.SizeFull.x - decoration_total_width;
            scroll_target_x = CalcScrollEdgeSnap(scroll_target_x, snap_x_min, snap_x_max, window.ScrollTargetEdgeSnapDist.x, center_x_ratio);
        }
        scroll.x = scroll_target_x - center_x_ratio * (window.SizeFull.x - decoration_total_width);
    }
    if (window.ScrollTarget.y < FLT_MAX)
    {
        float decoration_total_height = window.TitleBarHeight() + window.MenuBarHeight() + window.ScrollbarSizes.y;
        float center_y_ratio = window.ScrollTargetCenterRatio.y;
        float scroll_target_y = window.ScrollTarget.y;
        if (window.ScrollTargetEdgeSnapDist.y > 0.0)
        {
            float snap_y_min = 0.0;
            float snap_y_max = window.ScrollMax.y + window.SizeFull.y - decoration_total_height;
            scroll_target_y = CalcScrollEdgeSnap(scroll_target_y, snap_y_min, snap_y_max, window.ScrollTargetEdgeSnapDist.y, center_y_ratio);
        }
        scroll.y = scroll_target_y - center_y_ratio * (window.SizeFull.y - decoration_total_height);
    }
    scroll.x = IM_FLOOR(ImMax(scroll.x, 0.0));
    scroll.y = IM_FLOOR(ImMax(scroll.y, 0.0));
    if (!window.Collapsed && !window.SkipItems)
    {
        scroll.x = ImMin(scroll.x, window.ScrollMax.x);
        scroll.y = ImMin(scroll.y, window.ScrollMax.y);
    }
    return scroll;
}

void ImGui::ScrollToItem(ImGuiScrollFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    ScrollToRectEx(window, g.last_item_data.NavRect, flags);
}

void ImGui::ScrollToRect(ImGuiWindow* window, const ImRect& item_rect, ImGuiScrollFlags flags)
{
    ScrollToRectEx(window, item_rect, flags);
}

// scroll to keep newly navigated item fully into view
Vector2D ImGui::ScrollToRectEx(ImGuiWindow* window, const ImRect& item_rect, ImGuiScrollFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImRect window_rect(window.InnerRect.Min - DimgVec2D::new(1, 1), window.InnerRect.Max + DimgVec2D::new(1, 1));
    //GetForegroundDrawList(window)->add_rect(window_rect.min, window_rect.max, IM_COL32_WHITE); // [DEBUG]

    // Check that only one behavior is selected per axis
    IM_ASSERT((flags & ImGuiScrollFlags_MaskX_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskX_));
    IM_ASSERT((flags & ImGuiScrollFlags_MaskY_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskY_));

    // Defaults
    ImGuiScrollFlags in_flags = flags;
    if ((flags & ImGuiScrollFlags_MaskX_) == 0 && window.ScrollbarX)
        flags |= ImGuiScrollFlags_KeepVisibleEdgeX;
    if ((flags & ImGuiScrollFlags_MaskY_) == 0)
        flags |= window.Appearing ? ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeY;

    const bool fully_visible_x = item_rect.Min.x >= window_rect.Min.x && item_rect.Max.x <= window_rect.Max.x;
    const bool fully_visible_y = item_rect.Min.y >= window_rect.Min.y && item_rect.Max.y <= window_rect.Max.y;
    const bool can_be_fully_visible_x = (item_rect.GetWidth() + g.Style.ItemSpacing.x * 2.0) <= window_rect.GetWidth();
    const bool can_be_fully_visible_y = (item_rect.GetHeight() + g.Style.ItemSpacing.y * 2.0) <= window_rect.GetHeight();

    if ((flags & ImGuiScrollFlags_KeepVisibleEdgeX) && !fully_visible_x)
    {
        if (item_rect.Min.x < window_rect.Min.x || !can_be_fully_visible_x)
            SetScrollFromPosX(window, item_rect.Min.x - g.Style.ItemSpacing.x - window.Pos.x, 0.0);
        else if (item_rect.Max.x >= window_rect.Max.x)
            SetScrollFromPosX(window, item_rect.Max.x + g.Style.ItemSpacing.x - window.Pos.x, 1.0);
    }
    else if (((flags & ImGuiScrollFlags_KeepVisibleCenterX) && !fully_visible_x) || (flags & ImGuiScrollFlags_AlwaysCenterX))
    {
        float target_x = can_be_fully_visible_x ? ImFloor((item_rect.Min.x + item_rect.Max.x - window.InnerRect.GetWidth()) * 0.5) : item_rect.Min.x;
        SetScrollFromPosX(window, target_x - window.Pos.x, 0.0);
    }

    if ((flags & ImGuiScrollFlags_KeepVisibleEdgeY) && !fully_visible_y)
    {
        if (item_rect.Min.y < window_rect.Min.y || !can_be_fully_visible_y)
            SetScrollFromPosY(window, item_rect.Min.y - g.Style.ItemSpacing.y - window.Pos.y, 0.0);
        else if (item_rect.Max.y >= window_rect.Max.y)
            SetScrollFromPosY(window, item_rect.Max.y + g.Style.ItemSpacing.y - window.Pos.y, 1.0);
    }
    else if (((flags & ImGuiScrollFlags_KeepVisibleCenterY) && !fully_visible_y) || (flags & ImGuiScrollFlags_AlwaysCenterY))
    {
        float target_y = can_be_fully_visible_y ? ImFloor((item_rect.Min.y + item_rect.Max.y - window.InnerRect.GetHeight()) * 0.5) : item_rect.Min.y;
        SetScrollFromPosY(window, target_y - window.Pos.y, 0.0);
    }

    Vector2D next_scroll = CalcNextScrollFromScrollTargetAndClamp(window);
    Vector2D delta_scroll = next_scroll - window.Scroll;

    // Also scroll parent window to keep us into view if necessary
    if (!(flags & ImGuiScrollFlags_NoScrollParent) && (window.Flags & ImGuiWindowFlags_ChildWindow))
    {
        // FIXME-SCROLL: May be an option?
        if ((in_flags & (ImGuiScrollFlags_AlwaysCenterX | ImGuiScrollFlags_KeepVisibleCenterX)) != 0)
            in_flags = (in_flags & ~ImGuiScrollFlags_MaskX_) | ImGuiScrollFlags_KeepVisibleEdgeX;
        if ((in_flags & (ImGuiScrollFlags_AlwaysCenterY | ImGuiScrollFlags_KeepVisibleCenterY)) != 0)
            in_flags = (in_flags & ~ImGuiScrollFlags_MaskY_) | ImGuiScrollFlags_KeepVisibleEdgeY;
        delta_scroll += ScrollToRectEx(window.ParentWindow, ImRect(item_rect.Min - delta_scroll, item_rect.Max - delta_scroll), in_flags);
    }

    return delta_scroll;
}

float ImGui::GetScrollX()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.Scroll.x;
}

float ImGui::GetScrollY()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.Scroll.y;
}

float ImGui::GetScrollMaxX()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.ScrollMax.x;
}

float ImGui::GetScrollMaxY()
{
    ImGuiWindow* window = GImGui->CurrentWindow;
    return window.ScrollMax.y;
}

void ImGui::SetScrollX(ImGuiWindow* window, float scroll_x)
{
    window.ScrollTarget.x = scroll_x;
    window.ScrollTargetCenterRatio.x = 0.0;
    window.ScrollTargetEdgeSnapDist.x = 0.0;
}

void ImGui::SetScrollY(ImGuiWindow* window, float scroll_y)
{
    window.ScrollTarget.y = scroll_y;
    window.ScrollTargetCenterRatio.y = 0.0;
    window.ScrollTargetEdgeSnapDist.y = 0.0;
}

void ImGui::SetScrollX(float scroll_x)
{
    ImGuiContext& g = *GImGui;
    SetScrollX(g.CurrentWindow, scroll_x);
}

void ImGui::SetScrollY(float scroll_y)
{
    ImGuiContext& g = *GImGui;
    SetScrollY(g.CurrentWindow, scroll_y);
}

// Note that a local position will vary depending on initial scroll value,
// This is a little bit confusing so bear with us:
//  - local_pos = (absolution_pos - window->pos)
//  - So local_x/local_y are 0.0 for a position at the upper-left corner of a window,
//    and generally local_x/local_y are >(padding+decoration) && <(size-padding-decoration) when in the visible area.
//  - They mostly exists because of legacy API.
// Following the rules above, when trying to work with scrolling code, consider that:
//  - SetScrollFromPosY(0.0) == SetScrollY(0.0 + scroll.y) == has no effect!
//  - SetScrollFromPosY(-scroll.y) == SetScrollY(-scroll.y + scroll.y) == SetScrollY(0.0) == reset scroll. Of course writing SetScrollY(0.0) directly then makes more sense
// We store a target position so centering and clamping can occur on the next frame when we are guaranteed to have a known window size
void ImGui::SetScrollFromPosX(ImGuiWindow* window, float local_x, float center_x_ratio)
{
    IM_ASSERT(center_x_ratio >= 0.0 && center_x_ratio <= 1.0);
    window.ScrollTarget.x = IM_FLOOR(local_x + window.Scroll.x); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.x = center_x_ratio;
    window.ScrollTargetEdgeSnapDist.x = 0.0;
}

void ImGui::SetScrollFromPosY(ImGuiWindow* window, float local_y, float center_y_ratio)
{
    IM_ASSERT(center_y_ratio >= 0.0 && center_y_ratio <= 1.0);
    const float decoration_up_height = window.TitleBarHeight() + window.MenuBarHeight(); // FIXME: Would be nice to have a more standardized access to our scrollable/client rect;
    local_y -= decoration_up_height;
    window.ScrollTarget.y = IM_FLOOR(local_y + window.Scroll.y); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.y = center_y_ratio;
    window.ScrollTargetEdgeSnapDist.y = 0.0;
}

void ImGui::SetScrollFromPosX(float local_x, float center_x_ratio)
{
    ImGuiContext& g = *GImGui;
    SetScrollFromPosX(g.CurrentWindow, local_x, center_x_ratio);
}

void ImGui::SetScrollFromPosY(float local_y, float center_y_ratio)
{
    ImGuiContext& g = *GImGui;
    SetScrollFromPosY(g.CurrentWindow, local_y, center_y_ratio);
}

// center_x_ratio: 0.0 left of last item, 0.5 horizontal center of last item, 1.0 right of last item.
void ImGui::SetScrollHereX(float center_x_ratio)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    float spacing_x = ImMax(window.WindowPadding.x, g.Style.ItemSpacing.x);
    float target_pos_x = ImLerp(g.last_item_data.Rect.Min.x - spacing_x, g.last_item_data.Rect.Max.x + spacing_x, center_x_ratio);
    SetScrollFromPosX(window, target_pos_x - window.Pos.x, center_x_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.x = ImMax(0.0, window.WindowPadding.x - spacing_x);
}

// center_y_ratio: 0.0 top of last item, 0.5 vertical center of last item, 1.0 bottom of last item.
void ImGui::SetScrollHereY(float center_y_ratio)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    float spacing_y = ImMax(window.WindowPadding.y, g.Style.ItemSpacing.y);
    float target_pos_y = ImLerp(window.DC.CursorPosPrevLine.y - spacing_y, window.DC.CursorPosPrevLine.y + window.DC.PrevLineSize.y + spacing_y, center_y_ratio);
    SetScrollFromPosY(window, target_pos_y - window.Pos.y, center_y_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.y = ImMax(0.0, window.WindowPadding.y - spacing_y);
}

//-----------------------------------------------------------------------------
// [SECTION] TOOLTIPS
//-----------------------------------------------------------------------------

void ImGui::BeginTooltip()
{
    BeginTooltipEx(ImGuiTooltipFlags_None, ImGuiWindowFlags_None);
}

void ImGui::BeginTooltipEx(ImGuiTooltipFlags tooltip_flags, ImGuiWindowFlags extra_window_flags)
{
    ImGuiContext& g = *GImGui;

    if (g.DragDropWithinSource || g.DragDropWithinTarget)
    {
        // The default tooltip position is a little offset to give space to see the context menu (it's also clamped within the current viewport/monitor)
        // In the context of a dragging tooltip we try to reduce that offset and we enforce following the cursor.
        // Whatever we do we want to call SetNextWindowPos() to enforce a tooltip position and disable clipping the tooltip without our display area, like regular tooltip do.
        //Vector2D tooltip_pos = g.io.mouse_pos - g.ActiveIdClickOffset - g.style.window_padding;
        Vector2D tooltip_pos = g.io.MousePos + DimgVec2D::new(16 * g.Style.MouseCursorScale, 8 * g.Style.MouseCursorScale);
        SetNextWindowPos(tooltip_pos);
        SetNextWindowBgAlpha(g.Style.Colors[ImGuiCol_PopupBg].w * 0.60);
        //PushStyleVar(ImGuiStyleVar_Alpha, g.style.Alpha * 0.60); // This would be nice but e.g ColorButton with checkboard has issue with transparent colors :(
        tooltip_flags |= ImGuiTooltipFlags_OverridePreviousTooltip;
    }

    char window_name[16];
    ImFormatString(window_name, IM_ARRAYSIZE(window_name), "##Tooltip_%02d", g.TooltipOverrideCount);
    if (tooltip_flags & ImGuiTooltipFlags_OverridePreviousTooltip)
        if (ImGuiWindow* window = FindWindowByName(window_name))
            if (window.Active)
            {
                // Hide previous tooltip from being displayed. We can't easily "reset" the content of a window so we create a new one.
                window.Hidden = true;
                window.HiddenFramesCanSkipItems = 1; // FIXME: This may not be necessary?
                ImFormatString(window_name, IM_ARRAYSIZE(window_name), "##Tooltip_%02d", g.TooltipOverrideCount += 1);
            }
    ImGuiWindowFlags flags = ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_NoInputs | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoDocking;
    Begin(window_name, NULL, flags | extra_window_flags);
}

void ImGui::EndTooltip()
{
    IM_ASSERT(GetCurrentWindowRead().flags & ImGuiWindowFlags_Tooltip);   // Mismatched BeginTooltip()/EndTooltip() calls
    End();
}

void ImGui::SetTooltipV(const char* fmt, va_list args)
{
    BeginTooltipEx(ImGuiTooltipFlags_OverridePreviousTooltip, ImGuiWindowFlags_None);
    TextV(fmt, args);
    EndTooltip();
}

void ImGui::SetTooltip(const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    SetTooltipV(fmt, args);
    va_end(args);
}

//-----------------------------------------------------------------------------
// [SECTION] POPUPS
//-----------------------------------------------------------------------------

// Supported flags: ImGuiPopupFlags_AnyPopupId, ImGuiPopupFlags_AnyPopupLevel
bool ImGui::IsPopupOpen(ImGuiID id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    if (popup_flags & ImGuiPopupFlags_AnyPopupId)
    {
        // Return true if any popup is open at the current BeginPopup() level of the popup stack
        // This may be used to e.g. test for another popups already opened to handle popups priorities at the same level.
        IM_ASSERT(id == 0);
        if (popup_flags & ImGuiPopupFlags_AnyPopupLevel)
            return g.OpenPopupStack.Size > 0;
        else
            return g.OpenPopupStack.Size > g.BeginPopupStack.Size;
    }
    else
    {
        if (popup_flags & ImGuiPopupFlags_AnyPopupLevel)
        {
            // Return true if the popup is open anywhere in the popup stack
            for (int n = 0; n < g.OpenPopupStack.Size; n += 1)
                if (g.OpenPopupStack[n].PopupId == id)
                    return true;
            return false;
        }
        else
        {
            // Return true if the popup is open at the current BeginPopup() level of the popup stack (this is the most-common query)
            return g.OpenPopupStack.Size > g.BeginPopupStack.Size && g.OpenPopupStack[g.BeginPopupStack.Size].PopupId == id;
        }
    }
}

bool ImGui::IsPopupOpen(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiID id = (popup_flags & ImGuiPopupFlags_AnyPopupId) ? 0 : g.CurrentWindow->GetID(str_id);
    if ((popup_flags & ImGuiPopupFlags_AnyPopupLevel) && id != 0)
        IM_ASSERT(0 && "Cannot use IsPopupOpen() with a string id and ImGuiPopupFlags_AnyPopupLevel."); // But non-string version is legal and used internally
    return IsPopupOpen(id, popup_flags);
}

ImGuiWindow* ImGui::GetTopMostPopupModal()
{
    ImGuiContext& g = *GImGui;
    for (int n = g.OpenPopupStack.Size - 1; n >= 0; n--)
        if (ImGuiWindow* popup = g.OpenPopupStack.Data[n].Window)
            if (popup.flags & ImGuiWindowFlags_Modal)
                return popup;
    return NULL;
}

ImGuiWindow* ImGui::GetTopMostAndVisiblePopupModal()
{
    ImGuiContext& g = *GImGui;
    for (int n = g.OpenPopupStack.Size - 1; n >= 0; n--)
        if (ImGuiWindow* popup = g.OpenPopupStack.Data[n].Window)
            if ((popup.flags & ImGuiWindowFlags_Modal) && IsWindowActiveAndVisible(popup))
                return popup;
    return NULL;
}

void ImGui::OpenPopup(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiID id = g.CurrentWindow->GetID(str_id);
    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopup(\"%s\" -> 0x%08X\n", str_id, id);
    OpenPopupEx(id, popup_flags);
}

void ImGui::OpenPopup(ImGuiID id, ImGuiPopupFlags popup_flags)
{
    OpenPopupEx(id, popup_flags);
}

// Mark popup as open (toggle toward open state).
// Popups are closed when user click outside, or activate a pressable item, or CloseCurrentPopup() is called within a BeginPopup()/EndPopup() block.
// Popup identifiers are relative to the current id-stack (so OpenPopup and BeginPopup needs to be at the same level).
// One open popup per level of the popup hierarchy (NB: when assigning we reset the Window member of ImGuiPopupRef to NULL)
void ImGui::OpenPopupEx(ImGuiID id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* parent_window = g.CurrentWindow;
    const int current_stack_size = g.BeginPopupStack.Size;

    if (popup_flags & ImGuiPopupFlags_NoOpenOverExistingPopup)
        if (IsPopupOpen(0u, ImGuiPopupFlags_AnyPopupId))
            return;

    ImGuiPopupData popup_ref; // Tagged as new ref as Window will be set back to NULL if we write this into open_popup_stack.
    popup_ref.PopupId = id;
    popup_ref.Window = NULL;
    popup_ref.SourceWindow = g.nav_window;
    popup_ref.OpenFrameCount = g.FrameCount;
    popup_ref.OpenParentId = parent_window.IDStack.back();
    popup_ref.OpenPopupPos = NavCalcPreferredRefPos();
    popup_ref.OpenMousePos = is_mouse_pos_valid(&g.io.MousePos) ? g.io.MousePos : popup_ref.OpenPopupPos;

    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopupEx(0x%08X)\n", id);
    if (g.OpenPopupStack.Size < current_stack_size + 1)
    {
        g.OpenPopupStack.push_back(popup_ref);
    }
    else
    {
        // Gently handle the user mistakenly calling OpenPopup() every frame. It is a programming mistake! However, if we were to run the regular code path, the ui
        // would become completely unusable because the popup will always be in hidden-while-calculating-size state _while_ claiming focus. Which would be a very confusing
        // situation for the programmer. Instead, we silently allow the popup to proceed, it will keep reappearing and the programming error will be more obvious to understand.
        if (g.OpenPopupStack[current_stack_size].PopupId == id && g.OpenPopupStack[current_stack_size].OpenFrameCount == g.FrameCount - 1)
        {
            g.OpenPopupStack[current_stack_size].OpenFrameCount = popup_ref.OpenFrameCount;
        }
        else
        {
            // Close child popups if any, then flag popup for open/reopen
            ClosePopupToLevel(current_stack_size, false);
            g.OpenPopupStack.push_back(popup_ref);
        }

        // When reopening a popup we first refocus its parent, otherwise if its parent is itself a popup it would get closed by ClosePopupsOverWindow().
        // This is equivalent to what ClosePopupToLevel() does.
        //if (g.open_popup_stack[current_stack_size].popup_id == id)
        //    focus_window(parent_window);
    }
}

// When popups are stacked, clicking on a lower level popups puts focus back to it and close popups above it.
// This function closes any popups that are over 'ref_window'.
void ImGui::ClosePopupsOverWindow(ImGuiWindow* ref_window, bool restore_focus_to_window_under_popup)
{
    ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size == 0)
        return;

    // Don't close our own child popup windows.
    int popup_count_to_keep = 0;
    if (ref_window)
    {
        // Find the highest popup which is a descendant of the reference window (generally reference window = nav_window)
        for (; popup_count_to_keep < g.OpenPopupStack.Size; popup_count_to_keep += 1)
        {
            ImGuiPopupData& popup = g.OpenPopupStack[popup_count_to_keep];
            if (!popup.Window)
                continue;
            IM_ASSERT((popup.Window.flags & ImGuiWindowFlags_Popup) != 0);
            if (popup.Window.flags & ImGuiWindowFlags_ChildWindow)
                continue;

            // Trim the stack unless the popup is a direct parent of the reference window (the reference window is often the nav_window)
            // - With this stack of window, clicking/focusing Popup1 will close Popup2 and Popup3:
            //     Window -> Popup1 -> Popup2 -> Popup3
            // - Each popups may contain child windows, which is why we compare ->root_window_dock_tree!
            //     Window -> Popup1 -> Popup1_Child -> Popup2 -> Popup2_Child
            bool ref_window_is_descendent_of_popup = false;
            for (int n = popup_count_to_keep; n < g.OpenPopupStack.Size; n += 1)
                if (ImGuiWindow* popup_window = g.OpenPopupStack[n].Window)
                    //if (popup_window->root_window_dock_tree == ref_window->root_window_dock_tree) // FIXME-MERGE
                    if (IsWindowWithinBeginStackOf(ref_window, popup_window))
                    {
                        ref_window_is_descendent_of_popup = true;
                        break;
                    }
            if (!ref_window_is_descendent_of_popup)
                break;
        }
    }
    if (popup_count_to_keep < g.OpenPopupStack.Size) // This test is not required but it allows to set a convenient breakpoint on the statement below
    {
        IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupsOverWindow(\"%s\")\n", ref_window ? ref_window.Name : "<NULL>");
        ClosePopupToLevel(popup_count_to_keep, restore_focus_to_window_under_popup);
    }
}

void ImGui::ClosePopupsExceptModals()
{
    ImGuiContext& g = *GImGui;

    int popup_count_to_keep;
    for (popup_count_to_keep = g.OpenPopupStack.Size; popup_count_to_keep > 0; popup_count_to_keep--)
    {
        ImGuiWindow* window = g.OpenPopupStack[popup_count_to_keep - 1].Window;
        if (!window || window.Flags & ImGuiWindowFlags_Modal)
            break;
    }
    if (popup_count_to_keep < g.OpenPopupStack.Size) // This test is not required but it allows to set a convenient breakpoint on the statement below
        ClosePopupToLevel(popup_count_to_keep, true);
}

void ImGui::ClosePopupToLevel(int remaining, bool restore_focus_to_window_under_popup)
{
    ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupToLevel(%d), restore_focus_to_window_under_popup=%d\n", remaining, restore_focus_to_window_under_popup);
    IM_ASSERT(remaining >= 0 && remaining < g.OpenPopupStack.Size);

    // Trim open popup stack
    ImGuiWindow* focus_window = g.OpenPopupStack[remaining].SourceWindow;
    ImGuiWindow* popup_window = g.OpenPopupStack[remaining].Window;
    g.OpenPopupStack.resize(remaining);

    if (restore_focus_to_window_under_popup)
    {
        if (focus_window && !focus_window.WasActive && popup_window)
        {
            // Fallback
            FocusTopMostWindowUnderOne(popup_window, NULL);
        }
        else
        {
            if (g.NavLayer == ImGuiNavLayer_Main && focus_window)
                focus_window = NavRestoreLastChildNavWindow(focus_window);
            focus_window(focus_window);
        }
    }
}

// Close the popup we have begin-ed into.
void ImGui::CloseCurrentPopup()
{
    ImGuiContext& g = *GImGui;
    int popup_idx = g.BeginPopupStack.Size - 1;
    if (popup_idx < 0 || popup_idx >= g.OpenPopupStack.Size || g.BeginPopupStack[popup_idx].PopupId != g.OpenPopupStack[popup_idx].PopupId)
        return;

    // Closing a menu closes its top-most parent popup (unless a modal)
    while (popup_idx > 0)
    {
        ImGuiWindow* popup_window = g.OpenPopupStack[popup_idx].Window;
        ImGuiWindow* parent_popup_window = g.OpenPopupStack[popup_idx - 1].Window;
        bool close_parent = false;
        if (popup_window && (popup_window.Flags & ImGuiWindowFlags_ChildMenu))
            if (parent_popup_window && !(parent_popup_window.Flags & ImGuiWindowFlags_MenuBar))
                close_parent = true;
        if (!close_parent)
            break;
        popup_idx--;
    }
    IMGUI_DEBUG_LOG_POPUP("[popup] CloseCurrentPopup %d -> %d\n", g.BeginPopupStack.Size - 1, popup_idx);
    ClosePopupToLevel(popup_idx, true);

    // A common pattern is to close a popup when selecting a menu item/selectable that will open another window.
    // To improve this usage pattern, we avoid nav highlight for a single frame in the parent window.
    // Similarly, we could avoid mouse hover highlight in this window but it is less visually problematic.
    if (ImGuiWindow* window = g.nav_window)
        window.DC.NavHideHighlightOneFrame = true;
}

// Attention! BeginPopup() adds default flags which BeginPopupEx()!
bool ImGui::BeginPopupEx(ImGuiID id, ImGuiWindowFlags flags)
{
    ImGuiContext& g = *GImGui;
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    char name[20];
    if (flags & ImGuiWindowFlags_ChildMenu)
        ImFormatString(name, IM_ARRAYSIZE(name), "##Menu_%02d", g.BeginMenuCount); // Recycle windows based on depth
    else
        ImFormatString(name, IM_ARRAYSIZE(name), "##Popup_%08x", id); // Not recycling, so we can close/open during the same frame

    flags |= ImGuiWindowFlags_Popup | ImGuiWindowFlags_NoDocking;
    bool is_open = Begin(name, NULL, flags);
    if (!is_open) // NB: Begin can return false when the popup is completely clipped (e.g. zero size display)
        EndPopup();

    return is_open;
}

bool ImGui::BeginPopup(const char* str_id, ImGuiWindowFlags flags)
{
    ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size <= g.BeginPopupStack.Size) // Early out for performance
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }
    flags |= ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings;
    ImGuiID id = g.CurrentWindow->GetID(str_id);
    return BeginPopupEx(id, flags);
}

// If 'p_open' is specified for a modal popup window, the popup will have a regular close button which will close the popup.
// Note that popup visibility status is owned by Dear ImGui (and manipulated with e.g. OpenPopup) so the actual value of *p_open is meaningless here.
bool ImGui::BeginPopupModal(const char* name, bool* p_open, ImGuiWindowFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    const ImGuiID id = window.GetID(name);
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    // Center modal windows by default for increased visibility
    // (this won't really last as settings will kick in, and is mostly for backward compatibility. user may do the same themselves)
    // FIXME: Should test for (PosCond & window->set_window_pos_allow_flags) with the upcoming window.
    if ((g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasPos) == 0)
    {
        const ImGuiViewport* viewport = window.WasActive ? window.viewport : GetMainViewport(); // FIXME-VIEWPORT: What may be our reference viewport?
        SetNextWindowPos(viewport->GetCenter(), ImGuiCond_FirstUseEver, DimgVec2D::new(0.5, 0.5));
    }

    flags |= ImGuiWindowFlags_Popup | ImGuiWindowFlags_Modal | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoDocking;
    const bool is_open = Begin(name, p_open, flags);
    if (!is_open || (p_open && !*p_open)) // NB: is_open can be 'false' when the popup is completely clipped (e.g. zero size display)
    {
        EndPopup();
        if (is_open)
            ClosePopupToLevel(g.BeginPopupStack.Size, true);
        return false;
    }
    return is_open;
}

void ImGui::EndPopup()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    IM_ASSERT(window.Flags & ImGuiWindowFlags_Popup);  // Mismatched BeginPopup()/EndPopup() calls
    IM_ASSERT(g.BeginPopupStack.Size > 0);

    // Make all menus and popups wrap around for now, may need to expose that policy (e.g. focus scope could include wrap/loop policy flags used by new move requests)
    if (g.nav_window == window)
        NavMoveRequestTryWrapping(window, ImGuiNavMoveFlags_LoopY);

    // Child-popups don't need to be laid out
    IM_ASSERT(g.WithinEndChild == false);
    if (window.Flags & ImGuiWindowFlags_ChildWindow)
        g.WithinEndChild = true;
    End();
    g.WithinEndChild = false;
}

// Helper to open a popup if mouse button is released over the item
// - This is essentially the same as BeginPopupContextItem() but without the trailing BeginPopup()
void ImGui::OpenPopupOnItemClick(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
    {
        ImGuiID id = str_id ? window.GetID(str_id) : g.last_item_data.ID;    // If user hasn't passed an id, we can use the LastItemID. Using LastItemID as a Popup id won't conflict!
        IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
        OpenPopupEx(id, popup_flags);
    }
}

// This is a helper to handle the simplest case of associating one named popup to one given widget.
// - To create a popup associated to the last item, you generally want to pass a NULL value to str_id.
// - To create a popup with a specific identifier, pass it in str_id.
//    - This is useful when using using BeginPopupContextItem() on an item which doesn't have an identifier, e.g. a Text() call.
//    - This is useful when multiple code locations may want to manipulate/open the same popup, given an explicit id.
// - You may want to handle the whole on user side if you have specific needs (e.g. tweaking IsItemHovered() parameters).
//   This is essentially the same as:
//       id = str_id ? GetID(str_id) : GetItemID();
//       OpenPopupOnItemClick(str_id, ImGuiPopupFlags_MouseButtonRight);
//       return BeginPopup(id);
//   Which is essentially the same as:
//       id = str_id ? GetID(str_id) : GetItemID();
//       if (IsItemHovered() && IsMouseReleased(ImGuiMouseButton_Right))
//           OpenPopup(id);
//       return BeginPopup(id);
//   The main difference being that this is tweaked to avoid computing the id twice.
bool ImGui::BeginPopupContextItem(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    if (window.SkipItems)
        return false;
    ImGuiID id = str_id ? window.GetID(str_id) : g.last_item_data.ID;    // If user hasn't passed an id, we can use the LastItemID. Using LastItemID as a Popup id won't conflict!
    IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

bool ImGui::BeginPopupContextWindow(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    if (!str_id)
        str_id = "window_context";
    ImGuiID id = window.GetID(str_id);
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsWindowHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        if (!(popup_flags & ImGuiPopupFlags_NoOpenOverItems) || !IsAnyItemHovered())
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

bool ImGui::BeginPopupContextVoid(const char* str_id, ImGuiPopupFlags popup_flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    if (!str_id)
        str_id = "void_context";
    ImGuiID id = window.GetID(str_id);
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && !IsWindowHovered(ImGuiHoveredFlags_AnyWindow))
        if (GetTopMostPopupModal() == NULL)
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

// r_avoid = the rectangle to avoid (e.g. for tooltip it is a rectangle around the mouse cursor which we want to avoid. for popups it's a small point around the cursor.)
// r_outer = the visible area rectangle, minus safe area padding. If our popup size won't fit because of safe area padding we ignore it.
// (r_outer is usually equivalent to the viewport rectangle minus padding, but when multi-viewports are enabled and monitor
//  information are available, it may represent the entire platform monitor from the frame of reference of the current viewport.
//  this allows us to have tooltips/popups displayed out of the parent viewport.)
Vector2D ImGui::FindBestWindowPosForPopupEx(const Vector2D& ref_pos, const Vector2D& size, ImGuiDir* last_dir, const ImRect& r_outer, const ImRect& r_avoid, ImGuiPopupPositionPolicy policy)
{
    Vector2D base_pos_clamped = ImClamp(ref_pos, r_outer.Min, r_outer.Max - size);
    //GetForegroundDrawList()->add_rect(r_avoid.min, r_avoid.max, IM_COL32(255,0,0,255));
    //GetForegroundDrawList()->add_rect(r_outer.min, r_outer.max, IM_COL32(0,255,0,255));

    // Combo Box policy (we want a connecting edge)
    if (policy == ImGuiPopupPositionPolicy_ComboBox)
    {
        const ImGuiDir dir_prefered_order[ImGuiDir_COUNT] = { ImGuiDir_Down, ImGuiDir_Right, ImGuiDir_Left, ImGuiDir_Up };
        for (int n = (*last_dir != ImGuiDir_None) ? -1 : 0; n < ImGuiDir_COUNT; n += 1)
        {
            const ImGuiDir dir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;
            Vector2D pos;
            if (dir == ImGuiDir_Down)  pos = DimgVec2D::new(r_avoid.Min.x, r_avoid.Max.y);          // Below, Toward Right (default)
            if (dir == ImGuiDir_Right) pos = DimgVec2D::new(r_avoid.Min.x, r_avoid.Min.y - size.y); // Above, Toward Right
            if (dir == ImGuiDir_Left)  pos = DimgVec2D::new(r_avoid.Max.x - size.x, r_avoid.Max.y); // Below, Toward Left
            if (dir == ImGuiDir_Up)    pos = DimgVec2D::new(r_avoid.Max.x - size.x, r_avoid.Min.y - size.y); // Above, Toward Left
            if (!r_outer.Contains(ImRect(pos, pos + size)))
                continue;
            *last_dir = dir;
            return pos;
        }
    }

    // Tooltip and Default popup policy
    // (Always first try the direction we used on the last frame, if any)
    if (policy == ImGuiPopupPositionPolicy_Tooltip || policy == ImGuiPopupPositionPolicy_Default)
    {
        const ImGuiDir dir_prefered_order[ImGuiDir_COUNT] = { ImGuiDir_Right, ImGuiDir_Down, ImGuiDir_Up, ImGuiDir_Left };
        for (int n = (*last_dir != ImGuiDir_None) ? -1 : 0; n < ImGuiDir_COUNT; n += 1)
        {
            const ImGuiDir dir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;

            const float avail_w = (dir == ImGuiDir_Left ? r_avoid.Min.x : r_outer.Max.x) - (dir == ImGuiDir_Right ? r_avoid.Max.x : r_outer.Min.x);
            const float avail_h = (dir == ImGuiDir_Up ? r_avoid.Min.y : r_outer.Max.y) - (dir == ImGuiDir_Down ? r_avoid.Max.y : r_outer.Min.y);

            // If there not enough room on one axis, there's no point in positioning on a side on this axis (e.g. when not enough width, use a top/bottom position to maximize available width)
            if (avail_w < size.x && (dir == ImGuiDir_Left || dir == ImGuiDir_Right))
                continue;
            if (avail_h < size.y && (dir == ImGuiDir_Up || dir == ImGuiDir_Down))
                continue;

            Vector2D pos;
            pos.x = (dir == ImGuiDir_Left) ? r_avoid.Min.x - size.x : (dir == ImGuiDir_Right) ? r_avoid.Max.x : base_pos_clamped.x;
            pos.y = (dir == ImGuiDir_Up) ? r_avoid.Min.y - size.y : (dir == ImGuiDir_Down) ? r_avoid.Max.y : base_pos_clamped.y;

            // Clamp top-left corner of popup
            pos.x = ImMax(pos.x, r_outer.Min.x);
            pos.y = ImMax(pos.y, r_outer.Min.y);

            *last_dir = dir;
            return pos;
        }
    }

    // Fallback when not enough room:
    *last_dir = ImGuiDir_None;

    // For tooltip we prefer avoiding the cursor at all cost even if it means that part of the tooltip won't be visible.
    if (policy == ImGuiPopupPositionPolicy_Tooltip)
        return ref_pos + DimgVec2D::new(2, 2);

    // Otherwise try to keep within display
    Vector2D pos = ref_pos;
    pos.x = ImMax(ImMin(pos.x + size.x, r_outer.Max.x) - size.x, r_outer.Min.x);
    pos.y = ImMax(ImMin(pos.y + size.y, r_outer.Max.y) - size.y, r_outer.Min.y);
    return pos;
}

// Note that this is used for popups, which can overlap the non work-area of individual viewports.
ImRect ImGui::GetPopupAllowedExtentRect(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    ImRect r_screen;
    if (window.ViewportAllowPlatformMonitorExtend >= 0)
    {
        // Extent with be in the frame of reference of the given viewport (so min is likely to be negative here)
        const ImGuiPlatformMonitor& monitor = g.PlatformIO.Monitors[window.ViewportAllowPlatformMonitorExtend];
        r_screen.Min = monitor.WorkPos;
        r_screen.Max = monitor.WorkPos + monitor.WorkSize;
    }
    else
    {
        // Use the full viewport area (not work area) for popups
        r_screen = window.viewport->GetMainRect();
    }
    Vector2D padding = g.Style.DisplaySafeAreaPadding;
    r_screen.Expand(DimgVec2D::new((r_screen.GetWidth() > padding.x * 2) ? -padding.x : 0.0, (r_screen.GetHeight() > padding.y * 2) ? -padding.y : 0.0));
    return r_screen;
}

Vector2D ImGui::FindBestWindowPosForPopup(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;

    ImRect r_outer = GetPopupAllowedExtentRect(window);
    if (window.Flags & ImGuiWindowFlags_ChildMenu)
    {
        // Child menus typically request _any_ position within the parent menu item, and then we move the new menu outside the parent bounds.
        // This is how we end up with child menus appearing (most-commonly) on the right of the parent menu.
        ImGuiWindow* parent_window = window.ParentWindow;
        float horizontal_overlap = g.Style.ItemInnerSpacing.x; // We want some overlap to convey the relative depth of each menu (currently the amount of overlap is hard-coded to style.ItemSpacing.x).
        ImRect r_avoid;
        if (parent_window.DC.MenuBarAppending)
            r_avoid = ImRect(-FLT_MAX, parent_window.ClipRect.Min.y, FLT_MAX, parent_window.ClipRect.Max.y); // Avoid parent menu-bar. If we wanted multi-line menu-bar, we may instead want to have the calling window setup e.g. a next_window_data.PosConstraintAvoidRect field
        else
            r_avoid = ImRect(parent_window.Pos.x + horizontal_overlap, -FLT_MAX, parent_window.Pos.x + parent_window.Size.x - horizontal_overlap - parent_window.ScrollbarSizes.x, FLT_MAX);
        return FindBestWindowPosForPopupEx(window.Pos, window.Size, &window.AutoPosLastDirection, r_outer, r_avoid, ImGuiPopupPositionPolicy_Default);
    }
    if (window.Flags & ImGuiWindowFlags_Popup)
    {
        return FindBestWindowPosForPopupEx(window.Pos, window.Size, &window.AutoPosLastDirection, r_outer, ImRect(window.Pos, window.Pos), ImGuiPopupPositionPolicy_Default); // Ideally we'd disable r_avoid here
    }
    if (window.Flags & ImGuiWindowFlags_Tooltip)
    {
        // Position tooltip (always follows mouse)
        float sc = g.Style.MouseCursorScale;
        Vector2D ref_pos = NavCalcPreferredRefPos();
        ImRect r_avoid;
        if (!g.NavDisableHighlight && g.NavDisableMouseHover && !(g.io.ConfigFlags & ImGuiConfigFlags_NavEnableSetMousePos))
            r_avoid = ImRect(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 16, ref_pos.y + 8);
        else
            r_avoid = ImRect(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 24 * sc, ref_pos.y + 24 * sc); // FIXME: Hard-coded based on mouse cursor shape expectation. Exact dimension not very important.
        return FindBestWindowPosForPopupEx(ref_pos, window.Size, &window.AutoPosLastDirection, r_outer, r_avoid, ImGuiPopupPositionPolicy_Tooltip);
    }
    IM_ASSERT(0);
    return window.Pos;
}

//-----------------------------------------------------------------------------
// [SECTION] KEYBOARD/GAMEPAD NAVIGATION
//-----------------------------------------------------------------------------

// FIXME-NAV: The existence of SetNavID vs SetFocusID vs focus_window() needs to be clarified/reworked.
// In our terminology those should be interchangeable, yet right now this is super confusing.
// Those two functions are merely a legacy artifact, so at minimum naming should be clarified.

void ImGui::SetNavWindow(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    if (g.nav_window != window)
    {
        IMGUI_DEBUG_LOG_FOCUS("[focus] SetNavWindow(\"%s\")\n", window ? window.Name : "<NULL>");
        g.nav_window = window;
    }
    g.NavInitRequest = g.NavMoveSubmitted = g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

void ImGui::SetNavID(ImGuiID id, ImGuiNavLayer nav_layer, ImGuiID focus_scope_id, const ImRect& rect_rel)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.nav_window != NULL);
    IM_ASSERT(nav_layer == ImGuiNavLayer_Main || nav_layer == ImGuiNavLayer_Menu);
    g.NavId = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = focus_scope_id;
    g.nav_window->NavLastIds[nav_layer] = id;
    g.nav_window->NavRectRel[nav_layer] = rect_rel;
}

void ImGui::SetFocusID(ImGuiID id, ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(id != 0);

    if (g.nav_window != window)
       SetNavWindow(window);

    // Assume that SetFocusID() is called in the context where its window->dc.NavLayerCurrent and window->dc.NavFocusScopeIdCurrent are valid.
    // Note that window may be != g.current_window (e.g. SetFocusID call in InputTextEx for multi-line text)
    const ImGuiNavLayer nav_layer = window.DC.NavLayerCurrent;
    g.NavId = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = window.DC.NavFocusScopeIdCurrent;
    window.NavLastIds[nav_layer] = id;
    if (g.last_item_data.ID == id)
        window.NavRectRel[nav_layer] = WindowRectAbsToRel(window, g.last_item_data.NavRect);

    if (g.ActiveIdSource == ImGuiInputSource_Nav)
        g.NavDisableMouseHover = true;
    else
        g.NavDisableHighlight = true;
}

ImGuiDir ImGetDirQuadrantFromDelta(float dx, float dy)
{
    if (ImFabs(dx) > ImFabs(dy))
        return (dx > 0.0) ? ImGuiDir_Right : ImGuiDir_Left;
    return (dy > 0.0) ? ImGuiDir_Down : ImGuiDir_Up;
}

static float inline NavScoreItemDistInterval(float a0, float a1, float b0, float b1)
{
    if (a1 < b0)
        return a1 - b0;
    if (b1 < a0)
        return a0 - b1;
    return 0.0;
}

static void inline NavClampRectToVisibleAreaForMoveDir(ImGuiDir move_dir, ImRect& r, const ImRect& clip_rect)
{
    if (move_dir == ImGuiDir_Left || move_dir == ImGuiDir_Right)
    {
        r.Min.y = ImClamp(r.Min.y, clip_rect.Min.y, clip_rect.Max.y);
        r.Max.y = ImClamp(r.Max.y, clip_rect.Min.y, clip_rect.Max.y);
    }
    else // FIXME: PageUp/PageDown are leaving move_dir == None
    {
        r.Min.x = ImClamp(r.Min.x, clip_rect.Min.x, clip_rect.Max.x);
        r.Max.x = ImClamp(r.Max.x, clip_rect.Min.x, clip_rect.Max.x);
    }
}

// Scoring function for gamepad/keyboard directional navigation. Based on https://gist.github.com/rygorous/6981057
static bool ImGui::NavScoreItem(ImGuiNavItemData* result)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    if (g.NavLayer != window.DC.NavLayerCurrent)
        return false;

    // FIXME: Those are not good variables names
    ImRect cand = g.last_item_data.NavRect;   // Current item nav rectangle
    const ImRect curr = g.NavScoringRect;   // Current modified source rect (NB: we've applied max.x = min.x in NavUpdate() to inhibit the effect of having varied item width)
    g.NavScoringDebugCount += 1;

    // When entering through a NavFlattened border, we consider child window items as fully clipped for scoring
    if (window.ParentWindow == g.nav_window)
    {
        IM_ASSERT((window.Flags | g.nav_window.flags) & ImGuiWindowFlags_NavFlattened);
        if (!window.ClipRect.Overlaps(cand))
            return false;
        cand.ClipWithFull(window.ClipRect); // This allows the scored item to not overlap other candidates in the parent window
    }

    // We perform scoring on items bounding box clipped by the current clipping rectangle on the other axis (clipping on our movement axis would give us equal scores for all clipped items)
    // For example, this ensure that items in one column are not reached when moving vertically from items in another column.
    NavClampRectToVisibleAreaForMoveDir(g.NavMoveClipDir, cand, window.ClipRect);

    // Compute distance between boxes
    // FIXME-NAV: Introducing biases for vertical navigation, needs to be removed.
    float dbx = NavScoreItemDistInterval(cand.Min.x, cand.Max.x, curr.Min.x, curr.Max.x);
    float dby = NavScoreItemDistInterval(ImLerp(cand.Min.y, cand.Max.y, 0.2), ImLerp(cand.Min.y, cand.Max.y, 0.8), ImLerp(curr.Min.y, curr.Max.y, 0.2), ImLerp(curr.Min.y, curr.Max.y, 0.8)); // scale down on Y to keep using box-distance for vertically touching items
    if (dby != 0.0 && dbx != 0.0)
        dbx = (dbx / 1000.0) + ((dbx > 0.0) ? +1.0 : -1.0);
    float dist_box = ImFabs(dbx) + ImFabs(dby);

    // Compute distance between centers (this is off by a factor of 2, but we only compare center distances with each other so it doesn't matter)
    float dcx = (cand.Min.x + cand.Max.x) - (curr.Min.x + curr.Max.x);
    float dcy = (cand.Min.y + cand.Max.y) - (curr.Min.y + curr.Max.y);
    float dist_center = ImFabs(dcx) + ImFabs(dcy); // L1 metric (need this for our connectedness guarantee)

    // Determine which quadrant of 'curr' our candidate item 'cand' lies in based on distance
    ImGuiDir quadrant;
    float dax = 0.0, day = 0.0, dist_axial = 0.0;
    if (dbx != 0.0 || dby != 0.0)
    {
        // For non-overlapping boxes, use distance between boxes
        dax = dbx;
        day = dby;
        dist_axial = dist_box;
        quadrant = ImGetDirQuadrantFromDelta(dbx, dby);
    }
    else if (dcx != 0.0 || dcy != 0.0)
    {
        // For overlapping boxes with different centers, use distance between centers
        dax = dcx;
        day = dcy;
        dist_axial = dist_center;
        quadrant = ImGetDirQuadrantFromDelta(dcx, dcy);
    }
    else
    {
        // Degenerate case: two overlapping buttons with same center, break ties arbitrarily (note that LastItemId here is really the _previous_ item order, but it doesn't matter)
        quadrant = (g.last_item_data.ID < g.NavId) ? ImGuiDir_Left : ImGuiDir_Right;
    }

#if IMGUI_DEBUG_NAV_SCORING
    char buf[128];
    if (IsMouseHoveringRect(cand.Min, cand.Max))
    {
        ImFormatString(buf, IM_ARRAYSIZE(buf), "dbox (%.2,%.2->%.4)\ndcen (%.2,%.2->%.4)\nd (%.2,%.2->%.4)\nnav %c, quadrant %c", dbx, dby, dist_box, dcx, dcy, dist_center, dax, day, dist_axial, "WENS"[g.NavMoveDir], "WENS"[quadrant]);
        ImDrawList* draw_list = GetForegroundDrawList(window);
        draw_list->AddRect(curr.Min, curr.Max, IM_COL32(255,200,0,100));
        draw_list->AddRect(cand.Min, cand.Max, IM_COL32(255,255,0,200));
        draw_list->AddRectFilled(cand.Max - DimgVec2D::new(4, 4), cand.Max + CalcTextSize(buf) + DimgVec2D::new(4, 4), IM_COL32(40,0,0,150));
        draw_list->AddText(cand.Max, ~0U, buf);
    }
    else if (g.io.KeyCtrl) // Hold to preview score in matching quadrant. Press C to rotate.
    {
        if (quadrant == g.NavMoveDir)
        {
            ImFormatString(buf, IM_ARRAYSIZE(buf), "%.0/%.0", dist_box, dist_center);
            ImDrawList* draw_list = GetForegroundDrawList(window);
            draw_list->AddRectFilled(cand.Min, cand.Max, IM_COL32(255, 0, 0, 200));
            draw_list->AddText(cand.Min, IM_COL32(255, 255, 255, 255), buf);
        }
    }


    // Is it in the quadrant we're interesting in moving to?
    bool new_best = false;
    const ImGuiDir move_dir = g.NavMoveDir;
    if (quadrant == move_dir)
    {
        // Does it beat the current best candidate?
        if (dist_box < result->DistBox)
        {
            result->DistBox = dist_box;
            result->DistCenter = dist_center;
            return true;
        }
        if (dist_box == result->DistBox)
        {
            // Try using distance between center points to break ties
            if (dist_center < result->DistCenter)
            {
                result->DistCenter = dist_center;
                new_best = true;
            }
            else if (dist_center == result->DistCenter)
            {
                // Still tied! we need to be extra-careful to make sure everything gets linked properly. We consistently break ties by symbolically moving "later" items
                // (with higher index) to the right/downwards by an infinitesimal amount since we the current "best" button already (so it must have a lower index),
                // this is fairly easy. This rule ensures that all buttons with dx==dy==0 will end up being linked in order of appearance along the x axis.
                if (((move_dir == ImGuiDir_Up || move_dir == ImGuiDir_Down) ? dby : dbx) < 0.0) // moving bj to the right/down decreases distance
                    new_best = true;
            }
        }
    }

    // Axial check: if 'curr' has no link at all in some direction and 'cand' lies roughly in that direction, add a tentative link. This will only be kept if no "real" matches
    // are found, so it only augments the graph produced by the above method using extra links. (important, since it doesn't guarantee strong connectedness)
    // This is just to avoid buttons having no links in a particular direction when there's a suitable neighbor. you get good graphs without this too.
    // 2017/09/29: FIXME: This now currently only enabled inside menu bars, ideally we'd disable it everywhere. Menus in particular need to catch failure. For general navigation it feels awkward.
    // Disabling it may lead to disconnected graphs when nodes are very spaced out on different axis. Perhaps consider offering this as an option?
    if (result->DistBox == FLT_MAX && dist_axial < result->DistAxial)  // Check axial match
        if (g.NavLayer == ImGuiNavLayer_Menu && !(g.nav_window.flags & ImGuiWindowFlags_ChildMenu))
            if ((move_dir == ImGuiDir_Left && dax < 0.0) || (move_dir == ImGuiDir_Right && dax > 0.0) || (move_dir == ImGuiDir_Up && day < 0.0) || (move_dir == ImGuiDir_Down && day > 0.0))
            {
                result->DistAxial = dist_axial;
                new_best = true;
            }

    return new_best;
}

static void ImGui::NavApplyItemToResult(ImGuiNavItemData* result)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    result->Window = window;
    result->ID = g.last_item_data.ID;
    result->FocusScopeId = window.DC.NavFocusScopeIdCurrent;
    result->InFlags = g.last_item_data.InFlags;
    result->RectRel = WindowRectAbsToRel(window, g.last_item_data.NavRect);
}

// We get there when either nav_id == id, or when g.nav_any_request is set (which is updated by NavUpdateAnyRequestFlag above)
// This is called after last_item_data is set.
static void ImGui::NavProcessItem()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    const ImGuiID id = g.last_item_data.ID;
    const ImRect nav_bb = g.last_item_data.NavRect;
    const ImGuiItemFlags item_flags = g.last_item_data.InFlags;

    // Process Init Request
    if (g.NavInitRequest && g.NavLayer == window.DC.NavLayerCurrent && (item_flags & ImGuiItemFlags_Disabled) == 0)
    {
        // Even if 'ImGuiItemFlags_NoNavDefaultFocus' is on (typically collapse/close button) we record the first ResultId so they can be used as a fallback
        const bool candidate_for_nav_default_focus = (item_flags & ImGuiItemFlags_NoNavDefaultFocus) == 0;
        if (candidate_for_nav_default_focus || g.NavInitResultId == 0)
        {
            g.NavInitResultId = id;
            g.NavInitResultRectRel = WindowRectAbsToRel(window, nav_bb);
        }
        if (candidate_for_nav_default_focus)
        {
            g.NavInitRequest = false; // Found a match, clear request
            NavUpdateAnyRequestFlag();
        }
    }

    // Process Move Request (scoring for navigation)
    // FIXME-NAV: Consider policy for double scoring (scoring from nav_scoring_rect + scoring from a rect wrapped according to current wrapping policy)
    if (g.NavMoveScoringItems)
    {
        const bool is_tab_stop = (item_flags & ImGuiItemFlags_Inputable) && (item_flags & (ImGuiItemFlags_NoTabStop | ImGuiItemFlags_Disabled)) == 0;
        const bool is_tabbing = (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) != 0;
        if (is_tabbing)
        {
            if (is_tab_stop || (g.NavMoveFlags & ImGuiNavMoveFlags_FocusApi))
                NavProcessItemForTabbingRequest(id);
        }
        else if ((g.NavId != id || (g.NavMoveFlags & ImGuiNavMoveFlags_AllowCurrentNavId)) && !(item_flags & (ImGuiItemFlags_Disabled | ImGuiItemFlags_NoNav)))
        {
            ImGuiNavItemData* result = (window == g.nav_window) ? &g.NavMoveResultLocal : &g.NavMoveResultOther;
            if (!is_tabbing)
            {
                if (NavScoreItem(result))
                    NavApplyItemToResult(result);

                // Features like PageUp/PageDown need to maintain a separate score for the visible set of items.
                const float VISIBLE_RATIO = 0.70;
                if ((g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet) && window.ClipRect.Overlaps(nav_bb))
                    if (ImClamp(nav_bb.Max.y, window.ClipRect.Min.y, window.ClipRect.Max.y) - ImClamp(nav_bb.Min.y, window.ClipRect.Min.y, window.ClipRect.Max.y) >= (nav_bb.Max.y - nav_bb.Min.y) * VISIBLE_RATIO)
                        if (NavScoreItem(&g.NavMoveResultLocalVisible))
                            NavApplyItemToResult(&g.NavMoveResultLocalVisible);
            }
        }
    }

    // Update window-relative bounding box of navigated item
    if (g.NavId == id)
    {
        if (g.nav_window != window)
            SetNavWindow(window); // Always refresh g.nav_window, because some operations such as FocusItem() may not have a window.
        g.NavLayer = window.DC.NavLayerCurrent;
        g.NavFocusScopeId = window.DC.NavFocusScopeIdCurrent;
        g.NavIdIsAlive = true;
        window.NavRectRel[window.DC.NavLayerCurrent] = WindowRectAbsToRel(window, nav_bb);    // Store item bounding box (relative to window position)
    }
}

// Handle "scoring" of an item for a tabbing/focusing request initiated by NavUpdateCreateTabbingRequest().
// Note that SetKeyboardFocusHere() API calls are considered tabbing requests!
// - Case 1: no nav/active id:    set result to first eligible item, stop storing.
// - Case 2: tab forward:         on ref id set counter, on counter elapse store result
// - Case 3: tab forward wrap:    set result to first eligible item (preemptively), on ref id set counter, on next frame if counter hasn't elapsed store result. // FIXME-TABBING: Could be done as a next-frame forwarded request
// - Case 4: tab backward:        store all results, on ref id pick prev, stop storing
// - Case 5: tab backward wrap:   store all results, on ref id if no result keep storing until last // FIXME-TABBING: Could be done as next-frame forwarded requested
void ImGui::NavProcessItemForTabbingRequest(ImGuiID id)
{
    ImGuiContext& g = *GImGui;

    // Always store in nav_move_result_local (unlike directional request which uses nav_move_result_other on sibling/flattened windows)
    ImGuiNavItemData* result = &g.NavMoveResultLocal;
    if (g.NavTabbingDir == +1)
    {
        // Tab Forward or SetKeyboardFocusHere() with >= 0
        if (g.NavTabbingResultFirst.ID == 0)
            NavApplyItemToResult(&g.NavTabbingResultFirst);
        if (g.NavTabbingCounter -= 1 == 0)
            NavMoveRequestResolveWithLastItem(result);
        else if (g.NavId == id)
            g.NavTabbingCounter = 1;
    }
    else if (g.NavTabbingDir == -1)
    {
        // Tab Backward
        if (g.NavId == id)
        {
            if (result->ID)
            {
                g.NavMoveScoringItems = false;
                NavUpdateAnyRequestFlag();
            }
        }
        else
        {
            NavApplyItemToResult(result);
        }
    }
    else if (g.NavTabbingDir == 0)
    {
        // Tab Init
        if (g.NavTabbingResultFirst.ID == 0)
            NavMoveRequestResolveWithLastItem(&g.NavTabbingResultFirst);
    }
}

bool ImGui::NavMoveRequestButNoResultYet()
{
    ImGuiContext& g = *GImGui;
    return g.NavMoveScoringItems && g.NavMoveResultLocal.ID == 0 && g.NavMoveResultOther.ID == 0;
}

// FIXME: ScoringRect is not set
void ImGui::NavMoveRequestSubmit(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.nav_window != NULL);

    if (move_flags & ImGuiNavMoveFlags_Tabbing)
        move_flags |= ImGuiNavMoveFlags_AllowCurrentNavId;

    g.NavMoveSubmitted = g.NavMoveScoringItems = true;
    g.NavMoveDir = move_dir;
    g.NavMoveDirForDebug = move_dir;
    g.NavMoveClipDir = clip_dir;
    g.NavMoveFlags = move_flags;
    g.NavMoveScrollFlags = scroll_flags;
    g.NavMoveForwardToNextFrame = false;
    g.NavMoveKeyMods = g.io.KeyMods;
    g.NavMoveResultLocal.Clear();
    g.NavMoveResultLocalVisible.Clear();
    g.NavMoveResultOther.Clear();
    g.NavTabbingCounter = 0;
    g.NavTabbingResultFirst.Clear();
    NavUpdateAnyRequestFlag();
}

void ImGui::NavMoveRequestResolveWithLastItem(ImGuiNavItemData* result)
{
    ImGuiContext& g = *GImGui;
    g.NavMoveScoringItems = false; // Ensure request doesn't need more processing
    NavApplyItemToResult(result);
    NavUpdateAnyRequestFlag();
}

void ImGui::NavMoveRequestCancel()
{
    ImGuiContext& g = *GImGui;
    g.NavMoveSubmitted = g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

// Forward will reuse the move request again on the next frame (generally with modifications done to it)
void ImGui::NavMoveRequestForward(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.NavMoveForwardToNextFrame == false);
    NavMoveRequestCancel();
    g.NavMoveForwardToNextFrame = true;
    g.NavMoveDir = move_dir;
    g.NavMoveClipDir = clip_dir;
    g.NavMoveFlags = move_flags | ImGuiNavMoveFlags_Forwarded;
    g.NavMoveScrollFlags = scroll_flags;
}

// Navigation wrap-around logic is delayed to the end of the frame because this operation is only valid after entire
// popup is assembled and in case of appended popups it is not clear which EndPopup() call is final.
void ImGui::NavMoveRequestTryWrapping(ImGuiWindow* window, ImGuiNavMoveFlags wrap_flags)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(wrap_flags != 0); // Call with _WrapX, _WrapY, _LoopX, _LoopY
    // In theory we should test for NavMoveRequestButNoResultYet() but there's no point doing it, NavEndFrame() will do the same test
    if (g.nav_window == window && g.NavMoveScoringItems && g.NavLayer == ImGuiNavLayer_Main)
        g.NavMoveFlags |= wrap_flags;
}

// FIXME: This could be replaced by updating a frame number in each window when (window == nav_window) and (nav_layer == 0).
// This way we could find the last focused window among our children. It would be much less confusing this way?
static void ImGui::NavSaveLastChildNavWindowIntoParent(ImGuiWindow* nav_window)
{
    ImGuiWindow* parent = nav_window;
    while (parent && parent->RootWindow != parent && (parent.flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_ChildMenu)) == 0)
        parent = parent->ParentWindow;
    if (parent && parent != nav_window)
        parent->NavLastChildNavWindow = nav_window;
}

// Restore the last focused child.
// Call when we are expected to land on the Main Layer (0) after focus_window()
static ImGuiWindow* ImGui::NavRestoreLastChildNavWindow(ImGuiWindow* window)
{
    if (window.NavLastChildNavWindow && window.NavLastChildNavWindow->WasActive)
        return window.NavLastChildNavWindow;
    if (window.DockNodeAsHost && window.DockNodeAsHost->TabBar)
        if (ImGuiTabItem* tab = TabBarFindMostRecentlySelectedTabForActiveWindow(window.DockNodeAsHost->TabBar))
            return tab->Window;
    return window;
}

void ImGui::NavRestoreLayer(ImGuiNavLayer layer)
{
    ImGuiContext& g = *GImGui;
    if (layer == ImGuiNavLayer_Main)
    {
        ImGuiWindow* prev_nav_window = g.nav_window;
        g.nav_window = NavRestoreLastChildNavWindow(g.nav_window);    // FIXME-NAV: Should clear ongoing nav requests?
        if (prev_nav_window)
            IMGUI_DEBUG_LOG_FOCUS("[focus] NavRestoreLayer: from \"%s\" to SetNavWindow(\"%s\")\n", prev_nav_window.Name, g.nav_window->Name);
    }
    ImGuiWindow* window = g.nav_window;
    if (window.NavLastIds[layer] != 0)
    {
        SetNavID(window.NavLastIds[layer], layer, 0, window.NavRectRel[layer]);
    }
    else
    {
        g.NavLayer = layer;
        NavInitWindow(window, true);
    }
}

void ImGui::NavRestoreHighlightAfterMove()
{
    ImGuiContext& g = *GImGui;
    g.NavDisableHighlight = false;
    g.NavDisableMouseHover = g.NavMousePosDirty = true;
}

static inline void ImGui::NavUpdateAnyRequestFlag()
{
    ImGuiContext& g = *GImGui;
    g.NavAnyRequest = g.NavMoveScoringItems || g.NavInitRequest || (IMGUI_DEBUG_NAV_SCORING && g.nav_window != NULL);
    if (g.NavAnyRequest)
        IM_ASSERT(g.nav_window != NULL);
}

// This needs to be called before we submit any widget (aka in or before Begin)
void ImGui::NavInitWindow(ImGuiWindow* window, bool force_reinit)
{
    // FIXME: ChildWindow test here is wrong for docking
    ImGuiContext& g = *GImGui;
    IM_ASSERT(window == g.nav_window);

    if (window.Flags & ImGuiWindowFlags_NoNavInputs)
    {
        g.NavId = g.NavFocusScopeId = 0;
        return;
    }

    bool init_for_nav = false;
    if (window == window.RootWindow || (window.Flags & ImGuiWindowFlags_Popup) || (window.NavLastIds[0] == 0) || force_reinit)
        init_for_nav = true;
    IMGUI_DEBUG_LOG_NAV("[nav] nav_init_request: from NavInitWindow(), init_for_nav=%d, window=\"%s\", layer=%d\n", init_for_nav, window.Name, g.NavLayer);
    if (init_for_nav)
    {
        SetNavID(0, g.NavLayer, 0, ImRect());
        g.NavInitRequest = true;
        g.NavInitRequestFromMove = false;
        g.NavInitResultId = 0;
        g.NavInitResultRectRel = ImRect();
        NavUpdateAnyRequestFlag();
    }
    else
    {
        g.NavId = window.NavLastIds[0];
        g.NavFocusScopeId = 0;
    }
}

static Vector2D ImGui::NavCalcPreferredRefPos()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;
    if (g.NavDisableHighlight || !g.NavDisableMouseHover || !window)
    {
        // Mouse (we need a fallback in case the mouse becomes invalid after being used)
        // The +1.0 offset when stored by OpenPopupEx() allows reopening this or another popup (same or another mouse button) while not moving the mouse, it is pretty standard.
        // In theory we could move that +1.0 offset in OpenPopupEx()
        Vector2D p = is_mouse_pos_valid(&g.io.MousePos) ? g.io.MousePos : g.MouseLastValidPos;
        return DimgVec2D::new(p.x + 1.0, p.y);
    }
    else
    {
        // When navigation is active and mouse is disabled, pick a position around the bottom left of the currently navigated item
        // Take account of upcoming scrolling (maybe set mouse pos should be done in EndFrame?)
        ImRect rect_rel = WindowRectRelToAbs(window, window.NavRectRel[g.NavLayer]);
        if (window.LastFrameActive != g.FrameCount && (window.ScrollTarget.x != FLT_MAX || window.ScrollTarget.y != FLT_MAX))
        {
            Vector2D next_scroll = CalcNextScrollFromScrollTargetAndClamp(window);
            rect_rel.Translate(window.Scroll - next_scroll);
        }
        Vector2D pos = DimgVec2D::new(rect_rel.Min.x + ImMin(g.Style.FramePadding.x * 4, rect_rel.GetWidth()), rect_rel.Max.y - ImMin(g.Style.FramePadding.y, rect_rel.GetHeight()));
        ImGuiViewport* viewport = window.viewport;
        return ImFloor(ImClamp(pos, viewport.pos, viewport.pos + viewport->Size)); // ImFloor() is important because non-integer mouse position application in backend might be lossy and result in undesirable non-zero delta.
    }
}

const char* ImGui::GetNavInputName(ImGuiNavInput n)
{
    static const char* names[] =
    {
        "Activate", "Cancel", "Input", "Menu", "DpadLeft", "DpadRight", "DpadUp", "DpadDown", "LStickLeft", "LStickRight", "LStickUp", "LStickDown",
        "FocusPrev", "FocusNext", "TweakSlow", "TweakFast", "KeyLeft", "KeyRight", "KeyUp", "KeyDown"
    };
    IM_ASSERT(IM_ARRAYSIZE(names) == ImGuiNavInput_COUNT);
    IM_ASSERT(n >= 0 && n < ImGuiNavInput_COUNT);
    return names[n];
}

float ImGui::GetNavInputAmount(ImGuiNavInput n, ImGuiNavReadMode mode)
{
    ImGuiContext& g = *GImGui;
    if (mode == ImGuiNavReadMode_Down)
        return g.io.NavInputs[n];                         // Instant, read analog input (0.0..1.0, as provided by user)

    const float t = g.io.NavInputsDownDuration[n];
    if (t < 0.0 && mode == ImGuiNavReadMode_Released)  // Return 1.0 when just released, no repeat, ignore analog input.
        return (g.io.NavInputsDownDurationPrev[n] >= 0.0 ? 1.0 : 0.0);
    if (t < 0.0)
        return 0.0;
    if (mode == ImGuiNavReadMode_Pressed)               // Return 1.0 when just pressed, no repeat, ignore analog input.
        return (t == 0.0) ? 1.0 : 0.0;
    if (mode == ImGuiNavReadMode_Repeat)
        return (float)CalcTypematicRepeatAmount(t - g.io.DeltaTime, t, g.io.KeyRepeatDelay * 0.72, g.io.KeyRepeatRate * 0.80);
    if (mode == ImGuiNavReadMode_RepeatSlow)
        return (float)CalcTypematicRepeatAmount(t - g.io.DeltaTime, t, g.io.KeyRepeatDelay * 1.25, g.io.KeyRepeatRate * 2.00);
    if (mode == ImGuiNavReadMode_RepeatFast)
        return (float)CalcTypematicRepeatAmount(t - g.io.DeltaTime, t, g.io.KeyRepeatDelay * 0.72, g.io.KeyRepeatRate * 0.30);
    return 0.0;
}

Vector2D ImGui::GetNavInputAmount2d(ImGuiNavDirSourceFlags dir_sources, ImGuiNavReadMode mode, float slow_factor, float fast_factor)
{
    Vector2D delta(0.0, 0.0);
    if (dir_sources & ImGuiNavDirSourceFlags_RawKeyboard)
        delta += DimgVec2D::new((float)IsKeyDown(ImGuiKey_RightArrow) - (float)IsKeyDown(ImGuiKey_LeftArrow), (float)IsKeyDown(ImGuiKey_DownArrow) - (float)IsKeyDown(ImGuiKey_UpArrow));
    if (dir_sources & ImGuiNavDirSourceFlags_Keyboard)
        delta += DimgVec2D::new(GetNavInputAmount(ImGuiNavInput_KeyRight_, mode)   - GetNavInputAmount(ImGuiNavInput_KeyLeft_,   mode), GetNavInputAmount(ImGuiNavInput_KeyDown_,   mode) - GetNavInputAmount(ImGuiNavInput_KeyUp_,   mode));
    if (dir_sources & ImGuiNavDirSourceFlags_PadDPad)
        delta += DimgVec2D::new(GetNavInputAmount(ImGuiNavInput_DpadRight, mode)   - GetNavInputAmount(ImGuiNavInput_DpadLeft,   mode), GetNavInputAmount(ImGuiNavInput_DpadDown,   mode) - GetNavInputAmount(ImGuiNavInput_DpadUp,   mode));
    if (dir_sources & ImGuiNavDirSourceFlags_PadLStick)
        delta += DimgVec2D::new(GetNavInputAmount(ImGuiNavInput_LStickRight, mode) - GetNavInputAmount(ImGuiNavInput_LStickLeft, mode), GetNavInputAmount(ImGuiNavInput_LStickDown, mode) - GetNavInputAmount(ImGuiNavInput_LStickUp, mode));
    if (slow_factor != 0.0 && IsNavInputDown(ImGuiNavInput_TweakSlow))
        delta *= slow_factor;
    if (fast_factor != 0.0 && IsNavInputDown(ImGuiNavInput_TweakFast))
        delta *= fast_factor;
    return delta;
}

static void ImGui::NavUpdate()
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;

    io.WantSetMousePos = false;
    //if (g.nav_scoring_debug_count > 0) IMGUI_DEBUG_LOG_NAV("[nav] nav_scoring_debug_count %d for '%s' layer %d (Init:%d, Move:%d)\n", g.nav_scoring_debug_count, g.nav_window ? g.nav_window->name : "NULL", g.nav_layer, g.nav_init_request || g.nav_init_result_id != 0, g.NavMoveRequest);

    // Update Gamepad->Nav inputs mapping
    // Set input source as Gamepad when buttons are pressed (as some features differs when used with Gamepad vs Keyboard)
    const bool nav_gamepad_active = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    if (nav_gamepad_active && g.io.BackendUsingLegacyNavInputArray == false)
    {
        for (int n = 0; n < ImGuiNavInput_COUNT; n += 1)
            IM_ASSERT(io.NavInputs[n] == 0.0 && "Backend needs to either only use io.add_key_event()/io.add_key_analog_event(), either only fill legacy io.nav_inputs[]. Not both!");
        #define NAV_MAP_KEY(_KEY, _NAV_INPUT, _ACTIVATE_NAV)  do { io.NavInputs[_NAV_INPUT] = io.KeysData[_KEY - ImGuiKey_KeysData_OFFSET].AnalogValue; if (_ACTIVATE_NAV && io.NavInputs[_NAV_INPUT] > 0.0) { g.NavInputSource = ImGuiInputSource_Gamepad; } } while (0)
        NAV_MAP_KEY(ImGuiKey_GamepadFaceDown, ImGuiNavInput_Activate, true);
        NAV_MAP_KEY(ImGuiKey_GamepadFaceRight, ImGuiNavInput_Cancel, true);
        NAV_MAP_KEY(ImGuiKey_GamepadFaceLeft, ImGuiNavInput_Menu, true);
        NAV_MAP_KEY(ImGuiKey_GamepadFaceUp, ImGuiNavInput_Input, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadLeft, ImGuiNavInput_DpadLeft, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadRight, ImGuiNavInput_DpadRight, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadUp, ImGuiNavInput_DpadUp, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadDown, ImGuiNavInput_DpadDown, true);
        NAV_MAP_KEY(ImGuiKey_GamepadL1, ImGuiNavInput_FocusPrev, false);
        NAV_MAP_KEY(ImGuiKey_GamepadR1, ImGuiNavInput_FocusNext, false);
        NAV_MAP_KEY(ImGuiKey_GamepadL1, ImGuiNavInput_TweakSlow, false);
        NAV_MAP_KEY(ImGuiKey_GamepadR1, ImGuiNavInput_TweakFast, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickLeft, ImGuiNavInput_LStickLeft, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickRight, ImGuiNavInput_LStickRight, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickUp, ImGuiNavInput_LStickUp, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickDown, ImGuiNavInput_LStickDown, false);
        #undef NAV_MAP_KEY
    }

    // Update Keyboard->Nav inputs mapping
    const bool nav_keyboard_active = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;
    if (nav_keyboard_active)
    {
        #define NAV_MAP_KEY(_KEY, _NAV_INPUT)  do { if (IsKeyDown(_KEY)) { io.NavInputs[_NAV_INPUT] = 1.0; g.NavInputSource = ImGuiInputSource_Keyboard; } } while (0)
        NAV_MAP_KEY(ImGuiKey_Space,     ImGuiNavInput_Activate );
        NAV_MAP_KEY(ImGuiKey_Enter,     ImGuiNavInput_Input    );
        NAV_MAP_KEY(ImGuiKey_Escape,    ImGuiNavInput_Cancel   );
        NAV_MAP_KEY(ImGuiKey_LeftArrow, ImGuiNavInput_KeyLeft_ );
        NAV_MAP_KEY(ImGuiKey_RightArrow,ImGuiNavInput_KeyRight_);
        NAV_MAP_KEY(ImGuiKey_UpArrow,   ImGuiNavInput_KeyUp_   );
        NAV_MAP_KEY(ImGuiKey_DownArrow, ImGuiNavInput_KeyDown_ );
        if (io.KeyCtrl)
            io.NavInputs[ImGuiNavInput_TweakSlow] = 1.0;
        if (io.KeyShift)
            io.NavInputs[ImGuiNavInput_TweakFast] = 1.0;
        #undef NAV_MAP_KEY
    }
    memcpy(io.NavInputsDownDurationPrev, io.NavInputsDownDuration, sizeof(io.NavInputsDownDuration));
    for (int i = 0; i < IM_ARRAYSIZE(io.NavInputs); i += 1)
        io.NavInputsDownDuration[i] = (io.NavInputs[i] > 0.0) ? (io.NavInputsDownDuration[i] < 0.0 ? 0.0 : io.NavInputsDownDuration[i] + io.DeltaTime) : -1.0;

    // Process navigation init request (select first/default focus)
    if (g.NavInitResultId != 0)
        NavInitRequestApplyResult();
    g.NavInitRequest = false;
    g.NavInitRequestFromMove = false;
    g.NavInitResultId = 0;
    g.NavJustMovedToId = 0;

    // Process navigation move request
    if (g.NavMoveSubmitted)
        NavMoveRequestApplyResult();
    g.NavTabbingCounter = 0;
    g.NavMoveSubmitted = g.NavMoveScoringItems = false;

    // Schedule mouse position update (will be done at the bottom of this function, after 1) processing all move requests and 2) updating scrolling)
    bool set_mouse_pos = false;
    if (g.NavMousePosDirty && g.NavIdIsAlive)
        if (!g.NavDisableHighlight && g.NavDisableMouseHover && g.nav_window)
            set_mouse_pos = true;
    g.NavMousePosDirty = false;
    IM_ASSERT(g.NavLayer == ImGuiNavLayer_Main || g.NavLayer == ImGuiNavLayer_Menu);

    // Store our return window (for returning from Menu Layer to Main Layer) and clear it as soon as we step back in our own Layer 0
    if (g.nav_window)
        NavSaveLastChildNavWindowIntoParent(g.nav_window);
    if (g.nav_window && g.nav_window->NavLastChildNavWindow != NULL && g.NavLayer == ImGuiNavLayer_Main)
        g.nav_window->NavLastChildNavWindow = NULL;

    // Update CTRL+TAB and Windowing features (hold Square to move/resize/etc.)
    NavUpdateWindowing();

    // Set output flags for user application
    io.NavActive = (nav_keyboard_active || nav_gamepad_active) && g.nav_window && !(g.nav_window.flags & ImGuiWindowFlags_NoNavInputs);
    io.NavVisible = (io.NavActive && g.NavId != 0 && !g.NavDisableHighlight) || (g.NavWindowingTarget != NULL);

    // Process NavCancel input (to close a popup, get back to parent, clear focus)
    NavUpdateCancelRequest();

    // Process manual activation request
    g.NavActivateId = g.NavActivateDownId = g.NavActivatePressedId = g.NavActivateInputId = 0;
    g.NavActivateFlags = ImGuiActivateFlags_None;
    if (g.NavId != 0 && !g.NavDisableHighlight && !g.NavWindowingTarget && g.nav_window && !(g.nav_window.flags & ImGuiWindowFlags_NoNavInputs))
    {
        bool activate_down = IsNavInputDown(ImGuiNavInput_Activate);
        bool input_down = IsNavInputDown(ImGuiNavInput_Input);
        bool activate_pressed = activate_down && IsNavInputTest(ImGuiNavInput_Activate, ImGuiNavReadMode_Pressed);
        bool input_pressed = input_down && IsNavInputTest(ImGuiNavInput_Input, ImGuiNavReadMode_Pressed);
        if (g.active_id == 0 && activate_pressed)
        {
            g.NavActivateId = g.NavId;
            g.NavActivateFlags = ImGuiActivateFlags_PreferTweak;
        }
        if ((g.active_id == 0 || g.active_id == g.NavId) && input_pressed)
        {
            g.NavActivateInputId = g.NavId;
            g.NavActivateFlags = ImGuiActivateFlags_PreferInput;
        }
        if ((g.active_id == 0 || g.active_id == g.NavId) && activate_down)
            g.NavActivateDownId = g.NavId;
        if ((g.active_id == 0 || g.active_id == g.NavId) && activate_pressed)
            g.NavActivatePressedId = g.NavId;
    }
    if (g.nav_window && (g.nav_window.flags & ImGuiWindowFlags_NoNavInputs))
        g.NavDisableHighlight = true;
    if (g.NavActivateId != 0)
        IM_ASSERT(g.NavActivateDownId == g.NavActivateId);

    // Process programmatic activation request
    // FIXME-NAV: Those should eventually be queued (unlike focus they don't cancel each others)
    if (g.NavNextActivateId != 0)
    {
        if (g.NavNextActivateFlags & ImGuiActivateFlags_PreferInput)
            g.NavActivateInputId = g.NavNextActivateId;
        else
            g.NavActivateId = g.NavActivateDownId = g.NavActivatePressedId = g.NavNextActivateId;
        g.NavActivateFlags = g.NavNextActivateFlags;
    }
    g.NavNextActivateId = 0;

    // Process move requests
    NavUpdateCreateMoveRequest();
    if (g.NavMoveDir == ImGuiDir_None)
        NavUpdateCreateTabbingRequest();
    NavUpdateAnyRequestFlag();
    g.NavIdIsAlive = false;

    // Scrolling
    if (g.nav_window && !(g.nav_window.flags & ImGuiWindowFlags_NoNavInputs) && !g.NavWindowingTarget)
    {
        // *Fallback* manual-scroll with Nav directional keys when window has no navigable item
        ImGuiWindow* window = g.nav_window;
        const float scroll_speed = IM_ROUND(window.CalcFontSize() * 100 * io.DeltaTime); // We need round the scrolling speed because sub-pixel scroll isn't reliably supported.
        const ImGuiDir move_dir = g.NavMoveDir;
        if (window.DC.NavLayersActiveMask == 0x00 && window.DC.NavHasScroll && move_dir != ImGuiDir_None)
        {
            if (move_dir == ImGuiDir_Left || move_dir == ImGuiDir_Right)
                SetScrollX(window, ImFloor(window.Scroll.x + ((move_dir == ImGuiDir_Left) ? -1.0 : +1.0) * scroll_speed));
            if (move_dir == ImGuiDir_Up || move_dir == ImGuiDir_Down)
                SetScrollY(window, ImFloor(window.Scroll.y + ((move_dir == ImGuiDir_Up) ? -1.0 : +1.0) * scroll_speed));
        }

        // *Normal* Manual scroll with NavScrollXXX keys
        // Next movement request will clamp the nav_id reference rectangle to the visible area, so navigation will resume within those bounds.
        Vector2D scroll_dir = GetNavInputAmount2d(ImGuiNavDirSourceFlags_PadLStick, ImGuiNavReadMode_Down, 1.0 / 10.0, 10.0);
        if (scroll_dir.x != 0.0 && window.ScrollbarX)
            SetScrollX(window, ImFloor(window.Scroll.x + scroll_dir.x * scroll_speed));
        if (scroll_dir.y != 0.0)
            SetScrollY(window, ImFloor(window.Scroll.y + scroll_dir.y * scroll_speed));
    }

    // Always prioritize mouse highlight if navigation is disabled
    if (!nav_keyboard_active && !nav_gamepad_active)
    {
        g.NavDisableHighlight = true;
        g.NavDisableMouseHover = set_mouse_pos = false;
    }

    // Update mouse position if requested
    // (This will take into account the possibility that a scroll was queued in the window to offset our absolute mouse position before scroll has been applied)
    if (set_mouse_pos && (io.ConfigFlags & ImGuiConfigFlags_NavEnableSetMousePos) && (io.BackendFlags & ImGuiBackendFlags_HasSetMousePos))
    {
        io.MousePos = io.MousePosPrev = NavCalcPreferredRefPos();
        io.WantSetMousePos = true;
        //IMGUI_DEBUG_LOG_IO("SetMousePos: (%.1,%.1)\n", io.mouse_pos.x, io.mouse_pos.y);
    }

    // [DEBUG]
    g.NavScoringDebugCount = 0;
#if IMGUI_DEBUG_NAV_RECTS
    if (g.nav_window)
    {
        ImDrawList* draw_list = GetForegroundDrawList(g.nav_window);
        if (1) { for (int layer = 0; layer < 2; layer += 1) { ImRect r = WindowRectRelToAbs(g.nav_window, g.nav_window->NavRectRel[layer]); draw_list->AddRect(r.Min, r.Max, IM_COL32(255,200,0,255)); } } // [DEBUG]
        if (1) { ImU32 col = (!g.nav_window->Hidden) ? IM_COL32(255,0,255,255) : IM_COL32(255,0,0,255); Vector2D p = NavCalcPreferredRefPos(); char buf[32]; ImFormatString(buf, 32, "%d", g.NavLayer); draw_list->AddCircleFilled(p, 3.0, col); draw_list->AddText(NULL, 13.0, p + DimgVec2D::new(8,-4), col, buf); }
    }

}

void ImGui::NavInitRequestApplyResult()
{
    // In very rare cases g.nav_window may be null (e.g. clearing focus after requesting an init request, which does happen when releasing Alt while clicking on void)
    ImGuiContext& g = *GImGui;
    if (!g.nav_window)
        return;

    // Apply result from previous navigation init request (will typically select the first item, unless SetItemDefaultFocus() has been called)
    // FIXME-NAV: On _NavFlattened windows, g.nav_window will only be updated during subsequent frame. Not a problem currently.
    IMGUI_DEBUG_LOG_NAV("[nav] nav_init_request: ApplyResult: NavID 0x%08X in Layer %d Window \"%s\"\n", g.NavInitResultId, g.NavLayer, g.nav_window->Name);
    SetNavID(g.NavInitResultId, g.NavLayer, 0, g.NavInitResultRectRel);
    g.NavIdIsAlive = true; // Mark as alive from previous frame as we got a result
    if (g.NavInitRequestFromMove)
        NavRestoreHighlightAfterMove();
}

void ImGui::NavUpdateCreateMoveRequest()
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;
    ImGuiWindow* window = g.nav_window;

    if (g.NavMoveForwardToNextFrame && window != NULL)
    {
        // Forwarding previous request (which has been modified, e.g. wrap around menus rewrite the requests with a starting rectangle at the other side of the window)
        // (preserve most state, which were already set by the NavMoveRequestForward() function)
        IM_ASSERT(g.NavMoveDir != ImGuiDir_None && g.NavMoveClipDir != ImGuiDir_None);
        IM_ASSERT(g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded);
        IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequestForward %d\n", g.NavMoveDir);
    }
    else
    {
        // Initiate directional inputs request
        g.NavMoveDir = ImGuiDir_None;
        g.NavMoveFlags = ImGuiNavMoveFlags_None;
        g.NavMoveScrollFlags = ImGuiScrollFlags_None;
        if (window && !g.NavWindowingTarget && !(window.Flags & ImGuiWindowFlags_NoNavInputs))
        {
            const ImGuiNavReadMode read_mode = ImGuiNavReadMode_Repeat;
            if (!IsActiveIdUsingNavDir(ImGuiDir_Left)  && (IsNavInputTest(ImGuiNavInput_DpadLeft,  read_mode) || IsNavInputTest(ImGuiNavInput_KeyLeft_,  read_mode))) { g.NavMoveDir = ImGuiDir_Left; }
            if (!IsActiveIdUsingNavDir(ImGuiDir_Right) && (IsNavInputTest(ImGuiNavInput_DpadRight, read_mode) || IsNavInputTest(ImGuiNavInput_KeyRight_, read_mode))) { g.NavMoveDir = ImGuiDir_Right; }
            if (!IsActiveIdUsingNavDir(ImGuiDir_Up)    && (IsNavInputTest(ImGuiNavInput_DpadUp,    read_mode) || IsNavInputTest(ImGuiNavInput_KeyUp_,    read_mode))) { g.NavMoveDir = ImGuiDir_Up; }
            if (!IsActiveIdUsingNavDir(ImGuiDir_Down)  && (IsNavInputTest(ImGuiNavInput_DpadDown,  read_mode) || IsNavInputTest(ImGuiNavInput_KeyDown_,  read_mode))) { g.NavMoveDir = ImGuiDir_Down; }
        }
        g.NavMoveClipDir = g.NavMoveDir;
        g.NavScoringNoClipRect = ImRect(+FLT_MAX, +FLT_MAX, -FLT_MAX, -FLT_MAX);
    }

    // Update PageUp/PageDown/Home/End scroll
    // FIXME-NAV: Consider enabling those keys even without the master ImGuiConfigFlags_NavEnableKeyboard flag?
    const bool nav_keyboard_active = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;
    float scoring_rect_offset_y = 0.0;
    if (window && g.NavMoveDir == ImGuiDir_None && nav_keyboard_active)
        scoring_rect_offset_y = NavUpdatePageUpPageDown();
    if (scoring_rect_offset_y != 0.0)
    {
        g.NavScoringNoClipRect = window.InnerRect;
        g.NavScoringNoClipRect.TranslateY(scoring_rect_offset_y);
    }

    // [DEBUG] Always send a request
#if IMGUI_DEBUG_NAV_SCORING
    if (io.KeyCtrl && IsKeyPressed(ImGuiKey_C))
        g.NavMoveDirForDebug = (ImGuiDir)((g.NavMoveDirForDebug + 1) & 3);
    if (io.KeyCtrl && g.NavMoveDir == ImGuiDir_None)
    {
        g.NavMoveDir = g.NavMoveDirForDebug;
        g.NavMoveFlags |= ImGuiNavMoveFlags_DebugNoResult;
    }


    // Submit
    g.NavMoveForwardToNextFrame = false;
    if (g.NavMoveDir != ImGuiDir_None)
        NavMoveRequestSubmit(g.NavMoveDir, g.NavMoveClipDir, g.NavMoveFlags, g.NavMoveScrollFlags);

    // Moving with no reference triggers a init request (will be used as a fallback if the direction fails to find a match)
    if (g.NavMoveSubmitted && g.NavId == 0)
    {
        IMGUI_DEBUG_LOG_NAV("[nav] nav_init_request: from move, window \"%s\", layer=%d\n", window ? window.Name : "<NULL>", g.NavLayer);
        g.NavInitRequest = g.NavInitRequestFromMove = true;
        g.NavInitResultId = 0;
        g.NavDisableHighlight = false;
    }

    // When using gamepad, we project the reference nav bounding box into window visible area.
    // This is to allow resuming navigation inside the visible area after doing a large amount of scrolling, since with gamepad every movements are relative
    // (can't focus a visible object like we can with the mouse).
    if (g.NavMoveSubmitted && g.NavInputSource == ImGuiInputSource_Gamepad && g.NavLayer == ImGuiNavLayer_Main && window != NULL)// && (g.nav_move_flags & ImGuiNavMoveFlags_Forwarded))
    {
        bool clamp_x = (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapX)) == 0;
        bool clamp_y = (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopY | ImGuiNavMoveFlags_WrapY)) == 0;
        ImRect inner_rect_rel = WindowRectAbsToRel(window, ImRect(window.InnerRect.Min - DimgVec2D::new(1, 1), window.InnerRect.Max + DimgVec2D::new(1, 1)));
        if ((clamp_x || clamp_y) && !inner_rect_rel.Contains(window.NavRectRel[g.NavLayer]))
        {
            //IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: clamp nav_rect_rel for gamepad move\n");
            float pad_x = ImMin(inner_rect_rel.GetWidth(), window.CalcFontSize() * 0.5);
            float pad_y = ImMin(inner_rect_rel.GetHeight(), window.CalcFontSize() * 0.5); // Terrible approximation for the intent of starting navigation from first fully visible item
            inner_rect_rel.Min.x = clamp_x ? (inner_rect_rel.Min.x + pad_x) : -FLT_MAX;
            inner_rect_rel.Max.x = clamp_x ? (inner_rect_rel.Max.x - pad_x) : +FLT_MAX;
            inner_rect_rel.Min.y = clamp_y ? (inner_rect_rel.Min.y + pad_y) : -FLT_MAX;
            inner_rect_rel.Max.y = clamp_y ? (inner_rect_rel.Max.y - pad_y) : +FLT_MAX;
            window.NavRectRel[g.NavLayer].ClipWithFull(inner_rect_rel);
            g.NavId = g.NavFocusScopeId = 0;
        }
    }

    // For scoring we use a single segment on the left side our current item bounding box (not touching the edge to avoid box overlap with zero-spaced items)
    ImRect scoring_rect;
    if (window != NULL)
    {
        ImRect nav_rect_rel = !window.NavRectRel[g.NavLayer].IsInverted() ? window.NavRectRel[g.NavLayer] : ImRect(0, 0, 0, 0);
        scoring_rect = WindowRectRelToAbs(window, nav_rect_rel);
        scoring_rect.TranslateY(scoring_rect_offset_y);
        scoring_rect.Min.x = ImMin(scoring_rect.Min.x + 1.0, scoring_rect.Max.x);
        scoring_rect.Max.x = scoring_rect.Min.x;
        IM_ASSERT(!scoring_rect.IsInverted()); // Ensure if we have a finite, non-inverted bounding box here will allows us to remove extraneous ImFabs() calls in NavScoreItem().
        //GetForegroundDrawList()->add_rect(scoring_rect.min, scoring_rect.max, IM_COL32(255,200,0,255)); // [DEBUG]
        //if (!g.nav_scoring_no_clip_rect.IsInverted()) { GetForegroundDrawList()->add_rect(g.nav_scoring_no_clip_rect.min, g.nav_scoring_no_clip_rect.max, IM_COL32(255, 200, 0, 255)); } // [DEBUG]
    }
    g.NavScoringRect = scoring_rect;
    g.NavScoringNoClipRect.Add(scoring_rect);
}

void ImGui::NavUpdateCreateTabbingRequest()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;
    IM_ASSERT(g.NavMoveDir == ImGuiDir_None);
    if (window == NULL || g.NavWindowingTarget != NULL || (window.Flags & ImGuiWindowFlags_NoNavInputs))
        return;

    const bool tab_pressed = IsKeyPressed(ImGuiKey_Tab, true) && !IsActiveIdUsingKey(ImGuiKey_Tab) && !g.io.KeyCtrl && !g.io.KeyAlt;
    if (!tab_pressed)
        return;

    // Initiate tabbing request
    // (this is ALWAYS ENABLED, regardless of ImGuiConfigFlags_NavEnableKeyboard flag!)
    // Initially this was designed to use counters and modulo arithmetic, but that could not work with unsubmitted items (list clipper). Instead we use a strategy close to other move requests.
    // See NavProcessItemForTabbingRequest() for a description of the various forward/backward tabbing cases with and without wrapping.
    //// FIXME: We use (g.active_id == 0) but (g.NavDisableHighlight == false) might be righter once we can tab through anything
    g.NavTabbingDir = g.io.KeyShift ? -1 : (g.active_id == 0) ? 0 : +1;
    ImGuiScrollFlags scroll_flags = window.Appearing ? ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleEdgeY;
    ImGuiDir clip_dir = (g.NavTabbingDir < 0) ? ImGuiDir_Up : ImGuiDir_Down;
    NavMoveRequestSubmit(ImGuiDir_None, clip_dir, ImGuiNavMoveFlags_Tabbing, scroll_flags); // FIXME-NAV: Once we refactor tabbing, add LegacyApi flag to not activate non-inputable.
    g.NavTabbingCounter = -1;
}

// Apply result from previous frame navigation directional move request. Always called from NavUpdate()
void ImGui::NavMoveRequestApplyResult()
{
    ImGuiContext& g = *GImGui;
#if IMGUI_DEBUG_NAV_SCORING
    if (g.NavMoveFlags & ImGuiNavMoveFlags_DebugNoResult) // [DEBUG] Scoring all items in nav_window at all times
        return;


    // Select which result to use
    ImGuiNavItemData* result = (g.NavMoveResultLocal.ID != 0) ? &g.NavMoveResultLocal : (g.NavMoveResultOther.ID != 0) ? &g.NavMoveResultOther : NULL;

    // Tabbing forward wrap
    if (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing)
        if ((g.NavTabbingCounter == 1 || g.NavTabbingDir == 0) && g.NavTabbingResultFirst.ID)
            result = &g.NavTabbingResultFirst;

    // In a situation when there is no results but nav_id != 0, re-enable the Navigation highlight (because g.nav_id is not considered as a possible result)
    if (result == NULL)
    {
        if (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing)
            g.NavMoveFlags |= ImGuiNavMoveFlags_DontSetNavHighlight;
        if (g.NavId != 0 && (g.NavMoveFlags & ImGuiNavMoveFlags_DontSetNavHighlight) == 0)
            NavRestoreHighlightAfterMove();
        return;
    }

    // PageUp/PageDown behavior first jumps to the bottom/top mostly visible item, _otherwise_ use the result from the previous/next page.
    if (g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet)
        if (g.NavMoveResultLocalVisible.ID != 0 && g.NavMoveResultLocalVisible.ID != g.NavId)
            result = &g.NavMoveResultLocalVisible;

    // Maybe entering a flattened child from the outside? In this case solve the tie using the regular scoring rules.
    if (result != &g.NavMoveResultOther && g.NavMoveResultOther.ID != 0 && g.NavMoveResultOther.Window->ParentWindow == g.nav_window)
        if ((g.NavMoveResultOther.DistBox < result->DistBox) || (g.NavMoveResultOther.DistBox == result->DistBox && g.NavMoveResultOther.DistCenter < result->DistCenter))
            result = &g.NavMoveResultOther;
    IM_ASSERT(g.nav_window && result->Window);

    // scroll to keep newly navigated item fully into view.
    if (g.NavLayer == ImGuiNavLayer_Main)
    {
        if (g.NavMoveFlags & ImGuiNavMoveFlags_ScrollToEdgeY)
        {
            // FIXME: Should remove this
            float scroll_target = (g.NavMoveDir == ImGuiDir_Up) ? result->Window->ScrollMax.y : 0.0;
            SetScrollY(result->Window, scroll_target);
        }
        else
        {
            ImRect rect_abs = WindowRectRelToAbs(result->Window, result->RectRel);
            ScrollToRectEx(result->Window, rect_abs, g.NavMoveScrollFlags);
        }
    }

    if (g.nav_window != result->Window)
    {
        IMGUI_DEBUG_LOG_FOCUS("[focus] NavMoveRequest: SetNavWindow(\"%s\")\n", result->Window->Name);
        g.nav_window = result->Window;
    }
    if (g.active_id != result->ID)
        clear_active_id();
    if (g.NavId != result->ID)
    {
        // Don't set nav_just_moved_to_id if just landed on the same spot (which may happen with ImGuiNavMoveFlags_AllowCurrentNavId)
        g.NavJustMovedToId = result->ID;
        g.NavJustMovedToFocusScopeId = result->FocusScopeId;
        g.NavJustMovedToKeyMods = g.NavMoveKeyMods;
    }

    // Focus
    IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: result NavID 0x%08X in Layer %d Window \"%s\"\n", result->ID, g.NavLayer, g.nav_window->Name);
    SetNavID(result->ID, g.NavLayer, result->FocusScopeId, result->RectRel);

    // Tabbing: Activates Inputable or Focus non-Inputable
    if ((g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) && (result->InFlags & ImGuiItemFlags_Inputable))
    {
        g.NavNextActivateId = result->ID;
        g.NavNextActivateFlags = ImGuiActivateFlags_PreferInput | ImGuiActivateFlags_TryToPreserveState;
        g.NavMoveFlags |= ImGuiNavMoveFlags_DontSetNavHighlight;
    }

    // Activate
    if (g.NavMoveFlags & ImGuiNavMoveFlags_Activate)
    {
        g.NavNextActivateId = result->ID;
        g.NavNextActivateFlags = ImGuiActivateFlags_None;
    }

    // Enable nav highlight
    if ((g.NavMoveFlags & ImGuiNavMoveFlags_DontSetNavHighlight) == 0)
        NavRestoreHighlightAfterMove();
}

// Process NavCancel input (to close a popup, get back to parent, clear focus)
// FIXME: In order to support e.g. Escape to clear a selection we'll need:
// - either to store the equivalent of active_id_using_key_input_mask for a FocusScope and test for it.
// - either to move most/all of those tests to the epilogue/end functions of the scope they are dealing with (e.g. exit child window in EndChild()) or in EndFrame(), to allow an earlier intercept
static void ImGui::NavUpdateCancelRequest()
{
    ImGuiContext& g = *GImGui;
    if (!IsNavInputTest(ImGuiNavInput_Cancel, ImGuiNavReadMode_Pressed))
        return;

    IMGUI_DEBUG_LOG_NAV("[nav] ImGuiNavInput_Cancel\n");
    if (g.active_id != 0)
    {
        if (!IsActiveIdUsingNavInput(ImGuiNavInput_Cancel))
            clear_active_id();
    }
    else if (g.NavLayer != ImGuiNavLayer_Main)
    {
        // Leave the "menu" layer
        NavRestoreLayer(ImGuiNavLayer_Main);
        NavRestoreHighlightAfterMove();
    }
    else if (g.nav_window && g.nav_window != g.nav_window->RootWindow && !(g.nav_window.flags & ImGuiWindowFlags_Popup) && g.nav_window->ParentWindow)
    {
        // Exit child window
        ImGuiWindow* child_window = g.nav_window;
        ImGuiWindow* parent_window = g.nav_window->ParentWindow;
        IM_ASSERT(child_window.ChildId != 0);
        ImRect child_rect = child_window.Rect();
        focus_window(parent_window);
        SetNavID(child_window.ChildId, ImGuiNavLayer_Main, 0, WindowRectAbsToRel(parent_window, child_rect));
        NavRestoreHighlightAfterMove();
    }
    else if (g.OpenPopupStack.Size > 0 && !(g.OpenPopupStack.back().Window.flags & ImGuiWindowFlags_Modal))
    {
        // Close open popup/menu
        ClosePopupToLevel(g.OpenPopupStack.Size - 1, true);
    }
    else
    {
        // clear NavLastId for popups but keep it for regular child window so we can leave one and come back where we were
        if (g.nav_window && ((g.nav_window.flags & ImGuiWindowFlags_Popup) || !(g.nav_window.flags & ImGuiWindowFlags_ChildWindow)))
            g.nav_window->NavLastIds[0] = 0;
        g.NavId = g.NavFocusScopeId = 0;
    }
}

// Handle PageUp/PageDown/Home/End keys
// Called from NavUpdateCreateMoveRequest() which will use our output to create a move request
// FIXME-NAV: This doesn't work properly with NavFlattened siblings as we use nav_window rectangle for reference
// FIXME-NAV: how to get Home/End to aim at the beginning/end of a 2D grid?
static float ImGui::NavUpdatePageUpPageDown()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;
    if ((window.Flags & ImGuiWindowFlags_NoNavInputs) || g.NavWindowingTarget != NULL)
        return 0.0;

    const bool page_up_held = IsKeyDown(ImGuiKey_PageUp) && !IsActiveIdUsingKey(ImGuiKey_PageUp);
    const bool page_down_held = IsKeyDown(ImGuiKey_PageDown) && !IsActiveIdUsingKey(ImGuiKey_PageDown);
    const bool home_pressed = IsKeyPressed(ImGuiKey_Home) && !IsActiveIdUsingKey(ImGuiKey_Home);
    const bool end_pressed = IsKeyPressed(ImGuiKey_End) && !IsActiveIdUsingKey(ImGuiKey_End);
    if (page_up_held == page_down_held && home_pressed == end_pressed) // Proceed if either (not both) are pressed, otherwise early out
        return 0.0;

    if (g.NavLayer != ImGuiNavLayer_Main)
        NavRestoreLayer(ImGuiNavLayer_Main);

    if (window.DC.NavLayersActiveMask == 0x00 && window.DC.NavHasScroll)
    {
        // Fallback manual-scroll when window has no navigable item
        if (IsKeyPressed(ImGuiKey_PageUp, true))
            SetScrollY(window, window.Scroll.y - window.InnerRect.GetHeight());
        else if (IsKeyPressed(ImGuiKey_PageDown, true))
            SetScrollY(window, window.Scroll.y + window.InnerRect.GetHeight());
        else if (home_pressed)
            SetScrollY(window, 0.0);
        else if (end_pressed)
            SetScrollY(window, window.ScrollMax.y);
    }
    else
    {
        ImRect& nav_rect_rel = window.NavRectRel[g.NavLayer];
        const float page_offset_y = ImMax(0.0, window.InnerRect.GetHeight() - window.CalcFontSize() * 1.0 + nav_rect_rel.GetHeight());
        float nav_scoring_rect_offset_y = 0.0;
        if (IsKeyPressed(ImGuiKey_PageUp, true))
        {
            nav_scoring_rect_offset_y = -page_offset_y;
            g.NavMoveDir = ImGuiDir_Down; // Because our scoring rect is offset up, we request the down direction (so we can always land on the last item)
            g.NavMoveClipDir = ImGuiDir_Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_AlsoScoreVisibleSet;
        }
        else if (IsKeyPressed(ImGuiKey_PageDown, true))
        {
            nav_scoring_rect_offset_y = +page_offset_y;
            g.NavMoveDir = ImGuiDir_Up; // Because our scoring rect is offset down, we request the up direction (so we can always land on the last item)
            g.NavMoveClipDir = ImGuiDir_Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_AlsoScoreVisibleSet;
        }
        else if (home_pressed)
        {
            // FIXME-NAV: handling of Home/End is assuming that the top/bottom most item will be visible with scroll.y == 0/scroll_max.y
            // Scrolling will be handled via the ImGuiNavMoveFlags_ScrollToEdgeY flag, we don't scroll immediately to avoid scrolling happening before nav result.
            // Preserve current horizontal position if we have any.
            nav_rect_rel.Min.y = nav_rect_rel.Max.y = 0.0;
            if (nav_rect_rel.IsInverted())
                nav_rect_rel.Min.x = nav_rect_rel.Max.x = 0.0;
            g.NavMoveDir = ImGuiDir_Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        else if (end_pressed)
        {
            nav_rect_rel.Min.y = nav_rect_rel.Max.y = window.ContentSize.y;
            if (nav_rect_rel.IsInverted())
                nav_rect_rel.Min.x = nav_rect_rel.Max.x = 0.0;
            g.NavMoveDir = ImGuiDir_Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        return nav_scoring_rect_offset_y;
    }
    return 0.0;
}

static void ImGui::NavEndFrame()
{
    ImGuiContext& g = *GImGui;

    // Show CTRL+TAB list window
    if (g.NavWindowingTarget != NULL)
        NavUpdateWindowingOverlay();

    // Perform wrap-around in menus
    // FIXME-NAV: Wrap may need to apply a weight bias on the other axis. e.g. 4x4 grid with 2 last items missing on last item won't handle LoopY/WrapY correctly.
    // FIXME-NAV: Wrap (not Loop) support could be handled by the scoring function and then WrapX would function without an extra frame.
    const ImGuiNavMoveFlags wanted_flags = ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY;
    if (g.nav_window && NavMoveRequestButNoResultYet() && (g.NavMoveFlags & wanted_flags) && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded) == 0)
        NavUpdateCreateWrappingRequest();
}

static void ImGui::NavUpdateCreateWrappingRequest()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;

    bool do_forward = false;
    ImRect bb_rel = window.NavRectRel[g.NavLayer];
    ImGuiDir clip_dir = g.NavMoveDir;
    const ImGuiNavMoveFlags move_flags = g.NavMoveFlags;
    if (g.NavMoveDir == ImGuiDir_Left && (move_flags & (ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX)))
    {
        bb_rel.Min.x = bb_rel.Max.x = window.ContentSize.x + window.WindowPadding.x;
        if (move_flags & ImGuiNavMoveFlags_WrapX)
        {
            bb_rel.TranslateY(-bb_rel.GetHeight()); // Previous row
            clip_dir = ImGuiDir_Up;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == ImGuiDir_Right && (move_flags & (ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX)))
    {
        bb_rel.Min.x = bb_rel.Max.x = -window.WindowPadding.x;
        if (move_flags & ImGuiNavMoveFlags_WrapX)
        {
            bb_rel.TranslateY(+bb_rel.GetHeight()); // Next row
            clip_dir = ImGuiDir_Down;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == ImGuiDir_Up && (move_flags & (ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY)))
    {
        bb_rel.Min.y = bb_rel.Max.y = window.ContentSize.y + window.WindowPadding.y;
        if (move_flags & ImGuiNavMoveFlags_WrapY)
        {
            bb_rel.TranslateX(-bb_rel.GetWidth()); // Previous column
            clip_dir = ImGuiDir_Left;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == ImGuiDir_Down && (move_flags & (ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY)))
    {
        bb_rel.Min.y = bb_rel.Max.y = -window.WindowPadding.y;
        if (move_flags & ImGuiNavMoveFlags_WrapY)
        {
            bb_rel.TranslateX(+bb_rel.GetWidth()); // Next column
            clip_dir = ImGuiDir_Right;
        }
        do_forward = true;
    }
    if (!do_forward)
        return;
    window.NavRectRel[g.NavLayer] = bb_rel;
    NavMoveRequestForward(g.NavMoveDir, clip_dir, move_flags, g.NavMoveScrollFlags);
}

static int ImGui::FindWindowFocusIndex(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    IM_UNUSED(g);
    int order = window.FocusOrder;
    IM_ASSERT(window.RootWindow == window); // No child window (not testing _ChildWindow because of docking)
    IM_ASSERT(g.WindowsFocusOrder[order] == window);
    return order;
}

static ImGuiWindow* FindWindowNavFocusable(int i_start, int i_stop, int dir) // FIXME-OPT O(N)
{
    ImGuiContext& g = *GImGui;
    for (int i = i_start; i >= 0 && i < g.WindowsFocusOrder.Size && i != i_stop; i += dir)
        if (ImGui::IsWindowNavFocusable(g.WindowsFocusOrder[i]))
            return g.WindowsFocusOrder[i];
    return NULL;
}

static void NavUpdateWindowingHighlightWindow(int focus_change_dir)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.NavWindowingTarget);
    if (g.NavWindowingTarget.flags & ImGuiWindowFlags_Modal)
        return;

    const int i_current = ImGui::FindWindowFocusIndex(g.NavWindowingTarget);
    ImGuiWindow* window_target = FindWindowNavFocusable(i_current + focus_change_dir, -INT_MAX, focus_change_dir);
    if (!window_target)
        window_target = FindWindowNavFocusable((focus_change_dir < 0) ? (g.WindowsFocusOrder.Size - 1) : 0, i_current, focus_change_dir);
    if (window_target) // Don't reset windowing target if there's a single window in the list
        g.NavWindowingTarget = g.NavWindowingTargetAnim = window_target;
    g.NavWindowingToggleLayer = false;
}

// Windowing management mode
// Keyboard: CTRL+Tab (change focus/move/resize), Alt (toggle menu layer)
// Gamepad:  Hold Menu/Square (change focus/move/resize), Tap Menu/Square (toggle menu layer)
static void ImGui::NavUpdateWindowing()
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;

    ImGuiWindow* apply_focus_window = NULL;
    bool apply_toggle_layer = false;

    ImGuiWindow* modal_window = GetTopMostPopupModal();
    bool allow_windowing = (modal_window == NULL);
    if (!allow_windowing)
        g.NavWindowingTarget = NULL;

    // Fade out
    if (g.NavWindowingTargetAnim && g.NavWindowingTarget == NULL)
    {
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha - io.DeltaTime * 10.0, 0.0);
        if (g.DimBgRatio <= 0.0 && g.NavWindowingHighlightAlpha <= 0.0)
            g.NavWindowingTargetAnim = NULL;
    }

    // Start CTRL+Tab or Square+L/R window selection
    const bool start_windowing_with_gamepad = allow_windowing && !g.NavWindowingTarget && IsNavInputTest(ImGuiNavInput_Menu, ImGuiNavReadMode_Pressed);
    const bool start_windowing_with_keyboard = allow_windowing && !g.NavWindowingTarget && io.KeyCtrl && IsKeyPressed(ImGuiKey_Tab);
    if (start_windowing_with_gamepad || start_windowing_with_keyboard)
        if (ImGuiWindow* window = g.nav_window ? g.nav_window : FindWindowNavFocusable(g.WindowsFocusOrder.Size - 1, -INT_MAX, -1))
        {
            g.NavWindowingTarget = g.NavWindowingTargetAnim = window.RootWindow;
            g.NavWindowingTimer = g.NavWindowingHighlightAlpha = 0.0;
            g.NavWindowingToggleLayer = start_windowing_with_gamepad ? true : false; // Gamepad starts toggling layer
            g.NavInputSource = start_windowing_with_keyboard ? ImGuiInputSource_Keyboard : ImGuiInputSource_Gamepad;
        }

    // Gamepad update
    g.NavWindowingTimer += io.DeltaTime;
    if (g.NavWindowingTarget && g.NavInputSource == ImGuiInputSource_Gamepad)
    {
        // Highlight only appears after a brief time holding the button, so that a fast tap on PadMenu (to toggle nav_layer) doesn't add visual noise
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05));

        // Select window to focus
        const int focus_change_dir = IsNavInputTest(ImGuiNavInput_FocusPrev, ImGuiNavReadMode_RepeatSlow) - IsNavInputTest(ImGuiNavInput_FocusNext, ImGuiNavReadMode_RepeatSlow);
        if (focus_change_dir != 0)
        {
            NavUpdateWindowingHighlightWindow(focus_change_dir);
            g.NavWindowingHighlightAlpha = 1.0;
        }

        // Single press toggles nav_layer, long press with L/R apply actual focus on release (until then the window was merely rendered top-most)
        if (!IsNavInputDown(ImGuiNavInput_Menu))
        {
            g.NavWindowingToggleLayer &= (g.NavWindowingHighlightAlpha < 1.0); // Once button was held long enough we don't consider it a tap-to-toggle-layer press anymore.
            if (g.NavWindowingToggleLayer && g.nav_window)
                apply_toggle_layer = true;
            else if (!g.NavWindowingToggleLayer)
                apply_focus_window = g.NavWindowingTarget;
            g.NavWindowingTarget = NULL;
        }
    }

    // Keyboard: Focus
    if (g.NavWindowingTarget && g.NavInputSource == ImGuiInputSource_Keyboard)
    {
        // Visuals only appears after a brief time after pressing TAB the first time, so that a fast CTRL+TAB doesn't add visual noise
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05)); // 1.0
        if (IsKeyPressed(ImGuiKey_Tab, true))
            NavUpdateWindowingHighlightWindow(io.KeyShift ? +1 : -1);
        if (!io.KeyCtrl)
            apply_focus_window = g.NavWindowingTarget;
    }

    // Keyboard: Press and Release ALT to toggle menu layer
    // - Testing that only Alt is tested prevents Alt+Shift or AltGR from toggling menu layer.
    // - AltGR is normally Alt+Ctrl but we can't reliably detect it (not all backends/systems/layout emit it as Alt+Ctrl). But even on keyboards without AltGR we don't want Alt+Ctrl to open menu anyway.
	const bool nav_keyboard_active = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;
    if (nav_keyboard_active && IsKeyPressed(ImGuiKey_ModAlt))
    {
        g.NavWindowingToggleLayer = true;
        g.NavInputSource = ImGuiInputSource_Keyboard;
    }
    if (g.NavWindowingToggleLayer && g.NavInputSource == ImGuiInputSource_Keyboard)
    {
        // We cancel toggling nav layer when any text has been typed (generally while holding Alt). (See #370)
        // We cancel toggling nav layer when other modifiers are pressed. (See #4439)
        if (io.InputQueueCharacters.Size > 0 || io.KeyCtrl || io.KeyShift || io.KeySuper)
            g.NavWindowingToggleLayer = false;

        // Apply layer toggle on release
        // Important: as before version <18314 we lacked an explicit io event for focus gain/loss, we also compare mouse validity to detect old backends clearing mouse pos on focus loss.
        if (IsKeyReleased(ImGuiKey_ModAlt) && g.NavWindowingToggleLayer)
            if (g.active_id == 0 || g.ActiveIdAllowOverlap)
                if (is_mouse_pos_valid(&io.MousePos) == is_mouse_pos_valid(&io.MousePosPrev))
                    apply_toggle_layer = true;
        if (!IsKeyDown(ImGuiKey_ModAlt))
            g.NavWindowingToggleLayer = false;
    }

    // Move window
    if (g.NavWindowingTarget && !(g.NavWindowingTarget.flags & ImGuiWindowFlags_NoMove))
    {
        Vector2D move_delta;
        if (g.NavInputSource == ImGuiInputSource_Keyboard && !io.KeyShift)
            move_delta = GetNavInputAmount2d(ImGuiNavDirSourceFlags_RawKeyboard, ImGuiNavReadMode_Down);
        if (g.NavInputSource == ImGuiInputSource_Gamepad)
            move_delta = GetNavInputAmount2d(ImGuiNavDirSourceFlags_PadLStick, ImGuiNavReadMode_Down);
        if (move_delta.x != 0.0 || move_delta.y != 0.0)
        {
            const float NAV_MOVE_SPEED = 800.0;
            const float move_speed = ImFloor(NAV_MOVE_SPEED * io.DeltaTime * ImMin(io.DisplayFramebufferScale.x, io.DisplayFramebufferScale.y)); // FIXME: Doesn't handle variable framerate very well
            ImGuiWindow* moving_window = g.NavWindowingTarget->RootWindowDockTree;
            set_window_pos(moving_window, moving_window.Pos + move_delta * move_speed, Cond::Always);
            g.NavDisableMouseHover = true;
        }
    }

    // Apply final focus
    if (apply_focus_window && (g.nav_window == NULL || apply_focus_window != g.nav_window->RootWindow))
    {
        ImGuiViewport* previous_viewport = g.nav_window ? g.nav_window->Viewport : NULL;
        clear_active_id();
        NavRestoreHighlightAfterMove();
        apply_focus_window = NavRestoreLastChildNavWindow(apply_focus_window);
        ClosePopupsOverWindow(apply_focus_window, false);
        focus_window(apply_focus_window);
        if (apply_focus_window.NavLastIds[0] == 0)
            NavInitWindow(apply_focus_window, false);

        // If the window has ONLY a menu layer (no main layer), select it directly
        // Use NavLayersActiveMaskNext since windows didn't have a chance to be Begin()-ed on this frame,
        // so CTRL+Tab where the keys are only held for 1 frame will be able to use correct layers mask since
        // the target window as already been previewed once.
        // FIXME-NAV: This should be done in NavInit.. or in focus_window... However in both of those cases,
        // we won't have a guarantee that windows has been visible before and therefore NavLayersActiveMask*
        // won't be valid.
        if (apply_focus_window.DC.NavLayersActiveMaskNext == (1 << ImGuiNavLayer_Menu))
            g.NavLayer = ImGuiNavLayer_Menu;

        // Request OS level focus
        if (apply_focus_window.viewport != previous_viewport && g.PlatformIO.Platform_SetWindowFocus)
            g.PlatformIO.Platform_SetWindowFocus(apply_focus_window.viewport);
    }
    if (apply_focus_window)
        g.NavWindowingTarget = NULL;

    // Apply menu/layer toggle
    if (apply_toggle_layer && g.nav_window)
    {
        clear_active_id();

        // Move to parent menu if necessary
        ImGuiWindow* new_nav_window = g.nav_window;
        while (new_nav_window.ParentWindow
            && (new_nav_window.DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) == 0
            && (new_nav_window.Flags & ImGuiWindowFlags_ChildWindow) != 0
            && (new_nav_window.Flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_ChildMenu)) == 0)
            new_nav_window = new_nav_window.ParentWindow;
        if (new_nav_window != g.nav_window)
        {
            ImGuiWindow* old_nav_window = g.nav_window;
            focus_window(new_nav_window);
            new_nav_window.NavLastChildNavWindow = old_nav_window;
        }

        // Toggle layer
        const ImGuiNavLayer new_nav_layer = (g.nav_window->DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) ? (ImGuiNavLayer)(g.NavLayer ^ 1) : ImGuiNavLayer_Main;
        if (new_nav_layer != g.NavLayer)
        {
            // Reinitialize navigation when entering menu bar with the Alt key (FIXME: could be a properly of the layer?)
            const bool preserve_layer_1_nav_id = (new_nav_window.DockNodeAsHost != NULL);
            if (new_nav_layer == ImGuiNavLayer_Menu && !preserve_layer_1_nav_id)
                g.nav_window->NavLastIds[new_nav_layer] = 0;
            NavRestoreLayer(new_nav_layer);
            NavRestoreHighlightAfterMove();
        }
    }
}

// Window has already passed the IsWindowNavFocusable()
static const char* GetFallbackWindowNameForWindowingList(ImGuiWindow* window)
{
    if (window.Flags & ImGuiWindowFlags_Popup)
        return "(Popup)";
    if ((window.Flags & ImGuiWindowFlags_MenuBar) && strcmp(window.Name, "##MainMenuBar") == 0)
        return "(Main menu bar)";
    if (window.DockNodeAsHost)
        return "(Dock node)";
    return "(Untitled)";
}

// Overlay displayed when using CTRL+TAB. Called by EndFrame().
void ImGui::NavUpdateWindowingOverlay()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.NavWindowingTarget != NULL);

    if (g.NavWindowingTimer < NAV_WINDOWING_LIST_APPEAR_DELAY)
        return;

    if (g.NavWindowingListWindow == NULL)
        g.NavWindowingListWindow = FindWindowByName("###NavWindowingList");
    const ImGuiViewport* viewport = /*g.nav_window ? g.nav_window->viewport :*/ GetMainViewport();
    SetNextWindowSizeConstraints(DimgVec2D::new(viewport->Size.x * 0.20, viewport->Size.y * 0.20), DimgVec2D::new(FLT_MAX, FLT_MAX));
    SetNextWindowPos(viewport->GetCenter(), Cond::Always, DimgVec2D::new(0.5, 0.5));
    PushStyleVar(ImGuiStyleVar_WindowPadding, g.Style.WindowPadding * 2.0);
    Begin("###NavWindowingList", NULL, ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoFocusOnAppearing | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoInputs | ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoSavedSettings);
    for (int n = g.WindowsFocusOrder.Size - 1; n >= 0; n--)
    {
        ImGuiWindow* window = g.WindowsFocusOrder[n];
        IM_ASSERT(window != NULL); // Fix static analyzers
        if (!IsWindowNavFocusable(window))
            continue;
        const char* label = window.Name;
        if (label == FindRenderedTextEnd(label))
            label = GetFallbackWindowNameForWindowingList(window);
        Selectable(label, g.NavWindowingTarget == window);
    }
    End();
    PopStyleVar();
}


//-----------------------------------------------------------------------------
// [SECTION] DRAG AND DROP
//-----------------------------------------------------------------------------

bool ImGui::IsDragDropActive()
{
    ImGuiContext& g = *GImGui;
    return g.DragDropActive;
}

void ImGui::ClearDragDrop()
{
    ImGuiContext& g = *GImGui;
    g.DragDropActive = false;
    g.DragDropPayload.Clear();
    g.DragDropAcceptFlags = ImGuiDragDropFlags_None;
    g.DragDropAcceptIdCurr = g.DragDropAcceptIdPrev = 0;
    g.DragDropAcceptIdCurrRectSurface = FLT_MAX;
    g.DragDropAcceptFrameCount = -1;

    g.DragDropPayloadBufHeap.clear();
    memset(&g.DragDropPayloadBufLocal, 0, sizeof(g.DragDropPayloadBufLocal));
}

// When this returns true you need to: a) call SetDragDropPayload() exactly once, b) you may render the payload visual/description, c) call EndDragDropSource()
// If the item has an identifier:
// - This assume/require the item to be activated (typically via ButtonBehavior).
// - Therefore if you want to use this with a mouse button other than left mouse button, it is up to the item itself to activate with another button.
// - We then pull and use the mouse button that was used to activate the item and use it to carry on the drag.
// If the item has no identifier:
// - Currently always assume left mouse button.
bool ImGui::BeginDragDropSource(ImGuiDragDropFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;

    // FIXME-DRAGDROP: While in the common-most "drag from non-zero active id" case we can tell the mouse button,
    // in both SourceExtern and id==0 cases we may requires something else (explicit flags or some heuristic).
    ImGuiMouseButton mouse_button = ImGuiMouseButton_Left;

    bool source_drag_active = false;
    ImGuiID source_id = 0;
    ImGuiID source_parent_id = 0;
    if (!(flags & ImGuiDragDropFlags_SourceExtern))
    {
        source_id = g.last_item_data.ID;
        if (source_id != 0)
        {
            // Common path: items with id
            if (g.active_id != source_id)
                return false;
            if (g.ActiveIdMouseButton != -1)
                mouse_button = g.ActiveIdMouseButton;
            if (g.io.mouse_down[mouse_button] == false || window.SkipItems)
                return false;
            g.ActiveIdAllowOverlap = false;
        }
        else
        {
            // Uncommon path: items without id
            if (g.io.mouse_down[mouse_button] == false || window.SkipItems)
                return false;
            if ((g.last_item_data.StatusFlags & ImGuiItemStatusFlags_HoveredRect) == 0 && (g.active_id == 0 || g.active_id_window != window))
                return false;

            // If you want to use BeginDragDropSource() on an item with no unique identifier for interaction, such as Text() or Image(), you need to:
            // A) Read the explanation below, B) Use the ImGuiDragDropFlags_SourceAllowNullID flag.
            if (!(flags & ImGuiDragDropFlags_SourceAllowNullID))
            {
                IM_ASSERT(0);
                return false;
            }

            // Magic fallback to handle items with no assigned id, e.g. Text(), Image()
            // We build a throwaway id based on current id stack + relative AABB of items in window.
            // THE IDENTIFIER WON'T SURVIVE ANY REPOSITIONING/RESIZINGG OF THE WIDGET, so if your widget moves your dragging operation will be canceled.
            // We don't need to maintain/call clear_active_id() as releasing the button will early out this function and trigger !active_id_is_alive.
            // Rely on keeping other window->LastItemXXX fields intact.
            source_id = g.last_item_data.ID = window.GetIDFromRectangle(g.last_item_data.Rect);
            keep_alive_id(source_id);
            bool is_hovered = ItemHoverable(g.last_item_data.Rect, source_id);
            if (is_hovered && g.io.mouse_clicked[mouse_button])
            {
                SetActiveID(source_id, window);
                focus_window(window);
            }
            if (g.active_id == source_id) // Allow the underlying widget to display/return hovered during the mouse release frame, else we would get a flicker.
                g.ActiveIdAllowOverlap = is_hovered;
        }
        if (g.active_id != source_id)
            return false;
        source_parent_id = window.IDStack.back();
        source_drag_active = IsMouseDragging(mouse_button);

        // Disable navigation and key inputs while dragging + cancel existing request if any
        SetActiveIdUsingNavAndKeys();
    }
    else
    {
        window = NULL;
        source_id = ImHashStr("#SourceExtern");
        source_drag_active = true;
    }

    if (source_drag_active)
    {
        if (!g.DragDropActive)
        {
            IM_ASSERT(source_id != 0);
            ClearDragDrop();
            ImGuiPayload& payload = g.DragDropPayload;
            payload.SourceId = source_id;
            payload.SourceParentId = source_parent_id;
            g.DragDropActive = true;
            g.DragDropSourceFlags = flags;
            g.DragDropMouseButton = mouse_button;
            if (payload.SourceId == g.active_id)
                g.ActiveIdNoClearOnFocusLoss = true;
        }
        g.DragDropSourceFrameCount = g.FrameCount;
        g.DragDropWithinSource = true;

        if (!(flags & ImGuiDragDropFlags_SourceNoPreviewTooltip))
        {
            // Target can request the Source to not display its tooltip (we use a dedicated flag to make this request explicit)
            // We unfortunately can't just modify the source flags and skip the call to BeginTooltip, as caller may be emitting contents.
            BeginTooltip();
            if (g.DragDropAcceptIdPrev && (g.DragDropAcceptFlags & ImGuiDragDropFlags_AcceptNoPreviewTooltip))
            {
                ImGuiWindow* tooltip_window = g.CurrentWindow;
                tooltip_window.Hidden = tooltip_window.SkipItems = true;
                tooltip_window.HiddenFramesCanSkipItems = 1;
            }
        }

        if (!(flags & ImGuiDragDropFlags_SourceNoDisableHover) && !(flags & ImGuiDragDropFlags_SourceExtern))
            g.last_item_data.StatusFlags &= ~ImGuiItemStatusFlags_HoveredRect;

        return true;
    }
    return false;
}

void ImGui::EndDragDropSource()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.DragDropActive);
    IM_ASSERT(g.DragDropWithinSource && "Not after a BeginDragDropSource()?");

    if (!(g.DragDropSourceFlags & ImGuiDragDropFlags_SourceNoPreviewTooltip))
        EndTooltip();

    // Discard the drag if have not called SetDragDropPayload()
    if (g.DragDropPayload.DataFrameCount == -1)
        ClearDragDrop();
    g.DragDropWithinSource = false;
}

// Use 'cond' to choose to submit payload on drag start or every frame
bool ImGui::SetDragDropPayload(const char* type, const void* data, size_t data_size, ImGuiCond cond)
{
    ImGuiContext& g = *GImGui;
    ImGuiPayload& payload = g.DragDropPayload;
    if (cond == 0)
        cond = Cond::Always;

    IM_ASSERT(type != NULL);
    IM_ASSERT(strlen(type) < IM_ARRAYSIZE(payload.DataType) && "Payload type can be at most 32 characters long");
    IM_ASSERT((data != NULL && data_size > 0) || (data == NULL && data_size == 0));
    IM_ASSERT(cond == Cond::Always || cond == ImGuiCond_Once);
    IM_ASSERT(payload.SourceId != 0);                               // Not called between BeginDragDropSource() and EndDragDropSource()

    if (cond == Cond::Always || payload.DataFrameCount == -1)
    {
        // Copy payload
        ImStrncpy(payload.DataType, type, IM_ARRAYSIZE(payload.DataType));
        g.DragDropPayloadBufHeap.resize(0);
        if (data_size > sizeof(g.DragDropPayloadBufLocal))
        {
            // Store in heap
            g.DragDropPayloadBufHeap.resize(data_size);
            payload.Data = g.DragDropPayloadBufHeap.Data;
            memcpy(payload.Data, data, data_size);
        }
        else if (data_size > 0)
        {
            // Store locally
            memset(&g.DragDropPayloadBufLocal, 0, sizeof(g.DragDropPayloadBufLocal));
            payload.Data = g.DragDropPayloadBufLocal;
            memcpy(payload.Data, data, data_size);
        }
        else
        {
            payload.Data = NULL;
        }
        payload.DataSize = data_size;
    }
    payload.DataFrameCount = g.FrameCount;

    // Return whether the payload has been accepted
    return (g.DragDropAcceptFrameCount == g.FrameCount) || (g.DragDropAcceptFrameCount == g.FrameCount - 1);
}

bool ImGui::BeginDragDropTargetCustom(const ImRect& bb, ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    if (!g.DragDropActive)
        return false;

    ImGuiWindow* window = g.CurrentWindow;
    ImGuiWindow* hovered_window = g.HoveredWindowUnderMovingWindow;
    if (hovered_window == NULL || window.RootWindowDockTree != hovered_window.RootWindowDockTree)
        return false;
    IM_ASSERT(id != 0);
    if (!IsMouseHoveringRect(bb.Min, bb.Max) || (id == g.DragDropPayload.SourceId))
        return false;
    if (window.SkipItems)
        return false;

    IM_ASSERT(g.DragDropWithinTarget == false);
    g.DragDropTargetRect = bb;
    g.DragDropTargetId = id;
    g.DragDropWithinTarget = true;
    return true;
}

// We don't use BeginDragDropTargetCustom() and duplicate its code because:
// 1) we use LastItemRectHoveredRect which handles items that pushes a temporarily clip rectangle in their code. Calling BeginDragDropTargetCustom(LastItemRect) would not handle them.
// 2) and it's faster. as this code may be very frequently called, we want to early out as fast as we can.
// Also note how the hovered_window test is positioned differently in both functions (in both functions we optimize for the cheapest early out case)
bool ImGui::BeginDragDropTarget()
{
    ImGuiContext& g = *GImGui;
    if (!g.DragDropActive)
        return false;

    ImGuiWindow* window = g.CurrentWindow;
    if (!(g.last_item_data.StatusFlags & ImGuiItemStatusFlags_HoveredRect))
        return false;
    ImGuiWindow* hovered_window = g.HoveredWindowUnderMovingWindow;
    if (hovered_window == NULL || window.RootWindowDockTree != hovered_window.RootWindowDockTree || window.SkipItems)
        return false;

    const ImRect& display_rect = (g.last_item_data.StatusFlags & ImGuiItemStatusFlags_HasDisplayRect) ? g.last_item_data.DisplayRect : g.last_item_data.Rect;
    ImGuiID id = g.last_item_data.ID;
    if (id == 0)
    {
        id = window.GetIDFromRectangle(display_rect);
        keep_alive_id(id);
    }
    if (g.DragDropPayload.SourceId == id)
        return false;

    IM_ASSERT(g.DragDropWithinTarget == false);
    g.DragDropTargetRect = display_rect;
    g.DragDropTargetId = id;
    g.DragDropWithinTarget = true;
    return true;
}

bool ImGui::is_drag_drop_payload_being_accepted()
{
    ImGuiContext& g = *GImGui;
    return g.DragDropActive && g.DragDropAcceptIdPrev != 0;
}

const ImGuiPayload* ImGui::AcceptDragDropPayload(const char* type, ImGuiDragDropFlags flags)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    ImGuiPayload& payload = g.DragDropPayload;
    IM_ASSERT(g.DragDropActive);                        // Not called between BeginDragDropTarget() and EndDragDropTarget() ?
    IM_ASSERT(payload.DataFrameCount != -1);            // Forgot to call EndDragDropTarget() ?
    if (type != NULL && !payload.IsDataType(type))
        return NULL;

    // Accept smallest drag target bounding box, this allows us to nest drag targets conveniently without ordering constraints.
    // NB: We currently accept NULL id as target. However, overlapping targets requires a unique id to function!
    const bool was_accepted_previously = (g.DragDropAcceptIdPrev == g.DragDropTargetId);
    ImRect r = g.DragDropTargetRect;
    float r_surface = r.GetWidth() * r.GetHeight();
    if (r_surface <= g.DragDropAcceptIdCurrRectSurface)
    {
        g.DragDropAcceptFlags = flags;
        g.DragDropAcceptIdCurr = g.DragDropTargetId;
        g.DragDropAcceptIdCurrRectSurface = r_surface;
    }

    // Render default drop visuals
    // FIXME-DRAGDROP: Settle on a proper default visuals for drop target.
    payload.Preview = was_accepted_previously;
    flags |= (g.DragDropSourceFlags & ImGuiDragDropFlags_AcceptNoDrawDefaultRect); // Source can also inhibit the preview (useful for external sources that lives for 1 frame)
    if (!(flags & ImGuiDragDropFlags_AcceptNoDrawDefaultRect) && payload.Preview)
        window.DrawList->AddRect(r.Min - DimgVec2D::new(3.5,3.5), r.Max + DimgVec2D::new(3.5, 3.5), GetColorU32(ImGuiCol_DragDropTarget), 0.0, 0, 2.0);

    g.DragDropAcceptFrameCount = g.FrameCount;
    payload.Delivery = was_accepted_previously && !IsMouseDown(g.DragDropMouseButton); // For extern drag sources affecting os window focus, it's easier to just test !IsMouseDown() instead of IsMouseReleased()
    if (!payload.Delivery && !(flags & ImGuiDragDropFlags_AcceptBeforeDelivery))
        return NULL;

    return &payload;
}

const ImGuiPayload* ImGui::GetDragDropPayload()
{
    ImGuiContext& g = *GImGui;
    return g.DragDropActive ? &g.DragDropPayload : NULL;
}

// We don't really use/need this now, but added it for the sake of consistency and because we might need it later.
void ImGui::EndDragDropTarget()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.DragDropActive);
    IM_ASSERT(g.DragDropWithinTarget);
    g.DragDropWithinTarget = false;
}

//-----------------------------------------------------------------------------
// [SECTION] LOGGING/CAPTURING
//-----------------------------------------------------------------------------
// All text output from the interface can be captured into tty/file/clipboard.
// By default, tree nodes are automatically opened during logging.
//-----------------------------------------------------------------------------



//-----------------------------------------------------------------------------
// [SECTION] SETTINGS
//-----------------------------------------------------------------------------
// - UpdateSettings() [Internal]
// - MarkIniSettingsDirty() [Internal]
// - CreateNewWindowSettings() [Internal]
// - FindWindowSettings() [Internal]
// - FindOrCreateWindowSettings() [Internal]
// - FindSettingsHandler() [Internal]
// - ClearIniSettings() [Internal]
// - LoadIniSettingsFromDisk()
// - LoadIniSettingsFromMemory()
// - SaveIniSettingsToDisk()
// - SaveIniSettingsToMemory()
// - WindowSettingsHandler_***() [Internal]
//-----------------------------------------------------------------------------

// Called by NewFrame()
void ImGui::UpdateSettings()
{
    // Load settings on first frame (if not explicitly loaded manually before)
    ImGuiContext& g = *GImGui;
    if (!g.SettingsLoaded)
    {
        IM_ASSERT(g.SettingsWindows.empty());
        if (g.io.IniFilename)
            LoadIniSettingsFromDisk(g.io.IniFilename);
        g.SettingsLoaded = true;
    }

    // Save settings (with a delay after the last modification, so we don't spam disk too much)
    if (g.SettingsDirtyTimer > 0.0)
    {
        g.SettingsDirtyTimer -= g.io.DeltaTime;
        if (g.SettingsDirtyTimer <= 0.0)
        {
            if (g.io.IniFilename != NULL)
                SaveIniSettingsToDisk(g.io.IniFilename);
            else
                g.io.WantSaveIniSettings = true;  // Let user know they can call SaveIniSettingsToMemory(). user will need to clear io.want_save_ini_settings themselves.
            g.SettingsDirtyTimer = 0.0;
        }
    }
}

void ImGui::MarkIniSettingsDirty()
{
    ImGuiContext& g = *GImGui;
    if (g.SettingsDirtyTimer <= 0.0)
        g.SettingsDirtyTimer = g.io.IniSavingRate;
}

void ImGui::MarkIniSettingsDirty(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    if (!(window.Flags & ImGuiWindowFlags_NoSavedSettings))
        if (g.SettingsDirtyTimer <= 0.0)
            g.SettingsDirtyTimer = g.io.IniSavingRate;
}

ImGuiWindowSettings* ImGui::CreateNewWindowSettings(const char* name)
{
    ImGuiContext& g = *GImGui;

#if !IMGUI_DEBUG_INI_SETTINGS
    // Skip to the "###" marker if any. We don't skip past to match the behavior of GetID()
    // Preserve the full string when IMGUI_DEBUG_INI_SETTINGS is set to make .ini inspection easier.
    if (const char* p = strstr(name, "###"))
        name = p;

    const size_t name_len = strlen(name);

    // Allocate chunk
    const size_t chunk_size = sizeof(ImGuiWindowSettings) + name_len + 1;
    ImGuiWindowSettings* settings = g.SettingsWindows.alloc_chunk(chunk_size);
    IM_PLACEMENT_NEW(settings) ImGuiWindowSettings();
    settings->ID = ImHashStr(name, name_len);
    memcpy(settings->GetName(), name, name_len + 1);   // Store with zero terminator

    return settings;
}

ImGuiWindowSettings* ImGui::FindWindowSettings(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != NULL; settings = g.SettingsWindows.next_chunk(settings))
        if (settings->ID == id)
            return settings;
    return NULL;
}

ImGuiWindowSettings* ImGui::FindOrCreateWindowSettings(const char* name)
{
    if (ImGuiWindowSettings* settings = FindWindowSettings(ImHashStr(name)))
        return settings;
    return CreateNewWindowSettings(name);
}

void ImGui::AddSettingsHandler(const ImGuiSettingsHandler* handler)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(FindSettingsHandler(handler->TypeName) == NULL);
    g.SettingsHandlers.push_back(*handler);
}

void ImGui::RemoveSettingsHandler(const char* type_name)
{
    ImGuiContext& g = *GImGui;
    if (ImGuiSettingsHandler* handler = FindSettingsHandler(type_name))
        g.SettingsHandlers.erase(handler);
}

ImGuiSettingsHandler* ImGui::FindSettingsHandler(const char* type_name)
{
    ImGuiContext& g = *GImGui;
    const ImGuiID type_hash = ImHashStr(type_name);
    for (int handler_n = 0; handler_n < g.SettingsHandlers.Size; handler_n += 1)
        if (g.SettingsHandlers[handler_n].TypeHash == type_hash)
            return &g.SettingsHandlers[handler_n];
    return NULL;
}

void ImGui::ClearIniSettings()
{
    ImGuiContext& g = *GImGui;
    g.SettingsIniData.clear();
    for (int handler_n = 0; handler_n < g.SettingsHandlers.Size; handler_n += 1)
        if (g.SettingsHandlers[handler_n].ClearAllFn)
            g.SettingsHandlers[handler_n].ClearAllFn(&g, &g.SettingsHandlers[handler_n]);
}

void ImGui::LoadIniSettingsFromDisk(const char* ini_filename)
{
    size_t file_data_size = 0;
    char* file_data = (char*)ImFileLoadToMemory(ini_filename, "rb", &file_data_size);
    if (!file_data)
        return;
    if (file_data_size > 0)
        LoadIniSettingsFromMemory(file_data, file_data_size);
    IM_FREE(file_data);
}

// Zero-tolerance, no error reporting, cheap .ini parsing
void ImGui::LoadIniSettingsFromMemory(const char* ini_data, size_t ini_size)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.Initialized);
    //IM_ASSERT(!g.within_frame_scope && "Cannot be called between NewFrame() and EndFrame()");
    //IM_ASSERT(g.settings_loaded == false && g.frame_count == 0);

    // For user convenience, we allow passing a non zero-terminated string (hence the ini_size parameter).
    // For our convenience and to make the code simpler, we'll also write zero-terminators within the buffer. So let's create a writable copy..
    if (ini_size == 0)
        ini_size = strlen(ini_data);
    g.SettingsIniData.Buf.resize(ini_size + 1);
    char* const buf = g.SettingsIniData.Buf.Data;
    char* const buf_end = buf + ini_size;
    memcpy(buf, ini_data, ini_size);
    buf_end[0] = 0;

    // Call pre-read handlers
    // Some types will clear their data (e.g. dock information) some types will allow merge/override (window)
    for (int handler_n = 0; handler_n < g.SettingsHandlers.Size; handler_n += 1)
        if (g.SettingsHandlers[handler_n].ReadInitFn)
            g.SettingsHandlers[handler_n].ReadInitFn(&g, &g.SettingsHandlers[handler_n]);

    void* entry_data = NULL;
    ImGuiSettingsHandler* entry_handler = NULL;

    char* line_end = NULL;
    for (char* line = buf; line < buf_end; line = line_end + 1)
    {
        // Skip new lines markers, then find end of the line
        while (*line == '\n' || *line == '\r')
            line += 1;
        line_end = line;
        while (line_end < buf_end && *line_end != '\n' && *line_end != '\r')
            line_end += 1;
        line_end[0] = 0;
        if (line[0] == ';')
            continue;
        if (line[0] == '[' && line_end > line && line_end[-1] == ']')
        {
            // Parse "[Type][name]". Note that 'name' can itself contains [] characters, which is acceptable with the current format and parsing code.
            line_end[-1] = 0;
            const char* name_end = line_end - 1;
            const char* type_start = line + 1;
            char* type_end = (char*)(void*)ImStrchrRange(type_start, name_end, ']');
            const char* name_start = type_end ? ImStrchrRange(type_end + 1, name_end, '[') : NULL;
            if (!type_end || !name_start)
                continue;
            *type_end = 0; // Overwrite first ']'
            name_start += 1;  // Skip second '['
            entry_handler = FindSettingsHandler(type_start);
            entry_data = entry_handler ? entry_handler->ReadOpenFn(&g, entry_handler, name_start) : NULL;
        }
        else if (entry_handler != NULL && entry_data != NULL)
        {
            // Let type handler parse the line
            entry_handler->ReadLineFn(&g, entry_handler, entry_data, line);
        }
    }
    g.SettingsLoaded = true;

    // [DEBUG] Restore untouched copy so it can be browsed in Metrics (not strictly necessary)
    memcpy(buf, ini_data, ini_size);

    // Call post-read handlers
    for (int handler_n = 0; handler_n < g.SettingsHandlers.Size; handler_n += 1)
        if (g.SettingsHandlers[handler_n].ApplyAllFn)
            g.SettingsHandlers[handler_n].ApplyAllFn(&g, &g.SettingsHandlers[handler_n]);
}

void ImGui::SaveIniSettingsToDisk(const char* ini_filename)
{
    ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0.0;
    if (!ini_filename)
        return;

    size_t ini_data_size = 0;
    const char* ini_data = SaveIniSettingsToMemory(&ini_data_size);
    ImFileHandle f = ImFileOpen(ini_filename, "wt");
    if (!f)
        return;
    ImFileWrite(ini_data, sizeof(char), ini_data_size, f);
    ImFileClose(f);
}

// Call registered handlers (e.g. SettingsHandlerWindow_WriteAll() + custom handlers) to write their stuff into a text buffer
const char* ImGui::SaveIniSettingsToMemory(size_t* out_size)
{
    ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0.0;
    g.SettingsIniData.Buf.resize(0);
    g.SettingsIniData.Buf.push_back(0);
    for (int handler_n = 0; handler_n < g.SettingsHandlers.Size; handler_n += 1)
    {
        ImGuiSettingsHandler* handler = &g.SettingsHandlers[handler_n];
        handler->WriteAllFn(&g, handler, &g.SettingsIniData);
    }
    if (out_size)
        *out_size = g.SettingsIniData.size();
    return g.SettingsIniData.c_str();
}

static void WindowSettingsHandler_ClearAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiContext& g = *ctx;
    for (int i = 0; i != g.Windows.Size; i += 1)
        g.Windows[i]->SettingsOffset = -1;
    g.SettingsWindows.clear();
}

static void* WindowSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, const char* name)
{
    ImGuiWindowSettings* settings = ImGui::FindOrCreateWindowSettings(name);
    ImGuiID id = settings->ID;
    *settings = ImGuiWindowSettings(); // clear existing if recycling previous entry
    settings->ID = id;
    settings->WantApply = true;
    return (void*)settings;
}

static void WindowSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, void* entry, const char* line)
{
    ImGuiWindowSettings* settings = (ImGuiWindowSettings*)entry;
    int x, y;
    int i;
    ImU32 u1;
    if (sscanf(line, "pos=%i,%i", &x, &y) == 2)             { settings.pos = Vector2Dih((short)x, (short)y); }
    else if (sscanf(line, "size=%i,%i", &x, &y) == 2)       { settings->Size = Vector2Dih((short)x, (short)y); }
    else if (sscanf(line, "viewport_id=0x%08X", &u1) == 1)   { settings->ViewportId = u1; }
    else if (sscanf(line, "viewport_pos=%i,%i", &x, &y) == 2){ settings->ViewportPos = Vector2Dih((short)x, (short)y); }
    else if (sscanf(line, "collapsed=%d", &i) == 1)         { settings->Collapsed = (i != 0); }
    else if (sscanf(line, "dock_id=0x%x,%d", &u1, &i) == 2)  { settings->DockId = u1; settings->DockOrder = (short)i; }
    else if (sscanf(line, "dock_id=0x%x", &u1) == 1)         { settings->DockId = u1; settings->DockOrder = -1; }
    else if (sscanf(line, "class_id=0x%x", &u1) == 1)        { settings->ClassId = u1; }
}

// Apply to existing windows (if any)
static void WindowSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiContext& g = *ctx;
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != NULL; settings = g.SettingsWindows.next_chunk(settings))
        if (settings->WantApply)
        {
            if (ImGuiWindow* window = ImGui::FindWindowByID(settings->ID))
                ApplyWindowSettings(window, settings);
            settings->WantApply = false;
        }
}

static void WindowSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* buf)
{
    // Gather data from windows that were active during this session
    // (if a window wasn't opened in this session we preserve its settings)
    ImGuiContext& g = *ctx;
    for (int i = 0; i != g.Windows.Size; i += 1)
    {
        ImGuiWindow* window = g.Windows[i];
        if (window.Flags & ImGuiWindowFlags_NoSavedSettings)
            continue;

        ImGuiWindowSettings* settings = (window.SettingsOffset != -1) ? g.SettingsWindows.ptr_from_offset(window.SettingsOffset) : ImGui::FindWindowSettings(window.ID);
        if (!settings)
        {
            settings = ImGui::CreateNewWindowSettings(window.Name);
            window.SettingsOffset = g.SettingsWindows.offset_from_ptr(settings);
        }
        IM_ASSERT(settings->ID == window.ID);
        settings.pos = Vector2Dih(window.Pos - window.ViewportPos);
        settings->Size = Vector2Dih(window.SizeFull);
        settings->ViewportId = window.ViewportId;
        settings->ViewportPos = Vector2Dih(window.ViewportPos);
        IM_ASSERT(window.DockNode == NULL || window.DockNode->ID == window.DockId);
        settings->DockId = window.DockId;
        settings->ClassId = window.WindowClass.ClassId;
        settings->DockOrder = window.DockOrder;
        settings->Collapsed = window.Collapsed;
    }

    // Write to text buffer
    buf->reserve(buf->size() + g.SettingsWindows.size() * 6); // ballpark reserve
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != NULL; settings = g.SettingsWindows.next_chunk(settings))
    {
        const char* settings_name = settings->GetName();
        buf->appendf("[%s][%s]\n", handler->TypeName, settings_name);
        if (settings->ViewportId != 0 && settings->ViewportId != ImGui::IMGUI_VIEWPORT_DEFAULT_ID)
        {
            buf->appendf("viewport_pos=%d,%d\n", settings->ViewportPos.x, settings->ViewportPos.y);
            buf->appendf("viewport_id=0x%08X\n", settings->ViewportId);
        }
        if (settings.pos.x != 0 || settings.pos.y != 0 || settings->ViewportId == ImGui::IMGUI_VIEWPORT_DEFAULT_ID)
            buf->appendf("pos=%d,%d\n", settings.pos.x, settings.pos.y);
        if (settings->Size.x != 0 || settings->Size.y != 0)
            buf->appendf("size=%d,%d\n", settings->Size.x, settings->Size.y);
        buf->appendf("collapsed=%d\n", settings->Collapsed);
        if (settings->DockId != 0)
        {
            //buf->appendf("tab_id=0x%08X\n", ImHashStr("#TAB", 4, settings->id)); // window->tab_id: this is not read back but writing it makes "debugging" the .ini data easier.
            if (settings->DockOrder == -1)
                buf->appendf("dock_id=0x%08X\n", settings->DockId);
            else
                buf->appendf("dock_id=0x%08X,%d\n", settings->DockId, settings->DockOrder);
            if (settings->ClassId != 0)
                buf->appendf("class_id=0x%08X\n", settings->ClassId);
        }
        buf->append("\n");
    }
}


//-----------------------------------------------------------------------------
// [SECTION] VIEWPORTS, PLATFORM WINDOWS
//-----------------------------------------------------------------------------
// - GetMainViewport()
// - FindViewportByID()
// - FindViewportByPlatformHandle()
// - SetCurrentViewport() [Internal]
// - SetWindowViewport() [Internal]
// - GetWindowAlwaysWantOwnViewport() [Internal]
// - update_try_merge_window_into_host_viewport() [Internal]
// - UpdateTryMergeWindowIntoHostViewports() [Internal]
// - TranslateWindowsInViewport() [Internal]
// - ScaleWindowsInViewport() [Internal]
// - FindHoveredViewportFromPlatformWindowStack() [Internal]
// - UpdateViewportsNewFrame() [Internal]
// - UpdateViewportsEndFrame() [Internal]
// - AddUpdateViewport() [Internal]
// - WindowSelectViewport() [Internal]
// - WindowSyncOwnedViewport() [Internal]
// - UpdatePlatformWindows()
// - RenderPlatformWindowsDefault()
// - FindPlatformMonitorForPos() [Internal]
// - FindPlatformMonitorForRect() [Internal]
// - UpdateViewportPlatformMonitor() [Internal]
// - DestroyPlatformWindow() [Internal]
// - DestroyPlatformWindows()
//-----------------------------------------------------------------------------

ImGuiViewport* ImGui::GetMainViewport()
{
    ImGuiContext& g = *GImGui;
    return g.Viewports[0];
}

// FIXME: This leaks access to viewports not listed in platform_io.viewports[]. Problematic? (#4236)
ImGuiViewport* ImGui::FindViewportByID(ImGuiID id)
{
    ImGuiContext& g = *GImGui;
    for (int n = 0; n < g.Viewports.Size; n += 1)
        if (g.Viewports[n]->ID == id)
            return g.Viewports[n];
    return NULL;
}

ImGuiViewport* ImGui::FindViewportByPlatformHandle(void* platform_handle)
{
    ImGuiContext& g = *GImGui;
    for (int i = 0; i != g.Viewports.Size; i += 1)
        if (g.Viewports[i]->PlatformHandle == platform_handle)
            return g.Viewports[i];
    return NULL;
}

void ImGui::SetCurrentViewport(ImGuiWindow* current_window, ImGuiViewportP* viewport)
{
    ImGuiContext& g = *GImGui;
    (void)current_window;

    if (viewport)
        viewport->LastFrameActive = g.FrameCount;
    if (g.CurrentViewport == viewport)
        return;
    g.CurrentDpiScale = viewport ? viewport->DpiScale : 1.0;
    g.CurrentViewport = viewport;
    //IMGUI_DEBUG_LOG_VIEWPORT("[viewport] SetCurrentViewport changed '%s' 0x%08X\n", current_window ? current_window->name : NULL, viewport ? viewport->id : 0);

    // Notify platform layer of viewport changes
    // FIXME-DPI: This is only currently used for experimenting with handling of multiple DPI
    if (g.CurrentViewport && g.PlatformIO.Platform_OnChangedViewport)
        g.PlatformIO.Platform_OnChangedViewport(g.CurrentViewport);
}

void ImGui::SetWindowViewport(ImGuiWindow* window, ImGuiViewportP* viewport)
{
    // Abandon viewport
    if (window.viewport_owned && window.viewport->Window == window)
        window.viewport->Size = DimgVec2D::new(0.0, 0.0);

    window.viewport = viewport;
    window.ViewportId = viewport->ID;
    window.viewport_owned = (viewport->Window == window);
}

static bool ImGui::GetWindowAlwaysWantOwnViewport(ImGuiWindow* window)
{
    // Tooltips and menus are not automatically forced into their own viewport when the NoMerge flag is set, however the multiplication of viewports makes them more likely to protrude and create their own.
    ImGuiContext& g = *GImGui;
    if (g.io.ConfigViewportsNoAutoMerge || (window.WindowClass.ViewportFlagsOverrideSet & ImGuiViewportFlags_NoAutoMerge))
        if (g.config_flags_curr_frame & ConfigFlags::ViewportsEnable)
            if (!window.DockIsActive)
                if ((window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_ChildMenu | ImGuiWindowFlags_Tooltip)) == 0)
                    if ((window.Flags & ImGuiWindowFlags_Popup) == 0 || (window.Flags & ImGuiWindowFlags_Modal) != 0)
                        return true;
    return false;
}

static bool ImGui::update_try_merge_window_into_host_viewport(ImGuiWindow* window, ImGuiViewportP* viewport)
{
    ImGuiContext& g = *GImGui;
    if (window.viewport == viewport)
        return false;
    if ((viewport.flags & ImGuiViewportFlags_CanHostOtherWindows) == 0)
        return false;
    if ((viewport.flags & ImGuiViewportFlags_Minimized) != 0)
        return false;
    if (!viewport->GetMainRect().Contains(window.Rect()))
        return false;
    if (GetWindowAlwaysWantOwnViewport(window))
        return false;

    // FIXME: Can't use g.windows_focus_order[] for root windows only as we care about Z order. If we maintained a DisplayOrder along with focus_order we could..
    for (int n = 0; n < g.Windows.Size; n += 1)
    {
        ImGuiWindow* window_behind = g.Windows[n];
        if (window_behind == window)
            break;
        if (window_behind->WasActive && window_behind->ViewportOwned && !(window_behind.flags & ImGuiWindowFlags_ChildWindow))
            if (window_behind->Viewport->GetMainRect().Overlaps(window.Rect()))
                return false;
    }

    // Move to the existing viewport, Move child/hosted windows as well (FIXME-OPT: iterate child)
    ImGuiViewportP* old_viewport = window.viewport;
    if (window.viewport_owned)
        for (int n = 0; n < g.Windows.Size; n += 1)
            if (g.Windows[n]->Viewport == old_viewport)
                SetWindowViewport(g.Windows[n], viewport);
    SetWindowViewport(window, viewport);
    BringWindowToDisplayFront(window);

    return true;
}

// FIXME: handle 0 to N host viewports
static bool ImGui::UpdateTryMergeWindowIntoHostViewports(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    return update_try_merge_window_into_host_viewport(window, g.Viewports[0]);
}

// Translate Dear ImGui windows when a Host viewport has been moved
// (This additionally keeps windows at the same place when ConfigFlags::ViewportsEnable is toggled!)
void ImGui::TranslateWindowsInViewport(ImGuiViewportP* viewport, const Vector2D& old_pos, const Vector2D& new_pos)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(viewport->Window == NULL && (viewport.flags & ImGuiViewportFlags_CanHostOtherWindows));

    // 1) We test if ConfigFlags::ViewportsEnable was just toggled, which allows us to conveniently
    // translate imgui windows from OS-window-local to absolute coordinates or vice-versa.
    // 2) If it's not going to fit into the new size, keep it at same absolute position.
    // One problem with this is that most Win32 applications doesn't update their render while dragging,
    // and so the window will appear to teleport when releasing the mouse.
    const bool translate_all_windows = (g.config_flags_curr_frame & ConfigFlags::ViewportsEnable) != (g.ConfigFlagsLastFrame & ConfigFlags::ViewportsEnable);
    ImRect test_still_fit_rect(old_pos, old_pos + viewport->Size);
    Vector2D delta_pos = new_pos - old_pos;
    for (int window_n = 0; window_n < g.Windows.Size; window_n += 1) // FIXME-OPT
        if (translate_all_windows || (g.Windows[window_n]->Viewport == viewport && test_still_fit_rect.Contains(g.Windows[window_n]->Rect())))
            TranslateWindow(g.Windows[window_n], delta_pos);
}

// scale all windows (position, size). Use when e.g. changing DPI. (This is a lossy operation!)
void ImGui::ScaleWindowsInViewport(ImGuiViewportP* viewport, float scale)
{
    ImGuiContext& g = *GImGui;
    if (viewport->Window)
    {
        ScaleWindow(viewport->Window, scale);
    }
    else
    {
        for (int i = 0; i != g.Windows.Size; i += 1)
            if (g.Windows[i]->Viewport == viewport)
                ScaleWindow(g.Windows[i], scale);
    }
}

// If the backend doesn't set mouse_last_hovered_viewport or doesn't honor ViewportFlags::NoInputs, we do a search ourselves.
// A) It won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
// B) It requires Platform_GetWindowFocus to be implemented by backend.
ImGuiViewportP* ImGui::FindHoveredViewportFromPlatformWindowStack(const Vector2D& mouse_platform_pos)
{
    ImGuiContext& g = *GImGui;
    ImGuiViewportP* best_candidate = NULL;
    for (int n = 0; n < g.Viewports.Size; n += 1)
    {
        ImGuiViewportP* viewport = g.Viewports[n];
        if (!(viewport.flags & (ViewportFlags::NoInputs | ImGuiViewportFlags_Minimized)) && viewport->GetMainRect().Contains(mouse_platform_pos))
            if (best_candidate == NULL || best_candidate->LastFrontMostStampCount < viewport->LastFrontMostStampCount)
                best_candidate = viewport;
    }
    return best_candidate;
}

// Update viewports and monitor infos
// Note that this is running even if 'ConfigFlags::ViewportsEnable' is not set, in order to clear unused viewports (if any) and update monitor info.
static void ImGui::UpdateViewportsNewFrame()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.PlatformIO.Viewports.Size <= g.Viewports.Size);

    // Update Minimized status (we need it first in order to decide if we'll apply pos/size of the main viewport)
    const bool viewports_enabled = (g.config_flags_curr_frame & ConfigFlags::ViewportsEnable) != 0;
    if (viewports_enabled)
    {
        for (int n = 0; n < g.Viewports.Size; n += 1)
        {
            ImGuiViewportP* viewport = g.Viewports[n];
            const bool platform_funcs_available = viewport->PlatformWindowCreated;
            if (g.PlatformIO.Platform_GetWindowMinimized && platform_funcs_available)
            {
                bool minimized = g.PlatformIO.Platform_GetWindowMinimized(viewport);
                if (minimized)
                    viewport.flags |= ImGuiViewportFlags_Minimized;
                else
                    viewport.flags &= ~ImGuiViewportFlags_Minimized;
            }
        }
    }

    // Create/update main viewport with current platform position.
    // FIXME-VIEWPORT: size is driven by backend/user code for backward-compatibility but we should aim to make this more consistent.
    ImGuiViewportP* main_viewport = g.Viewports[0];
    IM_ASSERT(main_viewport->ID == IMGUI_VIEWPORT_DEFAULT_ID);
    IM_ASSERT(main_viewport->Window == NULL);
    Vector2D main_viewport_pos = viewports_enabled ? g.PlatformIO.Platform_GetWindowPos(main_viewport) : DimgVec2D::new(0.0, 0.0);
    Vector2D main_viewport_size = g.io.DisplaySize;
    if (viewports_enabled && (main_viewport.flags & ImGuiViewportFlags_Minimized))
    {
        main_viewport_pos = main_viewport.pos;    // Preserve last pos/size when minimized (FIXME: We don't do the same for size outside of the viewport path)
        main_viewport_size = main_viewport->Size;
    }
    AddUpdateViewport(NULL, IMGUI_VIEWPORT_DEFAULT_ID, main_viewport_pos, main_viewport_size, ImGuiViewportFlags_OwnedByApp | ImGuiViewportFlags_CanHostOtherWindows);

    g.CurrentDpiScale = 0.0;
    g.CurrentViewport = NULL;
    g.mouse_viewport = NULL;
    for (int n = 0; n < g.Viewports.Size; n += 1)
    {
        ImGuiViewportP* viewport = g.Viewports[n];
        viewport->Idx = n;

        // Erase unused viewports
        if (n > 0 && viewport->LastFrameActive < g.FrameCount - 2)
        {
            DestroyViewport(viewport);
            n--;
            continue;
        }

        const bool platform_funcs_available = viewport->PlatformWindowCreated;
        if (viewports_enabled)
        {
            // Update Position and size (from Platform Window to ImGui) if requested.
            // We do it early in the frame instead of waiting for UpdatePlatformWindows() to avoid a frame of lag when moving/resizing using OS facilities.
            if (!(viewport.flags & ImGuiViewportFlags_Minimized) && platform_funcs_available)
            {
                // viewport->work_pos and work_size will be updated below
                if (viewport->PlatformRequestMove)
                    viewport.pos = viewport->LastPlatformPos = g.PlatformIO.Platform_GetWindowPos(viewport);
                if (viewport->PlatformRequestResize)
                    viewport->Size = viewport->LastPlatformSize = g.PlatformIO.Platform_GetWindowSize(viewport);
            }
        }

        // Update/copy monitor info
        UpdateViewportPlatformMonitor(viewport);

        // Lock down space taken by menu bars and status bars, reset the offset for functions like BeginMainMenuBar() to alter them again.
        viewport->WorkOffsetMin = viewport->BuildWorkOffsetMin;
        viewport->WorkOffsetMax = viewport->BuildWorkOffsetMax;
        viewport->BuildWorkOffsetMin = viewport->BuildWorkOffsetMax = DimgVec2D::new(0.0, 0.0);
        viewport.update_work_rect();

        // Reset alpha every frame. Users of transparency (docking) needs to request a lower alpha back.
        viewport->Alpha = 1.0;

        // Translate Dear ImGui windows when a Host viewport has been moved
        // (This additionally keeps windows at the same place when ConfigFlags::ViewportsEnable is toggled!)
        const Vector2D viewport_delta_pos = viewport.pos - viewport->LastPos;
        if ((viewport.flags & ImGuiViewportFlags_CanHostOtherWindows) && (viewport_delta_pos.x != 0.0 || viewport_delta_pos.y != 0.0))
            TranslateWindowsInViewport(viewport, viewport->LastPos, viewport.pos);

        // Update DPI scale
        float new_dpi_scale;
        if (g.PlatformIO.Platform_GetWindowDpiScale && platform_funcs_available)
            new_dpi_scale = g.PlatformIO.Platform_GetWindowDpiScale(viewport);
        else if (viewport->PlatformMonitor != -1)
            new_dpi_scale = g.PlatformIO.Monitors[viewport->PlatformMonitor].DpiScale;
        else
            new_dpi_scale = (viewport->DpiScale != 0.0) ? viewport->DpiScale : 1.0;
        if (viewport->DpiScale != 0.0 && new_dpi_scale != viewport->DpiScale)
        {
            float scale_factor = new_dpi_scale / viewport->DpiScale;
            if (g.io.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleViewports)
                ScaleWindowsInViewport(viewport, scale_factor);
            //if (viewport == GetMainViewport())
            //    g.PlatformInterface.SetWindowSize(viewport, viewport->size * scale_factor);

            // scale our window moving pivot so that the window will rescale roughly around the mouse position.
            // FIXME-VIEWPORT: This currently creates a resizing feedback loop when a window is straddling a DPI transition border.
            // (Minor: since our sizes do not perfectly linearly scale, deferring the click offset scale until we know the actual window scale ratio may get us slightly more precise mouse positioning.)
            //if (g.moving_window != NULL && g.moving_window->viewport == viewport)
            //    g.ActiveIdClickOffset = ImFloor(g.ActiveIdClickOffset * scale_factor);
        }
        viewport->DpiScale = new_dpi_scale;
    }

    // Update fallback monitor
    if (g.PlatformIO.Monitors.Size == 0)
    {
        ImGuiPlatformMonitor* monitor = &g.FallbackMonitor;
        monitor->MainPos = main_viewport.pos;
        monitor->MainSize = main_viewport->Size;
        monitor->WorkPos = main_viewport->WorkPos;
        monitor->WorkSize = main_viewport->WorkSize;
        monitor->DpiScale = main_viewport->DpiScale;
    }

    if (!viewports_enabled)
    {
        g.mouse_viewport = main_viewport;
        return;
    }

    // Mouse handling: decide on the actual mouse viewport for this frame between the active/focused viewport and the hovered viewport.
    // Note that 'viewport_hovered' should skip over any viewport that has the ViewportFlags::NoInputs flags set.
    ImGuiViewportP* viewport_hovered = NULL;
    if (g.io.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport)
    {
        viewport_hovered = g.io.MouseHoveredViewport ? (ImGuiViewportP*)FindViewportByID(g.io.MouseHoveredViewport) : NULL;
        if (viewport_hovered && (viewport_hovered.flags & ViewportFlags::NoInputs))
            viewport_hovered = FindHoveredViewportFromPlatformWindowStack(g.io.MousePos); // Backend failed to handle _NoInputs viewport: revert to our fallback.
    }
    else
    {
        // If the backend doesn't know how to honor ViewportFlags::NoInputs, we do a search ourselves. Note that this search:
        // A) won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
        // B) won't take account of how the backend apply parent<>child relationship to secondary viewports, which affects their Z order.
        // C) uses LastFrameAsRefViewport as a flawed replacement for the last time a window was focused (we could/should fix that by introducing Focus functions in platform_io)
        viewport_hovered = FindHoveredViewportFromPlatformWindowStack(g.io.MousePos);
    }
    if (viewport_hovered != NULL)
        g.MouseLastHoveredViewport = viewport_hovered;
    else if (g.MouseLastHoveredViewport == NULL)
        g.MouseLastHoveredViewport = g.Viewports[0];

    // Update mouse reference viewport
    // (when moving a window we aim at its viewport, but this will be overwritten below if we go in drag and drop mode)
    // (MovingViewport->viewport will be NULL in the rare situation where the window disappared while moving, set UpdateMouseMovingWindowNewFrame() for details)
    if (g.moving_window && g.moving_window->Viewport)
        g.mouse_viewport = g.moving_window->Viewport;
    else
        g.mouse_viewport = g.MouseLastHoveredViewport;

    // When dragging something, always refer to the last hovered viewport.
    // - when releasing a moving window we will revert to aiming behind (at viewport_hovered)
    // - when we are between viewports, our dragged preview will tend to show in the last viewport _even_ if we don't have tooltips in their viewports (when lacking monitor info)
    // - consider the case of holding on a menu item to browse child menus: even thou a mouse button is held, there's no active id because menu items only react on mouse release.
    // FIXME-VIEWPORT: This is essentially broken, when ImGuiBackendFlags_HasMouseHoveredViewport is set we want to trust when viewport_hovered==NULL and use that.
    const bool is_mouse_dragging_with_an_expected_destination = g.DragDropActive;
    if (is_mouse_dragging_with_an_expected_destination && viewport_hovered == NULL)
        viewport_hovered = g.MouseLastHoveredViewport;
    if (is_mouse_dragging_with_an_expected_destination || g.active_id == 0 || !IsAnyMouseDown())
        if (viewport_hovered != NULL && viewport_hovered != g.mouse_viewport && !(viewport_hovered.flags & ViewportFlags::NoInputs))
            g.mouse_viewport = viewport_hovered;

    IM_ASSERT(g.mouse_viewport != NULL);
}

// Update user-facing viewport list (g.viewports -> g.platform_io.viewports after filtering out some)
static void ImGui::UpdateViewportsEndFrame()
{
    ImGuiContext& g = *GImGui;
    g.PlatformIO.Viewports.resize(0);
    for (int i = 0; i < g.Viewports.Size; i += 1)
    {
        ImGuiViewportP* viewport = g.Viewports[i];
        viewport->LastPos = viewport.pos;
        if (viewport->LastFrameActive < g.FrameCount || viewport->Size.x <= 0.0 || viewport->Size.y <= 0.0)
            if (i > 0) // Always include main viewport in the list
                continue;
        if (viewport->Window && !IsWindowActiveAndVisible(viewport->Window))
            continue;
        if (i > 0)
            IM_ASSERT(viewport->Window != NULL);
        g.PlatformIO.Viewports.push_back(viewport);
    }
    g.Viewports[0]->ClearRequestFlags(); // clear main viewport flags because UpdatePlatformWindows() won't do it and may not even be called
}

// FIXME: We should ideally refactor the system to call this every frame (we currently don't)
ImGuiViewportP* ImGui::AddUpdateViewport(ImGuiWindow* window, ImGuiID id, const Vector2D& pos, const Vector2D& size, ImGuiViewportFlags flags)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(id != 0);

    flags |= ImGuiViewportFlags_IsPlatformWindow;
    if (window != NULL)
    {
        if (g.moving_window && g.moving_window->RootWindowDockTree == window)
            flags |= ViewportFlags::NoInputs | ImGuiViewportFlags_NoFocusOnAppearing;
        if ((window.Flags & ImGuiWindowFlags_NoMouseInputs) && (window.Flags & ImGuiWindowFlags_NoNavInputs))
            flags |= ViewportFlags::NoInputs;
        if (window.Flags & ImGuiWindowFlags_NoFocusOnAppearing)
            flags |= ImGuiViewportFlags_NoFocusOnAppearing;
    }

    ImGuiViewportP* viewport = (ImGuiViewportP*)FindViewportByID(id);
    if (viewport)
    {
        // Always update for main viewport as we are already pulling correct platform pos/size (see #4900)
        if (!viewport->PlatformRequestMove || viewport->ID == IMGUI_VIEWPORT_DEFAULT_ID)
            viewport.pos = pos;
        if (!viewport->PlatformRequestResize || viewport->ID == IMGUI_VIEWPORT_DEFAULT_ID)
            viewport->Size = size;
        viewport.flags = flags | (viewport.flags & ImGuiViewportFlags_Minimized); // Preserve existing flags
    }
    else
    {
        // New viewport
        viewport = IM_NEW(ImGuiViewportP)();
        viewport->ID = id;
        viewport->Idx = g.Viewports.Size;
        viewport.pos = viewport->LastPos = pos;
        viewport->Size = size;
        viewport.flags = flags;
        UpdateViewportPlatformMonitor(viewport);
        g.Viewports.push_back(viewport);
        IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Add viewport %08X '%s'\n", id, window ? window.Name : "<NULL>");

        // We normally setup for all viewports in NewFrame() but here need to handle the mid-frame creation of a new viewport.
        // We need to extend the fullscreen clip rect so the OverlayDrawList clip is correct for that the first frame
        g.DrawListSharedData.ClipRectFullscreen.x = ImMin(g.DrawListSharedData.ClipRectFullscreen.x, viewport.pos.x);
        g.DrawListSharedData.ClipRectFullscreen.y = ImMin(g.DrawListSharedData.ClipRectFullscreen.y, viewport.pos.y);
        g.DrawListSharedData.ClipRectFullscreen.z = ImMax(g.DrawListSharedData.ClipRectFullscreen.z, viewport.pos.x + viewport->Size.x);
        g.DrawListSharedData.ClipRectFullscreen.w = ImMax(g.DrawListSharedData.ClipRectFullscreen.w, viewport.pos.y + viewport->Size.y);

        // Store initial dpi_scale before the OS platform window creation, based on expected monitor data.
        // This is so we can select an appropriate font size on the first frame of our window lifetime
        if (viewport->PlatformMonitor != -1)
            viewport->DpiScale = g.PlatformIO.Monitors[viewport->PlatformMonitor].DpiScale;
    }

    viewport->Window = window;
    viewport->LastFrameActive = g.FrameCount;
    viewport.update_work_rect();
    IM_ASSERT(window == NULL || viewport->ID == window.ID);

    if (window != NULL)
        window.viewport_owned = true;

    return viewport;
}

static void ImGui::DestroyViewport(ImGuiViewportP* viewport)
{
    // clear references to this viewport in windows (window->viewport_id becomes the master data)
    ImGuiContext& g = *GImGui;
    for (int window_n = 0; window_n < g.Windows.Size; window_n += 1)
    {
        ImGuiWindow* window = g.Windows[window_n];
        if (window.viewport != viewport)
            continue;
        window.viewport = NULL;
        window.viewport_owned = false;
    }
    if (viewport == g.MouseLastHoveredViewport)
        g.MouseLastHoveredViewport = NULL;

    // Destroy
    IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Delete viewport %08X '%s'\n", viewport->ID, viewport->Window ? viewport->Window->Name : "n/a");
    DestroyPlatformWindow(viewport); // In most circumstances the platform window will already be destroyed here.
    IM_ASSERT(g.PlatformIO.Viewports.contains(viewport) == false);
    IM_ASSERT(g.Viewports[viewport->Idx] == viewport);
    g.Viewports.erase(g.Viewports.Data + viewport->Idx);
    IM_DELETE(viewport);
}

// FIXME-VIEWPORT: This is all super messy and ought to be clarified or rewritten.
static void ImGui::WindowSelectViewport(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindowFlags flags = window.Flags;
    window.ViewportAllowPlatformMonitorExtend = -1;

    // Restore main viewport if multi-viewport is not supported by the backend
    ImGuiViewportP* main_viewport = (ImGuiViewportP*)(void*)GetMainViewport();
    if (!(g.config_flags_curr_frame & ConfigFlags::ViewportsEnable))
    {
        SetWindowViewport(window, main_viewport);
        return;
    }
    window.viewport_owned = false;

    // appearing popups reset their viewport so they can inherit again
    if ((flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && window.Appearing)
    {
        window.viewport = NULL;
        window.ViewportId = 0;
    }

    if ((g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasViewport) == 0)
    {
        // By default inherit from parent window
        if (window.viewport == NULL && window.ParentWindow && (!window.ParentWindow->IsFallbackWindow || window.ParentWindow->WasActive))
            window.viewport = window.ParentWindow->Viewport;

        // Attempt to restore saved viewport id (= window that hasn't been activated yet), try to restore the viewport based on saved 'window->viewport_pos' restored from .ini file
        if (window.viewport == NULL && window.ViewportId != 0)
        {
            window.viewport = (ImGuiViewportP*)FindViewportByID(window.ViewportId);
            if (window.viewport == NULL && window.ViewportPos.x != FLT_MAX && window.ViewportPos.y != FLT_MAX)
                window.viewport = AddUpdateViewport(window, window.ID, window.ViewportPos, window.Size, ImGuiViewportFlags_None);
        }
    }

    bool lock_viewport = false;
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasViewport)
    {
        // Code explicitly request a viewport
        window.viewport = (ImGuiViewportP*)FindViewportByID(g.NextWindowData.ViewportId);
        window.ViewportId = g.NextWindowData.ViewportId; // Store id even if viewport isn't resolved yet.
        lock_viewport = true;
    }
    else if ((flags & ImGuiWindowFlags_ChildWindow) || (flags & ImGuiWindowFlags_ChildMenu))
    {
        // Always inherit viewport from parent window
        if (window.DockNode && window.DockNode->HostWindow)
            IM_ASSERT(window.DockNode->HostWindow->Viewport == window.ParentWindow->Viewport);
        window.viewport = window.ParentWindow->Viewport;
    }
    else if (window.DockNode && window.DockNode->HostWindow)
    {
        // This covers the "always inherit viewport from parent window" case for when a window reattach to a node that was just created mid-frame
        window.viewport = window.DockNode->HostWindow->Viewport;
    }
    else if (flags & ImGuiWindowFlags_Tooltip)
    {
        window.viewport = g.mouse_viewport;
    }
    else if (GetWindowAlwaysWantOwnViewport(window))
    {
        window.viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_None);
    }
    else if (g.moving_window && g.moving_window->RootWindowDockTree == window && is_mouse_pos_valid())
    {
        if (window.viewport != NULL && window.viewport->Window == window)
            window.viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_None);
    }
    else
    {
        // merge into host viewport?
        // We cannot test window->viewport_owned as it set lower in the function.
        // Testing (g.active_id == 0 || g.active_id_allow_overlap) to avoid merging during a short-term widget interaction. Main intent was to avoid during resize (see #4212)
        bool try_to_merge_into_host_viewport = (window.viewport && window == window.viewport->Window && (g.active_id == 0 || g.ActiveIdAllowOverlap));
        if (try_to_merge_into_host_viewport)
            UpdateTryMergeWindowIntoHostViewports(window);
    }

    // Fallback: merge in default viewport if z-order matches, otherwise create a new viewport
    if (window.viewport == NULL)
        if (!update_try_merge_window_into_host_viewport(window, main_viewport))
            window.viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_None);

    // Mark window as allowed to protrude outside of its viewport and into the current monitor
    if (!lock_viewport)
    {
        if (flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup))
        {
            // We need to take account of the possibility that mouse may become invalid.
            // Popups/Tooltip always set viewport_allow_platform_monitor_extend so GetWindowAllowedExtentRect() will return full monitor bounds.
            Vector2D mouse_ref = (flags & ImGuiWindowFlags_Tooltip) ? g.io.MousePos : g.BeginPopupStack.back().OpenMousePos;
            bool use_mouse_ref = (g.NavDisableHighlight || !g.NavDisableMouseHover || !g.nav_window);
            bool mouse_valid = is_mouse_pos_valid(&mouse_ref);
            if ((window.Appearing || (flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_ChildMenu))) && (!use_mouse_ref || mouse_valid))
                window.ViewportAllowPlatformMonitorExtend = FindPlatformMonitorForPos((use_mouse_ref && mouse_valid) ? mouse_ref : NavCalcPreferredRefPos());
            else
                window.ViewportAllowPlatformMonitorExtend = window.viewport->PlatformMonitor;
        }
        else if (window.viewport && window != window.viewport->Window && window.viewport->Window && !(flags & ImGuiWindowFlags_ChildWindow) && window.DockNode == NULL)
        {
            // When called from Begin() we don't have access to a proper version of the hidden flag yet, so we replicate this code.
            const bool will_be_visible = (window.DockIsActive && !window.DockTabIsVisible) ? false : true;
            if ((window.Flags & ImGuiWindowFlags_DockNodeHost) && window.viewport->LastFrameActive < g.FrameCount && will_be_visible)
            {
                // Steal/transfer ownership
                IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Window '%s' steal viewport %08X from Window '%s'\n", window.Name, window.viewport->ID, window.viewport->Window->Name);
                window.viewport->Window = window;
                window.viewport->ID = window.ID;
                window.viewport->LastNameHash = 0;
            }
            else if (!UpdateTryMergeWindowIntoHostViewports(window)) // merge?
            {
                // New viewport
                window.viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_NoFocusOnAppearing);
            }
        }
        else if (window.ViewportAllowPlatformMonitorExtend < 0 && (flags & ImGuiWindowFlags_ChildWindow) == 0)
        {
            // Regular (non-child, non-popup) windows by default are also allowed to protrude
            // Child windows are kept contained within their parent.
            window.ViewportAllowPlatformMonitorExtend = window.viewport->PlatformMonitor;
        }
    }

    // Update flags
    window.viewport_owned = (window == window.viewport->Window);
    window.ViewportId = window.viewport->ID;

    // If the OS window has a title bar, hide our imgui title bar
    //if (window->viewport_owned && !(window->viewport->flags & ImGuiViewportFlags_NoDecoration))
    //    window->flags |= ImGuiWindowFlags_NoTitleBar;
}

void ImGui::WindowSyncOwnedViewport(ImGuiWindow* window, ImGuiWindow* parent_window_in_stack)
{
    ImGuiContext& g = *GImGui;

    bool viewport_rect_changed = false;

    // Synchronize window --> viewport in most situations
    // Synchronize viewport -> window in case the platform window has been moved or resized from the OS/WM
    if (window.viewport->PlatformRequestMove)
    {
        window.Pos = window.viewport.pos;
        MarkIniSettingsDirty(window);
    }
    else if (memcmp(&window.viewport.pos, &window.Pos, sizeof(window.Pos)) != 0)
    {
        viewport_rect_changed = true;
        window.viewport.pos = window.Pos;
    }

    if (window.viewport->PlatformRequestResize)
    {
        window.Size = window.SizeFull = window.viewport->Size;
        MarkIniSettingsDirty(window);
    }
    else if (memcmp(&window.viewport->Size, &window.Size, sizeof(window.Size)) != 0)
    {
        viewport_rect_changed = true;
        window.viewport->Size = window.Size;
    }
    window.viewport.update_work_rect();

    // The viewport may have changed monitor since the global update in UpdateViewportsNewFrame()
    // Either a SetNextWindowPos() call in the current frame or a set_window_pos() call in the previous frame may have this effect.
    if (viewport_rect_changed)
        UpdateViewportPlatformMonitor(window.viewport);

    // Update common viewport flags
    const ImGuiViewportFlags viewport_flags_to_clear = ImGuiViewportFlags_TopMost | ImGuiViewportFlags_NoTaskBarIcon | ImGuiViewportFlags_NoDecoration | ImGuiViewportFlags_NoRendererClear;
    ImGuiViewportFlags viewport_flags = window.viewport.flags & ~viewport_flags_to_clear;
    ImGuiWindowFlags window_flags = window.Flags;
    const bool is_modal = (window_flags & ImGuiWindowFlags_Modal) != 0;
    const bool is_short_lived_floating_window = (window_flags & (ImGuiWindowFlags_ChildMenu | ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup)) != 0;
    if (window_flags & ImGuiWindowFlags_Tooltip)
        viewport_flags |= ImGuiViewportFlags_TopMost;
    if ((g.io.ConfigViewportsNoTaskBarIcon || is_short_lived_floating_window) && !is_modal)
        viewport_flags |= ImGuiViewportFlags_NoTaskBarIcon;
    if (g.io.ConfigViewportsNoDecoration || is_short_lived_floating_window)
        viewport_flags |= ImGuiViewportFlags_NoDecoration;

    // Not correct to set modal as topmost because:
    // - Because other popups can be stacked above a modal (e.g. combo box in a modal)
    // - ImGuiViewportFlags_TopMost is currently handled different in backends: in Win32 it is "appear top most" whereas in GLFW and SDL it is "stay topmost"
    //if (flags & ImGuiWindowFlags_Modal)
    //    viewport_flags |= ImGuiViewportFlags_TopMost;

    // For popups and menus that may be protruding out of their parent viewport, we enable _NoFocusOnClick so that clicking on them
    // won't steal the OS focus away from their parent window (which may be reflected in OS the title bar decoration).
    // Setting _NoFocusOnClick would technically prevent us from bringing back to front in case they are being covered by an OS window from a different app,
    // but it shouldn't be much of a problem considering those are already popups that are closed when clicking elsewhere.
    if (is_short_lived_floating_window && !is_modal)
        viewport_flags |= ImGuiViewportFlags_NoFocusOnAppearing | ImGuiViewportFlags_NoFocusOnClick;

    // We can overwrite viewport flags using ImGuiWindowClass (advanced users)
    if (window.WindowClass.ViewportFlagsOverrideSet)
        viewport_flags |= window.WindowClass.ViewportFlagsOverrideSet;
    if (window.WindowClass.ViewportFlagsOverrideClear)
        viewport_flags &= ~window.WindowClass.ViewportFlagsOverrideClear;

    // We can also tell the backend that clearing the platform window won't be necessary,
    // as our window background is filling the viewport and we have disabled BgAlpha.
    // FIXME: Work on support for per-viewport transparency (#2766)
    if (!(window_flags & ImGuiWindowFlags_NoBackground))
        viewport_flags |= ImGuiViewportFlags_NoRendererClear;

    window.viewport.flags = viewport_flags;

    // Update parent viewport id
    // (the !is_fallback_window test mimic the one done in WindowSelectViewport())
    if (window.WindowClass.ParentViewportId != (ImGuiID)-1)
        window.viewport->ParentViewportId = window.WindowClass.ParentViewportId;
    else if ((window_flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && parent_window_in_stack && (!parent_window_in_stack->IsFallbackWindow || parent_window_in_stack->WasActive))
        window.viewport->ParentViewportId = parent_window_in_stack->Viewport->ID;
    else
        window.viewport->ParentViewportId = g.io.ConfigViewportsNoDefaultParent ? 0 : IMGUI_VIEWPORT_DEFAULT_ID;
}

// Called by user at the end of the main loop, after EndFrame()
// This will handle the creation/update of all OS windows via function defined in the ImGuiPlatformIO api.
void ImGui::UpdatePlatformWindows()
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.FrameCountEnded == g.FrameCount && "Forgot to call Render() or EndFrame() before UpdatePlatformWindows()?");
    IM_ASSERT(g.FrameCountPlatformEnded < g.FrameCount);
    g.FrameCountPlatformEnded = g.FrameCount;
    if (!(g.config_flags_curr_frame & ConfigFlags::ViewportsEnable))
        return;

    // Create/resize/destroy platform windows to match each active viewport.
    // Skip the main viewport (index 0), which is always fully handled by the application!
    for (int i = 1; i < g.Viewports.Size; i += 1)
    {
        ImGuiViewportP* viewport = g.Viewports[i];

        // Destroy platform window if the viewport hasn't been submitted or if it is hosting a hidden window
        // (the implicit/fallback Debug##Default window will be registering its viewport then be disabled, causing a dummy DestroyPlatformWindow to be made each frame)
        bool destroy_platform_window = false;
        destroy_platform_window |= (viewport->LastFrameActive < g.FrameCount - 1);
        destroy_platform_window |= (viewport->Window && !IsWindowActiveAndVisible(viewport->Window));
        if (destroy_platform_window)
        {
            DestroyPlatformWindow(viewport);
            continue;
        }

        // New windows that appears directly in a new viewport won't always have a size on their first frame
        if (viewport->LastFrameActive < g.FrameCount || viewport->Size.x <= 0 || viewport->Size.y <= 0)
            continue;

        // Create window
        bool is_new_platform_window = (viewport->PlatformWindowCreated == false);
        if (is_new_platform_window)
        {
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Create Platform Window %08X '%s'\n", viewport->ID, viewport->Window ? viewport->Window->Name : "n/a");
            g.PlatformIO.Platform_CreateWindow(viewport);
            if (g.PlatformIO.Renderer_CreateWindow != NULL)
                g.PlatformIO.Renderer_CreateWindow(viewport);
            viewport->LastNameHash = 0;
            viewport->LastPlatformPos = viewport->LastPlatformSize = DimgVec2D::new(FLT_MAX, FLT_MAX); // By clearing those we'll enforce a call to Platform_SetWindowPos/size below, before Platform_ShowWindow (FIXME: Is that necessary?)
            viewport->LastRendererSize = viewport->Size;                                       // We don't need to call Renderer_SetWindowSize() as it is expected Renderer_CreateWindow() already did it.
            viewport->PlatformWindowCreated = true;
        }

        // Apply Position and size (from ImGui to Platform/Renderer backends)
        if ((viewport->LastPlatformPos.x != viewport.pos.x || viewport->LastPlatformPos.y != viewport.pos.y) && !viewport->PlatformRequestMove)
            g.PlatformIO.Platform_SetWindowPos(viewport, viewport.pos);
        if ((viewport->LastPlatformSize.x != viewport->Size.x || viewport->LastPlatformSize.y != viewport->Size.y) && !viewport->PlatformRequestResize)
            g.PlatformIO.Platform_SetWindowSize(viewport, viewport->Size);
        if ((viewport->LastRendererSize.x != viewport->Size.x || viewport->LastRendererSize.y != viewport->Size.y) && g.PlatformIO.Renderer_SetWindowSize)
            g.PlatformIO.Renderer_SetWindowSize(viewport, viewport->Size);
        viewport->LastPlatformPos = viewport.pos;
        viewport->LastPlatformSize = viewport->LastRendererSize = viewport->Size;

        // Update title bar (if it changed)
        if (ImGuiWindow* window_for_title = GetWindowForTitleDisplay(viewport->Window))
        {
            const char* title_begin = window_for_title->Name;
            char* title_end = (char*)(intptr_t)FindRenderedTextEnd(title_begin);
            const ImGuiID title_hash = ImHashStr(title_begin, title_end - title_begin);
            if (viewport->LastNameHash != title_hash)
            {
                char title_end_backup_c = *title_end;
                *title_end = 0; // Cut existing buffer short instead of doing an alloc/free, no small gain.
                g.PlatformIO.Platform_SetWindowTitle(viewport, title_begin);
                *title_end = title_end_backup_c;
                viewport->LastNameHash = title_hash;
            }
        }

        // Update alpha (if it changed)
        if (viewport->LastAlpha != viewport->Alpha && g.PlatformIO.Platform_SetWindowAlpha)
            g.PlatformIO.Platform_SetWindowAlpha(viewport, viewport->Alpha);
        viewport->LastAlpha = viewport->Alpha;

        // Optional, general purpose call to allow the backend to perform general book-keeping even if things haven't changed.
        if (g.PlatformIO.Platform_UpdateWindow)
            g.PlatformIO.Platform_UpdateWindow(viewport);

        if (is_new_platform_window)
        {
            // On startup ensure new platform window don't steal focus (give it a few frames, as nested contents may lead to viewport being created a few frames late)
            if (g.FrameCount < 3)
                viewport.flags |= ImGuiViewportFlags_NoFocusOnAppearing;

            // Show window
            g.PlatformIO.Platform_ShowWindow(viewport);

            // Even without focus, we assume the window becomes front-most.
            // This is useful for our platform z-order heuristic when io.mouse_hovered_viewport is not available.
            if (viewport->LastFrontMostStampCount != g.ViewportFrontMostStampCount)
                viewport->LastFrontMostStampCount = g.ViewportFrontMostStampCount += 1;
            }

        // clear request flags
        viewport->ClearRequestFlags();
    }

    // Update our implicit z-order knowledge of platform windows, which is used when the backend cannot provide io.mouse_hovered_viewport.
    // When setting Platform_GetWindowFocus, it is expected that the platform backend can handle calls without crashing if it doesn't have data stored.
    // FIXME-VIEWPORT: We should use this information to also set dear imgui-side focus, allowing us to handle os-level alt+tab.
    if (g.PlatformIO.Platform_GetWindowFocus != NULL)
    {
        ImGuiViewportP* focused_viewport = NULL;
        for (int n = 0; n < g.Viewports.Size && focused_viewport == NULL; n += 1)
        {
            ImGuiViewportP* viewport = g.Viewports[n];
            if (viewport->PlatformWindowCreated)
                if (g.PlatformIO.Platform_GetWindowFocus(viewport))
                    focused_viewport = viewport;
        }

        // Store a tag so we can infer z-order easily from all our windows
        // We compare platform_last_focused_viewport_id so newly created viewports with _NoFocusOnAppearing flag
        // will keep the front most stamp instead of losing it back to their parent viewport.
        if (focused_viewport && g.PlatformLastFocusedViewportId != focused_viewport->ID)
        {
            if (focused_viewport->LastFrontMostStampCount != g.ViewportFrontMostStampCount)
                focused_viewport->LastFrontMostStampCount = g.ViewportFrontMostStampCount += 1;
            g.PlatformLastFocusedViewportId = focused_viewport->ID;
        }
    }
}

// This is a default/basic function for performing the rendering/swap of multiple Platform windows.
// Custom renderers may prefer to not call this function at all, and instead iterate the publicly exposed platform data and handle rendering/sync themselves.
// The Render/Swap functions stored in ImGuiPlatformIO are merely here to allow for this helper to exist, but you can do it yourself:
//
//    ImGuiPlatformIO& platform_io = ImGui::GetPlatformIO();
//    for (int i = 1; i < platform_io.viewports.size; i++)
//        if ((platform_io.viewports[i]->flags & ImGuiViewportFlags_Minimized) == 0)
//            MyRenderFunction(platform_io.viewports[i], my_args);
//    for (int i = 1; i < platform_io.viewports.size; i++)
//        if ((platform_io.viewports[i]->flags & ImGuiViewportFlags_Minimized) == 0)
//            MySwapBufferFunction(platform_io.viewports[i], my_args);
//
void ImGui::RenderPlatformWindowsDefault(void* platform_render_arg, void* renderer_render_arg)
{
    // Skip the main viewport (index 0), which is always fully handled by the application!
    ImGuiPlatformIO& platform_io = ImGui::GetPlatformIO();
    for (int i = 1; i < platform_io.Viewports.Size; i += 1)
    {
        ImGuiViewport* viewport = platform_io.Viewports[i];
        if (viewport.flags & ImGuiViewportFlags_Minimized)
            continue;
        if (platform_io.Platform_RenderWindow) platform_io.Platform_RenderWindow(viewport, platform_render_arg);
        if (platform_io.Renderer_RenderWindow) platform_io.Renderer_RenderWindow(viewport, renderer_render_arg);
    }
    for (int i = 1; i < platform_io.Viewports.Size; i += 1)
    {
        ImGuiViewport* viewport = platform_io.Viewports[i];
        if (viewport.flags & ImGuiViewportFlags_Minimized)
            continue;
        if (platform_io.Platform_SwapBuffers) platform_io.Platform_SwapBuffers(viewport, platform_render_arg);
        if (platform_io.Renderer_SwapBuffers) platform_io.Renderer_SwapBuffers(viewport, renderer_render_arg);
    }
}

static int ImGui::FindPlatformMonitorForPos(const Vector2D& pos)
{
    ImGuiContext& g = *GImGui;
    for (int monitor_n = 0; monitor_n < g.PlatformIO.Monitors.Size; monitor_n += 1)
    {
        const ImGuiPlatformMonitor& monitor = g.PlatformIO.Monitors[monitor_n];
        if (ImRect(monitor.MainPos, monitor.MainPos + monitor.MainSize).Contains(pos))
            return monitor_n;
    }
    return -1;
}

// Search for the monitor with the largest intersection area with the given rectangle
// We generally try to avoid searching loops but the monitor count should be very small here
// FIXME-OPT: We could test the last monitor used for that viewport first, and early
static int ImGui::FindPlatformMonitorForRect(const ImRect& rect)
{
    ImGuiContext& g = *GImGui;

    const int monitor_count = g.PlatformIO.Monitors.Size;
    if (monitor_count <= 1)
        return monitor_count - 1;

    // Use a minimum threshold of 1.0 so a zero-sized rect won't false positive, and will still find the correct monitor given its position.
    // This is necessary for tooltips which always resize down to zero at first.
    const float surface_threshold = ImMax(rect.GetWidth() * rect.GetHeight() * 0.5, 1.0);
    int best_monitor_n = -1;
    float best_monitor_surface = 0.001;

    for (int monitor_n = 0; monitor_n < g.PlatformIO.Monitors.Size && best_monitor_surface < surface_threshold; monitor_n += 1)
    {
        const ImGuiPlatformMonitor& monitor = g.PlatformIO.Monitors[monitor_n];
        const ImRect monitor_rect = ImRect(monitor.MainPos, monitor.MainPos + monitor.MainSize);
        if (monitor_rect.Contains(rect))
            return monitor_n;
        ImRect overlapping_rect = rect;
        overlapping_rect.ClipWithFull(monitor_rect);
        float overlapping_surface = overlapping_rect.GetWidth() * overlapping_rect.GetHeight();
        if (overlapping_surface < best_monitor_surface)
            continue;
        best_monitor_surface = overlapping_surface;
        best_monitor_n = monitor_n;
    }
    return best_monitor_n;
}

// Update monitor from viewport rectangle (we'll use this info to clamp windows and save windows lost in a removed monitor)
static void ImGui::UpdateViewportPlatformMonitor(ImGuiViewportP* viewport)
{
    viewport->PlatformMonitor = (short)FindPlatformMonitorForRect(viewport->GetMainRect());
}

// Return value is always != NULL, but don't hold on it across frames.
const ImGuiPlatformMonitor* ImGui::GetViewportPlatformMonitor(ImGuiViewport* viewport_p)
{
    ImGuiContext& g = *GImGui;
    ImGuiViewportP* viewport = (ImGuiViewportP*)(void*)viewport_p;
    int monitor_idx = viewport->PlatformMonitor;
    if (monitor_idx >= 0 && monitor_idx < g.PlatformIO.Monitors.Size)
        return &g.PlatformIO.Monitors[monitor_idx];
    return &g.FallbackMonitor;
}

void ImGui::DestroyPlatformWindow(ImGuiViewportP* viewport)
{
    ImGuiContext& g = *GImGui;
    if (viewport->PlatformWindowCreated)
    {
        if (g.PlatformIO.Renderer_DestroyWindow)
            g.PlatformIO.Renderer_DestroyWindow(viewport);
        if (g.PlatformIO.Platform_DestroyWindow)
            g.PlatformIO.Platform_DestroyWindow(viewport);
        IM_ASSERT(viewport->RendererUserData == NULL && viewport->PlatformUserData == NULL);

        // Don't clear PlatformWindowCreated for the main viewport, as we initially set that up to true in Initialize()
        // The righter way may be to leave it to the backend to set this flag all-together, and made the flag public.
        if (viewport->ID != IMGUI_VIEWPORT_DEFAULT_ID)
            viewport->PlatformWindowCreated = false;
    }
    else
    {
        IM_ASSERT(viewport->RendererUserData == NULL && viewport->PlatformUserData == NULL && viewport->PlatformHandle == NULL);
    }
    viewport->RendererUserData = viewport->PlatformUserData = viewport->PlatformHandle = NULL;
    viewport->ClearRequestFlags();
}

void ImGui::DestroyPlatformWindows()
{
    // We call the destroy window on every viewport (including the main viewport, index 0) to give a chance to the backend
    // to clear any data they may have stored in e.g. PlatformUserData, renderer_user_data.
    // It is convenient for the platform backend code to store something in the main viewport, in order for e.g. the mouse handling
    // code to operator a consistent manner.
    // It is expected that the backend can handle calls to Renderer_DestroyWindow/Platform_DestroyWindow without
    // crashing if it doesn't have data stored.
    ImGuiContext& g = *GImGui;
    for (int i = 0; i < g.Viewports.Size; i += 1)
        DestroyPlatformWindow(g.Viewports[i]);
}


//-----------------------------------------------------------------------------
// [SECTION] DOCKING
//-----------------------------------------------------------------------------
// Docking: Internal Types
// Docking: Forward Declarations
// Docking: ImGuiDockContext
// Docking: ImGuiDockContext Docking/Undocking functions
// Docking: ImGuiDockNode
// Docking: ImGuiDockNode Tree manipulation functions
// Docking: Public Functions (SetWindowDock, DockSpace, DockSpaceOverViewport)
// Docking: Builder Functions
// Docking: Begin/End Support Functions (called from Begin/End)
// Docking: Settings
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Typical Docking call flow: (root level is generally public API):
//-----------------------------------------------------------------------------
// - NewFrame()                               new dear imgui frame
//    | DockContextNewFrameUpdateUndocking()  - process queued undocking requests
//    | - DockContextProcessUndockWindow()    - process one window undocking request
//    | - DockContextProcessUndockNode()      - process one whole node undocking request
//    | DockContextNewFrameUpdateUndocking()  - process queue docking requests, create floating dock nodes
//    | - update g.hovered_dock_node            - [debug] update node hovered by mouse
//    | - DockContextProcessDock()            - process one docking request
//    | - DockNodeUpdate()
//    |   - DockNodeUpdateForRootNode()
//    |     - DockNodeUpdateFlagsAndCollapse()
//    |     - DockNodeFindInfo()
//    |   - destroy unused node or tab bar
//    |   - create dock node host window
//    |      - Begin() etc.
//    |   - DockNodeStartMouseMovingWindow()
//    |   - DockNodeTreeUpdatePosSize()
//    |   - DockNodeTreeUpdateSplitter()
//    |   - draw node background
//    |   - DockNodeUpdateTabBar()            - create/update tab bar for a docking node
//    |     - DockNodeAddTabBar()
//    |     - DockNodeUpdateWindowMenu()
//    |     - DockNodeCalcTabBarLayout()
//    |     - BeginTabBarEx()
//    |     - TabItemEx() calls
//    |     - EndTabBar()
//    |   - BeginDockableDragDropTarget()
//    |      - DockNodeUpdate()               - recurse into child nodes...
//-----------------------------------------------------------------------------
// - DockSpace()                              user submit a dockspace into a window
//    | Begin(Child)                          - create a child window
//    | DockNodeUpdate()                      - call main dock node update function
//    | End(Child)
//    | ItemSize()
//-----------------------------------------------------------------------------
// - Begin()
//    | BeginDocked()
//    | BeginDockableDragDropSource()
//    | BeginDockableDragDropTarget()
//    | - DockNodePreviewDockRender()
//-----------------------------------------------------------------------------
// - EndFrame()
//    | DockContextEndFrame()
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Docking: Internal Types
//-----------------------------------------------------------------------------
// - ImGuiDockRequestType
// - ImGuiDockRequest
// - ImGuiDockPreviewData
// - ImGuiDockNodeSettings
// - ImGuiDockContext
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Docking: Forward Declarations
//-----------------------------------------------------------------------------
//
// namespace ImGui
// {
//     // ImGuiDockContext
//     static ImGuiDockNode*   DockContextAddNode(ImGuiContext* ctx, ImGuiID id);
//     static void             DockContextRemoveNode(ImGuiContext* ctx, ImGuiDockNode* node, bool merge_sibling_into_parent_node);
//     static void             DockContextQueueNotifyRemovedNode(ImGuiContext* ctx, ImGuiDockNode* node);
//     static void             DockContextProcessDock(ImGuiContext* ctx, ImGuiDockRequest* req);
//     static void             DockContextProcessUndockWindow(ImGuiContext* ctx, ImGuiWindow* window, bool clear_persistent_docking_ref = true);
//     static void             DockContextProcessUndockNode(ImGuiContext* ctx, ImGuiDockNode* node);
//     static void             DockContextPruneUnusedSettingsNodes(ImGuiContext* ctx);
//     static ImGuiDockNode*   DockContextFindNodeByID(ImGuiContext* ctx, ImGuiID id);
//     static ImGuiDockNode*   DockContextBindNodeToWindow(ImGuiContext* ctx, ImGuiWindow* window);
//     static void             DockContextBuildNodesFromSettings(ImGuiContext* ctx, ImGuiDockNodeSettings* node_settings_array, int node_settings_count);
//     static void             DockContextBuildAddWindowsToNodes(ImGuiContext* ctx, ImGuiID root_id);                            // Use root_id==0 to add all
//
//     // ImGuiDockNode
//     static void             DockNodeAddWindow(ImGuiDockNode* node, ImGuiWindow* window, bool add_to_tab_bar);
//     static void             DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node);
//     static void             DockNodeMoveChildNodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node);
//     static ImGuiWindow*     DockNodeFindWindowByID(ImGuiDockNode* node, ImGuiID id);
//     static void             DockNodeApplyPosSizeToWindows(ImGuiDockNode* node);
//     static void             DockNodeRemoveWindow(ImGuiDockNode* node, ImGuiWindow* window, ImGuiID save_dock_id);
//     static void             DockNodeHideHostWindow(ImGuiDockNode* node);
//     static void             DockNodeUpdate(ImGuiDockNode* node);
//     static void             DockNodeUpdateForRootNode(ImGuiDockNode* node);
//     static void             DockNodeUpdateFlagsAndCollapse(ImGuiDockNode* node);
//     static void             DockNodeUpdateHasCentralNodeChild(ImGuiDockNode* node);
//     static void             DockNodeUpdateTabBar(ImGuiDockNode* node, ImGuiWindow* host_window);
//     static void             DockNodeAddTabBar(ImGuiDockNode* node);
//     static void             DockNodeRemoveTabBar(ImGuiDockNode* node);
//     static ImGuiID          DockNodeUpdateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar);
//     static void             DockNodeUpdateVisibleFlag(ImGuiDockNode* node);
//     static void             DockNodeStartMouseMovingWindow(ImGuiDockNode* node, ImGuiWindow* window);
//     static bool             DockNodeIsDropAllowed(ImGuiWindow* host_window, ImGuiWindow* payload_window);
//     static void             DockNodePreviewDockSetup(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* payload_window, ImGuiDockPreviewData* preview_data, bool is_explicit_target, bool is_outer_docking);
//     static void             DockNodePreviewDockRender(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* payload_window, const ImGuiDockPreviewData* preview_data);
//     static void             DockNodeCalcTabBarLayout(const ImGuiDockNode* node, ImRect* out_title_rect, ImRect* out_tab_bar_rect, Vector2D* out_window_menu_button_pos, Vector2D* out_close_button_pos);
//     static void             DockNodeCalcSplitRects(Vector2D& pos_old, Vector2D& size_old, Vector2D& pos_new, Vector2D& size_new, ImGuiDir dir, Vector2D size_new_desired);
//     static bool             DockNodeCalcDropRectsAndTestMousePos(const ImRect& parent, ImGuiDir dir, ImRect& out_draw, bool outer_docking, Vector2D* test_mouse_pos);
//     static const char*      DockNodeGetHostWindowTitle(ImGuiDockNode* node, char* buf, int buf_size) { ImFormatString(buf, buf_size, "##DockNode_%02X", node->id); return buf; }
//     static int              DockNodeGetTabOrder(ImGuiWindow* window);
//
//     // ImGuiDockNode tree manipulations
//     static void             DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, int split_first_child, float split_ratio, ImGuiDockNode* new_node);
//     static void             DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child);
//     static void             DockNodeTreeUpdatePosSize(ImGuiDockNode* node, Vector2D pos, Vector2D size, ImGuiDockNode* only_write_to_single_node = NULL);
//     static void             DockNodeTreeUpdateSplitter(ImGuiDockNode* node);
//     static ImGuiDockNode*   DockNodeTreeFindVisibleNodeByPos(ImGuiDockNode* node, Vector2D pos);
//     static ImGuiDockNode*   DockNodeTreeFindFallbackLeafNode(ImGuiDockNode* node);
//
//     // Settings
//     static void             DockSettingsRenameNodeReferences(ImGuiID old_node_id, ImGuiID new_node_id);
//     static void             DockSettingsRemoveNodeReferences(ImGuiID* node_ids, int node_ids_count);
//     static ImGuiDockNodeSettings*   DockSettingsFindNodeSettings(ImGuiContext* ctx, ImGuiID node_id);
//     static void             DockSettingsHandler_ClearAll(ImGuiContext*, ImGuiSettingsHandler*);
//     static void             DockSettingsHandler_ApplyAll(ImGuiContext*, ImGuiSettingsHandler*);
//     static void*            DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, const char* name);
//     static void             DockSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, void* entry, const char* line);
//     static void             DockSettingsHandler_WriteAll(ImGuiContext* imgui_ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* buf);
// }

//-----------------------------------------------------------------------------
// Docking: ImGuiDockContext
//-----------------------------------------------------------------------------
// The lifetime model is different from the one of regular windows: we always create a ImGuiDockNode for each ImGuiDockNodeSettings,
// or we always hold the entire docking node tree. Nodes are frequently hidden, e.g. if the window(s) or child nodes they host are not active.
// At boot time only, we run a simple GC to remove nodes that have no references.
// Because dock node settings (which are small, contiguous structures) are always mirrored by their corresponding dock nodes (more complete structures),
// we can also very easily recreate the nodes from scratch given the settings data (this is what DockContextRebuild() does).
// This is convenient as docking reconfiguration can be implemented by mostly poking at the simpler settings data.
//-----------------------------------------------------------------------------
// - DockContextInitialize()
// - DockContextShutdown()
// - DockContextClearNodes()
// - DockContextRebuildNodes()
// - DockContextNewFrameUpdateUndocking()
// - DockContextNewFrameUpdateDocking()
// - DockContextEndFrame()
// - DockContextFindNodeByID()
// - DockContextBindNodeToWindow()
// - DockContextGenNodeID()
// - DockContextAddNode()
// - DockContextRemoveNode()
// - ImGuiDockContextPruneNodeData
// - DockContextPruneUnusedSettingsNodes()
// - DockContextBuildNodesFromSettings()
// - DockContextBuildAddWindowsToNodes()
//-----------------------------------------------------------------------------

static int IMGUI_CDECL DockNodeComparerDepthMostFirst(const void* lhs, const void* rhs)
{
    const ImGuiDockNode* a = *(const ImGuiDockNode* const*)lhs;
    const ImGuiDockNode* b = *(const ImGuiDockNode* const*)rhs;
    return ImGui::DockNodeGetDepth(b) - ImGui::DockNodeGetDepth(a);
}

// Pre C++0x doesn't allow us to use a function-local type (without linkage) as template parameter, so we moved this here.
struct ImGuiDockContextPruneNodeData
{
    int         CountWindows, CountChildWindows, CountChildNodes;
    ImGuiID     RootId;
    ImGuiDockContextPruneNodeData() { CountWindows = CountChildWindows = CountChildNodes = 0; RootId = 0; }
};

// Garbage collect unused nodes (run once at init time)
static void ImGui::DockContextPruneUnusedSettingsNodes(ImGuiContext* ctx)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc  = &ctx->DockContext;
    IM_ASSERT(g.Windows.Size == 0);

    ImPool<ImGuiDockContextPruneNodeData> pool;
    pool.Reserve(dc->NodesSettings.Size);

    // Count child nodes and compute RootID
    for (int settings_n = 0; settings_n < dc->NodesSettings.Size; settings_n += 1)
    {
        ImGuiDockNodeSettings* settings = &dc->NodesSettings[settings_n];
        ImGuiDockContextPruneNodeData* parent_data = settings->ParentNodeId ? pool.GetByKey(settings->ParentNodeId) : 0;
        pool.GetOrAddByKey(settings->ID)->RootId = parent_data ? parent_data->RootId : settings->ID;
        if (settings->ParentNodeId)
            pool.GetOrAddByKey(settings->ParentNodeId)->CountChildNodes += 1;
    }

    // Count reference to dock ids from dockspaces
    // We track the 'auto-dock_node <- manual-Window <- manual-DockSpace' in order to avoid 'auto-dock_node' being ditched by DockContextPruneUnusedSettingsNodes()
    for (int settings_n = 0; settings_n < dc->NodesSettings.Size; settings_n += 1)
    {
        ImGuiDockNodeSettings* settings = &dc->NodesSettings[settings_n];
        if (settings->ParentWindowId != 0)
            if (ImGuiWindowSettings* window_settings = FindWindowSettings(settings->ParentWindowId))
                if (window_settings->DockId)
                    if (ImGuiDockContextPruneNodeData* data = pool.GetByKey(window_settings->DockId))
                        data->CountChildNodes += 1;
    }

    // Count reference to dock ids from window settings
    // We guard against the possibility of an invalid .ini file (RootID may point to a missing node)
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != NULL; settings = g.SettingsWindows.next_chunk(settings))
        if (ImGuiID dock_id = settings->DockId)
            if (ImGuiDockContextPruneNodeData* data = pool.GetByKey(dock_id))
            {
                data->CountWindows += 1;
                if (ImGuiDockContextPruneNodeData* data_root = (data->RootId == dock_id) ? data : pool.GetByKey(data->RootId))
                    data_root->CountChildWindows += 1;
            }

    // Prune
    for (int settings_n = 0; settings_n < dc->NodesSettings.Size; settings_n += 1)
    {
        ImGuiDockNodeSettings* settings = &dc->NodesSettings[settings_n];
        ImGuiDockContextPruneNodeData* data = pool.GetByKey(settings->ID);
        if (data->CountWindows > 1)
            continue;
        ImGuiDockContextPruneNodeData* data_root = (data->RootId == settings->ID) ? data : pool.GetByKey(data->RootId);

        bool remove = false;
        remove |= (data->CountWindows == 1 && settings->ParentNodeId == 0 && data->CountChildNodes == 0 && !(settings.flags & ImGuiDockNodeFlags_CentralNode));  // Floating root node with only 1 window
        remove |= (data->CountWindows == 0 && settings->ParentNodeId == 0 && data->CountChildNodes == 0); // Leaf nodes with 0 window
        remove |= (data_root->CountChildWindows == 0);
        if (remove)
        {
            IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextPruneUnusedSettingsNodes: Prune 0x%08X\n", settings->ID);
            DockSettingsRemoveNodeReferences(&settings->ID, 1);
            settings->ID = 0;
        }
    }
}

static void ImGui::DockContextBuildNodesFromSettings(ImGuiContext* ctx, ImGuiDockNodeSettings* node_settings_array, int node_settings_count)
{
    // build nodes
    for (int node_n = 0; node_n < node_settings_count; node_n += 1)
    {
        ImGuiDockNodeSettings* settings = &node_settings_array[node_n];
        if (settings->ID == 0)
            continue;
        ImGuiDockNode* node = DockContextAddNode(ctx, settings->ID);
        node->ParentNode = settings->ParentNodeId ? DockContextFindNodeByID(ctx, settings->ParentNodeId) : NULL;
        node.pos = DimgVec2D::new(settings.pos.x, settings.pos.y);
        node->Size = DimgVec2D::new(settings->Size.x, settings->Size.y);
        node->SizeRef = DimgVec2D::new(settings->SizeRef.x, settings->SizeRef.y);
        node->AuthorityForPos = node->AuthorityForSize = node->AuthorityForViewport = ImGuiDataAuthority_DockNode;
        if (node->ParentNode && node->ParentNode->ChildNodes[0] == NULL)
            node->ParentNode->ChildNodes[0] = node;
        else if (node->ParentNode && node->ParentNode->ChildNodes[1] == NULL)
            node->ParentNode->ChildNodes[1] = node;
        node->SelectedTabId = settings->SelectedTabId;
        node->SplitAxis = (ImGuiAxis)settings->SplitAxis;
        node->SetLocalFlags(settings.flags & ImGuiDockNodeFlags_SavedFlagsMask_);

        // Bind host window immediately if it already exist (in case of a rebuild)
        // This is useful as the root_window_for_title_bar_highlight links necessary to highlight the currently focused node requires node->host_window to be set.
        char host_window_title[20];
        ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        node->HostWindow = FindWindowByName(DockNodeGetHostWindowTitle(root_node, host_window_title, IM_ARRAYSIZE(host_window_title)));
    }
}

void ImGui::DockContextBuildAddWindowsToNodes(ImGuiContext* ctx, ImGuiID root_id)
{
    // Rebind all windows to nodes (they can also lazily rebind but we'll have a visible glitch during the first frame)
    ImGuiContext& g = *ctx;
    for (int n = 0; n < g.Windows.Size; n += 1)
    {
        ImGuiWindow* window = g.Windows[n];
        if (window.DockId == 0 || window.LastFrameActive < g.FrameCount - 1)
            continue;
        if (window.DockNode != NULL)
            continue;

        ImGuiDockNode* node = DockContextFindNodeByID(ctx, window.DockId);
        IM_ASSERT(node != NULL);   // This should have been called after DockContextBuildNodesFromSettings()
        if (root_id == 0 || DockNodeGetRootNode(node)->ID == root_id)
            DockNodeAddWindow(node, window, true);
    }
}

//-----------------------------------------------------------------------------
// Docking: ImGuiDockContext Docking/Undocking functions
//-----------------------------------------------------------------------------
// - DockContextQueueDock()
// - DockContextQueueUndockWindow()
// - DockContextQueueUndockNode()
// - DockContextQueueNotifyRemovedNode()
// - DockContextProcessDock()
// - DockContextProcessUndockWindow()
// - DockContextProcessUndockNode()
// - DockContextCalcDropPosForDocking()
//-----------------------------------------------------------------------------

void ImGui::DockContextQueueDock(ImGuiContext* ctx, ImGuiWindow* target, ImGuiDockNode* target_node, ImGuiWindow* payload, ImGuiDir split_dir, float split_ratio, bool split_outer)
{
    IM_ASSERT(target != payload);
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Dock;
    req.DockTargetWindow = target;
    req.DockTargetNode = target_node;
    req.DockPayload = payload;
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = split_ratio;
    req.DockSplitOuter = split_outer;
    ctx->DockContext.Requests.push_back(req);
}

void ImGui::DockContextQueueUndockWindow(ImGuiContext* ctx, ImGuiWindow* window)
{
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Undock;
    req.UndockTargetWindow = window;
    ctx->DockContext.Requests.push_back(req);
}

void ImGui::DockContextQueueUndockNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Undock;
    req.UndockTargetNode = node;
    ctx->DockContext.Requests.push_back(req);
}

void ImGui::DockContextQueueNotifyRemovedNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiDockContext* dc  = &ctx->DockContext;
    for (int n = 0; n < dc->Requests.Size; n += 1)
        if (dc->Requests[n].DockTargetNode == node)
            dc->Requests[n].Type = ImGuiDockRequestType_None;
}

void ImGui::DockContextProcessDock(ImGuiContext* ctx, ImGuiDockRequest* req)
{
    IM_ASSERT((req->Type == ImGuiDockRequestType_Dock && req->DockPayload != NULL) || (req->Type == ImGuiDockRequestType_Split && req->DockPayload == NULL));
    IM_ASSERT(req->DockTargetWindow != NULL || req->DockTargetNode != NULL);

    ImGuiContext& g = *ctx;
    IM_UNUSED(g);

    ImGuiWindow* payload_window = req->DockPayload;     // Optional
    ImGuiWindow* target_window = req->DockTargetWindow;
    ImGuiDockNode* node = req->DockTargetNode;
    if (payload_window)
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessDock node 0x%08X target '%s' dock window '%s', split_dir %d\n", node ? node->ID : 0, target_window ? target_window.Name : "NULL", payload_window ? payload_window.Name : "NULL", req->DockSplitDir);
    else
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessDock node 0x%08X, split_dir %d\n", node ? node->ID : 0, req->DockSplitDir);

    // Decide which Tab will be selected at the end of the operation
    ImGuiID next_selected_id = 0;
    ImGuiDockNode* payload_node = NULL;
    if (payload_window)
    {
        payload_node = payload_window.DockNodeAsHost;
        payload_window.DockNodeAsHost = NULL; // Important to clear this as the node will have its life as a child which might be merged/deleted later.
        if (payload_node && payload_node->IsLeafNode())
            next_selected_id = payload_node->TabBar->NextSelectedTabId ? payload_node->TabBar->NextSelectedTabId : payload_node->TabBar->SelectedTabId;
        if (payload_node == NULL)
            next_selected_id = payload_window.TabId;
    }

    // FIXME-DOCK: When we are trying to dock an existing single-window node into a loose window, transfer Node id as well
    // When processing an interactive split, usually last_frame_alive will be < g.frame_count. But DockBuilder operations can make it ==.
    if (node)
        IM_ASSERT(node->LastFrameAlive <= g.FrameCount);
    if (node && target_window && node == target_window.DockNodeAsHost)
        IM_ASSERT(node->Windows.Size > 0 || node->IsSplitNode() || node->IsCentralNode());

    // Create new node and add existing window to it
    if (node == NULL)
    {
        node = DockContextAddNode(ctx, 0);
        node.pos = target_window.Pos;
        node->Size = target_window.Size;
        if (target_window.DockNodeAsHost == NULL)
        {
            DockNodeAddWindow(node, target_window, true);
            node->TabBar->Tabs[0].Flags &= ~ImGuiTabItemFlags_Unsorted;
            target_window.DockIsActive = true;
        }
    }

    ImGuiDir split_dir = req->DockSplitDir;
    if (split_dir != ImGuiDir_None)
    {
        // split into two, one side will be our payload node unless we are dropping a loose window
        const ImGuiAxis split_axis = (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;
        const int split_inheritor_child_idx = (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 1 : 0; // Current contents will be moved to the opposite side
        const float split_ratio = req->DockSplitRatio;
        DockNodeTreeSplit(ctx, node, split_axis, split_inheritor_child_idx, split_ratio, payload_node);  // payload_node may be NULL here!
        ImGuiDockNode* new_node = node->ChildNodes[split_inheritor_child_idx ^ 1];
        new_node->HostWindow = node->HostWindow;
        node = new_node;
    }
    node->SetLocalFlags(node->LocalFlags & ~ImGuiDockNodeFlags_HiddenTabBar);

    if (node != payload_node)
    {
        // Create tab bar before we call DockNodeMoveWindows (which would attempt to move the old tab-bar, which would lead us to payload tabs wrongly appearing before target tabs!)
        if (node->Windows.Size > 0 && node->TabBar == NULL)
        {
            DockNodeAddTabBar(node);
            for (int n = 0; n < node->Windows.Size; n += 1)
                TabBarAddTab(node->TabBar, ImGuiTabItemFlags_None, node->Windows[n]);
        }

        if (payload_node != NULL)
        {
            // Transfer full payload node (with 1+ child windows or child nodes)
            if (payload_node->IsSplitNode())
            {
                if (node->Windows.Size > 0)
                {
                    // We can dock a split payload into a node that already has windows _only_ if our payload is a node tree with a single visible node.
                    // In this situation, we move the windows of the target node into the currently visible node of the payload.
                    // This allows us to preserve some of the underlying dock tree settings nicely.
                    IM_ASSERT(payload_node->OnlyNodeWithWindows != NULL); // The docking should have been blocked by DockNodePreviewDockSetup() early on and never submitted.
                    ImGuiDockNode* visible_node = payload_node->OnlyNodeWithWindows;
                    if (visible_node->TabBar)
                        IM_ASSERT(visible_node->TabBar->Tabs.Size > 0);
                    DockNodeMoveWindows(node, visible_node);
                    DockNodeMoveWindows(visible_node, node);
                    DockSettingsRenameNodeReferences(node->ID, visible_node->ID);
                }
                if (node->IsCentralNode())
                {
                    // Central node property needs to be moved to a leaf node, pick the last focused one.
                    // FIXME-DOCK: If we had to transfer other flags here, what would the policy be?
                    ImGuiDockNode* last_focused_node = DockContextFindNodeByID(ctx, payload_node->LastFocusedNodeId);
                    IM_ASSERT(last_focused_node != NULL);
                    ImGuiDockNode* last_focused_root_node = DockNodeGetRootNode(last_focused_node);
                    IM_ASSERT(last_focused_root_node == DockNodeGetRootNode(payload_node));
                    last_focused_node->SetLocalFlags(last_focused_node->LocalFlags | ImGuiDockNodeFlags_CentralNode);
                    node->SetLocalFlags(node->LocalFlags & ~ImGuiDockNodeFlags_CentralNode);
                    last_focused_root_node->CentralNode = last_focused_node;
                }

                IM_ASSERT(node->Windows.Size == 0);
                DockNodeMoveChildNodes(node, payload_node);
            }
            else
            {
                const ImGuiID payload_dock_id = payload_node->ID;
                DockNodeMoveWindows(node, payload_node);
                DockSettingsRenameNodeReferences(payload_dock_id, node->ID);
            }
            DockContextRemoveNode(ctx, payload_node, true);
        }
        else if (payload_window)
        {
            // Transfer single window
            const ImGuiID payload_dock_id = payload_window.DockId;
            node->VisibleWindow = payload_window;
            DockNodeAddWindow(node, payload_window, true);
            if (payload_dock_id != 0)
                DockSettingsRenameNodeReferences(payload_dock_id, node->ID);
        }
    }
    else
    {
        // When docking a floating single window node we want to reevaluate auto-hiding of the tab bar
        node->WantHiddenTabBarUpdate = true;
    }

    // Update selection immediately
    if (ImGuiTabBar* tab_bar = node->TabBar)
        tab_bar->NextSelectedTabId = next_selected_id;
    MarkIniSettingsDirty();
}

// Problem:
//   Undocking a large (~full screen) window would leave it so large that the bottom right sizing corner would more
//   than likely be off the screen and the window would be hard to resize to fit on screen. This can be particularly problematic
//   with 'config_windows_move_from_title_bar_only=true' and/or with 'config_windows_resize_from_edges=false' as well (the later can be
//   due to missing ImGuiBackendFlags_HasMouseCursors backend flag).
// Solution:
//   When undocking a window we currently force its maximum size to 90% of the host viewport or monitor.
// Reevaluate this when we implement preserving docked/undocked size ("docking_wip/undocked_size" branch).
static Vector2D FixLargeWindowsWhenUndocking(const Vector2D& size, ImGuiViewport* ref_viewport)
{
    if (ref_viewport == NULL)
        return size;

    ImGuiContext& g = *GImGui;
    Vector2D max_size = ImFloor(ref_viewport->WorkSize * 0.90);
    if (g.config_flags_curr_frame & ConfigFlags::ViewportsEnable)
    {
        const ImGuiPlatformMonitor* monitor = ImGui::GetViewportPlatformMonitor(ref_viewport);
        max_size = ImFloor(monitor->WorkSize * 0.90);
    }
    return ImMin(size, max_size);
}

void ImGui::DockContextProcessUndockWindow(ImGuiContext* ctx, ImGuiWindow* window, bool clear_persistent_docking_ref)
{
    ImGuiContext& g = *ctx;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockWindow window '%s', clear_persistent_docking_ref = %d\n", window.Name, clear_persistent_docking_ref);
    if (window.DockNode)
        DockNodeRemoveWindow(window.DockNode, window, clear_persistent_docking_ref ? 0 : window.DockId);
    else
        window.DockId = 0;
    window.Collapsed = false;
    window.DockIsActive = false;
    window.DockNodeIsVisible = window.DockTabIsVisible = false;
    window.Size = window.SizeFull = FixLargeWindowsWhenUndocking(window.SizeFull, window.viewport);

    MarkIniSettingsDirty();
}

void ImGui::DockContextProcessUndockNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiContext& g = *ctx;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockNode node %08X\n", node->ID);
    IM_ASSERT(node->IsLeafNode());
    IM_ASSERT(node->Windows.Size >= 1);

    if (node->IsRootNode() || node->IsCentralNode())
    {
        // In the case of a root node or central node, the node will have to stay in place. Create a new node to receive the payload.
        ImGuiDockNode* new_node = DockContextAddNode(ctx, 0);
        new_node.pos = node.pos;
        new_node->Size = node->Size;
        new_node->SizeRef = node->SizeRef;
        DockNodeMoveWindows(new_node, node);
        DockSettingsRenameNodeReferences(node->ID, new_node->ID);
        for (int n = 0; n < new_node->Windows.Size; n += 1)
        {
            ImGuiWindow* window = new_node->Windows[n];
            window.Flags &= ~ImGuiWindowFlags_ChildWindow;
            if (window.ParentWindow)
                window.ParentWindow->DC.ChildWindows.find_erase(window);
            UpdateWindowParentAndRootLinks(window, window.Flags, NULL);
        }
        node = new_node;
    }
    else
    {
        // Otherwise extract our node and merge our sibling back into the parent node.
        IM_ASSERT(node->ParentNode->ChildNodes[0] == node || node->ParentNode->ChildNodes[1] == node);
        int index_in_parent = (node->ParentNode->ChildNodes[0] == node) ? 0 : 1;
        node->ParentNode->ChildNodes[index_in_parent] = NULL;
        DockNodeTreeMerge(ctx, node->ParentNode, node->ParentNode->ChildNodes[index_in_parent ^ 1]);
        node->ParentNode->AuthorityForViewport = ImGuiDataAuthority_Window; // The node that stays in place keeps the viewport, so our newly dragged out node will create a new viewport
        node->ParentNode = NULL;
    }
    node->AuthorityForPos = node->AuthorityForSize = ImGuiDataAuthority_DockNode;
    node->Size = FixLargeWindowsWhenUndocking(node->Size, node->Windows[0]->Viewport);
    node->WantMouseMove = true;
    MarkIniSettingsDirty();
}

// This is mostly used for automation.
bool ImGui::DockContextCalcDropPosForDocking(ImGuiWindow* target, ImGuiDockNode* target_node, ImGuiWindow* payload, ImGuiDir split_dir, bool split_outer, Vector2D* out_pos)
{
    // In DockNodePreviewDockSetup() for a root central node instead of showing both "inner" and "outer" drop rects
    // (which would be functionally identical) we only show the outer one. Reflect this here.
    if (target_node && target_node->ParentNode == NULL && target_node->IsCentralNode() && split_dir != ImGuiDir_None)
        split_outer = true;
    ImGuiDockPreviewData split_data;
    DockNodePreviewDockSetup(target, target_node, payload, &split_data, false, split_outer);
    if (split_data.DropRectsDraw[split_dir+1].IsInverted())
        return false;
    *out_pos = split_data.DropRectsDraw[split_dir+1].GetCenter();
    return true;
}

//-----------------------------------------------------------------------------
// Docking: ImGuiDockNode
//-----------------------------------------------------------------------------
// - DockNodeGetTabOrder()
// - DockNodeAddWindow()
// - DockNodeRemoveWindow()
// - DockNodeMoveChildNodes()
// - DockNodeMoveWindows()
// - DockNodeApplyPosSizeToWindows()
// - DockNodeHideHostWindow()
// - ImGuiDockNodeFindInfoResults
// - DockNodeFindInfo()
// - DockNodeFindWindowByID()
// - DockNodeUpdateFlagsAndCollapse()
// - DockNodeUpdateHasCentralNodeFlag()
// - DockNodeUpdateVisibleFlag()
// - DockNodeStartMouseMovingWindow()
// - DockNodeUpdate()
// - DockNodeUpdateWindowMenu()
// - DockNodeBeginAmendTabBar()
// - DockNodeEndAmendTabBar()
// - DockNodeUpdateTabBar()
// - DockNodeAddTabBar()
// - DockNodeRemoveTabBar()
// - DockNodeIsDropAllowedOne()
// - DockNodeIsDropAllowed()
// - DockNodeCalcTabBarLayout()
// - DockNodeCalcSplitRects()
// - DockNodeCalcDropRectsAndTestMousePos()
// - DockNodePreviewDockSetup()
// - DockNodePreviewDockRender()
//-----------------------------------------------------------------------------

ImGuiDockNode::ImGuiDockNode(ImGuiID id)
{
    ID = id;
    SharedFlags = LocalFlags = LocalFlagsInWindows = MergedFlags = ImGuiDockNodeFlags_None;
    ParentNode = ChildNodes[0] = ChildNodes[1] = NULL;
    TabBar = NULL;
    SplitAxis = ImGuiAxis_None;

    State = ImGuiDockNodeState_Unknown;
    LastBgColor = IM_COL32_WHITE;
    HostWindow = VisibleWindow = NULL;
    CentralNode = OnlyNodeWithWindows = NULL;
    CountNodeWithWindows = 0;
    LastFrameAlive = LastFrameActive = LastFrameFocused = -1;
    LastFocusedNodeId = 0;
    SelectedTabId = 0;
    WantCloseTabId = 0;
    AuthorityForPos = AuthorityForSize = ImGuiDataAuthority_DockNode;
    AuthorityForViewport = ImGuiDataAuthority_Auto;
    IsVisible = true;
    IsFocused = HasCloseButton = HasWindowMenuButton = HasCentralNodeChild = false;
    IsBgDrawnThisFrame = false;
    WantCloseAll = WantLockSizeOnce = WantMouseMove = WantHiddenTabBarUpdate = WantHiddenTabBarToggle = false;
}

ImGuiDockNode::~ImGuiDockNode()
{
    IM_DELETE(TabBar);
    TabBar = NULL;
    ChildNodes[0] = ChildNodes[1] = NULL;
}

int ImGui::DockNodeGetTabOrder(ImGuiWindow* window)
{
    ImGuiTabBar* tab_bar = window.DockNode->TabBar;
    if (tab_bar == NULL)
        return -1;
    ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, window.TabId);
    return tab ? tab_bar->GetTabOrder(tab) : -1;
}

static void DockNodeHideWindowDuringHostWindowCreation(ImGuiWindow* window)
{
    window.Hidden = true;
    window.HiddenFramesCanSkipItems = window.Active ? 1 : 2;
}

static void ImGui::DockNodeAddWindow(ImGuiDockNode* node, ImGuiWindow* window, bool add_to_tab_bar)
{
    ImGuiContext& g = *GImGui; (void)g;
    if (window.DockNode)
    {
        // Can overwrite an existing window->dock_node (e.g. pointing to a disabled DockSpace node)
        IM_ASSERT(window.DockNode->ID != node->ID);
        DockNodeRemoveWindow(window.DockNode, window, 0);
    }
    IM_ASSERT(window.DockNode == NULL || window.DockNodeAsHost == NULL);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeAddWindow node 0x%08X window '%s'\n", node->ID, window.Name);

    // If more than 2 windows appeared on the same frame leading to the creation of a new hosting window,
    // we'll hide windows until the host window is ready. Hide the 1st window after its been output (so it is not visible for one frame).
    // We will call DockNodeHideWindowDuringHostWindowCreation() on ourselves in Begin()
    if (node->HostWindow == NULL && node->Windows.Size == 1 && node->Windows[0]->WasActive == false)
        DockNodeHideWindowDuringHostWindowCreation(node->Windows[0]);

    node->Windows.push_back(window);
    node->WantHiddenTabBarUpdate = true;
    window.DockNode = node;
    window.DockId = node->ID;
    window.DockIsActive = (node->Windows.Size > 1);
    window.DockTabWantClose = false;

    // When reactivating a node with one or two loose window, the window pos/size/viewport are authoritative over the node storage.
    // In particular it is important we init the viewport from the first window so we don't create two viewports and drop one.
    if (node->HostWindow == NULL && node->IsFloatingNode())
    {
        if (node->AuthorityForPos == ImGuiDataAuthority_Auto)
            node->AuthorityForPos = ImGuiDataAuthority_Window;
        if (node->AuthorityForSize == ImGuiDataAuthority_Auto)
            node->AuthorityForSize = ImGuiDataAuthority_Window;
        if (node->AuthorityForViewport == ImGuiDataAuthority_Auto)
            node->AuthorityForViewport = ImGuiDataAuthority_Window;
    }

    // Add to tab bar if requested
    if (add_to_tab_bar)
    {
        if (node->TabBar == NULL)
        {
            DockNodeAddTabBar(node);
            node->TabBar->SelectedTabId = node->TabBar->NextSelectedTabId = node->SelectedTabId;

            // Add existing windows
            for (int n = 0; n < node->Windows.Size - 1; n += 1)
                TabBarAddTab(node->TabBar, ImGuiTabItemFlags_None, node->Windows[n]);
        }
        TabBarAddTab(node->TabBar, ImGuiTabItemFlags_Unsorted, window);
    }

    DockNodeUpdateVisibleFlag(node);

    // Update this without waiting for the next time we Begin() in the window, so our host window will have the proper title bar color on its first frame.
    if (node->HostWindow)
        UpdateWindowParentAndRootLinks(window, window.Flags | ImGuiWindowFlags_ChildWindow, node->HostWindow);
}

static void ImGui::DockNodeRemoveWindow(ImGuiDockNode* node, ImGuiWindow* window, ImGuiID save_dock_id)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(window.DockNode == node);
    //IM_ASSERT(window->root_window_dock_tree == node->host_window);
    //IM_ASSERT(window->last_frame_active < g.frame_count);    // We may call this from Begin()
    IM_ASSERT(save_dock_id == 0 || save_dock_id == node->ID);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeRemoveWindow node 0x%08X window '%s'\n", node->ID, window.Name);

    window.DockNode = NULL;
    window.DockIsActive = window.DockTabWantClose = false;
    window.DockId = save_dock_id;
    window.Flags &= ~ImGuiWindowFlags_ChildWindow;
    if (window.ParentWindow)
        window.ParentWindow->DC.ChildWindows.find_erase(window);
    UpdateWindowParentAndRootLinks(window, window.Flags, NULL); // Update immediately

    // Remove window
    bool erased = false;
    for (int n = 0; n < node->Windows.Size; n += 1)
        if (node->Windows[n] == window)
        {
            node->Windows.erase(node->Windows.Data + n);
            erased = true;
            break;
        }
    if (!erased)
        IM_ASSERT(erased);
    if (node->VisibleWindow == window)
        node->VisibleWindow = NULL;

    // Remove tab and possibly tab bar
    node->WantHiddenTabBarUpdate = true;
    if (node->TabBar)
    {
        TabBarRemoveTab(node->TabBar, window.TabId);
        const int tab_count_threshold_for_tab_bar = node->IsCentralNode() ? 1 : 2;
        if (node->Windows.Size < tab_count_threshold_for_tab_bar)
            DockNodeRemoveTabBar(node);
    }

    if (node->Windows.Size == 0 && !node->IsCentralNode() && !node->IsDockSpace() && window.DockId != node->ID)
    {
        // Automatic dock node delete themselves if they are not holding at least one tab
        DockContextRemoveNode(&g, node, true);
        return;
    }

    if (node->Windows.Size == 1 && !node->IsCentralNode() && node->HostWindow)
    {
        ImGuiWindow* remaining_window = node->Windows[0];
        if (node->HostWindow->ViewportOwned && node->IsRootNode())
        {
            // Transfer viewport back to the remaining loose window
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Node %08X transfer viewport %08X=>%08X for Window '%s'\n", node->ID, node->HostWindow->Viewport->ID, remaining_window.ID, remaining_window.Name);
            IM_ASSERT(node->HostWindow->Viewport->Window == node->HostWindow);
            node->HostWindow->Viewport->Window = remaining_window;
            node->HostWindow->Viewport->ID = remaining_window.ID;
        }
        remaining_window.Collapsed = node->HostWindow->Collapsed;
    }

    // Update visibility immediately is required so the DockNodeUpdateRemoveInactiveChilds() processing can reflect changes up the tree
    DockNodeUpdateVisibleFlag(node);
}

static void ImGui::DockNodeMoveChildNodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
{
    IM_ASSERT(dst_node->Windows.Size == 0);
    dst_node->ChildNodes[0] = src_node->ChildNodes[0];
    dst_node->ChildNodes[1] = src_node->ChildNodes[1];
    if (dst_node->ChildNodes[0])
        dst_node->ChildNodes[0]->ParentNode = dst_node;
    if (dst_node->ChildNodes[1])
        dst_node->ChildNodes[1]->ParentNode = dst_node;
    dst_node->SplitAxis = src_node->SplitAxis;
    dst_node->SizeRef = src_node->SizeRef;
    src_node->ChildNodes[0] = src_node->ChildNodes[1] = NULL;
}

static void ImGui::DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
{
    // Insert tabs in the same orders as currently ordered (node->windows isn't ordered)
    IM_ASSERT(src_node && dst_node && dst_node != src_node);
    ImGuiTabBar* src_tab_bar = src_node->TabBar;
    if (src_tab_bar != NULL)
        IM_ASSERT(src_node->Windows.Size <= src_node->TabBar->Tabs.Size);

    // If the dst_node is empty we can just move the entire tab bar (to preserve selection, scrolling, etc.)
    bool move_tab_bar = (src_tab_bar != NULL) && (dst_node->TabBar == NULL);
    if (move_tab_bar)
    {
        dst_node->TabBar = src_node->TabBar;
        src_node->TabBar = NULL;
    }

    for (int n = 0; n < src_node->Windows.Size; n += 1)
    {
        // dock_node's tab_bar may have non-window Tabs manually appended by user
        if (ImGuiWindow* window = src_tab_bar ? src_tab_bar->Tabs[n].Window : src_node->Windows[n])
        {
            window.DockNode = NULL;
            window.DockIsActive = false;
            DockNodeAddWindow(dst_node, window, move_tab_bar ? false : true);
        }
    }
    src_node->Windows.clear();

    if (!move_tab_bar && src_node->TabBar)
    {
        if (dst_node->TabBar)
            dst_node->TabBar->SelectedTabId = src_node->TabBar->SelectedTabId;
        DockNodeRemoveTabBar(src_node);
    }
}

static void ImGui::DockNodeApplyPosSizeToWindows(ImGuiDockNode* node)
{
    for (int n = 0; n < node->Windows.Size; n += 1)
    {
        set_window_pos(node->Windows[n], node.pos, Cond::Always); // We don't assign directly to pos because it can break the calculation of SizeContents on next frame
        SetWindowSize(node->Windows[n], node->Size, Cond::Always);
    }
}

static void ImGui::DockNodeHideHostWindow(ImGuiDockNode* node)
{
    if (node->HostWindow)
    {
        if (node->HostWindow->DockNodeAsHost == node)
            node->HostWindow->DockNodeAsHost = NULL;
        node->HostWindow = NULL;
    }

    if (node->Windows.Size == 1)
    {
        node->VisibleWindow = node->Windows[0];
        node->Windows[0]->DockIsActive = false;
    }

    if (node->TabBar)
        DockNodeRemoveTabBar(node);
}

// Search function called once by root node in DockNodeUpdate()
struct ImGuiDockNodeTreeInfo
{
    ImGuiDockNode*      CentralNode;
    ImGuiDockNode*      FirstNodeWithWindows;
    int                 CountNodesWithWindows;
    //ImGuiWindowClass  WindowClassForMerges;

    ImGuiDockNodeTreeInfo() { memset(this, 0, sizeof(*this)); }
};

static void DockNodeFindInfo(ImGuiDockNode* node, ImGuiDockNodeTreeInfo* info)
{
    if (node->Windows.Size > 0)
    {
        if (info->FirstNodeWithWindows == NULL)
            info->FirstNodeWithWindows = node;
        info->CountNodesWithWindows += 1;
    }
    if (node->IsCentralNode())
    {
        IM_ASSERT(info->CentralNode == NULL); // Should be only one
        IM_ASSERT(node->IsLeafNode() && "If you get this assert: please submit .ini file + repro of actions leading to this.");
        info->CentralNode = node;
    }
    if (info->CountNodesWithWindows > 1 && info->CentralNode != NULL)
        return;
    if (node->ChildNodes[0])
        DockNodeFindInfo(node->ChildNodes[0], info);
    if (node->ChildNodes[1])
        DockNodeFindInfo(node->ChildNodes[1], info);
}

static ImGuiWindow* ImGui::DockNodeFindWindowByID(ImGuiDockNode* node, ImGuiID id)
{
    IM_ASSERT(id != 0);
    for (int n = 0; n < node->Windows.Size; n += 1)
        if (node->Windows[n]->ID == id)
            return node->Windows[n];
    return NULL;
}

// - Remove inactive windows/nodes.
// - Update visibility flag.
static void ImGui::DockNodeUpdateFlagsAndCollapse(ImGuiDockNode* node)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(node->ParentNode == NULL || node->ParentNode->ChildNodes[0] == node || node->ParentNode->ChildNodes[1] == node);

    // Inherit most flags
    if (node->ParentNode)
        node->SharedFlags = node->ParentNode->SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;

    // Recurse into children
    // There is the possibility that one of our child becoming empty will delete itself and moving its sibling contents into 'node'.
    // If 'node->ChildNode[0]' delete itself, then 'node->ChildNode[1]->windows' will be moved into 'node'
    // If 'node->ChildNode[1]' delete itself, then 'node->ChildNode[0]->windows' will be moved into 'node' and the "remove inactive windows" loop will have run twice on those windows (harmless)
    node->HasCentralNodeChild = false;
    if (node->ChildNodes[0])
        DockNodeUpdateFlagsAndCollapse(node->ChildNodes[0]);
    if (node->ChildNodes[1])
        DockNodeUpdateFlagsAndCollapse(node->ChildNodes[1]);

    // Remove inactive windows, collapse nodes
    // merge node flags overrides stored in windows
    node->LocalFlagsInWindows = ImGuiDockNodeFlags_None;
    for (int window_n = 0; window_n < node->Windows.Size; window_n += 1)
    {
        ImGuiWindow* window = node->Windows[window_n];
        IM_ASSERT(window.DockNode == node);

        bool node_was_active = (node->LastFrameActive + 1 == g.FrameCount);
        bool remove = false;
        remove |= node_was_active && (window.LastFrameActive + 1 < g.FrameCount);
        remove |= node_was_active && (node->WantCloseAll || node->WantCloseTabId == window.TabId) && window.HasCloseButton && !(window.Flags & ImGuiWindowFlags_UnsavedDocument);  // Submit all _expected_ closure from last frame
        remove |= (window.DockTabWantClose);
        if (remove)
        {
            window.DockTabWantClose = false;
            if (node->Windows.Size == 1 && !node->IsCentralNode())
            {
                DockNodeHideHostWindow(node);
                node->State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
                DockNodeRemoveWindow(node, window, node->ID); // Will delete the node so it'll be invalid on return
                return;
            }
            DockNodeRemoveWindow(node, window, node->ID);
            window_n--;
            continue;
        }

        // FIXME-DOCKING: Missing policies for conflict resolution, hence the "Experimental" tag on this.
        //node->LocalFlagsInWindow &= ~window->window_class.DockNodeFlagsOverrideClear;
        node->LocalFlagsInWindows |= window.WindowClass.DockNodeFlagsOverrideSet;
    }
    node->UpdateMergedFlags();

    // Auto-hide tab bar option
    ImGuiDockNodeFlags node_flags = node->MergedFlags;
    if (node->WantHiddenTabBarUpdate && node->Windows.Size == 1 && (node_flags & ImGuiDockNodeFlags_AutoHideTabBar) && !node->IsHiddenTabBar())
        node->WantHiddenTabBarToggle = true;
    node->WantHiddenTabBarUpdate = false;

    // Cancel toggling if we know our tab bar is enforced to be hidden at all times
    if (node->WantHiddenTabBarToggle && node->VisibleWindow && (node->VisibleWindow->WindowClass.DockNodeFlagsOverrideSet & ImGuiDockNodeFlags_HiddenTabBar))
        node->WantHiddenTabBarToggle = false;

    // Apply toggles at a single point of the frame (here!)
    if (node->Windows.Size > 1)
        node->SetLocalFlags(node->LocalFlags & ~ImGuiDockNodeFlags_HiddenTabBar);
    else if (node->WantHiddenTabBarToggle)
        node->SetLocalFlags(node->LocalFlags ^ ImGuiDockNodeFlags_HiddenTabBar);
    node->WantHiddenTabBarToggle = false;

    DockNodeUpdateVisibleFlag(node);
}

// This is rarely called as DockNodeUpdateForRootNode() generally does it most frames.
static void ImGui::DockNodeUpdateHasCentralNodeChild(ImGuiDockNode* node)
{
    node->HasCentralNodeChild = false;
    if (node->ChildNodes[0])
        DockNodeUpdateHasCentralNodeChild(node->ChildNodes[0]);
    if (node->ChildNodes[1])
        DockNodeUpdateHasCentralNodeChild(node->ChildNodes[1]);
    if (node->IsRootNode())
    {
        ImGuiDockNode* mark_node = node->CentralNode;
        while (mark_node)
        {
            mark_node->HasCentralNodeChild = true;
            mark_node = mark_node->ParentNode;
        }
    }
}

static void ImGui::DockNodeUpdateVisibleFlag(ImGuiDockNode* node)
{
    // Update visibility flag
    bool is_visible = (node->ParentNode == NULL) ? node->IsDockSpace() : node->IsCentralNode();
    is_visible |= (node->Windows.Size > 0);
    is_visible |= (node->ChildNodes[0] && node->ChildNodes[0]->IsVisible);
    is_visible |= (node->ChildNodes[1] && node->ChildNodes[1]->IsVisible);
    node->IsVisible = is_visible;
}

static void ImGui::DockNodeStartMouseMovingWindow(ImGuiDockNode* node, ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(node->WantMouseMove == true);
    start_mouse_moving_window(window);
    g.ActiveIdClickOffset = g.io.MouseClickedPos[0] - node.pos;
    g.moving_window = window; // If we are docked into a non moveable root window, start_mouse_moving_window() won't set g.moving_window. Override that decision.
    node->WantMouseMove = false;
}

// Update central_node, OnlyNodeWithWindows, LastFocusedNodeID. Copy window class.
static void ImGui::DockNodeUpdateForRootNode(ImGuiDockNode* node)
{
    DockNodeUpdateFlagsAndCollapse(node);

    // - Setup central node pointers
    // - Find if there's only a single visible window in the hierarchy (in which case we need to display a regular title bar -> FIXME-DOCK: that last part is not done yet!)
    // Cannot merge this with DockNodeUpdateFlagsAndCollapse() because FirstNodeWithWindows is found after window removal and child collapsing
    ImGuiDockNodeTreeInfo info;
    DockNodeFindInfo(node, &info);
    node->CentralNode = info.CentralNode;
    node->OnlyNodeWithWindows = (info.CountNodesWithWindows == 1) ? info.FirstNodeWithWindows : NULL;
    node->CountNodeWithWindows = info.CountNodesWithWindows;
    if (node->LastFocusedNodeId == 0 && info.FirstNodeWithWindows != NULL)
        node->LastFocusedNodeId = info.FirstNodeWithWindows->ID;

    // Copy the window class from of our first window so it can be used for proper dock filtering.
    // When node has mixed windows, prioritize the class with the most constraint (docking_allow_unclassed = false) as the reference to copy.
    // FIXME-DOCK: We don't recurse properly, this code could be reworked to work from DockNodeUpdateScanRec.
    if (ImGuiDockNode* first_node_with_windows = info.FirstNodeWithWindows)
    {
        node->WindowClass = first_node_with_windows->Windows[0]->WindowClass;
        for (int n = 1; n < first_node_with_windows->Windows.Size; n += 1)
            if (first_node_with_windows->Windows[n]->WindowClass.DockingAllowUnclassed == false)
            {
                node->WindowClass = first_node_with_windows->Windows[n]->WindowClass;
                break;
            }
    }

    ImGuiDockNode* mark_node = node->CentralNode;
    while (mark_node)
    {
        mark_node->HasCentralNodeChild = true;
        mark_node = mark_node->ParentNode;
    }
}

static void DockNodeSetupHostWindow(ImGuiDockNode* node, ImGuiWindow* host_window)
{
    // Remove ourselves from any previous different host window
    // This can happen if a user mistakenly does (see #4295 for details):
    //  - N+0: DockBuilderAddNode(id, 0)    // missing ImGuiDockNodeFlags_DockSpace
    //  - N+1: NewFrame()                   // will create floating host window for that node
    //  - N+1: DockSpace(id)                // requalify node as dockspace, moving host window
    if (node->HostWindow && node->HostWindow != host_window && node->HostWindow->DockNodeAsHost == node)
        node->HostWindow->DockNodeAsHost = NULL;

    host_window.DockNodeAsHost = node;
    node->HostWindow = host_window;
}

static void ImGui::DockNodeUpdate(ImGuiDockNode* node)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(node->LastFrameActive != g.FrameCount);
    node->LastFrameAlive = g.FrameCount;
    node->IsBgDrawnThisFrame = false;

    node->CentralNode = node->OnlyNodeWithWindows = NULL;
    if (node->IsRootNode())
        DockNodeUpdateForRootNode(node);

    // Remove tab bar if not needed
    if (node->TabBar && node->IsNoTabBar())
        DockNodeRemoveTabBar(node);

    // Early out for hidden root dock nodes (when all dock_id references are in inactive windows, or there is only 1 floating window holding on the dock_id)
    bool want_to_hide_host_window = false;
    if (node->IsFloatingNode())
    {
        if (node->Windows.Size <= 1 && node->IsLeafNode())
            if (!g.io.ConfigDockingAlwaysTabBar && (node->Windows.Size == 0 || !node->Windows[0]->WindowClass.DockingAlwaysTabBar))
                want_to_hide_host_window = true;
        if (node->CountNodeWithWindows == 0)
            want_to_hide_host_window = true;
    }
    if (want_to_hide_host_window)
    {
        if (node->Windows.Size == 1)
        {
            // Floating window pos/size is authoritative
            ImGuiWindow* single_window = node->Windows[0];
            node.pos = single_window.Pos;
            node->Size = single_window.SizeFull;
            node->AuthorityForPos = node->AuthorityForSize = node->AuthorityForViewport = ImGuiDataAuthority_Window;

            // Transfer focus immediately so when we revert to a regular window it is immediately selected
            if (node->HostWindow && g.nav_window == node->HostWindow)
                focus_window(single_window);
            if (node->HostWindow)
            {
                single_window.viewport = node->HostWindow->Viewport;
                single_window.ViewportId = node->HostWindow->ViewportId;
                if (node->HostWindow->ViewportOwned)
                {
                    single_window.viewport->Window = single_window;
                    single_window.viewport_owned = true;
                }
            }
        }

        DockNodeHideHostWindow(node);
        node->State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
        node->WantCloseAll = false;
        node->WantCloseTabId = 0;
        node->HasCloseButton = node->HasWindowMenuButton = false;
        node->LastFrameActive = g.FrameCount;

        if (node->WantMouseMove && node->Windows.Size == 1)
            DockNodeStartMouseMovingWindow(node, node->Windows[0]);
        return;
    }

    // In some circumstance we will defer creating the host window (so everything will be kept hidden),
    // while the expected visible window is resizing itself.
    // This is important for first-time (no ini settings restored) single window when io.config_docking_always_tab_bar is enabled,
    // otherwise the node ends up using the minimum window size. Effectively those windows will take an extra frame to show up:
    //   N+0: Begin(): window created (with no known size), node is created
    //   N+1: DockNodeUpdate(): node skip creating host window / Begin(): window size applied, not visible
    //   N+2: DockNodeUpdate(): node can create host window / Begin(): window becomes visible
    // We could remove this frame if we could reliably calculate the expected window size during node update, before the Begin() code.
    // It would require a generalization of CalcWindowExpectedSize(), probably extracting code away from Begin().
    // In reality it isn't very important as user quickly ends up with size data in .ini file.
    if (node->IsVisible && node->HostWindow == NULL && node->IsFloatingNode() && node->IsLeafNode())
    {
        IM_ASSERT(node->Windows.Size > 0);
        ImGuiWindow* ref_window = NULL;
        if (node->SelectedTabId != 0) // Note that we prune single-window-node settings on .ini loading, so this is generally 0 for them!
            ref_window = DockNodeFindWindowByID(node, node->SelectedTabId);
        if (ref_window == NULL)
            ref_window = node->Windows[0];
        if (ref_window.AutoFitFramesX > 0 || ref_window.AutoFitFramesY > 0)
        {
            node->State = ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing;
            return;
        }
    }

    const ImGuiDockNodeFlags node_flags = node->MergedFlags;

    // Decide if the node will have a close button and a window menu button
    node->HasWindowMenuButton = (node->Windows.Size > 0) && (node_flags & ImGuiDockNodeFlags_NoWindowMenuButton) == 0;
    node->HasCloseButton = false;
    for (int window_n = 0; window_n < node->Windows.Size; window_n += 1)
    {
        // FIXME-DOCK: Setting dock_is_active here means that for single active window in a leaf node, dock_is_active will be cleared until the next Begin() call.
        ImGuiWindow* window = node->Windows[window_n];
        node->HasCloseButton |= window.HasCloseButton;
        window.DockIsActive = (node->Windows.Size > 1);
    }
    if (node_flags & ImGuiDockNodeFlags_NoCloseButton)
        node->HasCloseButton = false;

    // Bind or create host window
    ImGuiWindow* host_window = NULL;
    bool beginned_into_host_window = false;
    if (node->IsDockSpace())
    {
        // [Explicit root dockspace node]
        IM_ASSERT(node->HostWindow);
        host_window = node->HostWindow;
    }
    else
    {
        // [Automatic root or child nodes]
        if (node->IsRootNode() && node->IsVisible)
        {
            ImGuiWindow* ref_window = (node->Windows.Size > 0) ? node->Windows[0] : NULL;

            // Sync pos
            if (node->AuthorityForPos == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowPos(ref_window.Pos);
            else if (node->AuthorityForPos == ImGuiDataAuthority_DockNode)
                SetNextWindowPos(node.pos);

            // Sync size
            if (node->AuthorityForSize == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowSize(ref_window.SizeFull);
            else if (node->AuthorityForSize == ImGuiDataAuthority_DockNode)
                SetNextWindowSize(node->Size);

            // Sync collapsed
            if (node->AuthorityForSize == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowCollapsed(ref_window.Collapsed);

            // Sync viewport
            if (node->AuthorityForViewport == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowViewport(ref_window.ViewportId);

            SetNextWindowClass(&node->WindowClass);

            // Begin into the host window
            char window_label[20];
            DockNodeGetHostWindowTitle(node, window_label, IM_ARRAYSIZE(window_label));
            ImGuiWindowFlags window_flags = ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoScrollWithMouse | ImGuiWindowFlags_DockNodeHost;
            window_flags |= ImGuiWindowFlags_NoFocusOnAppearing;
            window_flags |= ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoNavFocus | ImGuiWindowFlags_NoCollapse;
            window_flags |= ImGuiWindowFlags_NoTitleBar;

            SetNextWindowBgAlpha(0.0); // Don't set ImGuiWindowFlags_NoBackground because it disables borders
            PushStyleVar(ImGuiStyleVar_WindowPadding, DimgVec2D::new(0, 0));
            Begin(window_label, NULL, window_flags);
            PopStyleVar();
            beginned_into_host_window = true;

            host_window = g.CurrentWindow;
            DockNodeSetupHostWindow(node, host_window);
            host_window.DC.CursorPos = host_window.Pos;
            node.pos = host_window.Pos;
            node->Size = host_window.Size;

            // We set ImGuiWindowFlags_NoFocusOnAppearing because we don't want the host window to take full focus (e.g. steal nav_window)
            // But we still it bring it to the front of display. There's no way to choose this precise behavior via window flags.
            // One simple case to ponder if: window A has a toggle to create windows B/C/D. Dock B/C/D together, clear the toggle and enable it again.
            // When reappearing B/C/D will request focus and be moved to the top of the display pile, but they are not linked to the dock host window
            // during the frame they appear. The dock host window would keep its old display order, and the sorting in EndFrame would move B/C/D back
            // after the dock host window, losing their top-most status.
            if (node->HostWindow.appearing)
                BringWindowToDisplayFront(node->HostWindow);

            node->AuthorityForPos = node->AuthorityForSize = node->AuthorityForViewport = ImGuiDataAuthority_Auto;
        }
        else if (node->ParentNode)
        {
            node->HostWindow = host_window = node->ParentNode->HostWindow;
            node->AuthorityForPos = node->AuthorityForSize = node->AuthorityForViewport = ImGuiDataAuthority_Auto;
        }
        if (node->WantMouseMove && node->HostWindow)
            DockNodeStartMouseMovingWindow(node, node->HostWindow);
    }

    // Update focused node (the one whose title bar is highlight) within a node tree
    if (node->IsSplitNode())
        IM_ASSERT(node->TabBar == NULL);
    if (node->IsRootNode())
        if (g.nav_window && g.nav_window->RootWindow->DockNode && g.nav_window->RootWindow->ParentWindow == host_window)
            node->LastFocusedNodeId = g.nav_window->RootWindow->DockNode->ID;

    // Register a hit-test hole in the window unless we are currently dragging a window that is compatible with our dockspace
    ImGuiDockNode* central_node = node->CentralNode;
    const bool central_node_hole = node->IsRootNode() && host_window && (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0 && central_node != NULL && central_node->IsEmpty();
    bool central_node_hole_register_hit_test_hole = central_node_hole;
    if (central_node_hole)
        if (const ImGuiPayload* payload = ImGui::GetDragDropPayload())
            if (payload->IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) && DockNodeIsDropAllowed(host_window, *(ImGuiWindow**)payload->Data))
                central_node_hole_register_hit_test_hole = false;
    if (central_node_hole_register_hit_test_hole)
    {
        // We add a little padding to match the "resize from edges" behavior and allow grabbing the splitter easily.
        // (But we only add it if there's something else on the other side of the hole, otherwise for e.g. fullscreen
        // covering passthru node we'd have a gap on the edge not covered by the hole)
        IM_ASSERT(node->IsDockSpace()); // We cannot pass this flag without the DockSpace() api. Testing this because we also setup the hole in host_window->parent_node
        ImGuiDockNode* root_node = DockNodeGetRootNode(central_node);
        ImRect root_rect(root_node.pos, root_node.pos + root_node->Size);
        ImRect hole_rect(central_node.pos, central_node.pos + central_node->Size);
        if (hole_rect.Min.x > root_rect.Min.x) { hole_rect.Min.x += WINDOWS_HOVER_PADDING; }
        if (hole_rect.Max.x < root_rect.Max.x) { hole_rect.Max.x -= WINDOWS_HOVER_PADDING; }
        if (hole_rect.Min.y > root_rect.Min.y) { hole_rect.Min.y += WINDOWS_HOVER_PADDING; }
        if (hole_rect.Max.y < root_rect.Max.y) { hole_rect.Max.y -= WINDOWS_HOVER_PADDING; }
        //GetForegroundDrawList()->add_rect(hole_rect.min, hole_rect.max, IM_COL32(255, 0, 0, 255));
        if (central_node_hole && !hole_rect.IsInverted())
        {
            SetWindowHitTestHole(host_window, hole_rect.Min, hole_rect.Max - hole_rect.Min);
            if (host_window.ParentWindow)
                SetWindowHitTestHole(host_window.ParentWindow, hole_rect.Min, hole_rect.Max - hole_rect.Min);
        }
    }

    // Update position/size, process and draw resizing splitters
    if (node->IsRootNode() && host_window)
    {
        host_window.DrawList->ChannelsSetCurrent(1);
        DockNodeTreeUpdatePosSize(node, host_window.Pos, host_window.Size);
        DockNodeTreeUpdateSplitter(node);
    }

    // Draw empty node background (currently can only be the Central Node)
    if (host_window && node->IsEmpty() && node->IsVisible)
    {
        host_window.DrawList->ChannelsSetCurrent(0);
        node->LastBgColor = (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) ? 0 : GetColorU32(ImGuiCol_DockingEmptyBg);
        if (node->LastBgColor != 0)
            host_window.DrawList->AddRectFilled(node.pos, node.pos + node->Size, node->LastBgColor);
        node->IsBgDrawnThisFrame = true;
    }

    // Draw whole dockspace background if ImGuiDockNodeFlags_PassthruCentralNode if set.
    // We need to draw a background at the root level if requested by ImGuiDockNodeFlags_PassthruCentralNode, but we will only know the correct pos/size
    // _after_ processing the resizing splitters. So we are using the draw_list channel splitting facility to submit drawing primitives out of order!
    const bool render_dockspace_bg = node->IsRootNode() && host_window && (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0;
    if (render_dockspace_bg && node->IsVisible)
    {
        host_window.DrawList->ChannelsSetCurrent(0);
        if (central_node_hole)
            RenderRectFilledWithHole(host_window.DrawList, node->Rect(), central_node->Rect(), GetColorU32(ImGuiCol_WindowBg), 0.0);
        else
            host_window.DrawList->AddRectFilled(node.pos, node.pos + node->Size, GetColorU32(ImGuiCol_WindowBg), 0.0);
    }

    // Draw and populate Tab Bar
    if (host_window)
        host_window.DrawList->ChannelsSetCurrent(1);
    if (host_window && node->Windows.Size > 0)
    {
        DockNodeUpdateTabBar(node, host_window);
    }
    else
    {
        node->WantCloseAll = false;
        node->WantCloseTabId = 0;
        node->IsFocused = false;
    }
    if (node->TabBar && node->TabBar->SelectedTabId)
        node->SelectedTabId = node->TabBar->SelectedTabId;
    else if (node->Windows.Size > 0)
        node->SelectedTabId = node->Windows[0]->ID;

    // Draw payload drop target
    if (host_window && node->IsVisible)
        if (node->IsRootNode() && (g.moving_window == NULL || g.moving_window->RootWindowDockTree != host_window))
            BeginDockableDragDropTarget(host_window);

    // We update this after DockNodeUpdateTabBar()
    node->LastFrameActive = g.FrameCount;

    // Recurse into children
    // FIXME-DOCK FIXME-OPT: Should not need to recurse into children
    if (host_window)
    {
        if (node->ChildNodes[0])
            DockNodeUpdate(node->ChildNodes[0]);
        if (node->ChildNodes[1])
            DockNodeUpdate(node->ChildNodes[1]);

        // Render outer borders last (after the tab bar)
        if (node->IsRootNode())
        {
            host_window.DrawList->ChannelsSetCurrent(1);
            RenderWindowOuterBorders(host_window);
        }

        // Further rendering (= hosted windows background) will be drawn on layer 0
        host_window.DrawList->ChannelsSetCurrent(0);
    }

    // End host window
    if (beginned_into_host_window) //-V1020
        End();
}

// Compare TabItem nodes given the last known dock_order (will persist in .ini file as hint), used to sort tabs when multiple tabs are added on the same frame.
static int IMGUI_CDECL TabItemComparerByDockOrder(const void* lhs, const void* rhs)
{
    ImGuiWindow* a = ((const ImGuiTabItem*)lhs)->Window;
    ImGuiWindow* b = ((const ImGuiTabItem*)rhs)->Window;
    if (int d = ((a->DockOrder == -1) ? INT_MAX : a->DockOrder) - ((b->DockOrder == -1) ? INT_MAX : b->DockOrder))
        return d;
    return (a->BeginOrderWithinContext - b->BeginOrderWithinContext);
}

static ImGuiID ImGui::DockNodeUpdateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar)
{
    // Try to position the menu so it is more likely to stays within the same viewport
    ImGuiContext& g = *GImGui;
    ImGuiID ret_tab_id = 0;
    if (g.Style.WindowMenuButtonPosition == ImGuiDir_Left)
        SetNextWindowPos(DimgVec2D::new(node.pos.x, node.pos.y + GetFrameHeight()), Cond::Always, DimgVec2D::new(0.0, 0.0));
    else
        SetNextWindowPos(DimgVec2D::new(node.pos.x + node->Size.x, node.pos.y + GetFrameHeight()), Cond::Always, DimgVec2D::new(1.0, 0.0));
    if (BeginPopup("#WindowMenu"))
    {
        node->IsFocused = true;
        if (tab_bar->Tabs.Size == 1)
        {
            if (MenuItem("Hide tab bar", NULL, node->IsHiddenTabBar()))
                node->WantHiddenTabBarToggle = true;
        }
        else
        {
            for (int tab_n = 0; tab_n < tab_bar->Tabs.Size; tab_n += 1)
            {
                ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
                if (tab.flags & ImGuiTabItemFlags_Button)
                    continue;
                if (Selectable(tab_bar->GetTabName(tab), tab->ID == tab_bar->SelectedTabId))
                    ret_tab_id = tab->ID;
                SameLine();
                Text("   ");
            }
        }
        EndPopup();
    }
    return ret_tab_id;
}

// User helper to append/amend into a dock node tab bar. Most commonly used to add e.g. a "+" button.
bool ImGui::DockNodeBeginAmendTabBar(ImGuiDockNode* node)
{
    if (node->TabBar == NULL || node->HostWindow == NULL)
        return false;
    if (node->MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly)
        return false;
    Begin(node->HostWindow->Name);
    PushOverrideID(node->ID);
    bool ret = BeginTabBarEx(node->TabBar, node->TabBar->BarRect, node->TabBar.flags, node);
    IM_UNUSED(ret);
    IM_ASSERT(ret);
    return true;
}

void ImGui::DockNodeEndAmendTabBar()
{
    EndTabBar();
    PopID();
    End();
}

static bool IsDockNodeTitleBarHighlighted(ImGuiDockNode* node, ImGuiDockNode* root_node, ImGuiWindow* host_window)
{
    // CTRL+Tab highlight (only highlighting leaf node, not whole hierarchy)
    ImGuiContext& g = *GImGui;
    if (g.NavWindowingTarget)
        return (g.NavWindowingTarget->DockNode == node);

    // FIXME-DOCKING: May want alternative to treat central node void differently? e.g. if (g.nav_window == host_window)
    if (g.nav_window && g.nav_window->RootWindowForTitleBarHighlight == host_window.RootWindowDockTree && root_node->LastFocusedNodeId == node->ID)
        for (ImGuiDockNode* parent_node = g.nav_window->RootWindow->DockNode; parent_node != NULL; parent_node = parent_node->HostWindow ? parent_node->HostWindow->RootWindow->DockNode : NULL)
            if ((parent_node = ImGui::DockNodeGetRootNode(parent_node)) == root_node)
                return true;
    return false;
}

// Submit the tab bar corresponding to a dock node and various housekeeping details.
static void ImGui::DockNodeUpdateTabBar(ImGuiDockNode* node, ImGuiWindow* host_window)
{
    ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;

    const bool node_was_active = (node->LastFrameActive + 1 == g.FrameCount);
    const bool closed_all = node->WantCloseAll && node_was_active;
    const ImGuiID closed_one = node->WantCloseTabId && node_was_active;
    node->WantCloseAll = false;
    node->WantCloseTabId = 0;

    // Decide if we should use a focused title bar color
    bool is_focused = false;
    ImGuiDockNode* root_node = DockNodeGetRootNode(node);
    if (IsDockNodeTitleBarHighlighted(node, root_node, host_window))
        is_focused = true;

    // hidden tab bar will show a triangle on the upper-left (in Begin)
    if (node->IsHiddenTabBar() || node->IsNoTabBar())
    {
        node->VisibleWindow = (node->Windows.Size > 0) ? node->Windows[0] : NULL;
        node->IsFocused = is_focused;
        if (is_focused)
            node->LastFrameFocused = g.FrameCount;
        if (node->VisibleWindow)
        {
            // Notify root of visible window (used to display title in OS task bar)
            if (is_focused || root_node->VisibleWindow == NULL)
                root_node->VisibleWindow = node->VisibleWindow;
            if (node->TabBar)
                node->TabBar->VisibleTabId = node->VisibleWindow->TabId;
        }
        return;
    }

    // Move ourselves to the Menu layer (so we can be accessed by tapping Alt) + undo skip_items flag in order to draw over the title bar even if the window is collapsed
    bool backup_skip_item = host_window.SkipItems;
    if (!node->IsDockSpace())
    {
        host_window.SkipItems = false;
        host_window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;
    }

    // Use PushOverrideID() instead of PushID() to use the node id _without_ the host window id.
    // This is to facilitate computing those id from the outside, and will affect more or less only the id of the collapse button, popup and tabs,
    // as docked windows themselves will override the stack with their own root id.
    PushOverrideID(node->ID);
    ImGuiTabBar* tab_bar = node->TabBar;
    bool tab_bar_is_recreated = (tab_bar == NULL); // Tab bar are automatically destroyed when a node gets hidden
    if (tab_bar == NULL)
    {
        DockNodeAddTabBar(node);
        tab_bar = node->TabBar;
    }

    ImGuiID focus_tab_id = 0;
    node->IsFocused = is_focused;

    const ImGuiDockNodeFlags node_flags = node->MergedFlags;
    const bool has_window_menu_button = (node_flags & ImGuiDockNodeFlags_NoWindowMenuButton) == 0 && (style.WindowMenuButtonPosition != ImGuiDir_None);

    // In a dock node, the Collapse Button turns into the Window Menu button.
    // FIXME-DOCK FIXME-OPT: Could we recycle popups id across multiple dock nodes?
    if (has_window_menu_button && IsPopupOpen("#WindowMenu"))
    {
        if (ImGuiID tab_id = DockNodeUpdateWindowMenu(node, tab_bar))
            focus_tab_id = tab_bar->NextSelectedTabId = tab_id;
        is_focused |= node->IsFocused;
    }

    // Layout
    ImRect title_bar_rect, tab_bar_rect;
    Vector2D window_menu_button_pos;
    Vector2D close_button_pos;
    DockNodeCalcTabBarLayout(node, &title_bar_rect, &tab_bar_rect, &window_menu_button_pos, &close_button_pos);

    // Submit new tabs, they will be added as Unsorted and sorted below based on relative dock_order value.
    const int tabs_count_old = tab_bar->Tabs.Size;
    for (int window_n = 0; window_n < node->Windows.Size; window_n += 1)
    {
        ImGuiWindow* window = node->Windows[window_n];
        if (TabBarFindTabByID(tab_bar, window.TabId) == NULL)
            TabBarAddTab(tab_bar, ImGuiTabItemFlags_Unsorted, window);
    }

    // Title bar
    if (is_focused)
        node->LastFrameFocused = g.FrameCount;
    ImU32 title_bar_col = GetColorU32(host_window.Collapsed ? ImGuiCol_TitleBgCollapsed : is_focused ? ImGuiCol_TitleBgActive : ImGuiCol_TitleBg);
    ImDrawFlags rounding_flags = CalcRoundingFlagsForRectInRect(title_bar_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE);
    host_window.DrawList->AddRectFilled(title_bar_rect.Min, title_bar_rect.Max, title_bar_col, host_window.WindowRounding, rounding_flags);

    // Docking/Collapse button
    if (has_window_menu_button)
    {
        if (CollapseButton(host_window.GetID("#COLLAPSE"), window_menu_button_pos, node)) // == DockNodeGetWindowMenuButtonId(node)
            OpenPopup("#WindowMenu");
        if (IsItemActive())
            focus_tab_id = tab_bar->SelectedTabId;
    }

    // If multiple tabs are appearing on the same frame, sort them based on their persistent dock_order value
    int tabs_unsorted_start = tab_bar->Tabs.Size;
    for (int tab_n = tab_bar->Tabs.Size - 1; tab_n >= 0 && (tab_bar->Tabs[tab_n].Flags & ImGuiTabItemFlags_Unsorted); tab_n--)
    {
        // FIXME-DOCK: Consider only clearing the flag after the tab has been alive for a few consecutive frames, allowing late comers to not break sorting?
        tab_bar->Tabs[tab_n].Flags &= ~ImGuiTabItemFlags_Unsorted;
        tabs_unsorted_start = tab_n;
    }
    if (tab_bar->Tabs.Size > tabs_unsorted_start)
    {
        IMGUI_DEBUG_LOG_DOCKING("[docking] In node 0x%08X: %d new appearing tabs:%s\n", node->ID, tab_bar->Tabs.Size - tabs_unsorted_start, (tab_bar->Tabs.Size > tabs_unsorted_start + 1) ? " (will sort)" : "");
        for (int tab_n = tabs_unsorted_start; tab_n < tab_bar->Tabs.Size; tab_n += 1)
            IMGUI_DEBUG_LOG_DOCKING("[docking] - Tab '%s' Order %d\n", tab_bar->Tabs[tab_n].Window->Name, tab_bar->Tabs[tab_n].Window->DockOrder);
        if (tab_bar->Tabs.Size > tabs_unsorted_start + 1)
            ImQsort(tab_bar->Tabs.Data + tabs_unsorted_start, tab_bar->Tabs.Size - tabs_unsorted_start, sizeof(ImGuiTabItem), TabItemComparerByDockOrder);
    }

    // Apply nav_window focus back to the tab bar
    if (g.nav_window && g.nav_window->RootWindow->DockNode == node)
        tab_bar->SelectedTabId = g.nav_window->RootWindow->ID;

    // Selected newly added tabs, or persistent tab id if the tab bar was just recreated
    if (tab_bar_is_recreated && TabBarFindTabByID(tab_bar, node->SelectedTabId) != NULL)
        tab_bar->SelectedTabId = tab_bar->NextSelectedTabId = node->SelectedTabId;
    else if (tab_bar->Tabs.Size > tabs_count_old)
        tab_bar->SelectedTabId = tab_bar->NextSelectedTabId = tab_bar->Tabs.back().Window->TabId;

    // Begin tab bar
    ImGuiTabBarFlags tab_bar_flags = ImGuiTabBarFlags_Reorderable | ImGuiTabBarFlags_AutoSelectNewTabs; // | ImGuiTabBarFlags_NoTabListScrollingButtons);
    tab_bar_flags |= ImGuiTabBarFlags_SaveSettings | ImGuiTabBarFlags_DockNode;
    if (!host_window.Collapsed && is_focused)
        tab_bar_flags |= ImGuiTabBarFlags_IsFocused;
    BeginTabBarEx(tab_bar, tab_bar_rect, tab_bar_flags, node);
    //host_window->draw_list->add_rect(tab_bar_rect.min, tab_bar_rect.max, IM_COL32(255,0,255,255));

    // Backup style colors
    Vector4D backup_style_cols[ImGuiWindowDockStyleCol_COUNT];
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        backup_style_cols[color_n] = g.Style.Colors[GWindowDockStyleColors[color_n]];

    // Submit actual tabs
    node->VisibleWindow = NULL;
    for (int window_n = 0; window_n < node->Windows.Size; window_n += 1)
    {
        ImGuiWindow* window = node->Windows[window_n];
        if ((closed_all || closed_one == window.TabId) && window.HasCloseButton && !(window.Flags & ImGuiWindowFlags_UnsavedDocument))
            continue;
        if (window.LastFrameActive + 1 >= g.FrameCount || !node_was_active)
        {
            ImGuiTabItemFlags tab_item_flags = 0;
            tab_item_flags |= window.WindowClass.TabItemFlagsOverrideSet;
            if (window.Flags & ImGuiWindowFlags_UnsavedDocument)
                tab_item_flags |= ImGuiTabItemFlags_UnsavedDocument;
            if (tab_bar.flags & ImGuiTabBarFlags_NoCloseWithMiddleMouseButton)
                tab_item_flags |= ImGuiTabItemFlags_NoCloseWithMiddleMouseButton;

            // Apply stored style overrides for the window
            for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
                g.Style.Colors[GWindowDockStyleColors[color_n]] = ColorConvertU32ToFloat4(window.DockStyle.Colors[color_n]);

            // Note that TabItemEx() calls TabBarCalcTabID() so our tab item id will ignore the current id stack (rightly so)
            bool tab_open = true;
            TabItemEx(tab_bar, window.Name, window.HasCloseButton ? &tab_open : NULL, tab_item_flags, window);
            if (!tab_open)
                node->WantCloseTabId = window.TabId;
            if (tab_bar->VisibleTabId == window.TabId)
                node->VisibleWindow = window;

            // Store last item data so it can be queried with IsItemXXX functions after the user Begin() call
            window.DockTabItemStatusFlags = g.last_item_data.StatusFlags;
            window.DockTabItemRect = g.last_item_data.Rect;

            // Update navigation id on menu layer
            if (g.nav_window && g.nav_window->RootWindow == window && (window.DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) == 0)
                host_window.NavLastIds[1] = window.TabId;
        }
    }

    // Restore style colors
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        g.Style.Colors[GWindowDockStyleColors[color_n]] = backup_style_cols[color_n];

    // Notify root of visible window (used to display title in OS task bar)
    if (node->VisibleWindow)
        if (is_focused || root_node->VisibleWindow == NULL)
            root_node->VisibleWindow = node->VisibleWindow;

    // Close button (after visible_window was updated)
    // Note that visible_window may have been overrided by CTRL+Tabbing, so visible_window->tab_id may be != from tab_bar->selected_tab_id
    const bool close_button_is_enabled = node->HasCloseButton && node->VisibleWindow && node->VisibleWindow->HasCloseButton;
    const bool close_button_is_visible = node->HasCloseButton;
    //const bool close_button_is_visible = close_button_is_enabled; // Most people would expect this behavior of not even showing the button (leaving a hole since we can't claim that space as other windows in the tba bar have one)
    if (close_button_is_visible)
    {
        if (!close_button_is_enabled)
        {
            PushItemFlag(ImGuiItemFlags_Disabled, true);
            PushStyleColor(ImGuiCol_Text, style.Colors[ImGuiCol_Text] * Vector4D(1.0,1.0,1.0,0.4));
        }
        if (CloseButton(host_window.GetID("#CLOSE"), close_button_pos))
        {
            node->WantCloseAll = true;
            for (int n = 0; n < tab_bar->Tabs.Size; n += 1)
                TabBarCloseTab(tab_bar, &tab_bar->Tabs[n]);
        }
        //if (IsItemActive())
        //    focus_tab_id = tab_bar->selected_tab_id;
        if (!close_button_is_enabled)
        {
            PopStyleColor();
            PopItemFlag();
        }
    }

    // When clicking on the title bar outside of tabs, we still focus the selected tab for that node
    // FIXME: TabItem use AllowItemOverlap so we manually perform a more specific test for now (hovered || held)
    ImGuiID title_bar_id = host_window.GetID("#TITLEBAR");
    if (g.hovered_id == 0 || g.hovered_id == title_bar_id || g.active_id == title_bar_id)
    {
        bool held;
        ButtonBehavior(title_bar_rect, title_bar_id, NULL, &held, ImGuiButtonFlags_AllowItemOverlap);
        if (g.hovered_id == title_bar_id)
        {
            // ImGuiButtonFlags_AllowItemOverlap + SetItemAllowOverlap() required for appending into dock node tab bar,
            // otherwise dragging window will steal hovered_id and amended tabs cannot get them.
            g.last_item_data.ID = title_bar_id;
            SetItemAllowOverlap();
        }
        if (held)
        {
            if (IsMouseClicked(0))
                focus_tab_id = tab_bar->SelectedTabId;

            // Forward moving request to selected window
            if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, tab_bar->SelectedTabId))
                StartMouseMovingWindowOrNode(tab->Window ? tab->Window : node->HostWindow, node, false);
        }
    }

    // Forward focus from host node to selected window
    //if (is_focused && g.nav_window == host_window && !g.nav_windowing_target)
    //    focus_tab_id = tab_bar->selected_tab_id;

    // When clicked on a tab we requested focus to the docked child
    // This overrides the value set by "forward focus from host node to selected window".
    if (tab_bar->NextSelectedTabId)
        focus_tab_id = tab_bar->NextSelectedTabId;

    // Apply navigation focus
    if (focus_tab_id != 0)
        if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, focus_tab_id))
            if (tab->Window)
            {
                focus_window(tab->Window);
                NavInitWindow(tab->Window, false);
            }

    EndTabBar();
    PopID();

    // Restore skip_items flag
    if (!node->IsDockSpace())
    {
        host_window.DC.NavLayerCurrent = ImGuiNavLayer_Main;
        host_window.SkipItems = backup_skip_item;
    }
}

static void ImGui::DockNodeAddTabBar(ImGuiDockNode* node)
{
    IM_ASSERT(node->TabBar == NULL);
    node->TabBar = IM_NEW(ImGuiTabBar);
}

static void ImGui::DockNodeRemoveTabBar(ImGuiDockNode* node)
{
    if (node->TabBar == NULL)
        return;
    IM_DELETE(node->TabBar);
    node->TabBar = NULL;
}

static bool DockNodeIsDropAllowedOne(ImGuiWindow* payload, ImGuiWindow* host_window)
{
    if (host_window.DockNodeAsHost && host_window.DockNodeAsHost->IsDockSpace() && payload->BeginOrderWithinContext < host_window.BeginOrderWithinContext)
        return false;

    ImGuiWindowClass* host_class = host_window.DockNodeAsHost ? &host_window.DockNodeAsHost->WindowClass : &host_window.WindowClass;
    ImGuiWindowClass* payload_class = &payload->WindowClass;
    if (host_class->ClassId != payload_class->ClassId)
    {
        if (host_class->ClassId != 0 && host_class->DockingAllowUnclassed && payload_class->ClassId == 0)
            return true;
        if (payload_class->ClassId != 0 && payload_class->DockingAllowUnclassed && host_class->ClassId == 0)
            return true;
        return false;
    }

    // Prevent docking any window created above a popup
    // Technically we should support it (e.g. in the case of a long-lived modal window that had fancy docking features),
    // by e.g. adding a 'if (!ImGui::IsWindowWithinBeginStackOf(host_window, popup_window))' test.
    // But it would requires more work on our end because the dock host windows is technically created in NewFrame()
    // and our ->ParentXXX and ->RootXXX pointers inside windows are currently mislading or lacking.
    ImGuiContext& g = *GImGui;
    for (int i = g.OpenPopupStack.Size - 1; i >= 0; i--)
        if (ImGuiWindow* popup_window = g.OpenPopupStack[i].Window)
            if (ImGui::IsWindowWithinBeginStackOf(payload, popup_window))   // Payload is created from within a popup begin stack.
                return false;

    return true;
}

static bool ImGui::DockNodeIsDropAllowed(ImGuiWindow* host_window, ImGuiWindow* root_payload)
{
    if (root_payload->DockNodeAsHost && root_payload->DockNodeAsHost->IsSplitNode()) // FIXME-DOCK: Missing filtering
        return true;

    const int payload_count = root_payload->DockNodeAsHost ? root_payload->DockNodeAsHost->Windows.Size : 1;
    for (int payload_n = 0; payload_n < payload_count; payload_n += 1)
    {
        ImGuiWindow* payload = root_payload->DockNodeAsHost ? root_payload->DockNodeAsHost->Windows[payload_n] : root_payload;
        if (DockNodeIsDropAllowedOne(payload, host_window))
            return true;
    }
    return false;
}

// window menu button == collapse button when not in a dock node.
// FIXME: This is similar to RenderWindowTitleBarContents(), may want to share code.
static void ImGui::DockNodeCalcTabBarLayout(const ImGuiDockNode* node, ImRect* out_title_rect, ImRect* out_tab_bar_rect, Vector2D* out_window_menu_button_pos, Vector2D* out_close_button_pos)
{
    ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;

    ImRect r = ImRect(node.pos.x, node.pos.y, node.pos.x + node->Size.x, node.pos.y + g.FontSize + g.Style.FramePadding.y * 2.0);
    if (out_title_rect) { *out_title_rect = r; }

    r.Min.x += style.WindowBorderSize;
    r.Max.x -= style.WindowBorderSize;

    float button_sz = g.FontSize;

    Vector2D window_menu_button_pos = r.Min;
    r.Min.x += style.FramePadding.x;
    r.Max.x -= style.FramePadding.x;
    if (node->HasCloseButton)
    {
        r.Max.x -= button_sz;
        if (out_close_button_pos) *out_close_button_pos = DimgVec2D::new(r.Max.x - style.FramePadding.x, r.Min.y);
    }
    if (node->HasWindowMenuButton && style.WindowMenuButtonPosition == ImGuiDir_Left)
    {
        r.Min.x += button_sz + style.ItemInnerSpacing.x;
    }
    else if (node->HasWindowMenuButton && style.WindowMenuButtonPosition == ImGuiDir_Right)
    {
        r.Max.x -= button_sz + style.FramePadding.x;
        window_menu_button_pos = DimgVec2D::new(r.Max.x, r.Min.y);
    }
    if (out_tab_bar_rect) { *out_tab_bar_rect = r; }
    if (out_window_menu_button_pos) { *out_window_menu_button_pos = window_menu_button_pos; }
}

void ImGui::DockNodeCalcSplitRects(Vector2D& pos_old, Vector2D& size_old, Vector2D& pos_new, Vector2D& size_new, ImGuiDir dir, Vector2D size_new_desired)
{
    ImGuiContext& g = *GImGui;
    const float dock_spacing = g.Style.ItemInnerSpacing.x;
    const ImGuiAxis axis = (dir == ImGuiDir_Left || dir == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;
    pos_new[axis ^ 1] = pos_old[axis ^ 1];
    size_new[axis ^ 1] = size_old[axis ^ 1];

    // Distribute size on given axis (with a desired size or equally)
    const float w_avail = size_old[axis] - dock_spacing;
    if (size_new_desired[axis] > 0.0 && size_new_desired[axis] <= w_avail * 0.5)
    {
        size_new[axis] = size_new_desired[axis];
        size_old[axis] = IM_FLOOR(w_avail - size_new[axis]);
    }
    else
    {
        size_new[axis] = IM_FLOOR(w_avail * 0.5);
        size_old[axis] = IM_FLOOR(w_avail - size_new[axis]);
    }

    // Position each node
    if (dir == ImGuiDir_Right || dir == ImGuiDir_Down)
    {
        pos_new[axis] = pos_old[axis] + size_old[axis] + dock_spacing;
    }
    else if (dir == ImGuiDir_Left || dir == ImGuiDir_Up)
    {
        pos_new[axis] = pos_old[axis];
        pos_old[axis] = pos_new[axis] + size_new[axis] + dock_spacing;
    }
}

// Retrieve the drop rectangles for a given direction or for the center + perform hit testing.
bool ImGui::DockNodeCalcDropRectsAndTestMousePos(const ImRect& parent, ImGuiDir dir, ImRect& out_r, bool outer_docking, Vector2D* test_mouse_pos)
{
    ImGuiContext& g = *GImGui;

    const float parent_smaller_axis = ImMin(parent.GetWidth(), parent.GetHeight());
    const float hs_for_central_nodes = ImMin(g.FontSize * 1.5, ImMax(g.FontSize * 0.5, parent_smaller_axis / 8.0));
    float hs_w; // Half-size, longer axis
    float hs_h; // Half-size, smaller axis
    Vector2D off; // Distance from edge or center
    if (outer_docking)
    {
        //hs_w = ImFloor(ImClamp(parent_smaller_axis - hs_for_central_nodes * 4.0, g.font_size * 0.5, g.font_size * 8.0));
        //hs_h = ImFloor(hs_w * 0.15);
        //off = Vector2D(ImFloor(parent.get_width() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h), ImFloor(parent.get_height() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h));
        hs_w = ImFloor(hs_for_central_nodes * 1.50);
        hs_h = ImFloor(hs_for_central_nodes * 0.80);
        off = DimgVec2D::new(ImFloor(parent.GetWidth() * 0.5 - hs_h), ImFloor(parent.GetHeight() * 0.5 - hs_h));
    }
    else
    {
        hs_w = ImFloor(hs_for_central_nodes);
        hs_h = ImFloor(hs_for_central_nodes * 0.90);
        off = DimgVec2D::new(ImFloor(hs_w * 2.40), ImFloor(hs_w * 2.40));
    }

    Vector2D c = ImFloor(parent.GetCenter());
    if      (dir == ImGuiDir_None)  { out_r = ImRect(c.x - hs_w, c.y - hs_w,         c.x + hs_w, c.y + hs_w);         }
    else if (dir == ImGuiDir_Up)    { out_r = ImRect(c.x - hs_w, c.y - off.y - hs_h, c.x + hs_w, c.y - off.y + hs_h); }
    else if (dir == ImGuiDir_Down)  { out_r = ImRect(c.x - hs_w, c.y + off.y - hs_h, c.x + hs_w, c.y + off.y + hs_h); }
    else if (dir == ImGuiDir_Left)  { out_r = ImRect(c.x - off.x - hs_h, c.y - hs_w, c.x - off.x + hs_h, c.y + hs_w); }
    else if (dir == ImGuiDir_Right) { out_r = ImRect(c.x + off.x - hs_h, c.y - hs_w, c.x + off.x + hs_h, c.y + hs_w); }

    if (test_mouse_pos == NULL)
        return false;

    ImRect hit_r = out_r;
    if (!outer_docking)
    {
        // Custom hit testing for the 5-way selection, designed to reduce flickering when moving diagonally between sides
        hit_r.Expand(ImFloor(hs_w * 0.30));
        Vector2D mouse_delta = (*test_mouse_pos - c);
        float mouse_delta_len2 = ImLengthSqr(mouse_delta);
        float r_threshold_center = hs_w * 1.4;
        float r_threshold_sides = hs_w * (1.4 + 1.2);
        if (mouse_delta_len2 < r_threshold_center * r_threshold_center)
            return (dir == ImGuiDir_None);
        if (mouse_delta_len2 < r_threshold_sides * r_threshold_sides)
            return (dir == ImGetDirQuadrantFromDelta(mouse_delta.x, mouse_delta.y));
    }
    return hit_r.Contains(*test_mouse_pos);
}

// host_node may be NULL if the window doesn't have a dock_node already.
// FIXME-DOCK: This is misnamed since it's also doing the filtering.
static void ImGui::DockNodePreviewDockSetup(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, ImGuiDockPreviewData* data, bool is_explicit_target, bool is_outer_docking)
{
    ImGuiContext& g = *GImGui;

    // There is an edge case when docking into a dockspace which only has inactive nodes.
    // In this case DockNodeTreeFindNodeByPos() will have selected a leaf node which is inactive.
    // Because the inactive leaf node doesn't have proper pos/size yet, we'll use the root node as reference.
    ImGuiDockNode* root_payload_as_host = root_payload->DockNodeAsHost;
    ImGuiDockNode* ref_node_for_rect = (host_node && !host_node->IsVisible) ? DockNodeGetRootNode(host_node) : host_node;
    if (ref_node_for_rect)
        IM_ASSERT(ref_node_for_rect->IsVisible == true);

    // Filter, figure out where we are allowed to dock
    ImGuiDockNodeFlags src_node_flags = root_payload_as_host ? root_payload_as_host->MergedFlags : root_payload->WindowClass.DockNodeFlagsOverrideSet;
    ImGuiDockNodeFlags dst_node_flags = host_node ? host_node->MergedFlags : host_window.WindowClass.DockNodeFlagsOverrideSet;
    data->IsCenterAvailable = true;
    if (is_outer_docking)
        data->IsCenterAvailable = false;
    else if (dst_node_flags & ImGuiDockNodeFlags_NoDocking)
        data->IsCenterAvailable = false;
    else if (host_node && (dst_node_flags & ImGuiDockNodeFlags_NoDockingInCentralNode) && host_node->IsCentralNode())
        data->IsCenterAvailable = false;
    else if ((!host_node || !host_node->IsEmpty()) && root_payload_as_host && root_payload_as_host->IsSplitNode() && (root_payload_as_host->OnlyNodeWithWindows == NULL)) // Is _visibly_ split?
        data->IsCenterAvailable = false;
    else if (dst_node_flags & ImGuiDockNodeFlags_NoDockingOverMe)
        data->IsCenterAvailable = false;
    else if ((src_node_flags & ImGuiDockNodeFlags_NoDockingOverOther) && (!host_node || !host_node->IsEmpty()))
        data->IsCenterAvailable = false;
    else if ((src_node_flags & ImGuiDockNodeFlags_NoDockingOverEmpty) && host_node && host_node->IsEmpty())
        data->IsCenterAvailable = false;

    data->IsSidesAvailable = true;
    if ((dst_node_flags & ImGuiDockNodeFlags_NoSplit) || g.io.ConfigDockingNoSplit)
        data->IsSidesAvailable = false;
    else if (!is_outer_docking && host_node && host_node->ParentNode == NULL && host_node->IsCentralNode())
        data->IsSidesAvailable = false;
    else if ((dst_node_flags & ImGuiDockNodeFlags_NoDockingSplitMe) || (src_node_flags & ImGuiDockNodeFlags_NoDockingSplitOther))
        data->IsSidesAvailable = false;

    // build a tentative future node (reuse same structure because it is practical. Shape will be readjusted when previewing a split)
    data->FutureNode.HasCloseButton = (host_node ? host_node->HasCloseButton : host_window.HasCloseButton) || (root_payload->HasCloseButton);
    data->FutureNode.HasWindowMenuButton = host_node ? true : ((host_window.Flags & ImGuiWindowFlags_NoCollapse) == 0);
    data->FutureNode.Pos = ref_node_for_rect ? ref_node_for_rect.pos : host_window.Pos;
    data->FutureNode.Size = ref_node_for_rect ? ref_node_for_rect->Size : host_window.Size;

    // Calculate drop shapes geometry for allowed splitting directions
    IM_ASSERT(ImGuiDir_None == -1);
    data->SplitNode = host_node;
    data->SplitDir = ImGuiDir_None;
    data->IsSplitDirExplicit = false;
    if (!host_window.Collapsed)
        for (int dir = ImGuiDir_None; dir < ImGuiDir_COUNT; dir += 1)
        {
            if (dir == ImGuiDir_None && !data->IsCenterAvailable)
                continue;
            if (dir != ImGuiDir_None && !data->IsSidesAvailable)
                continue;
            if (DockNodeCalcDropRectsAndTestMousePos(data->FutureNode.Rect(), (ImGuiDir)dir, data->DropRectsDraw[dir+1], is_outer_docking, &g.io.MousePos))
            {
                data->SplitDir = (ImGuiDir)dir;
                data->IsSplitDirExplicit = true;
            }
        }

    // When docking without holding Shift, we only allow and preview docking when hovering over a drop rect or over the title bar
    data->IsDropAllowed = (data->SplitDir != ImGuiDir_None) || (data->IsCenterAvailable);
    if (!is_explicit_target && !data->IsSplitDirExplicit && !g.io.ConfigDockingWithShift)
        data->IsDropAllowed = false;

    // Calculate split area
    data->SplitRatio = 0.0;
    if (data->SplitDir != ImGuiDir_None)
    {
        ImGuiDir split_dir = data->SplitDir;
        ImGuiAxis split_axis = (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;
        Vector2D pos_new, pos_old = data->FutureNode.Pos;
        Vector2D size_new, size_old = data->FutureNode.Size;
        DockNodeCalcSplitRects(pos_old, size_old, pos_new, size_new, split_dir, root_payload->Size);

        // Calculate split ratio so we can pass it down the docking request
        float split_ratio = ImSaturate(size_new[split_axis] / data->FutureNode.Size[split_axis]);
        data->FutureNode.Pos = pos_new;
        data->FutureNode.Size = size_new;
        data->SplitRatio = (split_dir == ImGuiDir_Right || split_dir == ImGuiDir_Down) ? (1.0 - split_ratio) : (split_ratio);
    }
}

static void ImGui::DockNodePreviewDockRender(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, const ImGuiDockPreviewData* data)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.CurrentWindow == host_window);   // Because we rely on font size to calculate tab sizes

    // With this option, we only display the preview on the target viewport, and the payload viewport is made transparent.
    // To compensate for the single layer obstructed by the payload, we'll increase the alpha of the preview nodes.
    const bool is_transparent_payload = g.io.ConfigDockingTransparentPayload;

    // In case the two windows involved are on different viewports, we will draw the overlay on each of them.
    int overlay_draw_lists_count = 0;
    ImDrawList* overlay_draw_lists[2];
    overlay_draw_lists[overlay_draw_lists_count += 1] = GetForegroundDrawList(host_window.viewport);
    if (host_window.viewport != root_payload->Viewport && !is_transparent_payload)
        overlay_draw_lists[overlay_draw_lists_count += 1] = GetForegroundDrawList(root_payload->Viewport);

    // Draw main preview rectangle
    const ImU32 overlay_col_main = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 0.60 : 0.40);
    const ImU32 overlay_col_drop = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 0.90 : 0.70);
    const ImU32 overlay_col_drop_hovered = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 1.20 : 1.00);
    const ImU32 overlay_col_lines = GetColorU32(ImGuiCol_NavWindowingHighlight, is_transparent_payload ? 0.80 : 0.60);

    // Display area preview
    const bool can_preview_tabs = (root_payload->DockNodeAsHost == NULL || root_payload->DockNodeAsHost->Windows.Size > 0);
    if (data->IsDropAllowed)
    {
        ImRect overlay_rect = data->FutureNode.Rect();
        if (data->SplitDir == ImGuiDir_None && can_preview_tabs)
            overlay_rect.Min.y += GetFrameHeight();
        if (data->SplitDir != ImGuiDir_None || data->IsCenterAvailable)
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
                overlay_draw_lists[overlay_n]->AddRectFilled(overlay_rect.Min, overlay_rect.Max, overlay_col_main, host_window.WindowRounding, CalcRoundingFlagsForRectInRect(overlay_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE));
    }

    // Display tab shape/label preview unless we are splitting node (it generally makes the situation harder to read)
    if (data->IsDropAllowed && can_preview_tabs && data->SplitDir == ImGuiDir_None && data->IsCenterAvailable)
    {
        // Compute target tab bar geometry so we can locate our preview tabs
        ImRect tab_bar_rect;
        DockNodeCalcTabBarLayout(&data->FutureNode, NULL, &tab_bar_rect, NULL, NULL);
        Vector2D tab_pos = tab_bar_rect.Min;
        if (host_node && host_node->TabBar)
        {
            if (!host_node->IsHiddenTabBar() && !host_node->IsNoTabBar())
                tab_pos.x += host_node->TabBar->WidthAllTabs + g.Style.ItemInnerSpacing.x; // We don't use OffsetNewTab because when using non-persistent-order tab bar it is incremented with each Tab submission.
            else
                tab_pos.x += g.Style.ItemInnerSpacing.x + TabItemCalcSize(host_node->Windows[0]->Name, host_node->Windows[0]->HasCloseButton).x;
        }
        else if (!(host_window.Flags & ImGuiWindowFlags_DockNodeHost))
        {
            tab_pos.x += g.Style.ItemInnerSpacing.x + TabItemCalcSize(host_window.Name, host_window.HasCloseButton).x; // Account for slight offset which will be added when changing from title bar to tab bar
        }

        // Draw tab shape/label preview (payload may be a loose window or a host window carrying multiple tabbed windows)
        if (root_payload->DockNodeAsHost)
            IM_ASSERT(root_payload->DockNodeAsHost->Windows.Size <= root_payload->DockNodeAsHost->TabBar->Tabs.Size);
        ImGuiTabBar* tab_bar_with_payload = root_payload->DockNodeAsHost ? root_payload->DockNodeAsHost->TabBar : NULL;
        const int payload_count = tab_bar_with_payload ? tab_bar_with_payload->Tabs.Size : 1;
        for (int payload_n = 0; payload_n < payload_count; payload_n += 1)
        {
            // dock_node's tab_bar may have non-window Tabs manually appended by user
            ImGuiWindow* payload_window = tab_bar_with_payload ? tab_bar_with_payload->Tabs[payload_n].Window : root_payload;
            if (tab_bar_with_payload && payload_window == NULL)
                continue;
            if (!DockNodeIsDropAllowedOne(payload_window, host_window))
                continue;

            // Calculate the tab bounding box for each payload window
            Vector2D tab_size = TabItemCalcSize(payload_window.Name, payload_window.HasCloseButton);
            ImRect tab_bb(tab_pos.x, tab_pos.y, tab_pos.x + tab_size.x, tab_pos.y + tab_size.y);
            tab_pos.x += tab_size.x + g.Style.ItemInnerSpacing.x;
            const ImU32 overlay_col_text = GetColorU32(payload_window.DockStyle.Colors[ImGuiWindowDockStyleCol_Text]);
            const ImU32 overlay_col_tabs = GetColorU32(payload_window.DockStyle.Colors[ImGuiWindowDockStyleCol_TabActive]);
            PushStyleColor(ImGuiCol_Text, overlay_col_text);
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
            {
                ImGuiTabItemFlags tab_flags = ImGuiTabItemFlags_Preview | ((payload_window.Flags & ImGuiWindowFlags_UnsavedDocument) ? ImGuiTabItemFlags_UnsavedDocument : 0);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n]->PushClipRect(tab_bar_rect.Min, tab_bar_rect.Max);
                TabItemBackground(overlay_draw_lists[overlay_n], tab_bb, tab_flags, overlay_col_tabs);
                TabItemLabelAndCloseButton(overlay_draw_lists[overlay_n], tab_bb, tab_flags, g.Style.FramePadding, payload_window.Name, 0, 0, false, NULL, NULL);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n]->PopClipRect();
            }
            PopStyleColor();
        }
    }

    // Display drop boxes
    const float overlay_rounding = ImMax(3.0, g.Style.FrameRounding);
    for (int dir = ImGuiDir_None; dir < ImGuiDir_COUNT; dir += 1)
    {
        if (!data->DropRectsDraw[dir + 1].IsInverted())
        {
            ImRect draw_r = data->DropRectsDraw[dir + 1];
            ImRect draw_r_in = draw_r;
            draw_r_in.Expand(-2.0);
            ImU32 overlay_col = (data->SplitDir == (ImGuiDir)dir && data->IsSplitDirExplicit) ? overlay_col_drop_hovered : overlay_col_drop;
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
            {
                Vector2D center = ImFloor(draw_r_in.GetCenter());
                overlay_draw_lists[overlay_n]->AddRectFilled(draw_r.Min, draw_r.Max, overlay_col, overlay_rounding);
                overlay_draw_lists[overlay_n]->AddRect(draw_r_in.Min, draw_r_in.Max, overlay_col_lines, overlay_rounding);
                if (dir == ImGuiDir_Left || dir == ImGuiDir_Right)
                    overlay_draw_lists[overlay_n]->AddLine(DimgVec2D::new(center.x, draw_r_in.Min.y), DimgVec2D::new(center.x, draw_r_in.Max.y), overlay_col_lines);
                if (dir == ImGuiDir_Up || dir == ImGuiDir_Down)
                    overlay_draw_lists[overlay_n]->AddLine(DimgVec2D::new(draw_r_in.Min.x, center.y), DimgVec2D::new(draw_r_in.Max.x, center.y), overlay_col_lines);
            }
        }

        // Stop after ImGuiDir_None
        if ((host_node && (host_node->MergedFlags & ImGuiDockNodeFlags_NoSplit)) || g.io.ConfigDockingNoSplit)
            return;
    }
}

//-----------------------------------------------------------------------------
// Docking: ImGuiDockNode Tree manipulation functions
//-----------------------------------------------------------------------------
// - DockNodeTreeSplit()
// - DockNodeTreeMerge()
// - DockNodeTreeUpdatePosSize()
// - DockNodeTreeUpdateSplitterFindTouchingNode()
// - DockNodeTreeUpdateSplitter()
// - DockNodeTreeFindFallbackLeafNode()
// - DockNodeTreeFindNodeByPos()
//-----------------------------------------------------------------------------

void ImGui::DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, int split_inheritor_child_idx, float split_ratio, ImGuiDockNode* new_node)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(split_axis != ImGuiAxis_None);

    ImGuiDockNode* child_0 = (new_node && split_inheritor_child_idx != 0) ? new_node : DockContextAddNode(ctx, 0);
    child_0->ParentNode = parent_node;

    ImGuiDockNode* child_1 = (new_node && split_inheritor_child_idx != 1) ? new_node : DockContextAddNode(ctx, 0);
    child_1->ParentNode = parent_node;

    ImGuiDockNode* child_inheritor = (split_inheritor_child_idx == 0) ? child_0 : child_1;
    DockNodeMoveChildNodes(child_inheritor, parent_node);
    parent_node->ChildNodes[0] = child_0;
    parent_node->ChildNodes[1] = child_1;
    parent_node->ChildNodes[split_inheritor_child_idx]->VisibleWindow = parent_node->VisibleWindow;
    parent_node->SplitAxis = split_axis;
    parent_node->VisibleWindow = NULL;
    parent_node->AuthorityForPos = parent_node->AuthorityForSize = ImGuiDataAuthority_DockNode;

    float size_avail = (parent_node->Size[split_axis] - DOCKING_SPLITTER_SIZE);
    size_avail = ImMax(size_avail, g.Style.WindowMinSize[split_axis] * 2.0);
    IM_ASSERT(size_avail > 0.0); // If you created a node manually with DockBuilderAddNode(), you need to also call DockBuilderSetNodeSize() before splitting.
    child_0->SizeRef = child_1->SizeRef = parent_node->Size;
    child_0->SizeRef[split_axis] = ImFloor(size_avail * split_ratio);
    child_1->SizeRef[split_axis] = ImFloor(size_avail - child_0->SizeRef[split_axis]);

    DockNodeMoveWindows(parent_node->ChildNodes[split_inheritor_child_idx], parent_node);
    DockSettingsRenameNodeReferences(parent_node->ID, parent_node->ChildNodes[split_inheritor_child_idx]->ID);
    DockNodeUpdateHasCentralNodeChild(DockNodeGetRootNode(parent_node));
    DockNodeTreeUpdatePosSize(parent_node, parent_node.pos, parent_node->Size);

    // flags transfer (e.g. this is where we transfer the ImGuiDockNodeFlags_CentralNode property)
    child_0->SharedFlags = parent_node->SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    child_1->SharedFlags = parent_node->SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    child_inheritor->LocalFlags = parent_node->LocalFlags & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node->LocalFlags &= ~ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    child_0->UpdateMergedFlags();
    child_1->UpdateMergedFlags();
    parent_node->UpdateMergedFlags();
    if (child_inheritor->IsCentralNode())
        DockNodeGetRootNode(parent_node)->CentralNode = child_inheritor;
}

void ImGui::DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child)
{
    // When called from DockContextProcessUndockNode() it is possible that one of the child is NULL.
    ImGuiContext& g = *GImGui;
    ImGuiDockNode* child_0 = parent_node->ChildNodes[0];
    ImGuiDockNode* child_1 = parent_node->ChildNodes[1];
    IM_ASSERT(child_0 || child_1);
    IM_ASSERT(merge_lead_child == child_0 || merge_lead_child == child_1);
    if ((child_0 && child_0->Windows.Size > 0) || (child_1 && child_1->Windows.Size > 0))
    {
        IM_ASSERT(parent_node->TabBar == NULL);
        IM_ASSERT(parent_node->Windows.Size == 0);
    }
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeTreeMerge: 0x%08X + 0x%08X back into parent 0x%08X\n", child_0 ? child_0->ID : 0, child_1 ? child_1->ID : 0, parent_node->ID);

    Vector2D backup_last_explicit_size = parent_node->SizeRef;
    DockNodeMoveChildNodes(parent_node, merge_lead_child);
    if (child_0)
    {
        DockNodeMoveWindows(parent_node, child_0); // Generally only 1 of the 2 child node will have windows
        DockSettingsRenameNodeReferences(child_0->ID, parent_node->ID);
    }
    if (child_1)
    {
        DockNodeMoveWindows(parent_node, child_1);
        DockSettingsRenameNodeReferences(child_1->ID, parent_node->ID);
    }
    DockNodeApplyPosSizeToWindows(parent_node);
    parent_node->AuthorityForPos = parent_node->AuthorityForSize = parent_node->AuthorityForViewport = ImGuiDataAuthority_Auto;
    parent_node->VisibleWindow = merge_lead_child->VisibleWindow;
    parent_node->SizeRef = backup_last_explicit_size;

    // flags transfer
    parent_node->LocalFlags &= ~ImGuiDockNodeFlags_LocalFlagsTransferMask_; // Preserve Dockspace flag
    parent_node->LocalFlags |= (child_0 ? child_0->LocalFlags : 0) & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node->LocalFlags |= (child_1 ? child_1->LocalFlags : 0) & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node->LocalFlagsInWindows = (child_0 ? child_0->LocalFlagsInWindows : 0) | (child_1 ? child_1->LocalFlagsInWindows : 0); // FIXME: Would be more consistent to update from actual windows
    parent_node->UpdateMergedFlags();

    if (child_0)
    {
        ctx->DockContext.Nodes.SetVoidPtr(child_0->ID, NULL);
        IM_DELETE(child_0);
    }
    if (child_1)
    {
        ctx->DockContext.Nodes.SetVoidPtr(child_1->ID, NULL);
        IM_DELETE(child_1);
    }
}

// Update pos/size for a node hierarchy (don't affect child windows yet)
// (Depth-first, Pre-Order)
void ImGui::DockNodeTreeUpdatePosSize(ImGuiDockNode* node, Vector2D pos, Vector2D size, ImGuiDockNode* only_write_to_single_node)
{
    // During the regular dock node update we write to all nodes.
    // 'only_write_to_single_node' is only set when turning a node visible mid-frame and we need its size right-away.
    const bool write_to_node = only_write_to_single_node == NULL || only_write_to_single_node == node;
    if (write_to_node)
    {
        node.pos = pos;
        node->Size = size;
    }

    if (node->IsLeafNode())
        return;

    ImGuiDockNode* child_0 = node->ChildNodes[0];
    ImGuiDockNode* child_1 = node->ChildNodes[1];
    Vector2D child_0_pos = pos, child_1_pos = pos;
    Vector2D child_0_size = size, child_1_size = size;

    const bool child_0_is_toward_single_node = (only_write_to_single_node != NULL && DockNodeIsInHierarchyOf(only_write_to_single_node, child_0));
    const bool child_1_is_toward_single_node = (only_write_to_single_node != NULL && DockNodeIsInHierarchyOf(only_write_to_single_node, child_1));
    const bool child_0_is_or_will_be_visible = child_0->IsVisible || child_0_is_toward_single_node;
    const bool child_1_is_or_will_be_visible = child_1->IsVisible || child_1_is_toward_single_node;

    if (child_0_is_or_will_be_visible && child_1_is_or_will_be_visible)
    {
        ImGuiContext& g = *GImGui;
        const float spacing = DOCKING_SPLITTER_SIZE;
        const ImGuiAxis axis = (ImGuiAxis)node->SplitAxis;
        const float size_avail = ImMax(size[axis] - spacing, 0.0);

        // size allocation policy
        // 1) The first 0..WindowMinSize[axis]*2 are allocated evenly to both windows.
        const float size_min_each = ImFloor(ImMin(size_avail, g.Style.WindowMinSize[axis] * 2.0) * 0.5);

        // FIXME: Blocks 2) and 3) are essentially doing nearly the same thing.
        // Difference are: write-back to size_ref; application of a minimum size; rounding before ImFloor()
        // Clarify and rework differences between size & size_ref and purpose of WantLockSizeOnce

        // 2) Process locked absolute size (during a splitter resize we preserve the child of nodes not touching the splitter edge)
        if (child_0->WantLockSizeOnce && !child_1->WantLockSizeOnce)
        {
            child_0_size[axis] = child_0->SizeRef[axis] = ImMin(size_avail - 1.0, child_0->Size[axis]);
            child_1_size[axis] = child_1->SizeRef[axis] = (size_avail - child_0_size[axis]);
            IM_ASSERT(child_0->SizeRef[axis] > 0.0 && child_1->SizeRef[axis] > 0.0);
        }
        else if (child_1->WantLockSizeOnce && !child_0->WantLockSizeOnce)
        {
            child_1_size[axis] = child_1->SizeRef[axis] = ImMin(size_avail - 1.0, child_1->Size[axis]);
            child_0_size[axis] = child_0->SizeRef[axis] = (size_avail - child_1_size[axis]);
            IM_ASSERT(child_0->SizeRef[axis] > 0.0 && child_1->SizeRef[axis] > 0.0);
        }
        else if (child_0->WantLockSizeOnce && child_1->WantLockSizeOnce)
        {
            // FIXME-DOCK: We cannot honor the requested size, so apply ratio.
            // Currently this path will only be taken if code programmatically sets WantLockSizeOnce
            float split_ratio = child_0_size[axis] / (child_0_size[axis] + child_1_size[axis]);
            child_0_size[axis] = child_0->SizeRef[axis] = ImFloor(size_avail * split_ratio);
            child_1_size[axis] = child_1->SizeRef[axis] = (size_avail - child_0_size[axis]);
            IM_ASSERT(child_0->SizeRef[axis] > 0.0 && child_1->SizeRef[axis] > 0.0);
        }

        // 3) If one window is the central node (~ use remaining space, should be made explicit!), use explicit size from the other, and remainder for the central node
        else if (child_0->SizeRef[axis] != 0.0 && child_1->HasCentralNodeChild)
        {
            child_0_size[axis] = ImMin(size_avail - size_min_each, child_0->SizeRef[axis]);
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }
        else if (child_1->SizeRef[axis] != 0.0 && child_0->HasCentralNodeChild)
        {
            child_1_size[axis] = ImMin(size_avail - size_min_each, child_1->SizeRef[axis]);
            child_0_size[axis] = (size_avail - child_1_size[axis]);
        }
        else
        {
            // 4) Otherwise distribute according to the relative ratio of each size_ref value
            float split_ratio = child_0->SizeRef[axis] / (child_0->SizeRef[axis] + child_1->SizeRef[axis]);
            child_0_size[axis] = ImMax(size_min_each, ImFloor(size_avail * split_ratio + 0.5));
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }

        child_1_pos[axis] += spacing + child_0_size[axis];
    }

    if (only_write_to_single_node == NULL)
        child_0->WantLockSizeOnce = child_1->WantLockSizeOnce = false;

    const bool child_0_recurse = only_write_to_single_node ? child_0_is_toward_single_node : child_0->IsVisible;
    const bool child_1_recurse = only_write_to_single_node ? child_1_is_toward_single_node : child_1->IsVisible;
    if (child_0_recurse)
        DockNodeTreeUpdatePosSize(child_0, child_0_pos, child_0_size);
    if (child_1_recurse)
        DockNodeTreeUpdatePosSize(child_1, child_1_pos, child_1_size);
}

static void DockNodeTreeUpdateSplitterFindTouchingNode(ImGuiDockNode* node, ImGuiAxis axis, int side, ImVector<ImGuiDockNode*>* touching_nodes)
{
    if (node->IsLeafNode())
    {
        touching_nodes->push_back(node);
        return;
    }
    if (node->ChildNodes[0]->IsVisible)
        if (node->SplitAxis != axis || side == 0 || !node->ChildNodes[1]->IsVisible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node->ChildNodes[0], axis, side, touching_nodes);
    if (node->ChildNodes[1]->IsVisible)
        if (node->SplitAxis != axis || side == 1 || !node->ChildNodes[0]->IsVisible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node->ChildNodes[1], axis, side, touching_nodes);
}

// (Depth-First, Pre-Order)
void ImGui::DockNodeTreeUpdateSplitter(ImGuiDockNode* node)
{
    if (node->IsLeafNode())
        return;

    ImGuiContext& g = *GImGui;

    ImGuiDockNode* child_0 = node->ChildNodes[0];
    ImGuiDockNode* child_1 = node->ChildNodes[1];
    if (child_0->IsVisible && child_1->IsVisible)
    {
        // Bounding box of the splitter cover the space between both nodes (w = Spacing, h = size[xy^1] for when splitting horizontally)
        const ImGuiAxis axis = (ImGuiAxis)node->SplitAxis;
        IM_ASSERT(axis != ImGuiAxis_None);
        ImRect bb;
        bb.Min = child_0.pos;
        bb.Max = child_1.pos;
        bb.Min[axis] += child_0->Size[axis];
        bb.Max[axis ^ 1] += child_1->Size[axis ^ 1];
        //if (g.io.key_ctrl) GetForegroundDrawList(g.current_window->viewport)->add_rect(bb.min, bb.max, IM_COL32(255,0,255,255));

        const ImGuiDockNodeFlags merged_flags = child_0->MergedFlags | child_1->MergedFlags; // Merged flags for BOTH childs
        const ImGuiDockNodeFlags no_resize_axis_flag = (axis == ImGuiAxis_X) ? ImGuiDockNodeFlags_NoResizeX : ImGuiDockNodeFlags_NoResizeY;
        if ((merged_flags & ImGuiDockNodeFlags_NoResize) || (merged_flags & no_resize_axis_flag))
        {
            ImGuiWindow* window = g.CurrentWindow;
            window.DrawList->AddRectFilled(bb.Min, bb.Max, GetColorU32(ImGuiCol_Separator), g.Style.FrameRounding);
        }
        else
        {
            //bb.min[axis] += 1; // Display a little inward so highlight doesn't connect with nearby tabs on the neighbor node.
            //bb.max[axis] -= 1;
            PushID(node->ID);

            // Find resizing limits by gathering list of nodes that are touching the splitter line.
            ImVector<ImGuiDockNode*> touching_nodes[2];
            float min_size = g.Style.WindowMinSize[axis];
            float resize_limits[2];
            resize_limits[0] = node->ChildNodes[0].pos[axis] + min_size;
            resize_limits[1] = node->ChildNodes[1].pos[axis] + node->ChildNodes[1]->Size[axis] - min_size;

            ImGuiID splitter_id = GetID("##Splitter");
            if (g.active_id == splitter_id) // Only process when splitter is active
            {
                DockNodeTreeUpdateSplitterFindTouchingNode(child_0, axis, 1, &touching_nodes[0]);
                DockNodeTreeUpdateSplitterFindTouchingNode(child_1, axis, 0, &touching_nodes[1]);
                for (int touching_node_n = 0; touching_node_n < touching_nodes[0].Size; touching_node_n += 1)
                    resize_limits[0] = ImMax(resize_limits[0], touching_nodes[0][touching_node_n]->Rect().Min[axis] + min_size);
                for (int touching_node_n = 0; touching_node_n < touching_nodes[1].Size; touching_node_n += 1)
                    resize_limits[1] = ImMin(resize_limits[1], touching_nodes[1][touching_node_n]->Rect().Max[axis] - min_size);

                // [DEBUG] Render touching nodes & limits
                /*
                ImDrawList* draw_list = node->host_window ? GetForegroundDrawList(node->host_window) : GetForegroundDrawList(GetMainViewport());
                for (int n = 0; n < 2; n++)
                {
                    for (int touching_node_n = 0; touching_node_n < touching_nodes[n].size; touching_node_n++)
                        draw_list->add_rect(touching_nodes[n][touching_node_n]->pos, touching_nodes[n][touching_node_n]->pos + touching_nodes[n][touching_node_n]->size, IM_COL32(0, 255, 0, 255));
                    if (axis == ImGuiAxis_X)
                        draw_list->add_line(Vector2D(resize_limits[n], node->child_nodes[n]->pos.y), Vector2D(resize_limits[n], node->child_nodes[n]->pos.y + node->child_nodes[n]->size.y), IM_COL32(255, 0, 255, 255), 3.0);
                    else
                        draw_list->add_line(Vector2D(node->child_nodes[n]->pos.x, resize_limits[n]), Vector2D(node->child_nodes[n]->pos.x + node->child_nodes[n]->size.x, resize_limits[n]), IM_COL32(255, 0, 255, 255), 3.0);
                }
                */
            }

            // Use a short delay before highlighting the splitter (and changing the mouse cursor) in order for regular mouse movement to not highlight many splitters
            float cur_size_0 = child_0->Size[axis];
            float cur_size_1 = child_1->Size[axis];
            float min_size_0 = resize_limits[0] - child_0.pos[axis];
            float min_size_1 = child_1.pos[axis] + child_1->Size[axis] - resize_limits[1];
            ImU32 bg_col = GetColorU32(ImGuiCol_WindowBg);
            if (SplitterBehavior(bb, GetID("##Splitter"), axis, &cur_size_0, &cur_size_1, min_size_0, min_size_1, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER, bg_col))
            {
                if (touching_nodes[0].Size > 0 && touching_nodes[1].Size > 0)
                {
                    child_0->Size[axis] = child_0->SizeRef[axis] = cur_size_0;
                    child_1.pos[axis] -= cur_size_1 - child_1->Size[axis];
                    child_1->Size[axis] = child_1->SizeRef[axis] = cur_size_1;

                    // Lock the size of every node that is a sibling of the node we are touching
                    // This might be less desirable if we can merge sibling of a same axis into the same parental level.
                    for (int side_n = 0; side_n < 2; side_n += 1)
                        for (int touching_node_n = 0; touching_node_n < touching_nodes[side_n].Size; touching_node_n += 1)
                        {
                            ImGuiDockNode* touching_node = touching_nodes[side_n][touching_node_n];
                            //ImDrawList* draw_list = node->host_window ? GetForegroundDrawList(node->host_window) : GetForegroundDrawList(GetMainViewport());
                            //draw_list->add_rect(touching_node->pos, touching_node->pos + touching_node->size, IM_COL32(255, 128, 0, 255));
                            while (touching_node->ParentNode != node)
                            {
                                if (touching_node->ParentNode->SplitAxis == axis)
                                {
                                    // Mark other node so its size will be preserved during the upcoming call to DockNodeTreeUpdatePosSize().
                                    ImGuiDockNode* node_to_preserve = touching_node->ParentNode->ChildNodes[side_n];
                                    node_to_preserve->WantLockSizeOnce = true;
                                    //draw_list->add_rect(touching_node->pos, touching_node->rect().max, IM_COL32(255, 0, 0, 255));
                                    //draw_list->add_rect_filled(node_to_preserve->pos, node_to_preserve->rect().max, IM_COL32(0, 255, 0, 100));
                                }
                                touching_node = touching_node->ParentNode;
                            }
                        }

                    DockNodeTreeUpdatePosSize(child_0, child_0.pos, child_0->Size);
                    DockNodeTreeUpdatePosSize(child_1, child_1.pos, child_1->Size);
                    MarkIniSettingsDirty();
                }
            }
            PopID();
        }
    }

    if (child_0->IsVisible)
        DockNodeTreeUpdateSplitter(child_0);
    if (child_1->IsVisible)
        DockNodeTreeUpdateSplitter(child_1);
}

ImGuiDockNode* ImGui::DockNodeTreeFindFallbackLeafNode(ImGuiDockNode* node)
{
    if (node->IsLeafNode())
        return node;
    if (ImGuiDockNode* leaf_node = DockNodeTreeFindFallbackLeafNode(node->ChildNodes[0]))
        return leaf_node;
    if (ImGuiDockNode* leaf_node = DockNodeTreeFindFallbackLeafNode(node->ChildNodes[1]))
        return leaf_node;
    return NULL;
}

ImGuiDockNode* ImGui::DockNodeTreeFindVisibleNodeByPos(ImGuiDockNode* node, Vector2D pos)
{
    if (!node->IsVisible)
        return NULL;

    const float dock_spacing = 0.0;// g.style.ItemInnerSpacing.x; // FIXME: Relation to DOCKING_SPLITTER_SIZE?
    ImRect r(node.pos, node.pos + node->Size);
    r.Expand(dock_spacing * 0.5);
    bool inside = r.Contains(pos);
    if (!inside)
        return NULL;

    if (node->IsLeafNode())
        return node;
    if (ImGuiDockNode* hovered_node = DockNodeTreeFindVisibleNodeByPos(node->ChildNodes[0], pos))
        return hovered_node;
    if (ImGuiDockNode* hovered_node = DockNodeTreeFindVisibleNodeByPos(node->ChildNodes[1], pos))
        return hovered_node;

    return NULL;
}

//-----------------------------------------------------------------------------
// Docking: Public Functions (SetWindowDock, DockSpace, DockSpaceOverViewport)
//-----------------------------------------------------------------------------
// - SetWindowDock() [Internal]
// - DockSpace()
// - DockSpaceOverViewport()
//-----------------------------------------------------------------------------

// [Internal] Called via SetNextWindowDockID()
void ImGui::SetWindowDock(ImGuiWindow* window, ImGuiID dock_id, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowDockAllowFlags & cond) == 0)
        return;
    window.SetWindowDockAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    if (window.DockId == dock_id)
        return;

    // If the user attempt to set a dock id that is a split node, we'll dig within to find a suitable docking spot
    ImGuiContext* ctx = GImGui;
    if (ImGuiDockNode* new_node = DockContextFindNodeByID(ctx, dock_id))
        if (new_node->IsSplitNode())
        {
            // Policy: Find central node or latest focused node. We first move back to our root node.
            new_node = DockNodeGetRootNode(new_node);
            if (new_node->CentralNode)
            {
                IM_ASSERT(new_node->CentralNode->IsCentralNode());
                dock_id = new_node->CentralNode->ID;
            }
            else
            {
                dock_id = new_node->LastFocusedNodeId;
            }
        }

    if (window.DockId == dock_id)
        return;

    if (window.DockNode)
        DockNodeRemoveWindow(window.DockNode, window, 0);
    window.DockId = dock_id;
}

// Create an explicit dockspace node within an existing window. Also expose dock node flags and creates a central_node by default.
// The Central Node is always displayed even when empty and shrink/extend according to the requested size of its neighbors.
// DockSpace() needs to be submitted _before_ any window they can host. If you use a dockspace, submit it early in your app.
ImGuiID ImGui::DockSpace(ImGuiID id, const Vector2D& size_arg, ImGuiDockNodeFlags flags, const ImGuiWindowClass* window_class)
{
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;
    ImGuiWindow* window = GetCurrentWindow();
    if (!(g.io.ConfigFlags & ImGuiConfigFlags_DockingEnable))
        return 0;

    // Early out if parent window is hidden/collapsed
    // This is faster but also DockNodeUpdateTabBar() relies on TabBarLayout() running (which won't if skip_items=true) to set NextSelectedTabId = 0). See #2960.
    // If for whichever reason this is causing problem we would need to ensure that DockNodeUpdateTabBar() ends up clearing NextSelectedTabId even if skip_items=true.
    if (window.SkipItems)
        flags |= ImGuiDockNodeFlags_KeepAliveOnly;

    IM_ASSERT((flags & ImGuiDockNodeFlags_DockSpace) == 0);
    IM_ASSERT(id != 0);
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, id);
    if (!node)
    {
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X created\n", id);
        node = DockContextAddNode(ctx, id);
        node->SetLocalFlags(ImGuiDockNodeFlags_CentralNode);
    }
    if (window_class && window_class->ClassId != node->WindowClass.ClassId)
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X: setup window_class 0x%08X -> 0x%08X\n", id, node->WindowClass.ClassId, window_class->ClassId);
    node->SharedFlags = flags;
    node->WindowClass = window_class ? *window_class : ImGuiWindowClass();

    // When a DockSpace transitioned form implicit to explicit this may be called a second time
    // It is possible that the node has already been claimed by a docked window which appeared before the DockSpace() node, so we overwrite is_dock_space again.
    if (node->LastFrameActive == g.FrameCount && !(flags & ImGuiDockNodeFlags_KeepAliveOnly))
    {
        IM_ASSERT(node->IsDockSpace() == false && "Cannot call DockSpace() twice a frame with the same id");
        node->SetLocalFlags(node->LocalFlags | ImGuiDockNodeFlags_DockSpace);
        return id;
    }
    node->SetLocalFlags(node->LocalFlags | ImGuiDockNodeFlags_DockSpace);

    // Keep alive mode, this is allow windows docked into this node so stay docked even if they are not visible
    if (flags & ImGuiDockNodeFlags_KeepAliveOnly)
    {
        node->LastFrameAlive = g.FrameCount;
        return id;
    }

    const Vector2D content_avail = GetContentRegionAvail();
    Vector2D size = ImFloor(size_arg);
    if (size.x <= 0.0)
        size.x = ImMax(content_avail.x + size.x, 4.0); // Arbitrary minimum child size (0.0 causing too much issues)
    if (size.y <= 0.0)
        size.y = ImMax(content_avail.y + size.y, 4.0);
    IM_ASSERT(size.x > 0.0 && size.y > 0.0);

    node.pos = window.DC.CursorPos;
    node->Size = node->SizeRef = size;
    SetNextWindowPos(node.pos);
    SetNextWindowSize(node->Size);
    g.NextWindowData.PosUndock = false;

    // FIXME-DOCK: Why do we need a child window to host a dockspace, could we host it in the existing window?
    // FIXME-DOCK: What is the reason for not simply calling BeginChild()? (OK to have a reason but should be commented)
    ImGuiWindowFlags window_flags = ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_DockNodeHost;
    window_flags |= ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoTitleBar;
    window_flags |= ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoScrollWithMouse;
    window_flags |= ImGuiWindowFlags_NoBackground;

    char title[256];
    ImFormatString(title, IM_ARRAYSIZE(title), "%s/DockSpace_%08X", window.Name, id);

    PushStyleVar(ImGuiStyleVar_ChildBorderSize, 0.0);
    Begin(title, NULL, window_flags);
    PopStyleVar();

    ImGuiWindow* host_window = g.CurrentWindow;
    DockNodeSetupHostWindow(node, host_window);
    host_window.ChildId = window.GetID(title);
    node->OnlyNodeWithWindows = NULL;

    IM_ASSERT(node->IsRootNode());

    // We need to handle the rare case were a central node is missing.
    // This can happen if the node was first created manually with DockBuilderAddNode() but _without_ the ImGuiDockNodeFlags_Dockspace.
    // Doing it correctly would set the _CentralNode flags, which would then propagate according to subsequent split.
    // It would also be ambiguous to attempt to assign a central node while there are split nodes, so we wait until there's a single node remaining.
    // The specific sub-property of _CentralNode we are interested in recovering here is the "Don't delete when empty" property,
    // as it doesn't make sense for an empty dockspace to not have this property.
    if (node->IsLeafNode() && !node->IsCentralNode())
        node->SetLocalFlags(node->LocalFlags | ImGuiDockNodeFlags_CentralNode);

    // Update the node
    DockNodeUpdate(node);

    End();
    ItemSize(size);
    return id;
}

// Tips: Use with ImGuiDockNodeFlags_PassthruCentralNode!
// The limitation with this call is that your window won't have a menu bar.
// Even though we could pass window flags, it would also require the user to be able to call BeginMenuBar() somehow meaning we can't Begin/End in a single function.
// But you can also use BeginMainMenuBar(). If you really want a menu bar inside the same window as the one hosting the dockspace, you will need to copy this code somewhere and tweak it.
ImGuiID ImGui::DockSpaceOverViewport(const ImGuiViewport* viewport, ImGuiDockNodeFlags dockspace_flags, const ImGuiWindowClass* window_class)
{
    if (viewport == NULL)
        viewport = GetMainViewport();

    SetNextWindowPos(viewport->WorkPos);
    SetNextWindowSize(viewport->WorkSize);
    SetNextWindowViewport(viewport->ID);

    ImGuiWindowFlags host_window_flags = 0;
    host_window_flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoDocking;
    host_window_flags |= ImGuiWindowFlags_NoBringToFrontOnFocus | ImGuiWindowFlags_NoNavFocus;
    if (dockspace_flags & ImGuiDockNodeFlags_PassthruCentralNode)
        host_window_flags |= ImGuiWindowFlags_NoBackground;

    char label[32];
    ImFormatString(label, IM_ARRAYSIZE(label), "DockSpaceViewport_%08X", viewport->ID);

    PushStyleVar(ImGuiStyleVar_WindowRounding, 0.0);
    PushStyleVar(ImGuiStyleVar_WindowBorderSize, 0.0);
    PushStyleVar(ImGuiStyleVar_WindowPadding, DimgVec2D::new(0.0, 0.0));
    Begin(label, NULL, host_window_flags);
    PopStyleVar(3);

    ImGuiID dockspace_id = GetID("DockSpace");
    DockSpace(dockspace_id, DimgVec2D::new(0.0, 0.0), dockspace_flags, window_class);
    End();

    return dockspace_id;
}

//-----------------------------------------------------------------------------
// Docking: Builder Functions
//-----------------------------------------------------------------------------
// Very early end-user API to manipulate dock nodes.
// Only available in imgui_internal.h. Expect this API to change/break!
// It is expected that those functions are all called _before_ the dockspace node submission.
//-----------------------------------------------------------------------------
// - DockBuilderDockWindow()
// - DockBuilderGetNode()
// - DockBuilderSetNodePos()
// - DockBuilderSetNodeSize()
// - DockBuilderAddNode()
// - DockBuilderRemoveNode()
// - DockBuilderRemoveNodeChildNodes()
// - DockBuilderRemoveNodeDockedWindows()
// - DockBuilderSplitNode()
// - DockBuilderCopyNodeRec()
// - DockBuilderCopyNode()
// - DockBuilderCopyWindowSettings()
// - DockBuilderCopyDockSpace()
// - DockBuilderFinish()
//-----------------------------------------------------------------------------

void ImGui::DockBuilderDockWindow(const char* window_name, ImGuiID node_id)
{
    // We don't preserve relative order of multiple docked windows (by clearing dock_order back to -1)
    ImGuiID window_id = ImHashStr(window_name);
    if (ImGuiWindow* window = FindWindowByID(window_id))
    {
        // Apply to created window
        SetWindowDock(window, node_id, Cond::Always);
        window.DockOrder = -1;
    }
    else
    {
        // Apply to settings
        ImGuiWindowSettings* settings = FindWindowSettings(window_id);
        if (settings == NULL)
            settings = CreateNewWindowSettings(window_name);
        settings->DockId = node_id;
        settings->DockOrder = -1;
    }
}

ImGuiDockNode* ImGui::DockBuilderGetNode(ImGuiID node_id)
{
    ImGuiContext* ctx = GImGui;
    return DockContextFindNodeByID(ctx, node_id);
}

void ImGui::DockBuilderSetNodePos(ImGuiID node_id, Vector2D pos)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    node.pos = pos;
    node->AuthorityForPos = ImGuiDataAuthority_DockNode;
}

void ImGui::DockBuilderSetNodeSize(ImGuiID node_id, Vector2D size)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    IM_ASSERT(size.x > 0.0 && size.y > 0.0);
    node->Size = node->SizeRef = size;
    node->AuthorityForSize = ImGuiDataAuthority_DockNode;
}

// Make sure to use the ImGuiDockNodeFlags_DockSpace flag to create a dockspace node! Otherwise this will create a floating node!
// - Floating node: you can then call DockBuilderSetNodePos()/DockBuilderSetNodeSize() to position and size the floating node.
// - Dockspace node: calling DockBuilderSetNodePos() is unnecessary.
// - If you intend to split a node immediately after creation using DockBuilderSplitNode(), make sure to call DockBuilderSetNodeSize() beforehand!
//   For various reason, the splitting code currently needs a base size otherwise space may not be allocated as precisely as you would expect.
// - Use (id == 0) to let the system allocate a node identifier.
// - Existing node with a same id will be removed.
ImGuiID ImGui::DockBuilderAddNode(ImGuiID id, ImGuiDockNodeFlags flags)
{
    ImGuiContext* ctx = GImGui;

    if (id != 0)
        DockBuilderRemoveNode(id);

    ImGuiDockNode* node = NULL;
    if (flags & ImGuiDockNodeFlags_DockSpace)
    {
        DockSpace(id, DimgVec2D::new(0, 0), (flags & ~ImGuiDockNodeFlags_DockSpace) | ImGuiDockNodeFlags_KeepAliveOnly);
        node = DockContextFindNodeByID(ctx, id);
    }
    else
    {
        node = DockContextAddNode(ctx, id);
        node->SetLocalFlags(flags);
    }
    node->LastFrameAlive = ctx->FrameCount;   // Set this otherwise BeginDocked will undock during the same frame.
    return node->ID;
}

void ImGui::DockBuilderRemoveNode(ImGuiID node_id)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    DockBuilderRemoveNodeDockedWindows(node_id, true);
    DockBuilderRemoveNodeChildNodes(node_id);
    // Node may have moved or deleted if e.g. any merge happened
    node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    if (node->IsCentralNode() && node->ParentNode)
        node->ParentNode->SetLocalFlags(node->ParentNode->LocalFlags | ImGuiDockNodeFlags_CentralNode);
    DockContextRemoveNode(ctx, node, true);
}

// root_id = 0 to remove all, root_id != 0 to remove child of given node.
void ImGui::DockBuilderRemoveNodeChildNodes(ImGuiID root_id)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockContext* dc  = &ctx->DockContext;

    ImGuiDockNode* root_node = root_id ? DockContextFindNodeByID(ctx, root_id) : NULL;
    if (root_id && root_node == NULL)
        return;
    bool has_central_node = false;

    ImGuiDataAuthority backup_root_node_authority_for_pos = root_node ? root_node->AuthorityForPos : ImGuiDataAuthority_Auto;
    ImGuiDataAuthority backup_root_node_authority_for_size = root_node ? root_node->AuthorityForSize : ImGuiDataAuthority_Auto;

    // Process active windows
    ImVector<ImGuiDockNode*> nodes_to_remove;
    for (int n = 0; n < dc->Nodes.Data.Size; n += 1)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
        {
            bool want_removal = (root_id == 0) || (node->ID != root_id && DockNodeGetRootNode(node)->ID == root_id);
            if (want_removal)
            {
                if (node->IsCentralNode())
                    has_central_node = true;
                if (root_id != 0)
                    DockContextQueueNotifyRemovedNode(ctx, node);
                if (root_node)
                {
                    DockNodeMoveWindows(root_node, node);
                    DockSettingsRenameNodeReferences(node->ID, root_node->ID);
                }
                nodes_to_remove.push_back(node);
            }
        }

    // DockNodeMoveWindows->DockNodeAddWindow will normally set those when reaching two windows (which is only adequate during interactive merge)
    // Make sure we don't lose our current pos/size. (FIXME-DOCK: Consider tidying up that code in DockNodeAddWindow instead)
    if (root_node)
    {
        root_node->AuthorityForPos = backup_root_node_authority_for_pos;
        root_node->AuthorityForSize = backup_root_node_authority_for_size;
    }

    // Apply to settings
    for (ImGuiWindowSettings* settings = ctx->SettingsWindows.begin(); settings != NULL; settings = ctx->SettingsWindows.next_chunk(settings))
        if (ImGuiID window_settings_dock_id = settings->DockId)
            for (int n = 0; n < nodes_to_remove.Size; n += 1)
                if (nodes_to_remove[n]->ID == window_settings_dock_id)
                {
                    settings->DockId = root_id;
                    break;
                }

    // Not really efficient, but easier to destroy a whole hierarchy considering DockContextRemoveNode is attempting to merge nodes
    if (nodes_to_remove.Size > 1)
        ImQsort(nodes_to_remove.Data, nodes_to_remove.Size, sizeof(ImGuiDockNode*), DockNodeComparerDepthMostFirst);
    for (int n = 0; n < nodes_to_remove.Size; n += 1)
        DockContextRemoveNode(ctx, nodes_to_remove[n], false);

    if (root_id == 0)
    {
        dc->Nodes.Clear();
        dc->Requests.clear();
    }
    else if (has_central_node)
    {
        root_node->CentralNode = root_node;
        root_node->SetLocalFlags(root_node->LocalFlags | ImGuiDockNodeFlags_CentralNode);
    }
}

void ImGui::DockBuilderRemoveNodeDockedWindows(ImGuiID root_id, bool clear_settings_refs)
{
    // clear references in settings
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;
    if (clear_settings_refs)
    {
        for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != NULL; settings = g.SettingsWindows.next_chunk(settings))
        {
            bool want_removal = (root_id == 0) || (settings->DockId == root_id);
            if (!want_removal && settings->DockId != 0)
                if (ImGuiDockNode* node = DockContextFindNodeByID(ctx, settings->DockId))
                    if (DockNodeGetRootNode(node)->ID == root_id)
                        want_removal = true;
            if (want_removal)
                settings->DockId = 0;
        }
    }

    // clear references in windows
    for (int n = 0; n < g.Windows.Size; n += 1)
    {
        ImGuiWindow* window = g.Windows[n];
        bool want_removal = (root_id == 0) || (window.DockNode && DockNodeGetRootNode(window.DockNode)->ID == root_id) || (window.DockNodeAsHost && window.DockNodeAsHost->ID == root_id);
        if (want_removal)
        {
            const ImGuiID backup_dock_id = window.DockId;
            IM_UNUSED(backup_dock_id);
            DockContextProcessUndockWindow(ctx, window, clear_settings_refs);
            if (!clear_settings_refs)
                IM_ASSERT(window.DockId == backup_dock_id);
        }
    }
}

// If 'out_id_at_dir' or 'out_id_at_opposite_dir' are non NULL, the function will write out the id of the two new nodes created.
// Return value is id of the node at the specified direction, so same as (*out_id_at_dir) if that pointer is set.
// FIXME-DOCK: We are not exposing nor using split_outer.
ImGuiID ImGui::DockBuilderSplitNode(ImGuiID id, ImGuiDir split_dir, float size_ratio_for_node_at_dir, ImGuiID* out_id_at_dir, ImGuiID* out_id_at_opposite_dir)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(split_dir != ImGuiDir_None);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockBuilderSplitNode: node 0x%08X, split_dir %d\n", id, split_dir);

    ImGuiDockNode* node = DockContextFindNodeByID(&g, id);
    if (node == NULL)
    {
        IM_ASSERT(node != NULL);
        return 0;
    }

    IM_ASSERT(!node->IsSplitNode()); // Assert if already split

    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Split;
    req.DockTargetWindow = NULL;
    req.DockTargetNode = node;
    req.DockPayload = NULL;
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = ImSaturate((split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? size_ratio_for_node_at_dir : 1.0 - size_ratio_for_node_at_dir);
    req.DockSplitOuter = false;
    DockContextProcessDock(&g, &req);

    ImGuiID id_at_dir = node->ChildNodes[(split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 0 : 1]->ID;
    ImGuiID id_at_opposite_dir = node->ChildNodes[(split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 1 : 0]->ID;
    if (out_id_at_dir)
        *out_id_at_dir = id_at_dir;
    if (out_id_at_opposite_dir)
        *out_id_at_opposite_dir = id_at_opposite_dir;
    return id_at_dir;
}

static ImGuiDockNode* DockBuilderCopyNodeRec(ImGuiDockNode* src_node, ImGuiID dst_node_id_if_known, ImVector<ImGuiID>* out_node_remap_pairs)
{
    ImGuiContext& g = *GImGui;
    ImGuiDockNode* dst_node = ImGui::DockContextAddNode(&g, dst_node_id_if_known);
    dst_node->SharedFlags = src_node->SharedFlags;
    dst_node->LocalFlags = src_node->LocalFlags;
    dst_node->LocalFlagsInWindows = ImGuiDockNodeFlags_None;
    dst_node.pos = src_node.pos;
    dst_node->Size = src_node->Size;
    dst_node->SizeRef = src_node->SizeRef;
    dst_node->SplitAxis = src_node->SplitAxis;
    dst_node->UpdateMergedFlags();

    out_node_remap_pairs->push_back(src_node->ID);
    out_node_remap_pairs->push_back(dst_node->ID);

    for (int child_n = 0; child_n < IM_ARRAYSIZE(src_node->ChildNodes); child_n += 1)
        if (src_node->ChildNodes[child_n])
        {
            dst_node->ChildNodes[child_n] = DockBuilderCopyNodeRec(src_node->ChildNodes[child_n], 0, out_node_remap_pairs);
            dst_node->ChildNodes[child_n]->ParentNode = dst_node;
        }

    IMGUI_DEBUG_LOG_DOCKING("[docking] Fork node %08X -> %08X (%d childs)\n", src_node->ID, dst_node->ID, dst_node->IsSplitNode() ? 2 : 0);
    return dst_node;
}

void ImGui::DockBuilderCopyNode(ImGuiID src_node_id, ImGuiID dst_node_id, ImVector<ImGuiID>* out_node_remap_pairs)
{
    ImGuiContext* ctx = GImGui;
    IM_ASSERT(src_node_id != 0);
    IM_ASSERT(dst_node_id != 0);
    IM_ASSERT(out_node_remap_pairs != NULL);

    DockBuilderRemoveNode(dst_node_id);

    ImGuiDockNode* src_node = DockContextFindNodeByID(ctx, src_node_id);
    IM_ASSERT(src_node != NULL);

    out_node_remap_pairs->clear();
    DockBuilderCopyNodeRec(src_node, dst_node_id, out_node_remap_pairs);

    IM_ASSERT((out_node_remap_pairs->Size % 2) == 0);
}

void ImGui::DockBuilderCopyWindowSettings(const char* src_name, const char* dst_name)
{
    ImGuiWindow* src_window = FindWindowByName(src_name);
    if (src_window == NULL)
        return;
    if (ImGuiWindow* dst_window = FindWindowByName(dst_name))
    {
        dst_window.Pos = src_window.Pos;
        dst_window.Size = src_window.Size;
        dst_window.SizeFull = src_window.SizeFull;
        dst_window.Collapsed = src_window.Collapsed;
    }
    else if (ImGuiWindowSettings* dst_settings = FindOrCreateWindowSettings(dst_name))
    {
        Vector2Dih window_pos_2ih = Vector2Dih(src_window.Pos);
        if (src_window.ViewportId != 0 && src_window.ViewportId != IMGUI_VIEWPORT_DEFAULT_ID)
        {
            dst_settings->ViewportPos = window_pos_2ih;
            dst_settings->ViewportId = src_window.ViewportId;
            dst_settings.pos = Vector2Dih(0, 0);
        }
        else
        {
            dst_settings.pos = window_pos_2ih;
        }
        dst_settings->Size = Vector2Dih(src_window.SizeFull);
        dst_settings->Collapsed = src_window.Collapsed;
    }
}

// FIXME: Will probably want to change this signature, in particular how the window remapping pairs are passed.
void ImGui::DockBuilderCopyDockSpace(ImGuiID src_dockspace_id, ImGuiID dst_dockspace_id, ImVector<const char*>* in_window_remap_pairs)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(src_dockspace_id != 0);
    IM_ASSERT(dst_dockspace_id != 0);
    IM_ASSERT(in_window_remap_pairs != NULL);
    IM_ASSERT((in_window_remap_pairs->Size % 2) == 0);

    // Duplicate entire dock
    // FIXME: When overwriting dst_dockspace_id, windows that aren't part of our dockspace window class but that are docked in a same node will be split apart,
    // whereas we could attempt to at least keep them together in a new, same floating node.
    ImVector<ImGuiID> node_remap_pairs;
    DockBuilderCopyNode(src_dockspace_id, dst_dockspace_id, &node_remap_pairs);

    // Attempt to transition all the upcoming windows associated to dst_dockspace_id into the newly created hierarchy of dock nodes
    // (The windows associated to src_dockspace_id are staying in place)
    ImVector<ImGuiID> src_windows;
    for (int remap_window_n = 0; remap_window_n < in_window_remap_pairs->Size; remap_window_n += 2)
    {
        const char* src_window_name = (*in_window_remap_pairs)[remap_window_n];
        const char* dst_window_name = (*in_window_remap_pairs)[remap_window_n + 1];
        ImGuiID src_window_id = ImHashStr(src_window_name);
        src_windows.push_back(src_window_id);

        // Search in the remapping tables
        ImGuiID src_dock_id = 0;
        if (ImGuiWindow* src_window = FindWindowByID(src_window_id))
            src_dock_id = src_window.DockId;
        else if (ImGuiWindowSettings* src_window_settings = FindWindowSettings(src_window_id))
            src_dock_id = src_window_settings->DockId;
        ImGuiID dst_dock_id = 0;
        for (int dock_remap_n = 0; dock_remap_n < node_remap_pairs.Size; dock_remap_n += 2)
            if (node_remap_pairs[dock_remap_n] == src_dock_id)
            {
                dst_dock_id = node_remap_pairs[dock_remap_n + 1];
                //node_remap_pairs[dock_remap_n] = node_remap_pairs[dock_remap_n + 1] = 0; // clear
                break;
            }

        if (dst_dock_id != 0)
        {
            // Docked windows gets redocked into the new node hierarchy.
            IMGUI_DEBUG_LOG_DOCKING("[docking] Remap live window '%s' 0x%08X -> '%s' 0x%08X\n", src_window_name, src_dock_id, dst_window_name, dst_dock_id);
            DockBuilderDockWindow(dst_window_name, dst_dock_id);
        }
        else
        {
            // Floating windows gets their settings transferred (regardless of whether the new window already exist or not)
            // When this is leading to a Copy and not a Move, we would get two overlapping floating windows. Could we possibly dock them together?
            IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window settings '%s' -> '%s'\n", src_window_name, dst_window_name);
            DockBuilderCopyWindowSettings(src_window_name, dst_window_name);
        }
    }

    // Anything else in the source nodes of 'node_remap_pairs' are windows that were docked in src_dockspace_id but are not owned by it (unaffiliated windows, e.g. "ImGui Demo")
    // Find those windows and move to them to the cloned dock node. This may be optional?
    for (int dock_remap_n = 0; dock_remap_n < node_remap_pairs.Size; dock_remap_n += 2)
        if (ImGuiID src_dock_id = node_remap_pairs[dock_remap_n])
        {
            ImGuiID dst_dock_id = node_remap_pairs[dock_remap_n + 1];
            ImGuiDockNode* node = DockBuilderGetNode(src_dock_id);
            for (int window_n = 0; window_n < node->Windows.Size; window_n += 1)
            {
                ImGuiWindow* window = node->Windows[window_n];
                if (src_windows.contains(window.ID))
                    continue;

                // Docked windows gets redocked into the new node hierarchy.
                IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window '%s' %08X -> %08X\n", window.Name, src_dock_id, dst_dock_id);
                DockBuilderDockWindow(window.Name, dst_dock_id);
            }
        }
}

// FIXME-DOCK: This is awkward because in series of split user is likely to loose access to its root node.
void ImGui::DockBuilderFinish(ImGuiID root_id)
{
    ImGuiContext* ctx = GImGui;
    //DockContextRebuild(ctx);
    DockContextBuildAddWindowsToNodes(ctx, root_id);
}

//-----------------------------------------------------------------------------
// Docking: Begin/End Support Functions (called from Begin/End)
//-----------------------------------------------------------------------------
// - GetWindowAlwaysWantOwnTabBar()
// - DockContextBindNodeToWindow()
// - BeginDocked()
// - BeginDockableDragDropSource()
// - BeginDockableDragDropTarget()
//-----------------------------------------------------------------------------

bool ImGui::GetWindowAlwaysWantOwnTabBar(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    if (g.io.ConfigDockingAlwaysTabBar || window.WindowClass.DockingAlwaysTabBar)
        if ((window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoDocking)) == 0)
            if (!window.IsFallbackWindow)    // We don't support AlwaysTabBar on the fallback/implicit window to avoid unused dock-node overhead/noise
                return true;
    return false;
}

static ImGuiDockNode* ImGui::DockContextBindNodeToWindow(ImGuiContext* ctx, ImGuiWindow* window)
{
    ImGuiContext& g = *ctx;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, window.DockId);
    IM_ASSERT(window.DockNode == NULL);

    // We should not be docking into a split node (SetWindowDock should avoid this)
    if (node && node->IsSplitNode())
    {
        DockContextProcessUndockWindow(ctx, window);
        return NULL;
    }

    // Create node
    if (node == NULL)
    {
        node = DockContextAddNode(ctx, window.DockId);
        node->AuthorityForPos = node->AuthorityForSize = node->AuthorityForViewport = ImGuiDataAuthority_Window;
        node->LastFrameAlive = g.FrameCount;
    }

    // If the node just turned visible and is part of a hierarchy, it doesn't have a size assigned by DockNodeTreeUpdatePosSize() yet,
    // so we're forcing a pos/size update from the first ancestor that is already visible (often it will be the root node).
    // If we don't do this, the window will be assigned a zero-size on its first frame, which won't ideally warm up the layout.
    // This is a little wonky because we don't normally update the pos/size of visible node mid-frame.
    if (!node->IsVisible)
    {
        ImGuiDockNode* ancestor_node = node;
        while (!ancestor_node->IsVisible && ancestor_node->ParentNode)
            ancestor_node = ancestor_node->ParentNode;
        IM_ASSERT(ancestor_node->Size.x > 0.0 && ancestor_node->Size.y > 0.0);
        DockNodeUpdateHasCentralNodeChild(DockNodeGetRootNode(ancestor_node));
        DockNodeTreeUpdatePosSize(ancestor_node, ancestor_node.pos, ancestor_node->Size, node);
    }

    // Add window to node
    bool node_was_visible = node->IsVisible;
    DockNodeAddWindow(node, window, true);
    node->IsVisible = node_was_visible; // Don't mark visible right away (so DockContextEndFrame() doesn't render it, maybe other side effects? will see)
    IM_ASSERT(node == window.DockNode);
    return node;
}

void ImGui::BeginDocked(ImGuiWindow* window, bool* p_open)
{
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;

    // clear fields ahead so most early-out paths don't have to do it
    window.DockIsActive = window.DockNodeIsVisible = window.DockTabIsVisible = false;

    const bool auto_dock_node = GetWindowAlwaysWantOwnTabBar(window);
    if (auto_dock_node)
    {
        if (window.DockId == 0)
        {
            IM_ASSERT(window.DockNode == NULL);
            window.DockId = DockContextGenNodeID(ctx);
        }
    }
    else
    {
        // Calling SetNextWindowPos() undock windows by default (by setting PosUndock)
        bool want_undock = false;
        want_undock |= (window.Flags & ImGuiWindowFlags_NoDocking) != 0;
        want_undock |= (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasPos) && (window.SetWindowPosAllowFlags & g.NextWindowData.PosCond) && g.NextWindowData.PosUndock;
        if (want_undock)
        {
            DockContextProcessUndockWindow(ctx, window);
            return;
        }
    }

    // Bind to our dock node
    ImGuiDockNode* node = window.DockNode;
    if (node != NULL)
        IM_ASSERT(window.DockId == node->ID);
    if (window.DockId != 0 && node == NULL)
    {
        node = DockContextBindNodeToWindow(ctx, window);
        if (node == NULL)
            return;
    }

#if 0
    // Undock if the ImGuiDockNodeFlags_NoDockingInCentralNode got set
    if (node->IsCentralNode && (node.flags & ImGuiDockNodeFlags_NoDockingInCentralNode))
    {
        DockContextProcessUndockWindow(ctx, window);
        return;
    }


    // Undock if our dockspace node disappeared
    // Note how we are testing for last_frame_alive and NOT last_frame_active. A DockSpace node can be maintained alive while being inactive with ImGuiDockNodeFlags_KeepAliveOnly.
    if (node->LastFrameAlive < g.FrameCount)
    {
        // If the window has been orphaned, transition the docknode to an implicit node processed in DockContextNewFrameUpdateDocking()
        ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        if (root_node->LastFrameAlive < g.FrameCount)
            DockContextProcessUndockWindow(ctx, window);
        else
            window.DockIsActive = true;
        return;
    }

    // Store style overrides
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        window.DockStyle.Colors[color_n] = ColorConvertFloat4ToU32(g.Style.Colors[GWindowDockStyleColors[color_n]]);

    // Fast path return. It is common for windows to hold on a persistent dock_id but be the only visible window,
    // and never create neither a host window neither a tab bar.
    // FIXME-DOCK: replace ->host_window NULL compare with something more explicit (~was initially intended as a first frame test)
    if (node->HostWindow == NULL)
    {
        if (node->State == ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing)
            window.DockIsActive = true;
        if (node->Windows.Size > 1)
            DockNodeHideWindowDuringHostWindowCreation(window);
        return;
    }

    // We can have zero-sized nodes (e.g. children of a small-size dockspace)
    IM_ASSERT(node->HostWindow);
    IM_ASSERT(node->IsLeafNode());
    IM_ASSERT(node->Size.x >= 0.0 && node->Size.y >= 0.0);
    node->State = ImGuiDockNodeState_HostWindowVisible;

    // Undock if we are submitted earlier than the host window
    if (!(node->MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly) && window.BeginOrderWithinContext < node->HostWindow->BeginOrderWithinContext)
    {
        DockContextProcessUndockWindow(ctx, window);
        return;
    }

    // Position/size window
    SetNextWindowPos(node.pos);
    SetNextWindowSize(node->Size);
    g.NextWindowData.PosUndock = false; // Cancel implicit undocking of SetNextWindowPos()
    window.DockIsActive = true;
    window.DockNodeIsVisible = true;
    window.DockTabIsVisible = false;
    if (node->MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly)
        return;

    // When the window is selected we mark it as visible.
    if (node->VisibleWindow == window)
        window.DockTabIsVisible = true;

    // Update window flag
    IM_ASSERT((window.Flags & ImGuiWindowFlags_ChildWindow) == 0);
    window.Flags |= ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_AlwaysUseWindowPadding | ImGuiWindowFlags_NoResize;
    if (node->IsHiddenTabBar() || node->IsNoTabBar())
        window.Flags |= ImGuiWindowFlags_NoTitleBar;
    else
        window.Flags &= ~ImGuiWindowFlags_NoTitleBar;      // clear the NoTitleBar flag in case the user set it: confusingly enough we need a title bar height so we are correctly offset, but it won't be displayed!

    // Save new dock order only if the window has been visible once already
    // This allows multiple windows to be created in the same frame and have their respective dock orders preserved.
    if (node->TabBar && window.WasActive)
        window.DockOrder = (short)DockNodeGetTabOrder(window);

    if ((node->WantCloseAll || node->WantCloseTabId == window.TabId) && p_open != NULL)
        *p_open = false;

    // Update child_id to allow returning from Child to Parent with Escape
    ImGuiWindow* parent_window = window.DockNode->HostWindow;
    window.ChildId = parent_window.GetID(window.Name);
}

void ImGui::BeginDockableDragDropSource(ImGuiWindow* window)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.active_id == window.MoveId);
    IM_ASSERT(g.moving_window == window);
    IM_ASSERT(g.CurrentWindow == window);

    g.last_item_data.ID = window.MoveId;
    window = window.RootWindowDockTree;
    IM_ASSERT((window.Flags & ImGuiWindowFlags_NoDocking) == 0);
    bool is_drag_docking = (g.io.ConfigDockingWithShift) || ImRect(0, 0, window.SizeFull.x, GetFrameHeight()).Contains(g.ActiveIdClickOffset); // FIXME-DOCKING: Need to make this stateful and explicit
    if (is_drag_docking && BeginDragDropSource(ImGuiDragDropFlags_SourceNoPreviewTooltip | ImGuiDragDropFlags_SourceNoHoldToOpenOthers | ImGuiDragDropFlags_SourceAutoExpirePayload))
    {
        SetDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, &window, sizeof(window));
        EndDragDropSource();

        // Store style overrides
        for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
            window.DockStyle.Colors[color_n] = ColorConvertFloat4ToU32(g.Style.Colors[GWindowDockStyleColors[color_n]]);
    }
}

void ImGui::BeginDockableDragDropTarget(ImGuiWindow* window)
{
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;

    //IM_ASSERT(window->root_window_dock_tree == window); // May also be a DockSpace
    IM_ASSERT((window.Flags & ImGuiWindowFlags_NoDocking) == 0);
    if (!g.DragDropActive)
        return;
    //GetForegroundDrawList(window)->add_rect(window->pos, window->pos + window->size, IM_COL32(255, 255, 0, 255));
    if (!BeginDragDropTargetCustom(window.Rect(), window.ID))
        return;

    // Peek into the payload before calling AcceptDragDropPayload() so we can handle overlapping dock nodes with filtering
    // (this is a little unusual pattern, normally most code would call AcceptDragDropPayload directly)
    const ImGuiPayload* payload = &g.DragDropPayload;
    if (!payload->IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) || !DockNodeIsDropAllowed(window, *(ImGuiWindow**)payload->Data))
    {
        EndDragDropTarget();
        return;
    }

    ImGuiWindow* payload_window = *(ImGuiWindow**)payload->Data;
    if (AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, ImGuiDragDropFlags_AcceptBeforeDelivery | ImGuiDragDropFlags_AcceptNoDrawDefaultRect))
    {
        // Select target node
        // (Important: we cannot use g.hovered_dock_node here! Because each of our target node have filters based on payload, each candidate drop target will do its own evaluation)
        bool dock_into_floating_window = false;
        ImGuiDockNode* node = NULL;
        if (window.DockNodeAsHost)
        {
            // Cannot assume that node will != NULL even though we passed the rectangle test: it depends on padding/spacing handled by DockNodeTreeFindVisibleNodeByPos().
            node = DockNodeTreeFindVisibleNodeByPos(window.DockNodeAsHost, g.io.MousePos);

            // There is an edge case when docking into a dockspace which only has _inactive_ nodes (because none of the windows are active)
            // In this case we need to fallback into any leaf mode, possibly the central node.
            // FIXME-20181220: We should not have to test for is_leaf_node() here but we have another bug to fix first.
            if (node && node->IsDockSpace() && node->IsRootNode())
                node = (node->CentralNode && node->IsLeafNode()) ? node->CentralNode : DockNodeTreeFindFallbackLeafNode(node);
        }
        else
        {
            if (window.DockNode)
                node = window.DockNode;
            else
                dock_into_floating_window = true; // Dock into a regular window
        }

        const ImRect explicit_target_rect = (node && node->TabBar && !node->IsHiddenTabBar() && !node->IsNoTabBar()) ? node->TabBar->BarRect : ImRect(window.Pos, window.Pos + DimgVec2D::new(window.Size.x, GetFrameHeight()));
        const bool is_explicit_target = g.io.ConfigDockingWithShift || IsMouseHoveringRect(explicit_target_rect.Min, explicit_target_rect.Max);

        // preview docking request and find out split direction/ratio
        //const bool do_preview = true;     // Ignore testing for payload->is_preview() which removes one frame of delay, but breaks overlapping drop targets within the same window.
        const bool do_preview = payload->IsPreview() || payload->IsDelivery();
        if (do_preview && (node != NULL || dock_into_floating_window))
        {
            ImGuiDockPreviewData split_inner;
            ImGuiDockPreviewData split_outer;
            ImGuiDockPreviewData* split_data = &split_inner;
            if (node && (node->ParentNode || node->IsCentralNode()))
                if (ImGuiDockNode* root_node = DockNodeGetRootNode(node))
                {
                    DockNodePreviewDockSetup(window, root_node, payload_window, &split_outer, is_explicit_target, true);
                    if (split_outer.IsSplitDirExplicit)
                        split_data = &split_outer;
                }
            DockNodePreviewDockSetup(window, node, payload_window, &split_inner, is_explicit_target, false);
            if (split_data == &split_outer)
                split_inner.IsDropAllowed = false;

            // Draw inner then outer, so that previewed tab (in inner data) will be behind the outer drop boxes
            DockNodePreviewDockRender(window, node, payload_window, &split_inner);
            DockNodePreviewDockRender(window, node, payload_window, &split_outer);

            // Queue docking request
            if (split_data->IsDropAllowed && payload->IsDelivery())
                DockContextQueueDock(ctx, window, split_data->SplitNode, payload_window, split_data->SplitDir, split_data->SplitRatio, split_data == &split_outer);
        }
    }
    EndDragDropTarget();
}

//-----------------------------------------------------------------------------
// Docking: Settings
//-----------------------------------------------------------------------------
// - DockSettingsRenameNodeReferences()
// - DockSettingsRemoveNodeReferences()
// - DockSettingsFindNodeSettings()
// - DockSettingsHandler_ApplyAll()
// - DockSettingsHandler_ReadOpen()
// - DockSettingsHandler_ReadLine()
// - DockSettingsHandler_DockNodeToSettings()
// - DockSettingsHandler_WriteAll()
//-----------------------------------------------------------------------------

static void ImGui::DockSettingsRenameNodeReferences(ImGuiID old_node_id, ImGuiID new_node_id)
{
    ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockSettingsRenameNodeReferences: from 0x%08X -> to 0x%08X\n", old_node_id, new_node_id);
    for (int window_n = 0; window_n < g.Windows.Size; window_n += 1)
    {
        ImGuiWindow* window = g.Windows[window_n];
        if (window.DockId == old_node_id && window.DockNode == NULL)
            window.DockId = new_node_id;
    }
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != NULL; settings = g.SettingsWindows.next_chunk(settings))
        if (settings->DockId == old_node_id)
            settings->DockId = new_node_id;
}

// Remove references stored in ImGuiWindowSettings to the given ImGuiDockNodeSettings
static void ImGui::DockSettingsRemoveNodeReferences(ImGuiID* node_ids, int node_ids_count)
{
    ImGuiContext& g = *GImGui;
    int found = 0;
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != NULL; settings = g.SettingsWindows.next_chunk(settings))
        for (int node_n = 0; node_n < node_ids_count; node_n += 1)
            if (settings->DockId == node_ids[node_n])
            {
                settings->DockId = 0;
                settings->DockOrder = -1;
                if (found += 1 < node_ids_count)
                    break;
                return;
            }
}

static ImGuiDockNodeSettings* ImGui::DockSettingsFindNodeSettings(ImGuiContext* ctx, ImGuiID id)
{
    // FIXME-OPT
    ImGuiDockContext* dc  = &ctx->DockContext;
    for (int n = 0; n < dc->NodesSettings.Size; n += 1)
        if (dc->NodesSettings[n].ID == id)
            return &dc->NodesSettings[n];
    return NULL;
}

// clear settings data
static void ImGui::DockSettingsHandler_ClearAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiDockContext* dc  = &ctx->DockContext;
    dc->NodesSettings.clear();
    DockContextClearNodes(ctx, 0, true);
}

// Recreate nodes based on settings data
static void ImGui::DockSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    // Prune settings at boot time only
    ImGuiDockContext* dc  = &ctx->DockContext;
    if (ctx->Windows.Size == 0)
        DockContextPruneUnusedSettingsNodes(ctx);
    DockContextBuildNodesFromSettings(ctx, dc->NodesSettings.Data, dc->NodesSettings.Size);
    DockContextBuildAddWindowsToNodes(ctx, 0);
}

static void* ImGui::DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, const char* name)
{
    if (strcmp(name, "data") != 0)
        return NULL;
    return (void*)1;
}

static void ImGui::DockSettingsHandler_ReadLine(ImGuiContext* ctx, ImGuiSettingsHandler*, void*, const char* line)
{
    char c = 0;
    int x = 0, y = 0;
    int r = 0;

    // Parsing, e.g.
    // " dock_node   id=0x00000001 pos=383,193 size=201,322 split=Y,0.506 "
    // "   dock_node id=0x00000002 Parent=0x00000001 "
    // Important: this code expect currently fields in a fixed order.
    ImGuiDockNodeSettings node;
    line = ImStrSkipBlank(line);
    if      (strncmp(line, "dock_node", 8) == 0)  { line = ImStrSkipBlank(line + strlen("dock_node")); }
    else if (strncmp(line, "DockSpace", 9) == 0) { line = ImStrSkipBlank(line + strlen("DockSpace")); node.Flags |= ImGuiDockNodeFlags_DockSpace; }
    else return;
    if (sscanf(line, "id=0x%08X%n",      &node.ID, &r) == 1)            { line += r; } else return;
    if (sscanf(line, " Parent=0x%08X%n", &node.ParentNodeId, &r) == 1)  { line += r; if (node.ParentNodeId == 0) return; }
    if (sscanf(line, " Window=0x%08X%n", &node.ParentWindowId, &r) ==1) { line += r; if (node.ParentWindowId == 0) return; }
    if (node.ParentNodeId == 0)
    {
        if (sscanf(line, " pos=%i,%i%n",  &x, &y, &r) == 2)         { line += r; node.Pos = Vector2Dih((short)x, (short)y); } else return;
        if (sscanf(line, " size=%i,%i%n", &x, &y, &r) == 2)         { line += r; node.Size = Vector2Dih((short)x, (short)y); } else return;
    }
    else
    {
        if (sscanf(line, " size_ref=%i,%i%n", &x, &y, &r) == 2)      { line += r; node.SizeRef = Vector2Dih((short)x, (short)y); }
    }
    if (sscanf(line, " split=%c%n", &c, &r) == 1)                   { line += r; if (c == 'X') node.SplitAxis = ImGuiAxis_X; else if (c == 'Y') node.SplitAxis = ImGuiAxis_Y; }
    if (sscanf(line, " NoResize=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoResize; }
    if (sscanf(line, " central_node=%d%n", &x, &r) == 1)             { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_CentralNode; }
    if (sscanf(line, " NoTabBar=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoTabBar; }
    if (sscanf(line, " HiddenTabBar=%d%n", &x, &r) == 1)            { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_HiddenTabBar; }
    if (sscanf(line, " NoWindowMenuButton=%d%n", &x, &r) == 1)      { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoWindowMenuButton; }
    if (sscanf(line, " NoCloseButton=%d%n", &x, &r) == 1)           { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoCloseButton; }
    if (sscanf(line, " Selected=0x%08X%n", &node.SelectedTabId,&r) == 1) { line += r; }
    if (node.ParentNodeId != 0)
        if (ImGuiDockNodeSettings* parent_settings = DockSettingsFindNodeSettings(ctx, node.ParentNodeId))
            node.Depth = parent_settings->Depth + 1;
    ctx->DockContext.NodesSettings.push_back(node);
}

static void DockSettingsHandler_DockNodeToSettings(ImGuiDockContext* dc, ImGuiDockNode* node, int depth)
{
    ImGuiDockNodeSettings node_settings;
    IM_ASSERT(depth < (1 << (sizeof(node_settings.Depth) << 3)));
    node_settings.ID = node->ID;
    node_settings.ParentNodeId = node->ParentNode ? node->ParentNode->ID : 0;
    node_settings.ParentWindowId = (node->IsDockSpace() && node->HostWindow && node->HostWindow->ParentWindow) ? node->HostWindow->ParentWindow->ID : 0;
    node_settings.SelectedTabId = node->SelectedTabId;
    node_settings.SplitAxis = (signed char)(node->IsSplitNode() ? node->SplitAxis : ImGuiAxis_None);
    node_settings.Depth = (char)depth;
    node_settings.Flags = (node->LocalFlags & ImGuiDockNodeFlags_SavedFlagsMask_);
    node_settings.Pos = Vector2Dih(node.pos);
    node_settings.Size = Vector2Dih(node->Size);
    node_settings.SizeRef = Vector2Dih(node->SizeRef);
    dc->NodesSettings.push_back(node_settings);
    if (node->ChildNodes[0])
        DockSettingsHandler_DockNodeToSettings(dc, node->ChildNodes[0], depth + 1);
    if (node->ChildNodes[1])
        DockSettingsHandler_DockNodeToSettings(dc, node->ChildNodes[1], depth + 1);
}

static void ImGui::DockSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* buf)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc = &ctx->DockContext;
    if (!(g.io.ConfigFlags & ImGuiConfigFlags_DockingEnable))
        return;

    // Gather settings data
    // (unlike our windows settings, because nodes are always built we can do a full rewrite of the SettingsNode buffer)
    dc->NodesSettings.resize(0);
    dc->NodesSettings.reserve(dc->Nodes.Data.Size);
    for (int n = 0; n < dc->Nodes.Data.Size; n += 1)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
            if (node->IsRootNode())
                DockSettingsHandler_DockNodeToSettings(dc, node, 0);

    int max_depth = 0;
    for (int node_n = 0; node_n < dc->NodesSettings.Size; node_n += 1)
        max_depth = ImMax(dc->NodesSettings[node_n].Depth, max_depth);

    // Write to text buffer
    buf->appendf("[%s][data]\n", handler->TypeName);
    for (int node_n = 0; node_n < dc->NodesSettings.Size; node_n += 1)
    {
        const int line_start_pos = buf->size(); (void)line_start_pos;
        const ImGuiDockNodeSettings* node_settings = &dc->NodesSettings[node_n];
        buf->appendf("%*s%s%*s", node_settings->Depth * 2, "", (node_settings.flags & ImGuiDockNodeFlags_DockSpace) ? "DockSpace" : "dock_node ", (max_depth - node_settings->Depth) * 2, "");  // Text align nodes to facilitate looking at .ini file
        buf->appendf(" id=0x%08X", node_settings->ID);
        if (node_settings->ParentNodeId)
        {
            buf->appendf(" Parent=0x%08X size_ref=%d,%d", node_settings->ParentNodeId, node_settings->SizeRef.x, node_settings->SizeRef.y);
        }
        else
        {
            if (node_settings->ParentWindowId)
                buf->appendf(" Window=0x%08X", node_settings->ParentWindowId);
            buf->appendf(" pos=%d,%d size=%d,%d", node_settings.pos.x, node_settings.pos.y, node_settings->Size.x, node_settings->Size.y);
        }
        if (node_settings->SplitAxis != ImGuiAxis_None)
            buf->appendf(" split=%c", (node_settings->SplitAxis == ImGuiAxis_X) ? 'X' : 'Y');
        if (node_settings.flags & ImGuiDockNodeFlags_NoResize)
            buf->appendf(" NoResize=1");
        if (node_settings.flags & ImGuiDockNodeFlags_CentralNode)
            buf->appendf(" central_node=1");
        if (node_settings.flags & ImGuiDockNodeFlags_NoTabBar)
            buf->appendf(" NoTabBar=1");
        if (node_settings.flags & ImGuiDockNodeFlags_HiddenTabBar)
            buf->appendf(" HiddenTabBar=1");
        if (node_settings.flags & ImGuiDockNodeFlags_NoWindowMenuButton)
            buf->appendf(" NoWindowMenuButton=1");
        if (node_settings.flags & ImGuiDockNodeFlags_NoCloseButton)
            buf->appendf(" NoCloseButton=1");
        if (node_settings->SelectedTabId)
            buf->appendf(" Selected=0x%08X", node_settings->SelectedTabId);

#if IMGUI_DEBUG_INI_SETTINGS
        // [DEBUG] Include comments in the .ini file to ease debugging
        if (ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_settings->ID))
        {
            buf->appendf("%*s", ImMax(2, (line_start_pos + 92) - buf->size()), "");     // Align everything
            if (node->IsDockSpace() && node->HostWindow && node->HostWindow->ParentWindow)
                buf->appendf(" ; in '%s'", node->HostWindow->ParentWindow->Name);
            // Iterate settings so we can give info about windows that didn't exist during the session.
            int contains_window = 0;
            for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != NULL; settings = g.SettingsWindows.next_chunk(settings))
                if (settings->DockId == node_settings->ID)
                {
                    if (contains_window += 1 == 0)
                        buf->appendf(" ; contains ");
                    buf->appendf("'%s' ", settings->GetName());
                }
        }

        buf->appendf("\n");
    }
    buf->appendf("\n");
}


// Win32 API IME support (for Asian languages, etc.)
#if defined(_WIN32) && !defined(IMGUI_DISABLE_WIN32_FUNCTIONS) && !defined(IMGUI_DISABLE_WIN32_DEFAULT_IME_FUNCTIONS)

#include <imm.h>
#ifdef _MSC_VER
#pragma comment(lib, "imm32")


static void SetPlatformImeDataFn_DefaultImpl(ImGuiViewport* viewport, ImGuiPlatformImeData* data)
{
    // Notify OS Input Method Editor of text input position
    HWND hwnd = (HWND)viewport->PlatformHandleRaw;
#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    if (hwnd == 0)
        hwnd = (HWND)ImGui::GetIO().ImeWindowHandle;

    if (hwnd == 0)
        return;

    ::ImmAssociateContextEx(hwnd, NULL, data->WantVisible ? IACE_DEFAULT : 0);

    if (HIMC himc = ::ImmGetContext(hwnd))
    {
        COMPOSITIONFORM composition_form = {};
        composition_form.ptCurrentPos.x = (LONG)(data->InputPos.x - viewport.pos.x);
        composition_form.ptCurrentPos.y = (LONG)(data->InputPos.y - viewport.pos.y);
        composition_form.dwStyle = CFS_FORCE_POSITION;
        ::ImmSetCompositionWindow(himc, &composition_form);
        CANDIDATEFORM candidate_form = {};
        candidate_form.dwStyle = CFS_CANDIDATEPOS;
        candidate_form.ptCurrentPos.x = (LONG)(data->InputPos.x - viewport.pos.x);
        candidate_form.ptCurrentPos.y = (LONG)(data->InputPos.y - viewport.pos.y);
        ::ImmSetCandidateWindow(himc, &candidate_form);
        ::ImmReleaseContext(hwnd, himc);
    }
}

#else

static void SetPlatformImeDataFn_DefaultImpl(ImGuiViewport*, ImGuiPlatformImeData*) {}



//-----------------------------------------------------------------------------
// [SECTION] METRICS/DEBUGGER WINDOW
//-----------------------------------------------------------------------------
// - RenderViewportThumbnail() [Internal]
// - RenderViewportsThumbnails() [Internal]
// - DebugTextEncoding()
// - MetricsHelpMarker() [Internal]
// - ShowFontAtlas() [Internal]
// - ShowMetricsWindow()
// - DebugNodeColumns() [Internal]
// - DebugNodeDockNode() [Internal]
// - DebugNodeDrawList() [Internal]
// - DebugNodeDrawCmdShowMeshAndBoundingBox() [Internal]
// - DebugNodeFont() [Internal]
// - DebugNodeFontGlyph() [Internal]
// - DebugNodeStorage() [Internal]
// - DebugNodeTabBar() [Internal]
// - DebugNodeViewport() [Internal]
// - DebugNodeWindow() [Internal]
// - DebugNodeWindowSettings() [Internal]
// - DebugNodeWindowsList() [Internal]
// - DebugNodeWindowsListByBeginStackParent() [Internal]
//-----------------------------------------------------------------------------

#ifndef IMGUI_DISABLE_DEBUG_TOOLS

void ImGui::DebugRenderViewportThumbnail(ImDrawList* draw_list, ImGuiViewportP* viewport, const ImRect& bb)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;

    Vector2D scale = bb.GetSize() / viewport->Size;
    Vector2D off = bb.Min - viewport.pos * scale;
    float alpha_mul = (viewport.flags & ImGuiViewportFlags_Minimized) ? 0.30 : 1.00;
    window.DrawList->AddRectFilled(bb.Min, bb.Max, ImGui::GetColorU32(ImGuiCol_Border, alpha_mul * 0.40));
    for (int i = 0; i != g.Windows.Size; i += 1)
    {
        ImGuiWindow* thumb_window = g.Windows[i];
        if (!thumb_window.WasActive || (thumb_window.Flags & ImGuiWindowFlags_ChildWindow))
            continue;
        if (thumb_window.viewport != viewport)
            continue;

        ImRect thumb_r = thumb_window.Rect();
        ImRect title_r = thumb_window.TitleBarRect();
        thumb_r = ImRect(ImFloor(off + thumb_r.Min * scale), ImFloor(off +  thumb_r.Max * scale));
        title_r = ImRect(ImFloor(off + title_r.Min * scale), ImFloor(off +  DimgVec2D::new(title_r.Max.x, title_r.Min.y) * scale) + DimgVec2D::new(0,5)); // Exaggerate title bar height
        thumb_r.ClipWithFull(bb);
        title_r.ClipWithFull(bb);
        const bool window_is_focused = (g.nav_window && thumb_window.RootWindowForTitleBarHighlight == g.nav_window->RootWindowForTitleBarHighlight);
        window.DrawList->AddRectFilled(thumb_r.Min, thumb_r.Max, GetColorU32(ImGuiCol_WindowBg, alpha_mul));
        window.DrawList->AddRectFilled(title_r.Min, title_r.Max, GetColorU32(window_is_focused ? ImGuiCol_TitleBgActive : ImGuiCol_TitleBg, alpha_mul));
        window.DrawList->AddRect(thumb_r.Min, thumb_r.Max, GetColorU32(ImGuiCol_Border, alpha_mul));
        window.DrawList->AddText(g.Font, g.FontSize * 1.0, title_r.Min, GetColorU32(ImGuiCol_Text, alpha_mul), thumb_window.Name, FindRenderedTextEnd(thumb_window.Name));
    }
    draw_list->AddRect(bb.Min, bb.Max, GetColorU32(ImGuiCol_Border, alpha_mul));
}

static void RenderViewportsThumbnails()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;

    // We don't display full monitor bounds (we could, but it often looks awkward), instead we display just enough to cover all of our viewports.
    float SCALE = 1.0 / 8.0;
    ImRect bb_full(FLT_MAX, FLT_MAX, -FLT_MAX, -FLT_MAX);
    for (int n = 0; n < g.Viewports.Size; n += 1)
        bb_full.Add(g.Viewports[n]->GetMainRect());
    Vector2D p = window.DC.CursorPos;
    Vector2D off = p - bb_full.Min * SCALE;
    for (int n = 0; n < g.Viewports.Size; n += 1)
    {
        ImGuiViewportP* viewport = g.Viewports[n];
        ImRect viewport_draw_bb(off + (viewport.pos) * SCALE, off + (viewport.pos + viewport->Size) * SCALE);
        ImGui::DebugRenderViewportThumbnail(window.DrawList, viewport, viewport_draw_bb);
    }
    ImGui::Dummy(bb_full.GetSize() * SCALE);
}

static int IMGUI_CDECL ViewportComparerByFrontMostStampCount(const void* lhs, const void* rhs)
{
    const ImGuiViewportP* a = *(const ImGuiViewportP* const*)lhs;
    const ImGuiViewportP* b = *(const ImGuiViewportP* const*)rhs;
    return b->LastFrontMostStampCount - a->LastFrontMostStampCount;
}

// Helper tool to diagnose between text encoding issues and font loading issues. Pass your UTF-8 string and verify that there are correct.
void ImGui::DebugTextEncoding(const char* str)
{
    Text("Text: \"%s\"", str);
    if (!BeginTable("list", 4, ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg | ImGuiTableFlags_SizingFixedFit))
        return;
    TableSetupColumn("Offset");
    TableSetupColumn("UTF-8");
    TableSetupColumn("Glyph");
    TableSetupColumn("codepoint");
    TableHeadersRow();
    for (const char* p = str; *p != 0; )
    {
        unsigned int c;
        const int c_utf8_len = ImTextCharFromUtf8(&c, p, NULL);
        TableNextColumn();
        Text("%d", (p - str));
        TableNextColumn();
        for (int byte_index = 0; byte_index < c_utf8_len; byte_index += 1)
        {
            if (byte_index > 0)
                SameLine();
            Text("0x%02X", (unsigned char)p[byte_index]);
        }
        TableNextColumn();
        if (GetFont()->FindGlyphNoFallback((ImWchar)c))
            TextUnformatted(p, p + c_utf8_len);
        else
            TextUnformatted((c == IM_UNICODE_CODEPOINT_INVALID) ? "[invalid]" : "[missing]");
        TableNextColumn();
        Text("U+%04X", c);
        p += c_utf8_len;
    }
    EndTable();
}

// Avoid naming collision with imgui_demo.cpp's HelpMarker() for unity builds.
static void MetricsHelpMarker(const char* desc)
{
    ImGui::TextDisabled("(?)");
    if (ImGui::IsItemHovered())
    {
        ImGui::BeginTooltip();
        ImGui::PushTextWrapPos(ImGui::GetFontSize() * 35.0);
        ImGui::TextUnformatted(desc);
        ImGui::PopTextWrapPos();
        ImGui::EndTooltip();
    }
}

// [DEBUG] List fonts in a font atlas and display its texture
void ImGui::ShowFontAtlas(ImFontAtlas* atlas)
{
    for (int i = 0; i < atlas->Fonts.Size; i += 1)
    {
        ImFont* font = atlas->Fonts[i];
        PushID(font);
        DebugNodeFont(font);
        PopID();
    }
    if (TreeNode("Atlas texture", "Atlas texture (%dx%d pixels)", atlas->TexWidth, atlas->TexHeight))
    {
        Vector4D tint_col = Vector4D(1.0, 1.0, 1.0, 1.0);
        Vector4D border_col = Vector4D(1.0, 1.0, 1.0, 0.5);
        Image(atlas->TexID, DimgVec2D::new((float)atlas->TexWidth, (float)atlas->TexHeight), DimgVec2D::new(0.0, 0.0), DimgVec2D::new(1.0, 1.0), tint_col, border_col);
        TreePop();
    }
}

void ImGui::ShowMetricsWindow(bool* p_open)
{
    ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;
    ImGuiMetricsConfig* cfg = &g.DebugMetricsConfig;
    if (cfg->ShowDebugLog)
        ShowDebugLogWindow(&cfg->ShowDebugLog);
    if (cfg->ShowStackTool)
        ShowStackToolWindow(&cfg->ShowStackTool);

    if (!Begin("Dear ImGui Metrics/Debugger", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        End();
        return;
    }

    // Basic info
    Text("Dear ImGui %s", GetVersion());
    Text("Application average %.3 ms/frame (%.1 FPS)", 1000.0 / io.Framerate, io.Framerate);
    Text("%d vertices, %d indices (%d triangles)", io.MetricsRenderVertices, io.MetricsRenderIndices, io.MetricsRenderIndices / 3);
    Text("%d visible windows, %d active allocations", io.MetricsRenderWindows, io.MetricsActiveAllocations);
    //SameLine(); if (SmallButton("GC")) { g.gc_compact_all = true; }

    Separator();

    // Debugging enums
    enum { WRT_OuterRect, WRT_OuterRectClipped, WRT_InnerRect, WRT_InnerClipRect, WRT_WorkRect, WRT_Content, WRT_ContentIdeal, WRT_ContentRegionRect, WRT_Count }; // windows rect Type
    const char* wrt_rects_names[WRT_Count] = { "OuterRect", "outer_rect_clipped", "inner_rect", "inner_clip_rect", "work_rect", "Content", "ContentIdeal", "content_region_rect" };
    enum { TRT_OuterRect, TRT_InnerRect, TRT_WorkRect, TRT_HostClipRect, TRT_InnerClipRect, TRT_BackgroundClipRect, TRT_ColumnsRect, TRT_ColumnsWorkRect, TRT_ColumnsClipRect, TRT_ColumnsContentHeadersUsed, TRT_ColumnsContentHeadersIdeal, TRT_ColumnsContentFrozen, TRT_ColumnsContentUnfrozen, TRT_Count }; // tables rect Type
    const char* trt_rects_names[TRT_Count] = { "OuterRect", "inner_rect", "work_rect", "HostClipRect", "inner_clip_rect", "BackgroundClipRect", "ColumnsRect", "ColumnsWorkRect", "ColumnsClipRect", "ColumnsContentHeadersUsed", "ColumnsContentHeadersIdeal", "ColumnsContentFrozen", "ColumnsContentUnfrozen" };
    if (cfg->ShowWindowsRectsType < 0)
        cfg->ShowWindowsRectsType = WRT_WorkRect;
    if (cfg->ShowTablesRectsType < 0)
        cfg->ShowTablesRectsType = TRT_WorkRect;

    struct Funcs
    {
        static ImRect GetTableRect(ImGuiTable* table, int rect_type, int n)
        {
            ImGuiTableInstanceData* table_instance = TableGetInstanceData(table, table->InstanceCurrent); // Always using last submitted instance
            if (rect_type == TRT_OuterRect)                     { return table->OuterRect; }
            else if (rect_type == TRT_InnerRect)                { return table->InnerRect; }
            else if (rect_type == TRT_WorkRect)                 { return table->WorkRect; }
            else if (rect_type == TRT_HostClipRect)             { return table->HostClipRect; }
            else if (rect_type == TRT_InnerClipRect)            { return table->InnerClipRect; }
            else if (rect_type == TRT_BackgroundClipRect)       { return table->BgClipRect; }
            else if (rect_type == TRT_ColumnsRect)              { ImGuiTableColumn* c = &table->Columns[n]; return ImRect(c->MinX, table->InnerClipRect.Min.y, c->MaxX, table->InnerClipRect.Min.y + table_instance->LastOuterHeight); }
            else if (rect_type == TRT_ColumnsWorkRect)          { ImGuiTableColumn* c = &table->Columns[n]; return ImRect(c->WorkMinX, table->WorkRect.Min.y, c->WorkMaxX, table->WorkRect.Max.y); }
            else if (rect_type == TRT_ColumnsClipRect)          { ImGuiTableColumn* c = &table->Columns[n]; return c->ClipRect; }
            else if (rect_type == TRT_ColumnsContentHeadersUsed){ ImGuiTableColumn* c = &table->Columns[n]; return ImRect(c->WorkMinX, table->InnerClipRect.Min.y, c->ContentMaxXHeadersUsed, table->InnerClipRect.Min.y + table_instance->LastFirstRowHeight); } // Note: y1/y2 not always accurate
            else if (rect_type == TRT_ColumnsContentHeadersIdeal){ImGuiTableColumn* c = &table->Columns[n]; return ImRect(c->WorkMinX, table->InnerClipRect.Min.y, c->ContentMaxXHeadersIdeal, table->InnerClipRect.Min.y + table_instance->LastFirstRowHeight); }
            else if (rect_type == TRT_ColumnsContentFrozen)     { ImGuiTableColumn* c = &table->Columns[n]; return ImRect(c->WorkMinX, table->InnerClipRect.Min.y, c->ContentMaxXFrozen, table->InnerClipRect.Min.y + table_instance->LastFirstRowHeight); }
            else if (rect_type == TRT_ColumnsContentUnfrozen)   { ImGuiTableColumn* c = &table->Columns[n]; return ImRect(c->WorkMinX, table->InnerClipRect.Min.y + table_instance->LastFirstRowHeight, c->ContentMaxXUnfrozen, table->InnerClipRect.Max.y); }
            IM_ASSERT(0);
            return ImRect();
        }

        static ImRect GetWindowRect(ImGuiWindow* window, int rect_type)
        {
            if (rect_type == WRT_OuterRect)                 { return window.Rect(); }
            else if (rect_type == WRT_OuterRectClipped)     { return window.OuterRectClipped; }
            else if (rect_type == WRT_InnerRect)            { return window.InnerRect; }
            else if (rect_type == WRT_InnerClipRect)        { return window.InnerClipRect; }
            else if (rect_type == WRT_WorkRect)             { return window.WorkRect; }
            else if (rect_type == WRT_Content)       { Vector2D min = window.InnerRect.Min - window.Scroll + window.WindowPadding; return ImRect(min, min + window.ContentSize); }
            else if (rect_type == WRT_ContentIdeal)         { Vector2D min = window.InnerRect.Min - window.Scroll + window.WindowPadding; return ImRect(min, min + window.ContentSizeIdeal); }
            else if (rect_type == WRT_ContentRegionRect)    { return window.ContentRegionRect; }
            IM_ASSERT(0);
            return ImRect();
        }
    };

    // Tools
    if (TreeNode("Tools"))
    {
        bool show_encoding_viewer = TreeNode("UTF-8 Encoding viewer");
        SameLine();
        MetricsHelpMarker("You can also call ImGui::DebugTextEncoding() from your code with a given string to test that your UTF-8 encoding settings are correct.");
        if (show_encoding_viewer)
        {
            static char buf[100] = "";
            SetNextItemWidth(-FLT_MIN);
            InputText("##Text", buf, IM_ARRAYSIZE(buf));
            if (buf[0] != 0)
                DebugTextEncoding(buf);
            TreePop();
        }

        // The Item Picker tool is super useful to visually select an item and break into the call-stack of where it was submitted.
        if (Checkbox("Show Item Picker", &g.DebugItemPickerActive) && g.DebugItemPickerActive)
            DebugStartItemPicker();
        SameLine();
        MetricsHelpMarker("Will call the IM_DEBUG_BREAK() macro to break in debugger.\nWarning: If you don't have a debugger attached, this will probably crash.");

        // Stack Tool is your best friend!
        Checkbox("Show Debug Log", &cfg->ShowDebugLog);
        SameLine();
        MetricsHelpMarker("You can also call ImGui::ShowDebugLogWindow() from your code.");

        // Stack Tool is your best friend!
        Checkbox("Show Stack Tool", &cfg->ShowStackTool);
        SameLine();
        MetricsHelpMarker("You can also call ImGui::ShowStackToolWindow() from your code.");

        Checkbox("Show windows begin order", &cfg->ShowWindowsBeginOrder);
        Checkbox("Show windows rectangles", &cfg->ShowWindowsRects);
        SameLine();
        SetNextItemWidth(GetFontSize() * 12);
        cfg->ShowWindowsRects |= Combo("##show_windows_rect_type", &cfg->ShowWindowsRectsType, wrt_rects_names, WRT_Count, WRT_Count);
        if (cfg->ShowWindowsRects && g.nav_window != NULL)
        {
            BulletText("'%s':", g.nav_window->Name);
            Indent();
            for (int rect_n = 0; rect_n < WRT_Count; rect_n += 1)
            {
                ImRect r = Funcs::GetWindowRect(g.nav_window, rect_n);
                Text("(%6.1,%6.1) (%6.1,%6.1) size (%6.1,%6.1) %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), wrt_rects_names[rect_n]);
            }
            Unindent();
        }

        Checkbox("Show tables rectangles", &cfg->ShowTablesRects);
        SameLine();
        SetNextItemWidth(GetFontSize() * 12);
        cfg->ShowTablesRects |= Combo("##show_table_rects_type", &cfg->ShowTablesRectsType, trt_rects_names, TRT_Count, TRT_Count);
        if (cfg->ShowTablesRects && g.nav_window != NULL)
        {
            for (int table_n = 0; table_n < g.Tables.GetMapSize(); table_n += 1)
            {
                ImGuiTable* table = g.Tables.TryGetMapData(table_n);
                if (table == NULL || table->LastFrameActive < g.FrameCount - 1 || (table->OuterWindow != g.nav_window && table->InnerWindow != g.nav_window))
                    continue;

                BulletText("Table 0x%08X (%d columns, in '%s')", table->ID, table->ColumnsCount, table->OuterWindow->Name);
                if (IsItemHovered())
                    GetForegroundDrawList()->AddRect(table->OuterRect.Min - DimgVec2D::new(1, 1), table->OuterRect.Max + DimgVec2D::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
                Indent();
                char buf[128];
                for (int rect_n = 0; rect_n < TRT_Count; rect_n += 1)
                {
                    if (rect_n >= TRT_ColumnsRect)
                    {
                        if (rect_n != TRT_ColumnsRect && rect_n != TRT_ColumnsClipRect)
                            continue;
                        for (int column_n = 0; column_n < table->ColumnsCount; column_n += 1)
                        {
                            ImRect r = Funcs::GetTableRect(table, rect_n, column_n);
                            ImFormatString(buf, IM_ARRAYSIZE(buf), "(%6.1,%6.1) (%6.1,%6.1) size (%6.1,%6.1) Col %d %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), column_n, trt_rects_names[rect_n]);
                            Selectable(buf);
                            if (IsItemHovered())
                                GetForegroundDrawList()->AddRect(r.Min - DimgVec2D::new(1, 1), r.Max + DimgVec2D::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
                        }
                    }
                    else
                    {
                        ImRect r = Funcs::GetTableRect(table, rect_n, -1);
                        ImFormatString(buf, IM_ARRAYSIZE(buf), "(%6.1,%6.1) (%6.1,%6.1) size (%6.1,%6.1) %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), trt_rects_names[rect_n]);
                        Selectable(buf);
                        if (IsItemHovered())
                            GetForegroundDrawList()->AddRect(r.Min - DimgVec2D::new(1, 1), r.Max + DimgVec2D::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
                    }
                }
                Unindent();
            }
        }

        TreePop();
    }

    // windows
    if (TreeNode("windows", "windows (%d)", g.Windows.Size))
    {
        //SetNextItemOpen(true, ImGuiCond_Once);
        DebugNodeWindowsList(&g.Windows, "By display order");
        DebugNodeWindowsList(&g.WindowsFocusOrder, "By focus order (root windows)");
        if (TreeNode("By submission order (begin stack)"))
        {
            // Here we display windows in their submitted order/hierarchy, however note that the Begin stack doesn't constitute a Parent<>Child relationship!
            ImVector<ImGuiWindow*>& temp_buffer = g.WindowsTempSortBuffer;
            temp_buffer.resize(0);
            for (int i = 0; i < g.Windows.Size; i += 1)
                if (g.Windows[i]->LastFrameActive + 1 >= g.FrameCount)
                    temp_buffer.push_back(g.Windows[i]);
            struct Func { static int IMGUI_CDECL WindowComparerByBeginOrder(const void* lhs, const void* rhs) { return ((*(const ImGuiWindow* const *)lhs)->BeginOrderWithinContext - (*(const ImGuiWindow* const*)rhs)->BeginOrderWithinContext); } };
            ImQsort(temp_buffer.Data, temp_buffer.Size, sizeof(ImGuiWindow*), Func::WindowComparerByBeginOrder);
            DebugNodeWindowsListByBeginStackParent(temp_buffer.Data, temp_buffer.Size, NULL);
            TreePop();
        }

        TreePop();
    }

    // DrawLists
    int drawlist_count = 0;
    for (int viewport_i = 0; viewport_i < g.Viewports.Size; viewport_i += 1)
        drawlist_count += g.Viewports[viewport_i]->DrawDataBuilder.GetDrawListCount();
    if (TreeNode("DrawLists", "DrawLists (%d)", drawlist_count))
    {
        Checkbox("Show ImDrawCmd mesh when hovering", &cfg->ShowDrawCmdMesh);
        Checkbox("Show ImDrawCmd bounding boxes when hovering", &cfg->ShowDrawCmdBoundingBoxes);
        for (int viewport_i = 0; viewport_i < g.Viewports.Size; viewport_i += 1)
        {
            ImGuiViewportP* viewport = g.Viewports[viewport_i];
            bool viewport_has_drawlist = false;
            for (int layer_i = 0; layer_i < IM_ARRAYSIZE(viewport->DrawDataBuilder.Layers); layer_i += 1)
                for (int draw_list_i = 0; draw_list_i < viewport->DrawDataBuilder.Layers[layer_i].Size; draw_list_i += 1)
                {
                    if (!viewport_has_drawlist)
                        Text("active DrawLists in viewport #%d, id: 0x%08X", viewport->Idx, viewport->ID);
                    viewport_has_drawlist = true;
                    DebugNodeDrawList(NULL, viewport, viewport->DrawDataBuilder.Layers[layer_i][draw_list_i], "draw_list");
                }
        }
        TreePop();
    }

    // viewports
    if (TreeNode("viewports", "viewports (%d)", g.Viewports.Size))
    {
        Indent(GetTreeNodeToLabelSpacing());
        RenderViewportsThumbnails();
        Unindent(GetTreeNodeToLabelSpacing());

        bool open = TreeNode("Monitors", "Monitors (%d)", g.PlatformIO.Monitors.Size);
        SameLine();
        MetricsHelpMarker("Dear ImGui uses monitor data:\n- to query DPI settings on a per monitor basis\n- to position popup/tooltips so they don't straddle monitors.");
        if (open)
        {
            for (int i = 0; i < g.PlatformIO.Monitors.Size; i += 1)
            {
                const ImGuiPlatformMonitor& mon = g.PlatformIO.Monitors[i];
                BulletText("Monitor #%d: DPI %.0%%\n MainMin (%.0,%.0), MainMax (%.0,%.0), MainSize (%.0,%.0)\n WorkMin (%.0,%.0), WorkMax (%.0,%.0), work_size (%.0,%.0)",
                    i, mon.DpiScale * 100.0,
                    mon.MainPos.x, mon.MainPos.y, mon.MainPos.x + mon.MainSize.x, mon.MainPos.y + mon.MainSize.y, mon.MainSize.x, mon.MainSize.y,
                    mon.WorkPos.x, mon.WorkPos.y, mon.WorkPos.x + mon.WorkSize.x, mon.WorkPos.y + mon.WorkSize.y, mon.WorkSize.x, mon.WorkSize.y);
            }
            TreePop();
        }

        BulletText("mouse_viewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.mouse_viewport ? g.mouse_viewport->ID : 0, g.io.MouseHoveredViewport, g.MouseLastHoveredViewport ? g.MouseLastHoveredViewport->ID : 0);
        if (TreeNode("Inferred Z order (front-to-back)"))
        {
            static ImVector<ImGuiViewportP*> viewports;
            viewports.resize(g.Viewports.Size);
            memcpy(viewports.Data, g.Viewports.Data, g.Viewports.size_in_bytes());
            if (viewports.Size > 1)
                ImQsort(viewports.Data, viewports.Size, sizeof(ImGuiViewport*), ViewportComparerByFrontMostStampCount);
            for (int i = 0; i < viewports.Size; i += 1)
                BulletText("viewport #%d, id: 0x%08X, FrontMostStampCount = %08d, Window: \"%s\"", viewports[i]->Idx, viewports[i]->ID, viewports[i]->LastFrontMostStampCount, viewports[i]->Window ? viewports[i]->Window->Name : "N/A");
            TreePop();
        }

        for (int i = 0; i < g.Viewports.Size; i += 1)
            DebugNodeViewport(g.Viewports[i]);
        TreePop();
    }

    // Details for Popups
    if (TreeNode("Popups", "Popups (%d)", g.OpenPopupStack.Size))
    {
        for (int i = 0; i < g.OpenPopupStack.Size; i += 1)
        {
            ImGuiWindow* window = g.OpenPopupStack[i].Window;
            BulletText("PopupID: %08x, Window: '%s'%s%s", g.OpenPopupStack[i].PopupId, window ? window.Name : "NULL", window && (window.Flags & ImGuiWindowFlags_ChildWindow) ? " ChildWindow" : "", window && (window.Flags & ImGuiWindowFlags_ChildMenu) ? " ChildMenu" : "");
        }
        TreePop();
    }

    // Details for tab_bars
    if (TreeNode("tab_bars", "Tab Bars (%d)", g.TabBars.GetAliveCount()))
    {
        for (int n = 0; n < g.TabBars.GetMapSize(); n += 1)
            if (ImGuiTabBar* tab_bar = g.TabBars.TryGetMapData(n))
            {
                PushID(tab_bar);
                DebugNodeTabBar(tab_bar, "tab_bar");
                PopID();
            }
        TreePop();
    }

    // Details for tables
    if (TreeNode("tables", "tables (%d)", g.Tables.GetAliveCount()))
    {
        for (int n = 0; n < g.Tables.GetMapSize(); n += 1)
            if (ImGuiTable* table = g.Tables.TryGetMapData(n))
                DebugNodeTable(table);
        TreePop();
    }

    // Details for fonts
    ImFontAtlas* atlas = g.io.Fonts;
    if (TreeNode("fonts", "fonts (%d)", atlas->Fonts.Size))
    {
        ShowFontAtlas(atlas);
        TreePop();
    }

    // Details for InputText
    if (TreeNode("InputText"))
    {
        DebugNodeInputTextState(&g.InputTextState);
        TreePop();
    }

    // Details for Docking
#ifdef IMGUI_HAS_DOCK
    if (TreeNode("Docking"))
    {
        static bool root_nodes_only = true;
        ImGuiDockContext* dc = &g.DockContext;
        Checkbox("List root nodes", &root_nodes_only);
        Checkbox("Ctrl shows window dock info", &cfg->ShowDockingNodes);
        if (SmallButton("clear nodes")) { DockContextClearNodes(&g, 0, true); }
        SameLine();
        if (SmallButton("Rebuild all")) { dc->WantFullRebuild = true; }
        for (int n = 0; n < dc->Nodes.Data.Size; n += 1)
            if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
                if (!root_nodes_only || node->IsRootNode())
                    DebugNodeDockNode(node, "Node");
        TreePop();
    }
 // #ifdef IMGUI_HAS_DOCK

    // Settings
    if (TreeNode("Settings"))
    {
        if (SmallButton("clear"))
            ClearIniSettings();
        SameLine();
        if (SmallButton("Save to memory"))
            SaveIniSettingsToMemory();
        SameLine();
        if (SmallButton("Save to disk"))
            SaveIniSettingsToDisk(g.io.IniFilename);
        SameLine();
        if (g.io.IniFilename)
            Text("\"%s\"", g.io.IniFilename);
        else
            TextUnformatted("<NULL>");
        Text("settings_dirty_timer %.2", g.SettingsDirtyTimer);
        if (TreeNode("settings_handlers", "Settings handlers: (%d)", g.SettingsHandlers.Size))
        {
            for (int n = 0; n < g.SettingsHandlers.Size; n += 1)
                BulletText("%s", g.SettingsHandlers[n].TypeName);
            TreePop();
        }
        if (TreeNode("settings_windows", "Settings packed data: windows: %d bytes", g.SettingsWindows.size()))
        {
            for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != NULL; settings = g.SettingsWindows.next_chunk(settings))
                DebugNodeWindowSettings(settings);
            TreePop();
        }

        if (TreeNode("SettingsTables", "Settings packed data: tables: %d bytes", g.SettingsTables.size()))
        {
            for (ImGuiTableSettings* settings = g.SettingsTables.begin(); settings != NULL; settings = g.SettingsTables.next_chunk(settings))
                DebugNodeTableSettings(settings);
            TreePop();
        }

#ifdef IMGUI_HAS_DOCK
        if (TreeNode("SettingsDocking", "Settings packed data: Docking"))
        {
            ImGuiDockContext* dc = &g.DockContext;
            Text("In settings_windows:");
            for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != NULL; settings = g.SettingsWindows.next_chunk(settings))
                if (settings->DockId != 0)
                    BulletText("Window '%s' -> dock_id %08X", settings->GetName(), settings->DockId);
            Text("In SettingsNodes:");
            for (int n = 0; n < dc->NodesSettings.Size; n += 1)
            {
                ImGuiDockNodeSettings* settings = &dc->NodesSettings[n];
                const char* selected_tab_name = NULL;
                if (settings->SelectedTabId)
                {
                    if (ImGuiWindow* window = FindWindowByID(settings->SelectedTabId))
                        selected_tab_name = window.Name;
                    else if (ImGuiWindowSettings* window_settings = FindWindowSettings(settings->SelectedTabId))
                        selected_tab_name = window_settings->GetName();
                }
                BulletText("Node %08X, Parent %08X, SelectedTab %08X ('%s')", settings->ID, settings->ParentNodeId, settings->SelectedTabId, selected_tab_name ? selected_tab_name : settings->SelectedTabId ? "N/A" : "");
            }
            TreePop();
        }
 // #ifdef IMGUI_HAS_DOCK

        if (TreeNode("settings_ini_data", "Settings unpacked data (.ini): %d bytes", g.SettingsIniData.size()))
        {
            InputTextMultiline("##Ini", (char*)(void*)g.SettingsIniData.c_str(), g.SettingsIniData.Buf.Size, DimgVec2D::new(-FLT_MIN, GetTextLineHeight() * 20), ImGuiInputTextFlags_ReadOnly);
            TreePop();
        }
        TreePop();
    }

    // Misc Details
    if (TreeNode("Internal state"))
    {
        Text("WINDOWING");
        Indent();
        Text("hovered_window: '%s'", g.hovered_window ? g.hovered_window->Name : "NULL");
        Text("hovered_window->Root: '%s'", g.hovered_window ? g.hovered_window->RootWindowDockTree->Name : "NULL");
        Text("hovered_window_under_moving_window: '%s'", g.HoveredWindowUnderMovingWindow ? g.HoveredWindowUnderMovingWindow->Name : "NULL");
        Text("hovered_dock_node: 0x%08X", g.HoveredDockNode ? g.HoveredDockNode->ID : 0);
        Text("moving_window: '%s'", g.moving_window ? g.moving_window->Name : "NULL");
        Text("mouse_viewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.mouse_viewport->ID, g.io.MouseHoveredViewport, g.MouseLastHoveredViewport ? g.MouseLastHoveredViewport->ID : 0);
        Unindent();

        Text("ITEMS");
        Indent();
        Text("active_id: 0x%08X/0x%08X (%.2 sec), AllowOverlap: %d, Source: %s", g.active_id, g.ActiveIdPreviousFrame, g.ActiveIdTimer, g.ActiveIdAllowOverlap, GetInputSourceName(g.ActiveIdSource));
        Text("active_id_window: '%s'", g.active_id_window ? g.active_id_window->Name : "NULL");

        int active_id_using_key_input_count = 0;
        for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n += 1)
            active_id_using_key_input_count += g.ActiveIdUsingKeyInputMask[n] ? 1 : 0;
        Text("ActiveIdUsing: Wheel: %d, NavDirMask: %x, NavInputMask: %x, KeyInputMask: %d key(s)", g.ActiveIdUsingMouseWheel, g.ActiveIdUsingNavDirMask, g.ActiveIdUsingNavInputMask, active_id_using_key_input_count);
        Text("hovered_id: 0x%08X (%.2 sec), AllowOverlap: %d", g.HoveredIdPreviousFrame, g.HoveredIdTimer, g.HoveredIdAllowOverlap); // Not displaying g.hovered_id as it is update mid-frame
        Text("DragDrop: %d, source_id = 0x%08X, Payload \"%s\" (%d bytes)", g.DragDropActive, g.DragDropPayload.SourceId, g.DragDropPayload.DataType, g.DragDropPayload.DataSize);
        Unindent();

        Text("NAV,FOCUS");
        Indent();
        Text("nav_window: '%s'", g.nav_window ? g.nav_window->Name : "NULL");
        Text("nav_id: 0x%08X, nav_layer: %d", g.NavId, g.NavLayer);
        Text("nav_input_source: %s", GetInputSourceName(g.NavInputSource));
        Text("nav_active: %d, nav_visible: %d", g.io.NavActive, g.io.NavVisible);
        Text("nav_activate_id/DownId/PressedId/InputId: %08X/%08X/%08X/%08X", g.NavActivateId, g.NavActivateDownId, g.NavActivatePressedId, g.NavActivateInputId);
        Text("nav_activate_flags: %04X", g.NavActivateFlags);
        Text("NavDisableHighlight: %d, nav_disable_mouse_hover: %d", g.NavDisableHighlight, g.NavDisableMouseHover);
        Text("nav_focus_scope_id = 0x%08X", g.NavFocusScopeId);
        Text("nav_windowing_target: '%s'", g.NavWindowingTarget ? g.NavWindowingTarget->Name : "NULL");
        Unindent();

        TreePop();
    }

    // Overlay: Display windows Rectangles and Begin Order
    if (cfg->ShowWindowsRects || cfg->ShowWindowsBeginOrder)
    {
        for (int n = 0; n < g.Windows.Size; n += 1)
        {
            ImGuiWindow* window = g.Windows[n];
            if (!window.WasActive)
                continue;
            ImDrawList* draw_list = GetForegroundDrawList(window);
            if (cfg->ShowWindowsRects)
            {
                ImRect r = Funcs::GetWindowRect(window, cfg->ShowWindowsRectsType);
                draw_list->AddRect(r.Min, r.Max, IM_COL32(255, 0, 128, 255));
            }
            if (cfg->ShowWindowsBeginOrder && !(window.Flags & ImGuiWindowFlags_ChildWindow))
            {
                char buf[32];
                ImFormatString(buf, IM_ARRAYSIZE(buf), "%d", window.BeginOrderWithinContext);
                float font_size = GetFontSize();
                draw_list->AddRectFilled(window.Pos, window.Pos + DimgVec2D::new(font_size, font_size), IM_COL32(200, 100, 100, 255));
                draw_list->AddText(window.Pos, IM_COL32(255, 255, 255, 255), buf);
            }
        }
    }

    // Overlay: Display tables Rectangles
    if (cfg->ShowTablesRects)
    {
        for (int table_n = 0; table_n < g.Tables.GetMapSize(); table_n += 1)
        {
            ImGuiTable* table = g.Tables.TryGetMapData(table_n);
            if (table == NULL || table->LastFrameActive < g.FrameCount - 1)
                continue;
            ImDrawList* draw_list = GetForegroundDrawList(table->OuterWindow);
            if (cfg->ShowTablesRectsType >= TRT_ColumnsRect)
            {
                for (int column_n = 0; column_n < table->ColumnsCount; column_n += 1)
                {
                    ImRect r = Funcs::GetTableRect(table, cfg->ShowTablesRectsType, column_n);
                    ImU32 col = (table->HoveredColumnBody == column_n) ? IM_COL32(255, 255, 128, 255) : IM_COL32(255, 0, 128, 255);
                    float thickness = (table->HoveredColumnBody == column_n) ? 3.0 : 1.0;
                    draw_list->AddRect(r.Min, r.Max, col, 0.0, 0, thickness);
                }
            }
            else
            {
                ImRect r = Funcs::GetTableRect(table, cfg->ShowTablesRectsType, -1);
                draw_list->AddRect(r.Min, r.Max, IM_COL32(255, 0, 128, 255));
            }
        }
    }

#ifdef IMGUI_HAS_DOCK
    // Overlay: Display Docking info
    if (cfg->ShowDockingNodes && g.io.KeyCtrl && g.HoveredDockNode)
    {
        char buf[64] = "";
        char* p = buf;
        ImGuiDockNode* node = g.HoveredDockNode;
        ImDrawList* overlay_draw_list = node->HostWindow ? GetForegroundDrawList(node->HostWindow) : GetForegroundDrawList(GetMainViewport());
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "dock_id: %x%s\n", node->ID, node->IsCentralNode() ? " *central_node*" : "");
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "window_class: %08X\n", node->WindowClass.ClassId);
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "size: (%.0, %.0)\n", node->Size.x, node->Size.y);
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "size_ref: (%.0, %.0)\n", node->SizeRef.x, node->SizeRef.y);
        int depth = DockNodeGetDepth(node);
        overlay_draw_list->AddRect(node.pos + DimgVec2D::new(3, 3) * (float)depth, node.pos + node->Size - DimgVec2D::new(3, 3) * (float)depth, IM_COL32(200, 100, 100, 255));
        Vector2D pos = node.pos + DimgVec2D::new(3, 3) * (float)depth;
        overlay_draw_list->AddRectFilled(pos - DimgVec2D::new(1, 1), pos + CalcTextSize(buf) + DimgVec2D::new(1, 1), IM_COL32(200, 100, 100, 255));
        overlay_draw_list->AddText(NULL, 0.0, pos, IM_COL32(255, 255, 255, 255), buf);
    }
 // #ifdef IMGUI_HAS_DOCK

    End();
}

// [DEBUG] Display contents of Columns
void ImGui::DebugNodeColumns(ImGuiOldColumns* columns)
{
    if (!TreeNode((void*)(uintptr_t)columns->ID, "Columns Id: 0x%08X, Count: %d, flags: 0x%04X", columns->ID, columns->Count, columns.flags))
        return;
    BulletText("width: %.1 (MinX: %.1, MaxX: %.1)", columns->OffMaxX - columns->OffMinX, columns->OffMinX, columns->OffMaxX);
    for (int column_n = 0; column_n < columns->Columns.Size; column_n += 1)
        BulletText("column %02d: offset_norm %.3 (= %.1 px)", column_n, columns->Columns[column_n].OffsetNorm, GetColumnOffsetFromNorm(columns, columns->Columns[column_n].OffsetNorm));
    TreePop();
}

static void DebugNodeDockNodeFlags(ImGuiDockNodeFlags* p_flags, const char* label, bool enabled)
{
    using namespace ImGui;
    PushID(label);
    PushStyleVar(ImGuiStyleVar_FramePadding, DimgVec2D::new(0.0, 0.0));
    Text("%s:", label);
    if (!enabled)
        BeginDisabled();
    CheckboxFlags("NoSplit", p_flags, ImGuiDockNodeFlags_NoSplit);
    CheckboxFlags("NoResize", p_flags, ImGuiDockNodeFlags_NoResize);
    CheckboxFlags("NoResizeX", p_flags, ImGuiDockNodeFlags_NoResizeX);
    CheckboxFlags("NoResizeY",p_flags, ImGuiDockNodeFlags_NoResizeY);
    CheckboxFlags("NoTabBar", p_flags, ImGuiDockNodeFlags_NoTabBar);
    CheckboxFlags("HiddenTabBar", p_flags, ImGuiDockNodeFlags_HiddenTabBar);
    CheckboxFlags("NoWindowMenuButton", p_flags, ImGuiDockNodeFlags_NoWindowMenuButton);
    CheckboxFlags("NoCloseButton", p_flags, ImGuiDockNodeFlags_NoCloseButton);
    CheckboxFlags("NoDocking", p_flags, ImGuiDockNodeFlags_NoDocking);
    CheckboxFlags("NoDockingSplitMe", p_flags, ImGuiDockNodeFlags_NoDockingSplitMe);
    CheckboxFlags("NoDockingSplitOther", p_flags, ImGuiDockNodeFlags_NoDockingSplitOther);
    CheckboxFlags("NoDockingOverMe", p_flags, ImGuiDockNodeFlags_NoDockingOverMe);
    CheckboxFlags("NoDockingOverOther", p_flags, ImGuiDockNodeFlags_NoDockingOverOther);
    CheckboxFlags("NoDockingOverEmpty", p_flags, ImGuiDockNodeFlags_NoDockingOverEmpty);
    if (!enabled)
        EndDisabled();
    PopStyleVar();
    PopID();
}

// [DEBUG] Display contents of ImDockNode
void ImGui::DebugNodeDockNode(ImGuiDockNode* node, const char* label)
{
    ImGuiContext& g = *GImGui;
    const bool is_alive = (g.FrameCount - node->LastFrameAlive < 2);    // Submitted with ImGuiDockNodeFlags_KeepAliveOnly
    const bool is_active = (g.FrameCount - node->LastFrameActive < 2);  // Submitted
    if (!is_alive) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    bool open;
    ImGuiTreeNodeFlags tree_node_flags = node->IsFocused ? ImGuiTreeNodeFlags_Selected : ImGuiTreeNodeFlags_None;
    if (node->Windows.Size > 0)
        open = TreeNodeEx((void*)(intptr_t)node->ID, tree_node_flags, "%s 0x%04X%s: %d windows (vis: '%s')", label, node->ID, node->IsVisible ? "" : " (hidden)", node->Windows.Size, node->VisibleWindow ? node->VisibleWindow->Name : "NULL");
    else
        open = TreeNodeEx((void*)(intptr_t)node->ID, tree_node_flags, "%s 0x%04X%s: %s split (vis: '%s')", label, node->ID, node->IsVisible ? "" : " (hidden)", (node->SplitAxis == ImGuiAxis_X) ? "horizontal" : (node->SplitAxis == ImGuiAxis_Y) ? "vertical" : "n/a", node->VisibleWindow ? node->VisibleWindow->Name : "NULL");
    if (!is_alive) { PopStyleColor(); }
    if (is_active && IsItemHovered())
        if (ImGuiWindow* window = node->HostWindow ? node->HostWindow : node->VisibleWindow)
            GetForegroundDrawList(window)->AddRect(node.pos, node.pos + node->Size, IM_COL32(255, 255, 0, 255));
    if (open)
    {
        IM_ASSERT(node->ChildNodes[0] == NULL || node->ChildNodes[0]->ParentNode == node);
        IM_ASSERT(node->ChildNodes[1] == NULL || node->ChildNodes[1]->ParentNode == node);
        BulletText("pos (%.0,%.0), size (%.0, %.0) Ref (%.0, %.0)",
            node.pos.x, node.pos.y, node->Size.x, node->Size.y, node->SizeRef.x, node->SizeRef.y);
        DebugNodeWindow(node->HostWindow, "host_window");
        DebugNodeWindow(node->VisibleWindow, "visible_window");
        BulletText("SelectedTabID: 0x%08X, LastFocusedNodeID: 0x%08X", node->SelectedTabId, node->LastFocusedNodeId);
        BulletText("Misc:%s%s%s%s%s%s%s",
            node->IsDockSpace() ? " is_dock_space" : "",
            node->IsCentralNode() ? " is_central_node" : "",
            is_alive ? " IsAlive" : "", is_active ? " IsActive" : "", node->IsFocused ? " is_focused" : "",
            node->WantLockSizeOnce ? " WantLockSizeOnce" : "",
            node->HasCentralNodeChild ? " has_central_node_child" : "");
        if (TreeNode("flags", "flags Merged: 0x%04X, Local: 0x%04X, InWindows: 0x%04X, Shared: 0x%04X", node->MergedFlags, node->LocalFlags, node->LocalFlagsInWindows, node->SharedFlags))
        {
            if (BeginTable("flags", 4))
            {
                TableNextColumn(); DebugNodeDockNodeFlags(&node->MergedFlags, "merged_flags", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->LocalFlags, "local_flags", true);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->LocalFlagsInWindows, "local_flags_in_windows", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->SharedFlags, "shared_flags", true);
                EndTable();
            }
            TreePop();
        }
        if (node->ParentNode)
            DebugNodeDockNode(node->ParentNode, "parent_node");
        if (node->ChildNodes[0])
            DebugNodeDockNode(node->ChildNodes[0], "Child[0]");
        if (node->ChildNodes[1])
            DebugNodeDockNode(node->ChildNodes[1], "Child[1]");
        if (node->TabBar)
            DebugNodeTabBar(node->TabBar, "tab_bar");
        TreePop();
    }
}

// [DEBUG] Display contents of ImDrawList
// Note that both 'window' and 'viewport' may be NULL here. viewport is generally null of destroyed popups which previously owned a viewport.
void ImGui::DebugNodeDrawList(ImGuiWindow* window, ImGuiViewportP* viewport, const ImDrawList* draw_list, const char* label)
{
    ImGuiContext& g = *GImGui;
    ImGuiMetricsConfig* cfg = &g.DebugMetricsConfig;
    int cmd_count = draw_list->CmdBuffer.Size;
    if (cmd_count > 0 && draw_list->CmdBuffer.back().ElemCount == 0 && draw_list->CmdBuffer.back().UserCallback == NULL)
        cmd_count--;
    bool node_open = TreeNode(draw_list, "%s: '%s' %d vtx, %d indices, %d cmds", label, draw_list->_OwnerName ? draw_list->_OwnerName : "", draw_list->VtxBuffer.Size, draw_list->IdxBuffer.Size, cmd_count);
    if (draw_list == GetWindowDrawList())
    {
        SameLine();
        TextColored(Vector4D(1.0, 0.4, 0.4, 1.0), "CURRENTLY APPENDING"); // Can't display stats for active draw list! (we don't have the data double-buffered)
        if (node_open)
            TreePop();
        return;
    }

    ImDrawList* fg_draw_list = viewport ? GetForegroundDrawList(viewport) : NULL; // Render additional visuals into the top-most draw list
    if (window && IsItemHovered() && fg_draw_list)
        fg_draw_list->AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
    if (!node_open)
        return;

    if (window && !window.WasActive)
        TextDisabled("Warning: owning Window is inactive. This draw_list is not being rendered!");

    for (const ImDrawCmd* pcmd = draw_list->CmdBuffer.Data; pcmd < draw_list->CmdBuffer.Data + cmd_count; pcmd += 1)
    {
        if (pcmd->UserCallback)
        {
            BulletText("Callback %p, user_data %p", pcmd->UserCallback, pcmd->UserCallbackData);
            continue;
        }

        char buf[300];
        ImFormatString(buf, IM_ARRAYSIZE(buf), "DrawCmd:%5d tris, Tex 0x%p, clip_rect (%4.0,%4.0)-(%4.0,%4.0)",
            pcmd->ElemCount / 3, (void*)(intptr_t)pcmd->TextureId,
            pcmd->ClipRect.x, pcmd->ClipRect.y, pcmd->ClipRect.z, pcmd->ClipRect.w);
        bool pcmd_node_open = TreeNode((void*)(pcmd - draw_list->CmdBuffer.begin()), "%s", buf);
        if (IsItemHovered() && (cfg->ShowDrawCmdMesh || cfg->ShowDrawCmdBoundingBoxes) && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, cfg->ShowDrawCmdMesh, cfg->ShowDrawCmdBoundingBoxes);
        if (!pcmd_node_open)
            continue;

        // Calculate approximate coverage area (touched pixel count)
        // This will be in pixels squared as long there's no post-scaling happening to the renderer output.
        const ImDrawIdx* idx_buffer = (draw_list->IdxBuffer.Size > 0) ? draw_list->IdxBuffer.Data : NULL;
        const ImDrawVert* vtx_buffer = draw_list->VtxBuffer.Data + pcmd->VtxOffset;
        float total_area = 0.0;
        for (unsigned int idx_n = pcmd->IdxOffset; idx_n < pcmd->IdxOffset + pcmd->ElemCount; )
        {
            Vector2D triangle[3];
            for (int n = 0; n < 3; n += 1, idx_n += 1)
                triangle[n] = vtx_buffer[idx_buffer ? idx_buffer[idx_n] : idx_n].pos;
            total_area += ImTriangleArea(triangle[0], triangle[1], triangle[2]);
        }

        // Display vertex information summary. Hover to get all triangles drawn in wire-frame
        ImFormatString(buf, IM_ARRAYSIZE(buf), "Mesh: elem_count: %d, vtx_offset: +%d, idx_offset: +%d, Area: ~%0.f px", pcmd->ElemCount, pcmd->VtxOffset, pcmd->IdxOffset, total_area);
        Selectable(buf);
        if (IsItemHovered() && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, true, false);

        // Display individual triangles/vertices. Hover on to get the corresponding triangle highlighted.
        ImGuiListClipper clipper;
        clipper.Begin(pcmd->ElemCount / 3); // Manually coarse clip our print out of individual vertices to save CPU, only items that may be visible.
        while (clipper.Step())
            for (int prim = clipper.DisplayStart, idx_i = pcmd->IdxOffset + clipper.DisplayStart * 3; prim < clipper.DisplayEnd; prim += 1)
            {
                char* buf_p = buf, * buf_end = buf + IM_ARRAYSIZE(buf);
                Vector2D triangle[3];
                for (int n = 0; n < 3; n += 1, idx_i += 1)
                {
                    const ImDrawVert& v = vtx_buffer[idx_buffer ? idx_buffer[idx_i] : idx_i];
                    triangle[n] = v.pos;
                    buf_p += ImFormatString(buf_p, buf_end - buf_p, "%s %04d: pos (%8.2,%8.2), uv (%.6,%.6), col %08X\n",
                        (n == 0) ? "Vert:" : "     ", idx_i, v.pos.x, v.pos.y, v.uv.x, v.uv.y, v.col);
                }

                Selectable(buf, false);
                if (fg_draw_list && IsItemHovered())
                {
                    ImDrawListFlags backup_flags = fg_draw_list.flags;
                    fg_draw_list.flags &= ~ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
                    fg_draw_list->AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), ImDrawFlags_Closed, 1.0);
                    fg_draw_list.flags = backup_flags;
                }
            }
        TreePop();
    }
    TreePop();
}

// [DEBUG] Display mesh/aabb of a ImDrawCmd
void ImGui::DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList* out_draw_list, const ImDrawList* draw_list, const ImDrawCmd* draw_cmd, bool show_mesh, bool show_aabb)
{
    IM_ASSERT(show_mesh || show_aabb);

    // Draw wire-frame version of all triangles
    ImRect clip_rect = draw_cmd->ClipRect;
    ImRect vtxs_rect(FLT_MAX, FLT_MAX, -FLT_MAX, -FLT_MAX);
    ImDrawListFlags backup_flags = out_draw_list.flags;
    out_draw_list.flags &= ~ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
    for (unsigned int idx_n = draw_cmd->IdxOffset, idx_end = draw_cmd->IdxOffset + draw_cmd->ElemCount; idx_n < idx_end; )
    {
        ImDrawIdx* idx_buffer = (draw_list->IdxBuffer.Size > 0) ? draw_list->IdxBuffer.Data : NULL; // We don't hold on those pointers past iterations as ->AddPolyline() may invalidate them if out_draw_list==draw_list
        ImDrawVert* vtx_buffer = draw_list->VtxBuffer.Data + draw_cmd->VtxOffset;

        Vector2D triangle[3];
        for (int n = 0; n < 3; n += 1, idx_n += 1)
            vtxs_rect.Add((triangle[n] = vtx_buffer[idx_buffer ? idx_buffer[idx_n] : idx_n].pos));
        if (show_mesh)
            out_draw_list->AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), ImDrawFlags_Closed, 1.0); // In yellow: mesh triangles
    }
    // Draw bounding boxes
    if (show_aabb)
    {
        out_draw_list->AddRect(ImFloor(clip_rect.Min), ImFloor(clip_rect.Max), IM_COL32(255, 0, 255, 255)); // In pink: clipping rectangle submitted to GPU
        out_draw_list->AddRect(ImFloor(vtxs_rect.Min), ImFloor(vtxs_rect.Max), IM_COL32(0, 255, 255, 255)); // In cyan: bounding box of triangles
    }
    out_draw_list.flags = backup_flags;
}

// [DEBUG] Display details for a single font, called by ShowStyleEditor().
void ImGui::DebugNodeFont(ImFont* font)
{
    bool opened = TreeNode(font, "font: \"%s\"\n%.2 px, %d glyphs, %d file(s)",
        font->ConfigData ? font->ConfigData[0].Name : "", font->FontSize, font->Glyphs.Size, font->ConfigDataCount);
    SameLine();
    if (SmallButton("Set as default"))
        GetIO().FontDefault = font;
    if (!opened)
        return;

    // Display preview text
    PushFont(font);
    Text("The quick brown fox jumps over the lazy dog");
    PopFont();

    // Display details
    SetNextItemWidth(GetFontSize() * 8);
    DragFloat("font scale", &font->Scale, 0.005, 0.3, 2.0, "%.1");
    SameLine(); MetricsHelpMarker(
        "Note than the default embedded font is NOT meant to be scaled.\n\n"
        "font are currently rendered into bitmaps at a given size at the time of building the atlas. "
        "You may oversample them to get some flexibility with scaling. "
        "You can also render at multiple sizes and select which one to use at runtime.\n\n"
        "(Glimmer of hope: the atlas system will be rewritten in the future to make scaling more flexible.)");
    Text("ascent: %f, descent: %f, height: %f", font->Ascent, font->Descent, font->Ascent - font->Descent);
    char c_str[5];
    Text("Fallback character: '%s' (U+%04X)", ImTextCharToUtf8(c_str, font->FallbackChar), font->FallbackChar);
    Text("Ellipsis character: '%s' (U+%04X)", ImTextCharToUtf8(c_str, font->EllipsisChar), font->EllipsisChar);
    const int surface_sqrt = ImSqrt((float)font->MetricsTotalSurface);
    Text("Texture Area: about %d px ~%dx%d px", font->MetricsTotalSurface, surface_sqrt, surface_sqrt);
    for (int config_i = 0; config_i < font->ConfigDataCount; config_i += 1)
        if (font->ConfigData)
            if (const ImFontConfig* cfg = &font->ConfigData[config_i])
                BulletText("Input %d: \'%s\', Oversample: (%d,%d), pixel_snap_h: %d, Offset: (%.1,%.1)",
                    config_i, cfg->Name, cfg->OversampleH, cfg->OversampleV, cfg->PixelSnapH, cfg->GlyphOffset.x, cfg->GlyphOffset.y);

    // Display all glyphs of the fonts in separate pages of 256 characters
    if (TreeNode("glyphs", "glyphs (%d)", font->Glyphs.Size))
    {
        ImDrawList* draw_list = GetWindowDrawList();
        const ImU32 glyph_col = GetColorU32(ImGuiCol_Text);
        const float cell_size = font->FontSize * 1;
        const float cell_spacing = GetStyle().ItemSpacing.y;
        for (unsigned int base = 0; base <= IM_UNICODE_CODEPOINT_MAX; base += 256)
        {
            // Skip ahead if a large bunch of glyphs are not present in the font (test in chunks of 4k)
            // This is only a small optimization to reduce the number of iterations when IM_UNICODE_MAX_CODEPOINT
            // is large // (if ImWchar==ImWchar32 we will do at least about 272 queries here)
            if (!(base & 4095) && font->IsGlyphRangeUnused(base, base + 4095))
            {
                base += 4096 - 256;
                continue;
            }

            int count = 0;
            for (unsigned int n = 0; n < 256; n += 1)
                if (font->FindGlyphNoFallback((ImWchar)(base + n)))
                    count += 1;
            if (count <= 0)
                continue;
            if (!TreeNode((void*)(intptr_t)base, "U+%04X..U+%04X (%d %s)", base, base + 255, count, count > 1 ? "glyphs" : "glyph"))
                continue;

            // Draw a 16x16 grid of glyphs
            Vector2D base_pos = GetCursorScreenPos();
            for (unsigned int n = 0; n < 256; n += 1)
            {
                // We use ImFont::render_char as a shortcut because we don't have UTF-8 conversion functions
                // available here and thus cannot easily generate a zero-terminated UTF-8 encoded string.
                Vector2D cell_p1(base_pos.x + (n % 16) * (cell_size + cell_spacing), base_pos.y + (n / 16) * (cell_size + cell_spacing));
                Vector2D cell_p2(cell_p1.x + cell_size, cell_p1.y + cell_size);
                const ImFontGlyph* glyph = font->FindGlyphNoFallback((ImWchar)(base + n));
                draw_list->AddRect(cell_p1, cell_p2, glyph ? IM_COL32(255, 255, 255, 100) : IM_COL32(255, 255, 255, 50));
                if (!glyph)
                    continue;
                font->RenderChar(draw_list, cell_size, cell_p1, glyph_col, (ImWchar)(base + n));
                if (IsMouseHoveringRect(cell_p1, cell_p2))
                {
                    BeginTooltip();
                    DebugNodeFontGlyph(font, glyph);
                    EndTooltip();
                }
            }
            Dummy(DimgVec2D::new((cell_size + cell_spacing) * 16, (cell_size + cell_spacing) * 16));
            TreePop();
        }
        TreePop();
    }
    TreePop();
}

void ImGui::DebugNodeFontGlyph(ImFont*, const ImFontGlyph* glyph)
{
    Text("codepoint: U+%04X", glyph->Codepoint);
    Separator();
    Text("visible: %d", glyph->Visible);
    Text("advance_x: %.1", glyph->AdvanceX);
    Text("pos: (%.2,%.2)->(%.2,%.2)", glyph->X0, glyph->Y0, glyph->X1, glyph->Y1);
    Text("UV: (%.3,%.3)->(%.3,%.3)", glyph->U0, glyph->V0, glyph->U1, glyph->V1);
}

// [DEBUG] Display contents of ImGuiStorage
void ImGui::DebugNodeStorage(ImGuiStorage* storage, const char* label)
{
    if (!TreeNode(label, "%s: %d entries, %d bytes", label, storage->Data.Size, storage->Data.size_in_bytes()))
        return;
    for (int n = 0; n < storage->Data.Size; n += 1)
    {
        const ImGuiStorage::ImGuiStoragePair& p = storage->Data[n];
        BulletText("Key 0x%08X Value { i: %d }", p.key, p.val_i); // Important: we currently don't store a type, real value may not be integer.
    }
    TreePop();
}

// [DEBUG] Display contents of ImGuiTabBar
void ImGui::DebugNodeTabBar(ImGuiTabBar* tab_bar, const char* label)
{
    // Standalone tab bars (not associated to docking/windows functionality) currently hold no discernible strings.
    char buf[256];
    char* p = buf;
    const char* buf_end = buf + IM_ARRAYSIZE(buf);
    const bool is_active = (tab_bar->PrevFrameVisible >= GetFrameCount() - 2);
    p += ImFormatString(p, buf_end - p, "%s 0x%08X (%d tabs)%s", label, tab_bar->ID, tab_bar->Tabs.Size, is_active ? "" : " *Inactive*");
    p += ImFormatString(p, buf_end - p, "  { ");
    for (int tab_n = 0; tab_n < ImMin(tab_bar->Tabs.Size, 3); tab_n += 1)
    {
        ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
        p += ImFormatString(p, buf_end - p, "%s'%s'",
            tab_n > 0 ? ", " : "", (tab->Window || tab->NameOffset != -1) ? tab_bar->GetTabName(tab) : "???");
    }
    p += ImFormatString(p, buf_end - p, (tab_bar->Tabs.Size > 3) ? " ... }" : " } ");
    if (!is_active) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    bool open = TreeNode(label, "%s", buf);
    if (!is_active) { PopStyleColor(); }
    if (is_active && IsItemHovered())
    {
        ImDrawList* draw_list = GetForegroundDrawList();
        draw_list->AddRect(tab_bar->BarRect.Min, tab_bar->BarRect.Max, IM_COL32(255, 255, 0, 255));
        draw_list->AddLine(DimgVec2D::new(tab_bar->ScrollingRectMinX, tab_bar->BarRect.Min.y), DimgVec2D::new(tab_bar->ScrollingRectMinX, tab_bar->BarRect.Max.y), IM_COL32(0, 255, 0, 255));
        draw_list->AddLine(DimgVec2D::new(tab_bar->ScrollingRectMaxX, tab_bar->BarRect.Min.y), DimgVec2D::new(tab_bar->ScrollingRectMaxX, tab_bar->BarRect.Max.y), IM_COL32(0, 255, 0, 255));
    }
    if (open)
    {
        for (int tab_n = 0; tab_n < tab_bar->Tabs.Size; tab_n += 1)
        {
            const ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
            PushID(tab);
            if (SmallButton("<")) { TabBarQueueReorder(tab_bar, tab, -1); } SameLine(0, 2);
            if (SmallButton(">")) { TabBarQueueReorder(tab_bar, tab, +1); } SameLine();
            Text("%02d%c Tab 0x%08X '%s' Offset: %.1, width: %.1/%.1",
                tab_n, (tab->ID == tab_bar->SelectedTabId) ? '*' : ' ', tab->ID, (tab->Window || tab->NameOffset != -1) ? tab_bar->GetTabName(tab) : "???", tab->Offset, tab->Width, tab->ContentWidth);
            PopID();
        }
        TreePop();
    }
}

void ImGui::DebugNodeViewport(ImGuiViewportP* viewport)
{
    SetNextItemOpen(true, ImGuiCond_Once);
    if (TreeNode((void*)(intptr_t)viewport->ID, "viewport #%d, id: 0x%08X, Parent: 0x%08X, Window: \"%s\"", viewport->Idx, viewport->ID, viewport->ParentViewportId, viewport->Window ? viewport->Window->Name : "N/A"))
    {
        ImGuiWindowFlags flags = viewport.flags;
        BulletText("Main pos: (%.0,%.0), size: (%.0,%.0)\nWorkArea Offset Left: %.0 Top: %.0, Right: %.0, Bottom: %.0\nMonitor: %d, dpi_scale: %.0%%",
            viewport.pos.x, viewport.pos.y, viewport->Size.x, viewport->Size.y,
            viewport->WorkOffsetMin.x, viewport->WorkOffsetMin.y, viewport->WorkOffsetMax.x, viewport->WorkOffsetMax.y,
            viewport->PlatformMonitor, viewport->DpiScale * 100.0);
        if (viewport->Idx > 0) { SameLine(); if (SmallButton("Reset pos")) { viewport.pos = DimgVec2D::new(200, 200); viewport.update_work_rect(); if (viewport->Window) viewport->Window.pos = viewport.pos; } }
        BulletText("flags: 0x%04X =%s%s%s%s%s%s%s%s%s%s%s%s", viewport.flags,
            //(flags & ImGuiViewportFlags_IsPlatformWindow) ? " IsPlatformWindow" : "", // Omitting because it is the standard
            (flags & ImGuiViewportFlags_IsPlatformMonitor) ? " IsPlatformMonitor" : "",
            (flags & ImGuiViewportFlags_OwnedByApp) ? " OwnedByApp" : "",
            (flags & ImGuiViewportFlags_NoDecoration) ? " NoDecoration" : "",
            (flags & ImGuiViewportFlags_NoTaskBarIcon) ? " NoTaskBarIcon" : "",
            (flags & ImGuiViewportFlags_NoFocusOnAppearing) ? " NoFocusOnAppearing" : "",
            (flags & ImGuiViewportFlags_NoFocusOnClick) ? " NoFocusOnClick" : "",
            (flags & ViewportFlags::NoInputs) ? " NoInputs" : "",
            (flags & ImGuiViewportFlags_NoRendererClear) ? " NoRendererClear" : "",
            (flags & ImGuiViewportFlags_TopMost) ? " TopMost" : "",
            (flags & ImGuiViewportFlags_Minimized) ? " Minimized" : "",
            (flags & ImGuiViewportFlags_NoAutoMerge) ? " NoAutoMerge" : "",
            (flags & ImGuiViewportFlags_CanHostOtherWindows) ? " CanHostOtherWindows" : "");
        for (int layer_i = 0; layer_i < IM_ARRAYSIZE(viewport->DrawDataBuilder.Layers); layer_i += 1)
            for (int draw_list_i = 0; draw_list_i < viewport->DrawDataBuilder.Layers[layer_i].Size; draw_list_i += 1)
                DebugNodeDrawList(NULL, viewport, viewport->DrawDataBuilder.Layers[layer_i][draw_list_i], "draw_list");
        TreePop();
    }
}

void ImGui::DebugNodeWindow(ImGuiWindow* window, const char* label)
{
    if (window == NULL)
    {
        BulletText("%s: NULL", label);
        return;
    }

    ImGuiContext& g = *GImGui;
    const bool is_active = window.WasActive;
    ImGuiTreeNodeFlags tree_node_flags = (window == g.nav_window) ? ImGuiTreeNodeFlags_Selected : ImGuiTreeNodeFlags_None;
    if (!is_active) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    const bool open = TreeNodeEx(label, tree_node_flags, "%s '%s'%s", label, window.Name, is_active ? "" : " *Inactive*");
    if (!is_active) { PopStyleColor(); }
    if (IsItemHovered() && is_active)
        GetForegroundDrawList(window)->AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
    if (!open)
        return;

    if (window.MemoryCompacted)
        TextDisabled("Note: some memory buffers have been compacted/freed.");

    ImGuiWindowFlags flags = window.Flags;
    DebugNodeDrawList(window, window.viewport, window.DrawList, "draw_list");
    BulletText("pos: (%.1,%.1), size: (%.1,%.1), content_size (%.1,%.1) Ideal (%.1,%.1)", window.Pos.x, window.Pos.y, window.Size.x, window.Size.y, window.ContentSize.x, window.ContentSize.y, window.ContentSizeIdeal.x, window.ContentSizeIdeal.y);
    BulletText("flags: 0x%08X (%s%s%s%s%s%s%s%s%s..)", flags,
        (flags & ImGuiWindowFlags_ChildWindow)  ? "Child " : "",      (flags & ImGuiWindowFlags_Tooltip)     ? "Tooltip "   : "",  (flags & ImGuiWindowFlags_Popup) ? "Popup " : "",
        (flags & ImGuiWindowFlags_Modal)        ? "Modal " : "",      (flags & ImGuiWindowFlags_ChildMenu)   ? "ChildMenu " : "",  (flags & ImGuiWindowFlags_NoSavedSettings) ? "NoSavedSettings " : "",
        (flags & ImGuiWindowFlags_NoMouseInputs)? "NoMouseInputs":"", (flags & ImGuiWindowFlags_NoNavInputs) ? "NoNavInputs" : "", (flags & ImGuiWindowFlags_AlwaysAutoResize) ? "AlwaysAutoResize" : "");
    BulletText("WindowClassId: 0x%08X", window.WindowClass.ClassId);
    BulletText("scroll: (%.2/%.2,%.2/%.2) Scrollbar:%s%s", window.Scroll.x, window.ScrollMax.x, window.Scroll.y, window.ScrollMax.y, window.ScrollbarX ? "x" : "", window.ScrollbarY ? "Y" : "");
    BulletText("active: %d/%d, write_accessed: %d, begin_order_within_context: %d", window.Active, window.WasActive, window.WriteAccessed, (window.Active || window.WasActive) ? window.BeginOrderWithinContext : -1);
    BulletText("appearing: %d, hidden: %d (CanSkip %d Cannot %d), skip_items: %d", window.Appearing, window.Hidden, window.HiddenFramesCanSkipItems, window.HiddenFramesCannotSkipItems, window.SkipItems);
    for (int layer = 0; layer < ImGuiNavLayer_COUNT; layer += 1)
    {
        ImRect r = window.NavRectRel[layer];
        if (r.Min.x >= r.Max.y && r.Min.y >= r.Max.y)
        {
            BulletText("nav_last_ids[%d]: 0x%08X", layer, window.NavLastIds[layer]);
            continue;
        }
        BulletText("nav_last_ids[%d]: 0x%08X at +(%.1,%.1)(%.1,%.1)", layer, window.NavLastIds[layer], r.Min.x, r.Min.y, r.Max.x, r.Max.y);
        if (IsItemHovered())
            GetForegroundDrawList(window)->AddRect(r.Min + window.Pos, r.Max + window.Pos, IM_COL32(255, 255, 0, 255));
    }
    BulletText("NavLayersActiveMask: %x, nav_last_child_nav_window: %s", window.DC.NavLayersActiveMask, window.NavLastChildNavWindow ? window.NavLastChildNavWindow->Name : "NULL");

    BulletText("viewport: %d%s, viewport_id: 0x%08X, viewport_pos: (%.1,%.1)", window.viewport ? window.viewport->Idx : -1, window.viewport_owned ? " (Owned)" : "", window.ViewportId, window.ViewportPos.x, window.ViewportPos.y);
    BulletText("ViewportMonitor: %d", window.viewport ? window.viewport->PlatformMonitor : -1);
    BulletText("dock_id: 0x%04X, dock_order: %d, Act: %d, Vis: %d", window.DockId, window.DockOrder, window.DockIsActive, window.DockTabIsVisible);
    if (window.DockNode || window.DockNodeAsHost)
        DebugNodeDockNode(window.DockNodeAsHost ? window.DockNodeAsHost : window.DockNode, window.DockNodeAsHost ? "dock_node_as_host" : "dock_node");

    if (window.RootWindow != window)       { DebugNodeWindow(window.RootWindow, "root_window"); }
    if (window.RootWindowDockTree != window.RootWindow) { DebugNodeWindow(window.RootWindowDockTree, "root_window_dock_tree"); }
    if (window.ParentWindow != NULL)       { DebugNodeWindow(window.ParentWindow, "ParentWindow"); }
    if (window.DC.ChildWindows.Size > 0)   { DebugNodeWindowsList(&window.DC.ChildWindows, "ChildWindows"); }
    if (window.ColumnsStorage.Size > 0 && TreeNode("Columns", "Columns sets (%d)", window.ColumnsStorage.Size))
    {
        for (int n = 0; n < window.ColumnsStorage.Size; n += 1)
            DebugNodeColumns(&window.ColumnsStorage[n]);
        TreePop();
    }
    DebugNodeStorage(&window.StateStorage, "Storage");
    TreePop();
}

void ImGui::DebugNodeWindowSettings(ImGuiWindowSettings* settings)
{
    Text("0x%08X \"%s\" pos (%d,%d) size (%d,%d) collapsed=%d",
        settings->ID, settings->GetName(), settings.pos.x, settings.pos.y, settings->Size.x, settings->Size.y, settings->Collapsed);
}

void ImGui::DebugNodeWindowsList(ImVector<ImGuiWindow*>* windows, const char* label)
{
    if (!TreeNode(label, "%s (%d)", label, windows->Size))
        return;
    for (int i = windows->Size - 1; i >= 0; i--) // Iterate front to back
    {
        PushID((*windows)[i]);
        DebugNodeWindow((*windows)[i], "Window");
        PopID();
    }
    TreePop();
}

// FIXME-OPT: This is technically suboptimal, but it is simpler this way.
void ImGui::DebugNodeWindowsListByBeginStackParent(ImGuiWindow** windows, int windows_size, ImGuiWindow* parent_in_begin_stack)
{
    for (int i = 0; i < windows_size; i += 1)
    {
        ImGuiWindow* window = windows[i];
        if (window.ParentWindowInBeginStack != parent_in_begin_stack)
            continue;
        char buf[20];
        ImFormatString(buf, IM_ARRAYSIZE(buf), "[%04d] Window", window.BeginOrderWithinContext);
        //BulletText("[%04d] Window '%s'", window->begin_order_within_context, window->name);
        DebugNodeWindow(window, buf);
        Indent();
        DebugNodeWindowsListByBeginStackParent(windows + i + 1, windows_size - i - 1, window);
        Unindent();
    }
}

//-----------------------------------------------------------------------------
// [SECTION] DEBUG LOG
//-----------------------------------------------------------------------------

void ImGui::DebugLog(const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    DebugLogV(fmt, args);
    va_end(args);
}

void ImGui::DebugLogV(const char* fmt, va_list args)
{
    ImGuiContext& g = *GImGui;
    const int old_size = g.DebugLogBuf.size();
    g.DebugLogBuf.appendf("[%05d] ", g.FrameCount);
    g.DebugLogBuf.appendfv(fmt, args);
    if (g.DebugLogFlags & ImGuiDebugLogFlags_OutputToTTY)
        IMGUI_DEBUG_PRINTF("%s", g.DebugLogBuf.begin() + old_size);
}

void ImGui::ShowDebugLogWindow(bool* p_open)
{
    ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize))
        SetNextWindowSize(DimgVec2D::new(0.0, GetFontSize() * 12.0), ImGuiCond_FirstUseEver);
    if (!Begin("Dear ImGui Debug Log", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        End();
        return;
    }

    AlignTextToFramePadding();
    Text("Log events:");
    SameLine(); CheckboxFlags("All", &g.DebugLogFlags, ImGuiDebugLogFlags_EventMask_);
    SameLine(); CheckboxFlags("active_id", &g.DebugLogFlags, ImGuiDebugLogFlags_EventActiveId);
    SameLine(); CheckboxFlags("Focus", &g.DebugLogFlags, ImGuiDebugLogFlags_EventFocus);
    SameLine(); CheckboxFlags("Popup", &g.DebugLogFlags, ImGuiDebugLogFlags_EventPopup);
    SameLine(); CheckboxFlags("Nav", &g.DebugLogFlags, ImGuiDebugLogFlags_EventNav);
    SameLine(); CheckboxFlags("Docking", &g.DebugLogFlags, ImGuiDebugLogFlags_EventDocking);
    SameLine(); CheckboxFlags("viewport", &g.DebugLogFlags, ImGuiDebugLogFlags_EventViewport);

    if (SmallButton("clear"))
        g.DebugLogBuf.clear();
    SameLine();
    if (SmallButton("Copy"))
        SetClipboardText(g.DebugLogBuf.c_str());
    BeginChild("##log", DimgVec2D::new(0.0, 0.0), true, ImGuiWindowFlags_AlwaysVerticalScrollbar | ImGuiWindowFlags_AlwaysHorizontalScrollbar);
    TextUnformatted(g.DebugLogBuf.begin(), g.DebugLogBuf.end()); // FIXME-OPT: Could use a line index, but TextUnformatted() has a semi-decent fast path for large text.
    if (GetScrollY() >= GetScrollMaxY())
        SetScrollHereY(1.0);
    EndChild();

    End();
}

//-----------------------------------------------------------------------------
// [SECTION] OTHER DEBUG TOOLS (ITEM PICKER, STACK TOOL)
//-----------------------------------------------------------------------------

// [DEBUG] Item picker tool - start with DebugStartItemPicker() - useful to visually select an item and break into its call-stack.
void ImGui::UpdateDebugToolItemPicker()
{
    ImGuiContext& g = *GImGui;
    g.DebugItemPickerBreakId = 0;
    if (!g.DebugItemPickerActive)
        return;

    const ImGuiID hovered_id = g.HoveredIdPreviousFrame;
    SetMouseCursor(ImGuiMouseCursor_Hand);
    if (IsKeyPressed(ImGuiKey_Escape))
        g.DebugItemPickerActive = false;
    if (IsMouseClicked(0) && hovered_id)
    {
        g.DebugItemPickerBreakId = hovered_id;
        g.DebugItemPickerActive = false;
    }
    SetNextWindowBgAlpha(0.60);
    BeginTooltip();
    Text("hovered_id: 0x%08X", hovered_id);
    Text("Press ESC to abort picking.");
    TextColored(GetStyleColorVec4(hovered_id ? ImGuiCol_Text : ImGuiCol_TextDisabled), "Click to break in debugger!");
    EndTooltip();
}

// [DEBUG] Stack Tool: update queries. Called by NewFrame()
void ImGui::UpdateDebugToolStackQueries()
{
    ImGuiContext& g = *GImGui;
    ImGuiStackTool* tool = &g.DebugStackTool;

    // clear hook when stack tool is not visible
    g.DebugHookIdInfo = 0;
    if (g.FrameCount != tool->LastActiveFrame + 1)
        return;

    // Update queries. The steps are: -1: query Stack, >= 0: query each stack item
    // We can only perform 1 id Info query every frame. This is designed so the GetID() tests are cheap and constant-time
    const ImGuiID query_id = g.HoveredIdPreviousFrame ? g.HoveredIdPreviousFrame : g.active_id;
    if (tool->QueryId != query_id)
    {
        tool->QueryId = query_id;
        tool->StackLevel = -1;
        tool->Results.resize(0);
    }
    if (query_id == 0)
        return;

    // Advance to next stack level when we got our result, or after 2 frames (in case we never get a result)
    int stack_level = tool->StackLevel;
    if (stack_level >= 0 && stack_level < tool->Results.Size)
        if (tool->Results[stack_level].QuerySuccess || tool->Results[stack_level].QueryFrameCount > 2)
            tool->StackLevel += 1;

    // Update hook
    stack_level = tool->StackLevel;
    if (stack_level == -1)
        g.DebugHookIdInfo = query_id;
    if (stack_level >= 0 && stack_level < tool->Results.Size)
    {
        g.DebugHookIdInfo = tool->Results[stack_level].ID;
        tool->Results[stack_level].QueryFrameCount += 1;
    }
}

// [DEBUG] Stack tool: hooks called by GetID() family functions
void ImGui::DebugHookIdInfo(ImGuiID id, ImGuiDataType data_type, const void* data_id, const void* data_id_end)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    ImGuiStackTool* tool = &g.DebugStackTool;

    // Step 0: stack query
    // This assume that the id was computed with the current id stack, which tends to be the case for our widget.
    if (tool->StackLevel == -1)
    {
        tool->StackLevel += 1;
        tool->Results.resize(window.IDStack.Size + 1, ImGuiStackLevelInfo());
        for (int n = 0; n < window.IDStack.Size + 1; n += 1)
            tool->Results[n].ID = (n < window.IDStack.Size) ? window.IDStack[n] : id;
        return;
    }

    // Step 1+: query for individual level
    IM_ASSERT(tool->StackLevel >= 0);
    if (tool->StackLevel != window.IDStack.Size)
        return;
    ImGuiStackLevelInfo* info = &tool->Results[tool->StackLevel];
    IM_ASSERT(info->ID == id && info->QueryFrameCount > 0);

    switch (data_type)
    {
    case ImGuiDataType_S32:
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "%d", (intptr_t)data_id);
        break;
    case ImGuiDataType_String:
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "%.*s", data_id_end ? ((const char*)data_id_end - (const char*)data_id) : strlen((const char*)data_id), (const char*)data_id);
        break;
    case ImGuiDataType_Pointer:
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "(void*)0x%p", data_id);
        break;
    case ImGuiDataType_ID:
        if (info->Desc[0] != 0) // PushOverrideID() is often used to avoid hashing twice, which would lead to 2 calls to debug_hook_id_info(). We prioritize the first one.
            return;
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "0x%08X [override]", id);
        break;
    default:
        IM_ASSERT(0);
    }
    info->QuerySuccess = true;
    info->DataType = data_type;
}

static int StackToolFormatLevelInfo(ImGuiStackTool* tool, int n, bool format_for_ui, char* buf, size_t buf_size)
{
    ImGuiStackLevelInfo* info = &tool->Results[n];
    ImGuiWindow* window = (info->Desc[0] == 0 && n == 0) ? ImGui::FindWindowByID(info->ID) : NULL;
    if (window)                                                                 // Source: window name (because the root id don't call GetID() and so doesn't get hooked)
        return ImFormatString(buf, buf_size, format_for_ui ? "\"%s\" [window]" : "%s", window.Name);
    if (info->QuerySuccess)                                                     // Source: GetID() hooks (prioritize over ItemInfo() because we frequently use patterns like: PushID(str), Button("") where they both have same id)
        return ImFormatString(buf, buf_size, (format_for_ui && info->DataType == ImGuiDataType_String) ? "\"%s\"" : "%s", info->Desc);
    if (tool->StackLevel < tool->Results.Size)                                  // Only start using fallback below when all queries are done, so during queries we don't flickering ??? markers.
        return (*buf = 0);
#ifdef IMGUI_ENABLE_TEST_ENGINE
    if (const char* label = ImGuiTestEngine_FindItemDebugLabel(GImGui, info->ID))   // Source: ImGuiTestEngine's ItemInfo()
        return ImFormatString(buf, buf_size, format_for_ui ? "??? \"%s\"" : "%s", label);

    return ImFormatString(buf, buf_size, "???");
}

// Stack Tool: Display UI
void ImGui::ShowStackToolWindow(bool* p_open)
{
    ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize))
        SetNextWindowSize(DimgVec2D::new(0.0, GetFontSize() * 8.0), ImGuiCond_FirstUseEver);
    if (!Begin("Dear ImGui Stack Tool", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        End();
        return;
    }

    // Display hovered/active status
    ImGuiStackTool* tool = &g.DebugStackTool;
    const ImGuiID hovered_id = g.HoveredIdPreviousFrame;
    const ImGuiID active_id = g.active_id;
#ifdef IMGUI_ENABLE_TEST_ENGINE
    Text("hovered_id: 0x%08X (\"%s\"), active_id:  0x%08X (\"%s\")", hovered_id, hovered_id ? ImGuiTestEngine_FindItemDebugLabel(&g, hovered_id) : "", active_id, active_id ? ImGuiTestEngine_FindItemDebugLabel(&g, active_id) : "");
#else
    Text("hovered_id: 0x%08X, active_id:  0x%08X", hovered_id, active_id);

    SameLine();
    MetricsHelpMarker("Hover an item with the mouse to display elements of the id Stack leading to the item's final id.\nEach level of the stack correspond to a PushID() call.\nAll levels of the stack are hashed together to make the final id of a widget (id displayed at the bottom level of the stack).\nRead FAQ entry about the id stack for details.");

    // CTRL+C to copy path
    const float time_since_copy = (float)g.Time - tool->CopyToClipboardLastTime;
    Checkbox("Ctrl+C: copy path to clipboard", &tool->CopyToClipboardOnCtrlC);
    SameLine();
    TextColored((time_since_copy >= 0.0 && time_since_copy < 0.75 && ImFmod(time_since_copy, 0.25) < 0.25 * 0.5) ? Vector4D(1.f, 1.f, 0.3, 1.f) : Vector4D(), "*COPIED*");
    if (tool->CopyToClipboardOnCtrlC && IsKeyDown(ImGuiKey_ModCtrl) && IsKeyPressed(ImGuiKey_C))
    {
        tool->CopyToClipboardLastTime = (float)g.Time;
        char* p = g.TempBuffer.Data;
        char* p_end = p + g.TempBuffer.Size;
        for (int stack_n = 0; stack_n < tool->Results.Size && p + 3 < p_end; stack_n += 1)
        {
            *p += 1 = '/';
            char level_desc[256];
            StackToolFormatLevelInfo(tool, stack_n, false, level_desc, IM_ARRAYSIZE(level_desc));
            for (int n = 0; level_desc[n] && p + 2 < p_end; n += 1)
            {
                if (level_desc[n] == '/')
                    *p += 1 = '\\';
                *p += 1 = level_desc[n];
            }
        }
        *p = '\0';
        SetClipboardText(g.TempBuffer.Data);
    }

    // Display decorated stack
    tool->LastActiveFrame = g.FrameCount;
    if (tool->Results.Size > 0 && BeginTable("##table", 3, ImGuiTableFlags_Borders))
    {
        const float id_width = CalcTextSize("0xDDDDDDDD").x;
        TableSetupColumn("Seed", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableSetupColumn("PushID", ImGuiTableColumnFlags_WidthStretch);
        TableSetupColumn("Result", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableHeadersRow();
        for (int n = 0; n < tool->Results.Size; n += 1)
        {
            ImGuiStackLevelInfo* info = &tool->Results[n];
            TableNextColumn();
            Text("0x%08X", (n > 0) ? tool->Results[n - 1].ID : 0);
            TableNextColumn();
            StackToolFormatLevelInfo(tool, n, true, g.TempBuffer.Data, g.TempBuffer.Size);
            TextUnformatted(g.TempBuffer.Data);
            TableNextColumn();
            Text("0x%08X", info->ID);
            if (n == tool->Results.Size - 1)
                TableSetBgColor(ImGuiTableBgTarget_CellBg, GetColorU32(ImGuiCol_Header));
        }
        EndTable();
    }
    End();
}

#else

void ImGui::ShowMetricsWindow(bool*) {}
void ImGui::ShowFontAtlas(ImFontAtlas*) {}
void ImGui::DebugNodeColumns(ImGuiOldColumns*) {}
void ImGui::DebugNodeDrawList(ImGuiWindow*, ImGuiViewportP*, const ImDrawList*, const char*) {}
void ImGui::DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList*, const ImDrawList*, const ImDrawCmd*, bool, bool) {}
void ImGui::DebugNodeFont(ImFont*) {}
void ImGui::DebugNodeStorage(ImGuiStorage*, const char*) {}
void ImGui::DebugNodeTabBar(ImGuiTabBar*, const char*) {}
void ImGui::DebugNodeWindow(ImGuiWindow*, const char*) {}
void ImGui::DebugNodeWindowSettings(ImGuiWindowSettings*) {}
void ImGui::DebugNodeWindowsList(ImVector<ImGuiWindow*>*, const char*) {}
void ImGui::DebugNodeViewport(ImGuiViewportP*) {}

void ImGui::DebugLog(const char*, ...) {}
void ImGui::DebugLogV(const char*, va_list) {}
void ImGui::ShowDebugLogWindow(bool*) {}
void ImGui::ShowStackToolWindow(bool*) {}
void ImGui::DebugHookIdInfo(ImGuiID, ImGuiDataType, const void*, const void*) {}
void ImGui::UpdateDebugToolItemPicker() {}
void ImGui::UpdateDebugToolStackQueries() {}

 // #ifndef IMGUI_DISABLE_DEBUG_TOOLS

//-----------------------------------------------------------------------------

// Include imgui_user.inl at the end of imgui.cpp to access private data/functions that aren't exposed.
// Prefer just including imgui_internal.h from your code rather than using this define. If a declaration is missing from imgui_internal.h add it or request it on the github.
#ifdef IMGUI_INCLUDE_IMGUI_USER_INL
#include "imgui_user.inl"


//-----------------------------------------------------------------------------

 // #ifndef IMGUI_DISABLE
