use std::collections::HashSet;
use crate::clipboard::{get_clipboard_text_fn_dflt_impl, set_clipboard_text_fn_dflt_impl};
use crate::config::{BackendFlags, ConfigFlags};
use crate::context::Context;
use crate::font::Font;
use crate::font_atlas::FontAtlas;
use crate::input::{DimgInputEventType, InputSource, DimgKey, DimgKeyData, ModFlags};
use crate::input_event::InputEvent;
use crate::text::IM_UNICODE_CODEPOINT_INVALID;
use crate::types::{Id32, DimgWchar};
use crate::vectors::two_d::Vector2D;

#[derive(Debug,Default,Clone)]
pub struct Io {
    //------------------------------------------------------------------
    // Configuration                            // Default value
    //------------------------------------------------------------------

    pub config_flags: HashSet<ConfigFlags>,
    // = 0              // See ImGuiConfigFlags_ enum. Set by user/application. Gamepad/keyboard navigation options, etc.
    pub backend_flags: HashSet<BackendFlags>,
    // = 0              // See ImGuiBackendFlags_ enum. Set by backend (imgui_impl_xxx files or custom backend) to communicate features supported by the backend.
    pub display_size: Vector2D,
    // <unset>          // Main display size, in pixels (generally == GetMainViewport()->size). May change every frame.
    pub delta_time: f32,
    // = 1.0/60.0     // time elapsed since last frame, in seconds. May change every frame.
    pub ini_saving_rate: f32,
    // = 5.0           // Minimum time between saving positions/sizes to .ini file, in seconds.
    pub ini_filename: String,
    // const char* ini_filename;                    // = "imgui.ini"    // Path to .ini file (important: default "imgui.ini" is relative to current working dir!). Set NULL to disable automatic .ini loading/saving or if you want to manually call LoadIniSettingsXXX() / SaveIniSettingsXXX() functions.
    pub log_filename: String,
    // const char* log_filename;                    // = "imgui_log.txt"// Path to .log file (default parameter to ImGui::LogToFile when no file is specified).
    pub mouse_double_click_time: f32,
    // = 0.30          // time for a double-click, in seconds.
    pub mouse_double_click_max_dist: f32,
    // = 6.0           // Distance threshold to stay in to validate a double-click, in pixels.
    pub mouse_drag_threshold: f32,
    // = 6.0           // Distance threshold before considering we are dragging.
    pub key_repeat_delay: f32,
    // = 0.250         // When holding a key/button, time before it starts repeating, in seconds (for buttons in Repeat mode, etc.).
    pub key_repeat_rate: f32,
    // = 0.050         // When holding a key/button, rate at which it repeats, in seconds.
    pub user_data: Vec<u8>,
    // void*       user_data;                       // = NULL           // Store your own data for retrieval by callbacks.
    pub fonts: Vec<FontAtlas>,
    // ImFontAtlas*fonts;                          // <auto>           // font atlas: load, rasterize and pack one or more fonts into a single texture.
    pub font_global_scale: f32,
    // = 1.0           // Global scale all fonts
    pub font_allow_user_scaling: bool,
    // = false          // Allow user scaling text of individual window with CTRL+Wheel.
    pub font_default: Font,
    // ImFont*     font_default;                    // = NULL           // font to use on NewFrame(). Use NULL to uses fonts->fonts[0].
    pub display_framebuffer_scale: Vector2D,        // = (1, 1)         // For retina display or other situations where window coordinates are different from framebuffer coordinates. This generally ends up in ImDrawData::framebuffer_scale.

    // Docking options (when ImGuiConfigFlags_DockingEnable is set)
    pub config_docking_no_split: bool,
    // = false          // Simplified docking mode: disable window splitting, so docking is limited to merging multiple windows together into tab-bars.
    pub config_docking_with_shift: bool,
    // = false          // Enable docking with holding Shift key (reduce visual noise, allows dropping in wider space)
    pub config_docking_always_tab_bar: bool,
    // = false          // [BETA] [FIXME: This currently creates regression with auto-sizing and general overhead] Make every single floating window display within a docking node.
    pub config_docking_transparent_payload: bool,// = false          // [BETA] Make window or viewport transparent when docking and only display docking boxes on the target viewport. Useful if rendering of multiple viewport cannot be synced. Best used with config_viewports_no_auto_merge.

    // viewport options (when ConfigFlags::ViewportsEnable is set)
    pub config_viewports_no_auto_merge: bool,
    // = false;         // Set to make all floating imgui windows always create their own viewport. Otherwise, they are merged into the main host viewports when overlapping it. May also set ImGuiViewportFlags_NoAutoMerge on individual viewport.
    pub config_viewports_no_task_bar_icon: bool,
    // = false          // Disable default OS task bar icon flag for secondary viewports. When a viewport doesn't want a task bar icon, ImGuiViewportFlags_NoTaskBarIcon will be set on it.
    pub config_viewports_no_decoration: bool,
    // = true           // Disable default OS window decoration flag for secondary viewports. When a viewport doesn't want window decorations, ImGuiViewportFlags_NoDecoration will be set on it. Enabling decoration can create subsequent issues at OS levels (e.g. minimum window size).
    pub config_viewports_no_default_parent: bool, // = false          // Disable default OS parenting to main viewport for secondary viewports. By default, viewports are marked with parent_viewport_id = <main_viewport>, expecting the platform backend to setup a parent/child relationship between the OS windows (some backend may ignore this). Set to true if you want the default to be 0, then all viewports will be top-level OS windows.

    // Miscellaneous options
    pub mouse_draw_cursor: bool,
    // = false          // Request ImGui to draw a mouse cursor for you (if you are on a platform without a mouse cursor). Cannot be easily renamed to 'io.ConfigXXX' because this is frequently used by backend implementations.
    pub config_mac_osxbehaviors: bool,
    // = defined(__APPLE__) // OS x style: Text editing cursor movement using Alt instead of Ctrl, Shortcuts using Cmd/Super instead of Ctrl, Line/Text Start and End using Cmd+Arrows instead of Home/End, Double click selects by word instead of selecting whole text, Multi-selection in lists uses Cmd/Super instead of Ctrl.
    pub config_input_trickle_event_queue: bool,
    // = true           // Enable input queue trickling: some types of events submitted during the same frame (e.g. button down + up) will be spread over multiple frames, improving interactions with low framerates.
    pub config_input_text_cursor_blink: bool,
    // = true           // Enable blinking cursor (optional as some users consider it to be distracting).
    pub config_drag_click_to_input_text: bool,
    // = false          // [BETA] Enable turning DragXXX widgets into text input with a simple mouse click-release (without moving). Not desirable on devices without a keyboard.
    pub config_windows_resize_from_edges: bool,
    // = true           // Enable resizing of windows from their edges and from the lower-left corner. This requires (io.backend_flags & ImGuiBackendFlags_HasMouseCursors) because it needs mouse cursor feedback. (This used to be a per-window ImGuiWindowFlags_ResizeFromAnySide flag)
    pub config_windows_move_from_title_bar_only: bool,
    // = false       // Enable allowing to move windows only when clicking on their title bar. Does not apply to windows without a title bar.
    pub config_memory_compact_timer: f32,      // = 60.0          // Timer (in seconds) to free transient windows/tables memory buffers when unused. Set to -1.0 to disable.

    //------------------------------------------------------------------
    // Platform Functions
    // (the imgui_impl_xxxx backend files are setting those up for you)
    //------------------------------------------------------------------

    // Optional: Platform/Renderer backend name (informational only! will be displayed in About window) + User data for backend/wrappers to store their own stuff.
    // const char* backend_platform_name;            // = NULL
    pub backend_platform_name: String,
    // const char* backend_renderer_name;            // = NULL
    pub backend_renderer_name: String,
    // void*       backend_platform_user_data;        // = NULL           // User data for platform backend
    pub backend_platform_user_data: Vec<u8>,
    // void*       backend_renderer_user_data;        // = NULL           // User data for renderer backend
    pub backend_renderer_user_data: Vec<u8>,
    // void*       backend_language_user_data;        // = NULL           // User data for non C++ programming language backend
    pub backend_language_user_data: Vec<u8>,

    // Optional: Access OS clipboard
    // (default to use native Win32 clipboard on windows, otherwise uses a private clipboard. Override to access OS clipboard on other architectures)
    // const char* (*GetClipboardTextFn)(void* user_data);
    // void        (*set_clipboard_text_fn)(void* user_data, const char* text);
    // void*       clipboard_user_data;

    // Optional: Notify OS Input Method Editor of the screen position of your cursor for text input position (e.g. when using Japanese/Chinese IME on windows)
    // (default to use native imm32 api on windows)
    // void        (*SetPlatformImeDataFn)(ImGuiViewport* viewport, ImGuiPlatformImeData* data);
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     void*       ime_window_handle;                // = NULL           // [Obsolete] Set ImGuiViewport::platform_handle_raw instead. Set this to your HWND to get automatic IME cursor positioning.
    pub ime_window_handle: Id32,
// #else
//     void*       _UnusedPadding;                                     // Unused field to keep data structure the same size.
// #endif

    //------------------------------------------------------------------
    // Input - Call before calling NewFrame()
    //------------------------------------------------------------------

    // // Input Functions
    //  void  add_key_event(ImGuiKey key, bool down);                   // Queue a new key down/up event. Key should be "translated" (as in, generally ImGuiKey_A matches the key end-user would use to emit an 'A' character)
    //  void  add_key_analog_event(ImGuiKey key, bool down, float v);    // Queue a new key down/up event for analog values (e.g. ImGuiKey_Gamepad_ values). Dead-zones should be handled by the backend.
    //  void  add_mouse_pos_event(float x, float y);                     // Queue a mouse position update. Use -FLT_MAX,-FLT_MAX to signify no mouse (e.g. app not focused and not hovered)
    //  void  add_mouse_button_event(int button, bool down);             // Queue a mouse button change
    //  void  add_mouse_wheel_event(float wh_x, float wh_y);             // Queue a mouse wheel update
    //  void  add_mouse_viewport_event(ImGuiID id);                      // Queue a mouse hovered viewport. Requires backend to set ImGuiBackendFlags_HasMouseHoveredViewport to call this (for multi-viewport support).
    //  void  add_focus_event(bool focused);                            // Queue a gain/loss of focus for the application (generally based on OS/platform focus of your window)
    //  void  add_input_character(unsigned int c);                      // Queue a new character input
    //  void  add_input_character_utf16(ImWchar16 c);                    // Queue a new character input from an UTF-16 character, it can be a surrogate
    //  void  add_input_characters_utf8(const char* str);                // Queue a new characters input from an UTF-8 string
    // 
    //  void  set_key_event_native_data(ImGuiKey key, int native_keycode, int native_scancode, int native_legacy_index = -1); // [Optional] Specify index for legacy <1.87 IsKeyXXX() functions with native indices + specify native keycode, scancode.
    //  void  set_app_accepting_events(bool accepting_events);           // Set master flag for accepting key/mouse/text events (default to true). Useful if you have native dialog boxes that are interrupting your application loop/refresh, and you want to disable events being queued while your app is frozen.
    //  void  clear_input_characters();                                 // [Internal] clear the text input buffer manually
    //  void  clear_input_keys();                                       // [Internal] Release all keys

    //------------------------------------------------------------------
    // Output - Updated by NewFrame() or EndFrame()/Render()
    // (when reading from the io.want_capture_mouse, io.want_capture_keyboard flags to dispatch your inputs, it is
    //  generally easier and more correct to use their state BEFORE calling NewFrame(). See FAQ for details!)
    //------------------------------------------------------------------

    pub want_capture_mouse: bool,
    // Set when Dear ImGui will use mouse inputs, in this case do not dispatch them to your main game/application (either way, always pass on mouse inputs to imgui). (e.g. unclicked mouse is hovering over an imgui window, widget is active, mouse was clicked over an imgui window, etc.).
    pub want_capture_keyboard: bool,
    // Set when Dear ImGui will use keyboard inputs, in this case do not dispatch them to your main game/application (either way, always pass keyboard inputs to imgui). (e.g. InputText active, or an imgui window is focused and navigation is enabled, etc.).
    pub want_text_input: bool,
    // Mobile/console: when set, you may display an on-screen keyboard. This is set by Dear ImGui when it wants textual keyboard input to happen (e.g. when a InputText widget is active).
    pub want_set_mouse_pos: bool,
    // mouse_pos has been altered, backend should reposition mouse on next frame. Rarely used! Set only when ImGuiConfigFlags_NavEnableSetMousePos flag is enabled.
    pub want_save_ini_settings: bool,
    // When manual .ini load/save is active (io.ini_filename == NULL), this will be set to notify your application that you can call SaveIniSettingsToMemory() and save yourself. Important: clear io.want_save_ini_settings yourself after saving!
    pub nav_active: bool,
    // Keyboard/Gamepad navigation is currently allowed (will handle ImGuiKey_NavXXX events) = a window is focused and it doesn't use the ImGuiWindowFlags_NoNavInputs flag.
    pub nav_visible: bool,
    // Keyboard/Gamepad navigation is visible and allowed (will handle ImGuiKey_NavXXX events).
    pub framerate: f32,
    // Rough estimate of application framerate, in frame per second. Solely for convenience. Rolling average estimation based on io.delta_time over 120 frames.
    pub metrics_render_vertices: i32,
    // Vertices output during last call to Render()
    pub metrics_render_indices: i32,
    // Indices output during last call to Render() = number of triangles * 3
    pub metrics_render_windows: i32,
    // Number of visible windows
    pub metrics_active_windows: i32,
    // Number of active windows
    pub metrics_active_allocations: i32,
    // Number of active allocations, updated by MemAlloc/MemFree based on current context. May be off if you have multiple imgui contexts.
    pub mouse_delta: Vector2D,                         // Mouse delta. Note that this is zero if either current or previous position are invalid (-FLT_MAX,-FLT_MAX), so a disappearing/reappearing mouse won't have a huge delta.

    // Legacy: before 1.87, we required backend to fill io.key_map[] (imgui->native map) during initialization and io.keys_down[] (native indices) every frame.
    // This is still temporarily supported as a legacy feature. However the new preferred scheme is for backend to call io.add_key_event().
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     int         KeyMap[ImGuiKey_COUNT];             // [LEGACY] Input: map of indices into the keys_down[512] entries array which represent your "native" keyboard state. The first 512 are now unused and should be kept zero. Legacy backend will write into KeyMap[] using ImGuiKey_ indices which are always >512.
//     bool        keys_down[ImGuiKey_COUNT];           // [LEGACY] Input: Keyboard keys that are pressed (ideally left in the "native" order your engine has access to keyboard keys, so you can use your own defines/enums for keys). This used to be [512] sized. It is now ImGuiKey_COUNT to allow legacy io.keys_down[GetKeyIndex(...)] to work without an overflow.
// #endif

    //------------------------------------------------------------------
    // [Internal] Dear ImGui will maintain those fields. Forward compatibility not guaranteed!
    //------------------------------------------------------------------

    // Main Input state
    // (this block used to be written by backend, since 1.87 it is best to NOT write to those directly, call the AddXXX functions above instead)
    // (reading from those variables is fair game, as they are extremely unlikely to be moving anywhere)
    pub mouse_pos: Vector2D,
    // Mouse position, in pixels. Set to Vector2D(-FLT_MAX, -FLT_MAX) if mouse is unavailable (on another screen, etc.)
    pub mouse_down: [bool; 5],
    // bool        mouse_down[5];                       // Mouse buttons: 0=left, 1=right, 2=middle + extras (ImGuiMouseButton_COUNT == 5). Dear ImGui mostly uses left and right buttons. Others buttons allows us to track if the mouse is being used by your application + available to user as a convenience via IsMouse** API.
    pub mouse_wheel: f32,
    // Mouse wheel Vertical: 1 unit scrolls about 5 lines text.
    pub mouse_wheel_h: f32,
    // Mouse wheel Horizontal. Most users don't have a mouse with an horizontal wheel, may not be filled by all backends.
    pub mouse_hovered_viewport: Id32,
    // (Optional) Modify using io.add_mouse_viewport_event(). With multi-viewports: viewport the OS mouse is hovering. If possible _IGNORING_ viewports with the ViewportFlags::NoInputs flag is much better (few backends can handle that). Set io.backend_flags |= ImGuiBackendFlags_HasMouseHoveredViewport if you can provide this info. If you don't imgui will infer the value using the rectangles and last focused time of the viewports it knows about (ignoring other OS windows).
    pub key_ctrl: bool,
    // Keyboard modifier down: Control
    pub key_shift: bool,
    // Keyboard modifier down: Shift
    pub key_alt: bool,
    // Keyboard modifier down: Alt
    pub key_super: bool,
    // Keyboard modifier down: Cmd/Super/windows
    // float       nav_inputs[ImGuiNavInput_COUNT];     // Gamepad inputs. Cleared back to zero by EndFrame(). Keyboard keys will be auto-mapped and be written here by NewFrame().
    pub nav_inputs: Vec<f32>,

    // Other state maintained from data above + io function calls
    pub key_mods: ModFlags,
    // Key mods flags (same as io.key_ctrl/key_shift/key_alt/key_super but merged into flags), updated by NewFrame()
    pub keys_data: Vec<DimgKeyData>,
    // Key state for all known keys. Use IsKeyXXX() functions to access this.
    pub want_capture_mouse_unless_popup_close: bool,
    // Alternative to want_capture_mouse: (want_capture_mouse == true && want_capture_mouse_unless_popup_close == false) when a click over void is expected to close a popup.
    pub mouse_pos_prev: Vector2D,
    // Previous mouse position (note that mouse_delta is not necessary == mouse_pos-mouse_pos_prev, in case either position is invalid)
    pub mouse_clicked_pos: [Vector2D; 5],
    // Position at time of clicking
    pub mouse_clicked_time: [f64; 5],
    // time of last click (used to figure out double-click)
    pub mouse_clicked: [bool; 5],
    // Mouse button went from !down to down (same as mouse_clicked_count[x] != 0)
    pub mouse_double_clicked: [bool; 5],
    // Has mouse button been double-clicked? (same as mouse_clicked_count[x] == 2)
    pub mouse_clicked_count: [u16; 5],
    // == 0 (not clicked), == 1 (same as mouse_clicked[]), == 2 (double-clicked), == 3 (triple-clicked) etc. when going from !down to down
    pub mouse_clicked_last_count: [u16; 5],
    // count successive number of clicks. Stays valid after mouse release. Reset after another click is done.
    pub mouse_released: [bool; 5],
    // Mouse button went from down to !down
    pub mouse_down_owned: [bool; 5],
    // Track if button was clicked inside a dear imgui window or over void blocked by a popup. We don't request mouse capture from the application if click started outside ImGui bounds.
    pub mouse_down_owned_unless_popup_close: [bool; 5],
    // Track if button was clicked inside a dear imgui window.
    pub mouse_down_duration: [f32; 5],
    // Duration the mouse button has been down (0.0 == just clicked)
    pub mouse_down_duration_prev: [f32; 5],
    // Previous time the mouse button has been down
    pub mouse_drag_max_distance_abs: [Vector2D; 5],
    // Maximum distance, absolute, on each axis, of how much mouse has traveled from the clicking point
    pub mouse_drag_max_distance_sqr: [f32; 5],
    // Squared maximum distance of how much mouse has traveled from the clicking point (used for moving thresholds)
    pub nav_inputs_down_duration: Vec<f32>,
    pub nav_inputs_down_duration_prev: Vec<f32>,
    pub pen_pressure: f32,
    // Touch/Pen pressure (0.0 to 1.0, should be >0.0 only when mouse_down[0] == true). Helper storage currently unused by Dear ImGui.
    pub app_focus_lost: bool,
    // Only modify via add_focus_event()
    pub app_accepting_events: bool,
    // Only modify via set_app_accepting_events()
    pub backend_using_legacy_key_arrays: i8,
    // -1: unknown, 0: using add_key_event(), 1: using legacy io.keys_down[]
    pub backend_using_legacy_nav_input_array: bool,
    // 0: using add_key_analog_event(), 1: writing to legacy io.nav_inputs[] directly
    pub input_queue_surrogate: Vec<u8>,
    // For add_input_character_utf16()
    pub input_queue_characters: Vec<u8>,         // Queue of _characters_ input (obtained by platform backend). Fill using add_input_character() helper.

    //    ImGuiIO();
}

impl Io {
    pub fn new() -> Self {
        let mut out = Self { ..Default::default() };

        // Most fields are initialized with zero
        // memset(this, 0, sizeof(*this));
        // IM_STATIC_ASSERT(IM_ARRAYSIZE(ImGuiIO::mouse_down) == ImGuiMouseButton_COUNT && IM_ARRAYSIZE(ImGuiIO::mouse_clicked) == ImGuiMouseButton_COUNT);

        // Settings
        out.config_flags.insert(ConfigFlags::None);
        out.backend_flags.insert(BackendFlags::None);
        out.display_size = Vector2D::new(-1.0, -1.0);
        out.delta_time = 1.0 / 60.0;
        out.ini_saving_rate = 5.0;
        out.ini_filename = "imgui.ini".to_string(); // Important: "imgui.ini" is relative to current working dir, most apps will want to lock this to an absolute path (e.g. same path as executables).
        out.log_filename = "imgui_log.txt".to_string();
        out.mouse_double_click_time = 0.30;
        out.mouse_double_click_max_dist = 6.0;
        // # ifndef
        // IMGUI_DISABLE_OBSOLETE_KEYIO
        // for (int i = 0; i < ImGuiKey_COUNT; i+ +)
        // KeyMap[i] = -1;
        // # endif
        out.key_repeat_delay = 0.275;
        out.key_repeat_rate = 0.050;
        // out.user_data = NULL;

        // out.fonts = NULL;
        out.font_global_scale = 1.0;
        // out.font_default = NULL;
        out.font_allow_user_scaling = false;
        out.display_framebuffer_scale = Vector2D::new(1.0, 1.0);

        // Docking options (when ImGuiConfigFlags_DockingEnable is set)
        out.config_docking_no_split = false;
        out.config_docking_with_shift = false;
        out.config_docking_always_tab_bar = false;
        out.config_docking_transparent_payload = false;

        // viewport options (when ConfigFlags::ViewportsEnable is set)
        out.config_viewports_no_auto_merge = false;
        out.config_viewports_no_task_bar_icon = false;
        out.config_viewports_no_decoration = true;
        out.config_viewports_no_default_parent = false;

        // Miscellaneous options
        out.mouse_draw_cursor = false;
        // # ifdef
        // __APPLE__
        // config_mac_osxbehaviors = true;  // Set Mac OS x style defaults based on __APPLE__ compile time flag # else
        // config_mac_osxbehaviors = false;
        // # endif
        out.config_input_trickle_event_queue = true;
        out.config_input_text_cursor_blink = true;
        out.config_windows_resize_from_edges = true;
        out.config_windows_move_from_title_bar_only = false;
        out.config_memory_compact_timer = 60.0;

        // Platform Functions
        // out.backend_platform_name = backend_renderer_name = NULL;
        out.backend_platform_name = "".to_string();
        out.backend_renderer_name = "".to_string();
        // out.backend_platform_user_data = backend_renderer_user_data = backend_language_user_data = NULL;
        out.backend_platform_user_data = Vec::new();
        out.get_clipboard_text_fn = get_clipboard_text_fn_dflt_impl;   // Platform dependent default implementations
        out.set_clipboard_text_fn = set_clipboard_text_fn_dflt_impl;
        out.clipboard_user_data = Vec::new();
        out.set_platform_ime_data_fn = SetPlatformImeDataFn_DefaultImpl;

        // Input (NB: we already have memset zero the entire structure!)
        out.mouse_pos = Vector2D::new(-f32::MAX, -f32::MAX);
        out.mouse_pos_prev = Vector2D::new(-f32::MAX, -f32::MAX);
        out.mouse_drag_threshold = 6.0;
        // for (int i = 0; i < IM_ARRAYSIZE(mouse_down_duration); i+ +) mouse_down_duration[i] = mouse_down_duration_prev[i] = -1.0;
        out.mouse_down_duration_prev = [-1.0; 5];
        out.mouse_down_duration = [-1.0; 5];
        // TODO
        // for (int i = 0; i < IM_ARRAYSIZE(keys_data); i+ +) { keys_data[i].down_duration = keys_data[i].down_duration_prev = -1.0; }
        //out.keys_data =

        // for (int i = 0; i < IM_ARRAYSIZE(nav_inputs_down_duration); i+ +) nav_inputs_down_duration[i] = -1.0;
        // out.nav_inputs_down_duration
        out.app_accepting_events = true;
        out.backend_using_legacy_key_arrays = -1;
        out.backend_using_legacy_nav_input_array = true; // assume using legacy array until proven wrong
        out
    }


    // Input Functions
    //  void  add_key_event(ImGuiKey key, bool down);                   // Queue a new key down/up event. Key should be "translated" (as in, generally ImGuiKey_A matches the key end-user would use to emit an 'A' character)
    pub fn add_key_event(&mut self, key: &DimgKey, down: bool) {
        if !self.app_accepting_events {
            return;
        }
        self.add_key_analog_event(key, down, if down { 1.0 } else { 0.0 });
    }
    //  void  add_key_analog_event(ImGuiKey key, bool down, float v);    // Queue a new key down/up event for analog values (e.g. ImGuiKey_Gamepad_ values). Dead-zones should be handled by the backend.
    // Queue a new key down/up event.
// - ImGuiKey key:       Translated key (as in, generally ImGuiKey_A matches the key end-user would use to emit an 'A' character)
// - bool down:          Is the key down? use false to signify a key release.
// - float analog_value: 0.0..1.0
    pub fn add_key_analog_event(&mut self, ctx: &mut Context, key: &DimgKey, down: bool, v: f32) {
        //if (e->down) { IMGUI_DEBUG_LOG_IO("add_key_event() Key='%s' %d, NativeKeycode = %d, NativeScancode = %d\n", ImGui::GetKeyName(e->Key), e->down, e->NativeKeycode, e->NativeScancode); }
        if key == DimgKey::None || !self.app_accepting_events {
            return;
        }
        //ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");
        // IM_ASSERT(ImGui::IsNamedKey(key)); // Backend needs to pass a valid ImGuiKey_ constant. 0..511 values are legacy native key codes which are not accepted by this API.

        // Verify that backend isn't mixing up using new io.add_key_event() api and old io.keys_down[] + io.key_map[] data.
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     IM_ASSERT((backend_using_legacy_key_arrays == -1 || backend_using_legacy_key_arrays == 0) && "Backend needs to either only use io.add_key_event(), either only fill legacy io.keys_down[] + io.key_map[]. Not both!");
//     if (backend_using_legacy_key_arrays == -1)
//         for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n++)
//             IM_ASSERT(KeyMap[n] == -1 && "Backend needs to either only use io.add_key_event(), either only fill legacy io.keys_down[] + io.key_map[]. Not both!");
//     backend_using_legacy_key_arrays = 0;
// #endif
        if self.is_gamepad_key(key) {
            self.backend_using_legacy_nav_input_array = false;
        }

        // Partial filter of duplicates (not strictly needed, but makes data neater in particular for key mods and gamepad values which are most commonly spmamed)
        let key_data = self.get_key_data(key);
        if key_data.down == down && key_data.analog_value == analog_value {
            let mut found = false;
            // for (int n = g.input_events_queue.size - 1; n >= 0 && !found; n--){
            let mut n = ctx.InputEventsQueue.size - 1;
            while n >= 0 && !found {
                if ctx.InputEventsQueue[n].Type == DimgInputEventType::Key && ctx.InputEventsQueue[n].Key.Key == key {
                    found = true;
                }
            }
            if !found {
                return;
            }
        }

        // Add event
        let mut e: InputEvent = InputEvent::new();
        e.input_event_type = DimgInputEventType::Key;
        e.source = if self.is_gamepad_key(key) {
            InputSource::Gamepad
        } else { InputSource::Keyboard };
        e.Key.Key = key;
        e.Key.down = down;
        e.Key.analog_value = analog_value;
        ctx.InputEventsQueue.push_back(e);
    }
    //  void  add_mouse_pos_event(float x, float y);                     // Queue a mouse position update. Use -FLT_MAX,-FLT_MAX to signify no mouse (e.g. app not focused and not hovered)
    pub fn add_mouse_pos_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");
        if !self.app_accepting_events {
            return;
        }

        let mut e = InputEvent::new();
        e.input_event_type = DimgInputEventType::MousePos;
        e.source = InputSource::Mouse;
        e.MousePos.PosX = x;
        e.MousePos.PosY = y;
        ctx.InputEventsQueue.push_back(e);
    }
    //  void  add_mouse_button_event(int button, bool down);             // Queue a mouse button change
    pub fn add_mouse_button_event(&mut self, ctx: &mut Context, button: i32, down: bool) {
        // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");
        // IM_ASSERT(mouse_button >= 0 && mouse_button < ImGuiMouseButton_COUNT);
        if !self.app_accepting_events {
            return;
        }

        let mut e = InputEvent::new();
        e.input_event_type = DimgInputEventType::MouseButton;
        e.source = InputSource::Mouse;
        e.MouseButton.Button = button;
        e.MouseButton.down = down;
        ctx.InputEventsQueue.push_back(e);
    }
    //  void  add_mouse_wheel_event(float wh_x, float wh_y);             // Queue a mouse wheel update
    pub fn add_mouse_wheel_event(&mut self, ctx: &mut Context, wheel_x: f32, wheel_y: f32) {
        // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");
        if (wheel_x == 0.0 && wheel_y == 0.0) || !self.app_accepting_events {
            return;
        }

        //DimgInputEvent e;
        let mut e = InputEvent::new();
        e.input_event_type = DimgInputEventType::mouse_wheel;
        e.source = InputSource::Mouse;
        e.mouse_wheel.WheelX = wheel_x;
        e.mouse_wheel.WheelY = wheel_y;
        ctx.InputEventsQueue.push_back(e);
    }
    //  void  add_mouse_viewport_event(ImGuiID id);                      // Queue a mouse hovered viewport. Requires backend to set ImGuiBackendFlags_HasMouseHoveredViewport to call this (for multi-viewport support).
    pub fn add_mouse_viewport_event(&mut self, ctx: &mut Context, id: Id32) {
        // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");
        // IM_ASSERT(g.io.backend_flags & ImGuiBackendFlags_HasMouseHoveredViewport);

        // DimgInputEvent e;
        let mut e = InputEvent::new();
        e.input_event_type = DimgInputEventType::mouse_viewport;
        e.source = InputSource::Mouse;
        e.mouse_viewport.HoveredViewportID = id;
        ctx.InputEventsQueue.push_back(e);
    }
    //  void  add_focus_event(bool focused);                            // Queue a gain/loss of focus for the application (generally based on OS/platform focus of your window)
    pub fn add_focus_event(&mut self, ctx: &mut Context, focused: bool) {
        // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");

        // DimgInputEvent e;
        let mut e = InputEvent::new();
        e.input_event_type = DimgInputEventType::Focus;
        e.AppFocused.Focused = focused;
        ctx.InputEventsQueue.push_back(e);
    }
    //  void  add_input_character(unsigned int c);                      // Queue a new character input

    // Pass in translated ASCII characters for text input.
// - with glfw you can get those from the callback set in glfwSetCharCallback()
// - on windows you can get those using ToAscii+keyboard state, or via the WM_CHAR message
// FIXME: Should in theory be called "AddCharacterEvent()" to be consistent with new API
    pub fn add_input_character(&mut self, c: u32, ctx: &mut Context) {
        // ImGuiContext & g = *;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.".to_string());
        if c == 0 || !self.app_accepting_events {
            return;
        }
        let e: InputEvent = InputEvent {
            input_event_type: DimgInputEventType::Text,
            source: InputSource::Keyboard,
            val: DimgInputEventVal::new(),
            added_byt_test_engine: false
        };
        ctx.InputEventsQueue.push_back(e);
    }
    //  void  add_input_character_utf16(ImWchar16 c);                    // Queue a new character input from an UTF-16 character, it can be a surrogate
    // UTF16 strings use surrogate pairs to encode codepoints >= 0x10000, so
// we should save the high surrogate.
    pub fn add_input_character_utf16(&mut self, c: DimgWchar) {
        if (c == 0 && self.input_queue_surrogate.is_empty()) || !self.app_accepting_events {
            return;
        }

        if (c & 0xFC00) == 0xD800 { // High surrogate, must save {
            if self.input_queue_surrogate.is_empty() == false {
                self.add_input_character(IM_UNICODE_CODEPOINT_INVALID);
            }
            self.input_queue_surrogate = c;
            return;
        }

        //ImWchar cp = c;
        let cp: DimgWchar = c;
        if self.input_queue_surrogate != 0 {
            if (c & 0xFC00) != 0xDC00 { // Invalid low surrogate {
                self.add_input_character(IM_UNICODE_CODEPOINT_INVALID);
            } else {
// #if IM_UNICODE_CODEPOINT_MAX == 0xFFFF
//             cp = IM_UNICODE_CODEPOINT_INVALID; // codepoint will not fit in ImWchar
// #else
//             cp = (ImWchar)(((input_queue_surrogate - 0xD800) << 10) + (c - 0xDC00) + 0x10000);
// #endif
            }

            self.input_queue_surrogate.clear();
        }
        self.add_input_character(cp);
    }

    //  void  add_input_characters_utf8(const char* str);                // Queue a new characters input from an UTF-8 string
    pub fn add_input_characters_utf8(&mut self, in_str: &String) {
        // if !self.app_accepting_events {
        //     return;
        // }
        //
        // while *self.utf8_chars != 0 {
        //     let c: u32 = 0;
        //     utf8_chars += DimgTextCharFromUtf8(&c, utf8_chars, NULL);
        //     if c != 0 {
        //         self.add_input_character(c);
        //     }
        // }
        // TODO
    }

    //  void  set_key_event_native_data(ImGuiKey key, int native_keycode, int native_scancode, int native_legacy_index = -1); // [Optional] Specify index for legacy <1.87 IsKeyXXX() functions with native indices + specify native keycode, scancode.
    // [Optional] Call after add_key_event().
// Specify native keycode, scancode + Specify index for legacy <1.87 IsKeyXXX() functions with native indices.
// If you are writing a backend in 2022 or don't use IsKeyXXX() with native values that are not ImGuiKey values, you can avoid calling this.
    pub fn set_key_event_native_data(key: DimgKey, native_keycode: i32, native_scancode: i32, native_legacy_index: i32) {
        if key == DimgKey::None {
            return;
        }
        // IM_ASSERT(ImGui::IsNamedKey(key)); // >= 512
        // IM_ASSERT(native_legacy_index == -1 || ImGui::IsLegacyKey(native_legacy_index)); // >= 0 && <= 511
        // IM_UNUSED(native_keycode);  // Yet unused
        // IM_UNUSED(native_scancode); // Yet unused

        // build native->imgui map so old user code can still call key functions with native 0..511 values.
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     const int legacy_key = (native_legacy_index != -1) ? native_legacy_index : native_keycode;
//     if (!ImGui::IsLegacyKey(legacy_key))
//         return;
//     KeyMap[legacy_key] = key;
//     KeyMap[key] = legacy_key;
// #else
//     IM_UNUSED(key);
//     IM_UNUSED(native_legacy_index);
// #endif
    }
    //  void  set_app_accepting_events(bool accepting_events);           // Set master flag for accepting key/mouse/text events (default to true). Useful if you have native dialog boxes that are interrupting your application loop/refresh, and you want to disable events being queued while your app is frozen.
    // Set master flag for accepting key/mouse/text events (default to true). Useful if you have native dialog boxes that are interrupting your application loop/refresh, and you want to disable events being queued while your app is frozen.
    pub fn set_app_accepting_events(&mut self, accepting_events: bool) {
        self.app_accepting_events = accepting_events;
    }
    //  void  clear_input_characters();                                 // [Internal] clear the text input buffer manually
    pub fn clear_input_characters(&mut self) {
        self.input_queue_characters.resize(0, 0);
    }
    //  void  clear_input_keys();                                       // [Internal] Release all keys
    pub fn clear_input_keys(&mut self) {
        for n in 0..self.keys_data.len() {
            self.keys_data[n].down = false;
            self.keys_data[n].downDuration = -1.0;
            self.keys_data[n].downDurationPrev = -1.0;
        }
        self.key_ctrl = false;
        self.key_shift = false;
        self.key_alt = false;
        self.key_super = false;
        self.key_mods = ModFlags::None;
        // for (int n = 0; n < IM_ARRAYSIZE(nav_inputs_down_duration); n++)
        for n in 0..self.nav_inputs_down_duration.len() {
            self.nav_inputs_down_duration[n] = -1.0;
            self.nav_inputs_down_duration_prev[n] = -1.0;
        }
    }
}


// (Optional) Access via ImGui::GetPlatformIO()
#[derive(Debug,Clone,Default)]
pub struct PlatformIo
{
    //------------------------------------------------------------------
    // Input - Backend interface/functions + Monitor List
    //------------------------------------------------------------------

    // (Optional) Platform functions (e.g. Win32, GLFW, SDL2)
    // For reference, the second column shows which function are generally calling the Platform Functions:
    //   N = ImGui::NewFrame()                        ~ beginning of the dear imgui frame: read info from platform/OS windows (latest size/position)
    //   F = ImGui::Begin(), ImGui::EndFrame()        ~ during the dear imgui frame
    //   U = ImGui::UpdatePlatformWindows()           ~ after the dear imgui frame: create and update all platform/OS windows
    //   R = ImGui::RenderPlatformWindowsDefault()    ~ render
    //   D = ImGui::DestroyPlatformWindows()          ~ shutdown
    // The general idea is that NewFrame() we will read the current Platform/OS state, and UpdatePlatformWindows() will write to it.
    //
    // The functions are designed so we can mix and match 2 imgui_impl_xxxx files, one for the Platform (~window/input handling), one for Renderer.
    // Custom engine backends will often provide both Platform and Renderer interfaces and so may not need to use all functions.
    // Platform functions are typically called before their Renderer counterpart, apart from Destroy which are called the other way.

    // Platform function --------------------------------------------------- Called by -----


    // (Optional) Monitor list
    // - Updated by: app/backend. Update every frame to dynamically support changing monitor or DPI configuration.
    // - Used by: dear imgui to query DPI info, clamp popups/tooltips within same monitor and not have them straddle monitors.
    // ImVector<ImGuiPlatformMonitor>  Monitors;
    pub Monitors: Vec<ImGuiPlatformMonitor>,

    //------------------------------------------------------------------
    // Output - List of viewports to render into platform windows
    //------------------------------------------------------------------

    // viewports list (the list is updated by calling ImGui::EndFrame or ImGui::Render)
    // (in the future we will attempt to organize this feature to remove the need for a "main viewport")
    // ImVector<ImGuiViewport*>        viewports;                              // Main viewports, followed by all secondary viewports.
    pub Viewports: Vec<ImGuiViewport>,
    // ImGuiPlatformIO()               { memset(this, 0, sizeof(*this)); }     // Zero clear
}

impl PlatformIo {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    // void    (*Platform_CreateWindow)(ImGuiViewport* vp);                    // . . U . .  // Create a new platform window for the given viewport
    pub fn Platform_CreateWindow(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Platform_DestroyWindow)(ImGuiViewport* vp);                   // N . U . D  //
    pub fn Platform_DestroyWindow(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Platform_ShowWindow)(ImGuiViewport* vp);                      // . . U . .  // Newly created windows are initially hidden so set_window_pos/size/Title can be called on them before showing the window
    pub fn Platform_ShowWindow(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Platform_SetWindowPos)(ImGuiViewport* vp, Vector2D pos);        // . . U . .  // Set platform window position (given the upper-left corner of client area)
    pub fn Platform_SetWindowPos(&mut self, vp: &mut ImGuiViewport, pos: Vector2D) {
        todo!()
    }
    //     Vector2D  (*Platform_GetWindowPos)(ImGuiViewport* vp);                    // N . . . .  //
    pub fn Platform_GetWindowPos(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Platform_SetWindowSize)(ImGuiViewport* vp, Vector2D size);      // . . U . .  // Set platform window client area size (ignoring OS decorations such as OS title bar etc.)
    pub fn Platform_SetWindowSize(&mut self, vp: &mut ImGuiViewport, size: &Vector2D) {
        todo!()
    }
    //     Vector2D  (*Platform_GetWindowSize)(ImGuiViewport* vp);                   // N . . . .  // Get platform window client area size
    pub fn Platform_GetWindowSize(&mut self, vp: &mut ImGuiViewport) -> Vector2D {
        todo!()
    }
    //     void    (*Platform_SetWindowFocus)(ImGuiViewport* vp);                  // N . . . .  // Move window to front and set input focus
    pub fn Platform_SetWindowFocus(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }

    //     bool    (*Platform_GetWindowFocus)(ImGuiViewport* vp);                  // . . U . .  //
    pub fn Platform_GetWindowFocus(&mut self, vp: &mut ImGuiViewport) -> bool {
        todo!()
    }
    //     bool    (*Platform_GetWindowMinimized)(ImGuiViewport* vp);              // N . . . .  // Get platform window minimized state. When minimized, we generally won't attempt to get/set size and contents will be culled more easily
    pub fn Platform_GetWindowMinimized(&mut self, vp: &mut ImGuiViewport) -> bool {
        todo!()
    }
    //     void    (*Platform_SetWindowTitle)(ImGuiViewport* vp, const char* str); // . . U . .  // Set platform window title (given an UTF-8 string)
    pub fn Platform_SetWindowTitle(&mut self, vp: &mut ImGuiViewport, in_str: &String) {
        todo!()
    }
    //     void    (*Platform_SetWindowAlpha)(ImGuiViewport* vp, float alpha);     // . . U . .  // (Optional) Setup global transparency (not per-pixel transparency)
    pub fn Platform_SetWindowAlpha(&mut self, vp: &mut ImGuiViewport, alpha: f32) {
        todo!()
    }
    //     void    (*Platform_UpdateWindow)(ImGuiViewport* vp);                    // . . U . .  // (Optional) Called by UpdatePlatformWindows(). Optional hook to allow the platform backend from doing general book-keeping every frame.
    pub fn Platform_UpdateWindow(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Platform_RenderWindow)(ImGuiViewport* vp, void* render_arg);  // . . . R .  // (Optional) Main rendering (platform side! This is often unused, or just setting a "current" context for OpenGL bindings). 'render_arg' is the value passed to RenderPlatformWindowsDefault().
    pub fn Platform_RenderWindow(&mut self, vp: &mut ImGuiViewport, render_arg: *mut c_void) {
        todo!()
    }
    //     void    (*Platform_SwapBuffers)(ImGuiViewport* vp, void* render_arg);   // . . . R .  // (Optional) Call Present/SwapBuffers (platform side! This is often unused!). 'render_arg' is the value passed to RenderPlatformWindowsDefault().
    pub fn Platform_SwapBuffers(&mut self, vp: &mut ImGuiViewport, render_arg: *mut c_void) {
        todo!()
    }
    //     float   (*Platform_GetWindowDpiScale)(ImGuiViewport* vp);               // N . . . .  // (Optional) [BETA] FIXME-DPI: DPI handling: Return DPI scale for this viewport. 1.0 = 96 DPI.
    pub fn Platform_GetWindowDpiScale(&mut self, vp: &mut ImGuiViewport) -> f32 {
        todo!()
    }

    //     void    (*Platform_OnChangedViewport)(ImGuiViewport* vp);               // . F . . .  // (Optional) [BETA] FIXME-DPI: DPI handling: Called during Begin() every time the viewport we are outputting into changes, so backend has a chance to swap fonts to adjust style.
    pub fn Platform_OnChangedViewport(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     int     (*Platform_CreateVkSurface)(ImGuiViewport* vp, ImU64 vk_inst, const void* vk_allocators, ImU64* out_vk_surface); // (Optional) For a Vulkan Renderer to call into Platform code (since the surface creation needs to tie them both).
    pub fn Platform_CreateVkSurface(&mut self, vp: &mut ImGuiViewport, vk_inst: u64, vk_allocators: *const c_void, out_vk_surface: &mut u64) -> i32 {
        todo!()
    }

    //
    //     // (Optional) Renderer functions (e.g. DirectX, OpenGL, Vulkan)
    //     void    (*Renderer_CreateWindow)(ImGuiViewport* vp);                    // . . U . .  // Create swap chain, frame buffers etc. (called after Platform_CreateWindow)
    pub fn Platform_CreateWindow2(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Renderer_DestroyWindow)(ImGuiViewport* vp);                   // N . U . D  // Destroy swap chain, frame buffers etc. (called before Platform_DestroyWindow)
    pub fn Platform_DestroyWindow2(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Renderer_SetWindowSize)(ImGuiViewport* vp, Vector2D size);      // . . U . .  // Resize swap chain, frame buffers etc. (called after Platform_SetWindowSize)
    pub fn Renderer_SetWindowSize(&mut self, vp: &mut ImGuiViewport, size: Vector2D) {
        todo!()
    }
    //     void    (*Renderer_RenderWindow)(ImGuiViewport* vp, void* render_arg);  // . . . R .  // (Optional) clear framebuffer, setup render target, then render the viewport->draw_data. 'render_arg' is the value passed to RenderPlatformWindowsDefault().
    pub fn Renderer_RenderWindow(&mut self, vp: &mut ImGuiViewport, render_arg: *mut c_void) {
        todo!()
    }
    //     void    (*Renderer_SwapBuffers)(ImGuiViewport* vp, void* render_arg);   // . . . R .  // (Optional) Call Present/SwapBuffers. 'render_arg' is the value passed to RenderPlatformWindowsDefault().
    pub fn Renderer_SwapBuffers(&mut self, vp: &mut ImGuiViewport, render_arg: &mut c_void) {
        todo!()
    }
}
