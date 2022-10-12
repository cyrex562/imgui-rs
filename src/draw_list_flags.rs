#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImDrawListFlags;        // -> enum ImDrawListFlags_      // Flags: for ImDrawList instance
pub type ImDrawListFlags = c_int;

// Flags for ImDrawList instance. Those are set automatically by  functions from ImGuiIO settings, and generally not manipulated directly.
// It is however possible to temporarily alter flags between calls to ImDrawList:: functions.
// enum ImDrawListFlags_
// {
pub const ImDrawListFlags_None: ImDrawListFlags = 0;
pub const ImDrawListFlags_AntiAliasedLines: ImDrawListFlags = 1 << 0;
// Enable anti-aliased lines/borders (*2 the number of triangles for 1f32 wide line or lines thin enough to be drawn using textures; otherwise *3 the number of triangles)
pub const ImDrawListFlags_AntiAliasedLinesUseTex: ImDrawListFlags = 1 << 1;
// Enable anti-aliased lines/borders using textures when possible. Require backend to render with bilinear filtering (NOT point/nearest filtering).
pub const ImDrawListFlags_AntiAliasedFill: ImDrawListFlags = 1 << 2;
// Enable anti-aliased edge around filled shapes (rounded rectangles; circles).
pub const ImDrawListFlags_AllowVtxOffset: ImDrawListFlags = 1 << 3; // Can emit 'VtxOffset > 0' to allow large meshes. Set when 'ImGuiBackendFlags_RendererHasVtxOffset' is enabled.
                                                                    // };
