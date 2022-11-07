use crate::activate_flags::{ImGuiActivateFlags, IM_GUI_ACTIVATE_FLAGS_NONE};
use crate::chunk_stream::ImChunkStream;
use crate::color_edit_flags::{ImGuiColorEditFlags, ImGuiColorEditFlags_DefaultOptions_};
use crate::color_mod::ImGuiColorMod;
use crate::combo_preview_data::ImGuiComboPreviewData;
use crate::config_flags::{ImGuiConfigFlags_None, ImguiConfigFlags};
use crate::context_hook::ImGuiContextHook;
use crate::debug_log_flags::{ImGuiDebugLogFlags, IM_GUI_DEBUG_LOG_FLAGS_OUTPUT_TO_TTY};
use crate::direction::{ImGuiDir, ImGuiDir_None};
use crate::docking::dock_context::ImGuiDockContext;
use crate::docking::dock_node::ImGuiDockNode;
use crate::drag_drop::drag_drop_flags::{ImGuiDragDropFlags, ImGuiDragDropFlags_None};
use crate::draw_channel::ImDrawChannel;
use crate::draw_list_shared_data::Imgui_DrawListSharedData;
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::group_data::ImGuiGroupData;
use crate::input_event::ImguiInputEvent;
use crate::input_source::{ImGuiInputSource, ImGuiInputSource_None, ImGuiNavLayer};
use crate::input_text_state::ImGuiInputTextState;
use crate::io::ImguiIo;
use crate::item_flags::{ImGuiItemFlags, ImGuiItemFlags_None};
use crate::last_item_data::ImGuiLastItemData;
use crate::list_clipper_data::ImGuiListClipperData;
use crate::log_type::{ImGuiLogType, ImGuiLogType_None};
use crate::metrics_config::ImGuiMetricsConfig;
use crate::mod_flags::{ImGuiModFlags, ImGuiModFlags_None};
use crate::mouse_button::{ImGuiMouseButton, ImGuiMouseButton_Left};
use crate::mouse_cursor::{ImGuiMouseCursor, ImGuiMouseCursor_Arrow};
use crate::nav_item_data::ImGuiNavItemData;
use crate::nav_layer::{ImGuiNavLayer, ImGuiNavLayer_Main};
use crate::nav_move_flags::{ImGuiNavMoveFlags, ImGuiNavMoveFlags_None};
use crate::next_item_data::ImGuiNextItemData;
use crate::next_window_data::ImGuiNextWindowData;
use crate::payload::ImGuiPayload;
use crate::platform_ime_data::ImGuiPlatformImeData;
use crate::platform_io::ImguiPlatformIo;
use crate::platform_monitor::ImGuiPlatformMonitor;
use crate::pool::ImPool;
use crate::popup_data::ImGuiPopupData;
use crate::ptr_or_index::ImGuiPtrOrIndex;
use crate::rect::ImRect;
use crate::scroll_flags::{ImGuiScrollFlags, ImGuiScrollFlags_None};
use crate::settings_handler::SettingsHandler;
use crate::shrink_width_item::ImGuiShrinkWidthItem;
use crate::stack_tool::ImGuiStackTool;
use crate::storage::ImGuiStorage;
use crate::style::ImguiStyle;
use crate::style_mod::ImGuiStyleMod;
use crate::tab_bar::ImGuiTabBar;
use crate::table::ImGuiTable;
use crate::table_settings::ImGuiTableSettings;
use crate::table_temp_data::ImGuiTableTempData;
use crate::text_buffer::ImGuiTextBuffer;
use crate::type_defs::{
    ImBitArrayForNamedKeys, ImFileHandle, ImGuiDir, ImTextureID, ImguiHandle, INVALID_IMGUI_HANDLE,
};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::viewport::ImguiViewport;
use crate::window::window_settings::ImGuiWindowSettings;
use crate::window::window_stack_data::ImGuiWindowStackData;
use crate::window::ImguiWindow;
use crate::window_settings::ImGuiWindowSettings;
use crate::window_stack_data::ImGuiWindowStackData;
use crate::{Initialize, Shutdown};
use libc::{c_char, c_double, c_float, c_int, c_uchar, c_void};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ptr::null_mut;

#[derive(Default, Debug, Clone)]
pub struct ImguiContext {
    pub Initialized: bool,
    // IO.Fonts. is owned by the ImGuiContext and will be destructed along with it.
    pub FontAtlasOwnedByContext: bool,
    // ImGuiIO                 IO;
    pub IO: ImguiIo,
    pub PlatformIO: ImguiPlatformIo,
    // Input events which will be tricked/written into IO structure.
    pub InputEventsQueue: Vec<ImguiInputEvent>,
    // Past input events processed in NewFrame(). This is to allow domain-specific
    // application to access e.g mouse/pen trail.
    pub InputEventsTrail: Vec<ImguiInputEvent>,
    pub style: ImguiStyle,
    // g.IO.ConfigFlags at the time of NewFrame()
    pub ConfigFlagsCurrFrame: ImguiConfigFlags,
    pub ConfigFlagsLastFrame: ImguiConfigFlags,
    pub Font: ImFont,
    // Text height for current window.
    pub FontSize: f32,
    // (Shortcut) == IO.FontGlobalScale * Font->Scale * Font->FontSize. Base text height.
    pub FontBaseSize: c_float,
    pub DrawListSharedData: Imgui_DrawListSharedData,
    pub Time: u64,
    pub FrameCount: usize,
    pub FrameCountEnded: usize,
    pub FrameCountPlatformEnded: usize,
    pub FrameCountRendered: usize,
    // Set by NewFrame(), cleared by EndFrame()
    pub WithinFrameScope: bool,
    // Set by NewFrame(), cleared by EndFrame() when the implicit debug window has been
    // pushed
    pub WithinFrameScopeWithImplicitWindow: bool,
    // Set within EndChild()
    pub WithinEndChild: bool,
    // Request full GC
    pub GcCompactAll: bool,
    // Will call test engine hooks
    pub TestEngineHookItems: bool,
    // Test engine user data
    pub TestEngine: Vec<u8>,
    // Windows, sorted in display order, back to front
    pub windows: HashMap<ImguiHandle, ImguiWindow>,
    // Root windows, sorted in focus order, back to front.
    pub WindowsFocusOrder: Vec<ImguiHandle>,
    // Temporary buffer used in EndFrame() to reorder windows so parents are kept before
    // their child
    pub WindowsTempSortBuffer: Vec<ImguiHandle>,
    pub CurrentWindowStack: Vec<ImGuiWindowStackData>,
    // Number of unique windows submitted by frame
    pub WindowsActiveCount: i32,
    // Padding around resizable windows for which hovering on counts as hovering the
    // window == ImMax(style.TouchExtraPadding, WINDOWS_HOVER_PADDING)
    pub WindowsHoverPadding: ImVec2,
    // Window being drawn into
    pub CurrentWindow: ImguiHandle,
    // Window the mouse is hovering. Will typically catch mouse inputs.
    pub HoveredWindow: ImguiHandle,
    // Hovered window ignoring MovingWindow. Only set if MovingWindow is set.
    pub HoveredWindowUnderMovingWindow: ImguiHandle,
    // Track the window we clicked on (in order to preserve focus). The actual window
    // that is moved is generally Movingwindow.RootWindowDockTree.
    pub MovingWindow: ImguiHandle,
    // Track the window we started mouse-wheeling on. Until a timer elapse or mouse has
    // moved, generally keep scrolling the same window even if during the course of
    // scrolling the mouse ends up hovering a child window.
    pub WheelingWindow: ImguiWindow,
    pub WheelingWindowRefMousePos: ImVec2,
    pub WheelingWindowTimer: f32,
    // Will call core hooks: DebugHookIdInfo() from GetID functions, used by Stack Tool
    // [next HoveredId/ActiveId to not pull in an extra cache-line]
    pub DebugHookIdInfo: ImguiHandle,
    // Hovered widget, filled during the frame
    pub HoveredId: ImguiHandle,
    pub HoveredIdPreviousFrame: ImguiHandle,
    pub HoveredIdAllowOverlap: bool,
    // Hovered widget will use mouse wheel. Blocks scrolling the underlying window.
    pub HoveredIdUsingMouseWheel: bool,
    pub HoveredIdPreviousFrameUsingMouseWheel: bool,
    // At least one widget passed the rect test, but has been discarded by disabled flag
    // or popup inhibit. May be true even if HoveredId == 0.
    pub HoveredIdDisabled: bool,
    // Measure contiguous hovering time
    pub HoveredIdTimer: f32,
    // Measure contiguous hovering time where the item has not been active
    pub HoveredIdNotActiveTimer: f32,
    // Active widget
    pub ActiveId: ImguiHandle,
    // Active widget has been seen this frame (we can't use a as: bool the ActiveId may
    // change within the frame)
    pub ActiveIdIsAlive: ImguiHandle,
    pub ActiveIdTimer: f32,
    // Set at the time of activation for one frame
    pub ActiveIdIsJustActivated: bool,
    // Active widget allows another widget to steal active id (generally for overlapping
    // widgets, but not always)
    pub ActiveIdAllowOverlap: bool,
    // Disable losing active id if the active id window gets unfocused.
    pub ActiveIdNoClearOnFocusLoss: bool,
    // Track whether the active id led to a press (this is to allow changing between
    // PressOnClick and PressOnRelease without pressing twice). Used by range_select
    // branch.
    pub ActiveIdHasBeenPressedBefore: bool,
    // Was the value associated to the widget Edited over the course of the Active state.
    pub ActiveIdHasBeenEditedBefore: bool,
    pub ActiveIdHasBeenEditedThisFrame: bool,
    // Clicked offset from upper-left corner, if applicable (currently only set by
    // ButtonBehavior)
    pub ActiveIdClickOffset: ImVec2,
    pub ActiveIdWindow: ImguiHandle,
    // Activating with mouse or nav (gamepad/keyboard)
    pub ActiveIdSource: ImGuiInputSource,
    pub ActiveIdMouseButton: c_int,
    pub ActiveIdPreviousFrame: ImguiHandle,
    pub ActiveIdPreviousFrameIsAlive: bool,
    pub ActiveIdPreviousFrameHasBeenEditedBefore: bool,
    pub ActiveIdPreviousFrameWindow: ImguiHandle,
    // Store the last non-zero ActiveId, useful for animation.
    pub LastActiveId: ImguiHandle,
    // Store the last non-zero ActiveId timer since the beginning of activation, useful
    // for animation.
    pub LastActiveIdTimer: f32,
    // Active widget will want to read those nav move requests (e.g. can activate a button
    // and move away from it)
    pub ActiveIdUsingNavDirMask: u32,
    // Active widget will want to read those key inputs. When we grow the ImGuiKey enum
    // we'll need to either to order the enum to make useful keys come first, either
    // redesign this into e.g. a small array.
    pub ActiveIdUsingKeyInputMask: ImBitArrayForNamedKeys,
    pub CurrentItemFlags: ImGuiItemFlags,
    // Storage for SetNextItem** functions
    pub next_item_data: ImGuiNextItemData,
    // Storage for last submitted item (setup by ItemAdd)
    pub last_item_data: ImGuiLastItemData,
    // Storage for SetNextWindow** functions
    pub NextWindowData: ImGuiNextWindowData,
    // Shared stacks
    // Stack for PushStyleColor()/PopStyleColor() - inherited by Begin()
    pub ColorStack: Vec<ImGuiColorMod>,
    // Stack for PushStyleVar()/PopStyleVar() - inherited by Begin()
    pub StyleVarStack: Vec<ImGuiStyleMod>,
    // Stack for PushFont()/PopFont() - inherited by Begin()
    pub FontStack: Vec<ImFont>,
    // Stack for PushFocusScope()/PopFocusScope() - not inherited by Begin(), unless child window
    pub FocusScopeStack: Vec<ImguiHandle>,
    // Stack for PushItemFlag()/PopItemFlag() - inherited by Begin()
    pub ItemFlagsStack: Vec<ImGuiItemFlags>,
    // Stack for BeginGroup()/EndGroup() - not inherited by Begin()
    pub GroupStack: Vec<ImGuiGroupData>,
    // Which popups are open (persistent)
    pub OpenPopupStack: Vec<ImGuiPopupData>,
    // Which level of BeginPopup() we are in (reset every frame)
    pub BeginPopupStack: Vec<ImGuiPopupData>,
    pub BeginMenuCount: Vec<c_int>,
    // Active viewports (always 1+, and generally 1 unless multi-viewports are enabled).
    // Each viewports hold their copy of ImDrawData.
    pub Viewports: HashMap<ImguiHandle, ImguiViewport>,
    pub CurrentDpiScale: c_float,
    // We track changes of viewport (happening in Begin) so we can call
    // Platform_OnChangedViewport()
    pub CurrentViewport: ImguiHandle,
    pub MouseViewport: ImguiHandle,
    // Last known viewport that was hovered by mouse (even if we are not hovering any
    // viewport any more) + honoring the _NoInputs flag.
    pub MouseLastHoveredViewport: ImguiHandle,
    pub PlatformLastFocusedViewportId: ImguiHandle,
    // Virtual monitor used as fallback if backend doesn't provide monitor information.
    pub FallbackMonitor: ImGuiPlatformMonitor,
    // Every time the front-most window changes, we stamp its viewport with an
    // incrementing counter
    pub ViewportFrontMostStampCount: c_int,
    // Gamepad/keyboard Navigation
    // Focused window for navigation. Could be called 'FocusedWindow'
    pub NavWindow: ImguiHandle,
    // Focused item for navigation
    pub NavId: ImguiHandle,
    // Identify a selection scope (selection code often wants to "clear other items" when
    // landing on an item of the selection set)
    pub NavFocusScopeId: ImguiHandle,
    pub NavActivateId: ImguiHandle,
    pub NavActivateDownId: ImguiHandle,
    pub NavActivatePressedId: ImguiHandle,
    pub NavActivateInputId: ImguiHandle,
    pub NavActivateFlags: ImGuiActivateFlags,
    // Just navigated to this id (result of a successfully MoveRequest).
    pub NavJustMovedToId: ImguiHandle,
    // Just navigated to this focus scope id (result of a successfully MoveRequest).
    pub NavJustMovedToFocusScopeId: ImguiHandle,
    pub NavJustMovedToKeyMods: ImGuiModFlags,
    // Set by ActivateItem(), queued until next frame.
    pub NavNextActivateId: ImguiHandle,
    pub NavNextActivateFlags: ImGuiActivateFlags,
    // Keyboard or Gamepad mode? THIS WILL ONLY BE None or NavGamepad or NavKeyboard.
    pub NavInputSource: ImGuiInputSource,
    // Layer we are navigating on. For now the system is hard-coded for 0=main contents
    // and 1=menu/title bar, may expose layers later.
    pub NavLayer: ImGuiNavLayer,
    // Nav widget has been seen this frame ~~ NavRectRel is valid
    pub NavIdIsAlive: bool,
    // When set we will update mouse position if (io.ConfigFlags &
    // ImGuiConfigFlags_NavEnableSetMousePos) if set (NB: this not enabled by default)
    pub NavMousePosDirty: bool,
    // When user starts using mouse, we hide gamepad/keyboard highlight (NB: but they are
    // still available, which is why NavDisableHighlight isn't always !=
    // NavDisableMouseHover)
    pub NavDisableHighlight: bool,
    // When user starts using gamepad/keyboard, we hide mouse hovering highlight until
    // mouse is touched again.
    pub NavDisableMouseHover: bool,
    // Navigation: Init & Move Requests
    // ~~ NavMoveRequest || NavInitRequest this is to perform early out in ItemAdd()
    pub NavAnyRequest: bool,
    // Init request for appearing window to select first item
    pub NavInitRequest: bool,
    pub NavInitRequestFromMove: bool,
    // Init request result (first item of the window, or one for which
    // SetItemDefaultFocus() was called)
    pub NavInitResultId: ImguiHandle,
    // Init request result rectangle (relative to parent window)
    pub NavInitResultRectRel: ImRect,
    // Move request submitted, will process result on next NewFrame()
    pub NavMoveSubmitted: bool,
    // Move request submitted, still scoring incoming items
    pub NavMoveScoringItems: bool,
    pub NavMoveForwardToNextFrame: bool,
    pub NavMoveFlags: ImGuiNavMoveFlags,
    pub NavMoveScrollFlags: ImGuiScrollFlags,
    pub NavMoveKeyMods: ImGuiModFlags,
    // Direction of the move request (left/right/up/down)
    pub NavMoveDir: ImGuiDir,
    pub NavMoveDirForDebug: ImGuiDir,
    // FIXME-NAV: Describe the purpose of this better. Might want to rename?
    pub NavMoveClipDir: ImGuiDir,
    // Rectangle used for scoring, in screen space. Based of window.NavRectRel[], modified
    // for directional navigation scoring.
    pub NavScoringRect: ImRect,
    // Some nav operations (such as PageUp/PageDown) enforce a region which clipper will
    // attempt to always keep submitted
    pub NavScoringNoClipRect: ImRect,
    // Metrics for debugging
    pub NavScoringDebugCount: c_int,
    // Generally -1 or +1, 0 when tabbing without a nav id
    pub NavTabbingDir: c_int,
    // >0 when counting items for tabbing
    pub NavTabbingCounter: c_int,
    // Best move request candidate within NavWindow
    pub NavMoveResultLocal: ImGuiNavItemData,
    // Best move request candidate within NavWindow that are mostly visible (when using
    // ImGuiNavMoveFlags_AlsoScoreVisibleSet flag)
    pub NavMoveResultLocalVisible: ImGuiNavItemData,
    // Best move request candidate within NavWindow's flattened hierarchy (when using
    // ImGuiWindowFlags_NavFlattened flag)
    pub NavMoveResultOther: ImGuiNavItemData,
    // First tabbing request candidate within NavWindow and flattened hierarchy
    pub NavTabbingResultFirst: ImGuiNavItemData,
    // Navigation: Windowing (CTRL+TAB for list, or Menu button + keys or directional
    // pads to move/resize)
    // Target window when doing CTRL+Tab (or Pad Menu + FocusPrev/Next), this window is
    // temporarily displayed top-most!
    pub NavWindowingTarget: ImguiHandle,
    // Record of last valid NavWindowingTarget until DimBgRatio and
    // NavWindowingHighlightAlpha becomes 0.0, so the fade-out can stay on it.
    pub NavWindowingTargetAnim: ImguiHandle,
    // Internal window actually listing the CTRL+Tab contents
    pub NavWindowingListWindow: ImguiHandle,
    pub NavWindowingTimer: c_float,
    pub NavWindowingHighlightAlpha: c_float,
    pub NavWindowingToggleLayer: bool,
    pub NavWindowingAccumDeltaPos: ImVec2,
    pub NavWindowingAccumDeltaSize: ImVec2,
    // Render
    // 0.0..1.0 animation when fading in a dimming background (for modal window and CTRL+TAB list)
    pub DimBgRatio: c_float,
    pub MouseCursor: ImGuiMouseCursor,
    // Drag and Drop
    pub DragDropActive: bool,
    // Set when within a BeginDragDropXXX/EndDragDropXXX block for a drag source.
    pub DragDropWithinSource: bool,
    // Set when within a BeginDragDropXXX/EndDragDropXXX block for a drag target.
    pub DragDropWithinTarget: bool,
    pub DragDropSourceFlags: ImGuiDragDropFlags,
    pub DragDropSourceFrameCount: c_int,
    pub DragDropMouseButton: c_int,
    pub DragDropPayload: ImGuiPayload,
    // Store rectangle of current target candidate (we favor small targets when
    // overlapping)
    pub DragDropTargetRect: ImRect,
    pub DragDropTargetId: ImguiHandle,
    pub DragDropAcceptFlags: ImGuiDragDropFlags,
    // Target item surface (we resolve overlapping targets by prioritizing the smaller
    // surface)
    pub DragDropAcceptIdCurrRectSurface: c_float,
    // Target item id (set at the time of accepting the payload)
    pub DragDropAcceptIdCurr: ImguiHandle,
    // Target item id from previous frame (we need to store this to allow for overlapping
    // drag and drop targets)
    pub DragDropAcceptIdPrev: ImguiHandle,
    // Last time a target expressed a desire to accept the source
    pub DragDropAcceptFrameCount: c_int,
    // Set when holding a payload just made ButtonBehavior() return a press.
    pub DragDropHoldJustPressedId: ImguiHandle,
    // We don't expose the ImVector<> directly, ImGuiPayload only holds pointer+size
    pub DragDropPayloadBufHead: Vec<u8>,
    // Local buffer for small payloads
    pub DragDropPayloadBufLocal: Vec<u8>,
    // Clipper
    pub ClipperTempData: Vec<ImGuiListClipperData>,
    // Tables
    pub current_table: ImguiHandle,
    // Temporary table data size (because we leave previous instances undestructed, we
    // generally don't use TablesTempData.Size)
    pub TablesTempDataStacked: usize,
    // Temporary table data (buffers reused/shared across instances, support nesting)
    pub TablesTempData: Vec<ImGuiTableTempData>,
    // Persistent table data
    pub Tables: HashMap<ImguiHandle, ImGuiTable>,
    // Last used timestamp of each tables (SOA, for efficient GC)
    pub TablesLastTimeActive: Vec<c_float>,
    pub DrawChannelsTempMergeBuffer: Vec<ImDrawChannel>,
    // Tab bars
    pub CurrentTabBar: ImGuiTabBar,
    pub TabBars: HashMap<ImguiHandle, ImGuiTabBar>,
    pub CurrentTabBarStack: Vec<ImguiHandle>,
    pub ShrunkWidthBuffer: Vec<ImGuiShrinkWidthItem>,
    // Hover Delay system
    pub HoverDelayId: ImguiHandle,
    pub HoverDelayIdPreviousFrame: ImguiHandle,
    // Currently used IsItemHovered(), generally inferred from g.HoveredIdTimer but kept
    // uncleared until clear timer elapse.
    pub HoverDelayTimer: c_float,
    // Currently used IsItemHovered(): grace time before g.TooltipHoverTimer gets cleared.
    pub HoverDelayClearTimer: c_float,
    // Widget state
    pub MouseLastValidPos: ImVec2,
    pub InputTextState: ImGuiInputTextState,
    pub InputTextPasswordFont: ImFont,
    // Temporary text input when CTRL+clicking on a slider, etc.
    pub TempInputId: ImguiHandle,
    // Store user options for color edit widgets
    pub ColorEditOptions: ImGuiColorEditFlags,
    // Backup of last Hue associated to LastColor, so we can restore Hue in lossy RGB<>HSV
    // round trips
    pub ColorEditLastHue: c_float,
    // Backup of last Saturation associated to LastColor, so we can restore Saturation in
    // lossy RGB<>HSV round trips
    pub ColorEditLastSat: c_float,
    // RGB value with alpha set to 0.
    pub ColorEditLastColor: u32,
    // Initial/reference color at the time of opening the color picker.
    pub ColorPickerRef: ImVec4,
    pub ComboPreviewData: ImGuiComboPreviewData,
    pub SliderGrabClickOffset: c_float,
    // Accumulated slider delta when using navigation controls.
    pub SliderCurrentAccum: c_float,
    // Has the accumulated slider delta changed since last time we tried to apply it?
    pub SliderCurrentAccumDirty: bool,
    pub DragCurrentAccumDirty: bool,
    // Accumulator for dragging modification. Always high-precision, not rounded by
    // end-user precision settings
    pub DragCurrentAccum: c_float,
    // If speed == 0.0, uses (max-min) * DragSpeedDefaultRatio
    pub DragSpeedDefaultRatio: c_float,
    // Distance between mouse and center of grab box, normalized in parent space. Use storage?
    pub ScrollbarClickDeltaToGrabCenter: c_float,
    // Backup for style.Alpha for BeginDisabled()
    pub DisabledAlphaBackup: c_float,
    pub DisabledStackSize: i16,
    pub TooltipOverrideCount: i16,
    // If no custom clipboard handler is defined
    pub ClipboardHandlerData: Vec<c_char>,
    // A list of menu IDs that were rendered at least once
    pub MenusIdSubmittedThisFrame: Vec<ImguiHandle>,
    // Platform support
    // Data updated by current frame
    pub PlatformImeData: ImGuiPlatformImeData,
    // Previous frame data (when changing we will call io.SetPlatformImeDataFn
    pub PlatformImeDataPrev: ImGuiPlatformImeData,
    pub PlatformImeViewport: ImguiHandle,
    // '.' or *localeconv()->decimal_point
    pub PlatformLocaleDecimalPoint: c_char,
    // Extensions
    // FIXME: We could provide an API to register one slot in an array held in ImGuiContext?
    pub dock_context: ImGuiDockContext,
    // Settings
    pub SettingsLoaded: bool,
    // Save .ini Settings to memory when time reaches zero
    pub SettingsDirtyTimer: c_float,
    // In memory .ini settings
    pub SettingsIniData: String,
    // List of .ini settings handlers
    pub settings_handlers: Vec<SettingsHandler>,
    // ImGuiWindow .ini settings entries
    pub SettingsWindow: Vec<ImGuiWindowSettings>,
    // ImGuiTable .ini settings entries
    pub SettingsTables: Vec<ImGuiTableSettings>,
    // Hooks for extensions (e.g. test engine)
    pub hooks: HashMap<ImguiHandle, ImGuiContextHook>,
    // Currently capturing
    pub LogEnabled: bool,
    // Capture target
    pub LogType: ImGuiLogType,
    // If != NULL log to stdout/ file
    pub LogFile: Option<ImFileHandle>,
    // Accumulation buffer when log to clipboard. This is pointer so our GImGui static
    // constructor doesn't call heap allocators.
    pub LogBuffer: String,
    pub LogNextPrefix: Option<String>,
    pub LogNextSuffix: Option<String>,
    pub LogLinePosY: c_float,
    pub LogLineFirstItem: bool,
    pub LogDepthRef: c_int,
    pub LogDepthToExpand: c_int,
    // Default/stored value for LogDepthMaxExpand if not specified in the LogXXX function
    // call.
    pub LogDepthToExpandDefault: c_int,
    // Debug Tools
    pub DebugLogFlags: ImGuiDebugLogFlags,
    pub DebugLogBuf: String,
    // Item picker is active (started with DebugStartItemPicker())
    pub DebugItemPickerActive: bool,
    pub DebugItemPickerMouseButton: ImGuiMouseButton,
    // Will call IM_DEBUG_BREAK() when encountering this ID
    pub DebugItemPickerBreakId: ImguiHandle,
    pub DebugMetricsConfig: ImGuiMetricsConfig,
    pub DebugStackTool: ImGuiStackTool,
    // Hovered dock node.
    pub DebugHoveredDockNode: Option<ImGuiDockNode>,
    // Misc
    // Calculate estimate of framerate for user over the last 60 frames..
    pub FramerateSecPerFrame: [c_float; 60],
    pub FramerateSecPerFrameIdx: c_int,
    pub FramerateSecPerFrameCount: c_int,
    pub FramerateSecPerFrameAccum: c_float,
    // Explicit capture override via SetNextFrameWantCaptureMouse()
    // /SetNextFrameWantCaptureKeyboard(). Default to -1.
    pub WantCaptureMouseNextFrame: c_int,
    pub WantCaptureKeyboardNextFrame: c_int,
    pub WantTextInputNextFrame: c_int,
    // Temporary text buffer
    pub TempBuffer: Vec<c_char>,
}

impl ImguiContext {
    pub fn new(shared_font_atlas: Option<ImFontAtlas>) -> Self {
        let mut out = Self {
            Initialized: false,
            ConfigFlagsCurrFrame: ImGuiConfigFlags_None,
            ConfigFlagsLastFrame: ImGuiConfigFlags_None,
            FontAtlasOwnedByContext: if shared_font_atlas.is_null() == false {
                false
            } else {
                true
            },
            Font: ImFont::default(),
            FontSize: 0.0,
            FontBaseSize: 0.0,
            Time: 0,
            FrameCount: 0,
            FrameCountEnded: -1,
            FrameCountPlatformEnded: -1,
            FrameCountRendered: -1,
            WithinFrameScope: false,
            WithinFrameScopeWithImplicitWindow: false,
            WithinEndChild: false,
            GcCompactAll: false,
            TestEngineHookItems: false,
            TestEngine: vec![],
            WindowsActiveCount: 0,
            CurrentWindow: ImguiWindow::default(),
            HoveredWindow: ImguiWindow::default(),
            HoveredWindowUnderMovingWindow: ImguiWindow::default(),
            MovingWindow: ImguiWindow::default(),
            WheelingWindow: ImguiWindow::default(),
            WheelingWindowTimer: 0.0,
            DebugHookIdInfo: 0,
            HoveredId: 0,
            HoveredIdPreviousFrame: 0,
            HoveredIdAllowOverlap: false,
            HoveredIdUsingMouseWheel: false,
            HoveredIdPreviousFrameUsingMouseWheel: false,
            HoveredIdDisabled: false,
            HoveredIdTimer: 0.0,
            HoveredIdNotActiveTimer: 0.0,
            ActiveId: 0,
            ActiveIdIsAlive: 0,
            ActiveIdTimer: 0.0,
            ActiveIdIsJustActivated: false,
            ActiveIdAllowOverlap: false,
            ActiveIdNoClearOnFocusLoss: false,
            ActiveIdHasBeenPressedBefore: false,
            ActiveIdHasBeenEditedBefore: false,
            ActiveIdHasBeenEditedThisFrame: false,
            ActiveIdClickOffset: ImVec2::from_floats(-1.0, -1.0),
            ActiveIdWindow: ImguiWindow::default(),
            ActiveIdSource: ImGuiInputSource_None,
            ActiveIdMouseButton: -1,
            ActiveIdPreviousFrame: 0,
            ActiveIdPreviousFrameIsAlive: false,
            ActiveIdPreviousFrameHasBeenEditedBefore: false,
            ActiveIdPreviousFrameWindow: ImguiWindow::default(),
            LastActiveId: 0,
            LastActiveIdTimer: 0.0,
            ActiveIdUsingNavDirMask: 0x00,
            // #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
            //             ActiveIdUsingNavInputMask : 0x00,
            // #endif
            CurrentItemFlags: ImGuiItemFlags_None,
            BeginMenuCount: vec![],

            CurrentDpiScale: 0.0,
            CurrentViewport: ImguiViewport::default(),
            MouseViewport: ImguiViewport::default(),
            MouseLastHoveredViewport: ImguiViewport::default(),
            PlatformLastFocusedViewportId: 0,
            ViewportFrontMostStampCount: 0,

            NavWindow: INVALID_IMGUI_HANDLE,
            NavId: 0,
            NavFocusScopeId: 0,
            NavActivateId: 0,
            NavActivateDownId: 0,
            NavActivatePressedId: 0,
            NavActivateInputId: 0,
            NavJustMovedToId: 0,
            NavJustMovedToFocusScopeId: 0,
            NavNextActivateId: 0,
            NavActivateFlags: IM_GUI_ACTIVATE_FLAGS_NONE,
            NavNextActivateFlags: IM_GUI_ACTIVATE_FLAGS_NONE,
            NavJustMovedToKeyMods: ImGuiModFlags_None,
            NavInputSource: ImGuiInputSource_None,
            NavLayer: ImGuiNavLayer_Main,
            NavIdIsAlive: false,
            NavMousePosDirty: false,
            NavDisableHighlight: true,
            NavDisableMouseHover: false,
            NavAnyRequest: false,
            NavInitRequest: false,
            NavInitRequestFromMove: false,
            NavInitResultId: 0,
            NavMoveSubmitted: false,
            NavMoveScoringItems: false,
            NavMoveForwardToNextFrame: false,
            NavMoveFlags: ImGuiNavMoveFlags_None,
            NavMoveScrollFlags: ImGuiScrollFlags_None,
            NavMoveKeyMods: ImGuiModFlags_None,
            NavMoveDir: ImGuiDir_None,
            NavMoveDirForDebug: ImGuiDir_None,
            NavMoveClipDir: ImGuiDir_None,
            NavScoringDebugCount: 0,
            NavTabbingDir: 0,
            NavTabbingCounter: 0,

            NavWindowingTarget: INVALID_IMGUI_HANDLE,
            NavWindowingTargetAnim: INVALID_IMGUI_HANDLE,
            NavWindowingListWindow: INVALID_IMGUI_HANDLE,
            NavWindowingTimer: 0.0,
            NavWindowingHighlightAlpha: 0.0,
            NavWindowingToggleLayer: false,

            DimBgRatio: 0.0,
            MouseCursor: ImGuiMouseCursor_Arrow,

            DragDropActive: false,
            DragDropWithinSource: false,
            DragDropWithinTarget: false,
            DragDropSourceFlags: ImGuiDragDropFlags_None,
            DragDropSourceFrameCount: -1,
            DragDropMouseButton: -1,
            DragDropTargetId: 0,
            DragDropAcceptFlags: ImGuiDragDropFlags_None,
            DragDropAcceptIdCurrRectSurface: 0.0,
            DragDropAcceptIdPrev: 0,
            DragDropAcceptIdCurr: 0,
            DragDropAcceptFrameCount: -1,
            DragDropHoldJustPressedId: 0,
            // ClipperTempDataStacked: 0,
            current_table: INVALID_IMGUI_HANDLE,
            TablesTempDataStacked: 0,
            CurrentTabBar: ImGuiTabBar::default(),
            HoverDelayId: 0,
            HoverDelayIdPreviousFrame: 0,
            HoverDelayTimer: 0.0,
            HoverDelayClearTimer: 0.0,
            TempInputId: 0,
            ColorEditOptions: ImGuiColorEditFlags_DefaultOptions_,
            ColorEditLastHue: 0.0,
            ColorEditLastSat: 0.0,
            ColorEditLastColor: 0,
            SliderGrabClickOffset: 0.0,
            SliderCurrentAccum: 0.0,
            SliderCurrentAccumDirty: false,
            DragCurrentAccumDirty: false,
            DragCurrentAccum: 0.0,
            DragSpeedDefaultRatio: 1.0 / 100.0,
            ScrollbarClickDeltaToGrabCenter: 0.0,
            DisabledAlphaBackup: 0.0,
            DisabledStackSize: 0,
            TooltipOverrideCount: 0,
            PlatformImeViewport: 0,
            PlatformLocaleDecimalPoint: '.'.into(),
            SettingsLoaded: false,
            SettingsDirtyTimer: 0.0,
            // HookIdNext: vec![],
            LogEnabled: false,
            LogType: ImGuiLogType_None,
            LogNextPrefix: None,
            LogNextSuffix: None,
            LogFile: None,
            LogLinePosY: f32::MAX,
            LogLineFirstItem: false,
            LogDepthRef: 0,
            LogDepthToExpand: 2,
            LogDepthToExpandDefault: 2,
            DebugLogFlags: IM_GUI_DEBUG_LOG_FLAGS_OUTPUT_TO_TTY,
            DebugItemPickerActive: false,
            DebugItemPickerMouseButton: ImGuiMouseButton_Left,
            DebugItemPickerBreakId: 0,
            DebugHoveredDockNode: None,

            FramerateSecPerFrameIdx: 0,
            FramerateSecPerFrameCount: 0,
            FramerateSecPerFrameAccum: 0.0,
            WantCaptureMouseNextFrame: -1,
            WantCaptureKeyboardNextFrame: -1,
            WantTextInputNextFrame: -1,
            ..Default::default()
        };

        out.IO.Fonts = Some(match shared_font_atlas {
            Some(x) => x.clone(),
            None() => ImFontAtlas::default(),
        });
        out.ActiveIdUsingKeyInputMask.ClearAllBits();
        out.PlatformImeData.InputPos = ImVec2::default();
        out.PlatformImeDataPrev.InputPos = ImVec2::from_floats(-1.0, -1.0); // Different to ensure initial submission
                                                                            // libc::memset(
                                                                            //     out.DragDropPayloadBufLocal.as_mut_ptr(),
                                                                            //     0,
                                                                            //     libc::sizeof(out.DragDropPayloadBufLocal),
                                                                            // );
                                                                            // libc::memset(
                                                                            //     out.FramerateSecPerFrame.as_mut_ptr(),
                                                                            //     0,
                                                                            //     libc::sizeof(FramerateSecPerFrame),
                                                                            // );
        return out;
    }

    pub fn CurrentTabBar(&mut self) -> &mut ImGuiTabBar {
        todo!()
    }

    pub fn push_curr_win_draw_list_text_id(&mut self, texture_id: ImTextureID) {
        let mut curr_win_draw_list = &mut self.CurrentWindow.as_mut().unwrap().DrawList;
        curr_win_draw_list.PushTextureID(texture_id);
    }

    pub fn window_by_id_mut(&mut self, id: ImguiHandle) -> Option<&mut ImguiWindow> {
        self.windows.get_mut(&id)
    }

    pub fn current_window_mut(&mut self) -> Option<&mut ImguiWindow> {
        if self.CurrentWindow == INVALID_IMGUI_HANDLE {
            return None;
        }
        self.window_by_id_mut(self.CurrentWindow)
    }

    pub fn add_settings_handler(&mut self, handler: &SettingsHandler) {
        self.settings_handlers.push(handler.clone())
    }

    pub fn add_context_hook(&mut self, hook: &ImGuiContextHook) -> ImguiHandle {
        self.hooks.push(hook.clone());
        self.HookIdNext += 1;
        self.H
    }

    pub fn remove_context_hook(&mut self, hook_id: &ImguiHandle) {
        self.hooks.remove(hook_id);
    }

    pub fn call_context_hooks(&mut self, hook_type: ImGuiContextHookType) {
        for hook in self.hooks.values_mut() {
            if hook.hook_type == hook_type {
                hook.Callback(self, hook);
            }
        }
    }

    pub fn platform_io_mut(&mut self) -> &mut ImguiPlatformIo {
        self.PlatformIO.borrow_mut()
    }

    pub fn time(&self) -> u64 {
        self.Time
    }

    pub fn frame_count(&self) -> usize {
        self.FrameCount
    }
}

pub fn create_context(
    pg: Option<&mut ImguiContext>,
    shared_font_atlas: Option<ImFontAtlas>,
) -> ImguiContext {
    let mut ctx = ImguiContext::new(shared_font_atlas);
    // SetCurrentContext(&mut ctx);
    Initialize(g);
    if prev_ctx != None {
        SetCurrentContext(prev_ctx.unwrap().borrow_mut());
    } // Restore previous context if any, else keep new one.
    return ctx;
}

// c_void DestroyContext(g: &mut ImguiContext)
pub unsafe fn destroy_context(mut ctx: &mut ImguiContext) {
    // let mut prev_ctx = GetCurrentContext();
    // if ctx == None {
    //     //-V1051
    //     ctx = prev_ctx;
    // }
    // SetCurrentContext(ctx);
    Shutdown(g);
    // SetCurrentContext(if prev_ctx.unwrap() != ctx {
    //     prev_ctx.unwrap().borrow_mut()
    // } else {
    //     None
    // });
    // IM_DELETE(ctx);
}
