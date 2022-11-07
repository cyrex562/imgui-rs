#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiNavMoveFlags;          // -> enum ImGuiNavMoveFlags_       // Flags: for navigation requests
pub type ImGuiNavMoveFlags = c_int;


// enum ImGuiNavMoveFlags_
// {
pub const ImGuiNavMoveFlags_None: ImGuiNavMoveFlags = 0;
pub const ImGuiNavMoveFlags_LoopX: ImGuiNavMoveFlags = 1 << 0;
// On failed request, restart from opposite side
pub const ImGuiNavMoveFlags_LoopY: ImGuiNavMoveFlags = 1 << 1;
pub const ImGuiNavMoveFlags_WrapX: ImGuiNavMoveFlags = 1 << 2;
// On failed request, request from opposite side one line down (when NavDir==right) or one line up (when NavDir==left)
pub const ImGuiNavMoveFlags_WrapY: ImGuiNavMoveFlags = 1 << 3;
// This is not super useful but provided for completeness
pub const ImGuiNavMoveFlags_AllowCurrentNavId: ImGuiNavMoveFlags = 1 << 4;
// Allow scoring and considering the current NavId as a move target candidate. This is used when the move source is offset (e.g. pressing PageDown actually needs to send a Up move request, if we are pressing PageDown from the bottom-most item we need to stay in place)
pub const ImGuiNavMoveFlags_AlsoScoreVisibleSet: ImGuiNavMoveFlags = 1 << 5;
// Store alternate result in NavMoveResultLocalVisible that only comprise elements that are already fully visible (used by PageUp/PageDown)
pub const ImGuiNavMoveFlags_ScrollToEdgeY: ImGuiNavMoveFlags = 1 << 6;
// Force scrolling to min/max (used by Home/End) // FIXME-NAV: Aim to remove or reword, probably unnecessary
pub const ImGuiNavMoveFlags_Forwarded: ImGuiNavMoveFlags = 1 << 7;
pub const ImGuiNavMoveFlags_DebugNoResult: ImGuiNavMoveFlags = 1 << 8;
// Dummy scoring for debug purpose, don't apply result
pub const ImGuiNavMoveFlags_FocusApi: ImGuiNavMoveFlags = 1 << 9;
pub const ImGuiNavMoveFlags_Tabbing: ImGuiNavMoveFlags = 1 << 10;
// == Focus + Activate if item is Inputable + DontChangeNavHighlight
pub const ImGuiNavMoveFlags_Activate: ImGuiNavMoveFlags = 1 << 11;
pub const ImGuiNavMoveFlags_DontSetNavHighlight: ImGuiNavMoveFlags = 1 << 12;  // Do not alter the visible state of keyboard vs mouse nav highlight
// };
