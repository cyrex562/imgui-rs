// [DEAR IMGUI]
// This is a slightly modified version of stb_textedit.h 1.14.
// Those changes would need to be pushed into nothings/stb:
// - Fix in stb_textedit_discard_redo (see https://github.com/nothings/stb/issues/321)
// Grep for [DEAR IMGUI] to find the changes.

// stb_textedit.h - v1.14  - public domain - Sean Barrett
// Development of this library was sponsored by RAD Game Tools
//
// This C header file implements the guts of a multi-line text-editing
// widget; you implement display, word-wrapping, and low-level string
// insertion/deletion, and stb_textedit will map user inputs into
// insertions & deletions, plus updates to the cursor position,
// selection state, and undo state.
//
// It is intended for use in games and other systems that need to build
// their own custom widgets and which do not have heavy text-editing
// requirements (this library is not recommended for use for editing large
// texts, as its performance does not scale and it has limited undo).
//
// Non-trivial behaviors are modelled after windows text controls.
//
//
// LICENSE
//
// See end of file for license information.
//
//
// DEPENDENCIES
//
// Uses the C runtime function 'memmove', which you can override
// by defining STB_TEXTEDIT_memmove before the implementation.
// Uses no other functions. Performs no runtime allocations.
//
//
// VERSION HISTORY
//
//   1.14 (2021-07-11) page up/down, various fixes
//   1.13 (2019-02-07) fix bug in undo size management
//   1.12 (2018-01-29) user can change STB_TEXTEDIT_KEYTYPE, fix redo to avoid crash
//   1.11 (2017-03-03) fix HOME on last line, dragging off single-line textfield
//   1.10 (2016-10-25) supress warnings about casting away const with -Wcast-qual
//   1.9  (2016-08-27) customizable move-by-word
//   1.8  (2016-04-02) better keyboard handling when mouse button is down
//   1.7  (2015-09-13) change y range handling in case baseline is non-0
//   1.6  (2015-04-15) allow STB_TEXTEDIT_memmove
//   1.5  (2014-09-10) add support for secondary keys for OS x
//   1.4  (2014-08-17) fix signed/unsigned warnings
//   1.3  (2014-06-19) fix mouse clicking to round to nearest char boundary
//   1.2  (2014-05-27) fix some RAD types that had crept into the new code
//   1.1  (2013-12-15) move-by-word (requires STB_TEXTEDIT_IS_SPACE )
//   1.0  (2012-07-26) improve documentation, initial public release
//   0.3  (2012-02-24) bugfixes, single-line mode; insert mode
//   0.2  (2011-11-28) fixes to undo/redo
//   0.1  (2010-07-08) initial version
//
// ADDITIONAL CONTRIBUTORS
//
//   Ulf Winklemann: move-by-word in 1.1
//   Fabian Giesen: secondary key inputs in 1.5
//   Martins Mozeiko: STB_TEXTEDIT_memmove in 1.6
//   Louis Schnellbach: page up/down in 1.14
//
//   Bugfixes:
//      Scott Graham
//      Daniel Keller
//      Omar Cornut
//      Dan Thompson
//
// USAGE
//
// This file behaves differently depending on what symbols you define
// before including it.
//
//
// Header-file mode:
//
//   If you do not define STB_TEXTEDIT_IMPLEMENTATION before including this,
//   it will operate in "header file" mode. In this mode, it declares a
//   single public symbol, STB_TexteditState, which encapsulates the current
//   state of a text widget (except for the string, which you will store
//   separately).
//
//   To compile in this mode, you must define STB_TEXTEDIT_CHARTYPE to a
//   primitive type that defines a single character (e.g. char, wchar_t, etc).
//
//   To save space or increase undo-ability, you can optionally define the
//   following things that are used by the undo system:
//
//      STB_TEXTEDIT_POSITIONTYPE         small int type encoding a valid cursor position
//      STB_TEXTEDIT_UNDOSTATECOUNT       the number of undo states to allow
//      STB_TEXTEDIT_UNDOCHARCOUNT        the number of characters to store in the undo buffer
//
//   If you don't define these, they are set to permissive types and
//   moderate sizes. The undo system does no memory allocations, so
//   it grows STB_TexteditState by the worst-case storage which is (in bytes):
//
//        [4 + 3 * sizeof(STB_TEXTEDIT_POSITIONTYPE)] * STB_TEXTEDIT_UNDOSTATECOUNT
//      +          sizeof(STB_TEXTEDIT_CHARTYPE)      * STB_TEXTEDIT_UNDOCHARCOUNT
//
//
// Implementation mode:
//
//   If you define STB_TEXTEDIT_IMPLEMENTATION before including this, it
//   will compile the implementation of the text edit widget, depending
//   on a large number of symbols which must be defined before the include.
//
//   The implementation is defined only as static functions. You will then
//   need to provide your own APIs in the same file which will access the
//   static functions.
//
//   The basic concept is that you provide a "string" object which
//   behaves like an array of characters. stb_textedit uses indices to
//   refer to positions in the string, implicitly representing positions
//   in the displayed textedit. This is true for both plain text and
//   rich text; even with rich text stb_truetype interacts with your
//   code as if there was an array of all the displayed characters.
//
// Symbols that must be the same in header-file and implementation mode:
//
//     STB_TEXTEDIT_CHARTYPE             the character type
//     STB_TEXTEDIT_POSITIONTYPE         small type that is a valid cursor position
//     STB_TEXTEDIT_UNDOSTATECOUNT       the number of undo states to allow
//     STB_TEXTEDIT_UNDOCHARCOUNT        the number of characters to store in the undo buffer
//
// Symbols you must define for implementation mode:
//
//    STB_TEXTEDIT_STRING               the type of object representing a string being edited,
//                                      typically this is a wrapper object with other data you need
//
//    STB_TEXTEDIT_STRINGLEN(obj)       the length of the string (ideally O(1))
//    STB_TEXTEDIT_LAYOUTROW(&r,obj,n)  returns the results of laying out a line of characters
//                                        starting from character #n (see discussion below)
//    STB_TEXTEDIT_GETWIDTH(obj,n,i)    returns the pixel delta from the xpos of the i'th character
//                                        to the xpos of the i+1'th char for a line of characters
//                                        starting at character #n (i.e. accounts for kerning
//                                        with previous char)
//    STB_TEXTEDIT_KEYTOTEXT(k)         maps a keyboard input to an insertable character
//                                        (return type is int, -1 means not valid to insert)
//    STB_TEXTEDIT_GETCHAR(obj,i)       returns the i'th character of obj, 0-based
//    STB_TEXTEDIT_NEWLINE              the character returned by _GETCHAR() we recognize
//                                        as manually wordwrapping for end-of-line positioning
//
//    STB_TEXTEDIT_DELETECHARS(obj,i,n)      delete n characters starting at i
//    STB_TEXTEDIT_INSERTCHARS(obj,i,c*,n)   insert n characters at i (pointed to by STB_TEXTEDIT_CHARTYPE*)
//
//    STB_TEXTEDIT_K_SHIFT       a power of two that is or'd in to a keyboard input to represent the shift key
//
//    STB_TEXTEDIT_K_LEFT        keyboard input to move cursor left
//    STB_TEXTEDIT_K_RIGHT       keyboard input to move cursor right
//    STB_TEXTEDIT_K_UP          keyboard input to move cursor up
//    STB_TEXTEDIT_K_DOWN        keyboard input to move cursor down
//    STB_TEXTEDIT_K_PGUP        keyboard input to move cursor up a page
//    STB_TEXTEDIT_K_PGDOWN      keyboard input to move cursor down a page
//    STB_TEXTEDIT_K_LINESTART   keyboard input to move cursor to start of line  // e.g. HOME
//    STB_TEXTEDIT_K_LINEEND     keyboard input to move cursor to end of line    // e.g. END
//    STB_TEXTEDIT_K_TEXTSTART   keyboard input to move cursor to start of text  // e.g. ctrl-HOME
//    STB_TEXTEDIT_K_TEXTEND     keyboard input to move cursor to end of text    // e.g. ctrl-END
//    STB_TEXTEDIT_K_DELETE      keyboard input to delete selection or character under cursor
//    STB_TEXTEDIT_K_BACKSPACE   keyboard input to delete selection or character left of cursor
//    STB_TEXTEDIT_K_UNDO        keyboard input to perform undo
//    STB_TEXTEDIT_K_REDO        keyboard input to perform redo
//
// Optional:
//    STB_TEXTEDIT_K_INSERT              keyboard input to toggle insert mode
//    STB_TEXTEDIT_IS_SPACE(ch)          true if character is whitespace (e.g. 'isspace'),
//                                          required for default WORDLEFT/WORDRIGHT handlers
pub fn STB_TEXTEDIT_IS_SPACE(ch: ImWchar) -> bool {
    ch.is_ascii_whitespace()
}

//    STB_TEXTEDIT_MOVEWORDLEFT(obj,i)   custom handler for WORDLEFT, returns index to move cursor to
//    STB_TEXTEDIT_MOVEWORDRIGHT(obj,i)  custom handler for WORDRIGHT, returns index to move cursor to
//    STB_TEXTEDIT_K_WORDLEFT            keyboard input to move cursor left one word // e.g. ctrl-LEFT
//    STB_TEXTEDIT_K_WORDRIGHT           keyboard input to move cursor right one word // e.g. ctrl-RIGHT
//    STB_TEXTEDIT_K_LINESTART2          secondary keyboard input to move cursor to start of line
//    STB_TEXTEDIT_K_LINEEND2            secondary keyboard input to move cursor to end of line
//    STB_TEXTEDIT_K_TEXTSTART2          secondary keyboard input to move cursor to start of text
//    STB_TEXTEDIT_K_TEXTEND2            secondary keyboard input to move cursor to end of text
//
// Keyboard input must be encoded as a single integer value; e.g. a character code
// and some bitflags that represent shift states. to simplify the interface, SHIFT must
// be a bitflag, so we can test the shifted state of cursor movements to allow selection,
// i.e. (STB_TEXTEDIT_K_RIGHT|STB_TEXTEDIT_K_SHIFT) should be shifted right-arrow.
//
// You can encode other things, such as CONTROL or ALT, in additional bits, and
// then test for their presence in e.g. STB_TEXTEDIT_K_WORDLEFT. For example,
// my windows implementations add an additional CONTROL bit, and an additional KEYDOWN
// bit. Then all of the STB_TEXTEDIT_K_ values bitwise-or in the KEYDOWN bit,
// and I pass both WM_KEYDOWN and WM_CHAR events to the "key" function in the
// API below. The control keys will only match WM_KEYDOWN events because of the
// keydown bit I add, and STB_TEXTEDIT_KEYTOTEXT only tests for the KEYDOWN
// bit so it only decodes WM_CHAR events.
//
// STB_TEXTEDIT_LAYOUTROW returns information about the shape of one displayed
// row of characters assuming they start on the i'th character--the width and
// the height and the number of characters consumed. This allows this library
// to traverse the entire layout incrementally. You need to compute word-wrapping
// here.
//
// Each textfield keeps its own insert mode state, which is not how normal
// applications work. To keep an app-wide insert mode, update/copy the
// "insert_mode" field of STB_TexteditState before/after calling API functions.
//
// API
//
//    void stb_textedit_initialize_state(STB_TexteditState *state, int is_single_line)
//
//    void stb_textedit_click(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, float x, float y)
//    void stb_textedit_drag(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, float x, float y)
//    int  stb_textedit_cut(STB_TEXTEDIT_STRING *str, STB_TexteditState *state)
//    int  stb_textedit_paste(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, STB_TEXTEDIT_CHARTYPE *text, int len)
//    void stb_textedit_key(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, STB_TEXEDIT_KEYTYPE key)
//
//    Each of these functions potentially updates the string and updates the
//    state.
//
//      initialize_state:
//          set the textedit state to a known good default state when initially
//          constructing the textedit.
//
//      click:
//          call this with the mouse x,y on a mouse down; it will update the cursor
//          and reset the selection start/end to the cursor point. the x,y must
//          be relative to the text widget, with (0,0) being the top left.
//
//      drag:
//          call this with the mouse x,y on a mouse drag/up; it will update the
//          cursor and the selection end point
//
//      cut:
//          call this to delete the current selection; returns true if there was
//          one. you should FIRST copy the current selection to the system paste buffer.
//          (To copy, just copy the current selection out of the string yourself.)
//
//      paste:
//          call this to paste text at the current cursor point or over the current
//          selection if there is one.
//
//      key:
//          call this for keyboard inputs sent to the textfield. you can use it
//          for "key down" events or for "translated" key events. if you need to
//          do both (as in Win32), or distinguish Unicode characters from control
//          inputs, set a high bit to distinguish the two; then you can define the
//          various definitions like STB_TEXTEDIT_K_LEFT have the is-key-event bit
//          set, and make STB_TEXTEDIT_KEYTOCHAR check that the is-key-event bit is
//          clear. STB_TEXTEDIT_KEYTYPE defaults to int, but you can #define it to
//          anything other type you wante before including.
//
//
//   When rendering, you can read the cursor position and selection state from
//   the STB_TexteditState.
//
//
// Notes:
//
// This is designed to be usable in IMGUI, so it allows for the possibility of
// running in an IMGUI that has NOT cached the multi-line layout. For this
// reason, it provides an interface that is compatible with computing the
// layout incrementally--we try to make sure we make as few passes through
// as possible. (For example, to locate the mouse pointer in the text, we
// could define functions that return the x and Y positions of characters
// and binary search Y and then x, but if we're doing dynamic layout this
// will run the layout algorithm many times, so instead we manually search
// forward in one pass. Similar logic applies to e.g. up-arrow and
// down-arrow movement.)
//
// If it's run in a widget that *has* cached the layout, then this is less
// efficient, but it's not horrible on modern computers. But you wouldn't
// want to edit million-line files with it.


////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
////
////   Header-file mode
////
////

// #ifndef INCLUDE_STB_TEXTEDIT_H
// #define INCLUDE_STB_TEXTEDIT_H

////////////////////////////////////////////////////////////////////////
//
//     STB_TexteditState
//
// Definition of STB_TexteditState which you should store
// per-textfield; it includes cursor position, selection state,
// and undo state.
//


use std::ptr::null_mut;
use crate::imgui_globals::GImGui;
use crate::imgui_h::{ImGuiInputTextFlags, ImWchar};
use crate::imguI_string::{ImCharIsBlankW, is_separator, is_word_boundary_from_right};
use crate::imgui_text::{ImTextCountUtf8BytesFromStr, InputTextCalcTextSizeW};
use crate::imgui_text_input_state::ImGuiInputTextState;
use crate::imstb_text_edit_state::STB_TexteditState;

// #ifndef STB_TEXTEDIT_UNDOSTATECOUNT
// #define STB_TEXTEDIT_UNDOSTATECOUNT   99
// pub const STB_TEXTEDIT_UNDOSTATECOUNT: u32 = 99;
// #endif
// #ifndef STB_TEXTEDIT_UNDOCHARCOUNT
// #define STB_TEXTEDIT_UNDOCHARCOUNT   999
// pub const STB_TEXTEDIT_UNDOCHARCOUNT: u32 = 999;
// #endif
// #ifndef STB_TEXTEDIT_CHARTYPE
// #define STB_TEXTEDIT_CHARTYPE        int
// pub type STB_TEXTEDIT_CHARTYPE = i32;
// #endif
// #ifndef STB_TEXTEDIT_POSITIONTYPE
// #define STB_TEXTEDIT_POSITIONTYPE    int
// #endif
pub type STB_TEXTEDIT_POSITIONTYPE = i32;

// #undef STB_TEXTEDIT_STRING
// #undef STB_TEXTEDIT_CHARTYPE
// #define STB_TEXTEDIT_STRING             ImGuiInputTextState
pub type STB_TEXTEDIT_STRING = ImGuiInputTextState;
// #define STB_TEXTEDIT_CHARTYPE           ImWchar
pub type STB_TEXTEDIT_CHARTYPE = ImWchar;
// #define STB_TEXTEDIT_GETWIDTH_NEWLINE   (-1.0)
pub const STB_TEXTEDIT_GETWIDTH_NEWLINE: f32 = -1.0;
// #define STB_TEXTEDIT_UNDOSTATECOUNT     99
pub const STB_TEXTEDIT_UNDOSTATECOUNT: usize = 99;
// #define STB_TEXTEDIT_UNDOCHARCOUNT      999
pub const STB_TEXTEDIT_UNDOCHARCOUNT: usize = 999;

#[derive(Debug,Clone,Default)]
pub struct StbUndoRecord
{
   // private data
   // STB_TEXTEDIT_POSITIONTYPE  where;
   pub loc: usize,
   // STB_TEXTEDIT_POSITIONTYPE  insert_length;
   pub insert_length: usize,
   // STB_TEXTEDIT_POSITIONTYPE  delete_length;
   pub delete_length: usize,
   // int                        char_storage;
    pub char_storage: usize,
}

// typedef struct
#[derive(Debug,Clone,Default)]
pub struct StbUndoState
{
   // private data
   // StbUndoRecord          undo_rec [STB_TEXTEDIT_UNDOSTATECOUNT];
   pub undo_rec: [StbUndoRecord;STB_TEXTEDIT_UNDOSTATECOUNT as usize],
   // STB_TEXTEDIT_CHARTYPE  undo_char[STB_TEXTEDIT_UNDOCHARCOUNT];
   pub undo_char: [STB_TEXTEDIT_CHARTYPE;STB_TEXTEDIT_UNDOCHARCOUNT as usize],
   // short undo_point, redo_point;
   pub undo_point: usize,
   pub redo_point: usize,
   // int undo_char_point, redo_char_point;
   pub redo_char_point: usize,
    pub undo_char_point: usize,
}


////////////////////////////////////////////////////////////////////////
//
//     StbTexteditRow
//
// Result of layout query, used by stb_textedit to determine where
// the text in each row is.

// result of layout query
// typedef struct
#[derive(Default,Debug,Clone)]
pub struct StbTexteditRow
{
   // float x0,x1;             // starting x location, end x location (allows for align=right, etc)
   pub x0: f32,
   pub x1: f32,
   // float baseline_y_delta;  // position of baseline relative to previous row's baseline
   pub baseline_y_delta: f32,
   // float ymin,ymax;         // height of row above and below baseline
   pub ymin: f32,
   pub ymax: f32,
   // int num_chars;
   pub num_chars: usize,
}

// #endif //INCLUDE_STB_TEXTEDIT_H


////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
////
////   Implementation mode
////
////


// implementation isn't include-guarded, since it might have indirectly
// included just the "header" portion
// #ifdef STB_TEXTEDIT_IMPLEMENTATION

// #ifndef STB_TEXTEDIT_memmove
// #include <string.h>
// #define STB_TEXTEDIT_memmove memmove
// #endif

pub fn STB_TEXTEDIT_STRINGLEN(obj: *const ImGuiInputTextState) -> usize {
    obj.CurLenW
}




/////////////////////////////////////////////////////////////////////////////
//
//      Mouse input handling
//

// static ImWchar STB_TEXTEDIT_GETCHAR(const ImGuiInputTextState* obj, int idx)                      { return obj.TextW[idx]; }
pub fn STB_TEXTEDIT_GETCHAR(obj: *const ImGuiInputTextState, idx: usize) -> ImWchar {
    obj.TextW[idx]
}

// traverse the layout to locate the nearest character to a display position
// static int stb_text_locate_coord(STB_TEXTEDIT_STRING *str, float x, float y)
pub unsafe fn stb_text_locate_coord(stb_str: *mut STB_TEXTEDIT_STRING, x: f32, y: f32) -> usize
{
   // StbTexteditRow r;
   let mut r: StbTexteditRow = StbTexteditRow::new();
    // int n = STB_TEXTEDIT_STRINGLEN(str);
   let n = stb_str.len();
    // float base_y = 0, prev_x;
   let mut base_y: f32 = 0.0;
    let mut prev_x: f32 = 0.0;
    // int i=0, k;
    let mut i = 0usize;
    let mut k = 0usize;

   r.x0 = 0.0;
    r.x1 = 0.0;
    r.ymin = 0.0;
    r.ymax = 0.0;
   r.num_chars = 0;

   // search rows to find one that straddles 'y'
   while i < n {
      STB_TEXTEDIT_LAYOUTROW(&mut r, stb_str, i);
      if r.num_chars <= 0 {
          return n;
      }

      if i==0 && y < base_y + r.ymin {
          return 0;
      }

      if y < base_y + r.ymax {
          break;
      }

      i += r.num_chars;
      base_y += r.baseline_y_delta;
   }

   // below all text, return 'after' last character
   if i >= n {
       return n;
   }

   // check if it's before the beginning of the line
   if x < r.x0 {
       return i;
   }

   // check if it's before the end of the line
   if x < r.x1 {
      // search characters in row for one that straddles 'x'
      prev_x = r.x0;
      // for (k=0; k < r.num_chars; ++k) {
      for k in 0.. r.num_chars {
         // float w = STB_TEXTEDIT_GETWIDTH(str, i, k);
         let mut w = STB_TEXTEDIT_GETWIDTH(stb_str, i, k);
          if x < prev_x+w {
            if x < prev_x+w/2 {
                return k + i;
            }
            else {
                return k + i + 1;
            }
         }
         prev_x += w;
      }
      // shouldn't happen, but if it does, fall through to end-of-line case
   }

   // if the last character is a newline, return that. otherwise return 'after' the last character
   if STB_TEXTEDIT_GETCHAR(stb_str, i+r.num_chars-1) == STB_TEXTEDIT_NEWLINE {
       return (i + r.num_chars - 1);
   }
   else {
       return (i + r.num_chars);
   }
}

// API click: on mouse down, move the cursor to the clicked location, and reset the selection
// static void stb_textedit_click(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, float x, float y)
pub unsafe fn stb_textedit_click(stb_str: *mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState, x: f32, mut y: f32)
{
   // In single-line mode, just always make y = 0. This lets the drag keep working if the mouse
   // goes off the top or bottom of the text
   if state.single_line
   {
      // StbTexteditRow r;
      let mut r: StbTexteditRow = StbTexteditRow::default();
       STB_TEXTEDIT_LAYOUTROW(&mut r, stb_str, 0);
      y = r.ymin;
   }

   state.cursor = stb_text_locate_coord(stb_str, x, y);
   state.select_start = state.cursor;
   state.select_end = state.cursor;
   state.has_preferred_x = false;
}

// API drag: on mouse drag, move the cursor and selection endpoint to the clicked location
// static void stb_textedit_drag(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, float x, float y)
pub unsafe fn stb_textedit_drag(stb_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState, x: f32, mut y: f32) {
    // int p = 0;
    let mut p = 0usize;

    // In single-line mode, just always make y = 0. This lets the drag keep working if the mouse
    // goes off the top or bottom of the text
    if state.single_line {
        let mut r = StbTexteditRow::default();
        STB_TEXTEDIT_LAYOUTROW(&mut r, stb_str, 0);
        y = r.ymin;
    }

    if state.select_start == state.select_end {
        state.select_start = state.cursor;
    }

    p = stb_text_locate_coord(stb_str, x, y);
    state.cursor = p;
    state.select_end = p;
}

/////////////////////////////////////////////////////////////////////////////
//
//      Keyboard input handling
//

// forward declarations
// static void stb_text_undo(STB_TEXTEDIT_STRING *str, STB_TexteditState *state);
// static void stb_text_redo(STB_TEXTEDIT_STRING *str, STB_TexteditState *state);
// static void stb_text_makeundo_delete(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, int where, int length);
// static void stb_text_makeundo_insert(STB_TexteditState *state, int where, int length);
// static void stb_text_makeundo_replace(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, int where, int old_length, int new_length);

#[derive(Default,Debug,Clone)]
pub struct StbFindState
{
   //float x,y;    // position of n'th character
   pub x: f32,
   pub y: f32,
   // float height; // height of line
   pub height: f32,
   // int first_char, length; // first char of row, and length
   pub first_char: usize,
   pub length: usize,
   // int prev_first;  // first char of previous row
    pub prev_first: usize,
}


// find the x/y location of a character, and remember info about the previous row in
// case we get a move-up event (for page up, we'll have to rescan)
// static void stb_textedit_find_charpos(StbFindState *find, STB_TEXTEDIT_STRING *str, int n, int single_line)
pub unsafe fn stb_textedit_find_charpos(find: *mut StbFindState, stb_str: *mut STB_TEXTEDIT_STRING, n: usize, single_line: bool)
{
   // StbTexteditRow r;
   let mut r = StbTexteditRow::default();
    // int prev_start = 0;
   let mut prev_start = 0usize;
    // int z = STB_TEXTEDIT_STRINGLEN(str);
   let mut z = STB_TEXTEDIT_STRINGLEN(stb_str);
    // int i=0, first;
    let mut i = 0usize;
    let mut first = 0usize;

   if n == z {
      // if it's at the end, then find the last line -- simpler than trying to
      // explicitly handle this case in the regular code
      if single_line {
         STB_TEXTEDIT_LAYOUTROW(&mut r, stb_str, 0);
         find.y = 0.0;
         find.first_char = 0;
         find.length = z;
         find.height = r.ymax - r.ymin;
         find.x = r.x1;
      } else {
         find.y = 0.0;
         find.x = 0.0;
         find.height = 1.0;
         while i < z {
            STB_TEXTEDIT_LAYOUTROW(&mut r, stb_str, i);
            prev_start = i;
            i += r.num_chars;
         }
         find.first_char = i;
         find.length = 0;
         find.prev_first = prev_start;
      }
      return;
   }

   // search rows to find the one that straddles character n
   find.y = 0.0;

   // for(;;) {
   loop {
      STB_TEXTEDIT_LAYOUTROW(&mut r, stb_str, i);
      if n < i + r.num_chars {
          break;
      }
      prev_start = i;
      i += r.num_chars;
      find.y += r.baseline_y_delta;
   }

   find.first_char = i;
    first = i;
   find.length = r.num_chars;
   find.height = r.ymax - r.ymin;
   find.prev_first = prev_start;

   // now scan to find xpos
   find.x = r.x0;
   // for (i=0; first+i < n; ++i)
    i = 0;
   while first + i < n {
       find.x += STB_TEXTEDIT_GETWIDTH(stb_str, first, i);
       i += 1;
   }
}

// #define STB_TEXT_HAS_SELECTION(s)   ((s)->select_start != (s)->select_end)
pub fn STB_TEXT_HAS_SELECTION(s: *const STB_TexteditState) -> bool {
    s.select_start != s.select_end
}

// make the selection/cursor state valid if client altered the string
// static void stb_textedit_clamp(STB_TEXTEDIT_STRING *str, STB_TexteditState *state)
pub fn stb_textedit_clamp(stb_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState) {
    // int n = STB_TEXTEDIT_STRINGLEN(str);
    let mut n = stb_str.len();
    if STB_TEXT_HAS_SELECTION(state) {
        if state.select_start > n { state.select_start = n; }
        if state.select_end > n { state.select_end = n; }
        // if clamping forced them to be equal, move the cursor to match
        if state.select_start == state.select_end {
            state.cursor = state.select_start;
        }
    }
    if state.cursor > n { state.cursor = n; }
}


// static StbUndoRecord *stb_text_create_undo_record(StbUndoState *state, int numchars)
pub fn stb_text_create_undo_record(state: *mut StbUndoState, numchars: usize) -> *mut StbUndoRecord
    {
   // any time we create a new undo record, we discard redo
   stb_textedit_flush_redo(state);

   // if we have no free records, we have to make room, by sliding the
   // existing records down
   if state.undo_point == STB_TEXTEDIT_UNDOSTATECOUNT {
       stb_textedit_discard_undo(state);
   }

   // if the characters to store won't possibly fit in the buffer, we can't undo
   if numchars > STB_TEXTEDIT_UNDOCHARCOUNT {
      state.undo_point = 0;
      state.undo_char_point = 0;
      return null_mut();
   }

   // if we don't have enough free characters in the buffer, we have to make room
   while state.undo_char_point + numchars > STB_TEXTEDIT_UNDOCHARCOUNT {
       stb_textedit_discard_undo(state);
   }

        let x = state.undo_point;
        state.undo_point += 1;
   return &mut state.undo_rec[x];
}


// static STB_TEXTEDIT_CHARTYPE *stb_text_createundo(StbUndoState *state, int pos, int insert_len, int delete_len)
pub fn stb_text_createundo(state: *mut StbUndoState, pos: usize, insert_len: usize, delete_len: usize) -> *mut STB_TEXTEDIT_CHARTYPE
    {
   let r = stb_text_create_undo_record(state, insert_len);
   if r == null_mut() {
       return null_mut();
   }

   r.loc = pos;
   r.insert_length = insert_len;
   r.delete_length = delete_len;

   if insert_len == 0 {
      r.char_storage = -1;
      return null_mut();
   } else {
      r.char_storage = state.undo_char_point;
      state.undo_char_point += insert_len;
      return &mut state.undo_char[r.char_storage];
   }
}

// static void stb_text_makeundo_delete(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, int where, int length)
pub fn stb_text_makeundo_delete(stb_str: *mut ImGuiInputTextState, state: *mut STB_TexteditState, loc: usize, length: usize) {
    // int i;
    let mut i = 0i32;
    // STB_TEXTEDIT_CHARTYPE *p = stb_text_createundo(&state.undostate, where, length, 0);
    let mut p = stb_text_createundo(&mut state.undostate, loc, length, 0);
    if p {
        // for (i=0; i < length; i += 1){
        for i in 0..length {
            p[i] = STB_TEXTEDIT_GETCHAR(stb_str, loc + i);
        }
    }
}

// delete characters while updating undo
// static void stb_textedit_delete(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, int where, int len)
pub unsafe fn stb_textedit_delete(stb_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState, loc: usize, len: usize)
{
   stb_text_makeundo_delete(stb_str, state, loc, len);
   STB_TEXTEDIT_DELETECHARS(stb_str, loc, len);
   state.has_preferred_x = false;
}

// delete the section
// static void stb_textedit_delete_selection(STB_TEXTEDIT_STRING *str, STB_TexteditState *state)
pub unsafe fn stb_textedit_delete_selection(stb_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState)
{
   stb_textedit_clamp(stb_str, state);
   if STB_TEXT_HAS_SELECTION(state) {
      if state.select_start < state.select_end {
         stb_textedit_delete(stb_str, state, state.select_start, state.select_end - state.select_start);
         state.select_end = state.select_start;
          state.cursor = state.select_start;
      } else {
         stb_textedit_delete(stb_str, state, state.select_end, state.select_start - state.select_end);
         state.select_start = state.select_end;
          state.cursor = state.select_end;
      }
      state.has_preferred_x = false;
   }
}

// canoncialize the selection so start <= end
// static void stb_textedit_sortselection(STB_TexteditState *state)
pub fn stb_textedit_sortselection(state: *mut STB_TexteditState)
{
   if state.select_end < state.select_start {
      let mut temp = state.select_end;
      state.select_end = state.select_start;
      state.select_start = temp;
   }
}

// move cursor to first character of selection
// static void stb_textedit_move_to_first(STB_TexteditState *state)
pub fn stb_textedit_move_to_first(state: *mut STB_TexteditState)
{
   if STB_TEXT_HAS_SELECTION(state) {
      stb_textedit_sortselection(state);
      state.cursor = state.select_start;
      state.select_end = state.select_start;
      state.has_preferred_x = false;
   }
}

// move cursor to last character of selection
// static void stb_textedit_move_to_last(STB_TEXTEDIT_STRING *str, STB_TexteditState *state)
pub fn stb_textedit_move_to_last(stb_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState)
{
   if STB_TEXT_HAS_SELECTION(state) {
      stb_textedit_sortselection(state);
      stb_textedit_clamp(stb_str, state);
      state.cursor = state.select_end;
      state.select_start = state.select_end;
      state.has_preferred_x = false;
   }
}

// #ifdef STB_TEXTEDIT_IS_SPACE
// static int is_word_boundary( STB_TEXTEDIT_STRING *str, int idx )
pub fn is_word_boundary(stb_str: *mut STB_TEXTEDIT_STRING, idx: usize) -> bool {
    if idx > 0 { (STB_TEXTEDIT_IS_SPACE(STB_TEXTEDIT_GETCHAR(stb_str, idx - 1)) && !STB_TEXTEDIT_IS_SPACE(STB_TEXTEDIT_GETCHAR(stb_str, idx))) } else { true }
}

// #ifndef STB_TEXTEDIT_MOVEWORDLEFT
// static int stb_textedit_move_to_word_previous( STB_TEXTEDIT_STRING *str, int c )
pub fn stb_textedit_move_to_word_previous(stb_str: *mut STB_TEXTEDIT_STRING, mut c: usize) -> usize
{
   // --c; // always move at least one character
   c -= 1;
    while c >= 0 && !is_word_boundary(stb_str, c ) {
        c -= 1;
    }

   if c < 0 {
       c = 0;
   }

   return c;
}
// #define STB_TEXTEDIT_MOVEWORDLEFT stb_textedit_move_to_word_previous
// pub type STB_TEXTEDIT_MOVEWORDLEFT = stb_textedit_move_to_word_previous;
// #endif

// #ifndef STB_TEXTEDIT_MOVEWORDRIGHT
// static int stb_textedit_move_to_word_next( STB_TEXTEDIT_STRING *str, int c )
pub fn stb_textedit_move_to_word_next(in_str: *mut STB_TEXTEDIT_STRING, mut c: usize) -> usize
{
   // const int len = STB_TEXTEDIT_STRINGLEN(str);
   let len = in_str.len();
    // ++c; // always move at least one character
   c += 1;
    while c < len && !is_word_boundary(in_str, c) {
        c += 1;
    }

   if c > len {
       c = len;
   }

   return c;
}
// #define STB_TEXTEDIT_MOVEWORDRIGHT stb_textedit_move_to_word_next
// #endif
// pub type STB_TEXTEDIT_MOVEWORDRIGHT = stb_textedit_move_to_word_next;

// #endif

// update selection and cursor to match each other
// static void stb_textedit_prep_selection_at_cursor(STB_TexteditState *state)
pub fn stb_textedit_prep_selection_at_cursor(state: *mut STB_TexteditState)
{
   if !STB_TEXT_HAS_SELECTION(state) {
       state.select_start = state.cursor;
       state.select_end = state.cursor;
   }
   else {
       state.cursor = state.select_end;
   }
}

// API cut: delete selection
// static int stb_textedit_cut(STB_TEXTEDIT_STRING *str, STB_TexteditState *state)
pub unsafe fn stb_textedit_cut(stb_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState) -> bool
{
   if STB_TEXT_HAS_SELECTION(state) {
      stb_textedit_delete_selection(stb_str, state); // implicitly clamps
      state.has_preferred_x = false;
      return true;
   }
   return false;
}

// static void stb_text_makeundo_insert(STB_TexteditState *state, int where, int length)
pub fn stb_text_makeundo_insert(state: *mut STB_TexteditState, loc: usize, length: usize)
    {
   stb_text_createundo(&mut state.undostate, loc, 0, length);
}

// API paste: replace existing selection with passed-in text
// static int stb_textedit_paste_internal(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, STB_TEXTEDIT_CHARTYPE *text, int len)
pub unsafe fn stb_textedit_paste_internal(stb_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState, text: *mut STB_TEXTEDIT_CHARTYPE, len: usize) -> bool
{
   // if there's a selection, the paste should delete it
   stb_textedit_clamp(stb_str, state);
   stb_textedit_delete_selection(stb_str, state);
   // try to insert the characters
   if STB_TEXTEDIT_INSERTCHARS(stb_str, state.cursor, text, len) {
      stb_text_makeundo_insert(state, state.cursor, len);
      state.cursor += len;
      state.has_preferred_x = false;
      return true;
   }
   // note: paste failure will leave deleted selection, may be restored with an undo (see https://github.com/nothings/stb/issues/734 for details)
   return false;
}

// #ifndef STB_TEXTEDIT_KEYTYPE
// #define STB_TEXTEDIT_KEYTYPE int
// #endif
pub type STB_TEXTEDIT_KEYTYPE = i32;

// static void stb_text_makeundo_replace(STB_TEXTEDIT_STRING *in_str, STB_TexteditState *state, int where, int old_length, int new_length)
pub fn stb_text_makeundo_replace(stb_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState, loc: usize, old_length: usize, new_length: usize)
    {
   // int i;
   let mut i = 0i32;
        // STB_TEXTEDIT_CHARTYPE *p = stb_text_createundo(&state.undostate, where, old_length, new_length);
        let mut p = stb_text_createundo(&mut state.undostate, loc, old_length, new_length);
   if (p) {
      // for (i=0; i < old_length; i += 1)
      for i in 0 .. old_length
       {
          p[i] = STB_TEXTEDIT_GETCHAR(stb_str, loc + i);
      }
   }
}


// discard the oldest entry in the redo list--it's bad if this
// ever happens, but because undo & redo have to store the actual
// characters in different cases, the redo character buffer can
// fill up even though the undo buffer didn't
// static void stb_textedit_discard_redo(StbUndoState *state)
pub fn stb_textedit_discard_redo(state: *mut StbUndoState)
    {
   // int k = STB_TEXTEDIT_UNDOSTATECOUNT-1;
    let mut k = STB_TEXTEDIT_UNDOSTATECOUNT - 1;

   if state.redo_point <= k {
      // if the k'th undo state has characters, clean those up
      if state.undo_rec[k].char_storage >= 0 {
         // int n = state.undo_rec[k].insert_length, i;
         let mut n = state.undo_rec[k].insert_length;
          let mut i = 0;
          // move the remaining redo character data to the end of the buffer
         state.redo_char_point += n;
         // STB_TEXTEDIT_memmove(state.undo_char + state.redo_char_point, state.undo_char + state.redo_char_point-n, (size_t) ((STB_TEXTEDIT_UNDOCHARCOUNT - state.redo_char_point)*sizeof(STB_TEXTEDIT_CHARTYPE)));
         // adjust the position of all the other records to account for above memmove
         // for (i=state.redo_point; i < k; i += 1){
         for i in state.redo_point .. k {
             if state.undo_rec[i].char_storage >= 0 {
                 state.undo_rec[i].char_storage += n;
             }
         }
      }
      // now move all the redo records towards the end of the buffer; the first one is at 'redo_point'
      // [DEAR IMGUI]
      // size_t move_size = (size_t)((STB_TEXTEDIT_UNDOSTATECOUNT - state.redo_point - 1) * sizeof(state.undo_rec[0]));
      let mut move_size = ((STB_TEXTEDIT_UNDOSTATECOUNT - state.redo_point - 1) * (state.undo_rec[0]).len());
      // const char* buf_begin = (char*)state.undo_rec; (void)buf_begin;
      // TODOO
       // const char* buf_end   = (char*)state.undo_rec + sizeof(state.undo_rec); (void)buf_end;

       // IM_ASSERT(((char*)(state.undo_rec + state.redo_point)) >= buf_begin);
      // IM_ASSERT(((char*)(state.undo_rec + state.redo_point + 1) + move_size) <= buf_end);
      // STB_TEXTEDIT_memmove(state.undo_rec + state.redo_point+1, state.undo_rec + state.redo_point, move_size);

      // now move redo_point to point to the new one
      state.redo_point += 1;
   }
}




// static void stb_text_undo(STB_TEXTEDIT_STRING *in_str, STB_TexteditState *state)
pub unsafe fn stb_text_undo(in_str: *mut ImGuiInputTextState, state: *mut STB_TexteditState)
    {
   // StbUndoState *s = &state.undostate;
   let mut s = &mut state.undostate;
        // StbUndoRecord u, *r;
   let mut u = StbUndoRecord::default();
        let mut r : *mut StbUndoRecord = null_mut();
        if (s.undo_point == 0) {
            return;
        }

   // we need to do two things: apply the undo record, and create a redo record
   u = s.undo_rec[s.undo_point-1].clone();
   r = &mut s.undo_rec[s.redo_point-1].clone();
   r.char_storage = -1;

   r.insert_length = u.delete_length;
   r.delete_length = u.insert_length;
   r.loc = u.lock;

   if (u.delete_length) {
      // if the undo record says to delete characters, then the redo record will
      // need to re-insert the characters that get deleted, so we need to store
      // them.

      // there are three cases:
      //    there's enough room to store the characters
      //    characters stored for *redoing* don't leave room for redo
      //    characters stored for *undoing* don't leave room for redo
      // if the last is true, we have to bail

      if (s.undo_char_point + u.delete_length >= STB_TEXTEDIT_UNDOCHARCOUNT) {
         // the undo records take up too much character space; there's no space to store the redo characters
         r.insert_length = 0;
      } else {
         let mut i = 0i32;

         // there's definitely room to store the characters eventually
         while (s.undo_char_point + u.delete_length > s.redo_char_point) {
            // should never happen:
            if (s.redo_point == STB_TEXTEDIT_UNDOSTATECOUNT) {
                return;
            }
            // there's currently not enough room, so discard a redo record
            stb_textedit_discard_redo(s);
         }
         r = &mut s.undo_rec[s.redo_point-1];

         r.char_storage = (s.redo_char_point - u.delete_length);
         s.redo_char_point = s.redo_char_point - u.delete_length;

         // now save the characters
         // for (i=0; i < u.delete_length; i += 1)
         for i in 0 .. u.delete_length {
             s.undo_char[r.char_storage + i] = STB_TEXTEDIT_GETCHAR(in_str, u.loc + i);
         }
      }

      // now we can carry out the deletion
      STB_TEXTEDIT_DELETECHARS(in_str, u.loc, u.delete_length);
   }

   // check type of recorded action:
   if (u.insert_length) {
      // easy case: was a deletion, so we need to insert n characters
      STB_TEXTEDIT_INSERTCHARS(in_str, u.loc, &mut s.undo_char[u.char_storage], u.insert_length);
      s.undo_char_point -= u.insert_length;
   }

   state.cursor = u.loc + u.insert_length;

   s.undo_point -= 1;
   s.redo_point -= 1;
}


// static void stb_text_redo(STB_TEXTEDIT_STRING *in_str, STB_TexteditState *state)
pub unsafe fn stb_text_redo(stb_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState)
    {
   // StbUndoState *s = &state.undostate;
   let s: *mut StbUndoState = &mut state.undostate;
        // StbUndoRecord *u, r;
   let mut r: *mut StbUndoRecord = null_mut();
        let mut u: *mut StbUndoRecord = null_mut();
        if s.redo_point == STB_TEXTEDIT_UNDOSTATECOUNT {
            return;
        }

   // we need to do two things: apply the redo record, and create an undo record
   u = &mut s.undo_rec[s.undo_point];
   r = &mut s.undo_rec[s.redo_point];

   // we KNOW there must be room for the undo record, because the redo record
   // was derived from an undo record

   u.delete_length = r.insert_length;
   u.insert_length = r.delete_length;
   u.loc = r.loc;
   u.char_storage = -1;

   if r.delete_length {
      // the redo record requires us to delete characters, so the undo record
      // needs to store the characters

      if s.undo_char_point + u.insert_length > s.redo_char_point {
         u.insert_length = 0;
         u.delete_length = 0;
      } else {
         // int i;
         let mut i = 0i32;
          u.char_storage = s.undo_char_point;
         s.undo_char_point = s.undo_char_point + u.insert_length;

         // now save the characters
         // for (i=0; i < u.insert_length; i += 1)
         for i in 0 .. u.insert_length
          { s.undo_char[u.char_storage + i] = STB_TEXTEDIT_GETCHAR(stb_str, u.loc + i); }
      }

      STB_TEXTEDIT_DELETECHARS(stb_str, r.loc, r.delete_length);
   }

   if r.insert_length {
      // easy case: need to insert n characters
      STB_TEXTEDIT_INSERTCHARS(stb_str, r.loc, &mut s.undo_char[r.char_storage], r.insert_length);
      s.redo_char_point += r.insert_length;
   }

   state.cursor = r.loc + r.insert_length;

   s.undo_point += 1;
   s.redo_point += 1;
}




// API key: process a keyboard input
// static void stb_textedit_key(STB_TEXTEDIT_STRING *str, STB_TexteditState *state, STB_TEXTEDIT_KEYTYPE key)
pub unsafe fn stb_textedit_key(in_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState, mut key: STB_TEXTEDIT_KEYTYPE)
{
// retry:
   match key {
      _ => {
         let mut c = STB_TEXTEDIT_KEYTOTEXT(key);
         if c > 0 {
            let mut ch = c as STB_TEXTEDIT_CHARTYPE;

            // can't add newline in single-line mode
            if c == '\n' as i32 && (state.single_line) {
                // break;
            }

            if( state.insert_mode != 0) && !STB_TEXT_HAS_SELECTION(state) && (state.cursor < STB_TEXTEDIT_STRINGLEN(in_str)) {
               stb_text_makeundo_replace(in_str, state, state.cursor, 1, 1);
               STB_TEXTEDIT_DELETECHARS(in_str, state.cursor, 1);
               if STB_TEXTEDIT_INSERTCHARS(in_str, state.cursor, &mut ch, 1) {
                  // ++state.cursor;
                  state.cursor += 1;
                   state.has_preferred_x = false;
               }
            } else {
               stb_textedit_delete_selection(in_str,state); // implicitly clamps
               if STB_TEXTEDIT_INSERTCHARS(in_str, state.cursor, &mut ch, 1) {
                  stb_text_makeundo_insert(state, state.cursor, 1);
                  // ++state.cursor;
                  state.cursor += 1;
                   state.has_preferred_x = false;
               }
            }
         }
         // break;
      }

// #ifdef STB_TEXTEDIT_K_INSERT
//       case STB_TEXTEDIT_K_INSERT:
       STB_TEXTEDIT_K_INSERT => {
           state.insert_mode = !state.insert_mode;
           // break;
       }
// #endif

      // case STB_TEXTEDIT_K_UNDO:
       STB_TEXTEDIT_K_UNDO => {
           stb_text_undo(in_str, state);
           state.has_preferred_x = false;
           // break;
       }

      // case STB_TEXTEDIT_K_REDO:
       STB_TEXTEDIT_K_REDO => {
           stb_text_redo(in_str, state);
           state.has_preferred_x = false;
           // break;
       }

      // case STB_TEXTEDIT_K_LEFT:
       STB_TEXTEDIT_K_LEFT => {
           // if currently there's a selection, move cursor to start of selection
           if (STB_TEXT_HAS_SELECTION(state)) {
               stb_textedit_move_to_first(state);
           }
           else {
               if state.cursor > 0 { state.cursor -= 1; }
               {
                   state.has_preferred_x = false;
               }
           }
           // break;
       }

      // case STB_TEXTEDIT_K_RIGHT:
       STB_TEXTEDIT_K_RIGHT => {
           // if currently there's a selection, move cursor to end of selection
           if (STB_TEXT_HAS_SELECTION(state)) {
               stb_textedit_move_to_last(in_str, state);
           }
           else { state.cursor += 1; }
           stb_textedit_clamp(in_str, state);
           state.has_preferred_x = false;
           // break;
       }

      // case STB_TEXTEDIT_K_LEFT | STB_TEXTEDIT_K_SHIFT:
        STB_TEXTEDIT_K_LEFT | STB_TEXTEDIT_K_SHIFT => {
            stb_textedit_clamp(in_str, state);
            stb_textedit_prep_selection_at_cursor(state);
            // move selection left
            if state.select_end > 0 { state.select_end -= 1; }
            state.cursor = state.select_end;
            state.has_preferred_x = false;
            // break;
        }

// #ifdef STB_TEXTEDIT_MOVEWORDLEFT
//       case STB_TEXTEDIT_K_WORDLEFT:
       STB_TEXTEDIT_K_WORDLEFT => {
           if STB_TEXT_HAS_SELECTION(state) {
               stb_textedit_move_to_first(state);
           }
           else {
               state.cursor = stb_textedit_move_to_word_previous(in_str, state.cursor);
               stb_textedit_clamp(in_str, state);
           }
           // break;
       }

      // case STB_TEXTEDIT_K_WORDLEFT | STB_TEXTEDIT_K_SHIFT:
      STB_TEXTEDIT_K_WORDLEFT | STB_TEXTEDIT_K_SHIFT => {
          if (!STB_TEXT_HAS_SELECTION(state)) {
              stb_textedit_prep_selection_at_cursor(state);
          }

          state.cursor = stb_textedit_move_to_word_previous(in_str, state.cursor);
          state.select_end = state.cursor;

          stb_textedit_clamp(in_str, state);
          // break;
      }
// #endif

// #ifdef STB_TEXTEDIT_MOVEWORDRIGHT
//       case STB_TEXTEDIT_K_WORDRIGHT:
       STB_TEXTEDIT_K_WORDRIGHT => {
           if (STB_TEXT_HAS_SELECTION(state)) {
               stb_textedit_move_to_last(in_str, state);
           }
           else {
               state.cursor = stb_textedit_move_to_word_next(in_str, state.cursor);
               stb_textedit_clamp(in_str, state);
           }
           // break;
       }

      // case STB_TEXTEDIT_K_WORDRIGHT | STB_TEXTEDIT_K_SHIFT:
      STB_TEXTEDIT_K_WORDRIGHT | STB_TEXTEDIT_K_SHIFT => {
          if (!STB_TEXT_HAS_SELECTION(state)) {
              stb_textedit_prep_selection_at_cursor(state);
          }

          state.cursor = stb_textedit_move_to_word_next(in_str, state.cursor);
          state.select_end = state.cursor;

          stb_textedit_clamp(in_str, state);
          // break;
      }
// #endif

      // case STB_TEXTEDIT_K_RIGHT | STB_TEXTEDIT_K_SHIFT:
       STB_TEXTEDIT_K_RIGHT | STB_TEXTEDIT_K_SHIFT => {
           stb_textedit_prep_selection_at_cursor(state);
           // move selection right
           state.select_end += 1;
           stb_textedit_clamp(in_str, state);
           state.cursor = state.select_end;
           state.has_preferred_x = false;
           // break;
       }

      // case STB_TEXTEDIT_K_DOWN:
      // case STB_TEXTEDIT_K_DOWN | STB_TEXTEDIT_K_SHIFT:
      // case STB_TEXTEDIT_K_PGDOWN:
      // case STB_TEXTEDIT_K_PGDOWN | STB_TEXTEDIT_K_SHIFT:

       STB_TEXTEDIT_K_DOWN | STB_TEXTEDIT_K_SHIFT | STB_TEXTEDIT_K_PGDOWN =>
       {
         // StbFindState find;
         let mut find = StbFindState::default();
           // StbTexteditRow row;
         let mut row = StbTexteditRow::default();
           // int i, j, sel = (key & STB_TEXTEDIT_K_SHIFT) != 0;
         let mut i = (key & STB_TEXTEDIT_K_SHIFT) != 0;
           let mut j = (key &STB_TEXTEDIT_K_SHIFT) != 0;
           let mut sel = key & STB_TEXTEDIT_K_SHIFT != 0;
           // int is_page = (key & ~STB_TEXTEDIT_K_SHIFT) == STB_TEXTEDIT_K_PGDOWN;
         let mut is_page = ((key & !STB_TEXTEDIT_K_SHIFT) == STB_TEXTEDIT_K_PGDOWN as STB_TEXTEDIT_KEYTYPE);
           // int row_count = is_page ? state.row_count_per_page : 1;
            let mut row_count = if is_page {
                state.row_count_per_page
            } else {
                1
            };

         if !is_page && state.single_line != false {
            // on windows, up&down in single-line behave like left&right
            key = (STB_TEXTEDIT_K_RIGHT | (key & STB_TEXTEDIT_K_SHIFT)) as STB_TEXTEDIT_KEYTYPE;
             // TODO:
            // goto retry;
         }

         if sel {
             stb_textedit_prep_selection_at_cursor(state);
         }
         else if STB_TEXT_HAS_SELECTION(state) {
             stb_textedit_move_to_last(in_str, state);
         }

         // compute current position of cursor point
         stb_textedit_clamp(in_str, state);
         stb_textedit_find_charpos(&mut find, in_str, state.cursor, state.single_line);

         // for (j = 0; j < row_count; ++j) {
           for j in 0 .. row_count {
            // float x, goal_x = state.has_preferred_x ? state.preferred_x : find.x;
          let mut x = if state.has_preferred_x != false { state.preferred_x } else { find.x};
               let goal_x = x;
            // int start = find.first_char + find.length;
            let start = find.first_char + find.length;

            if find.length == 0 {
                break;
            }

            // [DEAR IMGUI]
            // going down while being on the last line shouldn't bring us to that line end
            if (STB_TEXTEDIT_GETCHAR(in_str, find.first_char + find.length - 1) != STB_TEXTEDIT_NEWLINE) {
                // break;
            }

            // now find character position down a row
            state.cursor = start;
            STB_TEXTEDIT_LAYOUTROW(&mut row, in_str, state.cursor);
            x = row.x0;
            // for (i=0; i < row.num_chars; ++i) {
            for i in 0 .. row.num_chars {
               // float dx = STB_TEXTEDIT_GETWIDTH(str, start, i);
               let mut dx =  STB_TEXTEDIT_GETWIDTH(in_str, start, i);
                // #ifdef STB_TEXTEDIT_GETWIDTH_NEWLINE
               if dx == STB_TEXTEDIT_GETWIDTH_NEWLINE {
                   break;
               }
               // #endif
               x += dx;
               if x > goal_x {
                   break;
               }
               state.cursor += 1;
            }
            stb_textedit_clamp(in_str, state);

            state.has_preferred_x = true;
            state.preferred_x = goal_x;

            if sel {
                state.select_end = state.cursor;
            }

            // go to next line
            find.first_char = find.first_char + find.length;
            find.length = row.num_chars;
         }
         // break;
      }

      // case STB_TEXTEDIT_K_UP:
      // case STB_TEXTEDIT_K_UP | STB_TEXTEDIT_K_SHIFT:
      // case STB_TEXTEDIT_K_PGUP:
      // case STB_TEXTEDIT_K_PGUP | STB_TEXTEDIT_K_SHIFT:

       STB_TEXTEDIT_K_UP | STB_TEXTEDIT_K_SHIFT | STB_TEXTEDIT_K_PGUP =>
       {
         // StbFindState find;
         let mut find = StbFindState::new();
           // StbTexteditRow row;
         let mut row = StbTexteditRow::new();
           // int i, j, prev_scan, sel = (key & STB_TEXTEDIT_K_SHIFT) != 0;
           let mut sel = (key & STB_TEXTEDIT_K_SHIFT) != 0;
           let mut i = sel;
           let mut j = sel;
           let mut prev_scan = sel;
         // int is_page = (key & ~STB_TEXTEDIT_K_SHIFT) == STB_TEXTEDIT_K_PGUP;
        let mut is_page = (key & !STB_TEXTEDIT_K_SHIFT) == STB_TEXTEDIT_K_PGUP as STB_TEXTEDIT_KEYTYPE;
         // int row_count = is_page ? state.row_count_per_page : 1;
        let mut row_count = if is_page { state.row_count_per_page} else { 1};

         if !is_page && state.single_line != false {
            // on windows, up&down become left&right
            key = (STB_TEXTEDIT_K_LEFT | (key & STB_TEXTEDIT_K_SHIFT)) as STB_TEXTEDIT_KEYTYPE;
            // goto retry;
         }

         if sel {
             stb_textedit_prep_selection_at_cursor(state);
         }
         else if (STB_TEXT_HAS_SELECTION(state)) {
             stb_textedit_move_to_first(state);
         }
         // compute current position of cursor point
         stb_textedit_clamp(in_str, state);
         stb_textedit_find_charpos(&mut find, in_str, state.cursor, state.single_line);

         // for (j = 0; j < row_count; j += 1) {
         for j in 0 .. row_count {
            // float  x, goal_x = state.has_preferred_x ? state.preferred_x : find.x;
             let mut x = if state.has_preferred_x { state.preferred_x} else {find.x};
             let mut goal_x = x;

            // can only go up if there's a previous row
            if (find.prev_first == find.first_char) {
                // break;
            }

            // now find character position up a row
            state.cursor = find.prev_first;
            STB_TEXTEDIT_LAYOUTROW(&mut row, in_str, state.cursor);
            x = row.x0;
            // for (i=0; i < row.num_chars; i += 1) {
            for i in 0 .. row.num_chars {
               // float dx = STB_TEXTEDIT_GETWIDTH(in_str, find.prev_first, i);
              let mut dx = STB_TEXTEDIT_GETWIDTH(in_str, find.prev_first, i);
               // #ifdef STB_TEXTEDIT_GETWIDTH_NEWLINE
               if dx == STB_TEXTEDIT_GETWIDTH_NEWLINE {
                   break;
               }
               // #endif
               x += dx;
               if x > goal_x {
                   // break;
               }
               state.cursor += 1;
            }
            stb_textedit_clamp(in_str, state);

            state.has_preferred_x = true;
            state.preferred_x = goal_x;

            if sel {
                state.select_end = state.cursor;
            }

            // go to previous line
            // (we need to scan previous line the hard way. maybe we could expose this as a new API function?)
            // prev_scan = find.prev_first > 0 ? find.prev_first - 1 : 0;
            let mut prev_scan = if find.prev_first > 0 {
                find.prev_first - 1
            } else {
                0
            };
             while prev_scan > 0 && STB_TEXTEDIT_GETCHAR(in_str, prev_scan - 1) != STB_TEXTEDIT_NEWLINE {
                 prev_scan -= 1;
             }
            find.first_char = find.prev_first;
            find.prev_first = prev_scan;
         }
         // break;
      }

      // case STB_TEXTEDIT_K_DELETE:
      // case STB_TEXTEDIT_K_DELETE | STB_TEXTEDIT_K_SHIFT:
      STB_TEXTEDIT_K_DELETE | STB_TEXTEDIT_K_SHIFT => {
          if (STB_TEXT_HAS_SELECTION(state)) {
              stb_textedit_delete_selection(in_str, state);
          }
          else {
              let mut n = in_str.len();
              if state.cursor < n {
                  stb_textedit_delete(in_str, state, state.cursor, 1);
              }
          }
          state.has_preferred_x = false;
          // break;
      }

      // case STB_TEXTEDIT_K_BACKSPACE:
      // case STB_TEXTEDIT_K_BACKSPACE | STB_TEXTEDIT_K_SHIFT:
       STB_TEXTEDIT_K_BACKSPACE | STB_TEXTEDIT_K_SHIFT=> {
           if STB_TEXT_HAS_SELECTION(state) {
               stb_textedit_delete_selection(in_str, state);
           }
           else {
               stb_textedit_clamp(in_str, state);
               if state.cursor > 0 {
                   stb_textedit_delete(in_str, state, state.cursor - 1, 1);
                   state.cursor -= 1;
               }
           }
           state.has_preferred_x = false;
           // break;
       }

// #ifdef STB_TEXTEDIT_K_TEXTSTART2
//       case STB_TEXTEDIT_K_TEXTSTART2:
// #endif
//       case STB_TEXTEDIT_K_TEXTSTART:
       STB_TEXTEDIT_K_TEXTSTART => {
           state.cursor = 0;
           state.select_start = 0;
           state.select_end = 0;
           state.has_preferred_x = false;
           // break;
       }

// #ifdef STB_TEXTEDIT_K_TEXTEND2
//       case STB_TEXTEDIT_K_TEXTEND2:
// #endif
//       case STB_TEXTEDIT_K_TEXTEND:
       STB_TEXTEDIT_K_TEXTEND => {
           state.cursor = in_str.len();
           state.select_start = 0;
           state.select_end = 0;
           state.has_preferred_x = false;
           // break;
       }
// #ifdef STB_TEXTEDIT_K_TEXTSTART2
//       case STB_TEXTEDIT_K_TEXTSTART2 | STB_TEXTEDIT_K_SHIFT:
// #endif
//       case STB_TEXTEDIT_K_TEXTSTART | STB_TEXTEDIT_K_SHIFT:
       STB_TEXTEDIT_K_SHIFT |  STB_TEXTEDIT_K_TEXTSTART => {
           stb_textedit_prep_selection_at_cursor(state);
           state.cursor = 0;
           state.select_end = 0;
           state.has_preferred_x = false;
           // break;
           }

// #ifdef STB_TEXTEDIT_K_TEXTEND2
//       case STB_TEXTEDIT_K_TEXTEND2 | STB_TEXTEDIT_K_SHIFT:
// #endif
//       case STB_TEXTEDIT_K_TEXTEND | STB_TEXTEDIT_K_SHIFT:
           STB_TEXTEDIT_K_SHIFT | STB_TEXTEDIT_K_TEXTEND => {
               stb_textedit_prep_selection_at_cursor(state);
               state.cursor = in_str.len();
               state.select_end = in_str.len();
               state.has_preferred_x = false;
               // break;
           }


// #ifdef STB_TEXTEDIT_K_LINESTART2
//       case STB_TEXTEDIT_K_LINESTART2:
// #endif
//       case STB_TEXTEDIT_K_LINESTART:
           STB_TEXTEDIT_K_LINESTART => {
               stb_textedit_clamp(in_str, state);
               stb_textedit_move_to_first(state);
               if state.single_line {
                   state.cursor = 0;
               }
               else {
                   while state.cursor > 0 && STB_TEXTEDIT_GETCHAR(in_str, state.cursor - 1) != STB_TEXTEDIT_NEWLINE {
                       state.cursor -= 1;
                   }
               }
               state.has_preferred_x = false;
               // break;
           }

// #ifdef STB_TEXTEDIT_K_LINEEND2
//       case STB_TEXTEDIT_K_LINEEND2:
// #endif
//       case STB_TEXTEDIT_K_LINEEND:
          STB_TEXTEDIT_K_LINEEND => {
         let mut  n = in_str.len();
         stb_textedit_clamp(in_str, state);
         stb_textedit_move_to_first(state);
         if state.single_line {
             state.cursor = n;
         }
         else {
             while state.cursor < n && STB_TEXTEDIT_GETCHAR(in_str, state.cursor) != STB_TEXTEDIT_NEWLINE {
                 state.cursor += 1;
             }
         }
         state.has_preferred_x = false;
         // break;
      }

// #ifdef STB_TEXTEDIT_K_LINESTART2
//       case STB_TEXTEDIT_K_LINESTART2 | STB_TEXTEDIT_K_SHIFT:
// #endif
//       case STB_TEXTEDIT_K_LINESTART | STB_TEXTEDIT_K_SHIFT:
          STB_TEXTEDIT_K_SHIFT | STB_TEXTEDIT_K_LINESTART=> {
              stb_textedit_clamp(in_str, state);
              stb_textedit_prep_selection_at_cursor(state);
              if state.single_line {
                  state.cursor = 0;
              }
              else {
                  while state.cursor > 0 && STB_TEXTEDIT_GETCHAR(in_str, state.cursor - 1) != STB_TEXTEDIT_NEWLINE {
                      state.cursor -= 1;
                  }
              }
              state.select_end = state.cursor;
              state.has_preferred_x = false;
              // break;
          }

// #ifdef STB_TEXTEDIT_K_LINEEND2
//       case STB_TEXTEDIT_K_LINEEND2 | STB_TEXTEDIT_K_SHIFT:
// #endif
//       case STB_TEXTEDIT_K_LINEEND | STB_TEXTEDIT_K_SHIFT:
       STB_TEXTEDIT_K_LINEEND | STB_TEXTEDIT_K_SHIFT =>
       {
         // int n = STB_TEXTEDIT_STRINGLEN(in_str);
         let mut n = in_str.len();
           stb_textedit_clamp(in_str, state);
         stb_textedit_prep_selection_at_cursor(state);
         if state.single_line {
             state.cursor = n;
         }
         else {while state.cursor < n && STB_TEXTEDIT_GETCHAR(in_str, state.cursor) != STB_TEXTEDIT_NEWLINE {
           state.cursor += 1;
       }}
         state.select_end = state.cursor;
         state.has_preferred_x = false;
         // break;
      }
   }
}

/////////////////////////////////////////////////////////////////////////////
//
//      Undo processing
//
// @OPTIMIZE: the undo/redo buffer should be circular

// static void stb_textedit_flush_redo(StbUndoState *state)
pub fn stb_textedit_flush_redo(state: *mut StbUndoState)
{
   state.redo_point = STB_TEXTEDIT_UNDOSTATECOUNT;
   state.redo_char_point = STB_TEXTEDIT_UNDOCHARCOUNT;
}

// discard the oldest entry in the undo list
// static void stb_textedit_discard_undo(StbUndoState *state)
pub fn stb_textedit_discard_undo(state: *mut StbUndoState)
{
   if state.undo_point > 0 {
      // if the 0th undo state has characters, clean those up
      if state.undo_rec[0].char_storage >= 0 {
         // int n =, i;
         let mut n =  state.undo_rec[0].insert_length;
          let mut i = 0;
          // delete n characters from all other records
         state.undo_char_point -= n;
         // STB_TEXTEDIT_memmove(state.undo_char, state.undo_char + n, (size_t) (state.undo_char_point*sizeof(STB_TEXTEDIT_CHARTYPE)));
         // TODO
         //  for (i=0; i < state.undo_point; i += 1)
          for i in 0..state.undo_point {
          if state.undo_rec[i].char_storage >= 0 {
              state.undo_rec[i].char_storage -= n; // @OPTIMIZE: get rid of char_storage and infer it}
          }
      }
      state.undo_point -= 1;
      // STB_TEXTEDIT_memmove(state.undo_rec, state.undo_rec+1, (size_t) (state.undo_point*sizeof(state.undo_rec[0])));
   }
}







// reset the state to default
// static void stb_textedit_clear_state(STB_TexteditState *state, int is_single_line)
pub fn stb_textedit_clear_state(state: *mut STB_TexteditState, is_single_line: bool)
    {
   state.undostate.undo_point = 0;
   state.undostate.undo_char_point = 0;
   state.undostate.redo_point = STB_TEXTEDIT_UNDOSTATECOUNT;
   state.undostate.redo_char_point = STB_TEXTEDIT_UNDOCHARCOUNT;
   state.select_end = 0;
        state.select_start = 0;
   state.cursor = 0;
   state.has_preferred_x = false;
   state.preferred_x = 0.0;
   state.cursor_at_end_of_line = false;
   state.initialized = true;
   state.single_line = is_single_line;
   state.insert_mode = 0;
   state.row_count_per_page = 0;
}

// API initialize
// static void stb_textedit_initialize_state(STB_TexteditState *state, int is_single_line)
pub fn stb_textedit_initialize_state(state: *mut STB_TexteditState, is_single_line: bool)
    {
   stb_textedit_clear_state(state, is_single_line);
}

// #if defined(__GNUC__) || defined(__clang__)
// #pragma GCC diagnostic push
// #pragma GCC diagnostic ignored "-Wcast-qual"
// #endif

// static int stb_textedit_paste(STB_TEXTEDIT_STRING *in_str, STB_TexteditState *state, STB_TEXTEDIT_CHARTYPE const *ctext, int len)
pub unsafe fn stb_textedit_paste(stb_str: *mut STB_TEXTEDIT_STRING, state: *mut STB_TexteditState, ctext: *mut STB_TEXTEDIT_CHARTYPE, len: usize) -> bool {
    return stb_textedit_paste_internal(stb_str, state, ctext, len);
}

// #if defined(__GNUC__) || defined(__clang__)
// #pragma GCC diagnostic pop
// #endif
//
// #endif//STB_TEXTEDIT_IMPLEMENTATION


// static int     STB_TEXTEDIT_STRINGLEN(const ImGuiInputTextState* obj)                             { return obj.CurLenW; }
    pub fn STB_TEXTEDIT_STRINGLEN(obj: *const ImGuiInputTextState) -> usize {
obj.CurLenW}
    }

// static float   STB_TEXTEDIT_GETWIDTH(ImGuiInputTextState* obj, int line_start_idx, int char_idx)  { ImWchar c = obj.TextW[line_start_idx + char_idx]; if (c == '\n') return STB_TEXTEDIT_GETWIDTH_NEWLINE; ImGuiContext& g = *GImGui; return g.font->get_char_advance(c) * (g.font_size / g.font->font_size); }
pub fn STB_TEXTEDIT_GETWIDTH(obj: *mut ImGuiInputTextState, line_start_idx: usize, char_idx: usize) -> f32 {
    let mut c = obj.TextW[line_start_idx + char_idx];
    if c == ImWchar::from('\n') {
        return STB_TEXTEDIT_GETWIDTH_NEWLINE;
    }

    GImGui.font.GetCharAdvance(c) * GImGui.FontSize / GImGui.font.FontSize
}
// static int     STB_TEXTEDIT_KEYTOTEXT(int key)                                                    { return key >= 0x200000 ? 0 : key; }
pub fn STB_TEXTEDIT_KEYTOTEXT(key: i32) -> i32 {
    if key >= 0x200000 {
         0
    } else {
        key
    }
}
pub const STB_TEXTEDIT_NEWLINE: ImWchar = ImWchar::from('\n');
// static void    STB_TEXTEDIT_LAYOUTROW(StbTexteditRow* r, ImGuiInputTextState* obj, int line_start_idx)
pub unsafe fn STB_TEXTEDIT_LAYOUTROW(r: *mut StbTexteditRow, obj: *mut ImGuiInputTextState, line_start_idx: usize)
{
    // const ImWchar* text = obj.TextW.data;
    let text = &mut obj.TextW;
    // const ImWchar* text_remaining = NULL;
    let mut text_remaining: *mut ImWchar = null_mut();
    // const Vector2D size = InputTextCalcTextSizeW(text + line_start_idx, text + obj.CurLenW, &text_remaining, NULL, true);
    let mut size = InputTextCalcTextSizeW(text + line_start_idx, text + obj.CurLenW, &mut text_remaining, null_mut(), true);
    r.x0 = 0.0;
    r.x1 = size.x;
    r.baseline_y_delta = size.y;
    r.ymin = 0.0;
    r.ymax = size.y;
    r.num_chars = (text_remaining - (text + line_start_idx));
}

// When ImGuiInputTextFlags_Password is set, we don't want actions such as CTRL+Arrow to leak the fact that underlying data are blanks or separators.
// static bool is_separator(unsigned int c)


// static int  is_word_boundary_from_left(ImGuiInputTextState* obj, int idx)
pub fn is_word_boundary_from_left(obj: *mut ImGuiInputTextState, idx: usize) -> bool
{
    if &obj.flags & ImGuiInputTextFlags::Password { return false; };
    return if idx > 0 { (!is_separator(obj.TextW[idx - 1]) && is_separator(obj.TextW[idx])) }else { true };
}

// static int  STB_TEXTEDIT_MOVEWORDLEFT_IMPL(ImGuiInputTextState* obj, int idx)
pub fn STB_TEXTEDIT_MOVEWORDLEFT_IMPL(obj: *mut ImGuiInputTextState, mut idx: usize) -> usize {
    idx -= 1;
    while idx >= 0 && !is_word_boundary_from_right(obj, idx) { idx -= 1 }
    return if idx < 0 { 0 } else { idx };
}

// static int  STB_TEXTEDIT_MOVEWORDRIGHT_MAC(ImGuiInputTextState* obj, int idx)
pub fn STB_TEXTEDIT_MOVEWORDRIGHT_MAC(obj: *mut ImGuiInputTextState, mut idx: usize) -> usize
{
    idx += 1;
    let mut len = obj.CurLenW;
    while (idx < len && !is_word_boundary_from_left(obj, idx)) { idx += 1 }
    return if idx > len { len } else { idx };
}

// #define STB_TEXTEDIT_MOVEWORDLEFT   STB_TEXTEDIT_MOVEWORDLEFT_IMPL    // They need to be #define for stb_textedit.h
// pub type STB_TEXTEDIT_MOVEWORDLEFT = STB_TEXTEDIT_MOVEWORDLEFT_IMPL;
// #ifdef __APPLE__    // FIXME: Move setting to io structure
// #define STB_TEXTEDIT_MOVEWORDRIGHT  STB_TEXTEDIT_MOVEWORDRIGHT_MAC
// #else
// static int  STB_TEXTEDIT_MOVEWORDRIGHT_WIN(ImGuiInputTextState* obj, int idx)   { idx += 1; int len = obj.CurLenW; while (idx < len && !is_word_boundary_from_right(obj, idx)) idx += 1; return idx > len ? len : idx; }
// #define STB_TEXTEDIT_MOVEWORDRIGHT  STB_TEXTEDIT_MOVEWORDRIGHT_WIN
// #endif

// static void STB_TEXTEDIT_DELETECHARS(ImGuiInputTextState* obj, int pos, int n)
pub unsafe fn STB_TEXTEDIT_DELETECHARS(obj: *mut ImGuiInputTextState, pos: usize, n: usize) {
    // ImWchar* dst = obj.TextW.data + pos;
    let mut dst: *mut ImWchar = &obj.TextW + pos;

    // We maintain our buffer length in both UTF-8 and wchar formats
    obj.Edited = true;
    obj.CurLenA -= ImTextCountUtf8BytesFromStr(dst, dst + n);
    obj.CurLenW -= n;

    // Offset remaining text (FIXME-OPT: Use memmove)
    // const ImWchar* src = obj.TextW.data + pos + n;
    let mut src: *mut ImWchar = &mut obj.TextW + pos + n;
    let mut c: ImWchar = *src;
    while c != 0 {
        *dst = c;
        dst += 1;
        src += 1;
    }
    *dst = ImWchar::from('\0');
}

// static bool STB_TEXTEDIT_INSERTCHARS(ImGuiInputTextState* obj, int pos, const ImWchar* new_text, int new_text_len)
pub unsafe fn STB_TEXTEDIT_INSERTCHARS(obj: *mut ImGuiInputTextState, pos: usize, new_text: *mut ImWchar, new_text_len: usize) -> bool
    {
    // const bool is_resizable = (obj.flags & ImGuiInputTextFlags_CallbackResize) != 0;
    let is_resizable = obj.flags & ImGuiInputTextFlags::CallbackResize != 0;
    // const int text_len = obj.CurLenW;
    let text_len = obj.CurLenW;
        // IM_ASSERT(pos <= text_len);

    let new_text_len_utf8 = ImTextCountUtf8BytesFromStr(new_text, new_text + new_text_len);
    if (!is_resizable && (new_text_len_utf8 + obj.CurLenA + 1 > obj.BufCapacityA)) {
        return false;
    }

    // Grow internal buffer if needed
    if (new_text_len + text_len + 1 > obj.TextW.size)
    {
        if (!is_resizable) {
            return false;
        }
        // IM_ASSERT(text_len < obj.TextW.size);
        obj.TextW.reserve(text_len + usize::clamp(new_text_len * 4, 32, usize::max(256, new_text_len)) + 1);
    }

    // ImWchar* text = obj.TextW.data;
    let text = obj.TextW.data;
        if pos != text_len {
        // TODO
        // memmove(text + pos + new_text_len, text + pos, (text_len - pos) * sizeof(ImWchar));
    }
    // memcpy(text + pos, new_text, new_text_len * sizeof(ImWchar));
    // TODO

    obj.Edited = true;
    obj.CurLenW += new_text_len;
    obj.CurLenA += new_text_len_utf8;
    obj.TextW[obj.CurLenW] = ImWchar::from('\0');

    return true;
}

// We don't use an enum so we can build even with conflicting symbols (if another user of stb_textedit.h leak their STB_TEXTEDIT_K_* symbols)
pub  const STB_TEXTEDIT_K_LEFT: u32 =         0x200000; // keyboard input to move cursor left
pub const STB_TEXTEDIT_K_RIGHT: u32 =         0x200001; // keyboard input to move cursor right
pub const STB_TEXTEDIT_K_UP: u32 =           0x200002; // keyboard input to move cursor up
pub const STB_TEXTEDIT_K_DOWN: u32 =         0x200003; // keyboard input to move cursor down
pub const STB_TEXTEDIT_K_LINESTART: u32 =    0x200004; // keyboard input to move cursor to start of line
pub const STB_TEXTEDIT_K_LINEEND: u32 =      0x200005; // keyboard input to move cursor to end of line
pub const STB_TEXTEDIT_K_TEXTSTART: u32 =    0x200006; // keyboard input to move cursor to start of text
pub const STB_TEXTEDIT_K_TEXTEND: u32 =      0x200007; // keyboard input to move cursor to end of text
pub const STB_TEXTEDIT_K_DELETE: u32 =       0x200008; // keyboard input to delete selection or character under cursor
pub const STB_TEXTEDIT_K_BACKSPACE: u32 =    0x200009; // keyboard input to delete selection or character left of cursor
pub const STB_TEXTEDIT_K_UNDO: u32 =         0x20000A; // keyboard input to perform undo

    pub const STB_TEXTEDIT_K_REDO: u32 =          0x20000B; // keyboard input to perform redo
pub const STB_TEXTEDIT_K_WORDLEFT: u32 =     0x20000C; // keyboard input to move cursor left one word
pub const STB_TEXTEDIT_K_WORDRIGHT: u32 =    0x20000D; // keyboard input to move cursor right one word
pub const STB_TEXTEDIT_K_PGUP: u32 =         0x20000E; // keyboard input to move cursor up a page
pub const STB_TEXTEDIT_K_PGDOWN: u32 =       0x20000F; // keyboard input to move cursor down a page
pub const STB_TEXTEDIT_K_SHIFT: u32 =        0x400000;

// #define STB_TEXTEDIT_IMPLEMENTATION
// #include "imstb_textedit_h.rs"

// stb_textedit internally allows for a single undo record to do addition and deletion, but somehow, calling
// the stb_textedit_paste() function creates two separate records, so we perform it manually. (FIXME: Report to nothings/stb?)
// static void stb_textedit_replace(ImGuiInputTextState* in_str, STB_TexteditState* state, const STB_TEXTEDIT_CHARTYPE* text, int text_len)
    pub unsafe fn stb_textedit_replace(tis: *mut ImGuiInputTextState, state: *mut STB_TexteditState, text: *mut STB_TEXTEDIT_CHARTYPE, text_len: usize)
{
    stb_text_makeundo_replace(tis, state, 0, tis.CurLenW, text_len);
    STB_TEXTEDIT_DELETECHARS(tis, 0, tis.CurLenW);
    if (text_len <= 0) {
        return;
    }
    if STB_TEXTEDIT_INSERTCHARS(tis, 0, text, text_len)
    {
        state.cursor = text_len;
        state.has_preferred_x = false;
        return;
    }
    // IM_ASSERT(0); // Failed to insert character, normally shouldn't happen because of how we currently use stb_textedit_replace()
}


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
