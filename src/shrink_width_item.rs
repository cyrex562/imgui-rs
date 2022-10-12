#![allow(non_snake_case)]

use libc::{c_float, c_int};

#[derive(Default,Debug,Clone)]
pub struct ImGuiShrinkWidthItem
{
pub Index:  c_int,
pub Width:  c_float,
pub InitialWidth:  c_float,
}
