// [DEAR IMGUI]
#![allow(non_camel_case_types)]
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
// Non-trivial behaviors are modelled after Windows text controls.
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
//   1.5  (2014-09-10) add support for secondary keys for OS X
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
//   To compile in this mode, you must define to: STB_TEXTEDIT_CHARTYPE a
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
//        [4 + 3 * sizeof] * STB_TEXTEDIT_UNDOSTATECOUNT
//      +          sizeof      * STB_TEXTEDIT_UNDOCHARCOUNT
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
// my Windows implementations add an additional CONTROL bit, and an additional KEYDOWN
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
//    void stb_textedit_initialize_state(state: &mut STB_TexteditState, int is_single_line)
//
//    void stb_textedit_click(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState, float x, float y)
//    void stb_textedit_drag(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState, float x, float y)
//    int  stb_textedit_cut(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState)
//    int  stb_textedit_paste(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState, text: &mut STB_TEXTEDIT_CHARTYPE, int len)
//    void stb_textedit_key(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState, STB_TEXEDIT_KEYTYPE key)
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
//          clear. defaults: STB_TEXTEDIT_KEYTYPE to int, but you can #define it to
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
// could define functions that return the X and Y positions of characters
// and binary search Y and then X, but if we're doing dynamic layout this
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
use libc::{c_char, c_float, c_int, c_short, c_uchar, size_t};
use crate::a_widgets::STB_TEXTEDIT_NEWLINE;
use crate::input_text_state::ImGuiInputTextState;
use crate::stb::stb_find_state::StbFindState;
use crate::stb::stb_text_edit_row::StbTexteditRow;
use crate::stb::stb_text_edit_state::STB_TexteditState;
use crate::stb::{STB_TEXTEDIT_DELETECHARS, STB_TEXTEDIT_GETCHAR, STB_TEXTEDIT_GETWIDTH, STB_TEXTEDIT_INSERTCHARS, STB_TEXTEDIT_LAYOUTROW, STB_TEXTEDIT_STRINGLEN};
use crate::stb::stb_undo_record::StbUndoRecord;
use crate::stb::stb_undo_state::StbUndoState;
use crate::stb_find_state::StbFindState;
use crate::stb_text_edit_state::STB_TexteditState;
use crate::stb_undo_record::StbUndoRecord;
use crate::stb_undo_state::StbUndoState;

// #ifndef STB_TEXTEDIT_UNDOSTATECOUNT
// #define STB_TEXTEDIT_UNDOSTATECOUNT   99
// #endif
pub const STB_TEXTEDIT_UNDOSTATECOUNT: usize = 99;

// #ifndef STB_TEXTEDIT_UNDOCHARCOUNT
// #define STB_TEXTEDIT_UNDOCHARCOUNT   999
// #endif
pub const STB_TEXTEDIT_UNDOCHARCOUNT: usize = 999;
// #ifndef STB_TEXTEDIT_CHARTYPE
// #define STB_TEXTEDIT_CHARTYPE        int
// #endif
pub type STB_TEXTEDIT_CHARTYPE = char;

// #ifndef STB_TEXTEDIT_POSITIONTYPE
// #define STB_TEXTEDIT_POSITIONTYPE    int
// #endif
pub type STB_TEXTEDIT_POSITIONTYPE  = c_int;


pub type STB_TEXTEDIT_STRING = String;

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


/////////////////////////////////////////////////////////////////////////////
//
//      Mouse input handling
//

// traverse the layout to locate the nearest character to a display position
pub unsafe fn stb_text_locate_coord(str_var: &mut ImGuiInputTextState, x: c_float, y: c_float) -> usize {
    let mut r = StbTexteditRow::default();
    let n = STB_TEXTEDIT_STRINGLEN(str_var);
    let mut base_y: c_float = 0.0;
    let mut prev_x: c_float = 0.0;
    let mut i: usize = 0;
    let mut k: usize = 0;

    r.x0 = 0.0;
    r.x1 = 0.0;
    r.ymin = 0;
    r.ymax = 0;
    r.num_chars = 0;

    // search rows to find one that straddles 'y'
    while i < n {
        STB_TEXTEDIT_LAYOUTROW(&mut r, str_var, i);
        if r.num_chars <= 0 {
            return n;
        }

        if i == 0 && y < base_y + r.ymin {
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
        // for (k=0; k < r.num_chars; ++k)
        for k in 0..r.num_chars {
            let w: c_float = STB_TEXTEDIT_GETWIDTH(str_var, i, k);
            if x < prev_x + w {
                if x < prev_x + w / 2 {
                    return k + i;
                } else {
                    return k + i1;
                }
            }
            prev_x += w;
        }
        // shouldn't happen, but if it does, fall through to end-of-line case
    }

    // if the last character is a newline, return that. otherwise return 'after' the last character
    return if STB_TEXTEDIT_GETCHAR(str_var, i + r.num_chars - 1) == STB_TEXTEDIT_NEWLINE {
        i + r.num_chars - 1
    } else {
        i + r.num_chars
    }
}

// API click: on mouse down, move the cursor to the clicked location, and reset the selection
pub unsafe fn stb_textedit_click(str_var: &mut ImGuiInputTextState,
                                 state: &mut STB_TexteditState,
                                 x: c_float,
                                 mut y: c_float) {
    // In single-line mode, just always make y = 0. This lets the drag keep working if the mouse
    // goes off the top or bottom of the text
    if state.single_line {
        let mut r = StbTexteditRow::default();
        STB_TEXTEDIT_LAYOUTROW(&mut r, str_var, 0);
        y = r.ymin;
    }

    state.cursor = stb_text_locate_coord(str_var, x, y);
    state.select_start = state.cursor;
    state.select_end = state.cursor;
    state.has_preferred_x = 0;
}

// API drag: on mouse drag, move the cursor and selection endpoint to the clicked location
pub unsafe fn stb_textedit_drag(str_var: &mut ImGuiInputTextState, state: &mut STB_TexteditState, x: c_float, mut y: c_float) {
    let mut p: c_int = 0;

    // In single-line mode, just always make y = 0. This lets the drag keep working if the mouse
    // goes off the top or bottom of the text
    if state.single_line {
        let mut r = StbTexteditRow::default();
        STB_TEXTEDIT_LAYOUTROW(&mut r, str_var, 0);
        y = r.ymin;
    }

    if state.select_start == state.select_end {
        state.select_start = state.cursor;
    }

    p = stb_text_locate_coord(str_var, x, y);
    state.cursor = p;
    state.select_end = p;
}

/////////////////////////////////////////////////////////////////////////////
//
//      Keyboard input handling
//



// find the x/y location of a character, and remember info about the previous row in
// case we get a move-up event (for page up, we'll have to rescan)
pub unsafe fn stb_textedit_find_charpos(find: &mut StbFindState,
                                 str_var: &mut ImGuiInputTextState,
                                 n: c_int,
                                 single_line: c_int) {
    let mut r = StbTexteditRow::default();
    let mut prev_start: c_int = 0;
    let z: c_int = STB_TEXTEDIT_STRINGLEN(str_var);
    let mut i: c_int = 0;
    let mut first = 0;

    if n == z {
        // if it's at the end, then find the last line -- simpler than trying to
        // explicitly handle this case in the regular code
        if single_line {
            STB_TEXTEDIT_LAYOUTROW(&mut r, str_var, 0);
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
                STB_TEXTEDIT_LAYOUTROW(&mut r, str_var, i);
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

    loop {
        STB_TEXTEDIT_LAYOUTROW(&mut r, str_var, i);
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
    let mut i = 0;
    while first + i < n {
        find.x += STB_TEXTEDIT_GETWIDTH(str_var, first, i);
        i += 1;
    }
}

// #define STB_TEXT_HAS_SELECTION(s)   ((s)->select_start != (s)->select_end)

// make the selection/cursor state valid if client altered the string
pub fn stb_textedit_clamp(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState) {
    let n: c_int = STB_TEXTEDIT_STRINGLEN(str_var);
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

// delete characters while updating undo
pub unsafe fn stb_textedit_delete(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState, stb_where_int: c_int, len: c_int)
{
   stb_text_makeundo_delete(str_var, state, stb_where_int, len);
   STB_TEXTEDIT_DELETECHARS(str_var, stb_where_int, len);
   state.has_preferred_x = 0;
}

// delete the section
pub unsafe fn stb_textedit_delete_selection(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState) {
    stb_textedit_clamp(str_var, state);
    if STB_TEXT_HAS_SELECTION(state) {
        if state.select_start < state.select_end {
            stb_textedit_delete(str_var, state, state.select_start, state.select_end - state.select_start);
            state.select_end = state.select_start;
            state.cursor = state.select_start;
        } else {
            stb_textedit_delete(str_var, state, state.select_end, state.select_start - state.select_end);
            state.select_start = state.select_end;
            state.cursor = state.select_end;
        }
        state.has_preferred_x = 0;
    }
}

// canoncialize the selection so start <= end
pub unsafe fn stb_textedit_sortselection(state: &mut STB_TexteditState) {
    if state.select_end < state.select_start {
        let temp: c_int = state.select_end;
        state.select_end = state.select_start;
        state.select_start = temp;
    }
}

// move cursor to first character of selection
pub unsafe fn stb_textedit_move_to_first(state: &mut STB_TexteditState) {
    if STB_TEXT_HAS_SELECTION(state) {
        stb_textedit_sortselection(state);
        state.cursor = state.select_start;
        state.select_end = state.select_start;
        state.has_preferred_x = 0;
    }
}

// move cursor to last character of selection
pub unsafe fn stb_textedit_move_to_last(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState) {
    if STB_TEXT_HAS_SELECTION(state) {
        stb_textedit_sortselection(state);
        stb_textedit_clamp(str_var, state);
        state.cursor = state.select_end;
        state.select_start = state.select_end;
        state.has_preferred_x = 0;
    }
}

// #ifdef STB_TEXTEDIT_IS_SPACE
pub fn is_word_boundary(str_var: &mut STB_TEXTEDIT_STRING, idx: c_int) -> bool {
    return if idx > 0 {
        STB_TEXTEDIT_IS_SPACE(STB_TEXTEDIT_GETCHAR(str_var, idx - 1))
        && !STB_TEXTEDIT_IS_SPACE(STB_TEXTEDIT_GETCHAR(str_var, idx)) }
    else {
        true
    };
}

// #ifndef STB_TEXTEDIT_MOVEWORDLEFT
pub fn stb_textedit_move_to_word_previous(str_var: &mut STB_TEXTEDIT_STRING, mut c: c_int) -> c_int {
    c -= 1; // always move at least one character
    while c >= 0 && !is_word_boundary(str_var, c) {
        c -= 1;
    }

    if c < 0 {
        c = 0;
    }

    return c;
}
// #define STB_TEXTEDIT_MOVEWORDLEFT stb_textedit_move_to_word_previous
// #endif

// #ifndef STB_TEXTEDIT_MOVEWORDRIGHT
pub fn stb_textedit_move_to_word_next( str_var: &mut STB_TEXTEDIT_STRING, mut c: c_int ) -> c_int
{
   let len: c_int = STB_TEXTEDIT_STRINGLEN(str_var);
   c += 1; // always move at least one character
   while c < len && !is_word_boundary(str_var, c ) {
       c += 1;
   }

   if c > len {
       c = len;
   }

   return c;
}
// #define STB_TEXTEDIT_MOVEWORDRIGHT stb_textedit_move_to_word_next
// #endif

// #endif

// update selection and cursor to match each other
pub unsafe fn stb_textedit_prep_selection_at_cursor(state: &mut STB_TexteditState) {
    if !STB_TEXT_HAS_SELECTION(state) {
        state.select_start = state.cursor;
        state.select_end = state.cursor;
    } else {
        state.cursor = state.select_end;
    }
}

// API cut: delete selection
pub unsafe fn stb_textedit_cut(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState) -> bool
{
   if STB_TEXT_HAS_SELECTION(state) {
      stb_textedit_delete_selection(str_var,state); // implicitly clamps
      state.has_preferred_x = 0;
      return true;
   }
   return false;
}

// API paste: replace existing selection with passed-in text
pub unsafe fn stb_textedit_paste_internal(str_var: &mut STB_TEXTEDIT_STRING,
                                   state: &mut STB_TexteditState,
                                   text: &mut STB_TEXTEDIT_CHARTYPE,
                                   len: c_int) -> bool
{
   // if there's a selection, the paste should delete it
   stb_textedit_clamp(str_var, state);
   stb_textedit_delete_selection(str_var,state);
   // try to insert the characters
   if STB_TEXTEDIT_INSERTCHARS(str_var, state.cursor, text, len) {
      stb_text_makeundo_insert(state, state.cursor, len);
      state.cursor += len;
      state.has_preferred_x = 0;
      return true;
   }
   // note: paste failure will leave deleted selection, may be restored with an undo (see https://github.com/nothings/stb/issues/734 for details)
   return false;
}

// #ifndef STB_TEXTEDIT_KEYTYPE
// #define int: STB_TEXTEDIT_KEYTYPE
// #endif

// API key: process a keyboard input
pub unsafe fn stb_textedit_key(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState, key: STB_TEXTEDIT_KEYTYPE)
{
// retry:
   match key {

// #ifdef STB_TEXTEDIT_K_INSERT
      STB_TEXTEDIT_K_INSERT => { state.insert_mode = !state.insert_mode },
         // break;
// #endif

      STB_TEXTEDIT_K_UNDO => {
          stb_text_undo(str_var, state);
          state.has_preferred_x = 0;
      },
         // break;

      STB_TEXTEDIT_K_REDO => {
          stb_text_redo(str_var, state);
          state.has_preferred_x = 0;
      },
         // break;

      STB_TEXTEDIT_K_LEFT => {
          // if currently there's a selection, move cursor to start of selection
          if STB_TEXT_HAS_SELECTION(state) {
              stb_textedit_move_to_first(state);
          } else {
              if state.cursor > 0 {
                  state.cursor -= 1;
              }
          }
          state.has_preferred_x = 0;
      },
         // break;

      STB_TEXTEDIT_K_RIGHT => {
          // if currently there's a selection, move cursor to end of selection
          if STB_TEXT_HAS_SELECTION(state) {
              stb_textedit_move_to_last(str_var, state);
          } else {
              state.cursor += 1;
          }
          stb_textedit_clamp(str_var, state);
          state.has_preferred_x = 0;
      },
         // break;

      STB_TEXTEDIT_K_LEFT | STB_TEXTEDIT_K_SHIFT => {
          stb_textedit_clamp(str_var, state);
          stb_textedit_prep_selection_at_cursor(state);
          // move selection left
          if state.select_end > 0 { state.select_end -= 1; }
          state.cursor = state.select_end;
          state.has_preferred_x = 0;
      },
         // break;

// #ifdef STB_TEXTEDIT_MOVEWORDLEFT
      STB_TEXTEDIT_K_WORDLEFT => {
          if STB_TEXT_HAS_SELECTION(state) {
              stb_textedit_move_to_first(state);
          } else {
              state.cursor = STB_TEXTEDIT_MOVEWORDLEFT(str_var, state.cursor);
              stb_textedit_clamp(str_var, state);
          }
      },
         // break;

      STB_TEXTEDIT_K_WORDLEFT | STB_TEXTEDIT_K_SHIFT => {
          if ! STB_TEXT_HAS_SELECTION( state ) {
              stb_textedit_prep_selection_at_cursor(state);
          }

          state.cursor = STB_TEXTEDIT_MOVEWORDLEFT(str_var, state.cursor); state.select_end = state.cursor;

          stb_textedit_clamp( str_var, state ); },

// #endif

// #ifdef STB_TEXTEDIT_MOVEWORDRIGHT
       STB_TEXTEDIT_K_WORDRIGHT => {
           if (STB_TEXT_HAS_SELECTION(state)) {
               stb_textedit_move_to_last(str_var, state);
           } else {
               state.cursor = STB_TEXTEDIT_MOVEWORDRIGHT(str_var, state.cursor);
               stb_textedit_clamp(str_var, state);
           }
       },
         // break;

      STB_TEXTEDIT_K_WORDRIGHT | STB_TEXTEDIT_K_SHIFT => {
          if (!STB_TEXT_HAS_SELECTION(state)) {
              stb_textedit_prep_selection_at_cursor(state);
          }

          state.cursor = STB_TEXTEDIT_MOVEWORDRIGHT(str_var, state.cursor);
          state.select_end = state.cursor;

          stb_textedit_clamp(str_var, state);
      },
         // break;
// #endif

      STB_TEXTEDIT_K_RIGHT | STB_TEXTEDIT_K_SHIFT => {
          stb_textedit_prep_selection_at_cursor(state);
          // move selection right
          state.select_end += 1;
          stb_textedit_clamp(str_var, state);
          state.cursor = state.select_end;
          state.has_preferred_x = 0;
      },


      STB_TEXTEDIT_K_DOWN | STB_TEXTEDIT_K_DOWN | STB_TEXTEDIT_K_SHIFT | STB_TEXTEDIT_K_PGDOWN | STB_TEXTEDIT_K_PGDOWN | STB_TEXTEDIT_K_SHIFT => {
         // let mut find = StbFindState::default();
         let mut find = StbFindState::default();
          let mut row = StbTexteditRow::default();
         let mut i: c_int = 0;
          let mut j: c_int = 0;
          let sel = (key & STB_TEXTEDIT_K_SHIFT) != 0;
         let is_page: bool = (key & !STB_TEXTEDIT_K_SHIFT) == STB_TEXTEDIT_K_PGDOWN;
         let row_count: c_int = if is_page { state.row_count_per_page } else { 1 };

         if !is_page && state.single_line != 0 {
            // on windows, up&down in single-line behave like left&right
            key = STB_TEXTEDIT_K_RIGHT | (key & STB_TEXTEDIT_K_SHIFT);
            // TODO replace with recursive function call
             // goto retry;
         }

         if sel {
             stb_textedit_prep_selection_at_cursor(state);
         }
         else if STB_TEXT_HAS_SELECTION(state) {
            stb_textedit_move_to_last(str_var, state);
        }

         // compute current position of cursor point
         stb_textedit_clamp(str_var, state);
         stb_textedit_find_charpos(&mut find, str_var, state.cursor, state.single_line as c_int);

         // for (j = 0; j < row_count; ++j)
         for j in 0 .. row_count
          {
            let mut x: c_float = 0.0;
              let mut goal_x = if state.has_preferred_x { state.preferred_x } else { find.x };
            let start: c_int = find.first_char + find.length;

            if find.length == 0 {
                break;
            }

            // [DEAR IMGUI]
            // going down while being on the last line shouldn't bring us to that line end
            if STB_TEXTEDIT_GETCHAR(str_var, find.first_char + find.length - 1) != STB_TEXTEDIT_NEWLINE {
                break;
            }

            // now find character position down a row
            state.cursor = start;
            STB_TEXTEDIT_LAYOUTROW(&row, str_var, state.cursor);
            x = row.x0;
            // for (i=0; i < row.num_chars; ++i)
            for i in 0 .. row.num_chars
              {
               let dx: c_float =  STB_TEXTEDIT_GETWIDTH(str_var, start, i);
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
            stb_textedit_clamp(str_var, state);

            state.has_preferred_x = 1;
            state.preferred_x = goal_x;

            if sel {
                state.select_end = state.cursor;
            }

            // go to next line
            find.first_char = find.first_char + find.length;
            find.length = row.num_chars;
         }
         // break;
      },

      STB_TEXTEDIT_K_UP | STB_TEXTEDIT_K_UP | STB_TEXTEDIT_K_SHIFT | STB_TEXTEDIT_K_PGUP | STB_TEXTEDIT_K_PGUP | STB_TEXTEDIT_K_SHIFT => {
         let mut find = StbFindState::default();
         let mut row = StbTexteditRow::default();
         // i: c_int, j, prev_scan,
        let mut i: c_int = 0;
          let mut j: c_int = 0;
          let mut prev_scan: c_int = 0;
          let sel = (key & STB_TEXTEDIT_K_SHIFT) != 0;
         let is_page: bool = (key & !STB_TEXTEDIT_K_SHIFT) == STB_TEXTEDIT_K_PGUP;
         let row_count: c_int = if is_page { state.row_count_per_page } else { 1 };

         if (!is_page && state.single_line != 0) {
            // on windows, up&down become left&right
            key = STB_TEXTEDIT_K_LEFT | (key & STB_TEXTEDIT_K_SHIFT);
            // TODO: recursive function call
             // goto retry;
         }

         if sel {
             stb_textedit_prep_selection_at_cursor(state);
         }
         else if STB_TEXT_HAS_SELECTION(state) {
             stb_textedit_move_to_first(state);
         }

         // compute current position of cursor point
         stb_textedit_clamp(str_var, state);
         stb_textedit_find_charpos(&mut find, str_var, state.cursor, state.single_line as c_int);

         // for (j = 0; j < row_count; ++j)
         for j in 0.. row_count
          {

             let mut x: c_float = 0.0;
              let goal_x = if state.has_preferred_x { state.preferred_x } else { find.x };

            // can only go up if there's a previous row
            if find.prev_first == find.first_char {
                break;
            }

            // now find character position up a row
            state.cursor = find.prev_first;
            STB_TEXTEDIT_LAYOUTROW(&row, str_var, state.cursor);
            x = row.x0;
            // for (i=0; i < row.num_chars; ++i)
            for i in 0 .. row.num_chars
              {
               let dx: c_float =  STB_TEXTEDIT_GETWIDTH(str_var, find.prev_first, i);
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
            stb_textedit_clamp(str_var, state);

            state.has_preferred_x = 1;
            state.preferred_x = goal_x;

            if (sel) {
                state.select_end = state.cursor;
            }

            // go to previous line
            // (we need to scan previous line the hard way. maybe we could expose this as a new API function?)
            prev_scan = if find.prev_first > 0 { find.prev_first - 1 } else { 0 };
            while prev_scan > 0 && STB_TEXTEDIT_GETCHAR(str_var, prev_scan - 1) != STB_TEXTEDIT_NEWLINE {
                prev_scan += 1;
            }
            find.first_char = find.prev_first;
            find.prev_first = prev_scan;
         }
         // break;
      },

      STB_TEXTEDIT_K_DELETE | STB_TEXTEDIT_K_DELETE | STB_TEXTEDIT_K_SHIFT => {
          if STB_TEXT_HAS_SELECTION(state) {
          stb_textedit_delete_selection(str_var, state);
          }
          else {
          let n: c_int = STB_TEXTEDIT_STRINGLEN(str_var);
          if state.cursor < n {
              stb_textedit_delete(str_var, state, state.cursor, 1);
          }
          }
          state.has_preferred_x = 0;  },

      STB_TEXTEDIT_K_BACKSPACE | STB_TEXTEDIT_K_BACKSPACE | STB_TEXTEDIT_K_SHIFT => {
          if STB_TEXT_HAS_SELECTION(state) {
              stb_textedit_delete_selection(str_var, state);
          } else {
              stb_textedit_clamp(str_var, state);
              if state.cursor > 0 {
                  stb_textedit_delete(str_var, state, state.cursor - 1, 1);
                  state.cursor += 1;
              }
          }
          state.has_preferred_x = 0;
      },

// #ifdef STB_TEXTEDIT_K_TEXTSTART2
  STB_TEXTEDIT_K_TEXTSTART2 |
// #endif
       STB_TEXTEDIT_K_TEXTSTART => {
      state.cursor = 0;
      state.select_start = 0;
      state.select_end = 0;
      state.has_preferred_x = 0;
  },
         // break;

// #ifdef STB_TEXTEDIT_K_TEXTEND2
      STB_TEXTEDIT_K_TEXTEND2 |
// #endif
      STB_TEXTEDIT_K_TEXTEND => {
          state.cursor = STB_TEXTEDIT_STRINGLEN(str_var);
          state.select_start = 0;
          state.select_end = 0;
          state.has_preferred_x = 0;
      },
         // break;

// #ifdef STB_TEXTEDIT_K_TEXTSTART2
      STB_TEXTEDIT_K_TEXTSTART2 | STB_TEXTEDIT_K_SHIFT |
// #endif
      STB_TEXTEDIT_K_TEXTSTART | STB_TEXTEDIT_K_SHIFT => {
          stb_textedit_prep_selection_at_cursor(state);
          state.cursor = 0;
          state.select_end = 0;
          state.has_preferred_x = 0;
      },
         // break;

// #ifdef STB_TEXTEDIT_K_TEXTEND2
      STB_TEXTEDIT_K_TEXTEND2 | STB_TEXTEDIT_K_SHIFT |
// #endif
      STB_TEXTEDIT_K_TEXTEND | STB_TEXTEDIT_K_SHIFT => {
          stb_textedit_prep_selection_at_cursor(state);
          state.cursor = STB_TEXTEDIT_STRINGLEN(str_var);;
          state.select_end = STB_TEXTEDIT_STRINGLEN(str_var);
          state.has_preferred_x = 0;
      },
         // break;


// #ifdef STB_TEXTEDIT_K_LINESTART2
      STB_TEXTEDIT_K_LINESTART2 |
// #endif
      STB_TEXTEDIT_K_LINESTART => {
          stb_textedit_clamp(str_var, state);
          stb_textedit_move_to_first(state);
          if state.single_line {
              state.cursor = 0;
          }
          else {
              while state.cursor > 0 && STB_TEXTEDIT_GETCHAR(str_var, state.cursor - 1) != STB_TEXTEDIT_NEWLINE {
                  state.cursor -= 1;
              }
          }
          state.has_preferred_x = 0;
      },
         // break;

// #ifdef STB_TEXTEDIT_K_LINEEND2
      STB_TEXTEDIT_K_LINEEND2 |
// #endif
       STB_TEXTEDIT_K_LINEEND => {
         let n: c_int = STB_TEXTEDIT_STRINGLEN(str_var);
         stb_textedit_clamp(str_var, state);
         stb_textedit_move_to_first(state);
         if state.single_line {
             state.cursor = n;
         }
         else {
            while state.cursor < n && STB_TEXTEDIT_GETCHAR(str_var, state.cursor) != STB_TEXTEDIT_NEWLINE { state.cursor += 1; }
        }
         state.has_preferred_x = 0;
         // break;
      },

// #ifdef STB_TEXTEDIT_K_LINESTART2
      STB_TEXTEDIT_K_LINESTART2 | STB_TEXTEDIT_K_SHIFT |
// #endif
      STB_TEXTEDIT_K_LINESTART | STB_TEXTEDIT_K_SHIFT => {
          stb_textedit_clamp(str_var, state);
          stb_textedit_prep_selection_at_cursor(state);
          if state.single_line {
              state.cursor = 0;
          }
          else {
              while state.cursor > 0 && STB_TEXTEDIT_GETCHAR(str_var, state.cursor - 1) != STB_TEXTEDIT_NEWLINE {
                  state.cursor -= 1;
              }
          }
          state.select_end = state.cursor;
          state.has_preferred_x = 0;
      },
         // break;

// #ifdef STB_TEXTEDIT_K_LINEEND2
      STB_TEXTEDIT_K_LINEEND2 | STB_TEXTEDIT_K_SHIFT |
// #endif
      STB_TEXTEDIT_K_LINEEND | STB_TEXTEDIT_K_SHIFT => {
         let n: c_int = STB_TEXTEDIT_STRINGLEN(str_var);
         stb_textedit_clamp(str_var, state);
         stb_textedit_prep_selection_at_cursor(state);
         if state.single_line {
             state.cursor = n;
         }
         else {
            while state.cursor < n && STB_TEXTEDIT_GETCHAR(str_var, state.cursor) != STB_TEXTEDIT_NEWLINE {
                state.cursor += 1;
            }
        }
         state.select_end = state.cursor;
         state.has_preferred_x = 0;
         // break;
      },
       _ => {
         let c: c_int = STB_TEXTEDIT_KEYTOTEXT(key);
         if c > 0 {
            let ch: STB_TEXTEDIT_CHARTYPE =  c;

            // can't add newline in single-line mode
            if c == '\n' as c_int && state.single_line != 0 {
                // break;
            } else {
            if state.insert_mode && !STB_TEXT_HAS_SELECTION(state) && state.cursor < STB_TEXTEDIT_STRINGLEN(str_var) {
               stb_text_makeundo_replace(str_var, state, state.cursor, 1, 1);
               STB_TEXTEDIT_DELETECHARS(str_var, state.cursor, 1);
               if STB_TEXTEDIT_INSERTCHARS(str_var, state.cursor, &ch, 1) {
                  state.cursor += 1;
                  state.has_preferred_x = 0;
               }
            } else {
               stb_textedit_delete_selection(str_var,state); // implicitly clamps
               if STB_TEXTEDIT_INSERTCHARS(str_var, state.cursor, &ch, 1) {
                  stb_text_makeundo_insert(state, state.cursor, 1);
                  state.cursor += 1;
                  state.has_preferred_x = 0;
               }
            }}
         }
         // break;
      }
   }
}

/////////////////////////////////////////////////////////////////////////////
//
//      Undo processing
//
// @OPTIMIZE: the undo/redo buffer should be circular

pub fn stb_textedit_flush_redo(state: *mut StbUndoState)
{
   state.redo_point = STB_TEXTEDIT_UNDOSTATECOUNT as c_short;
   state.redo_char_point = STB_TEXTEDIT_UNDOCHARCOUNT as c_int;
}

// discard the oldest entry in the undo list
pub unsafe fn stb_textedit_discard_undo(state: *mut StbUndoState)
{
   if state.undo_point > 0 {
      // if the 0th undo state has characters, clean those up
      if state.undo_rec[0].char_storage >= 0 {
         let n: c_int = state.undo_rec[0].insert_length;
          let mut i = 0;
         // delete n characters from all other records
         state.undo_char_point -= n;
         STB_TEXTEDIT_memmove(state.undo_char, state.undo_char + n,  state.undo_char_point * sizeof);
         // for (i=0; i < state.undo_point; ++i)
         for i in 0 .. state.undo_point
          {
             if state.undo_rec[i].char_storage >= 0 {
                 state.undo_rec[i].char_storage -= n;
             }
         } // @OPTIMIZE: get rid of char_storage and infer it
      }
      state.undo_point -= 1;
      STB_TEXTEDIT_memmove(&state.undo_rec, state.undo_rec1,  state.undo_point * sizeof(&state.undo_rec[0]));
   }
}

// discard the oldest entry in the redo list--it's bad if this
// ever happens, but because undo & redo have to store the actual
// characters in different cases, the redo character buffer can
// fill up even though the undo buffer didn't
pub unsafe fn stb_textedit_discard_redo(state: *mut StbUndoState)
{
   let k: c_int = (STB_TEXTEDIT_UNDOSTATECOUNT - 1) as c_int;

   if state.redo_point <= k as c_short {
      // if the k'th undo state has characters, clean those up
      if state.undo_rec[k].char_storage >= 0 {
         let n: c_int = state.undo_rec[k].insert_length;
          let mut i = 0;
         // move the remaining redo character data to the end of the buffer
         state.redo_char_point += n;
         STB_TEXTEDIT_memmove(state.undo_char + state.redo_char_point, state.undo_char + state.redo_char_point-n,  ((STB_TEXTEDIT_UNDOCHARCOUNT - state.redo_char_point)*sizeof));
         // adjust the position of all the other records to account for above memmove
         // for (i=state.redo_point; i < k; ++i)
         for i in state.redo_point .. k
          {
             if state.undo_rec[i].char_storage >= 0 {
                 state.undo_rec[i].char_storage += n;
             }
         }
      }
      // now move all the redo records towards the end of the buffer; the first one is at 'redo_point'
      // [DEAR IMGUI]
      let move_size: size_t = ((STB_TEXTEDIT_UNDOSTATECOUNT - state.redo_point - 1) * sizeof(&state.undo_rec[0]));
      // let mut buf_begin: *const c_char = state.undo_rec.as_ptr();
       // buf_begin;
      // let mut  buf_end: *const c_char = state.undo_rec + sizeof(state.undo_rec);
       // buf_end;
      // IM_ASSERT(((state->undo_rec + state->redo_point)) >= buf_begin);
      // IM_ASSERT(((state->undo_rec + state->redo_point + 1) + move_size) <= buf_end);
      STB_TEXTEDIT_memmove(&state.undo_rec + state.redo_point1, &state.undo_rec + state.redo_point, move_size);

      // now move redo_point to point to the new one
      state.redo_point += 1;
   }
}

pub unsafe fn stb_text_create_undo_record(state: *mut StbUndoState, numchars: usize) -> *mut StbUndoRecord {
    // any time we create a new undo record, we discard redo
    stb_textedit_flush_redo(state);

    // if we have no free records, we have to make room, by sliding the
    // existing records down
    if state.undo_point == STB_TEXTEDIT_UNDOSTATECOUNT as c_short {
        stb_textedit_discard_undo(state);
    }

    // if the characters to store won't possibly fit in the buffer, we can't undo
    if numchars > STB_TEXTEDIT_UNDOCHARCOUNT {
        state.undo_point = 0;
        state.undo_char_point = 0;
        return null_mut();
    }

    // if we don't have enough free characters in the buffer, we have to make room
    while state.undo_char_point + numchars > STB_TEXTEDIT_UNDOCHARCOUNT as c_int {
        stb_textedit_discard_undo(state);
    }

    let mut out = &mut state.undo_rec[state.undo_point];
    ;
    state.undo_point += 1;
    return out;
}

pub unsafe fn stb_text_createundo(state: *mut StbUndoState, pos: usize, insert_len: usize, delete_len: usize) -> *mut STB_TEXTEDIT_CHARTYPE {
    let mut r: *mut StbUndoRecord = stb_text_create_undo_record(state, insert_len);
    if r == null_mut() {
        return null_mut();
    }

    r.stb_where = pos;
    r.insert_length = insert_len;
    r.delete_length = delete_len;

    return if insert_len == 0 {
        r.char_storage = -1;
        null_mut()
    } else {
        r.char_storage = state.undo_char_point;
        state.undo_char_point += insert_len;
        &mut state.undo_char[r.char_storage]
    };
}

pub unsafe fn stb_text_undo(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState) {
    let s: *mut StbUndoState = &mut state.undostate;
    // StbUndoRecord u, *r;
    let mut u: StbUndoRecord = StbUndoRecord::defualt();
    let mut r: *mut StbUndoRecord = null_mut();
    if s.undo_point == 0 {
        return;
    }

    // we need to do two things: apply the undo record, and create a redo record
    u = s.undo_rec[s.undo_point - 1];
    r = &mut s.undo_rec[s.redo_point - 1];
    r.char_storage = -1;

    r.insert_length = u.delete_length;
    r.delete_length = u.insert_length;
    r.stb_where = u.stb_where;

    if u.delete_length {
        // if the undo record says to delete characters, then the redo record will
        // need to re-insert the characters that get deleted, so we need to store
        // them.

        // there are three cases:
        //    there's enough room to store the characters
        //    characters stored for *redoing* don't leave room for redo
        //    characters stored for *undoing* don't leave room for redo
        // if the last is true, we have to bail

        if s.undo_char_point + u.delete_length >= STB_TEXTEDIT_UNDOCHARCOUNT as c_int {
            // the undo records take up too much character space; there's no space to store the redo characters
            r.insert_length = 0;
        } else {
            let mut i: c_int = 0;

            // there's definitely room to store the characters eventually
            while s.undo_char_point + u.delete_length > s.redo_char_point {
                // should never happen:
                if s.redo_point == STB_TEXTEDIT_UNDOSTATECOUNT as c_short {
                    return;
                }
                // there's currently not enough room, so discard a redo record
                stb_textedit_discard_redo(s);
            }
            r = &mut s.undo_rec[s.redo_point - 1];

            r.char_storage = s.redo_char_point - u.delete_length;
            s.redo_char_point = s.redo_char_point - u.delete_length;

            // now save the characters
            // for (i=0; i < u.delete_length; ++i)
            for i in 0..u.delete_length {
                s.undo_char[r.char_storage + i] = STB_TEXTEDIT_GETCHAR(str_var, u.stb_where + i);
            }
        }

        // now we can carry out the deletion
        STB_TEXTEDIT_DELETECHARS(str_var, u.stb_where, u.delete_length);
    }

    // check type of recorded action:
    if u.insert_length {
        // easy case: was a deletion, so we need to insert n characters
        STB_TEXTEDIT_INSERTCHARS(str_var, u.stb_where, &s.undo_char[u.char_storage], u.insert_length);
        s.undo_char_point -= u.insert_length;
    }

    state.cursor = u.stb_where + u.insert_length;

    s.undo_point -= 1;
    s.redo_point -= 1;
}

pub unsafe fn stb_text_redo(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState) {
    let mut s: *mut StbUndoState = &mut state.undostate;
    let mut u: *mut StbUndoRecord = null_mut();
    let mut r: *mut StbUndoRecord = null_mut();
    if s.redo_point == STB_TEXTEDIT_UNDOSTATECOUNT as c_short {
        return;
    }

    // we need to do two things: apply the redo record, and create an undo record
    u = &mut s.undo_rec[s.undo_point];
    r = s.undo_rec[s.redo_point];

    // we KNOW there must be room for the undo record, because the redo record
    // was derived from an undo record

    u.delete_length = r.insert_length;
    u.insert_length = r.delete_length;
    u.stb_where = r.stb_where;
    u.char_storage = -1;

    if r.delete_length {
        // the redo record requires us to delete characters, so the undo record
        // needs to store the characters

        if s.undo_char_point + u.insert_length > s.redo_char_point {
            u.insert_length = 0;
            u.delete_length = 0;
        } else {
            let mut i: c_int = 0;
            u.char_storage = s.undo_char_point;
            s.undo_char_point = s.undo_char_point + u.insert_length;

            // now save the characters
            // for (i=0; i < u.insert_length; ++i)
            for i in 0..u.insert_length {
                s.undo_char[u.char_storage + i] = STB_TEXTEDIT_GETCHAR(str_var, u.stb_where + i);
            }
        }

        STB_TEXTEDIT_DELETECHARS(str_var, r.stb_where, r.delete_length);
    }

    if r.insert_length {
        // easy case: need to insert n characters
        STB_TEXTEDIT_INSERTCHARS(str_var, r.stb_where, &s.undo_char[r.char_storage], r.insert_length);
        s.redo_char_point += r.insert_length;
    }

    state.cursor = r.stb_where + r.insert_length;

    s.undo_point += 1;
    s.redo_point += 1;
}

pub unsafe fn stb_text_makeundo_insert(state: &mut STB_TexteditState, stb_where: c_int, length: c_int)
{
   stb_text_createundo(&mut state.undostate, stb_where, 0, length);
}

pub unsafe fn stb_text_makeundo_delete(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState, stb_where: c_int, length: c_int)
{
   let mut i: c_int = 0;
   p: &mut STB_TEXTEDIT_CHARTYPE = stb_text_createundo(&mut state.undostate, stb_where, length, 0);
   if p {
      // for (i=0; i < length; ++i)
      for i in 0 .. length
       {
          p[i] = STB_TEXTEDIT_GETCHAR(str_var, stb_where + i);
      }
   }
}

pub unsafe fn stb_text_makeundo_replace(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState, stb_where: usize, old_length: usize, new_length: usize)
{
   let mut i: c_int = 0;
   p: &mut STB_TEXTEDIT_CHARTYPE = stb_text_createundo(&mut state.undostate, stb_where, old_length, new_length);
   if p {
      // for (i=0; i < old_length; ++i)
      for i in 0 .. old_length
       {
          p[i] = STB_TEXTEDIT_GETCHAR(str_var, stb_where + i);
      }
   }
}

// reset the state to default
pub unsafe fn stb_textedit_clear_state(state: &mut STB_TexteditState, is_single_line: bool) {
    state.undostate.undo_point = 0;
    state.undostate.undo_char_point = 0;
    state.undostate.redo_point = STB_TEXTEDIT_UNDOSTATECOUNT as c_short;
    state.undostate.redo_char_point = STB_TEXTEDIT_UNDOCHARCOUNT as c_int;
    state.select_end = 0;
    state.select_start = 0;
    state.cursor = 0;
    state.has_preferred_x = 0;
    state.preferred_x = 0.0;
    state.cursor_at_end_of_line = 0;
    state.initialized = 1;
    state.single_line = is_single_line as c_uchar;
    state.insert_mode = 0;
    state.row_count_per_page = 0;
}

// API initialize
pub unsafe fn stb_textedit_initialize_state(state: &mut STB_TexteditState, is_single_line: bool)
{
   stb_textedit_clear_state(state, is_single_line);
}

// #if defined(__GNUC__) || defined(__clang__)
// #pragma GCC diagnostic push
// #pragma GCC diagnostic ignored "-Wcast-qual"
// #endif

pub unsafe fn stb_textedit_paste(str_var: &mut STB_TEXTEDIT_STRING, state: &mut STB_TexteditState, ctext: &mut STB_TEXTEDIT_CHARTYPE, len: c_int) -> bool
{
   return stb_textedit_paste_internal(str_var, state, ctext, len);
}

// #if defined(__GNUC__) || defined(__clang__)
// #pragma GCC diagnostic pop
// #endif

// #endif//STB_TEXTEDIT_IMPLEMENTATION

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
