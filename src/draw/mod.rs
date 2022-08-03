// dear imgui, v1.88
// (drawing and font code)


pub mod list;
pub mod command;
pub mod vertex;
pub mod list_shared_data;
pub mod list_splitter;
pub mod data;
pub mod bezier;
pub mod flags;
mod shade_verts;
mod channel;

/*

index of this file:

// [SECTION] STB libraries implementation
// [SECTION] style functions
// [SECTION] ImDrawList
// [SECTION] ImDrawListSplitter
// [SECTION] ImDrawData
// [SECTION] Helpers ShadeVertsXXX functions
// [SECTION] ImFontConfig
// [SECTION] ImFontAtlas
// [SECTION] ImFontAtlas glyph ranges helpers
// [SECTION] ImFontGlyphRangesBuilder
// [SECTION] ImFont
// [SECTION] ImGui Internal Render Helpers
// [SECTION] Decompression code
// [SECTION] Default font data (ProggyClean.ttf)

*/

// #if defined(_MSC_VER) && !defined(_CRT_SECURE_NO_WARNINGS)
// #define _CRT_SECURE_NO_WARNINGS
// #endif
//
// #include "defines.rs"
//
// #ifndef IMGUI_DISABLE
//
// #ifndef IMGUI_DEFINE_MATH_OPERATORS
// #define IMGUI_DEFINE_MATH_OPERATORS
// #endif
//
// #include "internal_h.rs"
// #ifdef IMGUI_ENABLE_FREETYPE
// #include "misc/freetype/imgui_freetype.h"
// #endif
//
// #include <stdio.h>      // vsnprintf, sscanf, printf
// #if !defined(alloca)
// #if defined(__GLIBC__) || defined(__sun) || defined(__APPLE__) || defined(__NEWLIB__)
// #include <alloca.h>     // alloca (glibc uses <alloca.h>. Note that Cygwin may have _WIN32 defined, so the order matters here)
// #elif defined(_WIN32)
// #include <malloc.h>     // alloca
// #if !defined(alloca)
// #define alloca _alloca  // for clang with MS Codegen
// #endif
// #else
// #include <stdlib.h>     // alloca
// #endif
// #endif
//
// // Visual Studio warnings
// #ifdef _MSC_VER
// #pragma warning (disable: 4127)     // condition expression is constant
// #pragma warning (disable: 4505)     // unreferenced local function has been removed (stb stuff)
// #pragma warning (disable: 4996)     // 'This function or variable may be unsafe': strcpy, strdup, sprintf, vsnprintf, sscanf, fopen
// #pragma warning (disable: 6255)     // [Static Analyzer] _alloca indicates failure by raising a stack overflow exception.  Consider using _malloca instead.
// #pragma warning (disable: 26451)    // [Static Analyzer] Arithmetic overflow : Using operator 'xxx' on a 4 byte value and then casting the result to a 8 byte value. Cast the value to the wider type before calling operator 'xxx' to avoid overflow(io.2).
// #pragma warning (disable: 26812)    // [Static Analyzer] The enum type 'xxx' is unscoped. Prefer 'enum class' over 'enum' (Enum.3). [MSVC Static Analyzer)
// #endif
//
// // Clang/GCC warnings with -Weverything
// #if defined(__clang__)
// #if __has_warning("-Wunknown-warning-option")
// #pragma clang diagnostic ignored "-Wunknown-warning-option"         // warning: unknown warning group 'xxx'                      // not all warnings are known by all Clang versions and they tend to be rename-happy.. so ignoring warnings triggers new warnings on some configuration. Great!
// #endif
// #if __has_warning("-Walloca")
// #pragma clang diagnostic ignored "-Walloca"                         // warning: use of function '__builtin_alloca' is discouraged
// #endif
// #pragma clang diagnostic ignored "-Wunknown-pragmas"                // warning: unknown warning group 'xxx'
// #pragma clang diagnostic ignored "-Wold-style-cast"                 // warning: use of old-style cast                            // yes, they are more terse.
// #pragma clang diagnostic ignored "-Wfloat-equal"                    // warning: comparing floating point with == or != is unsafe // storing and comparing against same constants ok.
// #pragma clang diagnostic ignored "-Wglobal-constructors"            // warning: declaration requires a global destructor         // similar to above, not sure what the exact difference is.
// #pragma clang diagnostic ignored "-Wsign-conversion"                // warning: implicit conversion changes signedness
// #pragma clang diagnostic ignored "-Wzero-as-null-pointer-constant"  // warning: zero as null pointer constant                    // some standard header variations use #define None 0
// #pragma clang diagnostic ignored "-Wcomma"                          // warning: possible misuse of comma operator here
// #pragma clang diagnostic ignored "-Wreserved-id-macro"              // warning: macro name is a reserved identifier
// #pragma clang diagnostic ignored "-Wdouble-promotion"               // warning: implicit conversion from 'float' to 'double' when passing argument to function  // using printf() is a misery with this as C++ va_arg ellipsis changes float to double.
// #pragma clang diagnostic ignored "-Wimplicit-int-float-conversion"  // warning: implicit conversion from 'xxx' to 'float' may lose precision
// #elif defined(__GNUC__)
// #pragma GCC diagnostic ignored "-Wpragmas"                  // warning: unknown option after '#pragma GCC diagnostic' kind
// #pragma GCC diagnostic ignored "-Wunused-function"          // warning: 'xxxx' defined but not used
// #pragma GCC diagnostic ignored "-Wdouble-promotion"         // warning: implicit conversion from 'float' to 'double' when passing argument to function
// #pragma GCC diagnostic ignored "-Wconversion"               // warning: conversion to 'xxxx' from 'xxxx' may alter its value
// #pragma GCC diagnostic ignored "-Wstack-protector"          // warning: stack protector not protecting local variables: variable length buffer
// #pragma GCC diagnostic ignored "-Wclass-memaccess"          // [__GNUC__ >= 8] warning: 'memset/memcpy' clearing/writing an object of type 'xxxx' with no trivial copy-assignment; use assignment or value-initialization instead
// #endif

//-------------------------------------------------------------------------
// [SECTION] STB libraries implementation (for stb_truetype and stb_rect_pack)
//-------------------------------------------------------------------------

// Compile time options:
//#define IMGUI_STB_NAMESPACE           ImStb
//#define IMGUI_STB_TRUETYPE_FILENAME   "my_folder/stb_truetype.h"
//#define IMGUI_STB_RECT_PACK_FILENAME  "my_folder/stb_rect_pack.h"
//#define IMGUI_DISABLE_STB_TRUETYPE_IMPLEMENTATION
//#define IMGUI_DISABLE_STB_RECT_PACK_IMPLEMENTATION
//
// #ifdef IMGUI_STB_NAMESPACE
// namespace IMGUI_STB_NAMESPACE
// {
// #endif
//
// #ifdef _MSC_VER
// #pragma warning (push)
// #pragma warning (disable: 4456)                             // declaration of 'xx' hides previous local declaration
// #pragma warning (disable: 6011)                             // (stb_rectpack) Dereferencing None pointer 'cur->next'.
// #pragma warning (disable: 6385)                             // (stb_truetype) Reading invalid data from 'buffer':  the readable size is '_Old_3`kernel_width' bytes, but '3' bytes may be read.
// #pragma warning (disable: 28182)                            // (stb_rectpack) Dereferencing None pointer. 'cur' contains the same None value as 'cur->next' did.
// #endif
//
// #if defined(__clang__)
// #pragma clang diagnostic push
// #pragma clang diagnostic ignored "-Wunused-function"
// #pragma clang diagnostic ignored "-Wmissing-prototypes"
// #pragma clang diagnostic ignored "-Wimplicit-fallthrough"
// #pragma clang diagnostic ignored "-Wcast-qual"              // warning: cast from 'const xxxx *' to 'xxx *' drops const qualifier
// #endif
//
// #if defined(__GNUC__)
// #pragma GCC diagnostic push
// #pragma GCC diagnostic ignored "-Wtype-limits"              // warning: comparison is always true due to limited range of data type [-Wtype-limits]
// #pragma GCC diagnostic ignored "-Wcast-qual"                // warning: cast from type 'const xxxx *' to type 'xxxx *' casts away qualifiers
// #endif
//
// #ifndef STB_RECT_PACK_IMPLEMENTATION                        // in case the user already have an implementation in the _same_ compilation unit (e.g. unity builds)
// #ifndef IMGUI_DISABLE_STB_RECT_PACK_IMPLEMENTATION          // in case the user already have an implementation in another compilation unit
// #define STBRP_STATIC
// #define STBRP_ASSERT(x)     do { IM_ASSERT(x); } while (0)
// #define STBRP_SORT          ImQsort
// #define STB_RECT_PACK_IMPLEMENTATION
// #endif
// #ifdef IMGUI_STB_RECT_PACK_FILENAME
// #include IMGUI_STB_RECT_PACK_FILENAME
// #else
// #include "stb_rectpack_h.rs"
//
// #endif
// #endif
//
// #ifdef  IMGUI_ENABLE_STB_TRUETYPE
// #ifndef STB_TRUETYPE_IMPLEMENTATION                         // in case the user already have an implementation in the _same_ compilation unit (e.g. unity builds)
// #ifndef IMGUI_DISABLE_STB_TRUETYPE_IMPLEMENTATION           // in case the user already have an implementation in another compilation unit
// #define STBTT_malloc(x,u)   ((void)(u), IM_ALLOC(x))
// #define STBTT_free(x,u)     ((void)(u), IM_FREE(x))
// #define STBTT_assert(x)     do { IM_ASSERT(x); } while(0)
// #define STBTT_fmod(x,y)     f32::mod(x,y)
// #define STBTT_sqrt(x)       ImSqrt(x)
// #define STBTT_pow(x,y)      ImPow(x,y)
// #define STBTT_fabs(x)       f32::abs(x)
// #define STBTT_ifloor(x)     (f32::floor(x))
// #define STBTT_iceil(x)      (ImCeil(x))
// #define STBTT_STATIC
// #define STB_TRUETYPE_IMPLEMENTATION
// #else
// #define STBTT_DEF extern
// #endif
// #ifdef IMGUI_STB_TRUETYPE_FILENAME
// #include IMGUI_STB_TRUETYPE_FILENAME
// #else
// #include "stb_truetype_h.rs"
//
// #endif
// #endif
// #endif // IMGUI_ENABLE_STB_TRUETYPE
//
// #if defined(__GNUC__)
// #pragma GCC diagnostic pop
// #endif
//
// #if defined(__clang__)
// #pragma clang diagnostic pop
// #endif
//
// #if defined(_MSC_VER)
// #pragma warning (pop)
// #endif
//
// #ifdef IMGUI_STB_NAMESPACE
// } // namespace ImStb
// using namespace IMGUI_STB_NAMESPACE;
// #endif

//-----------------------------------------------------------------------------
// [SECTION] style functions
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] ImDrawList
//-----------------------------------------------------------------------------

// ImDrawListSharedData::ImDrawListSharedData()
// {
//     memset(this, 0, sizeof(*this));
//     for (int i = 0; i < IM_ARRAYSIZE(arc_fast_vtx); i += 1)
//     {
//         let a = ((float)i * 2 * f32::PI) / (float)IM_ARRAYSIZE(arc_fast_vtx);
//         arc_fast_vtx[i] = Vector2D::new(ImCos(a), ImSin(a));
//     }
//     arc_fast_radius_cutoff = IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC_R(IM_DRAWLIST_ARCFAST_SAMPLE_MAX, CircleSegmentMaxError);
// }

// void ImDrawListSharedData::SetCircleTessellationMaxError(float max_error)
// {
//     if (CircleSegmentMaxError == max_error)
//         return;
//
//     IM_ASSERT(max_error > 0.0);
//     CircleSegmentMaxError = max_error;
//     for (int i = 0; i < IM_ARRAYSIZE(CircleSegmentCounts); i += 1)
//     {
//         let radius = (float)i;
//         CircleSegmentCounts[i] = (ImU8)((i > 0) ? IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC(radius, CircleSegmentMaxError) : IM_DRAWLIST_ARCFAST_SAMPLE_MAX);
//     }
//     arc_fast_radius_cutoff = IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC_R(IM_DRAWLIST_ARCFAST_SAMPLE_MAX, CircleSegmentMaxError);
// }

// void ImDrawList::_ClearFreeMemory()
// {
//     CmdBuffer.clear();
//     IdxBuffer.clear();
//     VtxBuffer.clear();
//     Flags = DrawListFlags::None;
//     _VtxCurrentIdx = 0;
//     _VtxWritePtr = None;
//     _IdxWritePtr = None;
//     _ClipRectStack.clear();
//     _TextureIdStack.clear();
//     _Path.clear();
//     _Splitter.ClearFreeMemory();
// }

// ImDrawList* ImDrawList::clone_output() const
// {
//     ImDrawList* dst = IM_NEW(ImDrawList(_Data));
//     dst.cmd_buffer = CmdBuffer;
//     dst.IdxBuffer = IdxBuffer;
//     dst.VtxBuffer = VtxBuffer;
//     dst.flags = Flags;
//     return dst;
// }

// void ImDrawList::AddDrawCmd()
// {
//     ImDrawCmd draw_cmd;
//     draw_cmd.clip_rect = _CmdHeader.clip_rect;    // Same as calling ImDrawCmd_HeaderCopy()
//     draw_cmd.TextureId = _CmdHeader.TextureId;
//     draw_cmd.VtxOffset = _CmdHeader.VtxOffset;
//     draw_cmd.IdxOffset = IdxBuffer.size;
//
//     // IM_ASSERT(draw_cmd.clip_rect.x <= draw_cmd.clip_rect.z && draw_cmd.clip_rect.y <= draw_cmd.clip_rect.w);
//     CmdBuffer.push_back(draw_cmd);
// }

// Pop trailing draw command (used before merging or presenting to user)
// Note that this leaves the ImDrawList in a state unfit for further commands, as most code assume that cmd_buffer.size > 0 && cmd_buffer.back().user_callback == None
// void ImDrawList::_PopUnusedDrawCmd()
// {
//     if (CmdBuffer.size == 0)
//         return;
//     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
//     if (curr_cmd.ElemCount == 0 && curr_cmd.UserCallback == None)
//         CmdBuffer.pop_back();
// }

// void ImDrawList::add_callback(ImDrawCallback callback, void* callback_data)
// {
//     // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
//     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
//     // IM_ASSERT(curr_cmd.UserCallback == None);
//     if (curr_cmd.ElemCount != 0)
//     {
//         AddDrawCmd();
//         curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
//     }
//     curr_cmd.UserCallback = callback;
//     curr_cmd.UserCallbackData = callback_data;
//
//     AddDrawCmd(); // Force a new command after us (see comment below)
// }

// Compare clip_rect, texture_id and vtx_offset with a single memcmp()
#define ImDrawCmd_HeaderSize                            (IM_OFFSETOF(ImDrawCmd, vtx_offset) + sizeof(unsigned int))
#define ImDrawCmd_HeaderCompare(CMD_LHS, CMD_RHS)       (memcmp(CMD_LHS, CMD_RHS, ImDrawCmd_HeaderSize))    // Compare clip_rect, texture_id, vtx_offset
#define ImDrawCmd_HeaderCopy(CMD_DST, CMD_SRC)          (memcpy(CMD_DST, CMD_SRC, ImDrawCmd_HeaderSize))    // Copy clip_rect, texture_id, vtx_offset
#define ImDrawCmd_AreSequentialIdxOffset(CMD_0, CMD_1)  (CMD_0.idx_offset + CMD_0.elem_count == CMD_1.idx_offset)

// Try to merge two last draw commands
// void ImDrawList::_TryMergeDrawCmds()
// {
//     // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
//     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
//     ImDrawCmd* prev_cmd = curr_cmd - 1;
//     if (ImDrawCmd_HeaderCompare(curr_cmd, prev_cmd) == 0 && ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) && curr_cmd.UserCallback == None && prev_cmd.UserCallback == None)
//     {
//         prev_cmd.ElemCount += curr_cmd.ElemCount;
//         CmdBuffer.pop_back();
//     }
// }

// Our scheme may appears a bit unusual, basically we want the most-common calls add_line add_rect etc. to not have to perform any check so we always have a command ready in the stack.
// The cost of figuring out if a new command has to be added or if we can merge is paid in those Update** functions only.
// void ImDrawList::_OnChangedClipRect()
// {
//     // If current command is used with different settings we need to add a new command
//     // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
//     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
//     if (curr_cmd.ElemCount != 0 && memcmp(&curr_cmd.clip_rect, &_CmdHeader.clip_rect, sizeof(Vector4D)) != 0)
//     {
//         AddDrawCmd();
//         return;
//     }
//     // IM_ASSERT(curr_cmd.UserCallback == None);
//
//     // Try to merge with previous command if it matches, else use current command
//     ImDrawCmd* prev_cmd = curr_cmd - 1;
//     if (curr_cmd.ElemCount == 0 && CmdBuffer.size > 1 && ImDrawCmd_HeaderCompare(&_CmdHeader, prev_cmd) == 0 && ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) && prev_cmd.UserCallback == None)
//     {
//         CmdBuffer.pop_back();
//         return;
//     }
//
//     curr_cmd.clip_rect = _CmdHeader.clip_rect;
// }

// void ImDrawList::_OnChangedTextureID()
// {
//     // If current command is used with different settings we need to add a new command
//     // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
//     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
//     if (curr_cmd.ElemCount != 0 && curr_cmd.TextureId != _CmdHeader.TextureId)
//     {
//         AddDrawCmd();
//         return;
//     }
//     // IM_ASSERT(curr_cmd.UserCallback == None);
//
//     // Try to merge with previous command if it matches, else use current command
//     ImDrawCmd* prev_cmd = curr_cmd - 1;
//     if (curr_cmd.ElemCount == 0 && CmdBuffer.size > 1 && ImDrawCmd_HeaderCompare(&_CmdHeader, prev_cmd) == 0 && ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) && prev_cmd.UserCallback == None)
//     {
//         CmdBuffer.pop_back();
//         return;
//     }
//
//     curr_cmd.TextureId = _CmdHeader.TextureId;
// }

// void ImDrawList::_OnChangedVtxOffset()
// {
//     // We don't need to compare curr_cmd->vtx_offset != _cmd_header.vtx_offset because we know it'll be different at the time we call this.
//     _VtxCurrentIdx = 0;
//     // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
//     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
//     //IM_ASSERT(curr_cmd->vtx_offset != _cmd_header.vtx_offset); // See #3349
//     if (curr_cmd.ElemCount != 0)
//     {
//         AddDrawCmd();
//         return;
//     }
//     // IM_ASSERT(curr_cmd.UserCallback == None);
//     curr_cmd.VtxOffset = _CmdHeader.VtxOffset;
// }
//
// int ImDrawList::_CalcCircleAutoSegmentCount(float radius) const
// {
// use alloc::vec::Vec;
// use imgui_rs::defines::Viewport;
// use imgui_rs::draw_cmd::DimgDrawCmd;
// use imgui_rs::draw_list::DimgDrawList;
// use imgui_rs::vec_nd::Vector2D;
// use std::collections::hash::set::HashSet;
// use draw_defines::DrawFlags;
// use draw_list::DrawList;
// use crate::types::Id32;
// use crate::viewport::Viewport;
//

//
// // Automatic segment count
//     let radius_idx = (radius + 0.999999); // ceil to never reduce accuracy
//     if (radius_idx < IM_ARRAYSIZE(_Data.CircleSegmentCounts))
//         return _Data.CircleSegmentCounts[radius_idx]; // Use cached value
//     else
//         return IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC(radius, _Data.CircleSegmentMaxError);
// }

// Render-level scissoring. This is passed down to your render function but not used for CPU-side coarse clipping. Prefer using higher-level ImGui::push_clip_rect() to affect logic (hit-testing and widget culling)
// void ImDrawList::PushClipRect(const Vector2D& cr_min, const Vector2D& cr_max, bool intersect_with_current_clip_rect)
// {
//     Vector4D cr(cr_min.x, cr_min.y, cr_max.x, cr_max.y);
//     if (intersect_with_current_clip_rect)
//     {
//         Vector4D current = _CmdHeader.clip_rect;
//         if (cr.x < current.x) cr.x = current.x;
//         if (cr.y < current.y) cr.y = current.y;
//         if (cr.z > current.z) cr.z = current.z;
//         if (cr.w > current.w) cr.w = current.w;
//     }
//     cr.z = ImMax(cr.x, cr.z);
//     cr.w = ImMax(cr.y, cr.w);
//
//     _ClipRectStack.push_back(cr);
//     _CmdHeader.clip_rect = cr;
//     _OnChangedClipRect();
// }

// void ImDrawList::PushClipRectFullScreen()
// {
//     PushClipRect(Vector2D::new(_Data.clip_rect_full_screen.x, _Data.clip_rect_full_screen.y), Vector2D::new(_Data.clip_rect_full_screen.z, _Data.clip_rect_full_screen.w));
// }

// void ImDrawList::PopClipRect()
// {
//     _ClipRectStack.pop_back();
//     _CmdHeader.clip_rect = (_ClipRectStack.size == 0) ? _Data.clip_rect_full_screen : _ClipRectStack.data[_ClipRectStack.size - 1];
//     _OnChangedClipRect();
// }

// void ImDrawList::PushTextureID(ImTextureID texture_id)
// {
//     _TextureIdStack.push_back(texture_id);
//     _CmdHeader.TextureId = texture_id;
//     _OnChangedTextureID();
// }

// void ImDrawList::pop_texture_id()
// {
//     _TextureIdStack.pop_back();
//     _CmdHeader.TextureId = (_TextureIdStack.size == 0) ? (ImTextureID)None : _TextureIdStack.data[_TextureIdStack.size - 1];
//     _OnChangedTextureID();
// }

// Reserve space for a number of vertices and indices.
// You must finish filling your reserved data before calling PrimReserve() again, as it may reallocate or
// submit the intermediate results. PrimUnreserve() can be used to release unused allocations.
// void ImDrawList::PrimReserve(int idx_count, int vtx_count)
// {
//     // Large mesh support (when enabled)
//     // IM_ASSERT_PARANOID(idx_count >= 0 && vtx_count >= 0);
//     if (sizeof(ImDrawIdx) == 2 && (_VtxCurrentIdx + vtx_count >= (1 << 16)) && (Flags & DrawListFlags::AllowVtxOffset))
//     {
//         // FIXME: In theory we should be testing that vtx_count <64k here.
//         // In practice, render_text() relies on reserving ahead for a worst case scenario so it is currently useful for us
//         // to not make that check until we rework the text functions to handle clipping and large horizontal lines better.
//         _CmdHeader.VtxOffset = VtxBuffer.size;
//         _OnChangedVtxOffset();
//     }
//
//     ImDrawCmd* draw_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
//     draw_cmd.ElemCount += idx_count;
//
//     int vtx_buffer_old_size = VtxBuffer.size;
//     VtxBuffer.resize(vtx_buffer_old_size + vtx_count);
//     _VtxWritePtr = VtxBuffer.data + vtx_buffer_old_size;
//
//     int idx_buffer_old_size = IdxBuffer.size;
//     IdxBuffer.resize(idx_buffer_old_size + idx_count);
//     _IdxWritePtr = IdxBuffer.data + idx_buffer_old_size;
// }

// Release the a number of reserved vertices/indices from the end of the last reservation made with PrimReserve().
// void ImDrawList::PrimUnreserve(int idx_count, int vtx_count)
// {
//     // IM_ASSERT_PARANOID(idx_count >= 0 && vtx_count >= 0);
//
//     ImDrawCmd* draw_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
//     draw_cmd.ElemCount -= idx_count;
//     VtxBuffer.shrink(VtxBuffer.size - vtx_count);
//     IdxBuffer.shrink(IdxBuffer.size - idx_count);
// }

// Fully unrolled with inline call to keep our debug builds decently fast.
// void ImDrawList::PrimRect(const Vector2D& a, const Vector2D& c, ImU32 col)
// {
//     Vector2D b(c.x, a.y), d(a.x, c.y), uv(_Data.TexUvWhitePixel);
//     ImDrawIdx idx = (ImDrawIdx)_VtxCurrentIdx;
//     _IdxWritePtr[0] = idx; _IdxWritePtr[1] = (ImDrawIdx)(idx+1); _IdxWritePtr[2] = (ImDrawIdx)(idx+2);
//     _IdxWritePtr[3] = idx; _IdxWritePtr[4] = (ImDrawIdx)(idx+2); _IdxWritePtr[5] = (ImDrawIdx)(idx+3);
//     _VtxWritePtr[0].pos = a; _VtxWritePtr[0].uv = uv; _VtxWritePtr[0].col = col;
//     _VtxWritePtr[1].pos = b; _VtxWritePtr[1].uv = uv; _VtxWritePtr[1].col = col;
//     _VtxWritePtr[2].pos = c; _VtxWritePtr[2].uv = uv; _VtxWritePtr[2].col = col;
//     _VtxWritePtr[3].pos = d; _VtxWritePtr[3].uv = uv; _VtxWritePtr[3].col = col;
//     _VtxWritePtr += 4;
//     _VtxCurrentIdx += 4;
//     _IdxWritePtr += 6;
// }
//
// void ImDrawList::prim_rect_uv(const Vector2D& a, const Vector2D& c, const Vector2D& uv_a, const Vector2D& uv_c, ImU32 col)
// {
//     Vector2D b(c.x, a.y), d(a.x, c.y), uv_b(uv_c.x, uv_a.y), uv_d(uv_a.x, uv_c.y);
//     ImDrawIdx idx = (ImDrawIdx)_VtxCurrentIdx;
//     _IdxWritePtr[0] = idx; _IdxWritePtr[1] = (ImDrawIdx)(idx+1); _IdxWritePtr[2] = (ImDrawIdx)(idx+2);
//     _IdxWritePtr[3] = idx; _IdxWritePtr[4] = (ImDrawIdx)(idx+2); _IdxWritePtr[5] = (ImDrawIdx)(idx+3);
//     _VtxWritePtr[0].pos = a; _VtxWritePtr[0].uv = uv_a; _VtxWritePtr[0].col = col;
//     _VtxWritePtr[1].pos = b; _VtxWritePtr[1].uv = uv_b; _VtxWritePtr[1].col = col;
//     _VtxWritePtr[2].pos = c; _VtxWritePtr[2].uv = uv_c; _VtxWritePtr[2].col = col;
//     _VtxWritePtr[3].pos = d; _VtxWritePtr[3].uv = uv_d; _VtxWritePtr[3].col = col;
//     _VtxWritePtr += 4;
//     _VtxCurrentIdx += 4;
//     _IdxWritePtr += 6;
// }

// void ImDrawList::prim_quad_uv(const Vector2D& a, const Vector2D& b, const Vector2D& c, const Vector2D& d, const Vector2D& uv_a, const Vector2D& uv_b, const Vector2D& uv_c, const Vector2D& uv_d, ImU32 col)
// {
//     ImDrawIdx idx = (ImDrawIdx)_VtxCurrentIdx;
//     _IdxWritePtr[0] = idx; _IdxWritePtr[1] = (ImDrawIdx)(idx+1); _IdxWritePtr[2] = (ImDrawIdx)(idx+2);
//     _IdxWritePtr[3] = idx; _IdxWritePtr[4] = (ImDrawIdx)(idx+2); _IdxWritePtr[5] = (ImDrawIdx)(idx+3);
//     _VtxWritePtr[0].pos = a; _VtxWritePtr[0].uv = uv_a; _VtxWritePtr[0].col = col;
//     _VtxWritePtr[1].pos = b; _VtxWritePtr[1].uv = uv_b; _VtxWritePtr[1].col = col;
//     _VtxWritePtr[2].pos = c; _VtxWritePtr[2].uv = uv_c; _VtxWritePtr[2].col = col;
//     _VtxWritePtr[3].pos = d; _VtxWritePtr[3].uv = uv_d; _VtxWritePtr[3].col = col;
//     _VtxWritePtr += 4;
//     _VtxCurrentIdx += 4;
//     _IdxWritePtr += 6;
// }

// On add_polyline() and add_convex_poly_filled() we intentionally avoid using Vector2D and superfluous function calls to optimize debug/non-inlined builds.
// - Those macros expects l-values and need to be used as their own statement.
// - Those macros are intentionally not surrounded by the 'do {} while (0)' idiom because even that translates to runtime with debug compilers.
#define normalize_2f_over_zero(VX,VY)     { let d2 =  VX*VX + VY*VY; if (d2 > 0.0) { let inv_len =  ImRsqrt(d2); VX *= inv_len; VY *= inv_len; } } (void)0
#define IM_FIXNORMAL2F_MAX_INVLEN2          100.0 // 500.0 (see #4053, #3366)
#define fix_normal_2f(VX,VY)               { let d2 =  VX*VX + VY*VY; if (d2 > 0.000001) { let inv_len2 =  1.0 / d2; if (inv_len2 > IM_FIXNORMAL2F_MAX_INVLEN2) inv_len2 = IM_FIXNORMAL2F_MAX_INVLEN2; VX *= inv_len2; VY *= inv_len2; } } (void)0

// TODO: Thickness anti-aliased lines cap are missing their AA fringe.
// We avoid using the Vector2D math operators here to reduce cost to a minimum for debug/non-inlined builds.
// void ImDrawList::add_polyline(const Vector2D* points, let points_count, ImU32 col, ImDrawFlags flags, float thickness)
// {
//     if (points_count < 2)
//         return;
//
//     const bool closed = (flags & DrawFlags::Closed) != 0;
//     const Vector2D opaque_uv = _Data.TexUvWhitePixel;
//     let count = closed ? points_count : points_count - 1; // The number of line segments we need to draw
//     const bool thick_line = (thickness > _FringeScale);
//
//     if (Flags & DrawListFlags::AntiAliasedLines)
//     {
//         // Anti-aliased stroke
//         let AA_SIZE = _FringeScale;
//         const ImU32 col_trans = col & ~IM_COL32_A_MASK;
//
//         // Thicknesses <1.0 should behave like thickness 1.0
//         thickness = ImMax(thickness, 1.0);
//         let integer_thickness = thickness;
//         let fractional_thickness = thickness - integer_thickness;
//
//         // Do we want to draw this line using a texture?
//         // - For now, only draw integer-width lines using textures to avoid issues with the way scaling occurs, could be improved.
//         // - If AA_SIZE is not 1.0 we cannot use the texture path.
//         const bool use_texture = (Flags & DrawListFlags::AntiAliasedLinesUseTex) && (integer_thickness < IM_DRAWLIST_TEX_LINES_WIDTH_MAX) && (fractional_thickness <= 0.00001) && (AA_SIZE == 1.0);
//
//         // We should never hit this, because NewFrame() doesn't set ImDrawListFlags_AntiAliasedLinesUseTex unless ImFontAtlasFlags_NoBakedLines is off
//         // IM_ASSERT_PARANOID(!use_texture || !(_Data.Font.container_atlas.flags & FontAtlasFlags::NoBakedLines));
//
//         let idx_count = use_texture ? (count * 6) : (thick_line ? count * 18 : count * 12);
//         let vtx_count = use_texture ? (points_count * 2) : (thick_line ? points_count * 4 : points_count * 3);
//         PrimReserve(idx_count, vtx_count);
//
//         // Temporary buffer
//         // The first <points_count> items are normals at each line point, then after that there are either 2 or 4 temp points for each line point
//         Vector2D* temp_normals = alloca(points_count * ((use_texture || !thick_line) ? 3 : 5) * sizeof(Vector2D)); //-V630
//         Vector2D* temp_points = temp_normals + points_count;
//
//         // Calculate normals (tangents) for each line segment
//         for (int i1 = 0; i1 < count; i1 += 1)
//         {
//             let i2 = (i1 + 1) == points_count ? 0 : i1 + 1;
//             float dx = points[i2].x - points[i1].x;
//             float dy = points[i2].y - points[i1].y;
//             IM_NORMALIZE2F_OVER_ZERO(dx, dy);
//             temp_normals[i1].x = dy;
//             temp_normals[i1].y = -dx;
//         }
//         if (!closed)
//             temp_normals[points_count - 1] = temp_normals[points_count - 2];
//
//         // If we are drawing a one-pixel-wide line without a texture, or a textured line of any width, we only need 2 or 3 vertices per point
//         if (use_texture || !thick_line)
//         {
//             // [PATH 1] Texture-based lines (thick or non-thick)
//             // [PATH 2] Non texture-based lines (non-thick)
//
//             // The width of the geometry we need to draw - this is essentially <thickness> pixels for the line itself, plus "one pixel" for AA.
//             // - In the texture-based path, we don't use AA_SIZE here because the +1 is tied to the generated texture
//             //   (see ImFontAtlasBuildRenderLinesTexData() function), and so alternate values won't work without changes to that code.
//             // - In the non texture-based paths, we would allow AA_SIZE to potentially be != 1.0 with a patch (e.g. fringe_scale patch to
//             //   allow scaling geometry while preserving one-screen-pixel AA fringe).
//             let half_draw_size = use_texture ? ((thickness * 0.5) + 1) : AA_SIZE;
//
//             // If line is not closed, the first and last points need to be generated differently as there are no normals to blend
//             if (!closed)
//             {
//                 temp_points[0] = points[0] + temp_normals[0] * half_draw_size;
//                 temp_points[1] = points[0] - temp_normals[0] * half_draw_size;
//                 temp_points[(points_count-1)*2+0] = points[points_count-1] + temp_normals[points_count-1] * half_draw_size;
//                 temp_points[(points_count-1)*2+1] = points[points_count-1] - temp_normals[points_count-1] * half_draw_size;
//             }
//
//             // Generate the indices to form a number of triangles for each line segment, and the vertices for the line edges
//             // This takes points n and n+1 and writes into n+1, with the first point in a closed line being generated from the final one (as n+1 wraps)
//             // FIXME-OPT: merge the different loops, possibly remove the temporary buffer.
//             unsigned int idx1 = _VtxCurrentIdx; // Vertex index for start of line segment
//             for (int i1 = 0; i1 < count; i1 += 1) // i1 is the first point of the line segment
//             {
//                 let i2 = (i1 + 1) == points_count ? 0 : i1 + 1; // i2 is the second point of the line segment
//                 const unsigned int idx2 = ((i1 + 1) == points_count) ? _VtxCurrentIdx : (idx1 + (use_texture ? 2 : 3)); // Vertex index for end of segment
//
//                 // Average normals
//                 float dm_x = (temp_normals[i1].x + temp_normals[i2].x) * 0.5;
//                 float dm_y = (temp_normals[i1].y + temp_normals[i2].y) * 0.5;
//                 IM_FIXNORMAL2F(dm_x, dm_y);
//                 dm_x *= half_draw_size; // dm_x, dm_y are offset to the outer edge of the AA area
//                 dm_y *= half_draw_size;
//
//                 // Add temporary vertexes for the outer edges
//                 Vector2D* out_vtx = &temp_points[i2 * 2];
//                 out_vtx[0].x = points[i2].x + dm_x;
//                 out_vtx[0].y = points[i2].y + dm_y;
//                 out_vtx[1].x = points[i2].x - dm_x;
//                 out_vtx[1].y = points[i2].y - dm_y;
//
//                 if (use_texture)
//                 {
//                     // Add indices for two triangles
//                     _IdxWritePtr[0] = (ImDrawIdx)(idx2 + 0); _IdxWritePtr[1] = (ImDrawIdx)(idx1 + 0); _IdxWritePtr[2] = (ImDrawIdx)(idx1 + 1); // Right tri
//                     _IdxWritePtr[3] = (ImDrawIdx)(idx2 + 1); _IdxWritePtr[4] = (ImDrawIdx)(idx1 + 1); _IdxWritePtr[5] = (ImDrawIdx)(idx2 + 0); // Left tri
//                     _IdxWritePtr += 6;
//                 }
//                 else
//                 {
//                     // Add indexes for four triangles
//                     _IdxWritePtr[0] = (ImDrawIdx)(idx2 + 0); _IdxWritePtr[1] = (ImDrawIdx)(idx1 + 0); _IdxWritePtr[2] = (ImDrawIdx)(idx1 + 2); // Right tri 1
//                     _IdxWritePtr[3] = (ImDrawIdx)(idx1 + 2); _IdxWritePtr[4] = (ImDrawIdx)(idx2 + 2); _IdxWritePtr[5] = (ImDrawIdx)(idx2 + 0); // Right tri 2
//                     _IdxWritePtr[6] = (ImDrawIdx)(idx2 + 1); _IdxWritePtr[7] = (ImDrawIdx)(idx1 + 1); _IdxWritePtr[8] = (ImDrawIdx)(idx1 + 0); // Left tri 1
//                     _IdxWritePtr[9] = (ImDrawIdx)(idx1 + 0); _IdxWritePtr[10] = (ImDrawIdx)(idx2 + 0); _IdxWritePtr[11] = (ImDrawIdx)(idx2 + 1); // Left tri 2
//                     _IdxWritePtr += 12;
//                 }
//
//                 idx1 = idx2;
//             }
//
//             // Add vertexes for each point on the line
//             if (use_texture)
//             {
//                 // If we're using textures we only need to emit the left/right edge vertices
//                 Vector4D tex_uvs = _Data.TexUvLines[integer_thickness];
//                 /*if (fractional_thickness != 0.0) // Currently always zero when use_texture==false!
//                 {
//                     const Vector4D tex_uvs_1 = _Data->tex_uv_lines[integer_thickness + 1];
//                     tex_uvs.x = tex_uvs.x + (tex_uvs_1.x - tex_uvs.x) * fractional_thickness; // inlined ImLerp()
//                     tex_uvs.y = tex_uvs.y + (tex_uvs_1.y - tex_uvs.y) * fractional_thickness;
//                     tex_uvs.z = tex_uvs.z + (tex_uvs_1.z - tex_uvs.z) * fractional_thickness;
//                     tex_uvs.w = tex_uvs.w + (tex_uvs_1.w - tex_uvs.w) * fractional_thickness;
//                 }*/
//                 Vector2D tex_uv0(tex_uvs.x, tex_uvs.y);
//                 Vector2D tex_uv1(tex_uvs.z, tex_uvs.w);
//                 for (int i = 0; i < points_count; i += 1)
//                 {
//                     _VtxWritePtr[0].pos = temp_points[i * 2 + 0]; _VtxWritePtr[0].uv = tex_uv0; _VtxWritePtr[0].col = col; // Left-side outer edge
//                     _VtxWritePtr[1].pos = temp_points[i * 2 + 1]; _VtxWritePtr[1].uv = tex_uv1; _VtxWritePtr[1].col = col; // Right-side outer edge
//                     _VtxWritePtr += 2;
//                 }
//             }
//             else
//             {
//                 // If we're not using a texture, we need the center vertex as well
//                 for (int i = 0; i < points_count; i += 1)
//                 {
//                     _VtxWritePtr[0].pos = points[i];              _VtxWritePtr[0].uv = opaque_uv; _VtxWritePtr[0].col = col;       // Center of line
//                     _VtxWritePtr[1].pos = temp_points[i * 2 + 0]; _VtxWritePtr[1].uv = opaque_uv; _VtxWritePtr[1].col = col_trans; // Left-side outer edge
//                     _VtxWritePtr[2].pos = temp_points[i * 2 + 1]; _VtxWritePtr[2].uv = opaque_uv; _VtxWritePtr[2].col = col_trans; // Right-side outer edge
//                     _VtxWritePtr += 3;
//                 }
//             }
//         }
//         else
//         {
//             // [PATH 2] Non texture-based lines (thick): we need to draw the solid line core and thus require four vertices per point
//             let half_inner_thickness = (thickness - AA_SIZE) * 0.5;
//
//             // If line is not closed, the first and last points need to be generated differently as there are no normals to blend
//             if (!closed)
//             {
//                 let points_last = points_count - 1;
//                 temp_points[0] = points[0] + temp_normals[0] * (half_inner_thickness + AA_SIZE);
//                 temp_points[1] = points[0] + temp_normals[0] * (half_inner_thickness);
//                 temp_points[2] = points[0] - temp_normals[0] * (half_inner_thickness);
//                 temp_points[3] = points[0] - temp_normals[0] * (half_inner_thickness + AA_SIZE);
//                 temp_points[points_last * 4 + 0] = points[points_last] + temp_normals[points_last] * (half_inner_thickness + AA_SIZE);
//                 temp_points[points_last * 4 + 1] = points[points_last] + temp_normals[points_last] * (half_inner_thickness);
//                 temp_points[points_last * 4 + 2] = points[points_last] - temp_normals[points_last] * (half_inner_thickness);
//                 temp_points[points_last * 4 + 3] = points[points_last] - temp_normals[points_last] * (half_inner_thickness + AA_SIZE);
//             }
//
//             // Generate the indices to form a number of triangles for each line segment, and the vertices for the line edges
//             // This takes points n and n+1 and writes into n+1, with the first point in a closed line being generated from the final one (as n+1 wraps)
//             // FIXME-OPT: merge the different loops, possibly remove the temporary buffer.
//             unsigned int idx1 = _VtxCurrentIdx; // Vertex index for start of line segment
//             for (int i1 = 0; i1 < count; i1 += 1) // i1 is the first point of the line segment
//             {
//                 let i2 = (i1 + 1) == points_count ? 0 : (i1 + 1); // i2 is the second point of the line segment
//                 const unsigned int idx2 = (i1 + 1) == points_count ? _VtxCurrentIdx : (idx1 + 4); // Vertex index for end of segment
//
//                 // Average normals
//                 float dm_x = (temp_normals[i1].x + temp_normals[i2].x) * 0.5;
//                 float dm_y = (temp_normals[i1].y + temp_normals[i2].y) * 0.5;
//                 IM_FIXNORMAL2F(dm_x, dm_y);
//                 float dm_out_x = dm_x * (half_inner_thickness + AA_SIZE);
//                 float dm_out_y = dm_y * (half_inner_thickness + AA_SIZE);
//                 float dm_in_x = dm_x * half_inner_thickness;
//                 float dm_in_y = dm_y * half_inner_thickness;
//
//                 // Add temporary vertices
//                 Vector2D* out_vtx = &temp_points[i2 * 4];
//                 out_vtx[0].x = points[i2].x + dm_out_x;
//                 out_vtx[0].y = points[i2].y + dm_out_y;
//                 out_vtx[1].x = points[i2].x + dm_in_x;
//                 out_vtx[1].y = points[i2].y + dm_in_y;
//                 out_vtx[2].x = points[i2].x - dm_in_x;
//                 out_vtx[2].y = points[i2].y - dm_in_y;
//                 out_vtx[3].x = points[i2].x - dm_out_x;
//                 out_vtx[3].y = points[i2].y - dm_out_y;
//
//                 // Add indexes
//                 _IdxWritePtr[0]  = (ImDrawIdx)(idx2 + 1); _IdxWritePtr[1]  = (ImDrawIdx)(idx1 + 1); _IdxWritePtr[2]  = (ImDrawIdx)(idx1 + 2);
//                 _IdxWritePtr[3]  = (ImDrawIdx)(idx1 + 2); _IdxWritePtr[4]  = (ImDrawIdx)(idx2 + 2); _IdxWritePtr[5]  = (ImDrawIdx)(idx2 + 1);
//                 _IdxWritePtr[6]  = (ImDrawIdx)(idx2 + 1); _IdxWritePtr[7]  = (ImDrawIdx)(idx1 + 1); _IdxWritePtr[8]  = (ImDrawIdx)(idx1 + 0);
//                 _IdxWritePtr[9]  = (ImDrawIdx)(idx1 + 0); _IdxWritePtr[10] = (ImDrawIdx)(idx2 + 0); _IdxWritePtr[11] = (ImDrawIdx)(idx2 + 1);
//                 _IdxWritePtr[12] = (ImDrawIdx)(idx2 + 2); _IdxWritePtr[13] = (ImDrawIdx)(idx1 + 2); _IdxWritePtr[14] = (ImDrawIdx)(idx1 + 3);
//                 _IdxWritePtr[15] = (ImDrawIdx)(idx1 + 3); _IdxWritePtr[16] = (ImDrawIdx)(idx2 + 3); _IdxWritePtr[17] = (ImDrawIdx)(idx2 + 2);
//                 _IdxWritePtr += 18;
//
//                 idx1 = idx2;
//             }
//
//             // Add vertices
//             for (int i = 0; i < points_count; i += 1)
//             {
//                 _VtxWritePtr[0].pos = temp_points[i * 4 + 0]; _VtxWritePtr[0].uv = opaque_uv; _VtxWritePtr[0].col = col_trans;
//                 _VtxWritePtr[1].pos = temp_points[i * 4 + 1]; _VtxWritePtr[1].uv = opaque_uv; _VtxWritePtr[1].col = col;
//                 _VtxWritePtr[2].pos = temp_points[i * 4 + 2]; _VtxWritePtr[2].uv = opaque_uv; _VtxWritePtr[2].col = col;
//                 _VtxWritePtr[3].pos = temp_points[i * 4 + 3]; _VtxWritePtr[3].uv = opaque_uv; _VtxWritePtr[3].col = col_trans;
//                 _VtxWritePtr += 4;
//             }
//         }
//         _VtxCurrentIdx += (ImDrawIdx)vtx_count;
//     }
//     else
//     {
//         // [PATH 4] Non texture-based, Non anti-aliased lines
//         let idx_count = count * 6;
//         let vtx_count = count * 4;    // FIXME-OPT: Not sharing edges
//         PrimReserve(idx_count, vtx_count);
//
//         for (int i1 = 0; i1 < count; i1 += 1)
//         {
//             let i2 = (i1 + 1) == points_count ? 0 : i1 + 1;
//             const Vector2D& p1 = points[i1];
//             const Vector2D& p2 = points[i2];
//
//             float dx = p2.x - p1.x;
//             float dy = p2.y - p1.y;
//             IM_NORMALIZE2F_OVER_ZERO(dx, dy);
//             dx *= (thickness * 0.5);
//             dy *= (thickness * 0.5);
//
//             _VtxWritePtr[0].pos.x = p1.x + dy; _VtxWritePtr[0].pos.y = p1.y - dx; _VtxWritePtr[0].uv = opaque_uv; _VtxWritePtr[0].col = col;
//             _VtxWritePtr[1].pos.x = p2.x + dy; _VtxWritePtr[1].pos.y = p2.y - dx; _VtxWritePtr[1].uv = opaque_uv; _VtxWritePtr[1].col = col;
//             _VtxWritePtr[2].pos.x = p2.x - dy; _VtxWritePtr[2].pos.y = p2.y + dx; _VtxWritePtr[2].uv = opaque_uv; _VtxWritePtr[2].col = col;
//             _VtxWritePtr[3].pos.x = p1.x - dy; _VtxWritePtr[3].pos.y = p1.y + dx; _VtxWritePtr[3].uv = opaque_uv; _VtxWritePtr[3].col = col;
//             _VtxWritePtr += 4;
//
//             _IdxWritePtr[0] = (ImDrawIdx)(_VtxCurrentIdx); _IdxWritePtr[1] = (ImDrawIdx)(_VtxCurrentIdx + 1); _IdxWritePtr[2] = (ImDrawIdx)(_VtxCurrentIdx + 2);
//             _IdxWritePtr[3] = (ImDrawIdx)(_VtxCurrentIdx); _IdxWritePtr[4] = (ImDrawIdx)(_VtxCurrentIdx + 2); _IdxWritePtr[5] = (ImDrawIdx)(_VtxCurrentIdx + 3);
//             _IdxWritePtr += 6;
//             _VtxCurrentIdx += 4;
//         }
//     }
// }

// - We intentionally avoid using Vector2D and its math operators here to reduce cost to a minimum for debug/non-inlined builds.
// - Filled shapes must always use clockwise winding order. The anti-aliasing fringe depends on it. Counter-clockwise shapes will have "inward" anti-aliasing.
// void ImDrawList::add_convex_poly_filled(const Vector2D* points, let points_count, ImU32 col)
// {
//     if (points_count < 3)
//         return;
//
//     const Vector2D uv = _Data.TexUvWhitePixel;
//
//     if (Flags & DrawListFlags::AntiAliasedFill)
//     {
//         // Anti-aliased Fill
//         let AA_SIZE = _FringeScale;
//         const ImU32 col_trans = col & ~IM_COL32_A_MASK;
//         let idx_count = (points_count - 2)*3 + points_count * 6;
//         let vtx_count = (points_count * 2);
//         PrimReserve(idx_count, vtx_count);
//
//         // Add indexes for fill
//         unsigned int vtx_inner_idx = _VtxCurrentIdx;
//         unsigned int vtx_outer_idx = _VtxCurrentIdx + 1;
//         for (int i = 2; i < points_count; i += 1)
//         {
//             _IdxWritePtr[0] = (ImDrawIdx)(vtx_inner_idx); _IdxWritePtr[1] = (ImDrawIdx)(vtx_inner_idx + ((i - 1) << 1)); _IdxWritePtr[2] = (ImDrawIdx)(vtx_inner_idx + (i << 1));
//             _IdxWritePtr += 3;
//         }
//
//         // Compute normals
//         Vector2D* temp_normals = alloca(points_count * sizeof(Vector2D)); //-V630
//         for (int i0 = points_count - 1, i1 = 0; i1 < points_count; i0 = i1 += 1)
//         {
//             const Vector2D& p0 = points[i0];
//             const Vector2D& p1 = points[i1];
//             float dx = p1.x - p0.x;
//             float dy = p1.y - p0.y;
//             IM_NORMALIZE2F_OVER_ZERO(dx, dy);
//             temp_normals[i0].x = dy;
//             temp_normals[i0].y = -dx;
//         }
//
//         for (int i0 = points_count - 1, i1 = 0; i1 < points_count; i0 = i1 += 1)
//         {
//             // Average normals
//             const Vector2D& n0 = temp_normals[i0];
//             const Vector2D& n1 = temp_normals[i1];
//             float dm_x = (n0.x + n1.x) * 0.5;
//             float dm_y = (n0.y + n1.y) * 0.5;
//             IM_FIXNORMAL2F(dm_x, dm_y);
//             dm_x *= AA_SIZE * 0.5;
//             dm_y *= AA_SIZE * 0.5;
//
//             // Add vertices
//             _VtxWritePtr[0].pos.x = (points[i1].x - dm_x); _VtxWritePtr[0].pos.y = (points[i1].y - dm_y); _VtxWritePtr[0].uv = uv; _VtxWritePtr[0].col = col;        // Inner
//             _VtxWritePtr[1].pos.x = (points[i1].x + dm_x); _VtxWritePtr[1].pos.y = (points[i1].y + dm_y); _VtxWritePtr[1].uv = uv; _VtxWritePtr[1].col = col_trans;  // Outer
//             _VtxWritePtr += 2;
//
//             // Add indexes for fringes
//             _IdxWritePtr[0] = (ImDrawIdx)(vtx_inner_idx + (i1 << 1)); _IdxWritePtr[1] = (ImDrawIdx)(vtx_inner_idx + (i0 << 1)); _IdxWritePtr[2] = (ImDrawIdx)(vtx_outer_idx + (i0 << 1));
//             _IdxWritePtr[3] = (ImDrawIdx)(vtx_outer_idx + (i0 << 1)); _IdxWritePtr[4] = (ImDrawIdx)(vtx_outer_idx + (i1 << 1)); _IdxWritePtr[5] = (ImDrawIdx)(vtx_inner_idx + (i1 << 1));
//             _IdxWritePtr += 6;
//         }
//         _VtxCurrentIdx += (ImDrawIdx)vtx_count;
//     }
//     else
//     {
//         // Non Anti-aliased Fill
//         let idx_count = (points_count - 2)*3;
//         let vtx_count = points_count;
//         PrimReserve(idx_count, vtx_count);
//         for (int i = 0; i < vtx_count; i += 1)
//         {
//             _VtxWritePtr[0].pos = points[i]; _VtxWritePtr[0].uv = uv; _VtxWritePtr[0].col = col;
//             _VtxWritePtr += 1;
//         }
//         for (int i = 2; i < points_count; i += 1)
//         {
//             _IdxWritePtr[0] = (ImDrawIdx)(_VtxCurrentIdx); _IdxWritePtr[1] = (ImDrawIdx)(_VtxCurrentIdx + i - 1); _IdxWritePtr[2] = (ImDrawIdx)(_VtxCurrentIdx + i);
//             _IdxWritePtr += 3;
//         }
//         _VtxCurrentIdx += (ImDrawIdx)vtx_count;
//     }
// }

// void ImDrawList::_PathArcToFastEx(const Vector2D& center, float radius, int a_min_sample, int a_max_sample, int a_step)
// {
//     if (radius < 0.5)
//     {
//         _Path.push_back(center);
//         return;
//     }
//
//     // Calculate arc auto segment step size
//     if (a_step <= 0)
//         a_step = IM_DRAWLIST_ARCFAST_SAMPLE_MAX / _CalcCircleAutoSegmentCount(radius);
//
//     // Make sure we never do steps larger than one quarter of the circle
//     a_step = ImClamp(a_step, 1, IM_DRAWLIST_ARCFAST_TABLE_SIZE / 4);
//
//     let sample_range = ImAbs(a_max_sample - a_min_sample);
//     let a_next_step = a_step;
//
//     int samples = sample_range + 1;
//     bool extra_max_sample = false;
//     if (a_step > 1)
//     {
//         samples            = sample_range / a_step + 1;
//         let overstep = sample_range % a_step;
//
//         if (overstep > 0)
//         {
//             extra_max_sample = true;
//             samples += 1;
//
//             // When we have overstep to avoid awkwardly looking one long line and one tiny one at the end,
//             // distribute first step range evenly between them by reducing first step size.
//             if (sample_range > 0)
//                 a_step -= (a_step - overstep) / 2;
//         }
//     }
//
//     _Path.resize(_Path.size + samples);
//     Vector2D* out_ptr = _Path.data + (_Path.size - samples);
//
//     int sample_index = a_min_sample;
//     if (sample_index < 0 || sample_index >= IM_DRAWLIST_ARCFAST_SAMPLE_MAX)
//     {
//         sample_index = sample_index % IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
//         if (sample_index < 0)
//             sample_index += IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
//     }
//
//     if (a_max_sample >= a_min_sample)
//     {
//         for (int a = a_min_sample; a <= a_max_sample; a += a_step, sample_index += a_step, a_step = a_next_step)
//         {
//             // a_step is clamped to IM_DRAWLIST_ARCFAST_SAMPLE_MAX, so we have guaranteed that it will not wrap over range twice or more
//             if (sample_index >= IM_DRAWLIST_ARCFAST_SAMPLE_MAX)
//                 sample_index -= IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
//
//             const Vector2D s = _Data.arc_fast_vtx[sample_index];
//             out_ptr.x = center.x + s.x * radius;
//             out_ptr.y = center.y + s.y * radius;
//             out_ptr += 1;
//         }
//     }
//     else
//     {
//         for (int a = a_min_sample; a >= a_max_sample; a -= a_step, sample_index -= a_step, a_step = a_next_step)
//         {
//             // a_step is clamped to IM_DRAWLIST_ARCFAST_SAMPLE_MAX, so we have guaranteed that it will not wrap over range twice or more
//             if (sample_index < 0)
//                 sample_index += IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
//
//             const Vector2D s = _Data.arc_fast_vtx[sample_index];
//             out_ptr.x = center.x + s.x * radius;
//             out_ptr.y = center.y + s.y * radius;
//             out_ptr += 1;
//         }
//     }
//
//     if (extra_max_sample)
//     {
//         int normalized_max_sample = a_max_sample % IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
//         if (normalized_max_sample < 0)
//             normalized_max_sample += IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
//
//         const Vector2D s = _Data.arc_fast_vtx[normalized_max_sample];
//         out_ptr.x = center.x + s.x * radius;
//         out_ptr.y = center.y + s.y * radius;
//         out_ptr += 1;
//     }
//
//     // IM_ASSERT_PARANOID(_Path.data + _Path.size == out_ptr);
// }

// void ImDrawList::_PathArcToN(const Vector2D& center, float radius, float a_min, float a_max, int num_segments)
// {
//     if (radius < 0.5)
//     {
//         _Path.push_back(center);
//         return;
//     }
//
//     // Note that we are adding a point at both a_min and a_max.
//     // If you are trying to draw a full closed circle you don't want the overlapping points!
//     _Path.reserve(_Path.size + (num_segments + 1));
//     for (int i = 0; i <= num_segments; i += 1)
//     {
//         let a = a_min + ((float)i / num_segments) * (a_max - a_min);
//         _Path.push_back(Vector2D::new(center.x + ImCos(a) * radius, center.y + ImSin(a) * radius));
//     }
// }

// 0: East, 3: South, 6: West, 9: North, 12: East
// void ImDrawList::path_arc_to_fast(const Vector2D& center, float radius, int a_min_of_12, int a_max_of_12)
// {
//     if (radius < 0.5)
//     {
//         _Path.push_back(center);
//         return;
//     }
//     _PathArcToFastEx(center, radius, a_min_of_12 * IM_DRAWLIST_ARCFAST_SAMPLE_MAX / 12, a_max_of_12 * IM_DRAWLIST_ARCFAST_SAMPLE_MAX / 12, 0);
// }

// void ImDrawList::PathArcTo(const Vector2D& center, float radius, float a_min, float a_max, int num_segments)
// {
//     if (radius < 0.5)
//     {
//         _Path.push_back(center);
//         return;
//     }
//
//     if (num_segments > 0)
//     {
//         _PathArcToN(center, radius, a_min, a_max, num_segments);
//         return;
//     }
//
//     // Automatic segment count
//     if (radius <= _Data.arc_fast_radius_cutoff)
//     {
//         const bool a_is_reverse = a_max < a_min;
//
//         // We are going to use precomputed values for mid samples.
//         // Determine first and last sample in lookup table that belong to the arc.
//         let a_min_sample_f = IM_DRAWLIST_ARCFAST_SAMPLE_MAX * a_min / (f32::PI * 2.0);
//         let a_max_sample_f = IM_DRAWLIST_ARCFAST_SAMPLE_MAX * a_max / (f32::PI * 2.0);
//
//         let a_min_sample = a_is_reverse ? f32::floor(a_min_sample_f) : ImCeil(a_min_sample_f);
//         let a_max_sample = a_is_reverse ? ImCeil(a_max_sample_f) : f32::floor(a_max_sample_f);
//         let a_mid_samples = a_is_reverse ? ImMax(a_min_sample - a_max_sample, 0) : ImMax(a_max_sample - a_min_sample, 0);
//
//         let a_min_segment_angle = a_min_sample * f32::PI * 2.0 / IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
//         let a_max_segment_angle = a_max_sample * f32::PI * 2.0 / IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
//         const bool a_emit_start = ImAbs(a_min_segment_angle - a_min) >= 1e-5f;
//         const bool a_emit_end = ImAbs(a_max - a_max_segment_angle) >= 1e-5f;
//
//         _Path.reserve(_Path.size + (a_mid_samples + 1 + (a_emit_start ? 1 : 0) + (a_emit_end ? 1 : 0)));
//         if (a_emit_start)
//             _Path.push_back(Vector2D::new(center.x + ImCos(a_min) * radius, center.y + ImSin(a_min) * radius));
//         if (a_mid_samples > 0)
//             _PathArcToFastEx(center, radius, a_min_sample, a_max_sample, 0);
//         if (a_emit_end)
//             _Path.push_back(Vector2D::new(center.x + ImCos(a_max) * radius, center.y + ImSin(a_max) * radius));
//     }
//     else
//     {
//         let arc_length = ImAbs(a_max - a_min);
//         let circle_segment_count = _CalcCircleAutoSegmentCount(radius);
//         let arc_segment_count = ImMax(ImCeil(circle_segment_count * arc_length / (f32::PI * 2.0)), (2.0 * f32::PI / arc_length));
//         _PathArcToN(center, radius, a_min, a_max, arc_segment_count);
//     }
// }

// void ImDrawList::path_bezier_cubic_curve_to(const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, int num_segments)
// {
//     Vector2D p1 = _Path.back();
//     if (num_segments == 0)
//     {
//         path_bezier_cubic_curve_toCasteljau(&self.path, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y, _Data.curve_tessellation_tol, 0); // Auto-tessellated
//     }
//     else
//     {
//         float t_step = 1.0 / num_segments;
//         for (int i_step = 1; i_step <= num_segments; i_step += 1)
//             _Path.push_back(ImBezierCubicCalc(p1, p2, p3, p4, t_step * i_step));
//     }
// }

// void ImDrawList::path_bezier_quadratic_curve_to(const Vector2D& p2, const Vector2D& p3, int num_segments)
// {
//     Vector2D p1 = _Path.back();
//     if (num_segments == 0)
//     {
//         path_bezier_quadratic_curve_toCasteljau(&self.path, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, _Data.curve_tessellation_tol, 0);// Auto-tessellated
//     }
//     else
//     {
//         float t_step = 1.0 / num_segments;
//         for (int i_step = 1; i_step <= num_segments; i_step += 1)
//             _Path.push_back(ImBezierQuadraticCalc(p1, p2, p3, t_step * i_step));
//     }
// }

// void ImDrawList::PathRect(const Vector2D& a, const Vector2D& b, float rounding, ImDrawFlags flags)
// {
//     flags = fix_rect_corner_flags(flags);
//     rounding = ImMin(rounding, f32::abs(b.x - a.x) * ( ((flags & DrawFlags::RoundCornersTop)  == DrawFlags::RoundCornersTop)  || ((flags & DrawFlags::RoundCornersBottom) == DrawFlags::RoundCornersBottom) ? 0.5 : 1.0 ) - 1.0);
//     rounding = ImMin(rounding, f32::abs(b.y - a.y) * ( ((flags & DrawFlags::RoundCornersLeft) == DrawFlags::RoundCornersLeft) || ((flags & DrawFlags::RoundCornersRight)  == DrawFlags::RoundCornersRight)  ? 0.5 : 1.0 ) - 1.0);
//
//     if (rounding < 0.5 || (flags & DrawFlags::RoundCornersMask_) == DrawFlags::RoundCornersNone)
//     {
//         PathLineTo(a);
//         PathLineTo(Vector2D::new(b.x, a.y));
//         PathLineTo(b);
//         PathLineTo(Vector2D::new(a.x, b.y));
//     }
//     else
//     {
//         let rounding_tl = (flags & DrawFlags::RoundCornersTopLeft)     ? rounding : 0.0;
//         let rounding_tr = (flags & DrawFlags::RoundCornersTopRight)    ? rounding : 0.0;
//         let rounding_br = (flags & DrawFlags::RoundCornersBottomRight) ? rounding : 0.0;
//         let rounding_bl = (flags & DrawFlags::RoundCornersBottomLeft)  ? rounding : 0.0;
//         path_arc_to_fast(Vector2D::new(a.x + rounding_tl, a.y + rounding_tl), rounding_tl, 6, 9);
//         path_arc_to_fast(Vector2D::new(b.x - rounding_tr, a.y + rounding_tr), rounding_tr, 9, 12);
//         path_arc_to_fast(Vector2D::new(b.x - rounding_br, b.y - rounding_br), rounding_br, 0, 3);
//         path_arc_to_fast(Vector2D::new(a.x + rounding_bl, b.y - rounding_bl), rounding_bl, 3, 6);
//     }
// }

// void ImDrawList::AddLine(const Vector2D& p1, const Vector2D& p2, ImU32 col, float thickness)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//     PathLineTo(p1 + Vector2D::new(0.5, 0.5));
//     PathLineTo(p2 + Vector2D::new(0.5, 0.5));
//     PathStroke(col, 0, thickness);
// }

// p_min = upper-left, p_max = lower-right
// Note we don't render 1 pixels sized rectangles properly.
// void ImDrawList::AddRect(const Vector2D& p_min, const Vector2D& p_max, ImU32 col, float rounding, ImDrawFlags flags, float thickness)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//     if (Flags & DrawListFlags::AntiAliasedLines)
//         PathRect(p_min + Vector2D::new(0.50, 0.50), p_max - Vector2D::new(0.50, 0.50), rounding, flags);
//     else
//         PathRect(p_min + Vector2D::new(0.50, 0.50), p_max - Vector2D::new(0.49, 0.49), rounding, flags); // Better looking lower-right corner and rounded non-AA shapes.
//     PathStroke(col, DrawFlags::Closed, thickness);
// }

// void ImDrawList::AddRectFilled(const Vector2D& p_min, const Vector2D& p_max, ImU32 col, float rounding, ImDrawFlags flags)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//     if (rounding < 0.5 || (flags & DrawFlags::RoundCornersMask_) == DrawFlags::RoundCornersNone)
//     {
//         PrimReserve(6, 4);
//         PrimRect(p_min, p_max, col);
//     }
//     else
//     {
//         PathRect(p_min, p_max, rounding, flags);
//         path_fill_convex(col);
//     }
// }

// p_min = upper-left, p_max = lower-right
// void ImDrawList::AddRectFilledMultiColor(const Vector2D& p_min, const Vector2D& p_max, ImU32 col_upr_left, ImU32 col_upr_right, ImU32 col_bot_right, ImU32 col_bot_left)
// {
//     if (((col_upr_left | col_upr_right | col_bot_right | col_bot_left) & IM_COL32_A_MASK) == 0)
//         return;
//
//     const Vector2D uv = _Data.TexUvWhitePixel;
//     PrimReserve(6, 4);
//     PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx)); PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx + 1)); PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx + 2));
//     PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx)); PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx + 2)); PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx + 3));
//     PrimWriteVtx(p_min, uv, col_upr_left);
//     PrimWriteVtx(Vector2D::new(p_max.x, p_min.y), uv, col_upr_right);
//     PrimWriteVtx(p_max, uv, col_bot_right);
//     PrimWriteVtx(Vector2D::new(p_min.x, p_max.y), uv, col_bot_left);
// }

// void ImDrawList::AddQuad(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col, float thickness)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//
//     PathLineTo(p1);
//     PathLineTo(p2);
//     PathLineTo(p3);
//     PathLineTo(p4);
//     PathStroke(col, DrawFlags::Closed, thickness);
// }

// void ImDrawList::AddQuadFilled(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//
//     PathLineTo(p1);
//     PathLineTo(p2);
//     PathLineTo(p3);
//     PathLineTo(p4);
//     path_fill_convex(col);
// }

// void ImDrawList::AddTriangle(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col, float thickness)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//
//     PathLineTo(p1);
//     PathLineTo(p2);
//     PathLineTo(p3);
//     PathStroke(col, DrawFlags::Closed, thickness);
// }

// void ImDrawList::AddTriangleFilled(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//
//     PathLineTo(p1);
//     PathLineTo(p2);
//     PathLineTo(p3);
//     path_fill_convex(col);
// }

// void ImDrawList::AddCircle(const Vector2D& center, float radius, ImU32 col, int num_segments, float thickness)
// {
//     if ((col & IM_COL32_A_MASK) == 0 || radius < 0.5)
//         return;
//
//     if (num_segments <= 0)
//     {
//         // Use arc with automatic segment count
//         _PathArcToFastEx(center, radius - 0.5, 0, IM_DRAWLIST_ARCFAST_SAMPLE_MAX, 0);
//         _Path.size--;
//     }
//     else
//     {
//         // Explicit segment count (still clamp to avoid drawing insanely tessellated shapes)
//         num_segments = ImClamp(num_segments, 3, IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX);
//
//         // Because we are filling a closed shape we remove 1 from the count of segments/points
//         let a_max = (f32::PI * 2.0) * ((float)num_segments - 1.0) / num_segments;
//         PathArcTo(center, radius - 0.5, 0.0, a_max, num_segments - 1);
//     }
//
//     PathStroke(col, DrawFlags::Closed, thickness);
// }

// void ImDrawList::AddCircleFilled(const Vector2D& center, float radius, ImU32 col, int num_segments)
// {
//     if ((col & IM_COL32_A_MASK) == 0 || radius < 0.5)
//         return;
//
//     if (num_segments <= 0)
//     {
//         // Use arc with automatic segment count
//         _PathArcToFastEx(center, radius, 0, IM_DRAWLIST_ARCFAST_SAMPLE_MAX, 0);
//         _Path.size--;
//     }
//     else
//     {
//         // Explicit segment count (still clamp to avoid drawing insanely tessellated shapes)
//         num_segments = ImClamp(num_segments, 3, IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX);
//
//         // Because we are filling a closed shape we remove 1 from the count of segments/points
//         let a_max = (f32::PI * 2.0) * ((float)num_segments - 1.0) / num_segments;
//         PathArcTo(center, radius, 0.0, a_max, num_segments - 1);
//     }
//
//     path_fill_convex(col);
// }

// // Guaranteed to honor 'num_segments'
// void ImDrawList::AddNgon(const Vector2D& center, float radius, ImU32 col, int num_segments, float thickness)
// {
//     if ((col & IM_COL32_A_MASK) == 0 || num_segments <= 2)
//         return;
//
//     // Because we are filling a closed shape we remove 1 from the count of segments/points
//     let a_max = (f32::PI * 2.0) * ((float)num_segments - 1.0) / num_segments;
//     PathArcTo(center, radius - 0.5, 0.0, a_max, num_segments - 1);
//     PathStroke(col, DrawFlags::Closed, thickness);
// }

// Guaranteed to honor 'num_segments'
// void ImDrawList::AddNgonFilled(const Vector2D& center, float radius, ImU32 col, int num_segments)
// {
//     if ((col & IM_COL32_A_MASK) == 0 || num_segments <= 2)
//         return;
//
//     // Because we are filling a closed shape we remove 1 from the count of segments/points
//     let a_max = (f32::PI * 2.0) * ((float)num_segments - 1.0) / num_segments;
//     PathArcTo(center, radius, 0.0, a_max, num_segments - 1);
//     path_fill_convex(col);
// }

// Cubic Bezier takes 4 controls points
// void ImDrawList::add_bezier_cubic(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col, float thickness, int num_segments)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//
//     PathLineTo(p1);
//     path_bezier_cubic_curve_to(p2, p3, p4, num_segments);
//     PathStroke(col, 0, thickness);
// }

// Quadratic Bezier takes 3 controls points
// void ImDrawList::add_bezier_quadratic(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col, float thickness, int num_segments)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//
//     PathLineTo(p1);
//     path_bezier_quadratic_curve_to(p2, p3, num_segments);
//     PathStroke(col, 0, thickness);
// }

// void ImDrawList::AddText(const ImFont* font, float font_size, const Vector2D& pos, ImU32 col, const char* text_begin, const char* text_end, float wrap_width, const Vector4D* cpu_fine_clip_rect)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//
//     if (text_end == None)
//         text_end = text_begin + strlen(text_begin);
//     if (text_begin == text_end)
//         return;
//
//     // Pull default font/size from the shared ImDrawListSharedData instance
//     if (font == None)
//         font = _Data.Font;
//     if (font_size == 0.0)
//         font_size = _Data.font_size;
//
//     // IM_ASSERT(font.container_atlas.TexID == _CmdHeader.TextureId);  // Use high-level ImGui::PushFont() or low-level ImDrawList::PushTextureId() to change font.
//
//     Vector4D clip_rect = _CmdHeader.clip_rect;
//     if (cpu_fine_clip_rect)
//     {
//         clip_rect.x = ImMax(clip_rect.x, cpu_fine_clip_rect.x);
//         clip_rect.y = ImMax(clip_rect.y, cpu_fine_clip_rect.y);
//         clip_rect.z = ImMin(clip_rect.z, cpu_fine_clip_rect.z);
//         clip_rect.w = ImMin(clip_rect.w, cpu_fine_clip_rect.w);
//     }
//     font.RenderText(this, font_size, pos, col, clip_rect, text_begin, text_end, wrap_width, cpu_fine_clip_rect != None);
// }

// void ImDrawList::AddText(const Vector2D& pos, ImU32 col, const char* text_begin, const char* text_end)
// {
//     AddText(None, 0.0, pos, col, text_begin, text_end);
// }

// void ImDrawList::AddImage(ImTextureID user_texture_id, const Vector2D& p_min, const Vector2D& p_max, const Vector2D& uv_min, const Vector2D& uv_max, ImU32 col)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//
//     const bool push_texture_id = user_texture_id != _CmdHeader.TextureId;
//     if (push_texture_id)
//         PushTextureID(user_texture_id);
//
//     PrimReserve(6, 4);
//     prim_rect_uv(p_min, p_max, uv_min, uv_max, col);
//
//     if (push_texture_id)
//         pop_texture_id();
// }

// void ImDrawList::add_image_quad(ImTextureID user_texture_id, const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, const Vector2D& uv1, const Vector2D& uv2, const Vector2D& uv3, const Vector2D& uv4, ImU32 col)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//
//     const bool push_texture_id = user_texture_id != _CmdHeader.TextureId;
//     if (push_texture_id)
//         PushTextureID(user_texture_id);
//
//     PrimReserve(6, 4);
//     prim_quad_uv(p1, p2, p3, p4, uv1, uv2, uv3, uv4, col);
//
//     if (push_texture_id)
//         pop_texture_id();
// }

// void ImDrawList::add_image_rounded(ImTextureID user_texture_id, const Vector2D& p_min, const Vector2D& p_max, const Vector2D& uv_min, const Vector2D& uv_max, ImU32 col, float rounding, ImDrawFlags flags)
// {
//     if ((col & IM_COL32_A_MASK) == 0)
//         return;
//
//     flags = fix_rect_corner_flags(flags);
//     if (rounding < 0.5 || (flags & DrawFlags::RoundCornersMask_) == DrawFlags::RoundCornersNone)
//     {
//         AddImage(user_texture_id, p_min, p_max, uv_min, uv_max, col);
//         return;
//     }
//
//     const bool push_texture_id = user_texture_id != _CmdHeader.TextureId;
//     if (push_texture_id)
//         PushTextureID(user_texture_id);
//
//     int vert_start_idx = VtxBuffer.size;
//     PathRect(p_min, p_max, rounding, flags);
//     path_fill_convex(col);
//     int vert_end_idx = VtxBuffer.size;
//     shade_verts_linear_uv(this, vert_start_idx, vert_end_idx, p_min, p_max, uv_min, uv_max, true);
//
//     if (push_texture_id)
//         pop_texture_id();
// }


//-----------------------------------------------------------------------------
// [SECTION] ImDrawListSplitter
//-----------------------------------------------------------------------------
// FIXME: This may be a little confusing, trying to be a little too low-level/optimal instead of just doing vector swap..
//-----------------------------------------------------------------------------

// void ImDrawListSplitter::ClearFreeMemory()
// {
//     for (int i = 0; i < _Channels.size; i += 1)
//     {
//         if (i == _Current)
//             memset(&_Channels[i], 0, sizeof(_Channels[i]));  // current channel is a copy of cmd_buffer/idx_buffer, don't destruct again
//         _Channels[i]._CmdBuffer.clear();
//         _Channels[i]._IdxBuffer.clear();
//     }
//     _Current = 0;
//     _Count = 1;
//     _Channels.clear();
// }

// void ImDrawListSplitter::Split(ImDrawList* draw_list, int channels_count)
// {
//     IM_UNUSED(draw_list);
//     // IM_ASSERT(_Current == 0 && _Count <= 1 && "Nested channel splitting is not supported. Please use separate instances of ImDrawListSplitter.");
//     int old_channels_count = _Channels.size;
//     if (old_channels_count < channels_count)
//     {
//         _Channels.reserve(channels_count); // Avoid over reserving since this is likely to stay stable
//         _Channels.resize(channels_count);
//     }
//     _Count = channels_count;
//
//     // Channels[] (24/32 bytes each) hold storage that we'll swap with draw_list->_cmd_buffer/_idx_buffer
//     // The content of Channels[0] at this point doesn't matter. We clear it to make state tidy in a debugger but we don't strictly need to.
//     // When we switch to the next channel, we'll copy draw_list->_cmd_buffer/_idx_buffer into Channels[0] and then Channels[1] into draw_list->cmd_buffer/_idx_buffer
//     memset(&_Channels[0], 0, sizeof(ImDrawChannel));
//     for (int i = 1; i < channels_count; i += 1)
//     {
//         if (i >= old_channels_count)
//         {
//             IM_PLACEMENT_NEW(&_Channels[i]) ImDrawChannel();
//         }
//         else
//         {
//             _Channels[i]._CmdBuffer.resize(0);
//             _Channels[i]._IdxBuffer.resize(0);
//         }
//     }
// }

// void ImDrawListSplitter::Merge(ImDrawList* draw_list)
// {
//     // Note that we never use or rely on _channels.size because it is merely a buffer that we never shrink back to 0 to keep all sub-buffers ready for use.
//     if (_Count <= 1)
//         return;
//
//     SetCurrentChannel(draw_list, 0);
//     draw_list->_PopUnusedDrawCmd();
//
//     // Calculate our final buffer sizes. Also fix the incorrect idx_offset values in each command.
//     int new_cmd_buffer_count = 0;
//     int new_idx_buffer_count = 0;
//     ImDrawCmd* last_cmd = (_Count > 0 && draw_list.cmd_buffer.size > 0) ? &draw_list.cmd_buffer.back() : None;
//     int idx_offset = last_cmd ? last_cmd.IdxOffset + last_cmd.ElemCount : 0;
//     for (int i = 1; i < _Count; i += 1)
//     {
//         ImDrawChannel& ch = _Channels[i];
//         if (ch._CmdBuffer.size > 0 && ch._CmdBuffer.back().elem_count == 0 && ch._CmdBuffer.back().user_callback == None) // Equivalent of PopUnusedDrawCmd()
//             ch._CmdBuffer.pop_back();
//
//         if (ch._CmdBuffer.size > 0 && last_cmd != None)
//         {
//             // Do not include ImDrawCmd_AreSequentialIdxOffset() in the compare as we rebuild idx_offset values ourselves.
//             // Manipulating idx_offset (e.g. by reordering draw commands like done by RenderDimmedBackgroundBehindWindow()) is not supported within a splitter.
//             ImDrawCmd* next_cmd = &ch._CmdBuffer[0];
//             if (ImDrawCmd_HeaderCompare(last_cmd, next_cmd) == 0 && last_cmd.UserCallback == None && next_cmd.UserCallback == None)
//             {
//                 // merge previous channel last draw command with current channel first draw command if matching.
//                 last_cmd.ElemCount += next_cmd.ElemCount;
//                 idx_offset += next_cmd.ElemCount;
//                 ch._CmdBuffer.erase(ch._CmdBuffer.data); // FIXME-OPT: Improve for multiple merges.
//             }
//         }
//         if (ch._CmdBuffer.size > 0)
//             last_cmd = &ch._CmdBuffer.back();
//         new_cmd_buffer_count += ch._CmdBuffer.size;
//         new_idx_buffer_count += ch._IdxBuffer.size;
//         for (int cmd_n = 0; cmd_n < ch._CmdBuffer.size; cmd_n += 1)
//         {
//             ch._CmdBuffer.data[cmd_n].IdxOffset = idx_offset;
//             idx_offset += ch._CmdBuffer.data[cmd_n].elem_count;
//         }
//     }
//     draw_list.cmd_buffer.resize(draw_list.cmd_buffer.size + new_cmd_buffer_count);
//     draw_list.IdxBuffer.resize(draw_list.IdxBuffer.size + new_idx_buffer_count);
//
//     // Write commands and indices in order (they are fairly small structures, we don't copy vertices only indices)
//     ImDrawCmd* cmd_write = draw_list.cmd_buffer.data + draw_list.cmd_buffer.size - new_cmd_buffer_count;
//     ImDrawIdx* idx_write = draw_list.IdxBuffer.data + draw_list.IdxBuffer.size - new_idx_buffer_count;
//     for (int i = 1; i < _Count; i += 1)
//     {
//         ImDrawChannel& ch = _Channels[i];
//         if (int sz = ch._CmdBuffer.size) { memcpy(cmd_write, ch._CmdBuffer.data, sz * sizeof(ImDrawCmd)); cmd_write += sz; }
//         if (int sz = ch._IdxBuffer.size) { memcpy(idx_write, ch._IdxBuffer.data, sz * sizeof(ImDrawIdx)); idx_write += sz; }
//     }
//     draw_list->_IdxWritePtr = idx_write;
//
//     // Ensure there's always a non-callback draw command trailing the command-buffer
//     if (draw_list.cmd_buffer.size == 0 || draw_list.cmd_buffer.back().user_callback != None)
//         draw_list.add_draw_cmd();
//
//     // If current command is used with different settings we need to add a new command
//     ImDrawCmd* curr_cmd = &draw_list.cmd_buffer.data[draw_list.cmd_buffer.size - 1];
//     if (curr_cmd.ElemCount == 0)
//         ImDrawCmd_HeaderCopy(curr_cmd, &draw_list->_CmdHeader); // Copy clip_rect, texture_id, vtx_offset
//     else if (ImDrawCmd_HeaderCompare(curr_cmd, &draw_list->_CmdHeader) != 0)
//         draw_list.add_draw_cmd();
//
//     _Count = 1;
// }

// void ImDrawListSplitter::SetCurrentChannel(ImDrawList* draw_list, int idx)
// {
//     // IM_ASSERT(idx >= 0 && idx < _Count);
//     if (_Current == idx)
//         return;
//
//     // Overwrite ImVector (12/16 bytes), four times. This is merely a silly optimization instead of doing .swap()
//     memcpy(&_Channels.data[_Current]._CmdBuffer, &draw_list.cmd_buffer, sizeof(draw_list.cmd_buffer));
//     memcpy(&_Channels.data[_Current]._IdxBuffer, &draw_list.IdxBuffer, sizeof(draw_list.IdxBuffer));
//     _Current = idx;
//     memcpy(&draw_list.cmd_buffer, &_Channels.data[idx]._CmdBuffer, sizeof(draw_list.cmd_buffer));
//     memcpy(&draw_list.IdxBuffer, &_Channels.data[idx]._IdxBuffer, sizeof(draw_list.IdxBuffer));
//     draw_list->_IdxWritePtr = draw_list.IdxBuffer.data + draw_list.IdxBuffer.size;
//
//     // If current command is used with different settings we need to add a new command
//     ImDrawCmd* curr_cmd = (draw_list.cmd_buffer.size == 0) ? None : &draw_list.cmd_buffer.data[draw_list.cmd_buffer.size - 1];
//     if (curr_cmd == None)
//         draw_list.add_draw_cmd();
//     else if (curr_cmd.ElemCount == 0)
//         ImDrawCmd_HeaderCopy(curr_cmd, &draw_list->_CmdHeader); // Copy clip_rect, texture_id, vtx_offset
//     else if (ImDrawCmd_HeaderCompare(curr_cmd, &draw_list->_CmdHeader) != 0)
//         draw_list.add_draw_cmd();
// }

//-----------------------------------------------------------------------------
// [SECTION] ImDrawData
//-----------------------------------------------------------------------------

// For backward compatibility: convert all buffers from indexed to de-indexed, in case you cannot render indexed. Note: this is slow and most likely a waste of resources. Always prefer indexed rendering!
// void ImDrawData::DeIndexAllBuffers()
// {
//     ImVector<ImDrawVert> new_vtx_buffer;
//     total_vtx_count = total_idx_count = 0;
//     for (int i = 0; i < cmd_lists_count; i += 1)
//     {
//         ImDrawList* cmd_list = CmdLists[i];
//         if (cmd_list.IdxBuffer.empty())
//             continue;
//         new_vtx_buffer.resize(cmd_list.IdxBuffer.size);
//         for (int j = 0; j < cmd_list.IdxBuffer.size; j += 1)
//             new_vtx_buffer[j] = cmd_list.VtxBuffer[cmd_list.IdxBuffer[j]];
//         cmd_list.VtxBuffer.swap(new_vtx_buffer);
//         cmd_list.IdxBuffer.resize(0);
//         total_vtx_count += cmd_list.VtxBuffer.size;
//     }
// }

// Helper to scale the clip_rect field of each ImDrawCmd.
// Use if your final output buffer is at a different scale than draw_data->display_size,
// or if there is a difference between your window resolution and framebuffer resolution.
// void ImDrawData::ScaleClipRects(const Vector2D& fb_scale)
// {
//     for (int i = 0; i < cmd_lists_count; i += 1)
//     {
//         ImDrawList* cmd_list = CmdLists[i];
//         for (int cmd_i = 0; cmd_i < cmd_list.cmd_buffer.size; cmd_i += 1)
//         {
//             ImDrawCmd* cmd = &cmd_list.cmd_buffer[cmd_i];
//             cmd.clip_rect = Vector4D(cmd.clip_rect.x * fb_scale.x, cmd.clip_rect.y * fb_scale.y, cmd.clip_rect.z * fb_scale.x, cmd.clip_rect.w * fb_scale.y);
//         }
//     }
// }

//-----------------------------------------------------------------------------
// [SECTION] Helpers ShadeVertsXXX functions
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] ImFontConfig
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] ImFontAtlas
//-----------------------------------------------------------------------------



//-------------------------------------------------------------------------
// [SECTION] ImFontAtlas glyph ranges helpers
//-------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] ImFontGlyphRangesBuilder
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] ImFont
//-----------------------------------------------------------------------------

 // #ifndef IMGUI_DISABLE


pub const ROUND_CORNERS_BOTTOM: HashSet<DrawFlags> = HashSet::from([DrawFlags::RoundCornersBottomLeft, DrawFlags::RoundCornersBottomRight]);

pub const ROUND_CORNERS_LEFT: HashSet<DrawFlags> = HashSet::from([DrawFlags::RoundCornersBottomLeft, DrawFlags::RoundCornersTopLeft]);

pub const ROUND_CORNERS_RIGHT: HashSet<DrawFlags> = HashSet::from([DrawFlags::RoundCornersBottomRight, DrawFlags::RoundCornersTopRight]);

pub const ROUND_CORNERS_ALL: HashSet<DrawFlags> = HashSet::from([DrawFlags::RoundCornersTopLeft, DrawFlags::RoundCornersTopRight, DrawFlags::RoundCornersBottomLeft, DrawFlags::RoundCornersBottomRight]);

pub const ROUND_CORNERS_DEFAULT: HashSet<DrawFlags>        = ROUND_CORNERS_ALL;

pub const ROUND_CORNERS_MASK: HashSet<DrawFlags> = HashSet::from([DrawFlags::RoundCornersTopLeft, DrawFlags::RoundCornersTopRight, DrawFlags::RoundCornersBottomLeft, DrawFlags::RoundCornersBottomRight, DrawFlags::RoundCornersNone]);

pub type DrawCallback = fn(&mut DrawList, &DimgDrawCmd);

pub fn im_draw_callback_nop(_: &mut DrawList, _: &DimgDrawCmd) {
    todo!()
}
