use libc::c_int;
use crate::a_stb_textedit::STB_TEXTEDIT_POSITIONTYPE;

#[derive(Default, Debug, Clone)]
pub struct StbUndoRecord {
    // private data
    pub stb_where: STB_TEXTEDIT_POSITIONTYPE,
    pub insert_length: STB_TEXTEDIT_POSITIONTYPE,
    pub delete_length: STB_TEXTEDIT_POSITIONTYPE,
    pub char_storage: c_int,
} 
