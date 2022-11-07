use libc::{c_float, c_int};

#[derive(Default,Debug,Clone.Copy)]
pub struct ImGuiPlotArrayGetterData {
    // *let mut Values: c_float = 0.0;
    pub Values: Vec<c_float>,
    // let mut Stride: c_int = 0;
    pub Stride: i32,
    // ImGuiPlotArrayGetterData(*values: c_float, stride: c_int) { Values = values; Stride = stride; }
}

impl ImGuiPlotArrayGetterData {
    pub fn new(values: &[c_float], stride: i32) -> Self {
        let mut out = Self {
            Values: vec![],
            Stride: 0,
        };
        for v in values {
            out.Values.push(*v)
        }
        out
    }
}
