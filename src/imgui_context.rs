
//-----------------------------------------------------------------------------
// [SECTION] ImGuiContext (main Dear ImGui context)
//-----------------------------------------------------------------------------
use std::ffi::{c_void};
use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::imgui_clipper::ImGuiListClipperData;
use crate::imgui_color::ImGuiColorMod;
use crate::imgui_dock::ImGuiDockNode;
use crate::imgui_draw_list::ImDrawListSharedData;
use crate::imgui_group::ImGuiGroupData;
use crate::imgui_h::{ImFont, ImFontAtlas, ImGuiColorEditFlags, ImGuiComboFlags, ImGuiConfigFlags, ImGuiDir, ImGuiDragDropFlags, ImGuiID, ImGuiKey, ImGuiModFlags, ImGuiMouseCursor, ImGuiNavLayer, ImGuiPayload, ImGuiPlatformImeData, ImGuiPlatformMonitor, ImGuiPtrOrIndex, ImGuiShrinkWidthItem, ImGuiStyleVar, ImGuiTextBuffer, ImGuiViewport, ImVec4};
use crate::imgui_input::ImGuiInputSource;
use crate::imgui_input_event::ImGuiInputEvent;
use crate::imgui_io::{ImGuiIO, ImGuiPlatformIO};
use crate::imgui_item::{ImGuiLastItemData, ImGuiNextItemData};
use crate::imgui_kv_store::ImGuiStorage;
use crate::imgui_log::{ImGuiDebugLogFlags, ImGuiLogType};
use crate::imgui_nav::{ImGuiActivateFlags, ImGuiNavItemData, ImGuiNavMoveFlags, ImGuiScrollFlags};
use crate::imgui_pool::ImGuiPool;
use crate::imgui_popup::ImGuiPopupData;
use crate::imgui_rect::ImRect;
use crate::imgui_style::{ImGuiStyle, ImGuiStyleMod};
use crate::imgui_tab_bar::ImGuiTabBar;
use crate::imgui_table::{ImGuiTable, ImGuiTableTempData};
use crate::imgui_text_input_state::ImGuiInputTextState;
use crate::imgui_vec::ImVec2;
use crate::imgui_window::{ImGuiItemFlags, ImGuiNextWindowData, ImGuiWindow, ImGuiWindowStackData};

#[derive()]
pub struct ImGuiContext {
    // bool                    Initialized;
    pub Initialized: bool,
    // bool                    FontAtlasOwnedByContext;            // IO.Fonts-> is owned by the ImGuiContext and will be destructed along with it.
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
    // float                   FontSize;                           // (Shortcut) == FontBaseSize * g.CurrentWindow->FontWindowScale == window->FontSize(). Text height for current window.
    pub FontSize: f32,
    // float                   FontBaseSize;                       // (Shortcut) == IO.FontGlobalScale * Font->Scale * Font->FontSize. Base text height.
    pub FontBaseSize: f32,
    // ImDrawListSharedData    DrawListSharedData;
    pub DrawListSharedData: ImDrawListSharedData,
    // double                  Time;
    pub Time: f32,
    // int                     FrameCount;
    pub FrameCount: i32,
    //int                     FrameCountEnded;
    pub FrameCountEnded: i32,
    // int                     FrameCountPlatformEnded;
    pub FrameCountPlatformEnded: i32,
    // int                     FrameCountRendered;
    pub FrameCountRendered: i32,
    // bool                    WithinFrameScope;                   // Set by NewFrame(), cleared by EndFrame()
    pub WithinFrameScope: bool,
    // bool                    WithinFrameScopeWithImplicitWindow; // Set by NewFrame(), cleared by EndFrame() when the implicit debug window has been pushed
    pub WithinFrameScopeWithImplicitWindow: bool,
    // bool                    WithinEndChild;                     // Set within EndChild()
    pub WithinEndChild: bool,
    // bool                    GcCompactAll;                       // Request full GC
    pub GcCompactAll: bool,
    // bool                    TestEngineHookItems;                // Will call test engine hooks: ImGuiTestEngineHook_ItemAdd(), ImGuiTestEngineHook_ItemInfo(), ImGuiTestEngineHook_Log()
    pub TestENgineHookItems: bool,
    // void*                   TestEngine;                         // Test engine user data
    pub TestEngine: *mut c_void,
    // Windows state
    // ImVector<ImGuiWindow*>  Windows;                            // Windows, sorted in display order, back to front
    pub Windows: Vec<ImGuiWindow>,
    // ImVector<ImGuiWindow*>  WindowsFocusOrder;                  // Root windows, sorted in focus order, back to front.
    pub WindowsFocusOrder: Vec<ImGuiWindow>,
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
    // ImGuiDockNode*          HoveredDockNode;                    // [Debug] Hovered dock node.
    pub HoveredDockNode: *mut ImGuiDockNode,
    // ImGuiWindow*            MovingWindow;                       // Track the window we clicked on (in order to preserve focus). The actual window that is moved is generally MovingWindow->RootWindowDockTree.
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
    // ImGuiID                 ActiveIdIsAlive;                    // Active widget has been seen this frame (we can't use a bool as the ActiveId may change within the frame)
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
    pub ActiveIdHassBeenEditedBefore: bool,
    // bool                    ActiveIdHasBeenEditedThisFrame;
    pub ActiveIdHasBeenEditedThisFrame: bool,
    // ImVec2                  ActiveIdClickOffset;                // Clicked offset from upper-left corner, if applicable (currently only set by ButtonBehavior)
    pub ActiveIdClockOffset: ImVec2,
    // ImGuiWindow*            ActiveIdWindow;
    pub ActiveIdWindow: *mut ImGuiWindow,
    // ImGuiInputSource        ActiveIdSource;                     // Activating with mouse or nav (gamepad/keyboard)
    pub ActiveIdSource: ImGuiInputSource,
    // int                     ActiveIdMouseButton;
    pub ActiveIdMouseButton: i32,
    // ImGuiID                 ActiveIdPreviousFrame;
    pub ActiveIdPreviousFrame: ImGuiID,
    //bool                    ActiveIdPreviousFrameIsAlive;
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
    // bool                    ActiveIdUsingMouseWheel;            // Active widget will want to read mouse wheel. Blocks scrolling the underlying window.
    pub ActiveIdUsingMouseWheel: bool,
    // ImU32                   ActiveIdUsingNavDirMask;            // Active widget will want to read those nav move requests (e.g. can activate a button and move away from it)
    pub ActiveIdUsingNavDirMask: u32,
    // ImU32                   ActiveIdUsingNavInputMask;          // Active widget will want to read those nav inputs.
    pub ActiveIdUsingNavInputMask: u32,
    // ImBitArrayForNamedKeys  ActiveIdUsingKeyInputMask;          // Active widget will want to read those key inputs. When we grow the ImGuiKey enum we'll need to either to order the enum to make useful keys come first, either redesign this into e.g. a small array.
    pub ActiveIdUsingKeyInputMask: Vec<ImGuiKey>,
    // Next window/item data
    // ImGuiItemFlags          CurrentItemFlags;                      // == g.ItemFlagsStack.back()
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
    pub BeginMenuCount: i32,

    // Viewports
    // ImVector<ImGuiViewportP*> Viewports;                        // Active viewports (always 1+, and generally 1 unless multi-viewports are enabled). Each viewports hold their copy of ImDrawData.
    pub Viewports: Vec<*mut ImGuiViewport>,
    // float                   CurrentDpiScale;                    // == CurrentViewport->DpiScale
    pub CurrentDpiScale: f32,
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
    pub ViewportFrontMostStampCount: i32,
    // Gamepad/keyboard Navigation
    // ImGuiWindow*            NavWindow;                          // Focused window for navigation. Could be called 'FocusedWindow'
    pub NavWindow: *mut ImGuiWindow,
    // ImGuiID                 NavId;                              // Focused item for navigation
    pub NavId: ImGuiID,
    // ImGuiID                 NavFocusScopeId;                    // Identify a selection scope (selection code often wants to "clear other items" when landing on an item of the selection set)
    pub NavFocusScopeId: ImGuiID,
    // ImGuiID                 NavActivateId;                      // ~~ (g.ActiveId == 0) && IsNavInputPressed(ImGuiNavInput_Activate) ? NavId : 0, also set when calling ActivateItem()
    pub NavActivateId: ImGuiID,
    // ImGuiID                 NavActivateDownId;                  // ~~ IsNavInputDown(ImGuiNavInput_Activate) ? NavId : 0
    pub NavActivateDownId: ImGuiID,
    // ImGuiID                 NavActivatePressedId;               // ~~ IsNavInputPressed(ImGuiNavInput_Activate) ? NavId : 0
    pub NavActivatePressedId: ImGuiID,
    // ImGuiID                 NavActivateInputId;                 // ~~ IsNavInputPressed(ImGuiNavInput_Input) ? NavId : 0; ImGuiActivateFlags_PreferInput will be set and NavActivateId will be 0.
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
    pub NavDisableHighLight: bool,
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
    pub NavMOveDirForDebug: ImGuiDir,
    // ImGuiDir                NavMoveClipDir;                     // FIXME-NAV: Describe the purpose of this better. Might want to rename?
    pub NavMoveClipDir: ImGuiDir,
    // ImRect                  NavScoringRect;                     // Rectangle used for scoring, in screen space. Based of window->NavRectRel[], modified for directional navigation scoring.
    pub NavScoringRect: ImRect,
    // ImRect                  NavScoringNoClipRect;               // Some nav operations (such as PageUp/PageDown) enforce a region which clipper will attempt to always keep submitted
    pub NavScoringNoClipRect: ImRect,
    // int                     NavScoringDebugCount;               // Metrics for debugging
    pub NavScoringDebugCount: i32,
    // int                     NavTabbingDir;                      // Generally -1 or +1, 0 when tabbing without a nav id
    pub NavTabbingDir: i32,
    // int                     NavTabbingCounter;                  // >0 when counting items for tabbing
    pub NavTabbingCounter: i32,
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
    // ImGuiWindow*            NavWindowingTargetAnim;             // Record of last valid NavWindowingTarget until DimBgRatio and NavWindowingHighlightAlpha becomes 0.0, so the fade-out can stay on it.
    pub NavWindowingTargetAnim: *mut ImGuiWindow,
    // ImGuiWindow*            NavWindowingListWindow;             // Internal window actually listing the CTRL+Tab contents
    pub NavWindowingListWindow: *mut ImGuiWindow,
    // float                   NavWindowingTimer;
    pub NavWindowingTimer: f32,
    // float                   NavWindowingHighlightAlpha;
    pub NavWindowingHighlightAlpha: f32,
    // bool                    NavWindowingToggleLayer;
    pub NavWindowingToggleLayer: bool,
    // Render
    // float                   DimBgRatio;                         // 0.0..1.0 animation when fading in a dimming background (for modal window and CTRL+TAB list)
    pub DimBgRation: f32,
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
    pub DragDropSourceFrameCount: i32,
    // int                     DragDropMouseButton;
    pub DragDropMouseButton: i32,
    // ImGuiPayload            DragDropPayload;
    pub DragDropPayload: ImGuiPayload,
    // ImRect                  DragDropTargetRect;                 // Store rectangle of current target candidate (we favor small targets when overlapping)
    pub DragDropTargetRect: ImRect,
    // ImGuiID                 DragDropTargetId;
    pub DragDropTargetId: ImGuiID,
    // ImGuiDragDropFlags      DragDropAcceptFlags;
    pub DragDropAcceptFlags: ImGuiDragDropFlags,
    // float                   DragDropAcceptIdCurrRectSurface;    // Target item surface (we resolve overlapping targets by prioritizing the smaller surface)
    pub DragDropAcceptIdCurrRectSurface: f32,
    // ImGuiID                 DragDropAcceptIdCurr;               // Target item id (set at the time of accepting the payload)
    pub DragDropAcceptIdCurr: ImGuiID,
    // ImGuiID                 DragDropAcceptIdPrev;               // Target item id from previous frame (we need to store this to allow for overlapping drag and drop targets)
    pub DragDropAcceptIdPrev: ImGuiID,
    // int                     DragDropAcceptFrameCount;           // Last time a target expressed a desire to accept the source
    pub DragDropAcceptFrameCount: i32,
    // ImGuiID                 DragDropHoldJustPressedId;          // Set when holding a payload just made ButtonBehavior() return a press.
    pub DragDropHoldJustPressedId: ImGuiID,
    // ImVector<unsigned char> DragDropPayloadBufHeap;             // We don't expose the ImVector<> directly, ImGuiPayload only holds pointer+size
    pub DragDropPayloadBufHeap: Vec<u8>,
    // unsigned char           DragDropPayloadBufLocal[16];        // Local buffer for small payloads
    pub DragDropPayloadBufLocal: [u8; 16],
    // Clipper
    // int                             ClipperTempDataStacked;
    pub ClipperTempDataStacked: i32,
    // ImVector<ImGuiListClipperData>  ClipperTempData;
    pub ClipperTempData: Vec<ImGuiListClipperData>,
    // Tables
    // ImGuiTable*                     CurrentTable;
    pub CurrentTable: *mut ImGuiTable,
    // int                             TablesTempDataStacked;      // Temporary table data size (because we leave previous instances undestructed, we generally don't use TablesTempData.Size)
    pub TablesTempDataStacked: i32,
    // ImVector<ImGuiTableTempData>    TablesTempData;             // Temporary table data (buffers reused/shared across instances, support nesting)
    pub TablesTempData: Vec<ImGuiTableTempData>,
    // ImGuiPool<ImGuiTable>              Tables;                     // Persistent table data
    pub Tables: ImGuiPool<ImGuiTable>,
    // ImVector<float>                 TablesLastTimeActive;       // Last used timestamp of each tables (SOA, for efficient GC)
    pub TablesLastTimeActive: Vec<f32>,
    // ImVector<ImDrawChannel>         DrawChannelsTempMergeBuffer;

    // Tab bars
    // ImGuiTabBar*                    CurrentTabBar;
    pub CurrentTabBar: *mut ImGuiTabBar,
    // ImGuiPool<ImGuiTabBar>             TabBars;
    pub TabBars: ImGuiPool<ImGuiTabBar>,
    // ImVector<ImGuiPtrOrIndex>       CurrentTabBarStack;
    pub CurrentTabBarStack: Vec<ImGuiPtrOrIndex>,
    // ImVector<ImGuiShrinkWidthItem>  ShrinkWidthBuffer;
    pub ShrinkWidthBuffer: Vec<ImGuiShrinkWidthItem>,
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
    pub ColorEditLastHue: f32,
    // float                   ColorEditLastSat;                   // Backup of last Saturation associated to LastColor, so we can restore Saturation in lossy RGB<>HSV round trips
    pub ColorEditLastSat: f32,
    // ImU32                   ColorEditLastColor;                 // RGB value with alpha set to 0.
    pub ColorEditLastColor: u32,
    // ImVec4                  ColorPickerRef;                     // Initial/reference color at the time of opening the color picker.
    pub ColorPickerRef: ImVec4,
    // ImGuiComboPreviewData   ComboPreviewData;
    pub ComboPreviewData: ImGuiComboPreviewData,
    // float                   SliderGrabClickOffset;
    pub SliderGrabClickOffset: f32,
    // float                   SliderCurrentAccum;                 // Accumulated slider delta when using navigation controls.
    pub SliderCurrentAccum: f32,
    // bool                    SliderCurrentAccumDirty;            // Has the accumulated slider delta changed since last time we tried to apply it?
    pub SliderCurrentAccumDirty: bool,
    // bool                    DragCurrentAccumDirty;
    pub DragCurrentAccumDirty: bool,
    // float                   DragCurrentAccum;                   // Accumulator for dragging modification. Always high-precision, not rounded by end-user precision settings
    pub DragCurrentAccum: f32,
    // float                   DragSpeedDefaultRatio;              // If speed == 0.0, uses (max-min) * DragSpeedDefaultRatio
    pub DragSpeedDefaultRatio: f32,
    // float                   ScrollbarClickDeltaToGrabCenter;    // Distance between mouse and center of grab box, normalized in parent space. Use storage?
    pub ScrollbarClickDeltaToGrabCenter: f32,
    // float                   DisabledAlphaBackup;                // Backup for style.Alpha for BeginDisabled()
    pub DisabledAlphaBackup: f32,
    // short                   DisabledStackSize;
    pub DisabledStackSize: i16,
    // short                   TooltipOverrideCount;
    pub TooltipOverrideCount: i16,
    // float                   TooltipSlowDelay;                   // Time before slow tooltips appears (FIXME: This is temporary until we merge in tooltip timer+priority work)
    pub TooltipSlowDelay: f32,
    // ImVector<char>          ClipboardHandlerData;               // If no custom clipboard handler is defined
    pub ClipboardHandlerData: Vec<char>,
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
    pub PlatformLocalDecimalPoint: char,
    // Extensions
    // FIXME: We could provide an API to register one slot in an array held in ImGuiContext?
    // ImGuiDockContext        DockContext;
    pub DockContext: ImGuiDockContext,
    // Settings
    // bool                    SettingsLoaded;
    pub SettingsLoaded: bool,
    // float                   SettingsDirtyTimer;                 // Save .ini Settings to memory when time reaches zero
    pub SettingsDirtyTimer: f32,
    // ImGuiTextBuffer         SettingsIniData;                    // In memory .ini settings
    pub SettingsIniData: ImGuiTextBuffer,
    // ImVector<ImGuiSettingsHandler>      SettingsHandlers;       // List of .ini settings handlers
    pub SettingsHandlers: Vec<ImGuiSettingsHandler>,
    // ImChunkStream<ImGuiWindowSettings>  SettingsWindows;        // ImGuiWindow .ini settings entries
    pub SettingsWindows: ImChunkStream<ImGuiWindowSettings>,
    // ImChunkStream<ImGuiTableSettings>   SettingsTables;         // ImGuiTable .ini settings entries
    pub SettingsTabls: ImChunkStream<ImGuiTableSettings>,
    // ImVector<ImGuiContextHook>          Hooks;                  // Hooks for extensions (e.g. test engine)
    pub Hooks: Vec<ImGuiContextHook>,
    // ImGuiID                             HookIdNext;             // Next available HookId
    pub HookIdNext: ImGuiID,
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
    pub LogLinePosY: f32,
    // bool                    LogLineFirstItem;
    pub LogLineFirstLine: bool,
    // int                     LogDepthRef;
    pub LogDepthRef: i32,
    // int                     LogDepthToExpand;
    pub LogDepthToExpand: i32,
    // int                     LogDepthToExpandDefault;            // Default/stored value for LogDepthMaxExpand if not specified in the LogXXX function call.
    pub LogDepthToExpandDefault: i32,
    // Debug Tools
    // ImGuiDebugLogFlags      DebugLogFlags;
    pub DebugLogFlags: ImGuiDebugLogFlags,
    // ImGuiTextBuffer         DebugLogBuf;
    pub DebugLogBuf: ImGuiTextBuffer,
    // bool                    DebugItemPickerActive;              // Item picker is active (started with DebugStartItemPicker())
    pub DebugItemPickerActive: bool,
    // ImGuiID                 DebugItemPickerBreakId;             // Will call IM_DEBUG_BREAK() when encountering this ID
    pub DebugItemPickerBreakId: ImGuiID,
    // ImGuiMetricsConfig      DebugMetricsConfig;
    pub DebugMetricsConfig: ImGuiMetricsConfig,
    // ImGuiStackTool          DebugStackTool;
    pub DebugStackTool: ImGuiStackTool,

    // Misc
    // float                   FramerateSecPerFrame[120];          // Calculate estimate of framerate for user over the last 2 seconds.
    pub FramerateSecPerFrame: [f32; 128],
    // int                     FramerateSecPerFrameIdx;
    pub FramerateSecPerFrameIdx: i32,
    // int                     FramerateSecPerFrameCount;
    pub FramerateSecPerFrameCount: i32,
    // float                   FramerateSecPerFrameAccum;
    pub FramerateSecPerFrameAccum: f32,
    // int                     WantCaptureMouseNextFrame;          // Explicit capture override via SetNextFrameWantCaptureMouse()/SetNextFrameWantCaptureKeyboard(). Default to -1.
    pub WantCaptureMouseNextFrame: i32,
    // int                     WantCaptureKeyboardNextFrame;       // "
    pub WantCaptureKeyboardNextFrame: i32,
    // int                     WantTextInputNextFrame;
    pub WantInputNextFrame: i32,
    // ImVector<char>          TempBuffer;                         // Temporary text buffer
    pub TempBuffer: Vec<u8>,

}

impl ImGuiContext {
    // ImGuiContext(ImFontAtlas* shared_font_atlas)
    pub fn new(shared_font_alias: *mut ImFontAtlas) -> Self
    {
        Self {
            Initialized: false,
            ConfigFlagsCurrFrame: ImGuiConfigFlags::ImGuiConfigFlags_None,
            ConfigFlagsLastFrame: ImGuiConfigFlags::ImGuiConfigFlags_None,
            FontAtlasOwnedByContext: if shared_font_atlas.is_null() { false } else { true }, Font: null_mut(),
            FontSize: 0.0,
            FontBaseSize: 0.0,
            IO: ImGuiIO::new(),
            PlatformIO: ImGuiPlatformIO::new(),
            InputEventsQueue: vec![],
            InputEventsTrail: vec![],
            Time: 0.0,
            FrameCount: 0,
            FrameCountEnded: -1,
            FrameCountPlatformEnded: -1,
            FrameCountRendered: -1,
            WithinFrameScope: false,
            WithinFrameScopeWithImplicitWindow: false,
            WithinEndChild: false,
            GcCompactAll: false,
            // TestEngineHookItems: false,
            TestEngine: null_mut(),
            Windows: vec![],
            WindowsFocusOrder: vec![],
            WindowsTempSortBuffer: vec![],
            CurrentWindowStack: vec![],
            WindowsById: Default::default(),
            WindowsActiveCount: 0,
            WindowsHoverPadding: Default::default(),
            CurrentWindow: NULL,
            HoveredWindow: NULL,
            HoveredWindowUnderMovingWindow: NULL,
            HoveredDockNode: NULL,
            MovingWindow: NULL,
            WheelingWindow: NULL,
            WheelingWindowRefMousePos: Default::default(),
            WheelingWindowTimer: 0.0,

            DebugHookIdInfo: 0,
            HoveredId: HoveredIdPreviousFrame = 0,
            HoveredIdPreviousFrame: (),
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
            // ActiveIdHasBeenEditedBefore: false,
            ActiveIdHasBeenEditedThisFrame: false,
            // ActiveIdClickOffset: ImVec2::new( - 1, -1),
            ActiveIdWindow: null_mut(),
            ActiveIdSource: ImGuiInputSource::ImGuiInputSource_None,
            ActiveIdMouseButton: - 1,
            ActiveIdPreviousFrame: 0,
            ActiveIdPreviousFrameIsAlive: false,
            ActiveIdPreviousFrameHasBeenEditedBefore: false,
            ActiveIdPreviousFrameWindow: null_mut(),
            LastActiveId: 0,
            LastActiveIdTimer: 0.0,
            ActiveIdUsingMouseWheel: false,
            ActiveIdUsingNavDirMask: 0x00,
            ActiveIdUsingNavInputMask: 0x00,
            // ActiveIdUsingKeyInputMask.ClearAllBits(),
            CurrentItemFlags: ImGuiItemFlags_None,
            NextItemData: (),
            LastItemData: (),
            NextWindowData: (),
            ColorStack: vec![],
            StyleVarStack: vec![],
            FontStack: vec![],
            FocusScopeStack: vec![],
            ItemFlagsStack: vec![],
            GroupStack: vec![],
            OpenPopupStack: vec![],
            BeginPopupStack: vec![],
            BeginMenuCount: 0,

            Viewports: vec![],
            CurrentDpiScale: 0.0,
            CurrentViewport: NULL,
            MouseViewport: null_mut(),
            // MouseLastHoveredViewport: NULL,
            MouseLastHoveredViewport: null_mut(),
            PlatformLastFocusedViewportId: 0,
            FallbackMonitor: (),
            ViewportFrontMostStampCount: 0,
            NavWindow: null_mut(),
            NavId: 0,
            NavFocusScopeId: (),
            NavActivateId: (),
            NavActivateDownId: (),
            NavActivatePressedId: (),
            NavJustMovedToId: 0,
            NavActivateFlags: ImGuiActivateFlags::ImGuiActivateFlags_None,
            NavJustMovedToKeyMods: ImGuiModeFlags::ImGuiModFlags_None,
            NavNextActivateId: (),
            NavNextActivateFlags: (),
            NavInputSource: ImGuiInputSource::ImGuiInputSource_None,
            NavLayer: ImGuiNavLayer::ImGuiNavLayer_Main,
            NavIdIsAlive: false,
            NavMousePosDirty: false,
            // NavDisableHighlight: true,
            NavDisableMouseHover: false,
            NavAnyRequest: false,
            NavInitRequest: false,
            NavInitRequestFromMove: false,
            NavInitResultId: 0,
            NavMoveSubmitted: false,
            NavMoveScoringItems: false,
            NavMoveForwardToNextFrame: false,
            NavMoveFlags: ImGuiNavMoveFlags::ImGuiNavMoveFlags_None,
            NavMoveScrollFlags: ImGuiScrollFlags::ImGuiScrollFlags_None,
            NavMoveKeyMods: ImGuiModFlags::None,
            NavMoveDir: NavMoveDirForDebug: NavMoveClipDir: ImGuiDir_None,
            NavMOveDirForDebug: ImGuiDir::None,
            NavMoveClipDir: ImGuiDir::None,
            NavScoringRect: (),
            NavScoringNoClipRect: (),
            NavScoringDebugCount: 0,
            NavTabbingDir: 0,
            NavTabbingCounter: 0,

            NavMoveResultLocal: (),
            NavMoveResultLocalVisible: (),
            NavMoveResultOther: (),
            NavTabbingResultFirst: (),
            NavWindowingTarget: null_mut(),
            NavWindowingTargetAnim: null_mut(),
            NavWindowingListWindow: null_mut(),
            // NavWindowingTargetAnim: (),
            // NavWindowingListWindow: (),
            NavWindowingTimer: 0.0,
            NavWindowingHighlightAlpha: 0.0,
            NavWindowingToggleLayer: false,
            // DimBgRatio: 0.0,
            MouseCursor: ImGuiMouseCursor::Arrow,
            DragDropActive: false,
            DragDropWithinSource: false,
            DragDropWithinTarget: false,
            // DragDropWithinSource: false,
            // DragDropWithinTarget: false,
            DragDropSourceFlags: ImGuiDragDropFlags::None,
            DragDropSourceFrameCount: - 1,
            DragDropMouseButton: - 1,
            DragDropPayload: Default::default(),
            DragDropTargetRect: (),
            DragDropTargetId: 0,
            DragDropAcceptFlags: ImGuiDragDropFlags::None,
            DragDropAcceptIdCurrRectSurface: 0.0,
            DragDropAcceptIdCurr: (),
            DragDropAcceptIdPrev: 0,
            // DragDropAcceptIdCurr: 0,
            DragDropAcceptFrameCount: - 1,
            DragDropHoldJustPressedId: 0,
            DragDropPayloadBufLocal: [0;16],

            ClipperTempDataStacked: 0,

            ClipperTempData: vec![],
            CurrentTable: null_mut(),
            TablesTempDataStacked: 0,
            TablesTempData: vec![],
            Tables: (),
            TablesLastTimeActive: vec![],
            CurrentTabBar: null_mut(),

            TabBars: (),
            CurrentTabBarStack: vec![],
            ShrinkWidthBuffer: vec![],
            MouseLastValidPos: Default::default(),
            InputTextState: (),
            InputTextPasswordFont: Default::default(),
            TempInputId: 0,
            ColorEditOptions: ImGuiColorEditFlags::DefaultOptions,
            ColorEditLastHue: 0.0,
            ColorEditLastSat: 0.0,
            ColorEditLastColor: 0,
            ColorPickerRef: Default::default(),
            ComboPreviewData: (),
            SliderGrabClickOffset: 0.0,
            SliderCurrentAccum: 0.0,
            SliderCurrentAccumDirty: false, DragCurrentAccumDirty: false, DragCurrentAccum: 0.0,
            DragSpeedDefaultRatio: 1.0 / 100.0,
            DisabledAlphaBackup: 0.0,
            DisabledStackSize: 0,
            ScrollbarClickDeltaToGrabCenter: 0.0,
            TooltipOverrideCount: 0,
            TooltipSlowDelay: 0.50,

            ClipboardHandlerData: vec![],
            MenusIdSubmittedThisFrame: vec![],
            PlatformImeData: ImGuiPlatformImeData::new(ImVec2::new(0.0,0.0)),
            // PlatformImeDataPrev.InputPos: ImVec2( - 1.0,
            // -1.0), // Different to ensure initial submission
            PlatformImeDataPrev: ImGuiPlatformImeData::new(ImVec2::new(-1.0,-1.0)),
            PlatformImeViewport: 0,
            PlatformLocalDecimalPoint: '.',
            // PlatformLocaleDecimalPoint: '.',
            SettingsLoaded: false,
            SettingsDirtyTimer: 0.0,
            SettingsIniData: Default::default(),
            SettingsHandlers: vec![],
            SettingsWindows: (),
            SettingsTabls: (),
            Hooks: vec![],
            HookIdNext: 0,
            LogEnabled: false,
            LogType: ImGuiLogType::None,
            LogNextPrefix: null_mut(),
            LogNextSuffix: null_mut(),
            LogFile: null_mut(),
            LogLinePosY: f32::MAX,
            // LogLineFirstItem: false,
            LogDepthRef: 0,
            LogDepthToExpand: 2,
            LogDepthToExpandDefault: 2,
            DebugLogFlags: ImGuiDebugLogFlags::OutputToTTY,
            DebugLogBuf: Default::default(),
            DebugItemPickerActive: false, DebugItemPickerBreakId: 0,

            DebugMetricsConfig: (),
            FramerateSecPerFrame: [0.0;128],
            FramerateSecPerFrameIdx: 0,
            FramerateSecPerFrameCount: 0,
            FramerateSecPerFrameAccum: 0.0,
            WantCaptureMouseNextFrame: -1,
            WantCaptureKeyboardNextFrame: -1,
            // WantTextInputNextFrame: - 1,
            // WantCaptureKeyboardNextFrame: 0,
            WantInputNextFrame: 0,
            Style: ImGuiStyle::new(),
            DrawListSharedData: (),
            TestENgineHookItems: false,
            ActiveIdHassBeenEditedBefore: false,
            ActiveIdClockOffset: Default::default(),
            NavActivateInputId: (),
            NavJustMovedToFocusScopeId: (),
            NavDisableHighLight: false,
            NavInitResultRectRel: (),
            // NavWindowingHighlightAlpha: 0.0,
            DimBgRation: 0.0,
            DragDropPayloadBufHeap: vec![],
            // ColorEditLastSat: 0.0,
            DockContext: (),
            LogBuffer: Default::default(),
            // LogNextSuffix: (),
            LogLineFirstLine: false,
            // LogDepthToExpandDefault: 0,
            DebugStackTool: (),
            // FramerateSecPerFrameCount: 0,
            TempBuffer: vec![]
        }
    }
}
