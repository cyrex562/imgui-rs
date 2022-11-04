use crate::input_text_flags::ImGuiInputTextFlags_CallbackResize;
use crate::input_text_state::ImGuiInputTextState;
use crate::io_ops::GetIO;
use crate::math_ops::{ImClamp, ImMax};
use crate::stb::stb_text_edit_row::StbTexteditRow;
use crate::stb::stb_text_edit_state::STB_TexteditState;
use crate::stb::stb_textedit::{
    stb_text_makeundo_replace, STB_TEXTEDIT_CHARTYPE, STB_TEXTEDIT_STRING,
};
use crate::string_ops::ImTextCountUtf8BytesFromStr;
use crate::type_defs::ImWchar;
use crate::utils::flag_set;
use crate::vec2::ImVec2;
use crate::{input_text, widgets, GImGui};
use libc::{c_float, c_int};

pub mod stb_find_state;
pub mod stb_ops;
pub mod stb_rp_context;
pub mod stb_text_edit_row;
pub mod stb_text_edit_state;
pub mod stb_textedit;
pub mod stb_truetype;
pub mod stb_tt_active_edge;
pub mod stb_tt_aligned_quad;
pub mod stb_tt_baked_char;
pub mod stb_tt_bitmap;
pub mod stb_tt_buf;
pub mod stb_tt_csctx;
pub mod stb_tt_edge;
pub mod stb_tt_encoding_id;
pub mod stb_tt_fontinfo;
pub mod stb_tt_hheap;
pub mod stb_tt_hheap_chunk;
pub mod stb_tt_kerning_entry;
pub mod stb_tt_lang_id;
pub mod stb_tt_packed_char;
pub mod stb_tt_packed_context;
pub mod stb_tt_packed_range;
pub mod stb_tt_platform_id;
pub mod stb_tt_point;
pub mod stb_tt_shapes;
pub mod stb_tt_types;
pub mod stb_tt_vertex;
pub mod stb_undo_record;
pub mod stb_undo_state;

pub fn STB_TEXTEDIT_STRINGLEN(obj: &ImGuiInputTextState) -> usize {
    return obj.CurLenW;
}

pub fn STB_TEXTEDIT_GETCHAR(obj: &ImGuiInputTextState, idx: usize) -> char {
    return obj.TextW[idx];
}

pub unsafe fn STB_TEXTEDIT_GETWIDTH(
    obj: &mut ImGuiInputTextState,
    line_start_idx: c_int,
    char_idx: c_int,
) -> c_float {
    let c = obj.TextW[line_start_idx + char_idx];
    if c == '\n' {
        return STB_TEXTEDIT_GETWIDTH_NEWLINE;
    }
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Font.GetCharAdvance(c) * (g.FontSize / g.Font.FontSize);
}

pub fn STB_TEXTEDIT_KEYTOTEXT(key: c_int) -> c_int {
    return if key >= 0x200000 { 0 } else { key };
}

pub unsafe fn STB_TEXTEDIT_LAYOUTROW(
    r: &mut StbTexteditRow,
    obj: &mut ImGuiInputTextState,
    line_start_idx: usize,
) {
    let text: *const ImWchar = obj.TextW.Data;
    let mut text_remaining: usize = 0;
    let size: ImVec2 =
        input_text::InputTextCalcTextSizeW(text + line_start_idx, &mut text_remaining, None, true);
    r.x0 = 0.0;
    r.x1 = size.x;
    r.baseline_y_delta = size.y;
    r.ymin = 0.0;
    r.ymax = size.y;
    r.num_chars = (text_remaining - (text + line_start_idx));
}

pub unsafe fn STB_TEXTEDIT_MOVEWORDLEFT_IMPL(
    obj: &mut ImGuiInputTextState,
    mut idx: usize,
) -> c_int {
    idx -= 1;
    while idx >= 0 && input_text::is_word_boundary_from_right(obj, idx) == false {
        idx -= 1;
    }
    return if idx < 0 { 0 } else { idx };
}

pub unsafe fn STB_TEXTEDIT_MOVEWORDRIGHT_MAC(
    obj: &mut ImGuiInputTextState,
    mut idx: usize,
) -> usize {
    idx += 1;
    let len = obj.CurLenW;
    while idx < len && input_text::is_word_boundary_from_left(obj, idx) == false {
        idx += 1;
    }
    return if idx > len { len } else { idx };
}

pub unsafe fn STB_TEXTEDIT_MOVEWORDRIGHT_WIN(
    obj: &mut ImGuiInputTextState,
    mut idx: usize,
) -> usize {
    idx += 1;
    let len = obj.CurLenW;
    while idx < len && input_text::is_word_boundary_from_right(obj, idx) == false {
        idx += 1;
    }
    return if idx > len { len } else { idx };
}

pub unsafe fn STB_TEXTEDIT_MOVEWORDRIGHT_IMPL(
    obj: &mut ImGuiInputTextState,
    mut idx: usize,
) -> usize {
    return if GetIO().ConfigMacOSXBehaviors {
        STB_TEXTEDIT_MOVEWORDRIGHT_MAC(obj, idx)
    } else {
        STB_TEXTEDIT_MOVEWORDRIGHT_WIN(obj, idx)
    };
}

pub unsafe fn STB_TEXTEDIT_DELETECHARS(obj: &mut ImGuiInputTextState, pos: c_int, n: c_int) {
    let mut dst = obj.TextW[pos..];

    // We maintain our buffer length in both UTF-8 and wchar formats
    obj.Edited = true;
    obj.CurLenA -= ImTextCountUtf8BytesFromStr(dst);
    obj.CurLenW -= n;

    // TODO
    // Offset remaining text (FIXME-OPT: Use memmove)
    // let src = &mut obj.TextW[pos + n..];
    //     while (let c: ImWchar = *src++){
    //     *dst + + = c;
    // }
    //     *dst = '\0';
}

pub unsafe fn STB_TEXTEDIT_INSERTCHARS(
    obj: &mut ImGuiInputTextState,
    pos: usize,
    new_text: Stringing,
    new_text_len: usize,
) -> bool {
    let is_resizable: bool = flag_set(obj.Flags, ImGuiInputTextFlags_CallbackResize);
    let text_len = obj.CurLenW;
    // IM_ASSERT(pos <= text_len);

    let new_text_len_utf8 = ImTextCountUtf8BytesFromStr(new_text);
    if !is_resizable && (new_text_len_utf8 + obj.CurLenA + 1 > obj.BufCapacityA) {
        return false;
    }

    // Grow internal buffer if needed
    if new_text_len + text_len + 1 > obj.TextW.len() {
        if !is_resizable {
            return false;
        }
        // IM_ASSERT(text_len < obj.TextW.Size);
        obj.TextW.resize(
            text_len + ImClamp(new_text_len * 4, 32, ImMax(256, new_text_len)) + 1,
            '\0',
        );
    }

    let text = obj.TextW.as_mut_slice();
    if pos != text_len {
        // TODO:
        // memmove(text + pos + new_text_len, text + pos, (text_len - pos) * sizeof);
    }
    // TODO:
    // memcpy(text + pos, new_text, new_text_len * sizeof);

    obj.Edited = true;
    obj.CurLenW += new_text_len;
    obj.CurLenA += new_text_len_utf8;
    obj.TextW[obj.CurLenW] = '\0';

    return true;
}

// stb_textedit internally allows for a single undo record to do addition and deletion, but somehow, calling
// the stb_textedit_paste() function creates two separate records, so we perform it manually. (FIXME: Report to nothings/stb?)
pub unsafe fn stb_textedit_replace(
    str_arg: &mut STB_TEXTEDIT_STRING,
    state: &mut STB_TexteditState,
    text: &STB_TEXTEDIT_CHARTYPE,
    text_len: usize,
) {
    stb_text_makeundo_replace(str_arg, state, 0, str_arg.CurLenW, text_len);
    ImStb::STB_TEXTEDIT_DELETECHARS(str, 0, str_arg.CurLenW);
    if text_len <= 0 {
        return;
    }
    if ImStb::STB_TEXTEDIT_INSERTCHARS(str_arg, 0, text, text_len) {
        state.cursor = text_len;
        state.has_preferred_x = 0;
        return;
    }
    // IM_ASSERT(0); // Failed to insert character, normally shouldn't happen because of how we currently use stb_textedit_replace()
}
