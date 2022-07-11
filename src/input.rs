use std::ffi::c_void;
use crate::imgui_h::{ImGuiCond, ImGuiID, ImGuiInputTextFlags, ImGuiKey};

// Shared state of InputText(), passed as an argument to your callback when a ImGuiInputTextFlags_Callback* flag is used.
// The callback function should return 0 by default.
// Callbacks (follow a flag name and see comments in ImGuiInputTextFlags_ declarations for more details)
// - ImGuiInputTextFlags_CallbackEdit:        Callback on buffer edit (note that InputText() already returns true on edit, the callback is useful mainly to manipulate the underlying buffer while focus is active)
// - ImGuiInputTextFlags_CallbackAlways:      Callback on each iteration
// - ImGuiInputTextFlags_CallbackCompletion:  Callback on pressing TAB
// - ImGuiInputTextFlags_CallbackHistory:     Callback on pressing Up/down arrows
// - ImGuiInputTextFlags_CallbackCharFilter:  Callback on character inputs to replace or discard them. Modify 'EventChar' to replace or discard, or return 1 in callback to discard.
// - ImGuiInputTextFlags_CallbackResize:      Callback on buffer capacity changes request (beyond 'buf_size' parameter value), allowing the string to grow.
pub struct ImGuiInputTextCallbackData
{
    pub EventFlag: ImGuiInputTextFlags,      // One ImGuiInputTextFlags_Callback*    // Read-only
    pub Flags: ImGuiInputTextFlags,          // What user passed to InputText()      // Read-only
    pub UserData: *mut c_void,       // What user passed to InputText()      // Read-only

    // Arguments for the different callback events
    // - To modify the text buffer in a callback, prefer using the InsertChars() / DeleteChars() function. InsertChars() will take care of calling the resize callback if necessary.
    // - If you know your edits are not going to resize the underlying buffer allocation, you may modify the contents of 'Buf[]' directly. You need to update 'BufTextLen' accordingly (0 <= BufTextLen < BufSize) and set 'BufDirty'' to true so InputText can update its internal state.
    pub EventChar: u8,      // Character input                      // Read-write   // [CharFilter] Replace character with another one, or set to zero to drop. return 1 is equivalent to setting EventChar=0;
    pub            EventKey: ImGuiKey,       // Key pressed (Up/down/TAB)            // Read-only    // [Completion,History]
    pub               Buf: String,            // Text buffer                          // Read-write   // [Resize] Can replace pointer / [Completion,History,Always] Only write to pointed data, don't replace the actual pointer!
    pub BufTextLen: i32,   // Text length (in bytes)               // Read-write   // [Resize,Completion,History,Always] Exclude zero-terminator storage. In C land: == strlen(some_text), in C++ land: string.length()
    pub BufSize: i32,      // Buffer size (in bytes) = capacity+1  // Read-only    // [Resize,Completion,History,Always] Include zero-terminator storage. In C land == ARRAYSIZE(my_char_array), in C++ land: string.capacity()+1
    pub BufDirty: bool,       // Set if you modify Buf/BufTextLen!    // Write        // [Completion,History,Always]
    pub CursorPos: i32,    //                                      // Read-write   // [Completion,History,Always]
    pub                 SelectionStart: i32, //                                      // Read-write   // [Completion,History,Always] == to SelectionEnd when no selection)
    pub SelectionEnd: i32, //                                      // Read-write   // [Completion,History,Always]

    // Helper functions for text manipulation.
    // Use those function to benefit from the CallbackResize behaviors. Calling those function reset the selection.

}

impl ImGuiInputTextCallbackData {
    //  ImGuiInputTextCallbackData();
    pub fn ImGuiInputTextCallbackData() {
        todo!();
    }
    //  void      DeleteChars(int pos, int bytes_count);
    pub fn DeleteChars(pos: i32, bytes_count: i32) {
        todo!();
    }
    //  void      InsertChars(int pos, const char* text, const char* text_end = NULL);
    pub fn InsertChars(pos: i32, text: &str, text_end: &str) {
        todo!();
    }
    // void                SelectAll()             { SelectionStart = 0; SelectionEnd = BufTextLen; }
    pub fn SelectAll(&mut self) {
        self.SelectionStart = 0;
        self.SelectionEnd = self.BufTextLen;
    }
    // void                ClearSelection()        { SelectionStart = SelectionEnd = BufTextLen; }
    pub fn ClearSelection(&mut self) {
        self.SelectionStart = self.BufTextLen;
        self.SelectionEnd = self.BufTextLen;
    }
    // bool                HasSelection() const    { return SelectionStart != SelectionEnd; }
    pub fn HasSelection(&self) -> bool {
        self.SelectionStart != self.SelectionEnd
    }
}

pub enum InputSource
{
    None = 0,
    Mouse,
    Keyboard,
    Gamepad,
    Clipboard,     // Currently only used by InputText()
    Nav,           // Stored in g.active_id_source only
}

#[derive(Default,Debug,Clone)]
pub struct ImGuiInputCallbackData {

}

pub enum DimgKey
{
    LegacyNativeKeyBegin = 0,
    LegacyNativeKeyEnd = 512,
    //GamepadBegin = GamepadStart,
    //GamepadEnd = GamepadRStickRight + 1
}

pub const GAME_PAD_BEGIN: DimgKey = GamepadStart;
pub const GAME_PAD_END: DimgKey = GamepadRStickRight;

pub enum DimgInputEventType
{
    None = 0,
    MousePos,
    MouseWheel,
    MouseButton,
    MouseViewport,
    Key,
    Text,
    Focus,
    LastItem
}


// FIXME: Structures in the union below need to be declared as anonymous unions appears to be an extension?
// Using Vector2D() would fail on Clang 'union member 'mouse_pos' has a non-trivial default constructor'
pub struct DimgInputEventMousePos { 
    pub pos_x: f32,
    pub pos_y: f32,
    // float PosX, PosY; 
}

pub struct ImGuiInputEventMouseWheel    { 
    // float WheelX, WheelY; 
pub wheel_x: f32,
    pub wheel_y: f32
}


pub struct ImGuiInputEventMouseButton   { 
    // int Button; bool Down; 
    pub button: i32,
    pub down: bool,
}

pub struct ImGuiInputEventMouseViewport { 
    // ImGuiID HoveredViewportID; 
    pub hovered_viewport_id: DimgId,
}

pub struct ImGuiInputEventKey           { 
    // ImGuiKey Key; bool Down; float AnalogValue; 
    pub key: DimgKey,
    pub down: bool,
    pub analog_value: f32,
}

pub struct ImGuiInputEventText          { 
    // unsigned int Char; 
    pub key: u32,
}

pub struct ImGuiInputEventAppFocused    { 
    // bool Focused; 
    pub focused: bool,
}

// Keys value 0 to 511 are left unused as legacy native/opaque key values (< 1.87)
// Keys value >= 512 are named keys (>= 1.87)
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgKey
{
    // Keyboard
    None = 0,
    Tab = 512,             // == ImGuiKey_NamedKey_BEGIN
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    PageUp,
    PageDown,
    Home,
    End,
    Insert,
    Delete,
    Backspace,
    Space,
    Enter,
    Escape,
    LeftCtrl, LeftShift, LeftAlt, LeftSuper,
    RightCtrl, RightShift, RightAlt, RightSuper,
    Menu,
    Key_0, Key_1, Key_2, Key_3, Key_4, Key_5, Key_6, Key_7, Key_8, Key_9,
    Key_A, Key_B, Key_C, Key_D, Key_E, Key_F, Key_G, Key_H, Key_I, Key_J,
    Key_K, Key_L, Key_M, Key_N, Key_O, Key_P, Key_Q, Key_R, Key_S, Key_T,
    Key_U, Key_V, Key_W, Key_X, Key_Y, Key_Z,
    F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12,
    Apostrophe,        // '
    Comma,             // ,
    Minus,             // -
    Period,            // .
    Slash,             // /
    Semicolon,         // ;
    Equal,             // =
    LeftBracket,       // [
    Backslash,         // \ (this text inhibit multiline comment caused by backslash)
    RightBracket,      // ]
    GraveAccent,       // `
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Keypad0, Keypad1, Keypad2, Keypad3, Keypad4,
    Keypad5, Keypad6, Keypad7, Keypad8, Keypad9,
    KeypadDecimal,
    KeypadDivide,
    KeypadMultiply,
    KeypadSubtract,
    KeypadAdd,
    KeypadEnter,
    KeypadEqual,

    // Gamepad (some of those are analog values, 0.0 to 1.0)                              // NAVIGATION action
    GamepadStart,          // Menu (Xbox)          + (Switch)   Start/Options (PS) // --
    GamepadBack,           // View (Xbox)          - (Switch)   Share (PS)         // --
    GamepadFaceUp,         // Y (Xbox)             x (Switch)   Triangle (PS)      // -> ImGuiNavInput_Input
    GamepadFaceDown,       // A (Xbox)             B (Switch)   Cross (PS)         // -> ImGuiNavInput_Activate
    GamepadFaceLeft,       // x (Xbox)             Y (Switch)   Square (PS)        // -> ImGuiNavInput_Menu
    GamepadFaceRight,      // B (Xbox)             A (Switch)   Circle (PS)        // -> ImGuiNavInput_Cancel
    GamepadDpadUp,         // D-pad Up                                             // -> ImGuiNavInput_DpadUp
    GamepadDpadDown,       // D-pad down                                           // -> ImGuiNavInput_DpadDown
    GamepadDpadLeft,       // D-pad Left                                           // -> ImGuiNavInput_DpadLeft
    GamepadDpadRight,      // D-pad Right                                          // -> ImGuiNavInput_DpadRight
    GamepadL1,             // L Bumper (Xbox)      L (Switch)   L1 (PS)            // -> ImGuiNavInput_FocusPrev + ImGuiNavInput_TweakSlow
    GamepadR1,             // R Bumper (Xbox)      R (Switch)   R1 (PS)            // -> ImGuiNavInput_FocusNext + ImGuiNavInput_TweakFast
    GamepadL2,             // L Trigger (Xbox)     ZL (Switch)  L2 (PS) [Analog]
    GamepadR2,             // R Trigger (Xbox)     ZR (Switch)  R2 (PS) [Analog]
    GamepadL3,             // L Thumbstick (Xbox)  L3 (Switch)  L3 (PS)
    GamepadR3,             // R Thumbstick (Xbox)  R3 (Switch)  R3 (PS)
    GamepadLStickUp,       // [Analog]                                             // -> ImGuiNavInput_LStickUp
    GamepadLStickDown,     // [Analog]                                             // -> ImGuiNavInput_LStickDown
    GamepadLStickLeft,     // [Analog]                                             // -> ImGuiNavInput_LStickLeft
    GamepadLStickRight,    // [Analog]                                             // -> ImGuiNavInput_LStickRight
    GamepadRStickUp,       // [Analog]
    GamepadRStickDown,     // [Analog]
    GamepadRStickLeft,     // [Analog]
    GamepadRStickRight,    // [Analog]

    // Keyboard Modifiers (explicitly submitted by backend via add_key_event() calls)
    // - This is mirroring the data also written to io.key_ctrl, io.key_shift, io.key_alt, io.key_super, in a format allowing
    //   them to be accessed via standard key API, allowing calls such as IsKeyPressed(), IsKeyReleased(), querying duration etc.
    // - Code polling every keys (e.g. an interface to detect a key press for input mapping) might want to ignore those
    //   and prefer using the real keys (e.g. ImGuiKey_LeftCtrl, ImGuiKey_RightCtrl instead of ImGuiKey_ModCtrl).
    // - In theory the value of keyboard modifiers should be roughly equivalent to a logical or of the equivalent left/right keys.
    //   In practice: it's complicated; mods are often provided from different sources. Keyboard layout, IME, sticky keys and
    //   backends tend to interfere and break that equivalence. The safer decision is to relay that ambiguity down to the end-user...
    ModCtrl, ModShift, ModAlt, ModSuper,

    // End of list
    LastItem,                 // No valid ImGuiKey is ever greater than this value

    // [Internal] Prior to 1.87 we required user to fill io.KeysDown[512] using their own native index + a io.KeyMap[] array.
    // We are ditching this method but keeping a legacy path for user code doing e.g. IsKeyPressed(MY_NATIVE_KEY_CODE)
    // ImGuiKey_NamedKey_BEGIN         = 512,
    // ImGuiKey_NamedKey_END           = ImGuiKey_COUNT,
    // ImGuiKey_NamedKey_COUNT         = ImGuiKey_NamedKey_END - ImGuiKey_NamedKey_BEGIN,
// #ifdef IMGUI_DISABLE_OBSOLETE_KEYIO
//     ImGuiKey_KeysData_SIZE          = ImGuiKey_NamedKey_COUNT,          // size of keys_data[]: only hold named keys
//     ImGuiKey_KeysData_OFFSET        = ImGuiKey_NamedKey_BEGIN           // First key stored in io.keys_data[0]. Accesses to io.keys_data[] must use (key - ImGuiKey_KeysData_OFFSET).
// #else
//     ImGuiKey_KeysData_SIZE          = ImGuiKey_COUNT,                   // size of keys_data[]: hold legacy 0..512 keycodes + named keys
//     ImGuiKey_KeysData_OFFSET        = 0                                 // First key stored in io.keys_data[0]. Accesses to io.keys_data[] must use (key - ImGuiKey_KeysData_OFFSET).
// #endif

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     , ImGuiKey_KeyPadEnter = ImGuiKey_KeypadEnter   // Renamed in 1.87
// #endif
}


pub const DIMG_NAMED_KEY_BEGIN: DimgKey         = DimgKey::Tab;

pub const DIMG_NAMED_KEY_END: DimgKey = DimgKey::LastItem;

pub const DIMG_KEYS_DATA_SZ: usize = DimgKey::LastItem as usize - DimgKey::Tab as usize;

pub const DIMG_KEYS_DATA_OFFSET: usize        = 0    ;

// Helper "flags" version of key-mods to store and compare multiple key-mods easily. Sometimes used for storage (e.g. io.key_mods) but otherwise not much used in public API.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ModFlags
{
    None,
    Ctrl,
    Shift,
    Alt,   // Menu
    Super    // Cmd/Super/windows key
}

// Gamepad/Keyboard navigation
// Since >= 1.87 backends you generally don't need to care about this enum since io.nav_inputs[] is setup automatically. This might become private/internal some day.
// Keyboard: Set io.config_flags |= ImGuiConfigFlags_NavEnableKeyboard to enable. NewFrame() will automatically fill io.nav_inputs[] based on your io.add_key_event() calls.
// Gamepad:  Set io.config_flags |= ImGuiConfigFlags_NavEnableGamepad to enable. Backend: set ImGuiBackendFlags_HasGamepad and fill the io.nav_inputs[] fields before calling NewFrame(). Note that io.nav_inputs[] is cleared by EndFrame().
// Read instructions in imgui.cpp for more details. Download PNG/PSD at http://dearimgui.org/controls_sheets.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImGuiNavInput
{
    // Gamepad Mapping
    ImGuiNavInput_Activate,      // Activate / Open / Toggle / Tweak value       // e.g. Cross  (PS4), A (Xbox), A (Switch), Space (Keyboard)
    ImGuiNavInput_Cancel,        // Cancel / Close / Exit                        // e.g. Circle (PS4), B (Xbox), B (Switch), Escape (Keyboard)
    ImGuiNavInput_Input,         // Text input / On-Screen keyboard              // e.g. Triang.(PS4), Y (Xbox), x (Switch), Return (Keyboard)
    ImGuiNavInput_Menu,          // Tap: Toggle menu / Hold: Focus, Move, Resize // e.g. Square (PS4), x (Xbox), Y (Switch), Alt (Keyboard)
    ImGuiNavInput_DpadLeft,      // Move / Tweak / Resize window (w/ PadMenu)    // e.g. D-pad Left/Right/Up/down (Gamepads), Arrow keys (Keyboard)
    ImGuiNavInput_DpadRight,     //
    ImGuiNavInput_DpadUp,        //
    ImGuiNavInput_DpadDown,      //
    ImGuiNavInput_LStickLeft,    // scroll / Move window (w/ PadMenu)            // e.g. Left Analog Stick Left/Right/Up/down
    ImGuiNavInput_LStickRight,   //
    ImGuiNavInput_LStickUp,      //
    ImGuiNavInput_LStickDown,    //
    ImGuiNavInput_FocusPrev,     // Focus Next window (w/ PadMenu)               // e.g. L1 or L2 (PS4), LB or LT (Xbox), L or ZL (Switch)
    ImGuiNavInput_FocusNext,     // Focus Prev window (w/ PadMenu)               // e.g. R1 or R2 (PS4), RB or RT (Xbox), R or ZL (Switch)
    ImGuiNavInput_TweakSlow,     // Slower tweaks                                // e.g. L1 or L2 (PS4), LB or LT (Xbox), L or ZL (Switch)
    ImGuiNavInput_TweakFast,     // Faster tweaks                                // e.g. R1 or R2 (PS4), RB or RT (Xbox), R or ZL (Switch)

    // [Internal] Don't use directly! This is used internally to differentiate keyboard from gamepad inputs for behaviors that require to differentiate them.
    // Keyboard behavior that have no corresponding gamepad mapping (e.g. CTRL+TAB) will be directly reading from keyboard keys instead of io.nav_inputs[].
    ImGuiNavInput_KeyLeft_,      // Move left                                    // = Arrow keys
    ImGuiNavInput_KeyRight_,     // Move right
    ImGuiNavInput_KeyUp_,        // Move up
    ImGuiNavInput_KeyDown_,      // Move down
    ImGuiNavInput_COUNT
}

// Identify a mouse button.
// Those values are guaranteed to be stable and we frequently use 0/1 directly. Named enums provided for convenience.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgMouseButton
{
    Left = 0,
    Right = 1,
    Middle = 2,
    COUNT = 5
}

// Enumeration for GetMouseCursor()
// User code may request backend to display given cursor by calling SetMouseCursor(), which is why we have some cursors that are marked unused here
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum MouseCursor
{
    None,
    Arrow,
    TextInput,         // When hovering over InputText, etc.
    ResizeAll,         // (Unused by Dear ImGui functions)
    ResizeNS,          // When hovering over an horizontal border
    ResizeEW,          // When hovering over a vertical border or a column
    ResizeNESW,        // When hovering over the bottom-left corner of a window
    ResizeNWSE,        // When hovering over the bottom-right corner of a window
    Hand,              // (Unused by Dear ImGui functions. Use for e.g. hyperlinks)
    NotAllowed,        // When hovering something with disallowed interaction. Usually a crossed circle.
}

pub const MouseButtonLeft: i32         = 0;

pub const MouseButtonDefault: i32      = 1;

// Debug options
// #define IMGUI_DEBUG_NAV_SCORING     0   // Display navigation scoring preview when hovering items. Display last moving direction matches when holding CTRL
pub const IMGUI_DEBUG_NAV_SCORING: bool = false;

// #define IMGUI_DEBUG_NAV_RECTS       0   // Display the reference navigation rectangle for each window
pub const IMGUI_DEBUG_NAV_RECTS: bool = false;

// static const float WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER    = 2.00;    // Lock scrolled window (so it doesn't pick child windows that are scrolling through) for a certain time, unless mouse moved.
pub const WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER: f32 = 2.00;

// [Internal] Storage used by IsKeyDown(), IsKeyPressed() etc functions.
// If prior to 1.87 you used io.KeysDownDuration[] (which was marked as internal), you should use GetKeyData(key)->down_duration and not io.keys_data[key]->down_duration.
pub struct DimgKeyData
{
    pub down: bool,               // True for if key is down
    pub down_duration: f32,      // Duration the key has been down (<0.0: not pressed, 0.0: just pressed, >0.0: time held)
    pub down_duration_prev: f32,  // Last frame duration the key has been down
    pub analog_value: f32,       // 0.0..1.0 for gamepad values
}

// Include imgui_user.h at the end of imgui.h (convenient for user to only explicitly include vanilla imgui.h)
// #ifdef IMGUI_INCLUDE_IMGUI_USER_H
// #include "imgui_user.h"
// #endif
//
// #endif // #ifndef IMGUI_DISABLE
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum NavLayer
{
    Main,    // Main scrolling layer
    Menu,    // Menu layer (access with Alt/ImGuiNavInput_Menu)

}
