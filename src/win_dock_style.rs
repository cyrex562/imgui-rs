#![allow(non_snake_case)]

use crate::window_dock_style_color::ImGuiWindowDockStyleCol_COUNT;

#[derive(Default,Debug,Clone)]
pub struct ImGuiWindowDockStyle
{
    pub Colors: [u32; ImGuiWindowDockStyleCol_COUNT as usize]
}
