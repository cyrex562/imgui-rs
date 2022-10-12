#![allow(non_snake_case)]

use libc::{c_float, c_int};
use crate::list_clipper::ImGuiListClipper;
use crate::list_clipper_range::ImGuiListClipperRange;
use crate::list_clipping::ImGuiListClipper;

// Temporary clipper data, buffers shared/reused between instances
#[derive(Default, Debug, Clone)]
pub struct ImGuiListClipperData {
    pub ListClipper: *mut ImGuiListClipper,
    pub LossynessOffset: c_float,
    pub StepNo: usize,
    pub ItemsFrozen: c_int,
    pub Ranges: Vec<ImGuiListClipperRange>,
}

impl ImGuiListClipperData {
    // ImGuiListClipperData()          { memset(this, 0, sizeof(*this)); }


    // void                            Reset(*mut ImGuiListClipper clipper) { ListClipper = clipper; StepNo = ItemsFrozen = 0; Ranges.resize(0); }
    pub fn Reset(&mut self, clipper: *mut ImGuiListClipper) {
        self.ListClipper = clipper;
        self.StepNo = 0;
        self.ItemsFrozen = 0;
        self.Ranges.clear();
    }
}

