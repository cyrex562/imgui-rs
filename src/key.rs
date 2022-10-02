#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiKey;               // -> enum ImGuiKey_             // Enum: A key identifier
pub type ImGuiKey = c_int;


// Keys value 0 to 511 are left unused as legacy native/opaque key values (< 1.87)
// Keys value >= 512 are named keys (>= 1.87)
// enum ImGuiKey_
// {
// Keyboard
pub const ImGuiKey_None: ImGuiKey = 0;
pub const ImGuiKey_Tab: ImGuiKey = 512;
// == ImGuiKey_NamedKey_BEGIN
pub const ImGuiKey_LeftArrow: ImGuiKey = 513;
pub const ImGuiKey_RightArrow: ImGuiKey = 514;
pub const ImGuiKey_UpArrow: ImGuiKey = 515;
pub const ImGuiKey_DownArrow: ImGuiKey = 516;
pub const ImGuiKey_PageUp: ImGuiKey = 517;
pub const ImGuiKey_PageDown: ImGuiKey = 518;
pub const ImGuiKey_Home: ImGuiKey = 519;
pub const ImGuiKey_End: ImGuiKey = 520;
pub const ImGuiKey_Insert: ImGuiKey = 521;
pub const ImGuiKey_Delete: ImGuiKey = 522;
pub const ImGuiKey_Backspace: ImGuiKey = 523;
pub const ImGuiKey_Space: ImGuiKey = 524;
pub const ImGuiKey_Enter: ImGuiKey = 525;
pub const ImGuiKey_Escape: ImGuiKey = 526;
pub const ImGuiKey_LeftCtrl: ImGuiKey = 527;
pub const ImGuiKey_LeftShift: ImGuiKey = 528;
pub const ImGuiKey_LeftAlt: ImGuiKey = 529;
pub const ImGuiKey_LeftSuper: ImGuiKey = 530;
pub const ImGuiKey_RightCtrl: ImGuiKey = 531;
pub const ImGuiKey_RightShift: ImGuiKey = 532;
pub const ImGuiKey_RightAlt: ImGuiKey = 533;
pub const ImGuiKey_RightSuper: ImGuiKey = 534;
pub const ImGuiKey_Menu: ImGuiKey = 535;
pub const ImGuiKey_0: ImGuiKey = 536;
pub const ImGuiKey_1: ImGuiKey = 537;
pub const ImGuiKey_2: ImGuiKey = 538;
pub const ImGuiKey_3: ImGuiKey = 539;
pub const ImGuiKey_4: ImGuiKey = 540;
pub const ImGuiKey_5: ImGuiKey = 550;
pub const ImGuiKey_6: ImGuiKey = 551;
pub const ImGuiKey_7: ImGuiKey = 552;
pub const ImGuiKey_8: ImGuiKey = 553;
pub const ImGuiKey_9: ImGuiKey = 554;
pub const ImGuiKey_A: ImGuiKey = 555;
pub const ImGuiKey_B: ImGuiKey = 556;
pub const ImGuiKey_C: ImGuiKey = 557;
pub const ImGuiKey_D: ImGuiKey = 558;
pub const ImGuiKey_E: ImGuiKey = 559;
pub const ImGuiKey_F: ImGuiKey = 560;
pub const ImGuiKey_G: ImGuiKey = 561;
pub const ImGuiKey_H: ImGuiKey = 562;
pub const ImGuiKey_I: ImGuiKey = 563;
pub const ImGuiKey_J: ImGuiKey = 564;
pub const ImGuiKey_K: ImGuiKey = 565;
pub const ImGuiKey_L: ImGuiKey = 566;
pub const ImGuiKey_M: ImGuiKey = 567;
pub const ImGuiKey_N: ImGuiKey = 568;
pub const ImGuiKey_O: ImGuiKey = 569;
pub const ImGuiKey_P: ImGuiKey = 570;
pub const ImGuiKey_Q: ImGuiKey = 571;
pub const ImGuiKey_R: ImGuiKey = 572;
pub const ImGuiKey_S: ImGuiKey = 573;
pub const ImGuiKey_T: ImGuiKey = 574;
pub const ImGuiKey_U: ImGuiKey = 575;
pub const ImGuiKey_V: ImGuiKey = 576;
pub const ImGuiKey_W: ImGuiKey = 577;
pub const ImGuiKey_X: ImGuiKey = 578;
pub const ImGuiKey_Y: ImGuiKey = 579;
pub const ImGuiKey_Z: ImGuiKey = 580;
pub const ImGuiKey_F1: ImGuiKey = 581;
pub const ImGuiKey_F2: ImGuiKey = 582;
pub const ImGuiKey_F3: ImGuiKey = 583;
pub const ImGuiKey_F4: ImGuiKey = 584;
pub const ImGuiKey_F5: ImGuiKey = 585;
pub const ImGuiKey_F6: ImGuiKey = 586;
pub const ImGuiKey_F7: ImGuiKey = 587;
pub const ImGuiKey_F8: ImGuiKey = 588;
pub const ImGuiKey_F9: ImGuiKey = 589;
pub const ImGuiKey_F10: ImGuiKey = 590;
pub const ImGuiKey_F11: ImGuiKey = 591;
pub const ImGuiKey_F12: ImGuiKey = 592;
pub const ImGuiKey_Apostrophe: ImGuiKey = 593;
// '
pub const ImGuiKey_Comma: ImGuiKey = 594;
// ;
pub const ImGuiKey_Minus: ImGuiKey = 595;
// -
pub const ImGuiKey_Period: ImGuiKey = 596;
// .
pub const ImGuiKey_Slash: ImGuiKey = 597;
// /
pub const ImGuiKey_Semicolon: ImGuiKey = 598;
// ;
pub const ImGuiKey_Equal: ImGuiKey = 599;
// =
pub const ImGuiKey_LeftBracket: ImGuiKey = 600;
// [
pub const ImGuiKey_Backslash: ImGuiKey = 601;
// \ (this text inhibit multiline comment caused by backslash)
pub const ImGuiKey_RightBracket: ImGuiKey = 602;
// ]
pub const ImGuiKey_GraveAccent: ImGuiKey = 603;
// `
pub const ImGuiKey_CapsLock: ImGuiKey = 604;
pub const ImGuiKey_ScrollLock: ImGuiKey = 605;
pub const ImGuiKey_NumLock: ImGuiKey = 606;
pub const ImGuiKey_PrintScreen: ImGuiKey = 607;
pub const ImGuiKey_Pause: ImGuiKey = 608;
pub const ImGuiKey_Keypad0: ImGuiKey = 609;
pub const ImGuiKey_Keypad1: ImGuiKey = 610;
pub const ImGuiKey_Keypad2: ImGuiKey = 611;
pub const ImGuiKey_Keypad3: ImGuiKey = 612;
pub const ImGuiKey_Keypad4: ImGuiKey = 613;
pub const ImGuiKey_Keypad5: ImGuiKey = 614;
pub const ImGuiKey_Keypad6: ImGuiKey = 615;
pub const ImGuiKey_Keypad7: ImGuiKey = 616;
pub const ImGuiKey_Keypad8: ImGuiKey = 617;
pub const ImGuiKey_Keypad9: ImGuiKey = 618;
pub const ImGuiKey_KeypadDecimal: ImGuiKey = 619;
pub const ImGuiKey_KeypadDivide: ImGuiKey = 620;
pub const ImGuiKey_KeypadMultiply: ImGuiKey = 621;
pub const ImGuiKey_KeypadSubtract: ImGuiKey = 622;
pub const ImGuiKey_KeypadAdd: ImGuiKey = 623;
pub const ImGuiKey_KeypadEnter: ImGuiKey = 624;
pub const ImGuiKey_KeypadEqual: ImGuiKey = 625;

// Gamepad (some of those are analog values; 0f32 to 1f32)                          // GAME NAVIGATION ACTION
// (download controller mapping PNG/PSD at http://dearimgui.org/controls_sheets)
pub const ImGuiKey_GamepadStart: ImGuiKey = 626;
// Menu (Xbox)      + (Switch)   Start/Options (PS)
pub const ImGuiKey_GamepadBack: ImGuiKey = 627;
// View (Xbox)      - (Switch)   Share (PS)
pub const ImGuiKey_GamepadFaceLeft: ImGuiKey = 628;
// X (Xbox)         Y (Switch)   Square (PS)        // Tap: Toggle Menu. Hold: Windowing mode (Focus/Move/Resize windows)
pub const ImGuiKey_GamepadFaceRight: ImGuiKey = 629;
// B (Xbox)         A (Switch)   Circle (PS)        // Cancel / Close / Exit
pub const ImGuiKey_GamepadFaceUp: ImGuiKey = 630;
// Y (Xbox)         X (Switch)   Triangle (PS)      // Text Input / On-screen Keyboard
pub const ImGuiKey_GamepadFaceDown: ImGuiKey = 631;
// A (Xbox)         B (Switch)   Cross (PS)         // Activate / Open / Toggle / Tweak
pub const ImGuiKey_GamepadDpadLeft: ImGuiKey = 632;
// D-pad Left                                       // Move / Tweak / Resize Window (in Windowing mode)
pub const ImGuiKey_GamepadDpadRight: ImGuiKey = 633;
// D-pad Right                                      // Move / Tweak / Resize Window (in Windowing mode)
pub const ImGuiKey_GamepadDpadUp: ImGuiKey = 634;
// D-pad Up                                         // Move / Tweak / Resize Window (in Windowing mode)
pub const ImGuiKey_GamepadDpadDown: ImGuiKey = 635;
// D-pad Down                                       // Move / Tweak / Resize Window (in Windowing mode)
pub const ImGuiKey_GamepadL1: ImGuiKey = 636;
// L Bumper (Xbox)  L (Switch)   L1 (PS)            // Tweak Slower / Focus Previous (in Windowing mode)
pub const ImGuiKey_GamepadR1: ImGuiKey = 637;
// R Bumper (Xbox)  R (Switch)   R1 (PS)            // Tweak Faster / Focus Next (in Windowing mode)
pub const ImGuiKey_GamepadL2: ImGuiKey = 638;
// L Trig. (Xbox)   ZL (Switch)  L2 (PS) [Analog]
pub const ImGuiKey_GamepadR2: ImGuiKey = 639;
// R Trig. (Xbox)   ZR (Switch)  R2 (PS) [Analog]
pub const ImGuiKey_GamepadL3: ImGuiKey = 640;
// L Stick (Xbox)   L3 (Switch)  L3 (PS)
pub const ImGuiKey_GamepadR3: ImGuiKey = 641;
// R Stick (Xbox)   R3 (Switch)  R3 (PS)
pub const ImGuiKey_GamepadLStickLeft: ImGuiKey = 642;
// [Analog]                                         // Move Window (in Windowing mode)
pub const ImGuiKey_GamepadLStickRight: ImGuiKey = 643;
// [Analog]                                         // Move Window (in Windowing mode)
pub const ImGuiKey_GamepadLStickUp: ImGuiKey = 644;
// [Analog]                                         // Move Window (in Windowing mode)
pub const ImGuiKey_GamepadLStickDown: ImGuiKey = 645;
// [Analog]                                         // Move Window (in Windowing mode)
pub const ImGuiKey_GamepadRStickLeft: ImGuiKey = 646;
// [Analog]
pub const ImGuiKey_GamepadRStickRight: ImGuiKey = 647;
// [Analog]
pub const ImGuiKey_GamepadRStickUp: ImGuiKey = 648;
// [Analog]
pub const ImGuiKey_GamepadRStickDown: ImGuiKey = 649;     // [Analog]

// Keyboard Modifiers (explicitly submitted by backend via AddKeyEvent() calls)
// - This is mirroring the data also written to io.KeyCtrl; io.KeyShift; io.KeyAlt; io.KeySuper; in a format allowing
//   them to be accessed via standard key API; allowing calls such as IsKeyPressed(); IsKeyReleased(); querying duration etc.
// - Code polling every keys (e.g. an interface to detect a key press for input mapping) might want to ignore those
//   and prefer using the real keys (e.g. pub const ImGuiKey_LeftCtrl: ImGuiKey = 0; ImGuiKey_RightCtrl instead of ImGuiKey_ModCtrl).
// - In theory the value of keyboard modifiers should be roughly equivalent to a logical or of the equivalent left/right keys.
//   In practice: it's complicated; mods are often provided from different sources. Keyboard layout; IME; sticky keys and
//   backends tend to interfere and break that equivalence. The safer decision is to relay that ambiguity down to the end-user...
pub const ImGuiKey_ModCtrl: ImGuiKey = 650;
pub const ImGuiKey_ModShift: ImGuiKey = 651;
pub const ImGuiKey_ModAlt: ImGuiKey = 652;
pub const ImGuiKey_ModSuper: ImGuiKey = 653;

// Mouse Buttons (auto-submitted from AddMouseButtonEvent() calls)
// - This is mirroring the data also written to io.MouseDown[]; io.MouseWheel; in a format allowing them to be accessed via standard key API.
pub const ImGuiKey_MouseLeft: ImGuiKey = 654;
pub const ImGuiKey_MouseRight: ImGuiKey = 655;
pub const ImGuiKey_MouseMiddle: ImGuiKey = 656;
pub const ImGuiKey_MouseX1: ImGuiKey = 657;
pub const ImGuiKey_MouseX2: ImGuiKey = 658;
pub const ImGuiKey_MouseWheelX: ImGuiKey = 659;
pub const ImGuiKey_MouseWheelY: ImGuiKey = 660;

// End of list
pub const ImGuiKey_COUNT: ImGuiKey = 661;                 // No valid ImGuiKey is ever greater than this value

// [Internal] Prior to 1.87 we required user to fill io.KeysDown[512] using their own native index + a io.KeyMap[] array.
// We are ditching this method but keeping a legacy path for user code doing e.g. IsKeyPressed(MY_NATIVE_KEY_CODE)
pub const ImGuiKey_NamedKey_BEGIN: ImGuiKey = 512;
pub const ImGuiKey_NamedKey_END: ImGuiKey = ImGuiKey_COUNT;
pub const ImGuiKey_NamedKey_COUNT: ImGuiKey = ImGuiKey_NamedKey_END - ImGuiKey_NamedKey_BEGIN;
// #ifdef IMGUI_DISABLE_OBSOLETE_KEYIO
pub const ImGuiKey_KeysData_SIZE: ImGuiKey = ImGuiKey_NamedKey_COUNT;
// Size of KeysData[]: only hold named keys
pub const ImGuiKey_KeysData_OFFSET: ImGuiKey = ImGuiKey_NamedKey_BEGIN;          // First key stored in io.KeysData[0]. Accesses to io.KeysData[] must use (key - ImGuiKey_KeysData_OFFSET).
// #else
//     ImGuiKey_KeysData_SIZE          = ImGuiKey_COUNT,                   // Size of KeysData[]: hold legacy 0..512 keycodes + named keys
//     ImGuiKey_KeysData_OFFSET        = 0,                                // First key stored in io.KeysData[0]. Accesses to io.KeysData[] must use (key - ImGuiKey_KeysData_OFFSET).
// #endif

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     ImGuiKey_KeyPadEnter = ImGuiKey_KeypadEnter,    // Renamed in 1.87
// #endif
// };

// Extend ImGuiKey_
// enum ImGuiKeyPrivate_
// {
pub const ImGuiKey_LegacyNativeKey_BEGIN: ImGuiKey = 0;
pub const ImGuiKey_LegacyNativeKey_END: ImGuiKey = 512;
pub const ImGuiKey_Keyboard_BEGIN: ImGuiKey = ImGuiKey_NamedKey_BEGIN;
pub const ImGuiKey_Keyboard_END: ImGuiKey = ImGuiKey_GamepadStart;
pub const ImGuiKey_Gamepad_BEGIN: ImGuiKey = ImGuiKey_GamepadStart;
pub const ImGuiKey_Gamepad_END: ImGuiKey = ImGuiKey_GamepadRStickDown + 1;
pub const ImGuiKey_Aliases_BEGIN: ImGuiKey = ImGuiKey_MouseLeft;
pub const ImGuiKey_Aliases_END: ImGuiKey = ImGuiKey_COUNT;

// [Internal] Named shortcuts for Navigation
pub const ImGuiKey_NavKeyboardTweakSlow: ImGuiKey = ImGuiKey_ModCtrl;
pub const ImGuiKey_NavKeyboardTweakFast: ImGuiKey = ImGuiKey_ModShift;
pub const ImGuiKey_NavGamepadTweakSlow: ImGuiKey = ImGuiKey_GamepadL1;
pub const ImGuiKey_NavGamepadTweakFast: ImGuiKey = ImGuiKey_GamepadR1;
pub const ImGuiKey_NavGamepadActivate: ImGuiKey = ImGuiKey_GamepadFaceDown;
pub const ImGuiKey_NavGamepadCancel: ImGuiKey = ImGuiKey_GamepadFaceRight;
pub const ImGuiKey_NavGamepadMenu: ImGuiKey = ImGuiKey_GamepadFaceLeft;
pub const ImGuiKey_NavGamepadInput: ImGuiKey = ImGuiKey_GamepadFaceUp;
// };