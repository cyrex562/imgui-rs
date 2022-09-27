#![allow(non_upper_case_globals)]

use crate::color::{ImGuiCol, ImGuiCol_Tab, ImGuiCol_TabActive, ImGuiCol_TabHovered, ImGuiCol_TabUnfocused, ImGuiCol_TabUnfocusedActive, ImGuiCol_Text};
use crate::window_dock_style_color::ImGuiWindowDockStyleCol_COUNT;

pub const  GWindowDockStyleColors: [ImGuiCol; ImGuiWindowDockStyleCol_COUNT as usize] =
[
ImGuiCol_Text, ImGuiCol_Tab, ImGuiCol_TabHovered, ImGuiCol_TabActive, ImGuiCol_TabUnfocused, ImGuiCol_TabUnfocusedActive
];