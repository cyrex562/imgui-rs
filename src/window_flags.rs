#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiWindowFlags;       // -> enum ImGuiWindowFlags_     // Flags: for Begin(), BeginChild()
pub type ImGuiWindowFlags = c_int;


// Flags for Begin()
// enum ImGuiWindowFlags_
// {
pub const ImGuiWindowFlags_None: ImGuiWindowFlags = 0;
pub const ImGuiWindowFlags_NoTitleBar: ImGuiWindowFlags = 1 << 0;
// Disable title-bar
pub const ImGuiWindowFlags_NoResize: ImGuiWindowFlags = 1 << 1;
// Disable user resizing with the lower-right grip
pub const ImGuiWindowFlags_NoMove: ImGuiWindowFlags = 1 << 2;
// Disable user moving the window
pub const ImGuiWindowFlags_NoScrollbar: ImGuiWindowFlags = 1 << 3;
// Disable scrollbars (window can still scroll with mouse or programmatically)
pub const ImGuiWindowFlags_NoScrollWithMouse: ImGuiWindowFlags = 1 << 4;
// Disable user vertically scrolling with mouse wheel. On child window; mouse wheel will be forwarded to the parent unless NoScrollbar is also set.
pub const ImGuiWindowFlags_NoCollapse: ImGuiWindowFlags = 1 << 5;
// Disable user collapsing window by double-clicking on it. Also referred to as Window Menu Button (e.g. within a docking node).
pub const ImGuiWindowFlags_AlwaysAutoResize: ImGuiWindowFlags = 1 << 6;
// Resize every window to its content every frame
pub const ImGuiWindowFlags_NoBackground: ImGuiWindowFlags = 1 << 7;
// Disable drawing background color (WindowBg; etc.) and outside border. Similar as using SetNextWindowBgAlpha(0f32).
pub const ImGuiWindowFlags_NoSavedSettings: ImGuiWindowFlags = 1 << 8;
// Never load/save settings in .ini file
pub const ImGuiWindowFlags_NoMouseInputs: ImGuiWindowFlags = 1 << 9;
// Disable catching mouse; hovering test with pass through.
pub const ImGuiWindowFlags_MenuBar: ImGuiWindowFlags = 1 << 10;
// Has a menu-bar
pub const ImGuiWindowFlags_HorizontalScrollbar: ImGuiWindowFlags = 1 << 11;
// Allow horizontal scrollbar to appear (off by default). You may use SetNextWindowContentSize(ImVec2::new(width;0f32)); prior to calling Begin() to specify width. Read code in imgui_demo in the "Horizontal Scrolling" section.
pub const ImGuiWindowFlags_NoFocusOnAppearing: ImGuiWindowFlags = 1 << 12;
// Disable taking focus when transitioning from hidden to visible state
pub const ImGuiWindowFlags_NoBringToFrontOnFocus: ImGuiWindowFlags = 1 << 13;
// Disable bringing window to front when taking focus (e.g. clicking on it or programmatically giving it focus)
pub const ImGuiWindowFlags_AlwaysVerticalScrollbar: ImGuiWindowFlags = 1 << 14;
// Always show vertical scrollbar (even if ContentSize.y < Size.y)
pub const ImGuiWindowFlags_AlwaysHorizontalScrollbar: ImGuiWindowFlags = 1 << 15;
// Always show horizontal scrollbar (even if ContentSize.x < Size.x)
pub const ImGuiWindowFlags_AlwaysUseWindowPadding: ImGuiWindowFlags = 1 << 16;
// Ensure child windows without border uses style.WindowPadding (ignored by default for non-bordered child windows; because more convenient)
pub const ImGuiWindowFlags_NoNavInputs: ImGuiWindowFlags = 1 << 18;
// No gamepad/keyboard navigation within the window
pub const ImGuiWindowFlags_NoNavFocus: ImGuiWindowFlags = 1 << 19;
// No focusing toward this window with gamepad/keyboard navigation (e.g. skipped by CTRL+TAB)
pub const ImGuiWindowFlags_UnsavedDocument: ImGuiWindowFlags = 1 << 20;
// Display a dot next to the title. When used in a tab/docking context; tab is selected when clicking the X + closure is not assumed (will wait for user to stop submitting the tab). Otherwise closure is assumed when pressing the X; so if you keep submitting the tab may reappear at end of tab bar.
pub const ImGuiWindowFlags_NoDocking: ImGuiWindowFlags = 1 << 21;  // Disable docking of this window

pub const ImGuiWindowFlags_NoNav: ImGuiWindowFlags = ImGuiWindowFlags_NoNavInputs | ImGuiWindowFlags_NoNavFocus;
pub const ImGuiWindowFlags_NoDecoration: ImGuiWindowFlags = ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoCollapse;
pub const ImGuiWindowFlags_NoInputs: ImGuiWindowFlags = ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs | ImGuiWindowFlags_NoNavFocus;

// [Internal]
pub const ImGuiWindowFlags_NavFlattened: ImGuiWindowFlags = 1 << 23;
// [BETA] On child window: allow gamepad/keyboard navigation to cross over parent border to this child or between sibling child windows.
pub const ImGuiWindowFlags_ChildWindow: ImGuiWindowFlags = 1 << 24;
// Don't use! For internal use by BeginChild()
pub const ImGuiWindowFlags_Tooltip: ImGuiWindowFlags = 1 << 25;
// Don't use! For internal use by BeginTooltip()
pub const ImGuiWindowFlags_Popup: ImGuiWindowFlags = 1 << 26;
// Don't use! For internal use by BeginPopup()
pub const ImGuiWindowFlags_Modal: ImGuiWindowFlags = 1 << 27;
// Don't use! For internal use by BeginPopupModal()
pub const ImGuiWindowFlags_ChildMenu: ImGuiWindowFlags = 1 << 28;
// Don't use! For internal use by BeginMenu()
pub const ImGuiWindowFlags_DockNodeHost: ImGuiWindowFlags = 1 << 29;  // Don't use! For internal use by Begin()/NewFrame()
// };
