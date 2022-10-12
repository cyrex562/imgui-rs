#![allow(non_snake_case)]
//-----------------------------------------------------------------------------
// [SECTION] ImGuiContext (main Dear ImGui context)
//-----------------------------------------------------------------------------

use std::ptr::null_mut;
use libc::{c_char, c_double, c_float, c_int, c_uchar, c_void};
use crate::activate_flags::{ImGuiActivateFlags, ImGuiActivateFlags_None};
use crate::chunk_stream::ImChunkStream;
use crate::color_edit_flags::{ImGuiColorEditFlags, ImGuiColorEditFlags_DefaultOptions_};
use crate::color_mod::ImGuiColorMod;
use crate::combo_preview_data::ImGuiComboPreviewData;
use crate::config_flags::{ImGuiConfigFlags, ImGuiConfigFlags_None};
use crate::context_hook::ImGuiContextHook;
use crate::debug_log_flags::{ImGuiDebugLogFlags, ImGuiDebugLogFlags_OutputToTTY};
use crate::direction::{ImGuiDir, ImGuiDir_None};
use crate::dock_context::ImGuiDockContext;
use crate::input_source::{ImGuiInputSource, ImGuiInputSource_None, ImGuiNavLayer};
use crate::draw_list_shared_data::ImDrawListSharedData;
use crate::group_data::ImGuiGroupData;
use crate::dock_node::ImGuiDockNode;
use crate::drag_drop_flags::{ImGuiDragDropFlags, ImGuiDragDropFlags_None};
use crate::draw_channel::ImDrawChannel;
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::input_event::ImGuiInputEvent;
use crate::input_text_state::ImGuiInputTextState;
use crate::storage::ImGuiStorage;
use crate::text_buffer::ImGuiTextBuffer;
use crate::window::ImGuiWindow;
use crate::io::ImGuiIO;
use crate::item_flags::{ImGuiItemFlags, ImGuiItemFlags_None};
use crate::platform_io::ImGuiPlatformIO;
use crate::rect::ImRect;
use crate::style::ImGuiStyle;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::viewport::ImGuiViewport;
use crate::last_item_data::ImGuiLastItemData;
use crate::list_clipper_data::ImGuiListClipperData;
use crate::log_type::{ImGuiLogType, ImGuiLogType_None};
use crate::metrics_config::ImGuiMetricsConfig;
use crate::mod_flags::{ImGuiModFlags, ImGuiModFlags_None};
use crate::mouse_button::{ImGuiMouseButton, ImGuiMouseButton_Left};
use crate::mouse_cursor::{ImGuiMouseCursor, ImGuiMouseCursor_Arrow};
use crate::nav_item_data::ImGuiNavItemData;
use crate::nav_layer::ImGuiNavLayer_Main;
use crate::nav_move_flags::{ImGuiNavMoveFlags, ImGuiNavMoveFlags_None};
use crate::next_item_data::ImGuiNextItemData;
use crate::next_window_data::ImGuiNextWindowData;
use crate::payload::ImGuiPayload;
use crate::platform_ime_data::ImGuiPlatformImeData;
use crate::platform_monitor::ImGuiPlatformMonitor;
use crate::pool::ImPool;
use crate::popup_data::ImGuiPopupData;
use crate::ptr_or_index::ImGuiPtrOrIndex;
use crate::scroll_flags::{ImGuiScrollFlags, ImGuiScrollFlags_None};
use crate::settings_handler::ImGuiSettingsHandler;
use crate::shrink_width_item::ImGuiShrinkWidthItem;
use crate::stack_tool::ImGuiStackTool;
use crate::style_mod::ImGuiStyleMod;
use crate::tab_bar::ImGuiTabBar;
use crate::table::ImGuiTable;
use crate::table_settings::ImGuiTableSettings;
use crate::table_temp_data::ImGuiTableTempData;
use crate::type_defs::{ImBitArrayForNamedKeys, ImFileHandle, ImGuiDir, ImGuiID};
use crate::window_settings::ImGuiWindowSettings;
use crate::window_stack_data::ImGuiWindowStackData;

#[derive(Default, Debug, Clone)]
pub struct ImGuiContext {
    // bool                    Initialized;
    pub Initialized: bool,

    // bool                    FontAtlasOwnedByContext;            // IO.Fonts. is owned by the ImGuiContext and will be destructed along with it.
    pub FontAtlasOwnedByContext: bool,

    // ImGuiIO                 IO;
    pub IO: ImGuiIO,

    // ImGuiPlatformIO         PlatformIO;
    pub PlatformIO: ImGuiPlatformIO,

    // ImVector<ImGuiInputEvent> InputEventsQueue;                 // Input events which will be tricked/written into IO structure.
    pub InputEventsQueue: Vec<ImGuiInputEvent>,

    // ImVector<ImGuiInputEvent> InputEventsTrail;                 // Past input events processed in NewFrame(). This is to allow domain-specific application to access e.g mouse/pen trail.
    pub InputEventsTrail: Vec<ImGuiInputEvent>,

    // ImGuiStyle              Style;
    pub Style: ImGuiStyle,

    // ImGuiConfigFlags        ConfigFlagsCurrFrame;               // = g.IO.ConfigFlags at the time of NewFrame()
    pub ConfigFlagsCurrFrame: ImGuiConfigFlags,

    // ImGuiConfigFlags        ConfigFlagsLastFrame;
    pub ConfigFlagsLastFrame: ImGuiConfigFlags,

    // ImFont*                 Font;                               // (Shortcut) == FontStack.empty() ? IO.Font : FontStack.back()
    pub Font: *mut ImFont,

    // float                   FontSize;                           // (Shortcut) == FontBaseSize * g.Currentwindow.FontWindowScale == window.FontSize(). Text height for current window.
    pub FontSize: f32,

    // float                   FontBaseSize;                       // (Shortcut) == IO.FontGlobalScale * Font->Scale * Font->FontSize. Base text height.
    pub FontBaseSize: c_float,

    // ImDrawListSharedData    DrawListSharedData;
    pub DrawListSharedData: ImDrawListSharedData,

    // double                  Time;
    pub Time: c_double,

    // int                     FrameCount;
    pub FrameCount: c_int,

    // int                     FrameCountEnded;
    pub FrameCountEnded: c_int,

    // int                     FrameCountPlatformEnded;
    pub FrameCountPlatformEnded: c_int,

    // int                     FrameCountRendered;
    pub FrameCountRendered: c_int,

    // bool                    WithinFrameScope;                   // Set by NewFrame(), cleared by EndFrame()
    pub WithinFrameScope: bool,

    // bool                    WithinFrameScopeWithImplicitWindow; // Set by NewFrame(), cleared by EndFrame() when the implicit debug window has been pushed
    pub WithinFrameScopeWithImplicitWindow: bool,

    // bool                    WithinEndChild;                     // Set within EndChild()
    pub WithinEndChild: bool,

    // bool                    GcCompactAll;                       // Request full GC
    pub GcCompactAll: bool,

    // bool                    TestEngineHookItems;                // Will call test engine hooks: ImGuiTestEngineHook_ItemAdd(), ImGuiTestEngineHook_ItemInfo(), ImGuiTestEngineHook_Log()
    pub TestEngineHookItems: bool,

    // void*                   TestEngine;                         // Test engine user data
    pub TestEngine: *mut c_void,

    // Windows state

    // ImVector<ImGuiWindow*>  Windows;                            // Windows, sorted in display order, back to front
    pub Windows: Vec<*mut ImGuiWindow>,

    // ImVector<ImGuiWindow*>  WindowsFocusOrder;                  // Root windows, sorted in focus order, back to front.
    pub WindowsFocusOrder: Vec<*mut ImGuiWindow>,

    // ImVector<ImGuiWindow*>  WindowsTempSortBuffer;              // Temporary buffer used in EndFrame() to reorder windows so parents are kept before their child
    pub WindowsTempSortBuffer: Vec<*mut ImGuiWindow>,

    // ImVector<ImGuiWindowStackData> CurrentWindowStack;
    pub CurrentWindowStack: Vec<ImGuiWindowStackData>,

    // ImGuiStorage            WindowsById;                        // Map window's ImGuiID to ImGuiWindow*
    pub WindowsById: ImGuiStorage,

    // int                     WindowsActiveCount;                 // Number of unique windows submitted by frame
    pub WindowsActiveCount: i32,

    // ImVec2                  WindowsHoverPadding;                // Padding around resizable windows for which hovering on counts as hovering the window == ImMax(style.TouchExtraPadding, WINDOWS_HOVER_PADDING)
    pub WindowsHoverPadding: ImVec2,

    // ImGuiWindow*            CurrentWindow;                      // Window being drawn into
    pub CurrentWindow: *mut ImGuiWindow,

    // ImGuiWindow*            HoveredWindow;                      // Window the mouse is hovering. Will typically catch mouse inputs.
    pub HoveredWindow: *mut ImGuiWindow,

    // ImGuiWindow*            HoveredWindowUnderMovingWindow;     // Hovered window ignoring MovingWindow. Only set if MovingWindow is set.
    pub HoveredWindowUnderMovingWindow: *mut ImGuiWindow,

    // ImGuiWindow*            MovingWindow;                       // Track the window we clicked on (in order to preserve focus). The actual window that is moved is generally Movingwindow.RootWindowDockTree.
    pub MovingWindow: *mut ImGuiWindow,

    // ImGuiWindow*            WheelingWindow;                     // Track the window we started mouse-wheeling on. Until a timer elapse or mouse has moved, generally keep scrolling the same window even if during the course of scrolling the mouse ends up hovering a child window.
    pub WheelingWindow: *mut ImGuiWindow,

    // ImVec2                  WheelingWindowRefMousePos;
    pub WheelingWindowRefMousePos: ImVec2,

    // float                   WheelingWindowTimer;
    pub WheelingWindowTimer: f32,

    // Item/widgets state and tracking information

    // ImGuiID                 DebugHookIdInfo;                    // Will call core hooks: DebugHookIdInfo() from GetID functions, used by Stack Tool [next HoveredId/ActiveId to not pull in an extra cache-line]
    pub DebugHookIdInfo: ImGuiID,

    // ImGuiID                 HoveredId;                          // Hovered widget, filled during the frame
    pub HoveredId: ImGuiID,

    // ImGuiID                 HoveredIdPreviousFrame;
    pub HoveredIdPreviousFrame: ImGuiID,

    // bool                    HoveredIdAllowOverlap;
    pub HoveredIdAllowOverlap: bool,

    // bool                    HoveredIdUsingMouseWheel;           // Hovered widget will use mouse wheel. Blocks scrolling the underlying window.
    pub HoveredIdUsingMouseWheel: bool,

    // bool                    HoveredIdPreviousFrameUsingMouseWheel;
    pub HoveredIdPreviousFrameUsingMouseWheel: bool,

    // bool                    HoveredIdDisabled;                  // At least one widget passed the rect test, but has been discarded by disabled flag or popup inhibit. May be true even if HoveredId == 0.
    pub HoveredIdDisabled: bool,

    // float                   HoveredIdTimer;                     // Measure contiguous hovering time
    pub HoveredIdTimer: f32,

    // float                   HoveredIdNotActiveTimer;            // Measure contiguous hovering time where the item has not been active
    pub HoveredIdNotActiveTimer: f32,

    // ImGuiID                 ActiveId;                           // Active widget
    pub ActiveId: ImGuiID,

    // ImGuiID                 ActiveIdIsAlive;                    // Active widget has been seen this frame (we can't use a as: bool the ActiveId may change within the frame)
    pub ActiveIdIsAlive: ImGuiID,

    // float                   ActiveIdTimer;
    pub ActiveIdTimer: f32,

    // bool                    ActiveIdIsJustActivated;            // Set at the time of activation for one frame
    pub ActiveIdIsJustActivated: bool,

    // bool                    ActiveIdAllowOverlap;               // Active widget allows another widget to steal active id (generally for overlapping widgets, but not always)
    pub ActiveIdAllowOverlap: bool,

    // bool                    ActiveIdNoClearOnFocusLoss;         // Disable losing active id if the active id window gets unfocused.
    pub ActiveIdNoClearOnFocusLoss: bool,

    // bool                    ActiveIdHasBeenPressedBefore;       // Track whether the active id led to a press (this is to allow changing between PressOnClick and PressOnRelease without pressing twice). Used by range_select branch.
    pub ActiveIdHasBeenPressedBefore: bool,

    // bool                    ActiveIdHasBeenEditedBefore;        // Was the value associated to the widget Edited over the course of the Active state.
    pub ActiveIdHasBeenEditedBefore: bool,

    // bool                    ActiveIdHasBeenEditedThisFrame;
    pub ActiveIdHasBeenEditedThisFrame: bool,

    // ImVec2                  ActiveIdClickOffset;                // Clicked offset from upper-left corner, if applicable (currently only set by ButtonBehavior)
    pub ActiveIdClickOffset: ImVec2,

    // ImGuiWindow*            ActiveIdWindow;
    pub ActiveIdWindow: *mut ImGuiWindow,

    // ImGuiInputSource        ActiveIdSource;                     // Activating with mouse or nav (gamepad/keyboard)
    pub ActiveIdSource: ImGuiInputSource,

    // int                     ActiveIdMouseButton;
    pub ActiveIdMouseButton: c_int,

    // ImGuiID                 ActiveIdPreviousFrame;
    pub ActiveIdPreviousFrame: ImGuiID,

    // bool                    ActiveIdPreviousFrameIsAlive;
    pub ActiveIdPreviousFrameIsAlive: bool,

    // bool                    ActiveIdPreviousFrameHasBeenEditedBefore;
    pub ActiveIdPreviousFrameHasBeenEditedBefore: bool,

    // ImGuiWindow*            ActiveIdPreviousFrameWindow;
    pub ActiveIdPreviousFrameWindow: *mut ImGuiWindow,

    // ImGuiID                 LastActiveId;                       // Store the last non-zero ActiveId, useful for animation.
    pub LastActiveId: ImGuiID,

    // float                   LastActiveIdTimer;                  // Store the last non-zero ActiveId timer since the beginning of activation, useful for animation.
    pub LastActiveIdTimer: f32,

    // Input Ownership

    // u32                   ActiveIdUsingNavDirMask;            // Active widget will want to read those nav move requests (e.g. can activate a button and move away from it)
    pub ActiveIdUsingNavDirMask: u32,

    // ImBitArrayForNamedKeys  ActiveIdUsingKeyInputMask;          // Active widget will want to read those key inputs. When we grow the ImGuiKey enum we'll need to either to order the enum to make useful keys come first, either redesign this into e.g. a small array.
    pub ActiveIdUsingKeyInputMask: ImBitArrayForNamedKeys,
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO

    // u32                   ActiveIdUsingNavInputMask;          // If you used this. Since (IMGUI_VERSION_NUM >= 18804) : 'g.ActiveIdUsingNavInputMask |= (1 << ImGuiNavInput_Cancel);' becomes 'SetActiveIdUsingKey(ImGuiKey_Escape); SetActiveIdUsingKey(ImGuiKey_NavGamepadCancel);'
// #endif

    // Next window/item data
    // ImGuiItemFlags          CurrentItemFlags;                   // == g.ItemFlagsStack.back()
    pub CurrentItemFlags: ImGuiItemFlags,

    // ImGuiNextItemData       NextItemData;                       // Storage for SetNextItem** functions
    pub NextItemData: ImGuiNextItemData,

    // ImGuiLastItemData       LastItemData;                       // Storage for last submitted item (setup by ItemAdd)
    pub LastItemData: ImGuiLastItemData,

    // ImGuiNextWindowData     NextWindowData;                     // Storage for SetNextWindow** functions
    pub NextWindowData: ImGuiNextWindowData,

    // Shared stacks
    // ImVector<ImGuiColorMod> ColorStack;                         // Stack for PushStyleColor()/PopStyleColor() - inherited by Begin()
    pub ColorStack: Vec<ImGuiColorMod>,

    // ImVector<ImGuiStyleMod> StyleVarStack;                      // Stack for PushStyleVar()/PopStyleVar() - inherited by Begin()
    pub StyleVarStack: Vec<ImGuiStyleMod>,

    // ImVector<ImFont*>       FontStack;                          // Stack for PushFont()/PopFont() - inherited by Begin()
    pub FontStack: Vec<ImFont>,

    // ImVector<ImGuiID>       FocusScopeStack;                    // Stack for PushFocusScope()/PopFocusScope() - not inherited by Begin(), unless child window
    pub FocusScopeStack: Vec<ImGuiID>,

    // ImVector<ImGuiItemFlags>ItemFlagsStack;                     // Stack for PushItemFlag()/PopItemFlag() - inherited by Begin()
    pub ItemFlagsStack: Vec<ImGuiItemFlags>,

    // ImVector<ImGuiGroupData>GroupStack;                         // Stack for BeginGroup()/EndGroup() - not inherited by Begin()
    pub GroupStack: Vec<ImGuiGroupData>,

    // ImVector<ImGuiPopupData>OpenPopupStack;                     // Which popups are open (persistent)
    pub OpenPopupStack: Vec<ImGuiPopupData>,

    // ImVector<ImGuiPopupData>BeginPopupStack;                    // Which level of BeginPopup() we are in (reset every frame)
    pub BeginPopupStack: Vec<ImGuiPopupData>,

    // int                     BeginMenuCount;
    pub BeginMenuCount: Vec<c_int>,

    // Viewports

    // ImVector<ImGuiViewportP*> Viewports;                        // Active viewports (always 1+, and generally 1 unless multi-viewports are enabled). Each viewports hold their copy of ImDrawData.
    pub Viewports: Vec<*mut ImGuiViewport>,

    // float                   CurrentDpiScale;                    // == CurrentViewport->DpiScale
    pub CurrentDpiScale: c_float,

    // ImGuiViewportP*         CurrentViewport;                    // We track changes of viewport (happening in Begin) so we can call Platform_OnChangedViewport()
    pub CurrentViewport: *mut ImGuiViewport,

    // ImGuiViewportP*         MouseViewport;
    pub MouseViewport: *mut ImGuiViewport,

    // ImGuiViewportP*         MouseLastHoveredViewport;           // Last known viewport that was hovered by mouse (even if we are not hovering any viewport any more) + honoring the _NoInputs flag.
    pub MouseLastHoveredViewport: *mut ImGuiViewport,

    // ImGuiID                 PlatformLastFocusedViewportId;
    pub PlatformLastFocusedViewportId: ImGuiID,

    // ImGuiPlatformMonitor    FallbackMonitor;                    // Virtual monitor used as fallback if backend doesn't provide monitor information.
    pub FallbackMonitor: ImGuiPlatformMonitor,

    // int                     ViewportFrontMostStampCount;        // Every time the front-most window changes, we stamp its viewport with an incrementing counter
    pub ViewportFrontMostStampCount: c_int,

    // Gamepad/keyboard Navigation

    // ImGuiWindow*            NavWindow;                          // Focused window for navigation. Could be called 'FocusedWindow'
    pub NavWindow: *mut ImGuiWindow,

    // ImGuiID                 NavId;                              // Focused item for navigation
    pub NavId: ImGuiID,

    // ImGuiID                 NavFocusScopeId;                    // Identify a selection scope (selection code often wants to "clear other items" when landing on an item of the selection set)
    pub NavFocusScopeId: ImGuiID,

    // ImGuiID                 NavActivateId;                      // ~~ (g.ActiveId == 0) && (IsKeyPressed(ImGuiKey_Space) || IsKeyPressed(ImGuiKey_NavGamepadActivate)) ? NavId : 0, also set when calling ActivateItem()
    pub NavActivateId: ImGuiID,

    // ImGuiID                 NavActivateDownId;                  // ~~ IsKeyDown(ImGuiKey_Space) || IsKeyDown(ImGuiKey_NavGamepadActivate) ? NavId : 0
    pub NavActivateDownId: ImGuiID,

    // ImGuiID                 NavActivatePressedId;               // ~~ IsKeyPressed(ImGuiKey_Space) || IsKeyPressed(ImGuiKey_NavGamepadActivate) ? NavId : 0 (no repeat)
    pub NavActivatePressedId: ImGuiID,

    // ImGuiID                 NavActivateInputId;                 // ~~ IsKeyPressed(ImGuiKey_Enter) || IsKeyPressed(ImGuiKey_NavGamepadInput) ? NavId : 0; ImGuiActivateFlags_PreferInput will be set and NavActivateId will be 0.
    pub NavActivateInputId: ImGuiID,

    // ImGuiActivateFlags      NavActivateFlags;
    pub NavActivateFlags: ImGuiActivateFlags,

    // ImGuiID                 NavJustMovedToId;                   // Just navigated to this id (result of a successfully MoveRequest).
    pub NavJustMovedToId: ImGuiID,

    // ImGuiID                 NavJustMovedToFocusScopeId;         // Just navigated to this focus scope id (result of a successfully MoveRequest).
    pub NavJustMovedToFocusScopeId: ImGuiID,

    // ImGuiModFlags           NavJustMovedToKeyMods;
    pub NavJustMovedToKeyMods: ImGuiModFlags,

    // ImGuiID                 NavNextActivateId;                  // Set by ActivateItem(), queued until next frame.
    pub NavNextActivateId: ImGuiID,

    // ImGuiActivateFlags      NavNextActivateFlags;
    pub NavNextActivateFlags: ImGuiActivateFlags,

    // ImGuiInputSource        NavInputSource;                     // Keyboard or Gamepad mode? THIS WILL ONLY BE None or NavGamepad or NavKeyboard.
    pub NavInputSource: ImGuiInputSource,

    // ImGuiNavLayer           NavLayer;                           // Layer we are navigating on. For now the system is hard-coded for 0=main contents and 1=menu/title bar, may expose layers later.
    pub NavLayer: ImGuiNavLayer,

    // bool                    NavIdIsAlive;                       // Nav widget has been seen this frame ~~ NavRectRel is valid
    pub NavIdIsAlive: bool,

    // bool                    NavMousePosDirty;                   // When set we will update mouse position if (io.ConfigFlags & ImGuiConfigFlags_NavEnableSetMousePos) if set (NB: this not enabled by default)
    pub NavMousePosDirty: bool,

    // bool                    NavDisableHighlight;                // When user starts using mouse, we hide gamepad/keyboard highlight (NB: but they are still available, which is why NavDisableHighlight isn't always != NavDisableMouseHover)
    pub NavDisableHighlight: bool,

    // bool                    NavDisableMouseHover;               // When user starts using gamepad/keyboard, we hide mouse hovering highlight until mouse is touched again.
    pub NavDisableMouseHover: bool,

    // Navigation: Init & Move Requests

    // bool                    NavAnyRequest;                      // ~~ NavMoveRequest || NavInitRequest this is to perform early out in ItemAdd()
    pub NavAnyRequest: bool,

    // bool                    NavInitRequest;                     // Init request for appearing window to select first item
    pub NavInitRequest: bool,

    // bool                    NavInitRequestFromMove;
    pub NavInitRequestFromMove: bool,

    // ImGuiID                 NavInitResultId;                    // Init request result (first item of the window, or one for which SetItemDefaultFocus() was called)
    pub NavInitResultId: ImGuiID,

    // ImRect                  NavInitResultRectRel;               // Init request result rectangle (relative to parent window)
    pub NavInitResultRectRel: ImRect,

    // bool                    NavMoveSubmitted;                   // Move request submitted, will process result on next NewFrame()
    pub NavMoveSubmitted: bool,

    // bool                    NavMoveScoringItems;                // Move request submitted, still scoring incoming items
    pub NavMoveScoringItems: bool,

    // bool                    NavMoveForwardToNextFrame;
    pub NavMoveForwardToNextFrame: bool,

    // ImGuiNavMoveFlags       NavMoveFlags;
    pub NavMoveFlags: ImGuiNavMoveFlags,

    // ImGuiScrollFlags        NavMoveScrollFlags;
    pub NavMoveScrollFlags: ImGuiScrollFlags,

    // ImGuiModFlags           NavMoveKeyMods;
    pub NavMoveKeyMods: ImGuiModFlags,

    // ImGuiDir                NavMoveDir;                         // Direction of the move request (left/right/up/down)
    pub NavMoveDir: ImGuiDir,

    // ImGuiDir                NavMoveDirForDebug;
    pub NavMoveDirForDebug: ImGuiDir,

    // ImGuiDir                NavMoveClipDir;                     // FIXME-NAV: Describe the purpose of this better. Might want to rename?
    pub NavMoveClipDir: ImGuiDir,

    // ImRect                  NavScoringRect;                     // Rectangle used for scoring, in screen space. Based of window.NavRectRel[], modified for directional navigation scoring.
    pub NavScoringRect: ImRect,

    // ImRect                  NavScoringNoClipRect;               // Some nav operations (such as PageUp/PageDown) enforce a region which clipper will attempt to always keep submitted
    pub NavScoringNoClipRect: ImRect,

    // int                     NavScoringDebugCount;               // Metrics for debugging
    pub NavScoringDebugCount: c_int,

    // int                     NavTabbingDir;                      // Generally -1 or +1, 0 when tabbing without a nav id
    pub NavTabbingDir: c_int,

    // int                     NavTabbingCounter;                  // >0 when counting items for tabbing
    pub NavTabbingCounter: c_int,

    // ImGuiNavItemData        NavMoveResultLocal;                 // Best move request candidate within NavWindow
    pub NavMoveResultLocal: ImGuiNavItemData,

    // ImGuiNavItemData        NavMoveResultLocalVisible;          // Best move request candidate within NavWindow that are mostly visible (when using ImGuiNavMoveFlags_AlsoScoreVisibleSet flag)
    pub NavMoveResultLocalVisible: ImGuiNavItemData,

    // ImGuiNavItemData        NavMoveResultOther;                 // Best move request candidate within NavWindow's flattened hierarchy (when using ImGuiWindowFlags_NavFlattened flag)
    pub NavMoveResultOther: ImGuiNavItemData,

    // ImGuiNavItemData        NavTabbingResultFirst;              // First tabbing request candidate within NavWindow and flattened hierarchy
    pub NavTabbingResultFirst: ImGuiNavItemData,

    // Navigation: Windowing (CTRL+TAB for list, or Menu button + keys or directional pads to move/resize)
    // ImGuiWindow*            NavWindowingTarget;                 // Target window when doing CTRL+Tab (or Pad Menu + FocusPrev/Next), this window is temporarily displayed top-most!
    pub NavWindowingTarget: *mut ImGuiWindow,

    // ImGuiWindow*            NavWindowingTargetAnim;             // Record of last valid NavWindowingTarget until DimBgRatio and NavWindowingHighlightAlpha becomes 0f32, so the fade-out can stay on it.
    pub NavWindowingTargetAnim: *mut ImGuiWindow,

    // ImGuiWindow*            NavWindowingListWindow;             // Internal window actually listing the CTRL+Tab contents
    pub NavWindowingListWindow: *mut ImGuiWindow,

    // float                   NavWindowingTimer;
    pub NavWindowingTimer: c_float,

    // float                   NavWindowingHighlightAlpha;
    pub NavWindowingHighlightAlpha: c_float,

    // bool                    NavWindowingToggleLayer;
    pub NavWindowingToggleLayer: bool,

    // ImVec2                  NavWindowingAccumDeltaPos;
    pub NavWindowingAccumDeltaPos: ImVec2,

    // ImVec2                  NavWindowingAccumDeltaSize;
    pub NavWindowingAccumDeltaSize: ImVec2,

    // Render

    // float                   DimBgRatio;                         // 0.0..1.0 animation when fading in a dimming background (for modal window and CTRL+TAB list)
    pub DimBgRatio: c_float,

    // ImGuiMouseCursor        MouseCursor;
    pub MouseCursor: ImGuiMouseCursor,

    // Drag and Drop
    // bool                    DragDropActive;
    pub DragDropActive: bool,

    // bool                    DragDropWithinSource;               // Set when within a BeginDragDropXXX/EndDragDropXXX block for a drag source.
    pub DragDropWithinSource: bool,

    // bool                    DragDropWithinTarget;               // Set when within a BeginDragDropXXX/EndDragDropXXX block for a drag target.
    pub DragDropWithinTarget: bool,

    // ImGuiDragDropFlags      DragDropSourceFlags;
    pub DragDropSourceFlags: ImGuiDragDropFlags,

    // int                     DragDropSourceFrameCount;
    pub DragDropSourceFrameCount: c_int,

    // int                     DragDropMouseButton;
    pub DragDropMouseButton: c_int,

    // ImGuiPayload            DragDropPayload;
    pub DragDropPayload: ImGuiPayload,

    // ImRect                  DragDropTargetRect;                 // Store rectangle of current target candidate (we favor small targets when overlapping)
    pub DragDropTargetRect: ImRect,

    // ImGuiID                 DragDropTargetId;
    pub DragDropTargetId: ImGuiID,

    // ImGuiDragDropFlags      DragDropAcceptFlags;
    pub DragDropAcceptFlags: ImGuiDragDropFlags,

    // float                   DragDropAcceptIdCurrRectSurface;    // Target item surface (we resolve overlapping targets by prioritizing the smaller surface)
    pub DragDropAcceptIdCurrRectSurface: c_float,

    // ImGuiID                 DragDropAcceptIdCurr;               // Target item id (set at the time of accepting the payload)
    pub DragDropAcceptIdCurr: ImGuiID,

    // ImGuiID                 DragDropAcceptIdPrev;               // Target item id from previous frame (we need to store this to allow for overlapping drag and drop targets)
    pub DragDropAcceptIdPrev: ImGuiID,

    // int                     DragDropAcceptFrameCount;           // Last time a target expressed a desire to accept the source
    pub DragDropAcceptFrameCount: c_int,

    // ImGuiID                 DragDropHoldJustPressedId;          // Set when holding a payload just made ButtonBehavior() return a press.
    pub DragDropHoldJustPressedId: ImGuiID,

    // ImVector<unsigned char> DragDropPayloadBufHeap;             // We don't expose the ImVector<> directly, ImGuiPayload only holds pointer+size
    pub DragDropPayloadBufHead: Vec<c_uchar>,

    // unsigned char           DragDropPayloadBufLocal[16];        // Local buffer for small payloads
    pub DragDropPayloadBufLocal: [c_uchar; 16],

    // Clipper

    // int                             ClipperTempDataStacked;
    pub ClipperTempDataStacked: usize,

    // ImVector<ImGuiListClipperData>  ClipperTempData;
    pub ClipperTempData: Vec<ImGuiListClipperData>,

    // Tables

    // ImGuiTable*                     CurrentTable;
    pub CurrentTable: *mut ImGuiTable,

    // int                             TablesTempDataStacked;      // Temporary table data size (because we leave previous instances undestructed, we generally don't use TablesTempData.Size)
    pub TablesTempDataStacked: c_int,

    // ImVector<ImGuiTableTempData>    TablesTempData;             // Temporary table data (buffers reused/shared across instances, support nesting)
    pub TablesTempData: Vec<ImGuiTableTempData>,

    // ImPool<ImGuiTable>              Tables;                     // Persistent table data
    pub Tables: ImPool<ImGuiTable>,

    // ImVector<float>                 TablesLastTimeActive;       // Last used timestamp of each tables (SOA, for efficient GC)
    pub TablesLastTimeActive: Vec<c_float>,

    // ImVector<ImDrawChannel>         DrawChannelsTempMergeBuffer;
    pub DrawChannelsTempMergeBuffer: Vec<ImDrawChannel>,

    // Tab bars

    // ImGuiTabBar*                    CurrentTabBar;
    pub CurrentTabBar: *mut ImGuiTabBar,

    // ImPool<ImGuiTabBar>             TabBars;
    pub TabBars: ImPool<ImGuiTabBar>,

    // ImVector<ImGuiPtrOrIndex>       CurrentTabBarStack;
    pub CurrentTabBarStack: Vec<ImGuiPtrOrIndex>,

    // ImVector<ImGuiShrinkWidthItem>  ShrinkWidthBuffer;
    pub ShrunkWidthBuffer: Vec<ImGuiShrinkWidthItem>,

    // Hover Delay system

    // ImGuiID                 HoverDelayId;
    pub HoverDelayId: ImGuiID,

    // ImGuiID                 HoverDelayIdPreviousFrame;
    pub HoverDelayIdPreviousFrame: ImGuiID,

    // float                   HoverDelayTimer;                    // Currently used IsItemHovered(), generally inferred from g.HoveredIdTimer but kept uncleared until clear timer elapse.
    pub HoverDelayTimer: c_float,

    // float                   HoverDelayClearTimer;               // Currently used IsItemHovered(): grace time before g.TooltipHoverTimer gets cleared.
    pub HoverDelayClearTimer: c_float,

    // Widget state
    // ImVec2                  MouseLastValidPos;
    pub MouseLastValidPos: ImVec2,

    // ImGuiInputTextState     InputTextState;
    pub InputTextState: ImGuiInputTextState,

    // ImFont                  InputTextPasswordFont;
    pub InputTextPasswordFont: ImFont,

    // ImGuiID                 TempInputId;                        // Temporary text input when CTRL+clicking on a slider, etc.
    pub TempInputId: ImGuiID,

    // ImGuiColorEditFlags     ColorEditOptions;                   // Store user options for color edit widgets
    pub ColorEditOptions: ImGuiColorEditFlags,

    // float                   ColorEditLastHue;                   // Backup of last Hue associated to LastColor, so we can restore Hue in lossy RGB<>HSV round trips
    pub ColorEditLastHue: c_float,

    // float                   ColorEditLastSat;                   // Backup of last Saturation associated to LastColor, so we can restore Saturation in lossy RGB<>HSV round trips
    pub ColorEditLastSat: c_float,

    // u32                   ColorEditLastColor;                 // RGB value with alpha set to 0.
    pub ColorEditLastColor: u32,

    // ImVec4                  ColorPickerRef;                     // Initial/reference color at the time of opening the color picker.
    pub ColorPickerRef: ImVec4,

    // ImGuiComboPreviewData   ComboPreviewData;
    pub ComboPreviewData: ImGuiComboPreviewData,

    // float                   SliderGrabClickOffset;
    pub SliderGrabClickOffset: c_float,

    // float                   SliderCurrentAccum;                 // Accumulated slider delta when using navigation controls.
    pub SliderCurrentAccum: c_float,

    // bool                    SliderCurrentAccumDirty;            // Has the accumulated slider delta changed since last time we tried to apply it?
    pub SliderCurrentAccumDirty: bool,

    // bool                    DragCurrentAccumDirty;
    pub DragCurrentAccumDirty: bool,

    // float                   DragCurrentAccum;                   // Accumulator for dragging modification. Always high-precision, not rounded by end-user precision settings
    pub DragCurrentAccum: c_float,

    // float                   DragSpeedDefaultRatio;              // If speed == 0f32, uses (max-min) * DragSpeedDefaultRatio
    pub DragSpeedDefaultRatio: c_float,

    // float                   ScrollbarClickDeltaToGrabCenter;    // Distance between mouse and center of grab box, normalized in parent space. Use storage?
    pub ScrollbarClickDeltaToGrabCenter: c_float,

    // float                   DisabledAlphaBackup;                // Backup for style.Alpha for BeginDisabled()
    pub DisabledAlphaBackup: c_float,

    // short                   DisabledStackSize;
    pub DisabledStackSize: i16,

    // short                   TooltipOverrideCount;
    pub TooltipOverrideCount: i16,

    // ImVector<char>          ClipboardHandlerData;               // If no custom clipboard handler is defined
    pub ClipboardHandlerData: Vec<c_char>,

    // ImVector<ImGuiID>       MenusIdSubmittedThisFrame;          // A list of menu IDs that were rendered at least once
    pub MenusIdSubmittedThisFrame: Vec<ImGuiID>,

    // Platform support

    // ImGuiPlatformImeData    PlatformImeData;                    // Data updated by current frame
    pub PlatformImeData: ImGuiPlatformImeData,

    // ImGuiPlatformImeData    PlatformImeDataPrev;                // Previous frame data (when changing we will call io.SetPlatformImeDataFn
    pub PlatformImeDataPrev: ImGuiPlatformImeData,

    // ImGuiID                 PlatformImeViewport;
    pub PlatformImeViewport: ImGuiID,

    // char                    PlatformLocaleDecimalPoint;         // '.' or *localeconv()->decimal_point
    pub PlatformLocaleDecimalPoint: c_char,

    // Extensions
    // FIXME: We could provide an API to register one slot in an array held in ImGuiContext?
    // ImGuiDockContext        DockContext;
    pub DockContext: ImGuiDockContext,

    // Settings
    // bool                    SettingsLoaded;
    pub SettingsLoaded: bool,

    // float                   SettingsDirtyTimer;                 // Save .ini Settings to memory when time reaches zero
    pub SettingsDirtyTimer: c_float,

    // ImGuiTextBuffer         SettingsIniData;                    // In memory .ini settings
    pub SettingsIniData: ImGuiTextBuffer,

    // ImVector<ImGuiSettingsHandler>      SettingsHandlers;       // List of .ini settings handlers
    pub SettingsHandlers: Vec<ImGuiSettingsHandler>,

    // ImChunkStream<ImGuiWindowSettings>  SettingsWindows;        // ImGuiWindow .ini settings entries
    pub SettingsWindow: Vec<ImGuiWindowSettings>,

    // ImChunkStream<ImGuiTableSettings>   SettingsTables;         // ImGuiTable .ini settings entries
    pub SettingsTables: ImChunkStream<ImGuiTableSettings>,

    // ImVector<ImGuiContextHook>          Hooks;                  // Hooks for extensions (e.g. test engine)
    pub Hooks: Vec<ImGuiContextHook>,

    // ImGuiID                             HookIdNext;             // Next available HookId
    pub HookIdNext: Vec<ImGuiID>,

    // Capture/Logging

    // bool                    LogEnabled;                         // Currently capturing
    pub LogEnabled: bool,

    // ImGuiLogType            LogType;                            // Capture target
    pub LogType: ImGuiLogType,

    // ImFileHandle            LogFile;                            // If != NULL log to stdout/ file
    pub LogFile: ImFileHandle,

    // ImGuiTextBuffer         LogBuffer;                          // Accumulation buffer when log to clipboard. This is pointer so our GImGui static constructor doesn't call heap allocators.
    pub LogBuffer: ImGuiTextBuffer,

    // const char*             LogNextPrefix;
    pub LogNextPrefix: *const c_char,

    // const char*             LogNextSuffix;
    pub LogNextSuffix: *const c_char,

    // float                   LogLinePosY;
    pub LogLinePosY: c_float,

    // bool                    LogLineFirstItem;
    pub LogLineFirstItem: bool,

    // int                     LogDepthRef;
    pub LogDepthRef: c_int,

    // int                     LogDepthToExpand;
    pub LogDepthToExpand: c_int,

    // int                     LogDepthToExpandDefault;            // Default/stored value for LogDepthMaxExpand if not specified in the LogXXX function call.
    pub LogDepthToExpandDefault: c_int,

    // Debug Tools
    // ImGuiDebugLogFlags      DebugLogFlags;
    pub DebugLogFlags: ImGuiDebugLogFlags,

    // ImGuiTextBuffer         DebugLogBuf;
    pub DebugLogBuf: ImGuiTextBuffer,

    // bool                    DebugItemPickerActive;              // Item picker is active (started with DebugStartItemPicker())
    pub DebugItemPickerActive: bool,

    // ImU8                    DebugItemPickerMouseButton;
    pub DebugItemPickerMouseButton: ImGuiMouseButton,

    // ImGuiID                 DebugItemPickerBreakId;             // Will call IM_DEBUG_BREAK() when encountering this ID
    pub DebugItemPickerBreakId: ImGuiID,

    // ImGuiMetricsConfig      DebugMetricsConfig;
    pub DebugMetricsConfig: ImGuiMetricsConfig,

    // ImGuiStackTool          DebugStackTool;
    pub DebugStackTool: ImGuiStackTool,

    // ImGuiDockNode*          DebugHoveredDockNode;               // Hovered dock node.
    pub DebugHoveredDockNode: *mut ImGuiDockNode,

    // Misc
    // float                   FramerateSecPerFrame[60];           // Calculate estimate of framerate for user over the last 60 frames..
    pub FramerateSecPerFrame: [c_float; 60],

    // int                     FramerateSecPerFrameIdx;
    pub FramerateSecPerFrameIdx: c_int,

    // int                     FramerateSecPerFrameCount;
    pub FramerateSecPerFrameCount: c_int,

    // float                   FramerateSecPerFrameAccum;
    pub FramerateSecPerFrameAccum: c_float,

    // int                     WantCaptureMouseNextFrame;          // Explicit capture override via SetNextFrameWantCaptureMouse()/SetNextFrameWantCaptureKeyboard(). Default to -1.
    pub WantCaptureMouseNextFrame: c_int,

    // int                     WantCaptureKeyboardNextFrame;       // "
    pub WantCaptureKeyboardNextFrame: c_int,

    // int                     WantTextInputNextFrame;
    pub WantTextInputNextFrame: c_int,

    // ImVector<char>          TempBuffer;                         // Temporary text buffer
    pub TempBuffer: Vec<c_char>,
}

impl ImGuiContext {
    pub unsafe fn new(shared_font_atlas: *mut ImFontAtlas) -> Self {
        let mut out = Self {
            Initialized: false,
            ConfigFlagsCurrFrame: ImGuiConfigFlags_None,
            ConfigFlagsLastFrame: ImGuiConfigFlags_None,
            FontAtlasOwnedByContext: if shared_font_atlas.is_null() == false { false } else { true },
            Font: null_mut(),
            FontSize: 0.0,
            FontBaseSize: 0.0,
            Time: 0.0,
            FrameCount: 0,
            FrameCountEnded: -1,
            FrameCountPlatformEnded: -1,
            FrameCountRendered: -1,
            WithinFrameScope: false,
            WithinFrameScopeWithImplicitWindow: false,
            WithinEndChild: false,
            GcCompactAll: false,
            TestEngineHookItems: false,
            TestEngine: null_mut(),
            WindowsActiveCount: 0,
            CurrentWindow: null_mut(),
            HoveredWindow: null_mut(),
            HoveredWindowUnderMovingWindow: null_mut(),
            MovingWindow: null_mut(),
            WheelingWindow: null_mut(),
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
            ActiveIdClickOffset: ImVec2::new(-1f32, -1f32),
            ActiveIdWindow: null_mut(),
            ActiveIdSource: ImGuiInputSource_None,
            ActiveIdMouseButton: -1,
            ActiveIdPreviousFrame: 0,
            ActiveIdPreviousFrameIsAlive: false,
            ActiveIdPreviousFrameHasBeenEditedBefore: false,
            ActiveIdPreviousFrameWindow: null_mut(),
            LastActiveId: 0,
            LastActiveIdTimer: 0.0,
            ActiveIdUsingNavDirMask: 0x00,
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//             ActiveIdUsingNavInputMask : 0x00,
// #endif

            CurrentItemFlags: ImGuiItemFlags_None,
            BeginMenuCount: vec![],

            CurrentDpiScale: 0.0,
            CurrentViewport: null_mut(),
            MouseViewport: null_mut(),
            MouseLastHoveredViewport: null_mut(),
            PlatformLastFocusedViewportId: 0,
            ViewportFrontMostStampCount: 0,

            NavWindow: null_mut(),
            NavId: 0,
            NavFocusScopeId: 0,
            NavActivateId: 0,
            NavActivateDownId: 0,
            NavActivatePressedId: 0,
            NavActivateInputId: 0,
            NavJustMovedToId: 0,
            NavJustMovedToFocusScopeId: 0,
            NavNextActivateId: 0,
            NavActivateFlags: ImGuiActivateFlags_None,
            NavNextActivateFlags: ImGuiActivateFlags_None,
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

            NavWindowingTarget: null_mut(),
            NavWindowingTargetAnim: null_mut(),
            NavWindowingListWindow: null_mut(),
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

            ClipperTempDataStacked: 0,

            CurrentTable: null_mut(),
            TablesTempDataStacked: 0,
            CurrentTabBar: null_mut(),

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
            DragSpeedDefaultRatio: 1f32 / 100.0,
            ScrollbarClickDeltaToGrabCenter: 0.0,
            DisabledAlphaBackup: 0.0,
            DisabledStackSize: 0,
            TooltipOverrideCount: 0,

            PlatformImeViewport: 0,
            PlatformLocaleDecimalPoint: '.'.into(),

            SettingsLoaded: false,
            SettingsDirtyTimer: 0.0,
            HookIdNext: vec![],

            LogEnabled: false,
            LogType: ImGuiLogType_None,
            LogNextPrefix: null_mut(),
            LogNextSuffix: null_mut(),
            LogFile: null_mut(),
            LogLinePosY: f32::MAX,
            LogLineFirstItem: false,
            LogDepthRef: 0,
            LogDepthToExpand: 2,
            LogDepthToExpandDefault: 2,

            DebugLogFlags: ImGuiDebugLogFlags_OutputToTTY,
            DebugItemPickerActive: false,
            DebugItemPickerMouseButton: ImGuiMouseButton_Left,
            DebugItemPickerBreakId: 0,
            DebugHoveredDockNode: null_mut(),

            FramerateSecPerFrameIdx: 0,
            FramerateSecPerFrameCount: 0,
            FramerateSecPerFrameAccum: 0.0,
            WantCaptureMouseNextFrame: -1,
            WantCaptureKeyboardNextFrame: -1,
            WantTextInputNextFrame: -1,
            ..Default::default()
        };

        out.IO.Fonts = if shared_font_atlas.is_null() == false { shared_font_atlas } else { IM_NEW(ImFontAtlas)() };
        out.ActiveIdUsingKeyInputMask.ClearAllBits();
        out.PlatformImeData.InputPos = ImVec2::new();
        out.PlatformImeDataPrev.InputPos = ImVec2::new(-1f32, -1f32); // Different to ensure initial submission
        libc::memset(out.DragDropPayloadBufLocal.as_mut_ptr(), 0, libc::sizeof(out.DragDropPayloadBufLocal));
        libc::memset(out.FramerateSecPerFrame.as_mut_ptr(), 0, libc::sizeof(FramerateSecPerFrame));
        return out;
    }
}
