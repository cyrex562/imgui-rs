#![allow(non_snake_case)]

use libc::c_short;

#[derive(Default, Debug, Clone)]
pub struct ImGuiStackSizes {
    pub SizeOfIDStack: c_short,
    pub SizeOfColorStack: c_short,
    pub SizeOfStyleVarStack: c_short,
    pub SizeOfFontStack: c_short,
    pub SizeOfFocusScopeStack: c_short,
    pub SizeOfGroupStack: c_short,
    pub SizeOfItemFlagsStack: c_short,
    pub SizeOfBeginPopupStack: c_short,
    pub SizeOfDisabledStack: c_short,

}

impl ImGuiStackSizes {
    // ImGuiStackSizes() { memset(this, 0, sizeof(*this)); }


    // c_void SetToCurrentState();
    pub fn SetToCurrentState(&mut self) {
        todo!()
    }

    // c_void CompareWithCurrentState();
    pub fn CompareWithCurrentState(&mut self) {
        todo!()
    }
}