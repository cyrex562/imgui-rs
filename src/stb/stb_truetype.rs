// [DEAR IMGUI]
#![allow(non_upper_case_globals)]
// This is a slightly modified version of stb_truetype.h 1.26.
// Mostly fixing for compiler and static analyzer warnings.
// Grep for [DEAR IMGUI] to find the changes.

// stb_truetype.h - v1.26 - public domain
// authored from 2009-2021 by Sean Barrett / RAD Game Tools
//
// =======================================================================
//
//    NO SECURITY GUARANTEE -- DO NOT USE THIS ON UNTRUSTED FONT FILES
//
// This library does no range checking of the offsets found in the file,
// meaning an attacker can use it to read arbitrary memory.
//
// =======================================================================
//
//   This library processes TrueType files:
//        parse files
//        extract glyph metrics
//        extract glyph shapes
//        render glyphs to one-channel bitmaps with antialiasing (box filter)
//        render glyphs to one-channel SDF bitmaps (signed-distance field/function)
//
//   Todo:
//        non-MS cmaps
//        crashproof on bad data
//        hinting? (no longer patented)
//        cleartype-style AA?
//        optimize: use simple memory allocator for intermediates
//        optimize: build edge-list directly from curves
//        optimize: rasterize directly from curves?
//
// ADDITIONAL CONTRIBUTORS
//
//   Mikko Mononen: compound shape support, more cmap formats
//   Tor Andersson: kerning, subpixel rendering
//   Dougall Johnson: OpenType / Type 2 font handling
//   Daniel Ribeiro Maciel: basic GPOS-based kerning
//
//   Misc other:
//       Ryan Gordon
//       Simon Glass
//       github:IntellectualKitty
//       Imanol Celaya
//       Daniel Ribeiro Maciel
//
//   Bug/warning reports/fixes:
//       "Zer" on mollyrocket       Fabian "ryg" Giesen   github:NiLuJe
//       Cass Everitt               Martins Mozeiko       github:aloucks
//       stoiko (Haemimont Games)   Cap Petschulat        github:oyvindjam
//       Brian Hook                 Omar Cornut           github:vassvik
//       Walter van Niftrik         Ryan Griege
//       David Gow                  Peter LaValle
//       David Given                Sergey Popov
//       Ivan-Assen Ivanov          Giumo X. Clanjor
//       Anthony Pesch              Higor Euripedes
//       Johan Duparc               Thomas Fields
//       Hou Qiming                 Derek Vinyard
//       Rob Loach                  Cort Stratton
//       Kenney Phillis Jr.         Brian Costabile
//       Ken Voskuil (kaesve)
//
// VERSION HISTORY
//
//   1.26 (2021-08-28) fix broken rasterizer
//   1.25 (2021-07-11) many fixes
//   1.24 (2020-02-05) fix warning
//   1.23 (2020-02-02) query SVG data for glyphs; query whole kerning table (but only kern not GPOS)
//   1.22 (2019-08-11) minimize missing-glyph duplication; fix kerning if both 'GPOS' and 'kern' are defined
//   1.21 (2019-02-25) fix warning
//   1.20 (2019-02-07) PackFontRange skips missing codepoints; GetScaleFontVMetrics()
//   1.19 (2018-02-11) GPOS kerning, STBTT_fmod
//   1.18 (2018-01-29) add missing function
//   1.17 (2017-07-23) make more arguments const; doc fix
//   1.16 (2017-07-12) SDF support
//   1.15 (2017-03-03) make more arguments const
//   1.14 (2017-01-16) num-fonts-in-TTC function
//   1.13 (2017-01-02) support OpenType fonts, certain Apple fonts
//   1.12 (2016-10-25) suppress warnings about casting away const with -Wcast-qual
//   1.11 (2016-04-02) fix unused-variable warning
//   1.10 (2016-04-02) user-defined fabs(); rare memory leak; remove duplicate typedef
//   1.09 (2016-01-16) warning fix; avoid crash on outofmem; use allocation userdata properly
//   1.08 (2015-09-13) document stbtt_Rasterize(); fixes for vertical & horizontal edges
//   1.07 (2015-08-01) allow PackFontRanges to accept arrays of sparse codepoints;
//                     variant PackFontRanges to pack and render in separate phases;
//                     fix stbtt_GetFontOFfsetForIndex (never worked for non-0 input?);
//                     fixed an assert() bug in the new rasterizer
//                     replace assert() with STBTT_assert() in new rasterizer
//
//   Full history can be found at the end of this file.
//
// LICENSE
//
//   See end of file for license information.
//
// USAGE
//
//   Include this file in whatever places need to refer to it. In ONE C/C++
//   file, write:
//      #define STB_TRUETYPE_IMPLEMENTATION
//   before the #include of this file. This expands out the actual
//   implementation into that C/C++ file.
//
//   To make the implementation private to the file that generates the implementation,
//      #define STBTT_STATIC
//
//   Simple 3D API (don't ship this, but it's fine for tools and quick start)
//           stbtt_BakeFontBitmap()               -- bake a font to a bitmap for use as texture
//           stbtt_GetBakedQuad()                 -- compute quad to draw for a given char
//
//   Improved 3D API (more shippable):
//           #include "stb_rect_pack.h"           -- optional, but you really want it
//           stbtt_PackBegin()
//           stbtt_PackSetOversampling()          -- for improved quality on small fonts
//           stbtt_PackFontRanges()               -- pack and renders
//           stbtt_PackEnd()
//           stbtt_GetPackedQuad()
//
//   "Load" a font file from a memory buffer (you have to keep the buffer loaded)
//           stbtt_InitFont()
//           stbtt_GetFontOffsetForIndex()        -- indexing for TTC font collections
//           stbtt_GetNumberOfFonts()             -- number of fonts for TTC font collections
//
//   Render a unicode codepoint to a bitmap
//           stbtt_GetCodepointBitmap()           -- allocates and returns a bitmap
//           stbtt_MakeCodepointBitmap()          -- renders into bitmap you provide
//           stbtt_GetCodepointBitmapBox()        -- how big the bitmap must be
//
//   Character advance/positioning
//           stbtt_GetCodepointHMetrics()
//           stbtt_GetFontVMetrics()
//           stbtt_GetFontVMetricsOS2()
//           stbtt_GetCodepointKernAdvance()
//
//   Starting with version 1.06, the rasterizer was replaced with a new,
//   faster and generally-more-precise rasterizer. The new rasterizer more
//   accurately measures pixel coverage for anti-aliasing, except in the case
//   where multiple shapes overlap, in which case it overestimates the AA pixel
//   coverage. Thus, anti-aliasing of intersecting shapes may look wrong. If
//   this turns out to be a problem, you can re-enable the old rasterizer with
//        #define STBTT_RASTERIZER_VERSION 1
//   which will incur about a 15% speed hit.
//
// ADDITIONAL DOCUMENTATION
//
//   Immediately after this block comment are a series of sample programs.
//
//   After the sample programs is the "header file" section. This section
//   includes documentation for each API function.
//
//   Some important concepts to understand to use this library:
//
//      Codepoint
//         Characters are defined by unicode codepoints, e.g. 65 is
//         uppercase A, 231 is lowercase c with a cedilla, 0x7e30 is
//         the hiragana for "ma".
//
//      Glyph
//         A visual character shape (every codepoint is rendered as
//         some glyph)
//
//      Glyph index
//         A font-specific integer ID representing a glyph
//
//      Baseline
//         Glyph shapes are defined relative to a baseline, which is the
//         bottom of uppercase characters. Characters extend both above
//         and below the baseline.
//
//      Current Point
//         As you draw text to the screen, you keep track of a "current point"
//         which is the origin of each character. The current point's vertical
//         position is the baseline. Even "baked fonts" use this model.
//
//      Vertical Font Metrics
//         The vertical qualities of the font, used to vertically position
//         and space the characters. See docs for stbtt_GetFontVMetrics.
//
//      Font Size in Pixels or Points
//         The preferred interface for specifying font sizes in stb_truetype
//         is to specify how tall the font's vertical extent should be in pixels.
//         If that sounds good enough, skip the next paragraph.
//
//         Most font APIs instead use "points", which are a common typographic
//         measurement for describing font size, defined as 72 points per inch.
//         stb_truetype provides a point API for compatibility. However, true
//         "per inch" conventions don't make much sense on computer displays
//         since different monitors have different number of pixels per
//         inch. For example, Windows traditionally uses a convention that
//         there are 96 pixels per inch, thus making 'inch' measurements have
//         nothing to do with inches, and thus effectively defining a point to
//         be 1.333 pixels. Additionally, the TrueType font data provides
//         an explicit scale factor to scale a given font's glyphs to points,
//         but the author has observed that this scale factor is often wrong
//         for non-commercial fonts, thus making fonts scaled in points
//         according to the TrueType spec incoherently sized in practice.
//
// DETAILED USAGE:
//
//  Scale:
//    Select how high you want the font to be, in points or pixels.
//    Call ScaleForPixelHeight or ScaleForMappingEmToPixels to compute
//    a scale factor SF that will be used by all other functions.
//
//  Baseline:
//    You need to select a y-coordinate that is the baseline of where
//    your text will appear. Call GetFontBoundingBox to get the baseline-relative
//    bounding box for all characters. SF*-y0 will be the distance in pixels
//    that the worst-case character could extend above the baseline, so if
//    you want the top edge of characters to appear at the top of the
//    screen where y=0, then you would set the baseline to SF*-y0.
//
//  Current point:
//    Set the current point where the first character will appear. The
//    first character could extend left of the current point; this is font
//    dependent. You can either choose a current point that is the leftmost
//    point and hope, or add some padding, or check the bounding box or
//    left-side-bearing of the first character to be displayed and set
//    the current point based on that.
//
//  Displaying a character:
//    Compute the bounding box of the character. It will contain signed values
//    relative to <current_point, baseline>. I.e. if it returns x0,y0,x1,y1,
//    then the character should be displayed in the rectangle from
//    <current_point+SF*x0, baseline+SF*y0> to <current_point+SF*x1,baseline+SF*y1).
//
//  Advancing for the next character:
//    Call GlyphHMetrics, and compute 'current_point += SF * advance'.
//
//
// ADVANCED USAGE
//
//   Quality:
//
//    - Use the functions with Subpixel at the end to allow your characters
//      to have subpixel positioning. Since the font is anti-aliased, not
//      hinted, this is very import for quality. (This is not possible with
//      baked fonts.)
//
//    - Kerning is now supported, and if you're supporting subpixel rendering
//      then kerning is worth using to give your text a polished look.
//
//   Performance:
//
//    - Convert Unicode codepoints to glyph indexes and operate on the glyphs;
//      if you don't do this, stb_truetype is forced to do the conversion on
//      every call.
//
//    - There are a lot of memory allocations. We should modify it to take
//      a temp buffer and allocate from the temp buffer (without freeing),
//      should help performance a lot.
//
// NOTES
//
//   The system uses the raw data found in the .ttf file without changing it
//   and without building auxiliary data structures. This is a bit inefficient
//   on little-endian systems (the data is big-endian), but assuming you're
//   caching the bitmaps or glyph shapes this shouldn't be a big deal.
//
//   It appears to be very hard to programmatically determine what font a
//   given file is in a general way. I provide an API for this, but I don't
//   recommend it.
//
//
// PERFORMANCE MEASUREMENTS FOR 1.06:
//
//                      32-bit     64-bit
//   Previous release:  8.83 s     7.68 s
//   Pool allocations:  7.72 s     6.34 s
//   Inline sort     :  6.54 s     5.65 s
//   New rasterizer  :  5.63 s     5.00 s

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////
////
////  SAMPLE PROGRAMS
////
//
//  Incomplete text-in-3d-api example, which draws quads properly aligned to be lossless.
//  See "tests/truetype_demo_win32.c" for a complete version.
// #if 0
// #define STB_TRUETYPE_IMPLEMENTATION  // force following include to generate implementation
// #include "stb_truetype.h"

use std::fmt::format;
use std::mem;
use std::ptr::null_mut;
use libc::{c_char, c_float, c_int, c_uchar, c_uint, c_ulong, c_void, size_t};
use crate::stb_rp_context::stbrp_context;
use crate::stb_rp_node::stbrp_node;
use crate::stb_tt_active_edge::stbtt__active_edge;
use crate::stb_tt_aligned_quad::stbtt_aligned_quad;
use crate::stb_tt_buf::stbtt__buf;
use crate::stb_tt_csctx::stbtt__csctx;
use crate::stb_tt_edge::stbtt__edge;
use crate::stb_tt_fontinfo::stbtt_fontinfo;
use crate::stb_tt_hheap::stbtt__hheap;
use crate::stb_tt_hheap_chunk::stbtt__hheap_chunk;
use crate::stb_tt_kerning_entry::stbtt_kerningentry;
use crate::stb_tt_packed_context::stbtt_pack_context;
use crate::stb_tt_packed_range::stbtt_pack_range;
use crate::stb_tt_shapes::{STBTT_vcubic, STBTT_vcurve, STBTT_vline, STBTT_vmove};
use crate::stb_tt_types::stbtt_vertex_type;
use crate::stb_tt_vertex::stbtt_vertex;
use crate::core::string_ops::str_to_const_c_char_ptr;
use crate::core::utils::flag_clear;

// c_uchar ttf_buffer[1<<20];
pub static ttf_buffer: [c_uchar;1<<20] = [0;1<<20];
// c_uchar temp_bitmap[*mut 512512];
pub static temp_bitmap: [c_uchar; 512 *512] = [0;512*512];

// stbtt_bakedcdata: [c_char;96]; // ASCII 32..126 is 95 glyphs
pub static stbtt_bakedcdata: [c_char;96] = [0;96];
// GLuint ftex;
pub static ftex: c_uint = 0;

pub unsafe fn my_stbtt_initfont() {
    libc::fread(ttf_buffer.as_mut_ptr(), 1, 1 << 20, libc::fopen(str_to_const_c_char_ptr("c:/windows/fonts/times.ttf"), str_to_const_c_char_ptr("rb")));
    stbtt_BakeFontBitmap(ttf_buffer.as_ptr(), 0, 32.0, temp_bitmap.as_mut_ptr(), 512, 512, 32, 96, cdata); // no guarantee this fits!
    // can free ttf_buffer at this point
    glGenTextures(1, &ftex);
    glBindTexture(GL_TEXTURE_2D, ftex);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_ALPHA, 512, 512, 0, GL_ALPHA, GL_UNSIGNED_BYTE, temp_bitmap);
    // can free temp_bitmap at this point
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
}

pub unsafe fn my_stbtt_print(mut x: c_float, mut y: c_float, mut text: *mut c_char) {
    // assume orthographic projection with units = screen pixels, origin at top left
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glEnable(GL_TEXTURE_2D);
    glBindTexture(GL_TEXTURE_2D, ftex);
    glBegin(GL_QUADS);
    while *text {
        if *text >= 32 && *text < 128 {
            let mut q = stbtt_aligned_quad::default();
            stbtt_GetBakedQuad(cdata, 512, 512, ((*text) - 32) as c_int, &mut x, &mut y, &mut q, 1);//1=opengl & d3d10+,0=d3d9
            glTexCoord2f(q.s0, q.t0);
            glVertex2f(q.x0, q.y0);
            glTexCoord2f(q.s1, q.t0);
            glVertex2f(q.x1, q.y0);
            glTexCoord2f(q.s1, q.t1);
            glVertex2f(q.x1, q.y1);
            glTexCoord2f(q.s0, q.t1);
            glVertex2f(q.x0, q.y1);
        }
        text += 1;
    }
    glEnd();
}

// #endif
//
//
//////////////////////////////////////////////////////////////////////////////
//
// Complete program (this compiles): get a single bitmap, print as ASCII art
//
// #if 0
// #include <stdio.h>
// #define STB_TRUETYPE_IMPLEMENTATION  // force following include to generate implementation
// #include "stb_truetype.h"

 // ttf_buffer: c_char[1<<25];

// main: c_int(argc: c_int, char **argv)
// {
//    stbtt_fontinfo font;
//    c_ubitmap: *mut c_char;
//    w: c_int,h,i,j,c = (argc > 1 ? atoi(argv[1]) : 'a'), s = (argc > 2 ? atoi(argv[2]) : 20);
//
//    fread(ttf_buffer, 1, 1<<25, fopen(argc > 3 ? argv[3] : "c:/windows/fonts/arialbd.ttf", "rb"));
//
//    stbtt_InitFont(&font, ttf_buffer, stbtt_GetFontOffsetForIndex(ttf_buffer,0));
//    bitmap = stbtt_GetCodepointBitmap(&font, 0,stbtt_ScaleForPixelHeight(&font, s), c, &w, &h, 0,0);
//
//    for (j=0; j < h; ++j) {
//       for (i=0; i < w; ++i)
//          putchar(" .:ioVM@"[bitmap[*mut jw+i]>>5]);
//       putchar('\n');
//    }
//    return 0;
// }
// #endif
//
// Output:
//
//     .ii.
//    @@@@@@.
//   V@Mio@@o
//   :i.  V@V
//     :oM@@M
//   :@@@MM@M
//   @@o  o@M
//  :@@.  M@M
//   @@@o@@@@
//   :M@@V:@@.
//
//////////////////////////////////////////////////////////////////////////////
//
// Complete program: print "Hello World!" banner, with bugs
//
// #if 0
//  buffer: c_char[24<<20];
// unsigned screen: [c_char;20][79];

// main: c_int(arg: c_int, char **argv)
// {
//    stbtt_fontinfo font;
//    i: c_int,j,ascent,baseline,ch=0;scale: c_float, xpos=2; // leave a little padding in case the character extends left
//    text: *mut c_char = "Heljo World!"; // intentionally misspelled to show 'lj' brokenness
//
//    fread(buffer, 1, 1000000, fopen("c:/windows/fonts/arialbd.ttf", "rb"));
//    stbtt_InitFont(&font, buffer, 0);
//
//    scale = stbtt_ScaleForPixelHeight(&font, 15);
//    stbtt_GetFontVMetrics(&font, &ascent,0,0);
//    baseline =  (*mut ascentscale);
//
//    while (text[ch]) {
//       advance: c_int,lsb,x0,y0,x1,y1;
//       let x_shift: c_float =  xpos -  floor(xpos);
//       stbtt_GetCodepointHMetrics(&font, text[ch], &advance, &lsb);
//       stbtt_GetCodepointBitmapBoxSubpixel(&font, text[ch], scale,scale,x_shift,0, &x0,&y0,&x1,&y1);
//       stbtt_MakeCodepointBitmapSubpixel(&font, &screen[baseline + y0][ xpos + x0], x1-x0,y1-y0, 79, scale,scale,x_shift,0, text[ch]);
//       // note that this stomps the old data, so where character boxes overlap (e.g. 'lj') it's wrong
//       // because this API is really for baking character bitmaps into textures. if you want to render
//       // a sequence of characters, you really need to render each bitmap to a temp buffer, then
//       // "alpha blend" that into the working buffer
//       xpos += (advance * scale);
//       if (text[ch1])
//          xpos += *mut scalestbtt_GetCodepointKernAdvance(&font, text[ch],text[ch1]);
//       ch += 1;
//    }
//
//    for (j=0; j < 20; ++j) {
//       for (i=0; i < 78; ++i)
//          putchar(" .:ioVM@"[screen[j][i]>>5]);
//       putchar('\n');
//    }
//
//    return 0;
// }
// // #endif


//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////
////
////   INTEGRATION WITH YOUR CODEBASE
////
////   The following sections allow you to supply alternate definitions
////   of C library functions used by stb_truetype, e.g. if you don't
////   link with the C runtime library.

// #ifdef STB_TRUETYPE_IMPLEMENTATION
   // #define your own (u)stbtt_int8/16/32 before including to override this
//    #ifndef stbtt_uint8
//    typedef c_uchar   stbtt_uint8;
//    typedef signed   char   stbtt_int8;
//    typedef unsigned c_short  stbtt_uint16;
//    typedef signed   c_short  stbtt_int16;
//    typedef c_uint    stbtt_uint32;
//    typedef signed   c_int    stbtt_int32;
//    #endif
//
//    typedef  stbtt__check_size32: c_char[sizeof==4 ? 1 : -1];
//    typedef  stbtt__check_size16: c_char[sizeof==2 ? 1 : -1];
//
//    // e.g. #define your own STBTT_ifloor/STBTT_iceil() to avoid math.h
//    #ifndef STBTT_ifloor
//    #include <math.h>
//    #define STBTT_ifloor(x)   ( floor(x))
//    #define STBTT_iceil(x)    ( ceil(x))
//    #endif
//
//    #ifndef STBTT_sqrt
//    #include <math.h>
//    #define STBTT_sqrt(x)      sqrt(x)
//    #define STBTT_pow(x,y)     pow(x,y)
//    #endif
//
//    #ifndef STBTT_fmod
//    #include <math.h>
//    #define STBTT_fmod(x,y)    fmod(x,y)
//    #endif
//
//    #ifndef STBTT_cos
//    #include <math.h>
//    #define STBTT_cos(x)       cos(x)
//    #define STBTT_acos(x)      acos(x)
//    #endif
//
//    #ifndef STBTT_fabs
//    #include <math.h>
//    #define STBTT_fabs(x)      fabs(x)
//    #endif
//
//    // #define your own functions "STBTT_malloc" / "STBTT_free" to avoid malloc.h
//    #ifndef STBTT_malloc
//    #include <stdlib.h>
//    #define STBTT_malloc(x,u)  ((u),malloc(x))
//    #define STBTT_free(x,u)    ((u),free(x))
//    #endif
//
//    #ifndef STBTT_assert
//    #include <assert.h>
//    #define STBTT_assert(x)    assert(x)
//    #endif
//
//    #ifndef STBTT_strlen
//    #include <string.h>
//    #define STBTT_strlen(x)    strlen(x)
//    #endif
//
//    #ifndef STBTT_memcpy
//    #include <string.h>
//    #define STBTT_memcpy       memcpy
//    #define STBTT_memset       memset
//    #endif
// // #endif

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
////
////   INTERFACE
////
////

// #ifndef __STB_INCLUDE_STB_TRUETYPE_H__
// #define __STB_INCLUDE_STB_TRUETYPE_H__

// #ifdef STBTT_STATIC
// #define STBTT_DEF static
// #else
// #define STBTT_DEF extern
// #endif

// #ifdef __cplusplus
// extern "C" {
// #endif



//////////////////////////////////////////////////////////////////////////////
//
// TEXTURE BAKING API
//
// If you use this API, you only have to call two functions ever.
//


//
// STBTT_DEF stbtt_BakeFontBitmap: c_int(data: *const c_uchar, offset: c_int,  // font location (use offset=0 for plain .tt0f32)pixel_height: c_float,                     // height of font in pixels
//                                 c_upixels: *mut c_char, pw: c_int, ph: c_int,  // bitmap to be filled in
//                                 first_char: c_int, num_chars: c_int,          // characters to bake
//                                 stbtt_bakedchardata: *mut c_char);             // you allocate this, it's num_chars long
// // if return is positive, the first unused row of the bitmap
// // if return is negative, returns the negative of the number of characters that fit
// // if return is 0, no characters fit and no rows were used
// // This uses a very crappy packing.
//
//
//
// STBTT_DEF c_void stbtt_GetBakedQuad(const stbtt_bakedchardata: *mut c_char, pw: c_int, ph: c_int,  // same data as above
//                                char_index: c_int,             // character to display
//                                c_float *xpos, c_float *ypos,   // pointers to current position in screen pixel space
//                                stbtt_aligned_quad *q,      // output: quad to draw
//                                opengl_fillrule: c_int);       // true if opengl fill rule; false if DX9 or earlier
// // Call GetBakedQuad with char_index = 'character - first_char', and it
// // creates the quad you need to draw and advances the current position.
// //
// // The coordinate system used assumes y increases downwards.
// //
// // Characters will extend both above and below the current position;
// // see discussion of "BASELINE" above.
// //
// // It's inefficient; you might want to c&p it and optimize it.
//
// STBTT_DEF c_void stbtt_GetScaledFontVMetrics(fontdata: *const c_uchar, index: c_int,size: c_float, c_float *ascent, c_float *descent, c_float *lineGap);
// // Query the font vertical metrics without having to create a font first.
//

//////////////////////////////////////////////////////////////////////////////
//
// NEW TEXTURE BAKING API
//
// This provides options for packing multiple fonts into one atlas, not
// perfectly but better than nothing.



// typedef struct stbtt_pack_context stbtt_pack_context;
// typedef struct stbtt_fontinfo stbtt_fontinfo;
// // #ifndef STB_RECT_PACK_VERSION
// typedef struct stbrp_rect stbrp_rect;
// // #endif

// STBTT_DEF c_int  stbtt_PackBegin(stbtt_pack_context *spc, c_upixels: *mut c_char, width: c_int, height: c_int, stride_in_bytes: c_int, padding: c_int, c_void *alloc_context);
// Initializes a packing context stored in the passed-in stbtt_pack_context.
// Future calls using this context will pack characters into the bitmap passed
// in here: a 1-channel bitmap that is width * height. stride_in_bytes is
// the distance from one row to the next (or 0 to mean they are packed tightly
// together). "padding" is the amount of padding to leave between each
// character (normally you want '1' for bitmaps you'll use as textures with
// bilinear filtering).
//
// Returns 0 on failure, 1 on success.

// STBTT_DEF c_void stbtt_PackEnd  (stbtt_pack_context *spc);
// Cleans up the packing context and frees all memory.

// #define STBTT_POINT_SIZE(x)   (-(x))

// STBTT_DEF c_int  stbtt_PackFontRange(stbtt_pack_context *spc, fontdata: *const c_uchar, font_index: c_int,font_size: c_float,
//                                 first_unicode_char_in_range: c_int, num_chars_in_range: c_int, stbtt_packedchardata_for_range: *mut c_char);
// Creates character bitmaps from the font_index'th font found in fontdata (use
// font_index=0 if you don't know what that is). It creates num_chars_in_range
// bitmaps for characters with unicode values starting at first_unicode_char_in_range
// and increasing. Data for how to render them is stored in chardata_for_range;
// pass these to stbtt_GetPackedQuad to get back renderable quads.
//
// font_size is the full height of the character from ascender to descender,
// as computed by stbtt_ScaleForPixelHeight. To use a point size as computed
// by stbtt_ScaleForMappingEmToPixels, wrap the point size in STBTT_POINT_SIZE()
// and pass that result as 'font_size':
//       ...,                  20 , ... // font max minus min y is 20 pixels tall
//       ..., STBTT_POINT_SIZE(20), ... // 'M' is 20 pixels tall



// STBTT_DEF c_int  stbtt_PackFontRanges(stbtt_pack_context *spc, fontdata: *const c_uchar, font_index: c_int, stbtt_pack_range *ranges, num_ranges: c_int);
// Creates character bitmaps from multiple ranges of characters stored in
// ranges. This will usually create a better-packed bitmap than multiple
// calls to stbtt_PackFontRange. Note that you can call this multiple
// times within a single PackBegin/PackEnd.

// STBTT_DEF c_void stbtt_PackSetOversampling(stbtt_pack_context *spc, h_oversample: c_uint, v_oversample: c_uint);
// Oversampling a font increases the quality by allowing higher-quality subpixel
// positioning, and is especially valuable at smaller text sizes.
//
// This function sets the amount of oversampling for all following calls to
// stbtt_PackFontRange(s) or stbtt_PackFontRangesGatherRects for a given
// pack context. The default (no oversampling) is achieved by h_oversample=1
// and v_oversample=1. The total number of pixels required is
// h_oversample*v_oversample larger than the default; for example, 2x2
// oversampling requires 4x the storage of 1x1. For best results, render
// oversampled textures with bilinear filtering. Look at the readme in
// stb/tests/oversample for information about oversampled fonts
//
// To use with PackFontRangesGather etc., you must set it before calls
// call to PackFontRangesGatherRects.

// STBTT_DEF c_void stbtt_PackSetSkipMissingCodepoints(stbtt_pack_context *spc, skip: c_int);
// If skip != 0, this tells stb_truetype to skip any codepoints for which
// there is no corresponding glyph. If skip=0, which is the default, then
// codepoints without a glyph recived the font's "missing character" glyph,
// typically an empty box by convention.

// STBTT_DEF c_void stbtt_GetPackedQuad(const stbtt_packedchardata: *mut c_char, pw: c_int, ph: c_int,  // same data as above
//                                char_index: c_int,             // character to display
//                                c_float *xpos, c_float *ypos,   // pointers to current position in screen pixel space
//                                stbtt_aligned_quad *q,      // output: quad to draw
//                                align_to_integer: c_int);

// STBTT_DEF c_int  stbtt_PackFontRangesGatherRects(stbtt_pack_context *spc, info: *const stbtt_fontinfo, stbtt_pack_range *ranges, num_ranges: c_int, stbrp_rect *rects);
// STBTT_DEF c_void stbtt_PackFontRangesPackRects(stbtt_pack_context *spc, stbrp_rect *rects, num_rects: c_int);
// STBTT_DEF c_int  stbtt_PackFontRangesRenderIntoRects(stbtt_pack_context *spc, info: *const stbtt_fontinfo, stbtt_pack_range *ranges, num_ranges: c_int, stbrp_rect *rects);
// Calling these functions in sequence is roughly equivalent to calling
// stbtt_PackFontRanges(). If you more control over the packing of multiple
// fonts, or if you want to pack custom data into a font texture, take a look
// at the source to of stbtt_PackFontRanges() and create a custom version
// using these functions, e.g. call GatherRects multiple times,
// building up a single array of rects, then call PackRects once,
// then call RenderIntoRects repeatedly. This may result in a
// better packing than calling PackFontRanges multiple times
// (or it may not).



//////////////////////////////////////////////////////////////////////////////
//
// FONT LOADING
//
//

// STBTT_DEF stbtt_GetNumberOfFonts: c_int(data: *const c_uchar);
// This function will determine the number of fonts in a font file.  TrueType
// collection (.ttc) files may contain multiple fonts, while TrueType font
// (.tt0f32) files only contain one font. The number of fonts can be used for
// indexing with the previous function where the index is between zero and one
// less than the total fonts. If an error occurs, -1 is returned.

// STBTT_DEF stbtt_GetFontOffsetForIndex: c_int(data: *const c_uchar, index: c_int);
// Each .ttf/.ttc file may have more than one font. Each font has a sequential
// index number starting from 0. Call this function to get the font offset for
// a given index; it returns -1 if the index is out of range. A regular .ttf
// file will only define one font and it always be at offset 0, so it will
// return '0' for index 0, and -1 for all other indices.

// STBTT_DEF stbtt_InitFont: c_int(stbtt_fontinfo *info, data: *const c_uchar, offset: c_int);
// Given an offset into the file that defines a font, this function builds
// the necessary cached info for the rest of the system. You must allocate
// the stbtt_fontinfo yourself, and stbtt_InitFont will fill it out. You don't
// need to do anything special to free it, because the contents are pure
// value data with no additional data structures. Returns 0 on failure.


//////////////////////////////////////////////////////////////////////////////
//
// CHARACTER TO GLYPH-INDEX CONVERSIOn

// STBTT_DEF stbtt_FindGlyphIndex: c_int(info: *const stbtt_fontinfo, unicode_codepoint: c_int);
// If you're going to perform multiple operations on the same character
// and you want a speed-up, call this function with the character you're
// going to process, then use glyph-based functions instead of the
// codepoint-based functions.
// Returns 0 if the character codepoint is not defined in the font.


//////////////////////////////////////////////////////////////////////////////
//
// CHARACTER PROPERTIES
//

// STBTT_DEFstbtt_ScaleForPixelHeight: c_float(info: *const stbtt_fontinfo,pixels: c_float);
// computes a scale factor to produce a font whose "height" is 'pixels' tall.
// Height is measured as the distance from the highest ascender to the lowest
// descender; in other words, it's equivalent to calling stbtt_GetFontVMetrics
// and computing:
//       scale = pixels / (ascent - descent)
// so if you prefer to measure height by the ascent only, use a similar calculation.

// STBTT_DEFstbtt_ScaleForMappingEmToPixels: c_float(info: *const stbtt_fontinfo,pixels: c_float);
// computes a scale factor to produce a font whose EM size is mapped to
// 'pixels' tall. This is probably what traditional APIs compute, but
// I'm not positive.

// STBTT_DEF c_void stbtt_GetFontVMetrics(info: *const stbtt_fontinfo, ascent: *mut c_int, descent: *mut c_int, lineGap: *mut c_int);
// ascent is the coordinate above the baseline the font extends; descent
// is the coordinate below the baseline the font extends (i.e. it is typically negative)
// lineGap is the spacing between one row's descent and the next row's ascent...
// so you should advance the vertical position by "*ascent - *descent + *lineGap"
//   these are expressed in unscaled coordinates, so you must multiply by
//   the scale factor for a given size

// STBTT_DEF c_int  stbtt_GetFontVMetricsOS2(info: *const stbtt_fontinfo, typoAscent: *mut c_int, typoDescent: *mut c_int, typoLineGap: *mut c_int);
// analogous to GetFontVMetrics, but returns the "typographic" values from the OS/2
// table (specific to MS/Windows TTF files).
//
// Returns 1 on success (table present), 0 on failure.

// STBTT_DEF c_void stbtt_GetFontBoundingBox(info: *const stbtt_fontinfo, x0: *mut c_int, y0: *mut c_int, x1: *mut c_int, y1: *mut c_int);
// the bounding box around all possible characters

// STBTT_DEF c_void stbtt_GetCodepointHMetrics(info: *const stbtt_fontinfo, codepoint: c_int, advanceWidth: *mut c_int, leftSideBearing: *mut c_int);
// leftSideBearing is the offset from the current horizontal position to the left edge of the character
// advanceWidth is the offset from the current horizontal position to the next horizontal position
//   these are expressed in unscaled coordinates

// STBTT_DEF c_int  stbtt_GetCodepointKernAdvance(info: *const stbtt_fontinfo, ch1: c_int, ch2: c_int);
// an additional amount to add to the 'advance' value between ch1 and ch2

// STBTT_DEF stbtt_GetCodepointBox: c_int(info: *const stbtt_fontinfo, codepoint: c_int, x0: *mut c_int, y0: *mut c_int, x1: *mut c_int, y1: *mut c_int);
// Gets the bounding box of the visible part of the glyph, in unscaled coordinates

// STBTT_DEF c_void stbtt_GetGlyphHMetrics(info: *const stbtt_fontinfo, glyph_index: c_int, advanceWidth: *mut c_int, leftSideBearing: *mut c_int);
// STBTT_DEF c_int  stbtt_GetGlyphKernAdvance(info: *const stbtt_fontinfo, glyph1: c_int, glyph2: c_int);
// STBTT_DEF c_int  stbtt_GetGlyphBox(info: *const stbtt_fontinfo, glyph_index: c_int, x0: *mut c_int, y0: *mut c_int, x1: *mut c_int, y1: *mut c_int);
// as above, but takes one or more glyph indices for greater efficiency


// STBTT_DEF c_int  stbtt_GetKerningTableLength(info: *const stbtt_fontinfo);
// STBTT_DEF c_int  stbtt_GetKerningTable(info: *const stbtt_fontinfo, *mut stbtt_kerningentry table, table_length: c_int);
// Retrieves a complete list of all of the kerning pairs provided by the font
// stbtt_GetKerningTable never writes more than table_length entries and returns how many entries it did write.
// The table will be sorted by (a.glyph1 == b.glyph1)?(a.glyph2 < b.glyph2):(a.glyph1 < b.glyph1)


// #endif

// STBTT_DEF stbtt_IsGlyphEmpty: c_int(info: *const stbtt_fontinfo, glyph_index: c_int);
// returns non-zero if nothing is drawn for this glyph

// STBTT_DEF stbtt_GetCodepointShape: c_int(info: *const stbtt_fontinfo, unicode_codepoint: c_int, vertices: *mut *mut stbtt_vertex);
// STBTT_DEF stbtt_GetGlyphShape: c_int(info: *const stbtt_fontinfo, glyph_index: c_int, vertices: *mut *mut stbtt_vertex);
// returns # of vertices and fills *vertices with the pointer to them
//   these are expressed in "unscaled" coordinates
//
// The shape is a series of contours. Each one starts with
// a STBTT_moveto, then consists of a series of mixed
// STBTT_lineto and STBTT_curveto segments. A lineto
// draws a line from previous endpoint to its x,y; a curveto
// draws a quadratic bezier from previous endpoint to
// its x,y, using cx,cy as the bezier control point.

// STBTT_DEF c_void stbtt_FreeShape(info: *const stbtt_fontinfo, vertices: *mut stbtt_vertex);
// frees the data allocated above

// STBTT_DEF c_ustbtt_FindSVGDoc: *mut c_char(info: *const stbtt_fontinfo, gl: c_int);
// STBTT_DEF stbtt_GetCodepointSVG: c_int(info: *const stbtt_fontinfo, unicode_codepoint: c_int, const char **svg);
// STBTT_DEF stbtt_GetGlyphSVG: c_int(info: *const stbtt_fontinfo, gl: c_int, const char **svg);
// fills svg with the character's SVG data.
// returns data size or 0 if SVG not found.

//////////////////////////////////////////////////////////////////////////////
//
// BITMAP RENDERING
//

// STBTT_DEF c_void stbtt_FreeBitmap(c_ubitmap: *mut c_char, c_void *userdata);
// frees the bitmap allocated below

// STBTT_DEF c_ustbtt_GetCodepointBitmap: *mut c_char(info: *const stbtt_fontinfo,scale_x: c_float,scale_y: c_float, codepoint: c_int, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int);
// allocates a large-enough single-channel 8bpp bitmap and renders the
// specified character/glyph at the specified scale into it, with
// antialiasing. 0 is no coverage (transparent), 255 is fully covered (opaque).
// *width & *height are filled out with the width & height of the bitmap,
// which is stored left-to-right, top-to-bottom.
//
// xoff/yoff are the offset it pixel space from the glyph origin to the top-left of the bitmap

// STBTT_DEF c_ustbtt_GetCodepointBitmapSubpixel: *mut c_char(info: *const stbtt_fontinfo,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, codepoint: c_int, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int);
// the same as stbtt_GetCodepoitnBitmap, but you can specify a subpixel
// shift for the character

// STBTT_DEF c_void stbtt_MakeCodepointBitmap(info: *const stbtt_fontinfo, c_uoutput: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float, codepoint: c_int);
// the same as stbtt_GetCodepointBitmap, but you pass in storage for the bitmap
// in the form of 'output', with row spacing of 'out_stride' bytes. the bitmap
// is clipped to out_w/out_h bytes. Call stbtt_GetCodepointBitmapBox to get the
// width and height and positioning info for it first.

// STBTT_DEF c_void stbtt_MakeCodepointBitmapSubpixel(info: *const stbtt_fontinfo, c_uoutput: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, codepoint: c_int);
// same as stbtt_MakeCodepointBitmap, but you can specify a subpixel
// shift for the character

// STBTT_DEF c_void stbtt_MakeCodepointBitmapSubpixelPrefilter(info: *const stbtt_fontinfo, c_uoutput: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, oversample_x: c_int, oversample_y: c_int, c_float *sub_x, c_float *sub_y, codepoint: c_int);
// same as stbtt_MakeCodepointBitmapSubpixel, but prefiltering
// is performed (see stbtt_PackSetOversampling)

// STBTT_DEF c_void stbtt_GetCodepointBitmapBox(font: *const stbtt_fontinfo, codepoint: c_int,scale_x: c_float,scale_y: c_float, ix0: *mut c_int, iy0: *mut c_int, ix1: *mut c_int, iy1: *mut c_int);
// get the bbox of the bitmap centered around the glyph origin; so the
// bitmap width is ix1-ix0, height is iy1-iy0, and location to place
// the bitmap top left is (leftSideBearing*scale,iy0).
// (Note that the bitmap uses y-increases-down, but the shape uses
// y-increases-up, so CodepointBitmapBox and CodepointBox are inverted.)

// STBTT_DEF c_void stbtt_GetCodepointBitmapBoxSubpixel(font: *const stbtt_fontinfo, codepoint: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, ix0: *mut c_int, iy0: *mut c_int, ix1: *mut c_int, iy1: *mut c_int);
// same as stbtt_GetCodepointBitmapBox, but you can specify a subpixel
// shift for the character

// the following functions are equivalent to the above functions, but operate
// on glyph indices instead of Unicode codepoints (for efficiency)
// STBTT_DEF c_ustbtt_GetGlyphBitmap: *mut c_char(info: *const stbtt_fontinfo,scale_x: c_float,scale_y: c_float, glyph: c_int, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int);
// STBTT_DEF c_ustbtt_GetGlyphBitmapSubpixel: *mut c_char(info: *const stbtt_fontinfo,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, glyph: c_int, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int);
// STBTT_DEF c_void stbtt_MakeGlyphBitmap(info: *const stbtt_fontinfo, c_uoutput: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float, glyph: c_int);
// STBTT_DEF c_void stbtt_MakeGlyphBitmapSubpixel(info: *const stbtt_fontinfo, c_uoutput: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, glyph: c_int);
// STBTT_DEF c_void stbtt_MakeGlyphBitmapSubpixelPrefilter(info: *const stbtt_fontinfo, c_uoutput: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, oversample_x: c_int, oversample_y: c_int, c_float *sub_x, c_float *sub_y, glyph: c_int);
// STBTT_DEF c_void stbtt_GetGlyphBitmapBox(font: *const stbtt_fontinfo, glyph: c_int,scale_x: c_float,scale_y: c_float, ix0: *mut c_int, iy0: *mut c_int, ix1: *mut c_int, iy1: *mut c_int);
// STBTT_DEF c_void stbtt_GetGlyphBitmapBoxSubpixel(font: *const stbtt_fontinfo, glyph: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, ix0: *mut c_int, iy0: *mut c_int, ix1: *mut c_int, iy1: *mut c_int);



// rasterize a shape with quadratic beziers into a bitmap
// STBTT_DEF c_void stbtt_Rasterize(stbtt__bitmap *result,        // 1-channel bitmap to draw intoflatness_in_pixels: c_float,     // allowable error of curve in pixels
//                                vertices: *mut stbtt_vertex,       // array of vertices defining shape
//                                num_verts: c_int,                // number of vertices in above arrayscale_x: c_float, c_float scale_y, // scale applied to input verticesshift_x: c_float, c_float shift_y, // translation applied to input vertices
//                                x_off: c_int, y_off: c_int,         // another translation applied to input
//                                invert: c_int,                   // if non-zero, vertically flip shape
//                                c_void *userdata);              // context for to STBTT_MALLOC

//////////////////////////////////////////////////////////////////////////////
//
// Signed Distance Function (or Field) rendering

// STBTT_DEF c_void stbtt_FreeSDF(c_ubitmap: *mut c_char, c_void *userdata);
// frees the SDF bitmap allocated below

// STBTT_DEF c_uchar * stbtt_GetGlyphSDF(info: *const stbtt_fontinfo,scale: c_float, glyph: c_int, padding: c_int, c_uchar onedge_value,pixel_dist_scale: c_float, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int);
// STBTT_DEF c_uchar * stbtt_GetCodepointSDF(info: *const stbtt_fontinfo,scale: c_float, codepoint: c_int, padding: c_int, c_uchar onedge_value,pixel_dist_scale: c_float, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int);
// These functions compute a discretized SDF field for a single character, suitable for storing
// in a single-channel texture, sampling with bilinear filtering, and testing against
// larger than some threshold to produce scalable fonts.
//        info              --  the font
//        scale             --  controls the size of the resulting SDF bitmap, same as it would be creating a regular bitmap
//        glyph/codepoint   --  the character to generate the SDF for
//        padding           --  extra "pixels" around the character which are filled with the distance to the character (not 0),
//                                 which allows effects like bit outlines
//        onedge_value      --  value 0-255 to test the SDF against to reconstruct the character (i.e. the isocontour of the character)
//        pixel_dist_scale  --  what value the SDF should increase by when moving one SDF "pixel" away from the edge (on the 0..255 scale)
//                                 if positive, > onedge_value is inside; if negative, < onedge_value is inside
//        width,height      --  output height & width of the SDF bitmap (including padding)
//        xoff,yoff         --  output origin of the character
//        return value      --  a 2D array of bytes 0..255, width*height in size
//
// pixel_dist_scale & onedge_value are a scale & bias that allows you to make
// optimal use of the limited 0..255 for your application, trading off precision
// and special effects. SDF values outside the range 0..255 are clamped to 0..255.
//
// Example:
//      scale = stbtt_ScaleForPixelHeight(22)
//      padding = 5
//      onedge_value = 180
//      pixel_dist_scale = 180/5.0 = 36.0
//
//      This will create an SDF bitmap in which the character is about 22 pixels
//      high but the whole bitmap is about 22+5+5=32 pixels high. To produce a filled
//      shape, sample the SDF at each pixel and fill the pixel if the SDF value
//      is greater than or equal to 180/255. (You'll actually want to antialias,
//      which is beyond the scope of this example.) Additionally, you can compute
//      offset outlines (e.g. to stroke the character border inside & outside,
//      or only outside). For example, to fill outside the character up to 3 SDF
//      pixels, you would compare against (180-36.0*3)/255 = 72/255. The above
//      choice of variables maps a range from 5 pixels outside the shape to
//      2 pixels inside the shape to 0..255; this is intended primarily for apply
//      outside effects only (the interior range is needed to allow proper
//      antialiasing of the font at *smaller* sizes)
//
// The function computes the SDF analytically at each SDF pixel, not by e.g.
// building a higher-res bitmap and approximating it. In theory the quality
// should be as high as possible for an SDF of this size & representation, but
// unclear if this is true in practice (perhaps building a higher-res bitmap
// and computing from that can allow drop-out prevention).
//
// The algorithm has not been optimized at all, so expect it to be slow
// if computing lots of characters or very large sizes.



//////////////////////////////////////////////////////////////////////////////
//
// Finding the right font...
//
// You should really just solve this offline, keep your own tables
// of what font is what, and don't try to get it out of the .ttf file.
// That's because getting it out of the .ttf file is really hard, because
// the names in the file can appear in many possible encodings, in many
// possible languages, and e.g. if you need a case-insensitive comparison,
// the details of that depend on the encoding & language in a complex way
// (actually underspecified in truetype, but also gigantic).
//
// But you can use the provided functions in two possible ways:
//     stbtt_FindMatchingFont() will use *case-sensitive* comparisons on
//             unicode-encoded names to try to find the font you want;
//             you can run this before calling stbtt_InitFont()
//
//     stbtt_GetFontNameString() lets you get any of the various strings
//             from the file yourself and do your own comparisons on them.
//             You have to have called stbtt_InitFont() first.


// STBTT_DEF stbtt_FindMatchingFont: c_int(fontdata: *const c_uchar, const name: *mut c_char, flags: c_int);
// returns the offset (not index) of the font that matches, or -1 if none
//   if you use STBTT_MACSTYLE_DONTCARE, use a font name like "Arial Bold".
//   if you use any other flag, use a font name like "Arial"; this checks
//     the 'macStyle' header field; i don't know if fonts set this consistently
// #define STBTT_MACSTYLE_DONTCARE     0
// #define STBTT_MACSTYLE_BOLD         1
// #define STBTT_MACSTYLE_ITALIC       2
// #define STBTT_MACSTYLE_UNDERSCORE   4
// #define STBTT_MACSTYLE_NONE         8   // <= not same as 0, this makes us check the bitfield is 0

// STBTT_DEF stbtt_CompareUTF8toUTF16_bigendian: c_int(const s1: *mut c_char, len1: c_int, const s2: *mut c_char, len2: c_int);
// returns 1/0 whether the first string interpreted as utf8 is identical to
// the second string interpreted as big-endian utf16... useful for strings from next func

// STBTT_DEF const stbtt_GetFontNameString: *mut c_char(font: *const stbtt_fontinfo, length: *mut c_int, platformID: c_int, encodingID: c_int, languageID: c_int, nameID: c_int);
// returns the string (which may be big-endian double byte, e.g. for unicode)
// and puts the length in bytes in *length.
//
// some of the values for the IDs are below; for more see the truetype spec:
//     http://developer.apple.com/textfonts/TTRefMan/RM06/Chap6name.html
//     http://www.microsoft.com/typography/otspec/name.htm




// #ifdef __cplusplus
// }
// #endif

// #endif // __STB_INCLUDE_STB_TRUETYPE_H__

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
////
////   IMPLEMENTATION
////
////

// #ifdef STB_TRUETYPE_IMPLEMENTATION

// #ifndef STBTT_MAX_OVERSAMPLE
// #define STBTT_MAX_OVERSAMPLE   8
// #endif

// #if STBTT_MAX_OVERSAMPLE > 255
// #error "STBTT_MAX_OVERSAMPLE cannot be > 255"
// #endif

// typedef stbtt__test_oversample_pow2: c_int[(STBTT_MAX_OVERSAMPLE & (STBTT_MAX_OVERSAMPLE-1)) == 0 ? 1 : -1];

// #ifndef STBTT_RASTERIZER_VERSION
// #define STBTT_RASTERIZER_VERSION 2
// #endif

// #ifdef _MSC_VER
// #define STBTT__NOTUSED(v)  (void)(v)
// #else
// #define STBTT__NOTUSED(v)  (void)sizeof(v)
// #endif

//////////////////////////////////////////////////////////////////////////
//
// helpers: stbtt__buf to parse data from file
//

pub fn stbtt__buf_get8(b: *mut stbtt__buf) -> u8 {
    if b.cursor >= b.size {
        return 0;
    }
    let out = b.data[b.cursor];
    b.cursor += 1;
    out
}

pub fn stbtt__buf_peek8(b: *mut stbtt__buf) -> u8
{
   if (b.cursor >= b.size) {
       return 0;
   }
   return b.data[b.cursor];
}

pub  fn stbtt__buf_seek(b: *mut stbtt__buf, o: size_t)
{
   // STBTT_assert(!(o > b.size || o < 0));
   b.cursor = if o > b.size || o < 0 { b.size} else { o};
}

pub  fn stbtt__buf_skip(b: *mut stbtt__buf, o: size_t)
{
   stbtt__buf_seek(b, b.cursor + o);
}

pub fn stbtt__buf_get(b: *mut stbtt__buf, n: size_t) -> u32 {
    let mut v: u32 = 0;
    let mut i: c_int = 0;
    // STBTT_assert(n >= 1 && n <= 4);
    // for (i = 0; i < n; i++)
    for i in 0..n {
        v = (v << 8) | stbtt__buf_get8(b);
    }
    return v;
}

pub fn stbtt__new_buf(p: *const c_void, size: size_t) -> stbtt__buf
{
   let mut r = stbtt__buf::default();
   // STBTT_assert(size < 0x40000000);
   r.data =  p;
   r.size = size;
   r.cursor = 0;
   return r;
}

// #define stbtt__buf_get16(b)  stbtt__buf_get((b), 2)
// #define stbtt__buf_get32(b)  stbtt__buf_get((b), 4)

pub fn stbtt__buf_range(b: *mut stbtt__buf, o: size_t, s: size_t) -> stbtt__buf {
    // r: stbtt__buf = stbtt__new_buf(None, 0);
    let mut r = stbtt__buf::default();
    if o < 0 || s < 0 || o > b.size || s > b.size - o { return r; }
    r.data = b.data + o;
    r.size = s;
    return r;
}

pub fn stbtt__cff_get_index(b: *mut stbtt__buf) -> stbtt__buf {
    // count: c_int, start, offsize;
    let mut count: size_t = 0;
    let mut start: size_t = 0;
    let mut offsize: size_t = 0;
    start = b.cursor;
    count = stbtt__buf_get16(b);
    if count {
        offsize = stbtt__buf_get8(b) as size_t;
        // STBTT_assert(offsize >= 1 && offsize <= 4);
        stbtt__buf_skip(b, offsize * count);
        stbtt__buf_skip(b, (stbtt__buf_get(b, offsize) - 1) as size_t);
    }
    return stbtt__buf_range(b, start, b.cursor - start);
}

pub fn stbtt__cff_int(b: *mut stbtt__buf) -> u32 {
    let b0: u32 = stbtt__buf_get8(b) as u32;
    if b0 >= 32 && b0 <= 246 { return b0 - 139; } else if b0 >= 247 && b0 <= 250 { return (b0 - 247) * 256 + stbtt__buf_get8(b) + 108; } else if b0 >= 251 && b0 <= 254 { return -(b0 - 251) * 256 - stbtt__buf_get8(b) - 108; } else if b0 == 28 { return stbtt__buf_get16(b); } else if b0 == 29 { return stbtt__buf_get32(b); }
    // STBTT_assert(0);
    return 0;
}

pub fn stbtt__cff_skip_operand(b: *mut stbtt__buf) {
   let mut v: c_int = 0;
    let mut b0: u32 = stbtt__buf_peek8(b) as u32;
   // STBTT_assert(b0 >= 28);
   if b0 == 30 {
      stbtt__buf_skip(b, 1);
      while b.cursor < b.size {
         v = stbtt__buf_get8(b) as c_int;
         if (v & 0x0f32) == 0xF || (v >> 4) == 0x0f32 {
             break;
         }
      }
   } else {
      stbtt__cff_int(b);
   }
}

pub fn stbtt__dict_get(b: *mut stbtt__buf, key: c_int) -> stbtt__buf
{
   stbtt__buf_seek(b, 0);
   while b.cursor < b.size {
      // let start: c_int = b.cursor, end, op;
      let mut start = b.cursor;
       let mut end = b.cursor;
       let mut op = b.cursor;
       while stbtt__buf_peek8(b) >= 28 {
          stbtt__cff_skip_operand(b);
      }
      end = b.cursor;
      op = stbtt__buf_get8(b) as size_t;
      if op == 12 { op = (stbtt__buf_get8(b) | 0x100) as size_t; }
      if op == key as size_t { return stbtt__buf_range(b, start, end - start); }
   }
   return stbtt__buf_range(b, 0, 0);
}

pub fn stbtt__dict_get_ints(b: *mut stbtt__buf, key: c_int, outcount: c_int, out: *mut size_t) {
    let mut i: c_int = 0;
    let mut operands = stbtt__dict_get(b, key);
    // for (i = 0; i < outcount && operands.cursor < operands.size; i++)
    for i in 0..outcount {
        out[i] = stbtt__cff_int(&mut operands);
        if operands_cursor >= operands.size {
            break;
        }
    }
}

pub fn stbtt__cff_index_count(b: *mut stbtt__buf) -> c_int
{
   stbtt__buf_seek(b, 0);
   return stbtt__buf_get16(b);
}

pub fn stbtt__cff_index_get(mut b: stbtt__buf, i: c_int) -> stbtt__buf
{
   // count: c_int, offsize, start, end;
   let mut count: size_t = 0;
    let mut offsize: size_t = 0;
    let mut start: size_t = 0;
    let mut end: size_t = 0;
    stbtt__buf_seek(&mut b, 0);
   count = stbtt__buf_get16(&b);
   offsize = stbtt__buf_get8(&mut b) as size_t;
   // STBTT_assert(i >= 0 && i < count);
   // STBTT_assert(offsize >= 1 && offsize <= 4);
   stbtt__buf_skip(&mut b, offsize);
   start = stbtt__buf_get(&mut b, offsize) as size_t;
   end = stbtt__buf_get(&mut b, offsize) as size_t;
   return stbtt__buf_range(&mut b, 2+(count1)*offsize+start, end - start);
}

//////////////////////////////////////////////////////////////////////////
//
// accessors to parse data from file
//

// on platforms that don't allow misaligned reads, if we want to allow
// truetype fonts that aren't padded to alignment, define ALLOW_UNALIGNED_TRUETYPE

// #define ttBYTE(p)     (* (stbtt_uint8 *) (p))
// #define ttCHAR(p)     (* (stbtt_int8 *) (p))
// #define ttFixed(p)    ttLONG(p)

pub fn ttUSHORT(p: *mut u8) -> u16 { return p[0]*256 + p[1]; }
pub fn ttSHORT(p: *mut u8) -> i16   { return p[0]*256 + p[1]; }
pub fn ttULONG(p: *mut u8)  -> c_ulong { return (p[0]<<24) + (p[1]<<16) + (p[2]<<8) + p[3]; }
pub fn  ttLONG(p: *mut u8) -> i32   { return (p[0]<<24) + (p[1]<<16) + (p[2]<<8) + p[3]; }

// #define stbtt_tag4(p,c0,c1,c2,c3) ((p)[0] == (c0) && (p)[1] == (c1) && (p)[2] == (c2) && (p)[3] == (c3))
// #define stbtt_tag(p,str)           stbtt_tag4(p,str[0],str[1],str[2],str[3])

pub fn stbtt__isfont(font: *mut u8) -> bool
{
   // check the version number
   if stbtt_tag4(font, '1', 0, 0, 0) { return true; }// TrueType 1
   if stbtt_tag(font, "typ1") { return true; }// TrueType with type 1 font -- we don't support this!
   if stbtt_tag(font, "OTTO") { return true; } // OpenType with CFF
   if stbtt_tag4(font, 0, 1, 0, 0) { return true; } // OpenType 1.0
   if stbtt_tag(font, "true") { return true; } // Apple specification for TrueType fonts
   false
}

// @OPTIMIZE: binary search
pub fn stbtt__find_table(data: *mut u8, fontstart: u32, tag: *const c_char) -> u32 {
    let num_tables = ttUSHORT(data + fontstart4);
    let tabledir: u32 = fontstart + 12;
    // i: i32;
    // for (i=0; i < num_tables; ++i)
    for i in 0..num_tables {
        let loc = tabledir + i;
        if stbtt_tag(data + loc0, tag) {
            return ttULONG(data + loc8) as u32;
        }
    }
    return 0;
}

pub fn stbtt_GetFontOffsetForIndex_internal(c_ufont_collection: *const c_uchar, index: c_int) -> c_int {
    // if it's just a font, there's only one valid index
    if stbtt__isfont(font_collection) {
        return if index == 0 {
            0
        } else { -1 };
    }

    // check if it's a TTC
    if stbtt_tag(font_collection, "ttcf") {
        // version 1?
        if ttULONG(font_collection4) == 0x00010000 || ttULONG(font_collection4) == 0x00020000 {
            let n = ttLONG(font_collection8);
            if index >= n {
                return -1;
            }
            return ttULONG(font_collection12 + index4) as c_int;
        }
    }
    return -1;
}

pub fn stbtt_GetNumberOfFonts_internal(c_ufont_collection: *mut c_char) -> c_int
{
   // if it's just a font, there's only one valid font
   if stbtt__isfont(font_collection) {
       return 1;
   }

   // check if it's a TTC
   if stbtt_tag(font_collection, "ttcf") {
      // version 1?
      if ttULONG(font_collection4) == 0x00010000 || ttULONG(font_collection4) == 0x00020000 {
         return ttLONG(font_collection8);
      }
   }
   return 0;
}

pub fn stbtt__get_subrs(mut cff: stbtt__buf, mut fontdict: stbtt__buf) -> stbtt__buf {
    let mut subrsoff: size_t = 0;
    let private_loc: [size_t; 2] = [0; 2];
    let mut pdict = stbtt__buf::default();
    stbtt__dict_get_ints(&mut fontdict, 18, 2, private_loc.as_mut_ptr());
    if !private_loc[1] || !private_loc[0] { return stbtt__new_buf(None, 0); }
    pdict = stbtt__buf_range(&mut cff, private_loc[1], private_loc[0]);
    stbtt__dict_get_ints(&mut pdict, 19, 1, &mut subrsoff);
    if !subrsoff { return stbtt__new_buf(None, 0); }
    stbtt__buf_seek(&mut cff, (private_loc[1] + subrsoff) as size_t);
    return stbtt__cff_get_index(&mut cff);
}

// since most people won't use this, find this table the first time it's needed
pub unsafe fn stbtt__get_svg(info: *mut stbtt_fontinfo) -> c_int
{
   let mut t: u32 = 0;
   if info.svg < 0 {
      t = stbtt__find_table(info.data, info.fontstart, str_to_const_c_char_ptr("SVG "));
      if t {
         let mut offset: u32 = ttULONG(info.data + t + 2) as u32;
         info.svg = t + offset;
      } else {
         info.svg = 0;
      }
   }
   return info.svg;
}

pub unsafe fn stbtt_InitFont_internal(info: *mut stbtt_fontinfo, c_udata: *const c_uchar, fontstart: u32) -> c_int
{
   // cmap: u32, t;
   let mut cmap: u32 = 0;
    let mut t: u32 = 0;
    // i: i32,numTables;
    let mut i = 0i32;
    let mut numtables = 0i32;

   info.data = data;
   info.fontstart = fontstart;
   info.cff = stbtt__new_buf(None, 0);

   cmap = stbtt__find_table(data, fontstart, str_to_const_c_char_ptr("cmap"));       // required
   info.loca = stbtt__find_table(data, fontstart, str_to_const_c_char_ptr("loca")); // required
   info.head = stbtt__find_table(data, fontstart, str_to_const_c_char_ptr("head")); // required
   info.glyf = stbtt__find_table(data, fontstart, str_to_const_c_char_ptr("glyf")); // required
   info.hhea = stbtt__find_table(data, fontstart, str_to_const_c_char_ptr("hhea")); // required
   info.hmtx = stbtt__find_table(data, fontstart, str_to_const_c_char_ptr("hmtx")); // required
   info.kern = stbtt__find_table(data, fontstart, str_to_const_c_char_ptr("kern")); // not required
   info.gpos = stbtt__find_table(data, fontstart, str_to_const_c_char_ptr("GPOS")); // not required

   if !cmap || !info.head || !info.hhea || !info.hmtx {
       return 0;
   }
   if (info.glyf) {
      // required for truetype
      if (!info.loca) { return 0; }
   } else {
      // initialization for CFF / Type2 fonts (OT0f32)
      // b: stbtt__buf, topdict, topdictidx;
      let mut b = stbtt__buf::default();
       let mut topdict = stbtt__buf::default();
       let mut topdictidx = stbtt__buf::default();
       // cstype: u32 = 2, charstrings = 0, fdarrayoff = 0, fdselectoff = 0;
      let mut cstype: size_t = 2;
       let mut charstrings: size_t = 0;
       let mut fdarrayoff: size_t = 0;
       let mut fdselectoff: size_t = 0;
       cff: u32;

      cff = stbtt__find_table(data, fontstart, str_to_const_c_char_ptr("CFF "));
      if (!cff) { return 0; }

      info.fontdicts = stbtt__new_buf(None, 0);
      info.fdselect = stbtt__new_buf(None, 0);

      // @TODO this should use size from table (not 512MB)
      info.cff = stbtt__new_buf(data+cff, 512*1024*1024);
      b = info.cff;

      // read the header
      stbtt__buf_skip(&mut b, 2);
      stbtt__buf_seek(&mut b, stbtt__buf_get8(&mut b) as size_t); // hdrsize

      // @TODO the name INDEX could list multiple fonts,
      // but we just use the first one.
      stbtt__cff_get_index(&mut b);  // name INDEX
      topdictidx = stbtt__cff_get_index(&mut b);
      topdict = stbtt__cff_index_get(topdictidx, 0);
      stbtt__cff_get_index(&mut b);  // string INDEX
      info.gsubrs = stbtt__cff_get_index(&mut b);

      stbtt__dict_get_ints(&mut topdict, 17, 1, &mut charstrings);
      stbtt__dict_get_ints(&mut topdict, 0x100 | 6, 1, &mut cstype);
      stbtt__dict_get_ints(&mut topdict, 0x100 | 36, 1, &mut fdarrayoff);
      stbtt__dict_get_ints(&mut topdict, 0x100 | 37, 1, &mut fdselectoff);
      info.subrs = stbtt__get_subrs(b, topdict);

      // we only support Type 2 charstrings
      if cstype != 2 { return 0; }
      if charstrings == 0 { return 0; }

      if fdarrayoff {
         // looks like a CID font
         if !fdselectoff { return 0; }
         stbtt__buf_seek(&mut b, fdarrayoff as size_t);
         info.fontdicts = stbtt__cff_get_index(&mut b);
         info.fdselect = stbtt__buf_range(&mut b, fdselectoff, (b.size - fdselectoff));
      }

      stbtt__buf_seek(&mut b, charstrings as size_t);
      info.charstrings = stbtt__cff_get_index(&mut b);
   }

   t = stbtt__find_table(data, fontstart, str_to_const_c_char_ptr("maxp"));
   if t {
       info.numGlyphs = ttUSHORT(data + t4);
   }
   else {
       info.numGlyphs = 0xffff;
   }

   info.svg = -1;

   // find a cmap encoding table we understand *now* to avoid searching
   // later. (todo: could make this installable)
   // the same regardless of glyph.
   numTables = ttUSHORT(data + cmap + 2);
   info.index_map = 0;
   // for (i=0; i < numTables; ++i)
   for i in 0 .. numTables
    {
      let mut encoding_record: u32 = cmap + 4 + 8 * i;
      // find an encoding we understand:
        match ttUSHORT(data + encoding_record) {
            STBTT_PLATFORM_ID_MICROSOFT => {
                match ttUSHORT(data + encoding_record2) {
                    STBTT_MS_EID_UNICODE_BMP | STBTT_MS_EID_UNICODE_FULL => {
                        // MS/Unicode
                        info.index_map = cmap + ttULONG(data + encoding_record4);
                    }
                    _ => {}
                }
            }

            STBTT_PLATFORM_ID_UNICODE => {
                // Mac/iOS has these
                // all the encodingIDs are unicode, so we don't bother to check it
                info.index_map = cmap + ttULONG(data + encoding_record4);
            }
            _ => {}
        }
   }
   if info.index_map == 0 {
       return 0;
   }

   info.indexToLocFormat = ttUSHORT(data+info.head + 50);
   return 1;
}

pub fn stbtt_FindGlyphIndex(info: *const stbtt_fontinfo, unicode_codepoint: c_int) -> c_int
{
   let mut data: *mut u8 = info.data;
   let mut index_map: u32 = info.index_map;

   let mut format = ttUSHORT(data + index_map + 0);
   if format == 0 { // apple byte encoding
      let bytes = ttUSHORT(data + index_map + 2);
      if unicode_codepoint < (bytes - 6) as c_int {
          return ttBYTE(data + index_map + 6 + unicode_codepoint);
      }
      return 0;
   } else if format == 6 {
      first: u32 = ttUSHORT(data + index_map + 6) as u32;
      count: u32 = ttUSHORT(data + index_map + 8) as u32;
      if unicode_codepoint >= first && unicode_codepoint < first+count {
           return ttUSHORT(data + index_map + 10 + (unicode_codepoint - first) * 2) as c_int;
       }
      return 0;
   } else if format == 2 {
      STBTT_assert(0); // @TODO: high-byte mapping for japanese/chinese/korean
      return 0;
   } else if format == 4 { // standard mapping for windows fonts: binary search collection of ranges
      let mut  segcount = ttUSHORT(data+index_map6) >> 1;
      let mut   searchRange = ttUSHORT(data+index_map8) >> 1;
      let mut   entrySelector = ttUSHORT(data+index_map10);
      let mut   rangeShift = ttUSHORT(data+index_map12) >> 1;

      // do a binary search of the segments
      endCount: u32 = index_map + 14;
      search: u32 = endCount;

      if unicode_codepoint > 0xffff {
          return 0;
      }

      // they lie from endCount .. endCount + segCount
      // but searchRange is the nearest power of two, so...
      if unicode_codepoint >= ttUSHORT(data + search + *rangeShift2) as c_int {
          search += *rangeShift2;
      }

      // now decrement to bias correctly to find smallest
      search -= 2;
      while entrySelector {
         let mut end = 0u16;
         searchRange >>= 1;
         end = ttUSHORT(data + search + *searchRange2);
         if unicode_codepoint > end as c_int {
             search += *searchRange2;
         }
         entrySelector -= 1;
      }
      search += 2;

      {
         // stbtt_uint16 offset, start, last;
         let mut offset = 0u16;
          let mut start = 0u16;
          let mut last = 0u16;
          let mut item =  ((search - endCount) >> 1);

         start = ttUSHORT(data + index_map + 14 + *segcount2 + 2 + *item);
         last = ttUSHORT(data + endCount + *item);
         if unicode_codepoint < start as c_int || unicode_codepoint > last as c_int {
             return 0;
         }

         offset = ttUSHORT(data + index_map + 14 + *segcount6 + 2 + *item);
         if offset == 0 {
             return unicode_codepoint + ttSHORT(data + index_map + 14 + *segcount4 + 2 + *item);
         }

         return ttUSHORT(data + offset + (unicode_codepoint - start) * 2 + index_map + 14 + segcount * 6 + 2 + 2 * item) as c_int;
      }
   } else if format == 12 || format == 13 {
      let ngroups = ttULONG(data+index_map12);
      // low: i32,high;
      let mut low = 0i32;
       let mut high = 0i32;
       low = 0; high = ngroups as i32;
      // Binary search the right group.
      while low < high {
         let mut mid = low + ((high-low) >> 1); // rounds down, so low <= mid < high
         let mut start_char = ttULONG(data+index_map16+mid*12);
         let mut end_char = ttULONG(data+index_map16+mid*124);
         if unicode_codepoint < start_char as c_int {
             high = mid;
         }
         else if unicode_codepoint > end_char as c_int {
             low = mid1;
         }
         else {
            let mut start_glyph = ttULONG(data+index_map16+mid*128);
            if format == 12 {
                return (start_glyph + unicode_codepoint - start_char) as c_int;
            }
                // format == 13
            else {
                return start_glyph as c_int;
            }
         }
      }
      return 0; // not found
   }
   // @TODO
   // STBTT_assert(0);
   return 0;
}

pub unsafe fn stbtt_GetCodepointShape(info: *const stbtt_fontinfo, unicode_codepoint: c_int, vertices: *mut *mut stbtt_vertex) -> c_int
{
   return stbtt_GetGlyphShape(info, stbtt_FindGlyphIndex(info, unicode_codepoint), vertices);
}

pub fn stbtt_setvertex(v: &mut stbtt_vertex, vertex_type: c_int, x: i32, y: i32, cx: i32, cy: i32)
{
   v.vertex_type = vertex_type as c_uchar;
   v.x = x as stbtt_vertex_type;
   v.y = y as stbtt_vertex_type;
   v.cx = cx as stbtt_vertex_type;
   v.cy = cy as stbtt_vertex_type;
}
pub fn stbtt__GetGlyfOffset(info: *const stbtt_fontinfo, glyph_index: c_int) -> c_int
{
   // g1: c_int,g2;
    let mut g1 = 0i32;
    let mut g2 = 0i32;

   // STBTT_assert(!info.cff.size);

   if glyph_index >= info.numGlyphs { return -1; }// glyph index out of range
   if info.indexToLocFormat >= 2 { return -1; }// unknown index->glyph map format

   if info.indexToLocFormat == 0 {
      g1 = info.glyf + ttUSHORT(info.data + info.loca + glyph_index * 2) * 2;
      g2 = info.glyf + ttUSHORT(info.data + info.loca + glyph_index * 2 + 2) * 2;
   } else {
      g1 = info.glyf + ttULONG (info.data + info.loca + glyph_index * 4);
      g2 = info.glyf + ttULONG (info.data + info.loca + glyph_index * 4 + 4);
   }

   return if g1==g2 { -1 } else { g1 }; // if length is 0, return -1
}

// static stbtt__GetGlyphInfoT2: c_int(info: *const stbtt_fontinfo, glyph_index: c_int, x0: *mut c_int, y0: *mut c_int, x1: *mut c_int, y1: *mut c_int);

pub unsafe fn stbtt_GetGlyphBox(info: *const stbtt_fontinfo, glyph_index: c_int, x0: *mut c_int, y0: *mut c_int, x1: *mut c_int, y1: *mut c_int) -> c_int
{
   if info.cff.size {
      stbtt__GetGlyphInfoT2(info, glyph_index, x0, y0, x1, y1);
   } else {
      let g: c_int = stbtt__GetGlyfOffset(info, glyph_index);
      if (g < 0) { return 0; }

      if (x0) { *x0 = ttSHORT(info.data + g + 2) as c_int; }
      if (y0) { *y0 = ttSHORT(info.data + g + 4) as c_int; }
      if (x1) { *x1 = ttSHORT(info.data + g + 6) as c_int; }
      if (y1) { *y1 = ttSHORT(info.data + g + 8) as c_int; }
   }
   return 1;
}

pub unsafe fn stbtt_GetCodepointBox(info: *const stbtt_fontinfo, codepoint: c_int, x0: *mut c_int, y0: *mut c_int, x1: *mut c_int, y1: *mut c_int) -> c_int
{
   return stbtt_GetGlyphBox(info, stbtt_FindGlyphIndex(info,codepoint), x0,y0,x1,y1);
}

pub unsafe fn stbtt_IsGlyphEmpty(info: *const stbtt_fontinfo, glyph_index: c_int) -> bool
{
   let mut numberOfContours = 0i16;
   let mut g: c_int = 0;
   if info.cff.size {
       return stbtt__GetGlyphInfoT2(info, glyph_index, None, None, None, null_mut()) == 0;
   }
   g = stbtt__GetGlyfOffset(info, glyph_index);
   if (g < 0) { return true; }
   numberOfContours = ttSHORT(info.data + g);
   return numberOfContours == 0;
}

pub fn stbtt__close_shape(vertices: *mut stbtt_vertex, mut num_vertices: c_int, was_off: c_int, start_off: c_int,
    sx: i32, sy: i32, scx: i32, scy: i32, cx: i32, cy: i32) -> c_int
{
   if (start_off) {
      if (was_off) {
          stbtt_setvertex(&mut vertices[num_vertices], STBTT_vcurve, (cx + scx) >> 1, (cy + scy) >> 1, cx, cy);
          num_vertices += 1;
      }
      stbtt_setvertex(&mut vertices[num_vertices], STBTT_vcurve, sx, sy, scx, scy);
       num_vertices += 1;
   } else {
      if (was_off) {
          stbtt_setvertex(&mut vertices[num_vertices], STBTT_vcurve, sx, sy, cx, cy);
          num_vertices += 1;
      }
      else {
          stbtt_setvertex(&mut vertices[num_vertices], STBTT_vline, sx, sy, 0, 0);
          num_vertices += 1;
      }
   }
   return num_vertices;
}

pub unsafe fn stbtt__GetGlyphShapeTT(info: *const stbtt_fontinfo, glyph_index: c_int, pvertices: *mut *mut stbtt_vertex) -> c_int
{
   let mut numberOfContours = 0i16;
   let mut endPtsOfContours: *mut u8 = None;
   let mut data: *mut u8 = info.data;
   let mut vertices: *mut stbtt_vertex= None;
   let mut num_vertices: c_int = 0;
   let g: c_int = stbtt__GetGlyfOffset(info, glyph_index);

   *pvertices = None;

   if g < 0 {
       return 0;
   }
   numberOfContours = ttSHORT(data + g);

   if numberOfContours > 0 {
      // flags: u8=0,flagcount;
      let mut flags = 0u8;
       let mut flagcount = 0u8;
       // ins: i32, i,j=0,m,n, next_move, was_off=0, off, start_off=0;
      let mut i = 0i32;
       let mut j = 0i32;
       let mut m = 0i32;
       let mut n = 0i32;
       let mut next_move = 0i32;
       let mut was_off = 0i32;
       let mut start_off = 0i32;
       let mut off = 0i32;
       // x: i32,y,cx,cy,sx,sy, scx,scy;
      let mut x = 0i32;
       let mut y = 0i32;
       let mut cx = 0i32;
       let mut cy = 0i32;
       let mut sx = 0i32;
       let mut sy = 0i32;
       let mut scx = 0i32;
       let mut scy = 0i32;
       let mut points: *mut u8 = None;
      endPtsOfContours = (data + g + 10);
      ins = ttUSHORT(data + g + 10 + numberOfContours * 2);
      points = data + g + 10 + numberOfContours * 2 + 2 + ins;

      n = (1 + ttUSHORT(endPtsOfContours + numberOfContours * 2 - 2)) as i32;

      m = n + 2*numberOfContours;  // a loose bound on how many vertices we might need
      vertices =  libc::malloc(m * sizeof(vertices[0]));
      if vertices.is_null() {
          return 0;
      }

      next_move = 0;
      flagcount=0;

      // in first pass, we load uninterpreted data into the allocated array
      // above, shifted to the end of the array so we won't overwrite it when
      // we create our final data starting from the front

      off = m - n; // starting offset for uninterpreted data, regardless of how m ends up being calculated

      // first load flags

      // for (i=0; i < n; ++i)
      for i in 0 .. n
       {
         if flagcount == 0 {
            flags = *points;
             points += 1;
            if flags & 8 {
                flagcount = *points;
                points += 1;
            }
         } else {
             flagcount -= 1;
         }
         vertices[off+i].vertex_type = flags;
      }

      // now load x coordinates
      x=0;
      // for (i=0; i < n; ++i)
      for i in 0 .. n
       {
         flags = vertices[off+i].vertex_type;
         if flags & 2 {
            let mut dx: i16 = *points as i16;
             points += 1;
            x += if flags & 16 { dx} else { -dx}; // ???
         } else {
            if flag_clear(flags, 16) {
               x = x +  (points[0]*256 + points[1]);
               points += 2;
            }
         }
         vertices[off+i].x =  x;
      }

      // now load y coordinates
      y=0;
      // for (i=0; i < n; ++i)
      for i in 0 .. n
       {
         flags = vertices[off+i].vertex_type;
         if flags & 4 {
            let mut dy = *points;
             points += 1;
            y += if flags & 32 { dy} else { -dy}; // ???
         } else {
            if flag_clear(flags, 32) {
               y = y +  (points[0]*256 + points[1]);
               points += 2;
            }
         }
         vertices[off+i].y =  y;
      }

      // now convert them to our format
      num_vertices=0;
      // sx = sy = cx = cy = scx = scy = 0;
      sx = 0;
       sy = 0;
       cx = 0;
       cy = 0;
       scx = 0;
       scy = 0;
       // for (i=0; i < n; ++i)
       for mut i in 0 .. n
       {
         flags = vertices[off+i].vertex_type;
         x     =  vertices[off+i].x;
         y     =  vertices[off+i].y;

         if next_move == i {
            if (i != 0) {
                num_vertices = stbtt__close_shape(vertices, num_vertices, was_off, start_off, sx, sy, scx, scy, cx, cy);
            }

            // now start the new one
            start_off = i32::from(flag_clear(flags, 1));
            if start_off {
               // if we start off with an off-curve point, then when we need to find a point on the curve
               // where we can start, and we need to save some state for when we wraparound.
               scx = x;
               scy = y;
               if !(vertices[off+i1].vertex_type & 1) {
                  // next point is also a curve point, so interpolate an on-point curve
                  sx = (x +  vertices[off+i1].x) >> 1;
                  sy = (y +  vertices[off+i1].y) >> 1;
               } else {
                  // otherwise just use the next point as our start point
                  sx =  vertices[off+i1].x;
                  sy =  vertices[off+i1].y;
                  i += 1; // we're using point i+1 as the starting point, so skip it
               }
            } else {
               sx = x;
               sy = y;
            }
            stbtt_setvertex(&mut vertices[num_vertices], STBTT_vmove, sx, sy, 0, 0);
             num_vertices += 1;
            was_off = 0;
            next_move = (1 + ttUSHORT(endPtsOfContours + j * 2)) as i32;
            j += 1;
         } else {
            if flag_clear(flags, 1) { // if it's a curve
               if was_off { // two off-curve control points in a row means interpolate an on-curve midpoint
                   stbtt_setvertex(&mut vertices[num_vertices], STBTT_vcurve, (cx + x) >> 1, (cy + y) >> 1, cx, cy);
               }
                num_vertices += 1;
               cx = x;
               cy = y;
               was_off = 1;
            } else {
               if was_off {
                   stbtt_setvertex(&mut vertices[num_vertices], STBTT_vcurve, x, y, cx, cy);
                   num_vertices += 1;
               }
               else {
                   stbtt_setvertex(&mut vertices[num_vertices], STBTT_vline, x, y, 0, 0);
                   num_vertices += 1;
               }
               was_off = 0;
            }
         }
      }
      num_vertices = stbtt__close_shape(vertices, num_vertices, was_off, start_off, sx,sy,scx,scy,cx,cy);
   } else if numberOfContours < 0 {
      // Compound shapes.
      let mut more: c_int = 1;
      comp: *mut u8 = data + g + 10;
      num_vertices = 0;
      vertices = None;
      while more {
         // stbtt_uint16 flags, gidx;
         let mut flags = 0u16;
          let mut gidx = 0u16;
          // let comp_num_verts: c_int = 0, i;
         let mut comp_num_verts = 0i32;
          let mut i = 0i32;
          let mut comp_verts: *mut stbtt_vertex = None;
          let mut tmp: *mut stbtt_vertex = None;
          let mut mtx: [c_float;6] = [0.0,0.0,0.0,1.0,0.0,0.0];
          let mut m = 0.0;
          let mut n =  0.0;

         flags = ttSHORT(comp) as u16;
          comp+=2;
         gidx = ttSHORT(comp) as u16;
          comp+=2;

         if flags & 2 { // XY values
            if flags & 1 { // shorts
               mtx[4] = ttSHORT(comp) as c_float;
                comp+=2;
               mtx[5] = ttSHORT(comp) as c_float;
                comp+=2;
            } else {
               mtx[4] = ttCHAR(comp);
                comp+=1;
               mtx[5] = ttCHAR(comp);
                comp+=1;
            }
         }
         else {
            // @TODO handle matching point
            STBTT_assert(0);
         }
         if flags & (1<<3) { // WE_HAVE_A_SCALE
            mtx[0] = (ttSHORT(comp) / 16384) as c_float;
             mtx[3] = (ttSHORT(comp) / 16384) as c_float;
             comp+=2;
            mtx[1] = 0.0;
             mtx[2] = 0.0;
         } else if flags & (1<<6) { // WE_HAVE_AN_X_AND_YSCALE
            mtx[0] = ttSHORT(comp)/16384.0;
             comp+=2;
            mtx[1] = 0.0;
             mtx[2] = 0.0;
            mtx[3] = ttSHORT(comp)/16384.0;
             comp+=2;
         } else if flags & (1<<7) { // WE_HAVE_A_TWO_BY_TWO
            mtx[0] = ttSHORT(comp)/16384.0;
             comp+=2;
            mtx[1] = ttSHORT(comp)/16384.0;
             comp+=2;
            mtx[2] = ttSHORT(comp)/16384.0;
             comp+=2;
            mtx[3] = ttSHORT(comp)/16384.0;
             comp+=2;
         }

         // Find transformation scales.
         m =  STBTT_sqrt(mtx[0]*mtx[0] + mtx[1]*mtx[1]);
         n =  STBTT_sqrt(mtx[2]*mtx[2] + mtx[3]*mtx[3]);

         // Get indexed glyph.
         comp_num_verts = stbtt_GetGlyphShape(info, gidx as c_int, &mut comp_verts);
         if comp_num_verts > 0 {
            // Transform vertices.
            // for (i = 0; i < comp_num_verts; ++i)
            for i in 0 .. comp_num_verts
             {
               let mut v = &comp_verts[i];
               // stbtt_vertex_type x,y;
               let mut x: stbtt_vertex_type = 0;
                 let mut y: stbtt_vertex_type = 0;
                 x=v.x; y=v.y;
               v.x = (stbtt_vertex_type)(m * (mtx[0]*x + mtx[2]*y + mtx[4]));
               v.y = (stbtt_vertex_type)(n * (mtx[1]*x + mtx[3]*y + mtx[5]));
               x=v.cx; y=v.cy;
               v.cx = (stbtt_vertex_type)(m * (mtx[0]*x + mtx[2]*y + mtx[4]));
               v.cy = (stbtt_vertex_type)(n * (mtx[1]*x + mtx[3]*y + mtx[5]));
            }
            // Append vertices.
            tmp = libc::malloc(((num_vertices + comp_num_verts) * mem::size_of::<stbtt_version>()) as size_t);
            if !tmp {
               if vertices { libc::free(vertices); }
               if comp_verts { libc::free(comp_verts); }
               return 0;
            }
            if num_vertices > 0 && vertices.is_null() == false { libc::memcpy(tmp, vertices, (num_vertices * mem::size_of::<stbtt_version>()) as size_t); }
            libc::memcpy(tmp+num_vertices, comp_verts, comp_num_verts*sizeof(stbtt_vertex));
            if vertices { libc::free(vertices); }
            vertices = tmp;
            libc::free(comp_verts);
            num_vertices += comp_num_verts;
         }
         // More components ?
         more = (flags & (1 << 5)) as c_int;
      }
   } else {
      // numberOfCounters == 0, do nothing
   }

   *pvertices = vertices;
   return num_vertices;
}


// #define STBTT__CSCTX_INIT(bounds) {bounds,0, 0,0, 0,0, 0,0,0,0, NULL, 0}

pub unsafe fn stbtt__track_vertex(c: *mut stbtt__csctx, x: i32, y: i32) {
    if x > c.max_x || !c.started == 1 { c.max_x = x; }
    if y > c.max_y || !c.started == 1 { c.max_y = y; }
    if x < c.min_x || !c.started == 1 { c.min_x = x; }
    if y < c.min_y || !c.started == 1 { c.min_y = y; }
    c.started = 1;
}

pub unsafe fn stbtt__csctx_v(c: *mut stbtt__csctx, vertex_type: u8, x: i32, y: i32, cx: i32, cy: i32, cx1: i32, cy1: i32) {
    if c.bounds {
        stbtt__track_vertex(c, x, y);
        if vertex_type == STBTT_vcubic as u8 {
            stbtt__track_vertex(c, cx, cy);
            stbtt__track_vertex(c, cx1, cy1);
        }
    } else {
        stbtt_setvertex(&mut c.pvertices[c.num_vertices], vertex_type as c_int, x, y, cx, cy);
        c.pvertices[c.num_vertices].cx1 = cx1;
        c.pvertices[c.num_vertices].cy1 = cy1;
    }
    c.num_vertices += 1;
}

pub unsafe fn stbtt__csctx_close_shape(ctx: *mut stbtt__csctx)
{
   if ctx.first_x != ctx.x || ctx.first_y != ctx.y {
       stbtt__csctx_v(ctx, STBTT_vline as u8, ctx.first_x as i32, ctx.first_y as i32, 0, 0, 0, 0);
   }
}

pub unsafe fn stbtt__csctx_rmove_to(ctx: *mut stbtt__csctx,dx: c_float,dy: c_float)
{
   stbtt__csctx_close_shape(ctx);
   ctx.first_x = ctx.x + dx;ctx.x = ctx.x + dx;
   ctx.first_y = ctx.y + dy;ctx.y = ctx.y + dy;
   stbtt__csctx_v(ctx, STBTT_vmove as u8, ctx.x as i32, ctx.y as i32, 0, 0, 0, 0);
}

pub unsafe fn stbtt__csctx_rline_to(ctx: *mut stbtt__csctx,dx: c_float,dy: c_float)
{
   ctx.x += dx;
   ctx.y += dy;
   stbtt__csctx_v(ctx, STBTT_vline as u8, ctx.x as i32, ctx.y as i32, 0, 0, 0, 0);
}

pub unsafe fn stbtt__csctx_rccurve_to(ctx: *mut stbtt__csctx,dx1: c_float,dy1: c_float,dx2: c_float,dy2: c_float,dx3: c_float,dy3: c_float)
{
   let cx1: c_float =  ctx.x + dx1;
   let cy1: c_float =  ctx.y + dy1;
   let cx2: c_float =  cx1 + dx2;
   let cy2: c_float =  cy1 + dy2;
   ctx.x = cx2 + dx3;
   ctx.y = cy2 + dy3;
   stbtt__csctx_v(ctx, STBTT_vcubic as u8, ctx.x as i32, ctx.y as i32, cx1 as i32, cy1 as i32, cx2 as i32, cy2 as i32);
}

pub fn stbtt__get_subr(mut idx: stbtt__buf, mut n: c_int) -> stbtt__buf {
    let count: c_int = stbtt__cff_index_count(&mut idx);
    let mut bias: c_int = 107;
    if count >= 33900 {
        bias = 32768;
    } else if count >= 1240 {
        bias = 1131;
    }
    n += bias;
    if n < 0 || n >= count {
        return stbtt__new_buf(None, 0);
    }
    return stbtt__cff_index_get(idx, n);
}

pub fn stbtt__cid_get_glyph_subrs(info: *const stbtt_fontinfo, glyph_index: c_int) -> stbtt__buf
{
   let mut fdselect: stbtt__buf = info.fdselect;
   // nranges: c_int, start, end, v, fmt, fdselector = -1, i;
let mut nranges: c_int = -1;
    let mut start: c_int = -1;
    let mut end: c_int = -1;
    let mut v: c_int = -1;
    let mut fmt: c_int = -1;
    let mut fdselector: c_int = -1;
    let mut i: c_int = -1;

   stbtt__buf_seek(&mut fdselect, 0);
   fmt = stbtt__buf_get8(&mut fdselect) as c_int;
   if fmt == 0 {
      // untested
      stbtt__buf_skip(&mut fdselect, glyph_index as size_t);
      fdselector = stbtt__buf_get8(&mut fdselect) as c_int;
   } else if fmt == 3 {
      nranges = stbtt__buf_get16(&fdselect);
      start = stbtt__buf_get16(&fdselect);
      // for (i = 0; i < nranges; i++)
      for i in 0 .. nranges
       {
         v = stbtt__buf_get8(&mut fdselect) as c_int;
         end = stbtt__buf_get16(&fdselect);
         if glyph_index >= start && glyph_index < end {
            fdselector = v;
            break;
         }
         start = end;
      }
   }
   if fdselector == -1 { stbtt__new_buf(None, 0); }
   return stbtt__get_subrs(info.cff, stbtt__cff_index_get(info.fontdicts, fdselector));
}

pub unsafe fn stbtt__run_charstring(info: *const stbtt_fontinfo, glyph_index: c_int, c: *mut stbtt__csctx) -> c_int
{
   // let in_header: c_int = 1, maskbits = 0, subr_stack_height = 0, sp = 0, v, i, b0;
   let mut in_header = 1i32;
    let mut maskbits = 0i32;
    let mut subr_stack_height = 0i32;
    let mut sp = 0i32;
    let mut v = 0i32;
    let mut i = 0i32;
    let mut b0 = 0i32;
    let mut has_subrs: c_int = 0;
    let mut clear_stack: c_float = 0.0;
    let mut s: [c_float;48] = [0.0;48];
   let mut subr_stack: [stbtt__buf;10] = [stbtt__buf::default();10];
    let mut subrs = info.subrs;
    let mut b;
   let mut f: c_float = 0.0;

// #define STBTT__CSERR(s) (0)

   // this currently ignores the initial width value, which isn't needed if we have hmtx
   b = stbtt__cff_index_get(info.charstrings, glyph_index);
   while b.cursor < b.size {
      i = 0;
      clear_stack = 1.0;
      b0 = stbtt__buf_get8(&mut b) as i32;
      match b0 {
      // @TODO implement hinting
      0x13 |
      0x14 => {
          // hintmask
          // cntrmask
          if in_header {
              maskbits += (sp / 2);
          }// implicit "vstem"
          in_header = 0;
          stbtt__buf_skip(&mut b, ((maskbits + 7) / 8) as size_t);
      },
         // break;

      0x01 | // hstem
      0x03 |  // vstem
      0x12 | // hstemhm
      0x17 => {
          // vstemhm
          maskbits += (sp / 2);
      },

      0x15 => {
          // rmoveto
          in_header = 0;
          if sp < 2 {
              return STBTT__CSERR("rmoveto stack");
          }
          stbtt__csctx_rmove_to(c, s[sp - 2], s[sp - 1]);
      },
      0x04 => {
          // vmoveto
          in_header = 0;
          if sp < 1 {
              return STBTT__CSERR("vmoveto stack");
          }
          stbtt__csctx_rmove_to(c, 0.0, s[sp - 1]);
      }
         // break;
      0x16 => {
          // hmoveto
          in_header = 0;
          if (sp < 1) {
              return STBTT__CSERR("hmoveto stack");
          }
          stbtt__csctx_rmove_to(c, s[sp - 1], 0.0);
      }
         // break;

      0x05 => {
          // rlineto
          if (sp < 2) {
              return STBTT__CSERR("rlineto stack");
          }
          // for (; i + 1 < sp; i += 2)
          while i + 1 < sp
          {
              stbtt__csctx_rline_to(c, s[i], s[i1]);
              i += 2;
          }
      }
         // break;

      // hlineto/vlineto and vhcurveto/hvcurveto alternate horizontal and vertical
      // starting from a different place.

      0x07 => {
          // vlineto
          if (sp < 1) {
              return STBTT__CSERR("vlineto stack");
          }
          // TODO:
          // goto           vlineto;
      }
      0x06 => {
          // hlineto
          if (sp < 1) { return STBTT__CSERR("hlineto stack"); }
          loop {
              if (i >= sp) {
                  break;
              }
              stbtt__csctx_rline_to(c, s[i], 0.0);
              i += 1;
              // TODO:
              // vlineto:
              if (i >= sp) {
                  break;
              }
              stbtt__csctx_rline_to(c, 0.0, s[i]);
              i += 1;
          }
      }
         // break;

      0x1F => {// hvcurveto
          if (sp < 4) { return STBTT__CSERR("hvcurveto stack"); }
          // TODO:
          // goto hvcurveto;
      },
      0x1E => {
          // vhcurveto
          if (sp < 4) { return STBTT__CSERR("vhcurveto stack"); }
          loop {
              if i + 3 >= sp {
              break(); }
              stbtt__csctx_rccurve_to(c, 0.0, s[i], s[i1], s[i2], s[i3], if sp - i == 5 { s[i + 4] } else { 0.0 });
              i += 4;
              // hvcurveto: if i + 3 >= sp {
              break(); }
              stbtt__csctx_rccurve_to(c, s[i], 0.0, s[i1], s[i2], if (sp - i == 5) { s[i4] } else { 0.0 }, s[i3]);
              i += 4;
          },
         // break;
        // rrcurveto
      0x08 => {

           if sp < 6 { return STBTT__CSERR("rcurveline stack")(); }
           // for (; i + 5 < sp; i += 6)
          while i + 5 < sp
          {
               stbtt__csctx_rccurve_to(c, s[i], s[i1], s[i2], s[i3], s[i4], s[i5]);
              i += 6;
           }
       },
         // break;
// rcurveline
      0x18 => {
           if sp < 8 { return STBTT__CSERR("rcurveline stack")(); }
           // for (; i + 5 < sp -2; i += 6)
          while i + 5 < sp - 2
          {
               stbtt__csctx_rccurve_to(c, s[i], s[i1], s[i2], s[i3], s[i4], s[i5]);
              i += 6;
           }
           if i + 1 >= sp { return STBTT__CSERR("rcurveline stack")(); }
           stbtt__csctx_rline_to(c, s[i], s[i1]);
       },
         // break;
// rlinecurve
      0x19 => {
           if sp < 8 { return STBTT__CSERR("rlinecurve stack")(); }
           // for (; i + 1 < sp -6; i += 2)
          while i + 1 < sp - 6
          {
               stbtt__csctx_rline_to(c, s[i], s[i1]);
              i += 2;
           }
           if i + 5 >= sp { return STBTT__CSERR("rlinecurve stack")(); }
           stbtt__csctx_rccurve_to(c, s[i], s[i1], s[i2], s[i3], s[i4], s[i5]);
       },
         // break;

       // vvcurveto, hhcurveto
      0x1A |
      0x1B => {
           if sp < 4 { return STBTT__CSERR("(vv|hh)curveto stack"); }
           f = 0.0;
           if sp & 1 {
               f = s[i];
               i += 1;
           }
           // for (; i + 3 < sp; i += 4)
          while i + 3 < sp
          {
               if b0 == 0x1B {
                   stbtt__csctx_rccurve_to(c, s[i], f, s[i1], s[i2], s[i3], 0.0);
               } else {
                   stbtt__csctx_rccurve_to(c, f, s[i], s[i1], s[i2], 0.0, s[i3]);
               }
               f = 0.0;
              i += 4
           }
       },
         // break;
// callsubr
      0x0A => {
           if (!has_subrs) {
               if (info.fdselect.size) {
                   subrs = stbtt__cid_get_glyph_subrs(info, glyph_index);
               }
               has_subrs = 1;
           }
       }
         // FALLTHROUGH
       // callgsubr
      0x1D => {
           if (sp < 1) { return STBTT__CSERR("call(g|)subr stack"); }
           v = s[--sp];
           if (subr_stack_height >= 10) { return STBTT__CSERR("recursion limit"); }
           subr_stack[subr_stack_height] = b;
          subr_stack_height += 1;
           b = stbtt__get_subr(if b0 == 0x0A { subrs } else { info.gsubrs }, v);
           if b.size == 0{
           return STBTT__CSERR("subr not found");}
           b.cursor = 0;
           clear_stack = 0.0;
       }
         // break;
// return
      0x0B => {
           if subr_stack_height <= 0 { return STBTT__CSERR("return outside subr")(); }
           b = subr_stack[--subr_stack_height];
           clear_stack = 0.0;
       },
         // break;
// endchar
      0x0E => {
           stbtt__csctx_close_shape(c);
           return 1;
       },
// two-byte escapedx1: c_float, dx2, dx3, dx4, dx5, dx6, dy1, dy2, dy3, dy4, dy5, dy6;dx: c_float, dy;
      0x0C => {
         let b1: c_int = stbtt__buf_get8(&mut b) as c_int;
         match (b1) {
         // @TODO These "flex" implementations ignore the flex-depth and resolution,
         // and always draw beziers.
             // hflex
         0x22 => {
             if sp < 7 { return STBTT__CSERR("hflex stack")(); }
             dx1 = s[0];
             dx2 = s[1];
             dy2 = s[2];
             dx3 = s[3];
             dx4 = s[4];
             dx5 = s[5];
             dx6 = s[6];
             stbtt__csctx_rccurve_to(c, dx1, 0.0, dx2, dy2, dx3, 0.0);
             stbtt__csctx_rccurve_to(c, dx4, 0.0, dx5, -dy2, dx6, 0.0);
         }
            // break;
 // flex
         0x23 => {
             if sp < 13 { return STBTT__CSERR("flex stack")(); }
             dx1 = s[0];
             dy1 = s[1];
             dx2 = s[2];
             dy2 = s[3];
             dx3 = s[4];
             dy3 = s[5];
             dx4 = s[6];
             dy4 = s[7];
             dx5 = s[8];
             dy5 = s[9];
             dx6 = s[10];
             dy6 = s[11];
             //fd is s[12]
             stbtt__csctx_rccurve_to(c, dx1, dy1, dx2, dy2, dx3, dy3);
             stbtt__csctx_rccurve_to(c, dx4, dy4, dx5, dy5, dx6, dy6);
         }
            // break;
// hflex1
         0x24 => {
             if sp < 9 { return STBTT__CSERR("hflex1 stack")(); }
             dx1 = s[0];
             dy1 = s[1];
             dx2 = s[2];
             dy2 = s[3];
             dx3 = s[4];
             dx4 = s[5];
             dx5 = s[6];
             dy5 = s[7];
             dx6 = s[8];
             stbtt__csctx_rccurve_to(c, dx1, dy1, dx2, dy2, dx3, 0.0);
             stbtt__csctx_rccurve_to(c, dx4, 0.0, dx5, dy5, dx6, -(dy1 + dy2 + dy5));
         }
            // break;
// flex1
         0x25 => {
             if (sp < 11) { return STBTT__CSERR("flex1 stack"); }
             dx1 = s[0];
             dy1 = s[1];
             dx2 = s[2];
             dy2 = s[3];
             dx3 = s[4];
             dy3 = s[5];
             dx4 = s[6];
             dy4 = s[7];
             dx5 = s[8];
             dy5 = s[9];
             dx6 = dy6 = s[10];
             dx = dx1 + dx2 + dx3 + dx4 + dx5;
             dy = dy1 + dy2 + dy3 + dy4 + dy5;
             if (STBTT_fabs(dx) > STBTT_fabs(dy)) {
                 dy6 = -dy;
             } else {
                 dx6 = -dx;
             }
             stbtt__csctx_rccurve_to(c, dx1, dy1, dx2, dy2, dx3, dy3);
             stbtt__csctx_rccurve_to(c, dx4, dy4, dx5, dy5, dx6, dy6);
         }
            // break;

         _ => {
             return STBTT__CSERR("unimplemented");
         }
         }
      }
       // break;

      // default:
     _ =>{
       if (b0 != 255 && b0 != 28 && b0 < 32) { return STBTT__CSERR("reserved operator"); }


         if (b0 == 255) {
             // push immediate
       f = stbtt__buf_get32( & b) / 0x10000;
       } else {
       stbtt__buf_skip( &mut b, - 1); f = stbtt__cff_int( &mut b) as c_float;
       }
       if sp >= 48 {  return STBTT__CSERR("push stack overflow")(); }
       s[sp] = f; sp += 1; clear_stack = 0.0;}
         // break;
      }
      if clear_stack {  sp = 0;}
   }
   return STBTT__CSERR("no endchar");

// #undef STBTT__CSERR
}

pub unsafe fn stbtt__GetGlyphShapeT2(info: *const stbtt_fontinfo, glyph_index: c_int, pvertices: *mut *mut stbtt_vertex) -> c_int {
    // runs the charstring twice, once to count and once to output (to avoid realloc)
    let mut count_ctx: stbtt__csctx = STBTT__CSCTX_INIT(1);
    let mut output_ctx: stbtt__csctx = STBTT__CSCTX_INIT(0);
    if stbtt__run_charstring(info, glyph_index, &mut count_ctx) {
        *pvertices = STBTT_malloc(count_ctx.num_vertices * sizeof(stbtt_vertex), info.userdata);
        output_ctx.pvertices = *pvertices;
        if stbtt__run_charstring(info, glyph_index, &mut output_ctx) {
            STBTT_assert(output_ctx.num_vertices == count_ctx.num_vertices);
            return output_ctx.num_vertices;
        }
    }
    *pvertices = None;
    return 0;
}

pub unsafe fn stbtt__GetGlyphInfoT2(info: *const stbtt_fontinfo, glyph_index: c_int, x0: *mut c_int, y0: *mut c_int, x1: *mut c_int, y1: *mut c_int) -> c_int {
    c: stbtt__csctx = STBTT__CSCTX_INIT(1);
    let r: c_int = stbtt__run_charstring(info, glyph_index, &mut c);
    if (x0) { *x0 = if r { c.min_x } else { 0 }; }
    if (y0) { *y0 = if r { c.min_y } else { 0 }; }
    if (x1) { *x1 = if r { c.max_x } else { 0 }; }
    if (y1) { *y1 = if r { c.max_y } else { 0 }; }
    return if r { c.num_vertices } else { 0 };
}

pub unsafe fn stbtt_GetGlyphShape(info: *const stbtt_fontinfo, glyph_index: c_int, pvertices: *mut *mut stbtt_vertex) -> c_int {
    return if !info.cff.size {
        stbtt__GetGlyphShapeTT(info, glyph_index, pvertices)
    } else {
        stbtt__GetGlyphShapeT2(info, glyph_index, pvertices)
    }
}

pub unsafe fn stbtt_GetGlyphHMetrics(info: *const stbtt_fontinfo, glyph_index: c_int, advanceWidth: *mut c_int, leftSideBearing: *mut c_int) {
    let mut numOfLongHorMetrics = ttUSHORT(info.data + info.hhea + 34);
    if glyph_index < numOfLongHorMetrics as c_int {
        if advanceWidth { *advanceWidth = ttSHORT(info.data + info.hmtx + 4 * glyph_index) as c_int; }
        if leftSideBearing { *leftSideBearing = ttSHORT(info.data + info.hmtx + 4 * glyph_index + 2) as c_int; }
    } else {
        if advanceWidth { *advanceWidth = ttSHORT(info.data + info.hmtx + 4 * (numOfLongHorMetrics - 1)) as c_int; }
        if leftSideBearing { *leftSideBearing = ttSHORT(info.data + info.hmtx + 4 * numOfLongHorMetrics + 2 * (glyph_index - numOfLongHorMetrics)) as c_int; }
    }
}

pub unsafe fn stbtt_GetKerningTableLength(info: *const stbtt_fontinfo) -> c_int {
    let mut data: *mut u8 = info.data + info.kern;

    // we only look at the first table. it must be 'horizontal' and format 0.
    if !info.kern {
        return 0;
    }
    if ttUSHORT(data2) < 1 {
// number of tables, need at least 1
        return 0;
    }
    if ttUSHORT(data8) != 1 {
// horizontal flag must be set in format
        return 0;
    }

    return ttUSHORT(data10) as c_int;
}

pub fn stbtt_GetKerningTable(info: *const stbtt_fontinfo, table: *mut stbtt_kerningentry, table_length: c_int) -> c_int {
    data: *mut u8 = info.data + info.kern;
    // k: c_int, length;
    let mut k: c_int = 0;
    let mut length: c_int = 0;

    // we only look at the first table. it must be 'horizontal' and format 0.
    if !info.kern {
        return 0;
    }
// number of tables, need at least 1
    if ttUSHORT(data2) < 1 { return 0; }
// horizontal flag must be set in format
    if ttUSHORT(data8) != 1 {
        return 0;
    }

    length = ttUSHORT(data10) as c_int;
    if table_length < length {
        length = table_length;
    }

    // for (k = 0; k < length; k++)
    for k in 0..length {
        table[k].glyph1 = ttUSHORT(data18 + (k * 6));
        table[k].glyph2 = ttUSHORT(data20 + (k * 6));
        table[k].advance = ttSHORT(data22 + (k * 6));
    }

    return length;
}

pub fn stbtt__GetGlyphKernInfoAdvance(info: *const stbtt_fontinfo, glyph1: c_int, glyph2: c_int) -> c_int {
    let mut data: *mut u8 = info.data + info.kern;
    // needle: u32, straw;
    let mut needle = 0u32;
    let mut straw = 0u32;
    // l: c_int, r, m;
    let mut l = 0i32;
    let mut r = 0i32;
    let mut m = 0i32;

    // we only look at the first table. it must be 'horizontal' and format 0.
    if !info.kern {
        return 0;
    }
    if ttUSHORT(data2) < 1 {// number of tables, need at least 1
        return 0;
    }
    if ttUSHORT(data8) != 1 { // horizontal flag must be set in format
        return 0;
    }

    l = 0;
    r = (ttUSHORT(data10) - 1) as i32;
    needle = (glyph1 << 16 | glyph2) as u32;
    while l <= r {
        m = (l + r) >> 1;
        straw = ttULONG(data18 + (m * 6)) as u32; // note: unaligned read
        if needle < straw {
            r = m - 1;
        } else if needle > straw {
            l = m + 1;
        } else {
            return ttSHORT(data22 + (m * 6)) as c_int;
        }
    }
    return 0;
}

pub fn stbtt__GetCoverageIndex(coverageTable: *mut u8, glyph: c_int) -> i32 {
    let mut coverageFormat: u16 = ttUSHORT(coverageTable);
    match (coverageFormat) {
        1 => {
            let mut glyphCount: u16 = ttUSHORT(coverageTable + 2);

            // Binary search.
            // l: i32=0, r=glyphCount-1, m;
            let mut l = 0i32;
            let mut r = glyphCount - 1;
            let mut m = 0i32;
            // straw: c_int, needle=glyph;
            let mut straw = 0i32;
            let mut needle = glyph;
            while l <= r as i32 {
                glyphArray: *mut u8 = coverageTable + 4;
                glyphID: u16;
                m = (l + r) >> 1;
                glyphID = ttUSHORT(glyphArray + 2 * m);
                straw = glyphID;
                if needle < straw {
                    r = (m - 1) as u16;
                } else if needle > straw {
                    l = m + 1;
                } else {
                    return m;
                }
            }
            // break;
        }

        2 => {
            let mut rangeCount: u16 = ttUSHORT(coverageTable + 2);
            let mut rangeArray: *mut u8 = coverageTable + 4;

            // Binary search.
            // l: i32=0, r=rangeCount-1, m;
            let mut l = 0i32;
            let mut r = rangeCount - 1;
            let mut m = 0i32;
            // strawStart: c_int, strawEnd, needle=glyph;
            let mut strawStart = 0i32;
            let mut strawEnd = 0i32;
            let mut needle = glyph;
            while l <= r as i32 {
                let mut rangeRecord: *mut u8 = None;
                m = (l + r) >> 1;
                rangeRecord = rangeArray + 6 * m;
                strawStart = ttUSHORT(rangeRecord) as i32;
                strawEnd = ttUSHORT(rangeRecord + 2) as i32;
                if needle < strawStart {
                    r = (m - 1) as u16;
                } else if needle > strawEnd {
                    l = m + 1;
                } else {
                    startCoverageIndex: u16 = ttUSHORT(rangeRecord + 4);
                    return startCoverageIndex + glyph - strawStart;
                }
            }
            // break;
        }

        _ => { return -1; } // unsupported
    }

    return -1;
}

pub fn stbtt__GetGlyphClass(classDefTable: *mut u8, glyph: c_int) -> u16 {
    let mut classDefFormat: u16 = ttUSHORT(classDefTable);
    match (classDefFormat) {
        1 => {
            let mut startGlyphID: u16 = ttUSHORT(classDefTable + 2);
            let mut glyphCount: u16 = ttUSHORT(classDefTable + 4);
            let mut classDef1ValueArray: *mut u8 = classDefTable + 6;

            if glyph >= startGlyphID as c_int && glyph < (startGlyphID + glyphCount) as c_int {
                return ttUSHORT(classDef1ValueArray + 2 * (glyph - startGlyphID));
            }
            // break;
        }

        2 => {
            let mut classRangeCount: u16 = ttUSHORT(classDefTable + 2);
            let mut classRangeRecords: *mut u8 = classDefTable + 4;

            // Binary search.
            // l: i32=0, r=classRangeCount-1, m;
            let mut l = 0i32;
            let mut r = classRangeCount - 1;
            let mut m = 0i32;
            // strawStart: c_int, strawEnd, needle=glyph;
            let mut strawStart = 0i32;
            let mut strawEnd = 0i32;
            let mut needle = glyph;
            while l <= r as i32 {
                let mut classRangeRecord: *mut u8 = None;
                m = (l + r) >> 1;
                classRangeRecord = classRangeRecords + 6 * m;
                strawStart = ttUSHORT(classRangeRecord) as i32;
                strawEnd = ttUSHORT(classRangeRecord + 2) as i32;
                if needle < strawStart {
                    r = (m - 1) as u16;
                } else if needle > strawEnd {
                    l = m + 1;
                } else {
                    return ttUSHORT(classRangeRecord + 4);
                }
            }
            // break;
        }

        // default:
        _ => {
            return -1;
        } // Unsupported definition type, return an error.
    }

    // "All glyphs not assigned to a class fall into class 0". (OpenType spec)
    return 0;
}

// Define to STBTT_assert(x) if you want to break on unimplemented formats.
// #define STBTT_GPOS_TODO_assert(x)

pub fn stbtt__GetGlyphGPOSInfoAdvance(info: *const stbtt_fontinfo, glyph1: c_int, glyph2: c_int) -> i32 {
    let mut lookupListOffset = 0i16;
    let mut lookupList: *mut u8 = None;
    let mut lookupCount = 0i16;
    let mut data: *mut u8 = None;
    // i: i32, sti;
    let mut i = 0i32;
    let mut sti = 0i32;

    if !info.gpos { return 0; }

    data = info.data + info.gpos;

    if ttUSHORT(data0) != 1 { return 0; } // Major version 1
    if ttUSHORT(data2) != 0 { return 0; } // Minor version 0

    lookupListOffset = ttUSHORT(data8) as i16;
    lookupList = data + lookupListOffset;
    lookupCount = ttUSHORT(lookupList) as i16;

    // for (i=0; i<lookupCount; ++i)
    for i in 0..lookupCount {
        let mut lookupOffset: u16 = ttUSHORT(lookupList + 2 + 2 * i);
        let mut lookupTable: *mut u8 = lookupList + lookupOffset;

        let mut lookupType: u16 = ttUSHORT(lookupTable);
        let mut subTableCount: u16 = ttUSHORT(lookupTable + 4);
        let mut subTableOffsets: *mut u8 = lookupTable + 6;
        // Pair Adjustment Positioning Subtable
        if lookupType != 2 {
            continue;
        }

        // for (sti=0; sti<subTableCount; sti++)
        for sti in 0..subTableCount {
            let mut subtableOffset: u16 = ttUSHORT(subTableOffsets + 2 * sti);
            let mut table: *mut u8 = lookupTable + subtableOffset;
            let mut posFormat: u16 = ttUSHORT(table);
            let mut coverageOffset: u16 = ttUSHORT(table + 2);
            let mut coverageIndex: i32 = stbtt__GetCoverageIndex(table + coverageOffset, glyph1);
            if coverageIndex == -1 { continue; }

            match posFormat {
                1 => {
                    // l: i32, r, m;
                    let mut l = 0i32;
                    let mut r = 0i32;
                    let mut m = 0i32;
                    // straw: c_int, needle;
                    let mut straw = 0i32;
                    let mut needle = 0i32;
                    let mut valueFormat1: u16 = ttUSHORT(table + 4);
                    let mut valueFormat2: u16 = ttUSHORT(table + 6);
                    if valueFormat1 == 4 && valueFormat2 == 0 { // Support more formats?
                        let mut valueRecordPairSizeInBytes: i32 = 2;
                        let mut pairSetCount: u16 = ttUSHORT(table + 8);
                        let mut pairPosOffset: u16 = ttUSHORT(table + 10 + 2 * coverageIndex);
                        let mut pairValueTable: *mut u8 = table + pairPosOffset;
                        let mut pairValueCount: u16 = ttUSHORT(pairValueTable);
                        let mut pairValueArray: *mut u8 = pairValueTable + 2;

                        if coverageIndex >= pairSetCount as i32 { return 0; }

                        needle = glyph2;
                        r = (pairValueCount - 1) as i32;
                        l = 0;

                        // Binary search.
                        while l <= r {
                            let mut secondGlyph: u16;
                            let mut pairValue: *mut u8 = None;
                            m = (l + r) >> 1;
                            pairValue = pairValueArray + (2 + valueRecordPairSizeInBytes) * m;
                            secondGlyph = ttUSHORT(pairValue);
                            straw = secondGlyph as i32;
                            if needle < straw {
                                r = m - 1;
                            } else if needle > straw {
                                l = m + 1;
                            } else {
                                let mut xAdvance: i16 = ttSHORT(pairValue + 2);
                                return xAdvance as i32;
                            }
                        }
                    } else {
                        return 0;
                    }
                    // break;
                }

                2 => {
                    let mut valueFormat1: u16 = ttUSHORT(table + 4);
                    let mut valueFormat2: u16 = ttUSHORT(table + 6);
                    return if valueFormat1 == 4 && valueFormat2 == 0 { // Support more formats?
                        let mut classDef1Offset: u16 = ttUSHORT(table + 8);
                        let mut classDef2Offset: u16 = ttUSHORT(table + 10);
                        let glyph1class: c_int = stbtt__GetGlyphClass(table + classDef1Offset, glyph1) as c_int;
                        let glyph2class: c_int = stbtt__GetGlyphClass(table + classDef2Offset, glyph2) as c_int;

                        let mut class1Count: u16 = ttUSHORT(table + 12);
                        let mut class2Count: u16 = ttUSHORT(table + 14);
                        let mut class1Records: *mut u8;
                        let mut class2Records: *mut u8;
                        let mut xAdvance = 0i16;

                        if glyph1class < 0 || glyph1class >= class1Count as c_int { return 0; } // malformed
                        if glyph2class < 0 || glyph2class >= class2Count as c_int { return 0; } // malformed

                        class1Records = table + 16;
                        class2Records = class1Records + 2 * (glyph1class * class2Count);
                        xAdvance = ttSHORT(class2Records + 2 * glyph2class);
                        xAdvance as i32
                    } else {
                        0
                    }
                    // break;
                }

                _ => {
                    return 0;
                } // Unsupported position format
            }
        }
    }

    return 0;
}

pub fn stbtt_GetGlyphKernAdvance(info: *const stbtt_fontinfo, g1: c_int, g2: c_int) -> c_int {
    let mut xAdvance: c_int = 0;

    if info.gpos {
        xAdvance += stbtt__GetGlyphGPOSInfoAdvance(info, g1, g2);
    } else if info.kern {
        xAdvance += stbtt__GetGlyphKernInfoAdvance(info, g1, g2);
    }

    return xAdvance;
}

pub fn stbtt_GetCodepointKernAdvance(info: *const stbtt_fontinfo, ch1: c_int, ch2: c_int) -> c_int {
    if !info.kern && !info.gpos {
// if no kerning table, don't waste time looking up both codepoint->glyphs
        return 0;
    }
    return stbtt_GetGlyphKernAdvance(info, stbtt_FindGlyphIndex(info, ch1), stbtt_FindGlyphIndex(info, ch2));
}

pub unsafe fn stbtt_GetCodepointHMetrics(info: *const stbtt_fontinfo, codepoint: c_int, advanceWidth: *mut c_int, leftSideBearing: *mut c_int) {
    stbtt_GetGlyphHMetrics(info, stbtt_FindGlyphIndex(info, codepoint), advanceWidth, leftSideBearing);
}

pub unsafe fn stbtt_GetFontVMetrics(info: *const stbtt_fontinfo, ascent: *mut c_int, descent: *mut c_int, lineGap: *mut c_int) {
    if ascent { *ascent = ttSHORT(info.data + info.hhea + 4) as c_int; }
    if descent { *descent = ttSHORT(info.data + info.hhea + 6) as c_int; }
    if lineGap { *lineGap = ttSHORT(info.data + info.hhea + 8) as c_int; }
}

pub unsafe fn stbtt_GetFontVMetricsOS2(info: *const stbtt_fontinfo, typoAscent: *mut c_int, typoDescent: *mut c_int, typoLineGap: *mut c_int) -> c_int
{
   let tab: c_int = stbtt__find_table(info.data, info.fontstart, str_to_const_c_char_ptr("OS/2")) as c_int;
   if !tab {
return 0;}
   if typoAscent { * typoAscent = ttSHORT(info.data + tab + 68) as c_int;}
   if typoDescent {* typoDescent = ttSHORT(info.data + tab + 70) as c_int;}
   if typoLineGap {* typoLineGap = ttSHORT(info.data + tab + 72) as c_int;}
   return 1;
}

pub unsafe fn stbtt_GetFontBoundingBox(info: *const stbtt_fontinfo, x0: *mut c_int, y0: *mut c_int, x1: *mut c_int, y1: *mut c_int)
{
   *x0 = ttSHORT(info.data + info.head + 36) as c_int;
   *y0 = ttSHORT(info.data + info.head + 38) as c_int;
   *x1 = ttSHORT(info.data + info.head + 40) as c_int;
   *y1 = ttSHORT(info.data + info.head + 42) as c_int;
}

pub fn stbtt_ScaleForPixelHeight(info: *const stbtt_fontinfo,height: c_float) -> f32
{
   let fheight: c_int = (ttSHORT(info.data + info.hhea + 4) - ttSHORT(info.data + info.hhea + 6)) as c_int;
   return  height / fheight;
}

pub fn stbtt_ScaleForMappingEmToPixels(info: *const stbtt_fontinfo,pixels: c_float) -> f32
{
   let unitsPerEm: c_int = ttUSHORT(info.data + info.head + 18) as c_int;
   return pixels / unitsPerEm;
}

pub fn stbtt_FreeShape(info: *const stbtt_fontinfo, v: *mut stbtt_vertex)
{
   STBTT_free(v, info.userdata);
}

pub unsafe fn stbtt_FindSVGDoc(info: *mut stbtt_fontinfo, gl: c_int) -> *mut u8 {
    let mut i: c_int = 0;
    data: *mut u8 = info.data;
    svg_doc_list: *mut u8 = data + stbtt__get_svg(info);
    let numEntries: c_int = ttUSHORT(svg_doc_list) as c_int;
    svg_docs: *mut u8 = svg_doc_list + 2;

    // for(i=0; i<numEntries; i++)
    for i in 0..numEntries {
        svg_doc: *mut u8 = svg_docs + (12 * i);
        if (gl >= ttUSHORT(svg_doc) as c_int) && (gl <= ttUSHORT(svg_doc + 2) as c_int) {
            return svg_doc;
        }
    }
    return None;
}

pub unsafe fn stbtt_GetGlyphSVG(info: *mut stbtt_fontinfo, gl: c_int, mut svg: *mut *mut c_char) -> c_int {
    data: *mut u8 = info.data;
    let mut svg_doc: *mut u8 = None;

    if info.svg == 0 {
        return 0;
    }

    svg_doc = stbtt_FindSVGDoc(info, gl);
    if svg_doc != None {
        *svg = data + info.svg + ttULONG(svg_doc + 4);
        return ttULONG(svg_doc + 8) as c_int;
    } else {
        return 0;
    }
}

pub unsafe fn stbtt_GetCodepointSVG(info: *mut stbtt_fontinfo, unicode_codepoint: c_int, svg: *mut *mut c_char) -> c_int
{
   return stbtt_GetGlyphSVG(info, stbtt_FindGlyphIndex(info, unicode_codepoint), svg);
}

//////////////////////////////////////////////////////////////////////////////
//
// antialiasing software rasterizer
//

pub unsafe fn stbtt_GetGlyphBitmapBoxSubpixel(font: *const stbtt_fontinfo, glyph: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, ix0: *mut c_int, iy0: *mut c_int, ix1: *mut c_int, iy1: *mut c_int)
{
   // let mut x0: c_int = 0,y0=0,x1,y1; // =0 suppresses compiler warning
   let mut x0 = 0i32;
    let mut y0 = 0i32;
    let mut x1 = 0i32;
    let mut y1 = 0i32;
    if !stbtt_GetGlyphBox(font, glyph, &mut x0, &mut y0, &mut x1, &mut y1) {
      // e.g. space character
      if ix0 { *ix0 = 0; }
      if iy0 { *iy0 = 0; }
      if ix1 { *ix1 = 0; }
      if iy1 { *iy1 = 0; }
   } else {
      // move to integral bboxes (treating pixels as little squares, what pixels get touched)?
      if ix0 { *ix0 = STBTT_ifloor(x0 * scale_x + shift_x); }
      if iy0 { *iy0 = STBTT_ifloor(-y1 * scale_y + shift_y); }
      if ix1 { *ix1 = STBTT_iceil(x1 * scale_x + shift_x); }
      if iy1 { *iy1 = STBTT_iceil(-y0 * scale_y + shift_y); }
   }
}

pub unsafe fn stbtt_GetGlyphBitmapBox(font: *const stbtt_fontinfo, glyph: c_int,scale_x: c_float,scale_y: c_float, ix0: *mut c_int, iy0: *mut c_int, ix1: *mut c_int, iy1: *mut c_int)
{
   stbtt_GetGlyphBitmapBoxSubpixel(font, glyph, scale_x, scale_y,0.0,0.0, ix0, iy0, ix1, iy1);
}

pub unsafe fn stbtt_GetCodepointBitmapBoxSubpixel(font: *const stbtt_fontinfo, codepoint: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, ix0: *mut c_int, iy0: *mut c_int, ix1: *mut c_int, iy1: *mut c_int)
{
   stbtt_GetGlyphBitmapBoxSubpixel(font, stbtt_FindGlyphIndex(font,codepoint), scale_x, scale_y,shift_x,shift_y, ix0,iy0,ix1,iy1);
}

pub unsafe fn stbtt_GetCodepointBitmapBox(font: *const stbtt_fontinfo, codepoint: c_int,scale_x: c_float,scale_y: c_float, ix0: *mut c_int, iy0: *mut c_int, ix1: *mut c_int, iy1: *mut c_int)
{
   stbtt_GetCodepointBitmapBoxSubpixel(font, codepoint, scale_x, scale_y,0.0,0.0, ix0,iy0,ix1,iy1);
}

//////////////////////////////////////////////////////////////////////////////
//
//  Rasterizer





pub unsafe fn stbtt__hheap_alloc(hh: *mut stbtt__hheap, size: size_t, userdata: *mut c_void) -> *mut c_void
{
   if hh.first_free {
      p: *mut c_void = hh.first_free;
      hh.first_free = *p;
      return p;
   } else {
      if hh.num_remaining_in_head_chunk == 0 {
         let count: c_int = (if size < 32 { 2000 } else {
             if size < 128 {
                 800
             } else { 100 }
         });
         c: *mut stbtt__hheap_chunk = STBTT_malloc(mem::size_of::<stbtt__hheap_chunk>() + size * count, userdata);
         if c == None {
            return None; }
         c.next = hh.head;
         hh.head = c;
         hh.num_remaining_in_head_chunk = count;
      }
      hh.num_remaining_in_head_chunk -= 1;
      return  (hh.head) + mem::size_of::<stbtt__hheap_chunk>() + size * hh.num_remaining_in_head_chunk;
   }
}

pub unsafe fn stbtt__hheap_free(hh: *mut stbtt__hheap, p: *mut c_void)
{
   *p = hh.first_free;
   hh.first_free = p;
}

pub unsafe fn stbtt__hheap_cleanup(hh: *mut stbtt__hheap, userdata: *mut c_void)
{
   c: *mut stbtt__hheap_chunk = hh.head;
   while (c) {
      n: *mut stbtt__hheap_chunk = c.next;
      STBTT_free(c, userdata);
      c = n;
   }
}





// #if STBTT_RASTERIZER_VERSION == 1
// #define STBTT_FIXSHIFT   10
// #define STBTT_FIX        (1 << STBTT_FIXSHIFT)
// #define STBTT_FIXMASK    (STBTT_FIX-1)

pub unsafe fn stbtt__new_active(hh: *mut stbtt__hheap, e: *mut stbtt__edge, off_x: c_int, start_point: c_float, userdata: *mut c_void) -> *mut stbtt__active_edge {
    let mut z: *mut stbtt__active_edge = stbtt__hheap_alloc(hh, sizeof(*z), userdata);
    let dxdy: c_float = (e.x1 - e.x0) / (e.y1 - e.y0);
    // STBTT_assert(z != null_mut());
    if !z { return z; }

    // round dx down to avoid overshooting
    if dxdy < 0.0 {
        z.dx = -STBTT_ifloor(STBTT_FIX * -dxdy);
    } else {
        z.dx = STBTT_ifloor(STBTT_FIX * dxdy);
    }
    z.x = STBTT_ifloor(STBTT_FIX * e.x0 + z.dx * (start_point - e.y0)); // use z->dx so when we offset later it's by the same amount
    z.x -= off_x * STBTT_FIX;

    z.ey = e.y1;
    z.next = None;
    z.direction = if e.invert { 1 } else { -1 } as c_float;
    return z;
}

// #elif STBTT_RASTERIZER_VERSION == 2
pub unsafe fn stbtt__new_active2(hh: *mut stbtt__hheap, e: *mut stbtt__edge, off_x: c_int,start_point: c_float, userdata: *mut c_void) -> *mut stbtt__active_edge
{
   let mut z: *mut stbtt__active_edge = stbtt__hheap_alloc(hh, sizeof(*z), userdata);
   let dxdy: c_float =  (e.x1 - e.x0) / (e.y1 - e.y0);
   // STBTT_assert(z != null_mut());
   //STBTT_assert(e->y0 <= start_point);
   if !z { return z;}
   z.fdx = dxdy;
   z.fdy = if dxdy != 0.0 { (1.0 / dxdy) } else { 0.0 };
   z.fx = e.x0 + dxdy * (start_point - e.y0);
   z.fx -= off_x;
   z.direction = if e.invert { 1.0 } else { -1.0 };
   z.sy = e.y0;
   z.ey = e.y1;
   z.next = None;
   return z;
}
// #else
// #error "Unrecognized value of STBTT_RASTERIZER_VERSION"
// #endif

// #if STBTT_RASTERIZER_VERSION == 1
// note: this routine clips fills that extend off the edges... ideally this
// wouldn't happen, but it could happen if the truetype glyph bounding boxes
// are wrong, or if the user supplies a too-small bitmap
pub unsafe fn stbtt__fill_active_edges(c_uscanline: *mut c_char, len: c_int, mut e: *mut stbtt__active_edge, max_weight: c_int)
{
   // non-zero winding fill
   // let mut x0: c_int = 0, w=0;
    let mut x0 = 0i32;
    let mut w = 0i32;

   while e {
      if w == 0 {
         // if we're currently at zero, we need to record the edge start point
         x0 = e.x as i32;
          w += e.direction;
      } else {
         let x1: c_int = e.x as c_int;
          w += e.direction;
         // if we went to zero, we need to draw
         if (w == 0) {
            let mut i: c_int = x0 >> STBTT_FIXSHIFT;
            let mut j: c_int = x1 >> STBTT_FIXSHIFT;

            if (i < len && j >= 0) {
               if (i == j) {
                  // x0,x1 are the same pixel, so compute combined coverage
                  scanline[i] = scanline[i] +  ((x1 - x0) * max_weight >> STBTT_FIXSHIFT);
               } else {
                   // add antialiasing for x0
                  if (i >= 0) {
                      scanline[i] = scanline[i] + (((STBTT_FIX - (x0 & STBTT_FIXMASK)) * max_weight) >> STBTT_FIXSHIFT);
                  }
                  else {
                      i = -1;
                  }
                   // clip

                  if (j < len) {
                      // add antialiasing for x1
                      scanline[j] = scanline[j] + (((x1 & STBTT_FIXMASK) * max_weight) >> STBTT_FIXSHIFT);
                  }
                  else {
                      j = len;
                  }
                   // clip

                  // for (i += 1; i < j; ++i)
                  for i in 1 .. j
                   {
                      // fill pixels between x0 and x1
                      scanline[i] = scanline[i] + max_weight;
                  }
               }
            }
         }
      }

      e = e.next;
   }
}

pub unsafe fn stbtt__rasterize_sorted_edges(result: *mut stbtt__bitmap, mut e: *mut stbtt__edge, n: c_int, vsubsample: c_int, off_x: c_int, off_y: c_int, userdata: *mut c_void)
{
   // stbtt__hheap hh = { 0, 0, 0 };
   let mut hh = stbtt__hheap::default();
    let mut active: *mut stbtt__active_edge= None;
   // y: c_int,j=0;
   let mut y = 0i32;
    let mut j = 0i32;
    let max_weight: c_int = (255 / vsubsample);  // weight per vertical scanline
   let mut s: c_int = 0; // vertical subsample index
   // unsigned scanline_data: [c_char;512], *scanline;
    let mut scanline_data: [c_uchar;512] = [0;512];
    let mut scanline: *mut c_uchar = None;

   if result.w > 512 {
       scanline = STBTT_malloc(result.w, userdata);
   }
   else {
       scanline = scanline_data.as_mut_ptr();
   }

   y = off_y * vsubsample;
   e[n].y0 = (off_y + result.h) *  vsubsample + 1;

   while j < result.h {
      STBTT_memset(scanline, 0, result.w);
      // for (s=0; s < vsubsample; ++s)
      for s in 0 .. vsubsample
       {
         // find center of pixel for this scanline
         let scan_y: c_float =  y + 0.5;
         stbtt__active_edge **step = &active;

         // update all active edges;
         // remove all active edges that terminate before the center of this scanline
         while (*step) {
            stbtt__active_edge * z = *step;
            if (z.ey <= scan_y) {
               *step = z.next; // delete from list
               STBTT_assert(z.direction);
               z.direction = 0;
               stbtt__hheap_free(&mut hh, z);
            } else {
               z.x += z.dx; // advance to position for current scanline
               step = &((*step).next); // advance through list
            }
         }

         // resort the list if needed
         loop {
            let mut changed: c_int = 0;
            step = &active;
            while (*step && (*step).next) {
               if ((*step).x > (*step).next.x) {
                  t: *mut stbtt__active_edge = *step;
                  q: *mut stbtt__active_edge = t.next;

                  t.next = q.next;
                  q.next = t;
                  *step = q;
                  changed = 1;
               }
               step = &(*step).next;
            }
            if (!changed) { break; }
         }

         // insert all edges that start before the center of this scanline -- omit ones that also end on this scanline
         while (e.y0 <= scan_y) {
            if (e.y1 > scan_y) {
               z: *mut stbtt__active_edge = stbtt__new_active(&mut hh, e, off_x, scan_y, userdata);
               if (z != null_mut()) {
                  // find insertion point
                  if (active == null_mut()) {
                      active = z;
                  }
                  else if (z.x < active.x) {
                     // insert at front
                     z.next = active;
                     active = z;
                  } else {
                     // find thing to insert AFTER
                     p: *mut stbtt__active_edge = active;
                     while (p.next && p.next.x < z.x){
                          p = p.next;
                      }
                     // at this point, p->next->x is NOT < z->x
                     z.next = p.next;
                     p.next = z;
                  }
               }
            }
            e += 1;
         }

         // now process all active edges in XOR fashion
         if active {
             stbtt__fill_active_edges(scanline as *mut c_char, result.w, active, max_weight);
         }

         y += 1;
      }
      STBTT_memcpy(result.pixels + j * result.stride, scanline, result.w);
      j += 1;
   }

   stbtt__hheap_cleanup(&mut hh, userdata);

   if scanline != scanline_data.as_mut_ptr() {
       STBTT_free(scanline, userdata);
   }
}

// #elif STBTT_RASTERIZER_VERSION == 2

// the edge passed in here does not cross the vertical line at x or the vertical line at x+1
// (i.e. it has already been clipped to those)
pub unsafe fn stbtt__handle_clipped_edge(scanline: &mut c_float, x: c_int, e: *mut stbtt__active_edge, mut x0: c_float, mut y0: c_float, mut x1: c_float, mut y1: c_float)
{
   if y0 == y1 { return; }
   STBTT_assert(y0 < y1);
   STBTT_assert(e.sy <= e.ey);
   if y0 > e.ey { return; }
   if y1 < e.sy { return; }
   if y0 < e.sy {
      x0 += (x1-x0) * (e.sy - y0) / (y1-y0);
      y0 = e.sy;
   }
   if y1 > e.ey {
      x1 += (x1-x0) * (e.ey - y1) / (y1-y0);
      y1 = e.ey;
   }

   if x0 == x as c_float {
       STBTT_assert(x1 <= x1);
   }
   else if x0 == x1 {
       STBTT_assert(x1 >= x as c_float);
   }
   else if x0 <= x as c_float as c_float {
       STBTT_assert(x1 <= x as c_float);
   }
   else if x0 >= x1 {
       STBTT_assert(x1 >= x1);
   }
   else {
       STBTT_assert(x1 >= x as c_float && x1 <= x1);
   }

   if x0 <= x as c_float as c_float && x1 <= x as c_float {
       scanline[x] += e.direction * (y1 - y0);
   }
   else if x0 >= x1 && x1 >= x1 {

   }
   else {
      STBTT_assert(x0 >= x as c_float && x0 <= x1 && x1 >= x as c_float && x1 <= x1);
      scanline[x] += e.direction * (y1-y0) * (1-((x0-x)+(x1-x))/2); // coverage = 1 - average x position
   }
}

pub unsafe fn stbtt__sized_trapezoid_area(height: c_float,top_width: c_float,bottom_width: c_float) -> f32
{
   STBTT_assert(top_width >= 0.0);
   STBTT_assert(bottom_width >= 0.0);
   return (top_width + bottom_width) / 2.0 * height;
}

pub unsafe fn stbtt__position_trapezoid_area(height: c_float,tx0: c_float,tx1: c_float,bx0: c_float,bx1: c_float) -> f32
{
   return stbtt__sized_trapezoid_area(height, tx1 - tx0, bx1 - bx0);
}

pub unsafe fn stbtt__sized_triangle_area(height: c_float,width: c_float) -> f32
{
   return height * width / 2;
}

pub unsafe fn stbtt__fill_active_edges_new(scanline: &mut c_float, scanline_fill: &mut c_float, len: c_int, mut e: *mut stbtt__active_edge,y_top: c_float)
{
   let y_bottom: c_float =  y_top1;

   while e {
      // brute force every pixel

      // compute intersection points with top & bottom
      STBTT_assert(e.ey >= y_top);

      if e.fdx == 0 as c_float {
         let x0: c_float =  e.fx;
         if x0 < len as c_float {
            if x0 >= 0 as c_float {
               stbtt__handle_clipped_edge(scanline, x0 as c_int, e, x0, y_top, x0, y_bottom);
               stbtt__handle_clipped_edge(scanline_fill-1, x01,e, x0,y_top, x0,y_bottom);
            } else {
               stbtt__handle_clipped_edge(scanline_fill-1,0,e, x0,y_top, x0,y_bottom);
            }
         }
      } else {
         let mut x0: c_float =  e.fx;
         let mut dx: c_float =  e.fdx;
         let mut xb: c_float =  x0 + dx;
          let mut x_top: c_float = 0.0;
          let mut x_bottom: c_float = 0.0;
          let mut sy0: c_float = 0.0;
          let mut sy1: c_float= 0.0;
         let mut dy: c_float =  e.fdy;
         STBTT_assert(e.sy <= y_bottom && e.ey >= y_top);

         // compute endpoints of line segment clipped to this scanline (if the
         // line segment starts on this scanline. x0 is the intersection of the
         // line with y_top, but that may be off the line segment.
         if e.sy > y_top {
            x_top = x0 + dx * (e.sy - y_top);
            sy0 = e.sy;
         } else {
            x_top = x0;
            sy0 = y_top;
         }
         if e.ey < y_bottom {
            x_bottom = x0 + dx * (e.ey - y_top);
            sy1 = e.ey;
         } else {
            x_bottom = xb;
            sy1 = y_bottom;
         }

         if x_top >= 0.0 && x_bottom >= 0.0 && x_top < len as c_float && x_bottom < len as c_float {
            // from here on, we don't have to range check x values

            if x_top ==  x_bottom {
               let mut height: c_float = 0.0;
               // simple case, only spans one pixel
               let x: c_int = x_top as c_int;
               height = (sy1 - sy0) * e.direction;
               STBTT_assert(x >= 0 && x < len);
               scanline[x]      += stbtt__position_trapezoid_area(height, x_top, x1f32, x_bottom, x1f32);
               scanline_fill[x] += height; // everything right of this pixel is filled
            } else {
               // x: c_int,x1,x2;y_crossing: c_float, y_final, step, sign, area;
               let mut x: c_int = 0;
                let mut x1: c_int = 0;
                let mut x2: c_int = 0;
                let mut y_crossing: c_float = 0.0;
                let mut y_final: c_float = 0.0;
                let mut step: c_float = 0.0;
                let mut sign: c_float = 0.0;
                let mut area: c_float = 0.0;


                // covers 2+ pixels
               if x_top > x_bottom {
                  // flip scanline vertically; signed area is the same
                  let mut t: c_float = 0.0;
                  sy0 = y_bottom - (sy0 - y_top);
                  sy1 = y_bottom - (sy1 - y_top);
                  t = sy0;
                   sy0 = sy1;
                   sy1 = t;
                  t = x_bottom;
                   x_bottom = x_top;
                   x_top = t;
                  dx = -dx;
                  dy = -dy;
                  t = x0;
                   x0 = xb;
                   xb = t;
               }
               // STBTT_assert(dy >= 0);
               // STBTT_assert(dx >= 0);

               x1 = x_top as c_int;
               x2 = x_bottom as c_int;
               // compute intersection with y axis at x1+1
               y_crossing = y_top + dy * (x11 - x0);

               // compute intersection with y axis at x2
               y_final = y_top + dy * (x2 - x0);

               //           x1    x_top                            x2    x_bottom
               //     y_top  +------|-----+------------+------------+--------|---+------------+
               //            |            |            |            |            |            |
               //            |            |            |            |            |            |
               //       sy0  |      Txxxxx|............|............|............|............|
               // y_crossing |            *xxxxx.......|............|............|............|
               //            |            |     xxxxx..|............|............|............|
               //            |            |     /-   xx*xxxx........|............|............|
               //            |            | dy <       |    xxxxxx..|............|............|
               //   y_final  |            |     \-     |          xx*xxx.........|............|
               //       sy1  |            |            |            |   xxxxxB...|............|
               //            |            |            |            |            |            |
               //            |            |            |            |            |            |
               //  y_bottom  +------------+------------+------------+------------+------------+
               //
               // goal is to measure the area covered by '.' in each pixel

               // if x2 is right at the right edge of x1, y_crossing can blow up, github #1057
               // @TODO: maybe test against sy1 rather than y_bottom?
               if y_crossing > y_bottom {
                   y_crossing = y_bottom;
               }

               sign = e.direction;

               // area of the rectangle covered from sy0..y_crossing
               area = sign * (y_crossing-sy0);

               // area of the triangle (x_top,sy0), (x1+1,sy0), (x1+1,y_crossing)
               scanline[x1] += stbtt__sized_triangle_area2(area, x11 - x_top);

               // check if final y_crossing is blown up; no test case for this
               if y_final > y_bottom {
                  let denom: c_int = (x2 - (x11));
                  y_final = y_bottom;
                  if denom != 0 { // [DEAR IMGUI] Avoid div by zero (https://github.com/nothings/stb/issues/1316)
                     dy = (y_final - y_crossing ) / denom; // if denom=0, y_final = y_crossing, so y_final <= y_bottom
                  }
               }

               // in second pixel, area covered by line segment found in first pixel
               // is always a rectangle 1 wide * the height of that line segment; this
               // is exactly what the variable 'area' stores. it also gets a contribution
               // from the line segment within it. the THIRD pixel will get the first
               // pixel's rectangle contribution, the second pixel's rectangle contribution,
               // and its own contribution. the 'own contribution' is the same in every pixel except
               // the leftmost and rightmost, a trapezoid that slides down in each pixel.
               // the second pixel's contribution to the third pixel will be the
               // rectangle 1 wide times the height change in the second pixel, which is dy.

               step = sign * dy * 1; // dy is dy/dx, change in y for every 1 change in x,
               // which multiplied by 1-pixel-width is how much pixel area changes for each step in x
               // so the area advances by 'step' every time

               // for (x = x11; x < x2; ++x)
               for x in x11 .. x2
                {
                  scanline[x] += area + step/2; // area of trapezoid is 1*step/2
                  area += step;
               }
               STBTT_assert(STBTT_fabs(area) <= 1.010f32); // accumulated error from area += step unless we round step down
               STBTT_assert(sy1 > y_final-0.010f32);

               // area covered in the last pixel is the rectangle from all the pixels to the left,
               // plus the trapezoid filled by the line segment in this pixel all the way to the right edge
               scanline[x2] += area + sign * stbtt__position_trapezoid_area(sy1-y_final, x2 as c_float, x21f32, x_bottom, x21f32);

               // the rest of the line is filled based on the total height of the line segment in this pixel
               scanline_fill[x2] += sign * (sy1-sy0);
            }
         } else {
            // if edge goes outside of box we're drawing, we require
            // clipping logic. since this does not match the intended use
            // of this library, we use a different, very slow brute
            // force implementation
            // note though that this does happen some of the time because
            // x_top and x_bottom can be extrapolated at the top & bottom of
            // the shape and actually lie outside the bounding box
            let mut x: c_int = 0;
            // for (x=0; x < len; ++x)
            for x in 0 .. len
             {
               // cases:
               //
               // there can be up to two intersections with the pixel. any intersection
               // with left or right edges can be handled by splitting into two (or three)
               // regions. intersections with top & bottom do not necessitate case-wise logic.
               //
               // the old way of doing this found the intersections with the left & right edges,
               // then used some simple logic to produce up to three segments in sorted order
               // from top-to-bottom. however, this had a problem: if an x edge was epsilon
               // across the x border, then the corresponding y position might not be distinct
               // from the other y segment, and it might ignored as an empty segment. to avoid
               // that, we need to explicitly produce segments based on x positions.

               // rename variables to clearly-defined pairs
               let y0: c_float =  y_top;
               let x1: c_float = (x) as c_float;
               let x2: c_float =   (x1);
               let x3: c_float =  xb;
               let y3: c_float =  y_bottom;

               // x = e->x + e->dx * (y-y_top)
               // (y-y_top) = (x - e->x) / e->dx
               // y = (x - e->x) / e->dx + y_top
               let y1: c_float =  (x - x0) / dx + y_top;
               let y2: c_float =  (x1 - x0) / dx + y_top;

               if x0 < x1 && x3 > x2 {         // three segments descending down-right
                  stbtt__handle_clipped_edge(scanline,x,e, x0,y0, x1,y1);
                  stbtt__handle_clipped_edge(scanline,x,e, x1,y1, x2,y2);
                  stbtt__handle_clipped_edge(scanline,x,e, x2,y2, x3,y3);
               } else if x3 < x1 && x0 > x2 {  // three segments descending down-left
                  stbtt__handle_clipped_edge(scanline,x,e, x0,y0, x2,y2);
                  stbtt__handle_clipped_edge(scanline,x,e, x2,y2, x1,y1);
                  stbtt__handle_clipped_edge(scanline,x,e, x1,y1, x3,y3);
               } else if x0 < x1 && x3 > x1 {  // two segments across x, down-right
                  stbtt__handle_clipped_edge(scanline,x,e, x0,y0, x1,y1);
                  stbtt__handle_clipped_edge(scanline,x,e, x1,y1, x3,y3);
               } else if x3 < x1 && x0 > x1 {  // two segments across x, down-left
                  stbtt__handle_clipped_edge(scanline,x,e, x0,y0, x1,y1);
                  stbtt__handle_clipped_edge(scanline,x,e, x1,y1, x3,y3);
               } else if x0 < x2 && x3 > x2 {  // two segments across x+1, down-right
                  stbtt__handle_clipped_edge(scanline,x,e, x0,y0, x2,y2);
                  stbtt__handle_clipped_edge(scanline,x,e, x2,y2, x3,y3);
               } else if x3 < x2 && x0 > x2 {  // two segments across x+1, down-left
                  stbtt__handle_clipped_edge(scanline,x,e, x0,y0, x2,y2);
                  stbtt__handle_clipped_edge(scanline,x,e, x2,y2, x3,y3);
               } else {  // one segment
                  stbtt__handle_clipped_edge(scanline,x,e, x0,y0, x3,y3);
               }
            }
         }
      }
      e = e.next;
   }
}

// directly AA rasterize edges w/o supersampling
pub unsafe fn stbtt__rasterize_sorted_edges2(result: *mut stbtt__bitmap, mut e: *mut stbtt__edge, n: c_int, vsubsample: c_int, off_x: c_int, off_y: c_int, userdata: *mut c_void)
{
   // stbtt__hheap hh = { 0, 0, 0 };
   let mut hh = stbtt__hheap::default();
   let mut active: *mut stbtt__active_edge= None;
   // y: c_int,j=0, i;scanline_data: c_float[129], *scanline, *scanline2;
    let mut y: c_int = 0;
    let mut j: c_int = 0;
    let mut i: c_int = 0;
    let mut scanline_data: [c_float;129] = [0.0;129];
    let mut scanline: &mut c_float = None;
    let mut scanline2: &mut c_float = None;

   // STBTT__NOTUSED(vsubsample);

   if result.w > 64 {
       scanline = STBTT_malloc((result.w * 21) * sizeof, userdata);
   }
   else {
       scanline = scanline_data.as_mut_ptr();
   }

   scanline2 = scanline + result.w;

   y = off_y;
   e[n].y0 =  (off_y + result.h) + 1;

   while (j < result.h) {
      // find center of pixel for this scanlinescan_y_top: c_float    = y + 0.0;
      let scan_y_bottom: c_float =  y + 1.0;
      stbtt__active_edge **step = &active;

      STBTT_memset(scanline , 0, result.w*sizeof(scanline[0]));
      STBTT_memset(scanline2, 0, (result.w1)*sizeof(scanline[0]));

      // update all active edges;
      // remove all active edges that terminate before the top of this scanline
      while (*step) {
         stbtt__active_edge * z = *step;
         if (z.ey <= scan_y_top) {
            *step = z.next; // delete from list
            STBTT_assert(z.direction);
            z.direction = 0;
            stbtt__hheap_free(&mut hh, z);
         } else {
            step = &((*step).next); // advance through list
         }
      }

      // insert all edges that start before the bottom of this scanline
      while (e.y0 <= scan_y_bottom) {
         if (e.y0 != e.y1) {
            z: *mut stbtt__active_edge = stbtt__new_active(&mut hh, e, off_x, scan_y_top, userdata);
            if (z != null_mut()) {
               if (j == 0 && off_y != 0) {
                  if (z.ey < scan_y_top) {
                     // this can happen due to subpixel positioning and some kind of fp rounding error i think
                     z.ey = scan_y_top;
                  }
               }
               STBTT_assert(z.ey >= scan_y_top); // if we get really unlucky a tiny bit of an edge can be out of bounds
               // insert at front
               z.next = active;
               active = z;
            }
         }
         e += 1;
      }

      // now process all active edges
      if (active) {
         stbtt__fill_active_edges_new(scanline, scanline21, result.w, active, scan_y_top);


         let mut sum: c_float =  0.0;
         // for (i=0; i < result.w; ++i)
         for i in 0 .. result.w
          {
            let mut k: c_float = 0.0;
            let mut m: c_int = 0;
            sum += scanline2[i];
            k = scanline[i] + sum;
            k =  STBTT_fabs(k)*255 + 0.5;
            m = k as c_int;
            if m > 255 { m = 255; }
            result.pixels[j*result.stride + i] =  m;
         }
      }
      // advance all the edges
      step = &active;
      while *step {
         let mut z: *mut stbtt__active_edge = *step;
         z.fx += z.fdx; // advance to position for current scanline
         step = &((*step).next); // advance through list
      }

      y += 1;
      j += 1;
   }

   stbtt__hheap_cleanup(&mut hh, userdata);

   if (scanline != scanline_data.as_mut_ptr()) {
       STBTT_free(scanline, userdata);
   }
}
// #else
// #error "Unrecognized value of STBTT_RASTERIZER_VERSION"
// #endif

// #define STBTT__COMPARE(a,b)  ((a)->y0 < (b)->y0)

pub unsafe fn stbtt__sort_edges_ins_sort(p: *mut stbtt__edge, n: c_int) {
    // i: c_int,j;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    // for (i=1; i < n; ++i)
    for i in 1..n {
        let mut t = p[i];
        let mut a = &mut t;
        j = i;
        while j > 0 {
            let mut b: *mut stbtt__edge = &mut p[j - 1];
            let c: c_int = STBTT__COMPARE(a, b);
            if (!c) { break; }
            p[j] = p[j - 1];
            j -= 1;
        }
        if (i != j) {
            p[j] = t;
        }
    }
}

pub unsafe fn stbtt__sort_edges_quicksort(mut p: *mut stbtt__edge, mut n: c_int)
{
   /* threshold for transitioning to insertion sort */
   while (n > 12) {
      // stbtt__edge t;
      let mut t = stbtt__edge::default();
       // c01: c_int,c12,c,m,i,j;
        let mut c01: c_int = 0;
       let mut c12: c_int = 0;
       let mut c: c_int = 0;
       let mut m: c_int = 0;
       let mut i: c_int = 0;
       let mut j: c_int = 0;

      /* compute median of three */
      m = n >> 1;
      c01 = STBTT__COMPARE(&p[0],&p[m]);
      c12 = STBTT__COMPARE(&p[m],&p[n-1]);
      /* if 0 >= mid >= end, or 0 < mid < end, then use mid */
      if (c01 != c12) {
         /* otherwise, we'll need to swap something else to middle */
         let mut z: c_int = 0;
         c = STBTT__COMPARE(&p[0],&p[n-1]);
         /* 0>mid && mid<n:  0>n => n; 0<n => 0 */
         /* 0<mid && mid>n:  0>n => 0; 0<n => n */
         z = if c == c12 { 0} else { n-1};
         t = p[z];
         p[z] = p[m];
         p[m] = t;
      }
      /* now p[m] is the median-of-three */
      /* swap it to the beginning so it won't move around */
      t = p[0];
      p[0] = p[m];
      p[m] = t;

      /* partition loop */
      i=1;
      j=n-1;
      loop {
         /* handling of equality is crucial here */
         /* for sentinels & efficiency with duplicates */
         loop {
            if (!STBTT__COMPARE(&p[i], &p[0])) {
                break;
            }
             i += 1;
         }
         loop {
            if (!STBTT__COMPARE(&p[0], &p[j])) { break; }
             j -= 1;
         }
         /* make sure we haven't crossed */
         if (i >= j) { break; }
         t = p[i];
         p[i] = p[j];
         p[j] = t;

         i += 1;
         j -= 1;
      }
      /* recurse on smaller side, iterate on larger */
      if j < (n-i) {
         stbtt__sort_edges_quicksort(p,j);
         p = p+i;
         n = n-i;
      } else {
         stbtt__sort_edges_quicksort(p+i, n-i);
         n = j;
      }
   }
}

pub unsafe fn stbtt__sort_edges(p: *mut stbtt__edge, n: c_int)
{
   stbtt__sort_edges_quicksort(p, n);
   stbtt__sort_edges_ins_sort(p, n);
}


pub unsafe fn stbtt__rasterize(result: *mut stbtt__bitmap, pts: *mut stbtt__point, wcount: *mut c_int, windings: c_int, scale_x: c_float, scale_y: c_float, shift_x: c_float, shift_y: c_float, off_x: c_int, off_y: c_int, invert: c_int, userdata: *mut c_void) {
    let y_scale_inv: c_float = if invert { -scale_y } else { scale_y };
    e: *mut stbtt__edge;
    // n: c_int,i,j,k,m;
    let mut n: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut k: c_int = 0;
    let mut m: c_int = 0;
// #if STBTT_RASTERIZER_VERSION == 1
    let vsubsample: c_int = if result.h < 8 { 15 } else { 5 };
// #elif STBTT_RASTERIZER_VERSION == 2
    let vsubsample: c_int = 1;
// #else
//    #error "Unrecognized value of STBTT_RASTERIZER_VERSION"
// #endif
    // vsubsample should divide 255 evenly; otherwise we won't reach full opacity

    // now we have to blow out the windings into explicit edge lists
    n = 0;
    // for (i=0; i < windings; ++i)
    for i in 0..windings {
        n += wcount[i];
    }

    e = STBTT_malloc(sizeof(*e) * (n1), userdata); // add an extra one as a sentinel
    if e == 0 { return; }
    n = 0;

    m = 0;
    // for (i=0; i < windings; ++i)
    for i in 0..windings {
        p: *mut stbtt__point = pts + m;
        m += wcount[i];
        j = wcount[i] - 1;
        // for (k=0; k < wcount[i]; j=k++)
        for k in 0..wcount[i] {
            let mut a: c_int = k;
            b = j;
            // skip the edge if horizontal
            if (p[j].y == p[k].y) {
                continue;
            }
            // add edge from j to k to the list
            e[n].invert = 0;
            if (if invert { p[j].y > p[k].y } else { p[j].y < p[k].y }) {
                e[n].invert = 1;
                a = j;
                b = k;
            }
            e[n].x0 = p[a].x * scale_x + shift_x;
            e[n].y0 = (p[a].y * y_scale_inv + shift_y) * vsubsample;
            e[n].x1 = p[b].x * scale_x + shift_x;
            e[n].y1 = (p[b].y * y_scale_inv + shift_y) * vsubsample;
            n += 1;
            j = k;
        }
    }

    // now sort the edges by their highest point (should snap to integer, and then by x)
    //STBTT_sort(e, n, sizeof(e[0]), stbtt__edge_compare);
    stbtt__sort_edges(e, n);

    // now, traverse the scanlines and find the intersections on each scanline, use xor winding rule
    stbtt__rasterize_sorted_edges(result, e, n, vsubsample, off_x, off_y, userdata);

    STBTT_free(e, userdata);
}

pub unsafe fn stbtt__add_point(points: *mut stbtt__point, n: c_int,x: c_float,y: c_float)
{
   if (!points) { return; } // during first pass, it's unallocated
   points[n].x = x;
   points[n].y = y;
}

// tessellate until threshold p is happy... @TODO warped to compensate for non-linear stretching
pub unsafe fn stbtt__tesselate_curve(points: *mut stbtt__point, num_points: *mut c_int,x0: c_float,y0: c_float,x1: c_float,y1: c_float,x2: c_float,y2: c_float,objspace_flatness_squared: c_float, n: c_int) -> c_int
{
   // midpoint
   let mx: c_float =  (x0 + 2*x1 + x2)/4;
   let my: c_float =  (y0 + 2*y1 + y2)/4;
   // versus directly drawn line
   let dx: c_float =  (x0+x2)/2 - mx;
   let dy: c_float =  (y0+y2)/2 - my;
   if (n > 16) { // 65536 segments on one curve better be enough!
       return 1;
   }
   if (dx*dx+dy*dy > objspace_flatness_squared) { // half-pixel error allowed... need to be smaller if AA
      stbtt__tesselate_curve(points, num_points, x0,y0, (x0+x1)/2.0,(y0+y1)/2.0, mx,my, objspace_flatness_squared,n1);
      stbtt__tesselate_curve(points, num_points, mx,my, (x1+x2)/2.0,(y1+y2)/2.0, x2,y2, objspace_flatness_squared,n1);
   } else {
      stbtt__add_point(points, *num_points,x2,y2);
      *num_points = *num_points1;
   }
   return 1;
}

pub unsafe fn stbtt__tesselate_cubic(points: *mut stbtt__point, num_points: *mut c_int,x0: c_float,y0: c_float,x1: c_float,y1: c_float,x2: c_float,y2: c_float,x3: c_float,y3: c_float,objspace_flatness_squared: c_float, n: c_int)
{
   // @TODO this "flatness" calculation is just made-up nonsense that seems to work well enough
   let dx0: c_float =  x1-x0;
   let dy0: c_float =  y1-y0;
   let dx1: c_float =  x2-x1;
   let dy1: c_float =  y2-y1;
   let dx2: c_float =  x3-x2;
   let dy2: c_float =  y3-y2;
   let dx: c_float =  x3-x0;
   let dy: c_float =  y3-y0;
   let longlen: c_float =   (STBTT_sqrt(dx0*dx0+dy0*dy0)+STBTT_sqrt(dx1*dx1+dy1*dy1)+STBTT_sqrt(dx2*dx2+dy2*dy2));
   let shortlen: c_float =   STBTT_sqrt(dx*dx+dy*dy);
   let flatness_squared: c_float =  longlen*longlen-shortlen*shortlen;

    // 65536 segments on one curve better be enough!
   if (n > 16) {
       return;
   }
   if (flatness_squared > objspace_flatness_squared) {
      let x01: c_float =  (x0+x1)/2;
      let y01: c_float =  (y0+y1)/2;
      let x12: c_float =  (x1+x2)/2;
      let y12: c_float =  (y1+y2)/2;
      let x23: c_float =  (x2+x3)/2;
      let y23: c_float =  (y2+y3)/2;

      let xa: c_float =  (x01+x12)/2;
      let ya: c_float =  (y01+y12)/2;
      let xb: c_float =  (x12+x23)/2;
      let yb: c_float =  (y12+y23)/2;

      let mx: c_float =  (xa+xb)/2;
      let my: c_float =  (ya+yb)/2;

      stbtt__tesselate_cubic(points, num_points, x0,y0, x01,y01, xa,ya, mx,my, objspace_flatness_squared,n1);
      stbtt__tesselate_cubic(points, num_points, mx,my, xb,yb, x23,y23, x3,y3, objspace_flatness_squared,n1);
   } else {
      stbtt__add_point(points, *num_points,x3,y3);
      *num_points = *num_points1;
   }
}

// returns number of contours
pub unsafe fn stbtt_FlattenCurves(vertices: *mut stbtt_vertex, num_verts: c_int,objspace_flatness: c_float, contour_lengths: *mut *mut c_int, num_contours: *mut c_int, userdata: *mut c_void) -> *mut stbtt__point
{
   let mut points: *mut stbtt__point=None;
   let mut num_points: c_int = 0;

   let objspace_flatness_squared: c_float =  objspace_flatness * objspace_flatness;
   // i: c_int,n=0,start=0, pass;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut start: c_int = 0;
    let mut pass: c_int = 0;

   // count how many "moves" there are to get the contour count
   // for (i=0; i < num_verts; ++i)
   for i in 0 .. num_verts
    {
        if (vertices[i].vertex_type == STBTT_vmove)
       {
            n += 1;}
   }

   *num_contours = n;
   if n == 0 { return  None; }

   *contour_lengths = STBTT_malloc(sizeof(**contour_lengths) * n, userdata);

   if (*contour_lengths).is_null() {
      *num_contours = 0;
      return None;
   }

   // make two passes through the points so we don't need to realloc
   // for (pass=0; pass < 2; ++pass)
   for pass in 0 .. 2
    {
       // x: c_float=0,y=0;
      let mut x: c_float = 0.0;
        let mut y: c_float = 0.0;
        if (pass == 1) {
         points = STBTT_malloc(num_points * sizeof(points[0]), userdata);
         // if points == None {  goto error(); }
      }
      num_points = 0;
      n= -1;
      // for (i=0; i < num_verts; ++i)
      for i in 0 .. num_verts
        {
         match vertices[i].vertex_type {
            STBTT_vmove => {
                // start the next contour
                if (n >= 0) { (*contour_lengths)[n] = num_points - start; }
                n += 1;
                start = num_points;

                x = vertices[i].x;
                y = vertices[i].y;
                stbtt__add_point(points, num_points, x, y);
                num_points += 1;
            }
               // break;
            STBTT_vline => {
                x = vertices[i].x;
                y = vertices[i].y;
                stbtt__add_point(points, num_points, x, y);
                num_points += 1;
            }
               // break;
            STBTT_vcurve => {
                stbtt__tesselate_curve(points, &mut num_points, x, y,
                                       vertices[i].cx, vertices[i].cy,
                                       vertices[i].x, vertices[i].y,
                                       objspace_flatness_squared, 0);
                x = vertices[i].x;
                y = vertices[i].y;
            }
               // break;
            STBTT_vcubic => {
                stbtt__tesselate_cubic(points, &mut num_points, x, y,
                                       vertices[i].cx, vertices[i].cy,
                                       vertices[i].cx1, vertices[i].cy1,
                                       vertices[i].x, vertices[i].y,
                                       objspace_flatness_squared, 0);
                x = vertices[i].x;
                y = vertices[i].y;
            }
               // break;
         }
      }
      (*contour_lengths)[n] = num_points - start;
   }

   return points;
// error:
//    STBTT_free(points, userdata);
//    STBTT_free(*contour_lengths, userdata);
//    *contour_lengths = 0;
//    *num_contours = 0;
//    return None;
}

pub unsafe fn stbtt_Rasterize(result: *mut stbtt__bitmap,flatness_in_pixels: c_float, vertices: *mut stbtt_vertex, num_verts: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, x_off: c_int, y_off: c_int, invert: c_int, userdata: *mut c_void)
{
    let mut scale: c_float            = if scale_x > scale_y { scale_y } else { scale_x };
   let mut winding_count: c_int =  0;
   winding_lengths: *mut c_int   = None;
   windings: *mut stbtt__point = stbtt_FlattenCurves(vertices, num_verts, flatness_in_pixels / scale, &mut winding_lengths, &mut winding_count, userdata);
   if windings {
      stbtt__rasterize(result, windings, winding_lengths, winding_count, scale_x, scale_y, shift_x, shift_y, x_off, y_off, invert, userdata);
      STBTT_free(winding_lengths, userdata);
      STBTT_free(windings, userdata);
   }
}

pub unsafe fn stbtt_FreeBitmap(c_ubitmap: *mut c_char, userdata: *mut c_void)
{
   STBTT_free(bitmap, userdata);
}

pub unsafe fn  c_ustbtt_GetGlyphBitmapSubpixel(info: *const stbtt_fontinfo, mut scale_x: c_float, mut scale_y: c_float, shift_x: c_float, shift_y: c_float, glyph: c_int, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int) -> *mut c_char
{
   // ix0: c_int,iy0,ix1,iy1;
   let mut ix0: c_int = 0;
    let mut iy0: c_int = 0;
    let mut ix1: c_int = 0;
    let mut iy1: c_int = 0;
    // stbtt__bitmap gbm;
   let mut gbm = stbtt__bitmap::default();
    let mut vertices: *mut stbtt_vertex = None;
   let num_verts: c_int = stbtt_GetGlyphShape(info, glyph, &mut vertices);

   if scale_x == 0.0 {  scale_x = scale_y;}
   if scale_y == 0.0 {
      if scale_x == 0.0 {
         STBTT_free(vertices, info.userdata);
         return None;
      }
      scale_y = scale_x;
   }

   stbtt_GetGlyphBitmapBoxSubpixel(info, glyph, scale_x, scale_y, shift_x, shift_y, &mut ix0,&mut iy0,&mut ix1,&mut iy1);

   // now we get the size
   gbm.w = (ix1 - ix0);
   gbm.h = (iy1 - iy0);
   gbm.pixels= None; // in case we error

   if (width ) { *width = gbm.w; }
   if (height) { *height = gbm.h; }
   if (xoff  ) { *xoff = ix0; }
   if (yoff  ) { *yoff = iy0; }

   if (gbm.w && gbm.h) {
      gbm.pixels = STBTT_malloc(gbm.w * gbm.h, info.userdata);
      if (gbm.pixels) {
         gbm.stride = gbm.w;

         stbtt_Rasterize(&mut gbm, 0.35, vertices, num_verts, scale_x, scale_y, shift_x, shift_y, ix0, iy0, 1, info.userdata);
      }
   }
   STBTT_free(vertices, info.userdata);
   return gbm.pixels;
}

pub unsafe fn  c_ustbtt_GetGlyphBitmap(info: *const stbtt_fontinfo,scale_x: c_float,scale_y: c_float, glyph: c_int, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int) -> *mut c_char
{
   return stbtt_GetGlyphBitmapSubpixel(info, scale_x, scale_y, 0.0, 0.0, glyph, width, height, xoff, yoff);
}

pub unsafe fn stbtt_MakeGlyphBitmapSubpixel(info: *const stbtt_fontinfo, c_uoutput: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, glyph: c_int)
{
   // ix0: c_int,iy0;
   let mut ix0: c_int = 0;
    let mut iy0: c_int = 0;
    let mut vertices: *mut stbtt_vertex = None;
   let num_verts: c_int = stbtt_GetGlyphShape(info, glyph, &mut vertices);
   // stbtt__bitmap gbm;
    let mut gbm = stbtt__bitmap::default();

   stbtt_GetGlyphBitmapBoxSubpixel(info, glyph, scale_x, scale_y, shift_x, shift_y, &mut ix0,&mut iy0,None,null_mut());
   gbm.pixels = output;
   gbm.w = out_w;
   gbm.h = out_h;
   gbm.stride = out_stride;

   if gbm.w && gbm.h {
       stbtt_Rasterize(&mut gbm, 0.35, vertices, num_verts, scale_x, scale_y, shift_x, shift_y, ix0, iy0, 1, info.userdata);
   }

   STBTT_free(vertices, info.userdata);
}

pub unsafe fn stbtt_MakeGlyphBitmap(info: *const stbtt_fontinfo, output: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float, glyph: c_int)
{
   stbtt_MakeGlyphBitmapSubpixel(info, output, out_w, out_h, out_stride, scale_x, scale_y, 0.0,0.0, glyph);
}

pub unsafe fn c_ustbtt_GetCodepointBitmapSubpixel(info: *const stbtt_fontinfo,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, codepoint: c_int, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int) -> *mut c_char
{
   return stbtt_GetGlyphBitmapSubpixel(info, scale_x, scale_y,shift_x,shift_y, stbtt_FindGlyphIndex(info,codepoint), width,height,xoff,yoff);
}

pub unsafe fn stbtt_MakeCodepointBitmapSubpixelPrefilter(info: *const stbtt_fontinfo, output: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, oversample_x: c_int, oversample_y: c_int, sub_x: &mut c_float, sub_y: &mut c_float, codepoint: c_int)
{
   stbtt_MakeGlyphBitmapSubpixelPrefilter(info, output, out_w, out_h, out_stride, scale_x, scale_y, shift_x, shift_y, oversample_x, oversample_y, sub_x, sub_y, stbtt_FindGlyphIndex(info,codepoint));
}

pub unsafe fn stbtt_MakeCodepointBitmapSubpixel(info: *const stbtt_fontinfo, output: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, codepoint: c_int)
{
   stbtt_MakeGlyphBitmapSubpixel(info, output, out_w, out_h, out_stride, scale_x, scale_y, shift_x, shift_y, stbtt_FindGlyphIndex(info,codepoint));
}

pub unsafe fn c_ustbtt_GetCodepointBitmap(info: *const stbtt_fontinfo,scale_x: c_float,scale_y: c_float, codepoint: c_int, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int) -> *mut c_char
{
   return stbtt_GetCodepointBitmapSubpixel(info, scale_x, scale_y, 0.0,0.0, codepoint, width,height,xoff,yoff);
}

pub unsafe fn stbtt_MakeCodepointBitmap(info: *const stbtt_fontinfo, output: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float, codepoint: c_int)
{
   stbtt_MakeCodepointBitmapSubpixel(info, output, out_w, out_h, out_stride, scale_x, scale_y, 0.0,0.0, codepoint);
}

//////////////////////////////////////////////////////////////////////////////
//
// bitmap baking
//
// This is SUPER-CRAPPY packing to keep source code small

pub unsafe fn stbtt_BakeFontBitmap_internal(c_udata: *const c_uchar, offset: c_int,  // font location (use offset=0 for plain .tt0f32) ->
                                     pixel_height: c_float,                     // height of font in pixels
                                     c_upixels: *mut c_uchar,
                                     pw: c_int,
                                     ph: c_int,  // bitmap to be filled in
                                     first_char: c_int,
                                     num_chars: c_int,          // characters to bake
                                     stbtt_bakedchardata: *mut c_uchar) -> c_int
{
   let mut scale: c_float = 0.0;
   // x: c_int,y,bottom_y, i;
   let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut bottom_y: c_int = 0;
    let mut i: c_int = 0;
    let mut f = stbtt_fontinfo::default();
   f.userdata= None;
   if !stbtt_InitFont(&mut f, data, offset) {
       return -1;
   }
   STBTT_memset(pixels, 0, pw*ph); // background of 0 around pixels
   x=1;y=1;
   bottom_y = 1;

   scale = stbtt_ScaleForPixelHeight(&f, pixel_height);

   // for (i=0; i < num_chars; ++i)
   for i in 0 .. num_chars
    {
      // advance: c_int, lsb, x0,y0,x1,y1,gw,gh;
      let mut advance: c_int = 0;
        let mut lsb: c_int = 0;
        let mut x0: c_int = 0;
        let mut y0:c_int = 0;
        let mut x1: c_int = 0;
        let mut y1: c_int = 0;
        let mut gw: c_int = 0;
        let mut gh: c_int = 0;

        let g: c_int = stbtt_FindGlyphIndex(&f, first_char + i);
      stbtt_GetGlyphHMetrics(&f, g, &mut advance, &mut lsb);
      stbtt_GetGlyphBitmapBox(&f, g, scale,scale, &mut x0,&mut y0,&mut x1,&mut y1);
      gw = x1-x0;
      gh = y1-y0;
      if (x + gw + 1 >= pw) {
          y = bottom_y;
          x = 1;
      } // advance to next row
      if (y + gh + 1 >= ph) { // check if it fits vertically AFTER potentially moving to next row
          return -i;
      }
      STBTT_assert(x+gw < pw);
      STBTT_assert(y+gh < ph);
      stbtt_MakeGlyphBitmap(&f, pixels+x+y*pw, gw,gh,pw, scale,scale, g);
      chardata[i].x0 =  x;
      chardata[i].y0 =  y;
      chardata[i].x1 =  (x + gw);
      chardata[i].y1 =  (y + gh);
      chardata[i].xadvance = scale * advance;
      chardata[i].xoff     =  x0;
      chardata[i].yoff     =  y0;
      x = x + gw + 1;
      if (y+gh1 > bottom_y) {
          bottom_y = y + gh1;
      }
   }
   return bottom_y;
}

pub unsafe fn stbtt_GetBakedQuad(stbtt_bakedchardata: *mut c_char, pw: c_int, ph: c_int, char_index: c_int, xpos: &mut c_float, ypos: &mut c_float, q: *mut stbtt_aligned_quad, opengl_fillrule: c_int)
{
   let d3d_bias: c_float =  if opengl_fillrule { 0.0 } else { -0.5 };
   let ipw: c_float =  1.0 / pw;
    let iph = 1.0 / ph;
   const stbtt_bakedb: *mut c_char = chardata + char_index;
   let round_x: c_int = STBTT_ifloor((*xpos + b.xoff) + 0.5);
   let round_y: c_int = STBTT_ifloor((*ypos + b.yoff) + 0.5);

   q.x0 = round_x + d3d_bias;
   q.y0 = round_y + d3d_bias;
   q.x1 = round_x + b.x1 - b.x0 + d3d_bias;
   q.y1 = round_y + b.y1 - b.y0 + d3d_bias;

   q.s0 = b.x0 * ipw;
   q.t0 = b.y0 * iph;
   q.s1 = b.x1 * ipw;
   q.t1 = b.y1 * iph;

   *xpos += b.xadvance;
}

//////////////////////////////////////////////////////////////////////////////
//
// rectangle packing replacement routines if you don't have stb_rect_pack.h
//

// #ifndef STB_RECT_PACK_VERSION

// typedef let mut stbrp_coord: c_int = 0;

////////////////////////////////////////////////////////////////////////////////////
//                                                                                //
//                                                                                //
// COMPILER WARNING ?!?!?                                                         //
//                                                                                //
//                                                                                //
// if you get a compile warning due to these symbols being defined more than      //
// once, move #include "stb_rect_pack.h" before #include "stb_truetype.h"         //
//                                                                                //
////////////////////////////////////////////////////////////////////////////////////







pub unsafe fn stbrp_init_target(con: *mut stbrp_context, pw: c_int, ph: c_int, nodes: *mut stbrp_node, num_nodes: c_int)
{
   con.width  = pw;
   con.height = ph;
   con.x = 0;
   con.y = 0;
   con.bottom_y = 0;
   STBTT__NOTUSED(nodes);
   STBTT__NOTUSED(num_nodes);
}

pub unsafe fn stbrp_pack_rects(con: *mut stbrp_context, rects: *mut stbrp_rect, num_rects: c_int) {
    let mut i: c_int = 0;
    // for (i=0; i < num_rects; ++i)
    for i in 0..num_rects {
        if (con.x + rects[i].w > con.width) {
            con.x = 0;
            con.y = con.bottom_y;
        }
        if (con.y + rects[i].h > con.height) {
            break;
        }
        rects[i].x = con.x;
        rects[i].y = con.y;
        rects[i].was_packed = 1;
        con.x += rects[i].w;
        if (con.y + rects[i].h > con.bottom_y) {
            con.bottom_y = con.y + rects[i].h;
        }
    }
    // for (   ; i < num_rects; ++i)
    while i < num_rects {
        rects[i].was_packed = 0;
        i += 1;
    }
}
// #endif

//////////////////////////////////////////////////////////////////////////////
//
// bitmap baking
//
// This is SUPER-AWESOME (tm Ryan Gordon) packing using stb_rect_pack.h. If
// stb_rect_pack.h isn't available, it uses the BakeFontBitmap strategy.

pub unsafe fn stbtt_PackBegin(spc: *mut stbtt_pack_context, c_upixels: *mut c_char, pw: c_int, ph: c_int, stride_in_bytes: c_int, padding: c_int, alloc_context: *mut c_void) -> c_int
{
   let mut context: *mut stbrp_context = STBTT_malloc(sizeof(*context),alloc_context);
   let mut num_nodes: c_int =  pw - padding;
   let mut nodes: *mut stbrp_node   = STBTT_malloc(sizeof(*nodes  ) * num_nodes,alloc_context);

   if context == None || nodes == None {
      if context != None { STBTT_free(context, alloc_context); }
      if nodes   != None { STBTT_free(nodes, alloc_context); }
      return 0;
   }

   spc.user_allocator_context = alloc_context;
   spc.width = pw;
   spc.height = ph;
   spc.pixels = pixels;
   spc.pack_info = context;
   spc.nodes = nodes;
   spc.padding = padding;
   spc.stride_in_bytes = if stride_in_bytes != 0 { stride_in_bytes } else { pw };
   spc.h_oversample = 1;
   spc.v_oversample = 1;
   spc.skip_missing = 0;

   stbrp_init_target(context, pw-padding, ph-padding, nodes, num_nodes);

   if (pixels) {
       STBTT_memset(pixels, 0, pw * ph);
   } // background of 0 around pixels

   return 1;
}

pub unsafe fn stbtt_PackEnd  (spc: *mut stbtt_pack_context)
{
   STBTT_free(spc.nodes    , spc.user_allocator_context);
   STBTT_free(spc.pack_info, spc.user_allocator_context);
}

pub unsafe fn stbtt_PackSetOversampling(spc: *mut stbtt_pack_context, h_oversample: c_uint, v_oversample: c_uint)
{
   STBTT_assert(h_oversample <= STBTT_MAX_OVERSAMPLE);
   STBTT_assert(v_oversample <= STBTT_MAX_OVERSAMPLE);
   if h_oversample <= STBTT_MAX_OVERSAMPLE{
      spc.h_oversample = h_oversample;}
   if v_oversample <= STBTT_MAX_OVERSAMPLE{
      spc.v_oversample = v_oversample;}
}

pub unsafe fn stbtt_PackSetSkipMissingCodepoints(spc: *mut stbtt_pack_context, skip: c_int)
{
   spc.skip_missing = skip;
}

// #define STBTT__OVER_MASK  (STBTT_MAX_OVERSAMPLE-1)

pub unsafe fn stbtt__h_prefilter(c_upixels: *mut c_char, w: c_int, h: c_int, stride_in_bytes: c_int, kernel_width: c_uint)
{
   let mut buffer: [c_uchar;STBTT_MAX_OVERSAMPLE] = [0;STBTT_MAX_OVERSAMPLE];
   let safe_w: c_int = w - kernel_width;
   let mut j: c_int = 0;
   STBTT_memset(buffer, 0, STBTT_MAX_OVERSAMPLE); // suppress bogus warning from VS2013 -analyze
   // for (j=0; j < h; ++j)
   for j in 0 .. h
    {
      let mut i: c_int = 0;
      total: c_uint;
      STBTT_memset(buffer, 0, kernel_width);

      total = 0;

      // make kernel_width a constant in common cases so compiler can optimize out the divide
      match (kernel_width) {
         2 => {
             // for (i=0; i < = safe_w; + + i)
             for i in 0 .. safe_w
             {
                 total += pixels[i] - buffer[i & STBTT__OVER_MASK];
                 buffer[(i + kernel_width) & STBTT__OVER_MASK] = pixels[i];
                 pixels[i] = (total / 2);
             }
         }
            // break;
         3 => {
             // for (i=0; i < = safe_w; + + i)
             for i in 0 .. safe_w
             {
                 total += pixels[i] - buffer[i & STBTT__OVER_MASK];
                 buffer[(i + kernel_width) & STBTT__OVER_MASK] = pixels[i];
                 pixels[i] = (total / 3);
             }
         }
            // break;
         4 => {
             // for (i=0; i < = safe_w; + + i)
             for i in 0 .. safe_w
             {
                 total += pixels[i] - buffer[i & STBTT__OVER_MASK];
                 buffer[(i + kernel_width) & STBTT__OVER_MASK] = pixels[i];
                 pixels[i] = (total / 4);
             }
         }
            // break;
         5 => {
             // for (i=0; i < = safe_w; + + i)
             for i in 0 .. safe_w
             {
                 total += pixels[i] - buffer[i & STBTT__OVER_MASK];
                 buffer[(i + kernel_width) & STBTT__OVER_MASK] = pixels[i];
                 pixels[i] = (total / 5);
             }
         }
            // break;
         _ => {
             // for (i=0; i < = safe_w; + + i)
             for i in 0 .. safe_w
             {
                 total += pixels[i] - buffer[i & STBTT__OVER_MASK];
                 buffer[(i + kernel_width) & STBTT__OVER_MASK] = pixels[i];
                 pixels[i] = (total / kernel_width);
             }
         }
            // break;
      }

      // for (; i < w; ++i)
      while i < w
        {
         STBTT_assert(pixels[i] == 0);
         total -= buffer[i & STBTT__OVER_MASK];
         pixels[i] =  (total / kernel_width);
            i += 1;
      }

      pixels += stride_in_bytes;
   }
}

pub unsafe fn stbtt__v_prefilter(c_upixels: *mut c_char, w: c_int, h: c_int, stride_in_bytes: c_int, kernel_width: c_uint)
{
   // c_uchar buffer[STBTT_MAX_OVERSAMPLE];
   let mut buffer: [c_uchar;STBTT_MAX_OVERSAMPLE] = [0;STBTT_MAX_OVERSAMPLE];
    let safe_h: c_int = h - kernel_width;
   let mut j: c_int = 0;
   STBTT_memset(buffer, 0, STBTT_MAX_OVERSAMPLE); // suppress bogus warning from VS2013 -analyze
   // for (j=0; j < w; ++j)
   for j in 0 .. w
    {
      let mut i: c_int = 0;
      let mut total: c_uint = 0;
      STBTT_memset(buffer, 0, kernel_width);

      total = 0;

      // make kernel_width a constant in common cases so compiler can optimize out the divide
      match (kernel_width) {
         2 => {
             // for (i=0; i < = safe_h; + + i)
             for i in 0 .. safe_h
             {
                 total += pixels[i * stride_in_bytes] - buffer[i & STBTT__OVER_MASK];
                 buffer[(i + kernel_width) & STBTT__OVER_MASK] = pixels[i * stride_in_bytes];
                 pixels[i * stride_in_bytes] = (total / 2);
             }
         }
            // break;
         3 => {
             // for (i=0; i < = safe_h; + + i)
             for i in 0 .. safe_h
             {
                 total += pixels[i * stride_in_bytes] - buffer[i & STBTT__OVER_MASK];
                 buffer[(i + kernel_width) & STBTT__OVER_MASK] = pixels[i * stride_in_bytes];
                 pixels[i * stride_in_bytes] = (total / 3);
             }
         }
            // break;
         4 => {
             // for (i=0; i < = safe_h; + + i)
             for i in 0 .. safe_h
             {
                 total += pixels[i * stride_in_bytes] - buffer[i & STBTT__OVER_MASK];
                 buffer[(i + kernel_width) & STBTT__OVER_MASK] = pixels[i * stride_in_bytes];
                 pixels[i * stride_in_bytes] = (total / 4);
             }
         }
            // break;
         5 => {
             // for (i=0; i < = safe_h; + + i)
             for i in 0 .. safe_h
             {
                 total += pixels[i * stride_in_bytes] - buffer[i & STBTT__OVER_MASK];
                 buffer[(i + kernel_width) & STBTT__OVER_MASK] = pixels[i * stride_in_bytes];
                 pixels[i * stride_in_bytes] = (total / 5);
             }
         }
            // break;
         _ => {
             // for (i=0; i < = safe_h; + + i)
             for i in 0 .. safe_h
             {
                 total += pixels[i * stride_in_bytes] - buffer[i & STBTT__OVER_MASK];
                 buffer[(i + kernel_width) & STBTT__OVER_MASK] = pixels[i * stride_in_bytes];
                 pixels[i * stride_in_bytes] = (total / kernel_width);
             }
         }
            // break;
      }

      // for (; i < h; ++i)
      while i < h
        {
         STBTT_assert(pixels[i*stride_in_bytes] == 0);
         total -= buffer[i & STBTT__OVER_MASK];
         pixels[i*stride_in_bytes] =  (total / kernel_width);
            i += 1;
      }

      pixels += 1;
   }
}

pub unsafe fn staticstbtt__oversample_shift(oversample: c_int) -> f32
{
   if (!oversample) {
       return 0.0;
   }

   // The prefilter is a box filter of width "oversample",
   // which shifts phase by (oversample - 1)/2 pixels in
   // oversampled space. We want to shift in the opposite
   // direction to counter this.
   return -(oversample - 1) / (2.0 * oversample);
}

// rects array must be big enough to accommodate all characters in the given ranges
pub unsafe fn stbtt_PackFontRangesGatherRects(spc: *mut stbtt_pack_context, info: *const stbtt_fontinfo, ranges: *mut stbtt_pack_range, num_ranges: c_int, rects: *mut stbrp_rect) -> c_int
{
   // i: c_int,j,k;
   let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut k: c_int = 0;
    let mut missing_glyph_added: c_int = 0;

   k=0;
   // for (i=0; i < num_ranges; ++i)
   for i in 0 .. num_ranges
    {
      let fh: c_float =  ranges[i].font_size;
      let scale: c_float =  if fh > 0.0 { stbtt_ScaleForPixelHeight(info, fh) } else { stbtt_ScaleForMappingEmToPixels(info, -fh) };
      ranges[i].h_oversample =  spc.h_oversample;
      ranges[i].v_oversample =  spc.v_oversample;
      // for (j=0; j < ranges[i].num_chars; ++j)
      for j in 0 .. ranges[i].num_chars
        {
         // x0: c_int,y0,x1,y1;
         let mut x0: c_int = 0;
            let mut y0: c_int = 0;
            let mut x1: c_int = 0;
            let mut y1: c_int = 0;
            let codepoint: c_int = if ranges[i].array_of_unicode_codepoints == None { ranges[i].first_unicode_codepoint_in_range + j } else { ranges[i].array_of_unicode_codepoints[j] };
         let glyph: c_int = stbtt_FindGlyphIndex(info, codepoint);
         if glyph == 0 && (spc.skip_missing || missing_glyph_added) {
            rects[k].w = rects[k].h = 0;
         } else {
            stbtt_GetGlyphBitmapBoxSubpixel(info,glyph,
                                            scale * spc.h_oversample,
                                            scale * spc.v_oversample,
                                            0.0,0.0,
                                            &mut x0,&mut y0,&mut x1,&mut y1);
            rects[k].w =  (x1-x0 + spc.padding + spc.h_oversample-1);
            rects[k].h =  (y1-y0 + spc.padding + spc.v_oversample-1);
            if glyph == 0 {
               missing_glyph_added = 1;}
         }
         k += 1;
      }
   }

   return k;
}

pub unsafe fn stbtt_MakeGlyphBitmapSubpixelPrefilter(info: *const stbtt_fontinfo, c_uoutput: *mut c_char, out_w: c_int, out_h: c_int, out_stride: c_int,scale_x: c_float,scale_y: c_float,shift_x: c_float,shift_y: c_float, prefilter_x: c_int, prefilter_y: c_int, sub_x: &mut c_float, sub_y: &mut c_float, glyph: c_int)
{
   stbtt_MakeGlyphBitmapSubpixel(info,
                                 output,
                                 out_w - (prefilter_x - 1),
                                 out_h - (prefilter_y - 1),
                                 out_stride,
                                 scale_x,
                                 scale_y,
                                 shift_x,
                                 shift_y,
                                 glyph);

   if prefilter_x > 1 {
       stbtt__h_prefilter(output, out_w, out_h, out_stride, prefilter_x as c_uint);
   }

   if prefilter_y > 1 {
       stbtt__v_prefilter(output, out_w, out_h, out_stride, prefilter_y as c_uint);
   }

   *sub_x = stbtt__oversample_shift(prefilter_x);
   *sub_y = stbtt__oversample_shift(prefilter_y);
}

// rects array must be big enough to accommodate all characters in the given ranges
pub unsafe fn stbtt_PackFontRangesRenderIntoRects(spc: *mut stbtt_pack_context, info: *const stbtt_fontinfo, ranges: *mut stbtt_pack_range, num_ranges: c_int, rects: *mut stbrp_rect) -> c_int
{
   // i: c_int,j,k, missing_glyph = -1, return_value = 1;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut k: c_int = 0;
    let mut missing_glyph: c_int = -1;
    let mut return_value: c_int = 1;

   // save current values
   let old_h_over: c_int = spc.h_oversample as c_int;
   let old_v_over: c_int = spc.v_oversample as c_int;

   k = 0;
   // for (i=0; i < num_ranges; ++i)
   for i in 0 .. num_ranges
    {
      let fh: c_float =  ranges[i].font_size;
      let scale: c_float =  if fh > 0.0 { stbtt_ScaleForPixelHeight(info, fh) } else { stbtt_ScaleForMappingEmToPixels(info, -fh) };
        // recip_h: c_float,recip_v,sub_x,sub_y;
      let mut recip_h: c_float = 0.0;
        let mut recip_v: c_float = 0.0;
        let mut sub_x: c_float = 0.0;
        let mut sub_y: c_float = 0.0;
        spc.h_oversample = ranges[i].h_oversample;
      spc.v_oversample = ranges[i].v_oversample;
      recip_h = 1.0 / spc.h_oversample;
      recip_v = 1.0 / spc.v_oversample;
      sub_x = stbtt__oversample_shift(spc.h_oversample);
      sub_y = stbtt__oversample_shift(spc.v_oversample);
      // for (j=0; j < ranges[i].num_chars; ++j)
      for j in 0 .. ranges[i].num_chars
        {
         let mut r: *mut stbrp_rect = &mut rects[k];
         if (r.was_packed && r.w != 0 && r.h != 0)
         {
           let mut stbtt_packedbc: *mut c_char = &mut ranges[i].chardata_for_range[j];
            // advance: c_int, lsb, x0,y0,x1,y1;
            let mut advance: c_int = 0;
             let mut lsb: c_int = 0;
             let mut x0: c_int = 0;
             let mut y0: c_int = 0;
             let mut x1: c_int = 0;
             let mut y1: c_int = 0;
             let codepoint: c_int = if ranges[i].array_of_unicode_codepoints == None { ranges[i].first_unicode_codepoint_in_range + j } else { ranges[i].array_of_unicode_codepoints[j] };
            let glyph: c_int = stbtt_FindGlyphIndex(info, codepoint);
            let mut pad =  spc.padding;

            // pad on left and top
            r.x += pad;
            r.y += pad;
            r.w -= pad;
            r.h -= pad;
            stbtt_GetGlyphHMetrics(info, glyph, &mut advance, &mut lsb);
            stbtt_GetGlyphBitmapBox(info, glyph,
                                    scale * spc.h_oversample,
                                    scale * spc.v_oversample,
                                    &mut x0,&mut y0,&mut x1,&mut y1);
            stbtt_MakeGlyphBitmapSubpixel(info,
                                          spc.pixels + r.x + r.y*spc.stride_in_bytes,
                                          r.w - spc.h_oversample1,
                                          r.h - spc.v_oversample1,
                                          spc.stride_in_bytes,
                                          scale * spc.h_oversample,
                                          scale * spc.v_oversample,
                                          0.0,0.0,
                                          glyph);

            if spc.h_oversample > 1 {
                stbtt__h_prefilter(spc.pixels + r.x + r.y * spc.stride_in_bytes,
                                   r.w, r.h, spc.stride_in_bytes,
                                   spc.h_oversample);
            }

            if spc.v_oversample > 1 {
                stbtt__v_prefilter(spc.pixels + r.x + r.y * spc.stride_in_bytes,
                                   r.w, r.h, spc.stride_in_bytes,
                                   spc.v_oversample);
            }

            bc.x0       =   r.x;
            bc.y0       =   r.y;
            bc.x1       =  (r.x + r.w);
            bc.y1       =  (r.y + r.h);
            bc.xadvance =                scale * advance;
            bc.xoff     =         x0 * recip_h + sub_x;
            bc.yoff     =         y0 * recip_v + sub_y;
            bc.xoff2    =                (x0 + r.w) * recip_h + sub_x;
            bc.yoff2    =                (y0 + r.h) * recip_v + sub_y;

            if glyph == 0 {
               missing_glyph = j;}
         } else if (spc.skip_missing) {
            return_value = 0;
         } else if (r.was_packed && r.w == 0 && r.h == 0 && missing_glyph >= 0) {
            ranges[i].chardata_for_range[j] = ranges[i].chardata_for_range[missing_glyph];
         } else {
            return_value = 0; // if any fail, report failure
         }

         k += 1;
      }
   }

   // restore original values
   spc.h_oversample = old_h_over as c_uint;
   spc.v_oversample = old_v_over as c_uint;

   return return_value;
}

pub unsafe fn stbtt_PackFontRangesPackRects(spc: *mut stbtt_pack_context, rects: *mut stbrp_rect, num_rects: c_int)
{
   stbrp_pack_rects(spc.pack_info, rects, num_rects);
}

pub unsafe fn stbtt_PackFontRanges(spc: *mut stbtt_pack_context, fontdata: *const c_uchar, font_index: c_int, ranges: *mut stbtt_pack_range, num_ranges: c_int) -> c_int
{
   // stbtt_fontinfo info;
   let mut info = stbtt_fontinfo::default();
    // i: c_int, j, n, return_value; // [DEAR IMGUI] removed = 1;
   let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut n: c_int = 0;
    let mut return_value: c_int = 0;

    //stbrp_context *context = (stbrp_context *) spc->pack_info;
   // stbrp_rect    *rects;
    let mut rects: *mut stbrp_rect = None;

   // flag all characters as NOT packed
   // for (i=0; i < num_ranges; ++i)
   for i in 0 .. num_ranges
    {
       // for (j=0; j < ranges[i].num_chars; + + j)
      for j in 0 .. ranges[i].num_chars
       {
           ranges[i].chardata_for_range[j].x0 = 0;
           ranges[i].chardata_for_range[j].y0 = 0;
           ranges[i].chardata_for_range[j].x1 = 0;
           ranges[i].chardata_for_range[j].y1 = 0;
       }
   }

   n = 0;
   // for (i=0; i < num_ranges; ++i)
   for i in 0 .. num_ranges
    {
       n += ranges[i].num_chars;
   }

   rects = STBTT_malloc(sizeof(*rects) * n, spc.user_allocator_context);
   if rects == None { return  0; }

   info.userdata = spc.user_allocator_context;
   stbtt_InitFont(&mut info, fontdata, stbtt_GetFontOffsetForIndex(fontdata,font_index));

   n = stbtt_PackFontRangesGatherRects(spc, &info, ranges, num_ranges, rects);

   stbtt_PackFontRangesPackRects(spc, rects, n);

   return_value = stbtt_PackFontRangesRenderIntoRects(spc, &info, ranges, num_ranges, rects);

   STBTT_free(rects, spc.user_allocator_context);
   return return_value;
}

pub unsafe fn stbtt_PackFontRange(spc: *mut stbtt_pack_context, fontdata: *const c_uchar, font_index: c_int,font_size: c_float,
            first_unicode_codepoint_in_range: c_int, num_chars_in_range: c_int, stbtt_packedchardata_for_range: *mut c_char) -> c_int
{
   // stbtt_pack_range range;
   let mut range = stbtt_pack_range::default();
    range.first_unicode_codepoint_in_range = first_unicode_codepoint_in_range;
   range.array_of_unicode_codepoints= None;
   range.num_chars                   = num_chars_in_range;
   range.chardata_for_range          = chardata_for_range;
   range.font_size                   = font_size;
   return stbtt_PackFontRanges(spc, fontdata, font_index, &mut range, 1);
}

pub unsafe fn stbtt_GetScaledFontVMetrics(fontdata: *const c_uchar, index: c_int,size: c_float, ascent: &mut c_float, descent: &mut c_float, lineGap: &mut c_float)
{
   // i_ascent: c_int, i_descent, i_lineGap;
   let mut i_ascent: c_int = 0;
    let mut i_descent: c_int = 0;
    let mut i_lineGap: c_int = 0;
    let mut scale: c_float = 0.0;
   // stbtt_fontinfo info;
   let mut info = stbtt_fontinfo::default();
    stbtt_InitFont(&mut info, fontdata, stbtt_GetFontOffsetForIndex(fontdata, index));
   scale = if size > 0.0 { stbtt_ScaleForPixelHeight(&info, size) }else { stbtt_ScaleForMappingEmToPixels(&mut info, -size)};
   stbtt_GetFontVMetrics(&info, &mut i_ascent, &mut i_descent, &mut i_lineGap);
   *ascent  =  i_ascent  * scale;
   *descent =  i_descent * scale;
   *lineGap =  i_lineGap * scale;
}

pub unsafe fn stbtt_GetPackedQuad(stbtt_packedchardata: *mut c_char, pw: c_int, ph: c_int, char_index: c_int, xpos: &mut c_float, ypos: &mut c_float, q: *mut stbtt_aligned_quad, align_to_integer: c_int)
{
   let ipw: c_float =  1.0 / pw;
    let mut iph: c_float = 1.0 / ph;
   let stbtt_packedb: *mut c_char = chardata + char_index;

   if align_to_integer {
      let x: c_float =   STBTT_ifloor((*xpos + b.xoff) + 0.5);
      let y: c_float =   STBTT_ifloor((*ypos + b.yoff) + 0.5);
      q.x0 = x;
      q.y0 = y;
      q.x1 = x + b.xoff2 - b.xoff;
      q.y1 = y + b.yoff2 - b.yoff;
   } else {
      q.x0 = *xpos + b.xoff;
      q.y0 = *ypos + b.yoff;
      q.x1 = *xpos + b.xoff2;
      q.y1 = *ypos + b.yoff2;
   }

   q.s0 = b.x0 * ipw;
   q.t0 = b.y0 * iph;
   q.s1 = b.x1 * ipw;
   q.t1 = b.y1 * iph;

   *xpos += b.xadvance;
}

//////////////////////////////////////////////////////////////////////////////
//
// sdf computation
//

// #define STBTT_min(a,b)  ((a) < (b) ? (a) : (b))
// #define STBTT_max(a,b)  ((a) < (b) ? (b) : (a))

pub fn stbtt__ray_intersect_bezier(orig: [c_float;2],
                                   ray: [c_float;2],
                                   q0: [c_float;2],
                                   q1: [c_float;2],
                                   q2: [c_float;2],
                                   mut hits: [[c_float;2];2]) -> c_int
{
   let q0perp: c_float =  q0[1]*ray[0] - q0[0]*ray[1];
   let q1perp: c_float =  q1[1]*ray[0] - q1[0]*ray[1];
   let q2perp: c_float =  q2[1]*ray[0] - q2[0]*ray[1];
   let roperp: c_float =  orig[1]*ray[0] - orig[0]*ray[1];

   let a: c_float =  q0perp - 2*q1perp + q2perp;
   let b: c_float =  q1perp - q0perp;
   let c: c_float =  q0perp - roperp;

   let mut s0: c_float =  0.0;
    let mut s1: c_float = 0.0;
   let mut num_s: c_int = 0;

   if (a != 0.0) {
      let discr: c_float =  b*b - a*c;
      if (discr > 0.0) {
         let rcpna: c_float =  -1 / a;
         let d: c_float =   STBTT_sqrt(discr);
         s0 = (b+d) * rcpna;
         s1 = (b-d) * rcpna;
         if s0 >= 0.0 && s0 <= 1.0 {
            num_s = 1;}
         if (d > 0.0 && s1 >= 0.0 && s1 <= 1.0) {
            if num_s == 0 {  s0 = s1;}
            num_s += 1;
         }
      }
   } else {
      // 2*b*s + c = 0
      // s = -c / (2*b)
      s0 = c / (-2 * b);
      if s0 >= 0.0 && s0 <= 1.0 {
         num_s = 1;}
   }

   if num_s == 0 { return  0; }
   else {
      let rcp_len2: c_float =  1 / (ray[0]*ray[0] + ray[1]*ray[1]);
      let rayn_x: c_float =  ray[0] * rcp_len2;
       let rayn_y = ray[1] * rcp_len2;

      let q0d: c_float =    q0[0]*rayn_x +   q0[1]*rayn_y;
      let q1d: c_float =    q1[0]*rayn_x +   q1[1]*rayn_y;
      let q2d: c_float =    q2[0]*rayn_x +   q2[1]*rayn_y;
      let rod: c_float =  orig[0]*rayn_x + orig[1]*rayn_y;

      let q10d: c_float =  q1d - q0d;
      let q20d: c_float =  q2d - q0d;
      let q0rd: c_float =  q0d - rod;

      hits[0][0] = q0rd + s0*(2.0 - 2.0 * s0)*q10d + s0*s0*q20d;
      hits[0][1] = a*s0+b;

      if (num_s > 1) {
         hits[1][0] = q0rd + s1*(2.0 - 2.0 * s1)*q10d + s1*s1*q20d;
         hits[1][1] = a*s1+b;
         return 2;
      } else {
         return 1;
      }
   }
}

pub fn equal(a: &mut c_float, b: &mut c_float) -> bool
{
   return (a[0] == b[0] && a[1] == b[1]);
}

pub fn stbtt__compute_crossings_x(x: c_float,mut y: c_float, nverts: c_int, verts: *mut stbtt_vertex) -> c_int
{
   let mut i: c_int = 0;
    let mut orig: [c_float;2] = [0.0;2];
    let mut ray: [c_float;2] =  [1.0, 0.0 ];
   let mut y_frac: c_float = 0.0;
   let mut winding: c_int = 0;

   // make sure y never passes through a vertex of the shape
   y_frac =  STBTT_fmod(y, 1.0);
   if (y_frac < 0.01) {
       y += 0.01;
   }
   else if (y_frac > 0.99) {
       y -= 0.01;
   }

   orig[0] = x;
   orig[1] = y;

   // test a ray from (-infinity,y) to (x,y)
   // for (i=0; i < nverts; ++i)
   for i in 0 .. nverts
    {
      if (verts[i].vertex_type == STBTT_vline) {
         let x0: c_int =  verts[i-1].x;
          let y0 =  verts[i-1].y;
         let x1: c_int =  verts[i  ].x;
          let y1 =  verts[i  ].y;
         if (y > STBTT_min(y0,y1) && y < STBTT_max(y0,y1) && x > STBTT_min(x0,x1)) {
            let x_inter: c_float =  (y - y0) / (y1 - y0) * (x1-x0) + x0;
            if (x_inter < x) {
                winding += if y0 < y1 { 1 } else { -1 };
            }
         }
      }
      if (verts[i].vertex_type == STBTT_vcurve) {
         let mut x0: c_int =  verts[i-1].x;
          let y0 =  verts[i-1].y ;
         let mut x1: c_int =  verts[i  ].cx;
          let y1 =  verts[i  ].cy;
         let x2: c_int =  verts[i  ].x;
          let y2 =  verts[i  ].y ;
         let ax: c_int = STBTT_min(x0,STBTT_min(x1,x2));
          let ay = STBTT_min(y0,STBTT_min(y1,y2));
         let by: c_int = STBTT_max(y0,STBTT_max(y1,y2));
         if (y > ay && y < by as c_float && x > ax as c_float) {
             let mut q0: [c_float;2] = [0.0;2];
             let mut q1: [c_float;2] = [0.0;2];
             let mut q2: [c_float;2] = [0.0;2];
             let mut hits: [[c_float;2];2] = [[0.0;2];2];
            q0[0] = x0 as c_float;
            q0[1] = y0;
            q1[0] = x1 as c_float;
            q1[1] = y1;
            q2[0] = x2 as c_float;
            q2[1] = y2;
            if (equal(q0.as_mut_ptr(),q1.as_mut_ptr()) || equal(q1.as_mut_ptr(),q2.as_mut_ptr())) {
               x0 = verts[i-1].x;
               y0 = verts[i-1].y;
               x1 = verts[i  ].x;
               y1 = verts[i  ].y;
               if (y > STBTT_min(y0,y1) && y < STBTT_max(y0,y1) && x > STBTT_min(x0,x1)) {
                  let x_inter: c_float =  (y - y0) / (y1 - y0) * (x1-x0) + x0;
                  if (x_inter < x) {
                      winding += if y0 < y1 { 1 } else { -1 };
                  }
               }
            } else {
               let num_hits: c_int = stbtt__ray_intersect_bezier(orig, ray, q0, q1, q2, hits);
               if num_hits >= 1 {
                   if hits[0][0] < 0.0 {

                       winding += (if hits[0][1] < 0.0 { -1 }else { 1 });
                   }
               }
               if (num_hits >= 2) {
                   if (hits[1][0] < 0.0) {
                       winding += (if hits[1][1] < 0.0 { -1 }else { 1 });
                   }
               }
            }
         }
      }
   }
   return winding;
}

pub unsafe fn staticstbtt__cuberoot(x: c_float) -> f32 {
    if (x < 0.0) {
        return -STBTT_pow(-x, 1.0 / 3.0);
    } else {
        return STBTT_pow(x, 1.0 / 3.0);
    }
}

// x^3 + a*x^2 + b*x + c = 0
pub fn stbtt__solve_cubic(a: c_float,b: c_float,c: c_float, r: &mut c_float) -> c_int
{
   let s: c_float =  -a / 3;
   let p: c_float =  b - a*a / 3;
   let q: c_float =  a * (2*a*a - 9*b) / 27 + c;
   let p3: c_float =  p*p*p;
   let d: c_float =  q*q + 4*p3 / 27;
   if d >= 0.0 {
      let z: c_float =   STBTT_sqrt(d);
      let mut u: c_float =  (-q + z) / 2;
      let mut v: c_float =  (-q - z) / 2;
      u = stbtt__cuberoot(u);
      v = stbtt__cuberoot(v);
      r[0] = s + u + v;
      return 1;
   } else {
      let u: c_float =   STBTT_sqrt(-p/3);
      let v: c_float =   STBTT_acos(-STBTT_sqrt(-27/p3) * q / 2) / 3; // p3 must be negative, since d is negative
      let m: c_float =   STBTT_cos(v);
      let n: c_float =   STBTT_cos(v-3.141592/2)*1.732050808;
      r[0] = s + u * 2 * m;
      r[1] = s - u * (m + n);
      r[2] = s - u * (m - n);

      //STBTT_assert( STBTT_fabs(((r[0]+a)*r[0]+b)*r[0]+c) < 0.05f32);  // these asserts may not be safe at all scales, though they're in bezier t parameter units so maybe?
      //STBTT_assert( STBTT_fabs(((r[1]+a)*r[1]+b)*r[1]+c) < 0.05f32);
      //STBTT_assert( STBTT_fabs(((r[2]+a)*r[2]+b)*r[2]+c) < 0.05f32);
      return 3;
   }
}

pub unsafe fn stbtt_GetGlyphSDF(info: *const stbtt_fontinfo,scale: c_float, glyph: c_int, padding: c_int, onedge_value: c_uchar, pixel_dist_scale: c_float, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int) -> *mut c_uchar
{
   let mut scale_x: c_float = 0.0;
    let mut scale: c_float = 0.0;
    let mut scale_y: c_float = 0.0;

   // ix0: c_int,iy0,ix1,iy1;
   let mut ix0: c_int = 0;
    let mut iy0: c_int = 0;
    let mut ix1: c_int = 0;
    let mut iy1: c_int = 0;
    // w: c_int,h;
    let mut w: c_int = 0;
    let mut h: c_int = 0;
   let mut c_udata: *mut c_char = None;

   if scale == 0.0 {  return None; }

   stbtt_GetGlyphBitmapBoxSubpixel(info, glyph, scale, scale, 0.0,0.0, &mut ix0,&mut iy0,&mut ix1,&mut iy1);

   // if empty, return NULL
   if ix0 == ix1 || iy0 == iy1{
      return None;}

   ix0 -= padding;
   iy0 -= padding;
   ix1 += padding;
   iy1 += padding;

   w = (ix1 - ix0);
   h = (iy1 - iy0);

   if (width ) { *width = w; }
   if (height) { *height = h; }
   if (xoff  ) { *xoff = ix0; }
   if (yoff  ) { *yoff = iy0; }

   // invert for y-downwards bitmaps
   scale_y = -scale_y;

   {
      // x: c_int,y,i,j;
      let mut x: c_int = 0;
       let mut y: c_int = 0;
       let mut i: c_int = 0;
       let mut j: c_int = 0;
       let mut precompute: &mut c_float = None;
      let mut verts: *mut stbtt_vertex = None;
      let num_verts: c_int = stbtt_GetGlyphShape(info, glyph, &mut verts);
      data = STBTT_malloc(w * h, info.userdata);
      precompute =  STBTT_malloc(num_verts * sizeof, info.userdata);

      // for (i=0,j=num_verts-1; i < num_verts; j=i++)
      // for i in 0 .. num_verts - 1
      i = 0;
       j = num_verts - 1;
       while i < num_verts
       {
         if (verts[i].vertex_type == STBTT_vline) {
            let x0: c_float =  verts[i].x*scale_x;
             let y0 = verts[i].y*scale_y;
            let x1: c_float =  verts[j].x*scale_x;
             let y1 = verts[j].y*scale_y;
            let dist: c_float =   STBTT_sqrt((x1-x0)*(x1-x0) + (y1-y0)*(y1-y0));
            precompute[i] = if dist == 0.0 { 0.0} else { 1.0 / dist};
         } else if (verts[i].vertex_type == STBTT_vcurve) {
            let x2: c_float =  verts[j].x *scale_x;
             let y2 = verts[j].y *scale_y;
            let x1: c_float =  verts[i].cx*scale_x;
             let y1 = verts[i].cy*scale_y;
            let x0: c_float =  verts[i].x *scale_x;
             let y0 = verts[i].y *scale_y;
            let bx: c_float =  x0 - 2*x1 + x2;
             let by = y0 - 2*y1 + y2;
            let len2: c_float =  bx*bx + by*by;
            if (len2 != 0.0) {
                precompute[i] = 1.0 / (bx * bx + by * by);
            }
            else {
                precompute[i] = 0.0;
            }
         } else {
             precompute[i] = 0.0;
         }

           j = i;
           i += 1;
      }

      // for (y=iy0; y < iy1; ++y)
      for y in iy0 .. iy1
       {
         // for (x=ix0; x < ix1; ++x)
         for x in ix0 .. ix1
           {
            let mut val: c_float = 0.0;
            let mut min_dist: c_float =  999999.0;
            let sx: c_float =   x + 0.5;
            let sy: c_float =   y + 0.5;
            let x_gspace: c_float =  (sx / scale_x);
            let y_gspace: c_float =  (sy / scale_y);

            let winding: c_int = stbtt__compute_crossings_x(x_gspace, y_gspace, num_verts, verts); // @OPTIMIZE: this could just be a rasterization, but needs to be line vs. non-tesselated curves so a new path

            // for (i=0; i < num_verts; ++i)
            for i in 0 .. num_verts
               {
               let x0: c_float =  verts[i].x*scale_x;
                   let y0 = verts[i].y*scale_y;

               if (verts[i].vertex_type == STBTT_vline && precompute[i] != 0.0) {
                  let x1: c_float =  verts[i-1].x*scale_x;
                   let y1 = verts[i-1].y*scale_y;dist: c_float;
                   let dist2 = (x0-sx)*(x0-sx) + (y0-sy)*(y0-sy);
                  if (dist2 < min_dist*min_dist) {
                      min_dist = STBTT_sqrt(dist2);
                  }

                  // coarse culling against bbox
                  //if (sx > STBTT_min(x0,x1)-min_dist && sx < STBTT_max(x0,x1)+min_dist &&
                  //    sy > STBTT_min(y0,y1)-min_dist && sy < STBTT_max(y0,y1)+min_dist)
                  dist =  STBTT_fabs((x1-x0)*(y0-sy) - (y1-y0)*(x0-sx)) * precompute[i];
                  STBTT_assert(i != 0);
                  if (dist < min_dist) {
                     // check position along line
                     // x' = x0 + t*(x1-x0), y' = y0 + t*(y1-y0)
                     // minimize (x'-sx)*(x'-sx)+(y'-sy)*(y'-sy)
                     let dx: c_float =  x1-x0;
                      let dy = y1-y0;
                     let px: c_float =  x0-sx;
                      let py = y0-sy;
                     // minimize (px+t*dx)^2 + (py+t*dy)^2 = px*px + 2*px*dx*t + t^2*dx*dx + py*py + 2*py*dy*t + t^2*dy*dy
                     // derivative: 2*px*dx + 2*py*dy + (2*dx*dx+2*dy*dy)*t, set to 0 and solve
                     let t: c_float =  -(px*dx + py*dy) / (dx*dx + dy*dy);
                     if t >= 0.0 && t <= 1.0 {
                        min_dist = dist;}
                  }
               } else if (verts[i].vertex_type == STBTT_vcurve) {
                  let x2: c_float =  verts[i-1].x *scale_x;
                   let y2 = verts[i-1].y *scale_y;
                  let x1: c_float =  verts[i  ].cx*scale_x;
                   let y1 = verts[i  ].cy*scale_y;
                  let box_x0: c_float =  STBTT_min(STBTT_min(x0,x1),x2);
                  let box_y0: c_float =  STBTT_min(STBTT_min(y0,y1),y2);
                  let box_x1: c_float =  STBTT_max(STBTT_max(x0,x1),x2);
                  let box_y1: c_float =  STBTT_max(STBTT_max(y0,y1),y2);
                  // coarse culling against bbox to avoid computing cubic unnecessarily
                  if sx > box_x0-min_dist && sx < box_x1+min_dist && sy > box_y0-min_dist && sy < box_y1+min_dist {
                     let mut num: c_int = 0;
                     let ax: c_float =  x1-x0;
                      let ay = y1-y0;
                     let bx: c_float =  x0 - 2*x1 + x2;
                      let by = y0 - 2*y1 + y2;
                     let mx: c_float =  x0 - sx;
                      let my = y0 - sy;res: c_float[3] = [0.0;3];
                      let mut px: c_float = 0.0;
                      // ,py,t,it,dist2;
                     let mut py: c_float = 0.0;
                      let mut t: c_float = 0.0;
                      let mut it: c_float = 0.0;
                      let mut dist2: c_float = 0.0;
                      let a_inv: c_float =  precompute[i];
                     if a_inv == 0.0 { // if a_inv is 0, it's 2nd degree so use quadratic formula
                        let a: c_float =  3*(ax*bx + ay*by);
                        let b: c_float =  2*(ax*ax + ay*ay) + (mx*bx+my*by);
                        let c: c_float =  mx*ax+my*ay;
                        if a == 0.0 { // if a is 0, it's linear
                           if b != 0.0 {
                              res[num] = -c/b;
                               num += 1;
                           }
                        } else {
                           let discriminant: c_float =  b*b - 4*a*c;
                           if discriminant < 0.0 {
                              num = 0;}
                           else {
                              let root: c_float =   STBTT_sqrt(discriminant);
                              res[0] = (-b - root)/(2*a);
                              res[1] = (-b + root)/(2*a);
                              num = 2; // don't bother distinguishing 1-solution case, as code below will still work
                           }
                        }
                     } else {
                        let b: c_float =  3*(ax*bx + ay*by) * a_inv; // could precompute this as it doesn't depend on sample point
                        let c: c_float =  (2*(ax*ax + ay*ay) + (mx*bx+my*by)) * a_inv;
                        let d: c_float =  (mx*ax+my*ay) * a_inv;
                        num = stbtt__solve_cubic(b, c, d, res);
                     }
                     dist2 = (x0-sx)*(x0-sx) + (y0-sy)*(y0-sy);
                     if (dist2 < min_dist*min_dist) {
                         min_dist = STBTT_sqrt(dist2);
                     }

                     if (num >= 1 && res[0] >= 0.0 && res[0] <= 1.0) {
                        t = res[0];
                         it = 1.0 - t;
                        px = it*it*x0 + 2*t*it*x1 + t*t*x2;
                        py = it*it*y0 + 2*t*it*y1 + t*t*y2;
                        dist2 = (px-sx)*(px-sx) + (py-sy)*(py-sy);
                        if (dist2 < min_dist * min_dist) {
                            min_dist = STBTT_sqrt(dist2);
                        }
                     }
                     if (num >= 2 && res[1] >= 0.0 && res[1] <= 1.0) {
                        t = res[1];
                         it = 1.0 - t;
                        px = it*it*x0 + 2*t*it*x1 + t*t*x2;
                        py = it*it*y0 + 2*t*it*y1 + t*t*y2;
                        dist2 = (px-sx)*(px-sx) + (py-sy)*(py-sy);
                        if (dist2 < min_dist * min_dist) {
                            min_dist = STBTT_sqrt(dist2);
                        }
                     }
                     if (num >= 3 && res[2] >= 0.0 && res[2] <= 1.0) {
                        t = res[2];
                         it = 1.0 - t;
                        px = it*it*x0 + 2*t*it*x1 + t*t*x2;
                        py = it*it*y0 + 2*t*it*y1 + t*t*y2;
                        dist2 = (px-sx)*(px-sx) + (py-sy)*(py-sy);
                        if (dist2 < min_dist * min_dist) {
                            min_dist = STBTT_sqrt(dist2);
                        }
                     }
                  }
               }
            }
            if (winding == 0) {
                min_dist = -min_dist;
            }// if outside the shape, value is negative
            val = onedge_value + pixel_dist_scale * min_dist;
            if val < 0.0 {
               val = 0.0;}
            else if val > 255.0 {
               val = 255.0;}
            data[(y-iy0)*w+(x-ix0)] =  val;
         }
      }
      STBTT_free(precompute, info.userdata);
      STBTT_free(verts, info.userdata);
   }
   return data;
}

pub unsafe fn stbtt_GetCodepointSDF(info: *const stbtt_fontinfo,scale: c_float, codepoint: c_int, padding: c_int, onedge_value: c_uchar,pixel_dist_scale: c_float, width: *mut c_int, height: *mut c_int, xoff: *mut c_int, yoff: *mut c_int) -> *mut c_uchar
{
   return stbtt_GetGlyphSDF(info, scale, stbtt_FindGlyphIndex(info, codepoint), padding, onedge_value, pixel_dist_scale, width, height, xoff, yoff);
}

pub unsafe fn stbtt_FreeSDF(c_ubitmap: *mut c_char, userdata: *mut c_void)
{
   STBTT_free(bitmap, userdata);
}

//////////////////////////////////////////////////////////////////////////////
//
// font name matching -- recommended not to use this
//

// check if a utf8 string contains a prefix which is the utf16 string; if so return length of matching utf8 string
pub fn stbtt__CompareUTF8toUTF16_bigendian_prefix(mut s1: *mut u8, mut len1: i32, mut s2: *mut u8, mut len2: i32) -> i32
{
   let mut i: i32=0;

   // convert utf16 to utf8 and compare the results while converting
   while len2 {
      ch: u16 = s2[0]*256 + s2[1];
      if (ch < 0x80) {
         if (i >= len1) { return -1; }
         if (s1[i] != ch) { return -1; }
          i += 1;
      } else if (ch < 0x800) {
         if (i1 >= len1) { return -1; }
         if (s1[i] != 0xc0 + (ch >> 6)) { return -1; }
          i += 1;
         if (s1[i] != 0x80 + (ch & 0x30)) { return -1; }
          i += 1;
      } else if (ch >= 0xd800 && ch < 0xdc00) {
         c: u32;
         ch2: u16 = s2[2]*256 + s2[3];
         if (i3 >= len1) { return -1; }
         c = ((ch - 0xd800) << 10) + (ch2 - 0xdc00) + 0x10000;
         if (s1[i] != 0xf0 + (c >> 18)) { return -1; }
          i += 1;
         if (s1[i] != 0x80 + ((c >> 12) & 0x30)) { return -1; }
          i += 1;
         if (s1[i] != 0x80 + ((c >>  6) & 0x30)) { return -1; }
          i += 1;
         if (s1[i] != 0x80 + ((c      ) & 0x30)) { return -1; }
          i += 1;
         s2 += 2; // plus another 2 below
         len2 -= 2;
      } else if (ch >= 0xdc00 && ch < 0xe000) {
         return -1;
      } else {
         if (i2 >= len1) { return -1; }
         if (s1[i] != 0xe0 + (ch >> 12)) { return -1; }
          i += 1;
         if (s1[i] != 0x80 + ((ch >> 6) & 0x30f32)) { return -1; }
          i += 1;
         if (s1[i] != 0x80 + ((ch     ) & 0x30f32)) { return -1; }
          i += 1;
      }
      s2 += 2;
      len2 -= 2;
   }
   return i;
}

pub fn stbtt_CompareUTF8toUTF16_bigendian_internal(s1: *mut c_uchar, len1: c_int, s2: *mut c_uchar, len2: c_int) -> bool
{
   return len1 == stbtt__CompareUTF8toUTF16_bigendian_prefix( s1, len1,  s2, len2);
}

// returns results in whatever encoding you request... but note that 2-byte encodings
// will be BIG-ENDIAN... use stbtt_CompareUTF8toUTF16_bigendian() to compare
pub unsafe fn stbtt_GetFontNameString(font: *const stbtt_fontinfo, length: *mut c_int, platformID: c_int, encodingID: c_int, languageID: c_int, nameID: c_int) -> *mut c_char
{
   // i: i32,count,stringOffset;
let mut i: i32 = 0;
let mut count: i32 = 0;
let mut stringOffset: i32 = 0;
let mut fc: *mut u8 = font.data;
   let mut offset: u32 = font.fontstart;
   let mut nm: u32 = stbtt__find_table(fc, offset, str_to_const_c_char_ptr("name"));
   if (!nm) {return None; }

   count = ttUSHORT(fc + nm2) as i32;
   stringOffset = (nm + ttUSHORT(fc + nm4)) as i32;
   // for (i=0; i < count; ++i)
   for i in 0 .. count
    {
      loc: u32 = nm + 6 + 12 * i;
      if platformID == ttUSHORT(fc + loc0) as c_int && encodingID == ttUSHORT(fc + loc2) as c_int
          && languageID == ttUSHORT(fc + loc4) as c_int && nameID == ttUSHORT(fc + loc6) as c_int {
         *length = ttUSHORT(fc + loc8) as c_int;
         return fc+stringOffset+ttUSHORT(fc+loc10);
      }
   }
   return None;
}

pub fn stbtt__matchpair(fc: *mut u8, nm: u32, name: *mut u8, nlen: i32, target_id: i32, next_id: i32) -> c_int
{
   let mut i: i32;
   let mut count: i32 = ttUSHORT(fc + nm2) as i32;
   let mut stringOffset: i32 = (nm + ttUSHORT(fc + nm4)) as i32;

   // for (i=0; i < count; ++i)
   for i in 0 .. count
    {
      loc: u32 = nm + 6 + 12 * i;
      let mut id: i32 = ttUSHORT(fc + loc6) as i32;
      if id == target_id {
         // find the encoding
         let mut platform = ttUSHORT(fc+loc0);
          let mut encoding = ttUSHORT(fc+loc2);
          let mut language = ttUSHORT(fc+loc4);

         // is this a Unicode encoding?
         if platform == 0 || (platform == 3 && encoding == 1) || (platform == 3 && encoding == 10) {
            let mut slen = ttUSHORT(fc+loc8);
            let mut off = ttUSHORT(fc+loc10);

            // check if there's a prefix match
            matchlen: i32 = stbtt__CompareUTF8toUTF16_bigendian_prefix(name as *mut u8, nlen, fc+stringOffset+off, slen as i32);
            if (matchlen >= 0) {
               // check for target_id+1 immediately following, with same encoding & language
               if i1 < count && ttUSHORT(fc+loc126) == next_id as u16 && ttUSHORT(fc+loc12) == platform && ttUSHORT(fc+loc122) == encoding && ttUSHORT(fc+loc124) == language {
                  slen = ttUSHORT(fc+loc128);
                  off = ttUSHORT(fc+loc1210);
                  if (slen == 0) {
                     if matchlen == nlen { return  1; }
                  } else if (matchlen < nlen && name[matchlen] == ' ') {
                     matchlen += 1;
                     if stbtt_CompareUTF8toUTF16_bigendian_internal((name+matchlen), nlen-matchlen, (fc+stringOffset+off), slen as c_int) { return  1; }
                  }
               } else {
                  // if nothing immediately following
                  if matchlen == nlen { return  1; }
               }
            }
         }

         // @TODO handle other encodings
      }
   }
   return 0;
}

pub unsafe fn stbtt__matches(fc: *mut u8, offset: u32, name: *mut u8, flags: i32) -> c_int
{
   let mut nlen: i32 =  STBTT_strlen( name);
   let mut nm: u32 = 0;
    let mut hd: u32 = 0;
   if !stbtt__isfont(fc+offset) { return  0; }

   // check italics/bold/underline flags in macStyle...
   if (flags) {
      hd = stbtt__find_table(fc, offset, str_to_const_c_char_ptr("head"));
      if (ttUSHORT(fc+hd44) & 7) != flag_set(flags, 7) as u16 { return  0; }
   }

   nm = stbtt__find_table(fc, offset, str_to_const_c_char_ptr("name"));
   if !nm { return  0; }

   if (flags) {
      // if we checked the macStyle flags, then just check the family and ignore the subfamily
      if stbtt__matchpair(fc, nm, name, nlen, 16, -1) { return  1; }
      if stbtt__matchpair(fc, nm, name, nlen,  1, -1) { return  1; }
      if stbtt__matchpair(fc, nm, name, nlen,  3, -1) { return  1; }
   } else {
      if stbtt__matchpair(fc, nm, name, nlen, 16, 17) { return  1; }
      if stbtt__matchpair(fc, nm, name, nlen,  1,  2) { return  1; }
      if stbtt__matchpair(fc, nm, name, nlen,  3, -1) { return  1; }
   }

   return 0;
}

pub unsafe fn stbtt_FindMatchingFont_internal(c_ufont_collection: *mut c_char, name_utf8: *mut u8, flags: i32) -> c_int {
    let mut i: i32 = 0;
    // for (i=0;;++i)
    loop {
        off: i32 = stbtt_GetFontOffsetForIndex(font_collection, i);
        if off < 0 { return off; }
        if stbtt__matches( font_collection, off,  name_utf8, flags) { return off; }
        i += 1;
    }
}

// #if defined(__GNUC__) || defined(__clang__)
// #pragma GCC diagnostic push
// #pragma GCC diagnostic ignored "-Wcast-qual"
// #endif

pub unsafe fn stbtt_BakeFontBitmap(data: *const c_uchar, offset: c_int,pixel_height: c_float, c_upixels: *mut c_uchar, pw: c_int, ph: c_int,
                                first_char: c_int, num_chars: c_int, stbtt_bakedchardata: *mut c_uchar) -> c_int
{
   return stbtt_BakeFontBitmap_internal( data, offset, pixel_height, pixels, pw, ph, first_char, num_chars, stbtt_bakedchardata);
}

pub unsafe fn stbtt_GetFontOffsetForIndex(data: *const c_uchar, index: c_int) -> c_int
{
   return stbtt_GetFontOffsetForIndex_internal(data, index);
}

pub unsafe fn stbtt_GetNumberOfFonts(data: *mut c_char) -> c_int
{
   return stbtt_GetNumberOfFonts_internal(data);
}

pub unsafe fn stbtt_InitFont(info: *mut stbtt_fontinfo, data: *const c_uchar, offset: c_int) -> c_int
{
   return stbtt_InitFont_internal(info, data, offset as u32);
}

pub unsafe fn stbtt_FindMatchingFont(fontdata: *mut c_char, name: *mut u8, flags: c_int) -> c_int
{
   return stbtt_FindMatchingFont_internal(fontdata,  name, flags);
}

pub unsafe fn stbtt_CompareUTF8toUTF16_bigendian(s1: *mut c_uchar, len1: c_int, s2: *mut c_uchar, len2: c_int) -> bool
{
   return stbtt_CompareUTF8toUTF16_bigendian_internal( s1, len1,  s2, len2);
}

// #if defined(__GNUC__) || defined(__clang__)
// #pragma GCC diagnostic pop
// #endif

// #endif // STB_TRUETYPE_IMPLEMENTATION


// FULL VERSION HISTORY
//
//   1.25 (2021-07-11) many fixes
//   1.24 (2020-02-05) fix warning
//   1.23 (2020-02-02) query SVG data for glyphs; query whole kerning table (but only kern not GPOS)
//   1.22 (2019-08-11) minimize missing-glyph duplication; fix kerning if both 'GPOS' and 'kern' are defined
//   1.21 (2019-02-25) fix warning
//   1.20 (2019-02-07) PackFontRange skips missing codepoints; GetScaleFontVMetrics()
//   1.19 (2018-02-11) OpenType GPOS kerning (horizontal only), STBTT_fmod
//   1.18 (2018-01-29) add missing function
//   1.17 (2017-07-23) make more arguments const; doc fix
//   1.16 (2017-07-12) SDF support
//   1.15 (2017-03-03) make more arguments const
//   1.14 (2017-01-16) num-fonts-in-TTC function
//   1.13 (2017-01-02) support OpenType fonts, certain Apple fonts
//   1.12 (2016-10-25) suppress warnings about casting away const with -Wcast-qual
//   1.11 (2016-04-02) fix unused-variable warning
//   1.10 (2016-04-02) allow user-defined fabs() replacement
//                     fix memory leak if fontsize=0.0
//                     fix warning from duplicate typedef
//   1.09 (2016-01-16) warning fix; avoid crash on outofmem; use alloc userdata for PackFontRanges
//   1.08 (2015-09-13) document stbtt_Rasterize(); fixes for vertical & horizontal edges
//   1.07 (2015-08-01) allow PackFontRanges to accept arrays of sparse codepoints;
//                     allow PackFontRanges to pack and render in separate phases;
//                     fix stbtt_GetFontOFfsetForIndex (never worked for non-0 input?);
//                     fixed an assert() bug in the new rasterizer
//                     replace assert() with STBTT_assert() in new rasterizer
//   1.06 (2015-07-14) performance improvements (~35% faster on x86 and x64 on test machine)
//                     also more precise AA rasterizer, except if shapes overlap
//                     remove need for STBTT_sort
//   1.05 (2015-04-15) fix misplaced definitions for STBTT_STATIC
//   1.04 (2015-04-15) typo in example
//   1.03 (2015-04-12) STBTT_STATIC, fix memory leak in new packing, various fixes
//   1.02 (2014-12-10) fix various warnings & compile issues w/ stb_rect_pack, C++
//   1.01 (2014-12-08) fix subpixel position when oversampling to exactly match
//                        non-oversampled; STBTT_POINT_SIZE for packed case only
//   1.00 (2014-12-06) add new PackBegin etc. API, w/ support for oversampling
//   0.99 (2014-09-18) fix multiple bugs with subpixel rendering (ryg)
//   0.9  (2014-08-07) support certain mac/iOS fonts without an MS platformID
//   0.8b (2014-07-07) fix a warning
//   0.8  (2014-05-25) fix a few more warnings
//   0.7  (2013-09-25) bugfix: subpixel glyph bug fixed in 0.5 had come back
//   0.6c (2012-07-24) improve documentation
//   0.6b (2012-07-20) fix a few more warnings
//   0.6  (2012-07-17) fix warnings; added stbtt_ScaleForMappingEmToPixels,
//                        stbtt_GetFontBoundingBox, stbtt_IsGlyphEmpty
//   0.5  (2011-12-09) bugfixes:
//                        subpixel glyph renderer computed wrong bounding box
//                        first vertex of shape can be off-curve (FreeSans)
//   0.4b (2011-12-03) fixed an error in the font baking example
//   0.4  (2011-12-01) kerning, subpixel rendering (tor)
//                    bugfixes for:
//                        codepoint-to-glyph conversion using table fmt=12
//                        codepoint-to-glyph conversion using table fmt=4
//                        stbtt_GetBakedQuad with non-square texture (Zer)
//                    updated Hello World! sample to use kerning and subpixel
//                    fixed some warnings
//   0.3  (2009-06-24) cmap fmt=12, compound shapes (MM)
//                    userdata, malloc-from-userdata, non-zero fill (stb)
//   0.2  (2009-03-11) Fix unsigned/signed char warnings
//   0.1  (2009-03-09) First public release
//

/*
------------------------------------------------------------------------------
This software is available under 2 licenses -- choose whichever you prefer.
------------------------------------------------------------------------------
ALTERNATIVE A - MIT License
Copyright (c) 2017 Sean Barrett
Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
------------------------------------------------------------------------------
ALTERNATIVE B - Public Domain (www.unlicense.org)
This is free and unencumbered software released into the public domain.
Anyone is free to copy, modify, publish, use, compile, sell, or distribute this
software, either in source code form or as a compiled binary, for any purpose,
commercial or non-commercial, and by any means.
In jurisdictions that recognize copyright laws, the author or authors of this
software dedicate any and all copyright interest in the software to the public
domain. We make this dedication for the benefit of the public at large and to
the detriment of our heirs and successors. We intend this dedication to be an
overt act of relinquishment in perpetuity of all present and future rights to
this software under copyright law.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
------------------------------------------------------------------------------
*/
