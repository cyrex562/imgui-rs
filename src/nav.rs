use std::ptr::{null, null_mut};
use std::collections::HashSet;
use crate::config::ConfigFlags;
use crate::Context;
use crate::types::Direction;
use crate::draw::draw_list::get_foreground_draw_list;
use crate::imgui_h::ImGuiID;
use crate::imgui_rect::Rect;
use crate::imgui_window::{ImGuiItemFlags, ImGuiWindow};
use crate::input::{InputSource, NavInput, NavLayer};
use crate::item::ItemFlags;
use crate::orig_imgui_single_file::{ImGuiViewport, ImGuiWindow};
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::{Window, WindowFlags};

pub enum ActivateFlags
{
    None                 = 0,
    PreferInput         ,       // Favor activation that requires keyboard text input (e.g. for Slider/Drag). Default if keyboard is available.
    PreferTweak         ,       // Favor activation for tweaking with arrows or gamepad (e.g. for Slider/Drag). Default if keyboard is not available.
    TryToPreserveState   = 1 << 2        // Request widget to preserve state if it can (e.g. InputText will try to preserve cursor/selection)
}


// Early work-in-progress API for ScrollToItem()
pub enum ScrollFlags
{
    None                   = 0,
    KeepVisibleEdgeX      ,       // If item is not visible: scroll as little as possible on x axis to bring item back into view [default for x axis]
    KeepVisibleEdgeY      ,       // If item is not visible: scroll as little as possible on Y axis to bring item back into view [default for Y axis for windows that are already visible]
    KeepVisibleCenterX    ,       // If item is not visible: scroll to make the item centered on x axis [rarely used]
    KeepVisibleCenterY    ,       // If item is not visible: scroll to make the item centered on Y axis
    AlwaysCenterX         ,       // Always center the result item on x axis [rarely used]
    AlwaysCenterY         ,       // Always center the result item on Y axis [default for Y axis for appearing window)
    NoScrollParent        ,       // Disable forwarding scrolling to parent window if required to keep item/rect visible (only scroll window the function was applied to).
    
}

pub const ImGuiScrollFlags_MaskX: ScrollFlags = ScrollFlags::KeepVisibleEdgeX | ScrollFlags::KeepVisibleCenterX | ScrollFlags::AlwaysCenterX;
pub const ImGuiScrollFlags_MaskY: ScrollFlags = ScrollFlags::KeepVisibleEdgeY | ScrollFlags::KeepVisibleCenterY | ScrollFlags::AlwaysCenterY;

pub enum NavHighlightFlags
{
    INone             = 0,
    ITypeDefault     ,
    ITypeThin        ,
    IAlwaysDraw      ,       // Draw rectangular highlight if (g.nav_id == id) _even_ when using the mouse.
    INoRounding       = 1 << 3
}

pub enum NavDirSourceFlags
{
    None             = 0,
    RawKeyboard     ,   // Raw keyboard (not pulled from nav), facilitate use of some functions before we can unify nav and keys
    Keyboard        ,
    PadDPad         ,
    PadLStick        = 1 << 3
}

pub enum NavMoveFlags
{
    None                  = 0,
    LoopX                ,   // On failed request, restart from opposite side
    LoopY                ,
    WrapX                ,   // On failed request, request from opposite side one line down (when NavDir==right) or one line up (when NavDir==left)
    WrapY                ,   // This is not super useful but provided for completeness
    AllowCurrentNavId    ,   // Allow scoring and considering the current nav_id as a move target candidate. This is used when the move source is offset (e.g. pressing PageDown actually needs to send a Up move request, if we are pressing PageDown from the bottom-most item we need to stay in place)
    AlsoScoreVisibleSet  ,   // Store alternate result in nav_move_result_local_visible that only comprise elements that are already fully visible (used by PageUp/PageDown)
    ScrollToEdgeY        ,   // Force scrolling to min/max (used by Home/End) // FIXME-NAV: Aim to remove or reword, probably unnecessary
    Forwarded            ,
    DebugNoResult        ,   // Dummy scoring for debug purpose, don't apply result
    FocusApi             ,
    Tabbing              ,  // == Focus + Activate if item is Inputable + DontChangeNavHighlight
    Activate             ,
    DontSetNavHighlight   = 1 << 12   // Do not alter the visible state of keyboard vs mouse nav highlight
}

#[derive(Default,Debug,Clone)]
pub struct NavItemData
{
    // ImGuiWindow*        window;         // Init,Move    // Best candidate window (result->ItemWindow->root_window_for_nav == request->window)
    pub Window: *mut ImGuiWindow,
    // ImGuiID             id;             // Init,Move    // Best candidate item id
    pub ID: ImGuiID,
    // ImGuiID             focus_scope_id;   // Init,Move    // Best candidate focus scope id
    pub FocusScopeId: ImGuiID,
    // ImRect              RectRel;        // Init,Move    // Best candidate bounding box in window relative space
    pub RectRel: Rect,
    // ImGuiItemFlags      in_flags;        // ????,Move    // Best candidate item flags
    pub InFlags: ImGuiItemFlags,
    // float               DistBox;        //      Move    // Best candidate box distance to current nav_id
    pub DistBox: f32,
    // float               DistCenter;     //      Move    // Best candidate center distance to current nav_id
    pub DistCenter: f32,
    // float               DistAxial;      //      Move    // Best candidate axial distance to current nav_id
    pub DistAxial: f32,
}

impl NavItemData {
    // ImGuiNavItemData()  { clear(); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     void clear()        { window = None; id = focus_scope_id = 0; in_flags = 0; DistBox = DistCenter = DistAxial = FLT_MAX; }
    pub fn Clear(&mut self) {
        self.Window = null_mut();
        self.id = INVALID_ID;
        self.FocusScopeId = 0;
        self.InFlags = ImGuiItemFlags::None;
        self.DistBox = f32::MAX;
        self.DistCenter = f32::MAX;
        self.DistAxial = f32::MAX;
    }
}


// FIXME-NAV: Clarify/expose various repeat delay/rate
pub enum NavReadMode
{
    Down,
    Pressed,
    Released,
    Repeat,
    RepeatSlow,
    RepeatFast
}

pub const NAV_RESIZE_SPEED: f32 = 600.0;

// void SetNavWindow(ImGuiWindow* window)
pub fn set_nav_window(g: &mut Context, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;
    if (g.nav_window != window)
    {
        IMGUI_DEBUG_LOG_FOCUS("[focus] SetNavWindow(\"%s\")\n", window ? window.name : "<None>");
        g.nav_window = window;
    }
    g.NavInitRequest = g.NavMoveSubmitted = g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

// void SetNavID(ImGuiID id, ImGuiNavLayer nav_layer, ImGuiID focus_scope_id, const Rect& rect_rel)
pub fn set_nav_id(g: &mut Context, id: Id32, nav_layer: NavLayer, focus_scope_id: Id32, rect_rel: &Rect)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.nav_window != None);
    // IM_ASSERT(nav_layer == NavLayer::Main || nav_layer == NavLayer::Menu);
    g.nav_id = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = focus_scope_id;
    g.nav_window.nav_last_ids[nav_layer] = id;
    g.nav_window.NavRectRel[nav_layer] = rect_rel;
}

// void SetFocusID(ImGuiID id, ImGuiWindow* window)
pub fn set_focus_id(g: &mut Context, id: Id32, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(id != 0);

    if (g.nav_window != window)
       SetNavWindow(window);

    // Assume that SetFocusID() is called in the context where its window->dc.NavLayerCurrent and window->dc.nav_focus_scope_id_current are valid.
    // Note that window may be != g.current_window (e.g. SetFocusID call in InputTextEx for multi-line text)
    const ImGuiNavLayer nav_layer = window.dcnav_layer_current;
    g.nav_id = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = window.dc.NavFocusScopeIdCurrent;
    window.nav_last_ids[nav_layer] = id;
    if (g.last_item_data.id == id)
        window.NavRectRel[nav_layer] = window_rect_abs_to_rel(window, g.last_item_data.NavRect);

    if (g.active_id_source == InputSource::Nav)
        g.nav_disable_mouse_hover = true;
    else
        g.nav_disable_highlight = true;
}

// ImGuiDir ImGetDirQuadrantFromDelta(float dx, float dy)
pub fn im_get_dir_quadrant_from_delta(g: &mut Context, dx: f32, dy: f32) -> Direction
{
    if (f32::abs(dx) > f32::abs(dy))
        return (dx > 0.0) ? Direction::Right : Direction::Left;
    return (dy > 0.0) ? Direction::Down : Direction::Up;
}

// static float inline NavScoreItemDistInterval(float a0, float a1, float b0, float b1)
pub fn nav_score_item_dist_interval(g: &mut Context, a0: f32, a1: f32, b0: f32, b1: f32) -> f32
{
    if (a1 < b0)
        return a1 - b0;
    if (b1 < a0)
        return a0 - b1;
    return 0.0;
}

// static void inline NavClampRectToVisibleAreaForMoveDir(ImGuiDir move_dir, Rect& r, const Rect& clip_rect)
pub fn nav_clamp_rect_to_visible_area_for_move_dir(g: &mut Context, move_dir: Direction, r: &Rect, clip_rect: &Rect)
{
    if (move_dir == Direction::Left || move_dir == Direction::Right)
    {
        r.min.y = ImClamp(r.min.y, clip_rect.min.y, clip_rect.max.y);
        r.max.y = ImClamp(r.max.y, clip_rect.min.y, clip_rect.max.y);
    }
    else // FIXME: PageUp/PageDown are leaving move_dir == None
    {
        r.min.x = ImClamp(r.min.x, clip_rect.min.x, clip_rect.max.x);
        r.max.x = ImClamp(r.max.x, clip_rect.min.x, clip_rect.max.x);
    }
}

// Scoring function for gamepad/keyboard directional navigation. Based on https://gist.github.com/rygorous/6981057
// static bool NavScoreItem(ImGuiNavItemData* result)
pub fn nav_score_item(g: &mut Context, result: &mut NavItemData) -> bool
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (g.NavLayer != window.dcnav_layer_current)
        return false;

    // FIXME: Those are not good variables names
    Rect cand = g.last_item_data.NavRect;   // Current item nav rectangle
    const Rect curr = g.NavScoringRect;   // Current modified source rect (NB: we've applied max.x = min.x in NavUpdate() to inhibit the effect of having varied item width)
    g.NavScoringDebugCount += 1;

    // When entering through a NavFlattened border, we consider child window items as fully clipped for scoring
    if (window.parent_window == g.nav_window)
    {
        // IM_ASSERT((window.flags | g.nav_window.flags) & WindowFlags::NavFlattened);
        if (!window.clip_rect.Overlaps(cand))
            return false;
        cand.ClipWithFull(window.clip_rect); // This allows the scored item to not overlap other candidates in the parent window
    }

    // We perform scoring on items bounding box clipped by the current clipping rectangle on the other axis (clipping on our movement axis would give us equal scores for all clipped items)
    // For example, this ensure that items in one column are not reached when moving vertically from items in another column.
    NavClampRectToVisibleAreaForMoveDir(g.NavMoveClipDir, cand, window.clip_rect);

    // Compute distance between boxes
    // FIXME-NAV: Introducing biases for vertical navigation, needs to be removed.
    float dbx = NavScoreItemDistInterval(cand.min.x, cand.max.x, curr.min.x, curr.max.x);
    float dby = NavScoreItemDistInterval(ImLerp(cand.min.y, cand.max.y, 0.2), ImLerp(cand.min.y, cand.max.y, 0.8), ImLerp(curr.min.y, curr.max.y, 0.2), ImLerp(curr.min.y, curr.max.y, 0.8)); // scale down on Y to keep using box-distance for vertically touching items
    if (dby != 0.0 && dbx != 0.0)
        dbx = (dbx / 1000.0) + ((dbx > 0.0) ? +1.0 : -1.0);
    float dist_box = f32::abs(dbx) + f32::abs(dby);

    // Compute distance between centers (this is off by a factor of 2, but we only compare center distances with each other so it doesn't matter)
    float dcx = (cand.min.x + cand.max.x) - (curr.min.x + curr.max.x);
    float dcy = (cand.min.y + cand.max.y) - (curr.min.y + curr.max.y);
    float dist_center = f32::abs(dcx) + f32::abs(dcy); // L1 metric (need this for our connectedness guarantee)

    // Determine which quadrant of 'curr' our candidate item 'cand' lies in based on distance
    ImGuiDir quadrant;
    float dax = 0.0, day = 0.0, dist_axial = 0.0;
    if (dbx != 0.0 || dby != 0.0)
    {
        // For non-overlapping boxes, use distance between boxes
        dax = dbx;
        day = dby;
        dist_axial = dist_box;
        quadrant = ImGetDirQuadrantFromDelta(dbx, dby);
    }
    else if (dcx != 0.0 || dcy != 0.0)
    {
        // For overlapping boxes with different centers, use distance between centers
        dax = dcx;
        day = dcy;
        dist_axial = dist_center;
        quadrant = ImGetDirQuadrantFromDelta(dcx, dcy);
    }
    else
    {
        // Degenerate case: two overlapping buttons with same center, break ties arbitrarily (note that LastItemId here is really the _previous_ item order, but it doesn't matter)
        quadrant = (g.last_item_data.id < g.nav_id) ? Direction::Left : Direction::Right;
    }

// #ifIMGUI_DEBUG_NAV_SCORING
    char buf[128];
    if (is_mouse_hovering_rect(cand.min, cand.max))
    {
        ImFormatString(buf, IM_ARRAYSIZE(buf), "dbox (%.2,%.2->%.4)\ndcen (%.2,%.2->%.4)\nd (%.2,%.2->%.4)\nnav %c, quadrant %c", dbx, dby, dist_box, dcx, dcy, dist_center, dax, day, dist_axial, "WENS"[g.NavMoveDir], "WENS"[quadrant]);
        ImDrawList* draw_list = get_foreground_draw_list(window);
        draw_list.add_rect(curr.min, curr.max, IM_COL32(255,200,0,100));
        draw_list.add_rect(cand.min, cand.max, IM_COL32(255,255,0,200));
        draw_list.add_rect_filled(cand.max - Vector2D::new(4, 4), cand.max + CalcTextSize(buf) + Vector2D::new(4, 4), IM_COL32(40,0,0,150));
        draw_list.AddText(cand.max, ~0U, buf);
    }
    else if (g.io.key_ctrl) // Hold to preview score in matching quadrant. Press C to rotate.
    {
        if (quadrant == g.NavMoveDir)
        {
            ImFormatString(buf, IM_ARRAYSIZE(buf), "%.0/%.0", dist_box, dist_center);
            ImDrawList* draw_list = get_foreground_draw_list(window);
            draw_list.add_rect_filled(cand.min, cand.max, IM_COL32(255, 0, 0, 200));
            draw_list.AddText(cand.min, IM_COL32(255, 255, 255, 255), buf);
        }
    }


    // Is it in the quadrant we're interesting in moving to?
    bool new_best = false;
    const ImGuiDir move_dir = g.NavMoveDir;
    if (quadrant == move_dir)
    {
        // Does it beat the current best candidate?
        if (dist_box < result.DistBox)
        {
            result.DistBox = dist_box;
            result.DistCenter = dist_center;
            return true;
        }
        if (dist_box == result.DistBox)
        {
            // Try using distance between center points to break ties
            if (dist_center < result.DistCenter)
            {
                result.DistCenter = dist_center;
                new_best = true;
            }
            else if (dist_center == result.DistCenter)
            {
                // Still tied! we need to be extra-careful to make sure everything gets linked properly. We consistently break ties by symbolically moving "later" items
                // (with higher index) to the right/downwards by an infinitesimal amount since we the current "best" button already (so it must have a lower index),
                // this is fairly easy. This rule ensures that all buttons with dx==dy==0 will end up being linked in order of appearance along the x axis.
                if (((move_dir == Direction::Up || move_dir == Direction::Down) ? dby : dbx) < 0.0) // moving bj to the right/down decreases distance
                    new_best = true;
            }
        }
    }

    // Axial check: if 'curr' has no link at all in some direction and 'cand' lies roughly in that direction, add a tentative link. This will only be kept if no "real" matches
    // are found, so it only augments the graph produced by the above method using extra links. (important, since it doesn't guarantee strong connectedness)
    // This is just to avoid buttons having no links in a particular direction when there's a suitable neighbor. you get good graphs without this too.
    // 2017/09/29: FIXME: This now currently only enabled inside menu bars, ideally we'd disable it everywhere. Menus in particular need to catch failure. For general navigation it feels awkward.
    // Disabling it may lead to disconnected graphs when nodes are very spaced out on different axis. Perhaps consider offering this as an option?
    if (result.DistBox == f32::MAX && dist_axial < result.DistAxial)  // Check axial match
        if (g.NavLayer == NavLayer::Menu && !(g.nav_window.flags & WindowFlags::ChildMenu))
            if ((move_dir == Direction::Left && dax < 0.0) || (move_dir == Direction::Right && dax > 0.0) || (move_dir == Direction::Up && day < 0.0) || (move_dir == Direction::Down && day > 0.0))
            {
                result.DistAxial = dist_axial;
                new_best = true;
            }

    return new_best;
}

// static void NavApplyItemToResult(ImGuiNavItemData* result)
pub fn nav_apply_item_to_result(g: &mut Context, result: &mut NavItemData)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    result.Window = window;
    result.id = g.last_item_data.id;
    result.FocusScopeId = window.dc.NavFocusScopeIdCurrent;
    result.InFlags = g.last_item_data.InFlags;
    result.RectRel = window_rect_abs_to_rel(window, g.last_item_data.NavRect);
}

// We get there when either nav_id == id, or when g.nav_any_request is set (which is updated by NavUpdateAnyRequestFlag above)
// This is called after last_item_data is set.
// static void NavProcessItem()
pub fn nav_process_item(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    const ImGuiID id = g.last_item_data.id;
    const Rect nav_bb = g.last_item_data.NavRect;
    const ImGuiItemFlags item_flags = g.last_item_data.InFlags;

    // Process Init Request
    if (g.NavInitRequest && g.NavLayer == window.dcnav_layer_current && (item_flags & ItemFlags::Disabled) == 0)
    {
        // Even if 'ImGuiItemFlags_NoNavDefaultFocus' is on (typically collapse/close button) we record the first ResultId so they can be used as a fallback
        const bool candidate_for_nav_default_focus = (item_flags & ItemFlags::NoNavDefaultFocus) == 0;
        if (candidate_for_nav_default_focus || g.NavInitResultId == 0)
        {
            g.NavInitResultId = id;
            g.NavInitResultRectRel = window_rect_abs_to_rel(window, nav_bb);
        }
        if (candidate_for_nav_default_focus)
        {
            g.NavInitRequest = false; // Found a match, clear request
            NavUpdateAnyRequestFlag();
        }
    }

    // Process Move Request (scoring for navigation)
    // FIXME-NAV: Consider policy for double scoring (scoring from nav_scoring_rect + scoring from a rect wrapped according to current wrapping policy)
    if (g.NavMoveScoringItems)
    {
        const bool is_tab_stop = (item_flags & ItemFlags::Inputable) && (item_flags & (ItemFlags::NoTabStop | ItemFlags::Disabled)) == 0;
        const bool is_tabbing = (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) != 0;
        if (is_tabbing)
        {
            if (is_tab_stop || (g.NavMoveFlags & ImGuiNavMoveFlags_FocusApi))
                NavProcessItemForTabbingRequest(id);
        }
        else if ((g.nav_id != id || (g.NavMoveFlags & ImGuiNavMoveFlags_AllowCurrentNavId)) && !(item_flags & (ItemFlags::Disabled | ItemFlags::NoNav)))
        {
            ImGuiNavItemData* result = (window == g.nav_window) ? &g.NavMoveResultLocal : &g.NavMoveResultOther;
            if (!is_tabbing)
            {
                if (NavScoreItem(result))
                    NavApplyItemToResult(result);

                // Features like PageUp/PageDown need to maintain a separate score for the visible set of items.
                let VISIBLE_RATIO = 0.70;
                if ((g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet) && window.clip_rect.Overlaps(nav_bb))
                    if (ImClamp(nav_bb.max.y, window.clip_rect.min.y, window.clip_rect.max.y) - ImClamp(nav_bb.min.y, window.clip_rect.min.y, window.clip_rect.max.y) >= (nav_bb.max.y - nav_bb.min.y) * VISIBLE_RATIO)
                        if (NavScoreItem(&g.NavMoveResultLocalVisible))
                            NavApplyItemToResult(&g.NavMoveResultLocalVisible);
            }
        }
    }

    // Update window-relative bounding box of navigated item
    if (g.nav_id == id)
    {
        if (g.nav_window != window)
            SetNavWindow(window); // Always refresh g.nav_window, because some operations such as FocusItem() may not have a window.
        g.NavLayer = window.dcnav_layer_current;
        g.NavFocusScopeId = window.dc.NavFocusScopeIdCurrent;
        g.NavIdIsAlive = true;
        window.NavRectRel[window.dcnav_layer_current] = window_rect_abs_to_rel(window, nav_bb);    // Store item bounding box (relative to window position)
    }
}

// Handle "scoring" of an item for a tabbing/focusing request initiated by NavUpdateCreateTabbingRequest().
// Note that SetKeyboardFocusHere() API calls are considered tabbing requests!
// - Case 1: no nav/active id:    set result to first eligible item, stop storing.
// - Case 2: tab forward:         on ref id set counter, on counter elapse store result
// - Case 3: tab forward wrap:    set result to first eligible item (preemptively), on ref id set counter, on next frame if counter hasn't elapsed store result. // FIXME-TABBING: Could be done as a next-frame forwarded request
// - Case 4: tab backward:        store all results, on ref id pick prev, stop storing
// - Case 5: tab backward wrap:   store all results, on ref id if no result keep storing until last // FIXME-TABBING: Could be done as next-frame forwarded requested
// void NavProcessItemForTabbingRequest(ImGuiID id)
pub fn nav_process_item_for_tabbing_request(g: &mut Context, id: Id32)
{
    // ImGuiContext& g = *GImGui;

    // Always store in nav_move_result_local (unlike directional request which uses nav_move_result_other on sibling/flattened windows)
    ImGuiNavItemData* result = &g.NavMoveResultLocal;
    if (g.NavTabbingDir == +1)
    {
        // Tab Forward or SetKeyboardFocusHere() with >= 0
        if (g.NavTabbingResultFirst.id == 0)
            NavApplyItemToResult(&g.NavTabbingResultFirst);
        if (g.NavTabbingCounter -= 1 == 0)
            NavMoveRequestResolveWithLastItem(result);
        else if (g.nav_id == id)
            g.NavTabbingCounter = 1;
    }
    else if (g.NavTabbingDir == -1)
    {
        // Tab Backward
        if (g.nav_id == id)
        {
            if (result.id)
            {
                g.NavMoveScoringItems = false;
                NavUpdateAnyRequestFlag();
            }
        }
        else
        {
            NavApplyItemToResult(result);
        }
    }
    else if (g.NavTabbingDir == 0)
    {
        // Tab Init
        if (g.NavTabbingResultFirst.id == 0)
            NavMoveRequestResolveWithLastItem(&g.NavTabbingResultFirst);
    }
}

// bool NavMoveRequestButNoResultYet()
pub fn nav_move_request_but_no_result_yet(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    return g.NavMoveScoringItems && g.NavMoveResultLocal.id == 0 && g.NavMoveResultOther.id == 0;
}

// FIXME: ScoringRect is not set
// void NavMoveRequestSubmit(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags)
pub fn nav_move_request_submit(g: &mut Context, move_dir: Direction, clip_dir: Direction, move_flags: &HashSet<NavMoveFlags>, scroll_flags: &HashSet<ScrollFlags>)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.nav_window != None);

    if (move_flags & ImGuiNavMoveFlags_Tabbing)
        move_flags |= ImGuiNavMoveFlags_AllowCurrentNavId;

    g.NavMoveSubmitted = g.NavMoveScoringItems = true;
    g.NavMoveDir = move_dir;
    g.NavMoveDirForDebug = move_dir;
    g.NavMoveClipDir = clip_dir;
    g.NavMoveFlags = move_flags;
    g.NavMoveScrollFlags = scroll_flags;
    g.NavMoveForwardToNextFrame = false;
    g.NavMoveKeyMods = g.io.key_mods;
    g.NavMoveResultLocal.Clear();
    g.NavMoveResultLocalVisible.Clear();
    g.NavMoveResultOther.Clear();
    g.NavTabbingCounter = 0;
    g.NavTabbingResultFirst.Clear();
    NavUpdateAnyRequestFlag();
}

// void NavMoveRequestResolveWithLastItem(ImGuiNavItemData* result)
pub fn nav_move_request_resolve_with_last_item(g: &mut Context, result: &mut NavItemData)
{
    // ImGuiContext& g = *GImGui;
    g.NavMoveScoringItems = false; // Ensure request doesn't need more processing
    NavApplyItemToResult(result);
    NavUpdateAnyRequestFlag();
}

// void NavMoveRequestCancel()
pub fn nav_move_request_cancel(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    g.NavMoveSubmitted = g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

// Forward will reuse the move request again on the next frame (generally with modifications done to it)
// void NavMoveRequestForward(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags)
pub fn nav_move_request_forward(g: &mut Context, move_dir: Direction, clip_dir: Direction, move_flags: &HashSet<NavMoveFlags>, scroll_flags: &HashSet<ScrollFlags>)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavMoveForwardToNextFrame == false);
    NavMoveRequestCancel();
    g.NavMoveForwardToNextFrame = true;
    g.NavMoveDir = move_dir;
    g.NavMoveClipDir = clip_dir;
    g.NavMoveFlags = move_flags | ImGuiNavMoveFlags_Forwarded;
    g.NavMoveScrollFlags = scroll_flags;
}

// Navigation wrap-around logic is delayed to the end of the frame because this operation is only valid after entire
// popup is assembled and in case of appended popups it is not clear which EndPopup() call is final.
// void NavMoveRequestTryWrapping(ImGuiWindow* window, ImGuiNavMoveFlags wrap_flags)
pub fn nav_move_request_try_wrapping(g: &mut Context, window: &mut Window, wrap_flags: &HashSet<NavMoveFlags>)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(wrap_flags != 0); // Call with _WrapX, _WrapY, _LoopX, _LoopY
    // In theory we should test for NavMoveRequestButNoResultYet() but there's no point doing it, NavEndFrame() will do the same test
    if (g.nav_window == window && g.NavMoveScoringItems && g.NavLayer == NavLayer::Main)
        g.NavMoveFlags |= wrap_flags;
}

// FIXME: This could be replaced by updating a frame number in each window when (window == nav_window) and (nav_layer == 0).
// This way we could find the last focused window among our children. It would be much less confusing this way?
// static void NavSaveLastChildNavWindowIntoParent(ImGuiWindow* nav_window)
pub fn nav_save_last_child_nav_window_int_parent(g: &mut Context, nav_window: &mut Window)
{
    ImGuiWindow* parent = nav_window;
    while (parent && parent.root_window != parent && (parent.flags & (WindowFlags::Popup | WindowFlags::ChildMenu)) == 0)
        parent = parent.parent_window;
    if (parent && parent != nav_window)
        parent.NavLastChildNavWindow = nav_window;
}

// Restore the last focused child.
// Call when we are expected to land on the Main Layer (0) after focus_window()
// static ImGuiWindow* NavRestoreLastChildNavWindow(ImGuiWindow* window)
pub fn nav_restore_last_child_nav_window(g: &mut Context, window: &mut Window)
{
    if (window.NavLastChildNavWindow && window.NavLastChildNavWindow.WasActive)
        return window.NavLastChildNavWindow;
    if (window.dock_node_as_host_id && window.dock_node_as_host_id.tab_bar)
        if (ImGuiTabItem* tab = TabBarFindMostRecentlySelectedTabForActiveWindow(window.dock_node_as_host_id.tab_bar))
            return tab.Window;
    return window;
}

// void NavRestoreLayer(ImGuiNavLayer layer)
pub fn nav_restore_layer(g: &mut Context, layer: NavLayer)
{
    // ImGuiContext& g = *GImGui;
    if (layer == NavLayer::Main)
    {
        ImGuiWindow* prev_nav_window = g.nav_window;
        g.nav_window = NavRestoreLastChildNavWindow(g.nav_window);    // FIXME-NAV: Should clear ongoing nav requests?
        if (prev_nav_window)
            IMGUI_DEBUG_LOG_FOCUS("[focus] NavRestoreLayer: from \"%s\" to SetNavWindow(\"%s\")\n", prev_nav_window.name, g.nav_window.name);
    }
    ImGuiWindow* window = g.nav_window;
    if (window.nav_last_ids[layer] != 0)
    {
        SetNavID(window.nav_last_ids[layer], layer, 0, window.NavRectRel[layer]);
    }
    else
    {
        g.NavLayer = layer;
        nav_init_window(window, true);
    }
}

// void NavRestoreHighlightAfterMove()
pub fn nav_restore_highlight_after_move(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    g.nav_disable_highlight = false;
    g.nav_disable_mouse_hover = g.NavMousePosDirty = true;
}

// static inline void NavUpdateAnyRequestFlag()
pub fn nav_update_any_request_flag(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    g.NavAnyRequest = g.NavMoveScoringItems || g.NavInitRequest || (IMGUI_DEBUG_NAV_SCORING && g.nav_window != None);
    if (g.NavAnyRequest)
        // IM_ASSERT(g.nav_window != None);
}

// This needs to be called before we submit any widget (aka in or before Begin)
// void nav_init_window(ImGuiWindow* window, bool force_reinit)
pub fn nav_init_window(g: &mut Context, window: &mut Window, force_reinit: bool)
{
    // FIXME: ChildWindow test here is wrong for docking
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window == g.nav_window);

    if (window.flags & WindowFlags::NoNavInputs)
    {
        g.nav_id = g.NavFocusScopeId = 0;
        return;
    }

    bool init_for_nav = false;
    if (window == window.root_window || (window.flags & WindowFlags::Popup) || (window.nav_last_ids[0] == 0) || force_reinit)
        init_for_nav = true;
    IMGUI_DEBUG_LOG_NAV("[nav] nav_init_request: from nav_init_window(), init_for_nav=%d, window=\"%s\", layer=%d\n", init_for_nav, window.name, g.NavLayer);
    if (init_for_nav)
    {
        SetNavID(0, g.NavLayer, 0, Rect());
        g.NavInitRequest = true;
        g.NavInitRequestFromMove = false;
        g.NavInitResultId = 0;
        g.NavInitResultRectRel = Rect();
        NavUpdateAnyRequestFlag();
    }
    else
    {
        g.nav_id = window.nav_last_ids[0];
        g.NavFocusScopeId = 0;
    }
}

// static Vector2D NavCalcPreferredRefPos()
pub fn nav_cal_preferred_ref_pos(g: &mut Context) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;
    if (g.nav_disable_highlight || !g.nav_disable_mouse_hover || !window)
    {
        // Mouse (we need a fallback in case the mouse becomes invalid after being used)
        // The +1.0 offset when stored by OpenPopupEx() allows reopening this or another popup (same or another mouse button) while not moving the mouse, it is pretty standard.
        // In theory we could move that +1.0 offset in OpenPopupEx()
        Vector2D p = is_mouse_pos_valid(&g.io.mouse_pos) ? g.io.mouse_pos : g.mouse_last_valid_pos;
        return Vector2D::new(p.x + 1.0, p.y);
    }
    else
    {
        // When navigation is active and mouse is disabled, pick a position around the bottom left of the currently navigated item
        // Take account of upcoming scrolling (maybe set mouse pos should be done in EndFrame?)
        Rect rect_rel = WindowRectRelToAbs(window, window.NavRectRel[g.NavLayer]);
        if (window.last_frame_active != g.frame_count && (window.ScrollTarget.x != f32::MAX || window.ScrollTarget.y != f32::MAX))
        {
            Vector2D next_scroll = CalcNextScrollFromScrollTargetAndClamp(window);
            rect_rel.Translate(window.scroll - next_scroll);
        }
        Vector2D pos = Vector2D::new(rect_rel.min.x + ImMin(g.style.frame_padding.x * 4, rect_rel.get_width()), rect_rel.max.y - ImMin(g.style.frame_padding.y, rect_rel.get_height()));
        ImGuiViewport* viewport = window.viewport;
        return f32::floor(ImClamp(pos, viewport.pos, viewport.pos + viewport.size)); // f32::floor() is important because non-integer mouse position application in backend might be lossy and result in undesirable non-zero delta.
    }
}

// const char* GetNavInputName(ImGuiNavInput n)
pub fn get_nav_input_name(g: &mut Context, n: NavInput) -> String
{
    static const char* names[] =
    {
        "Activate", "Cancel", "Input", "Menu", "DpadLeft", "DpadRight", "DpadUp", "DpadDown", "LStickLeft", "LStickRight", "LStickUp", "LStickDown",
        "FocusPrev", "FocusNext", "TweakSlow", "TweakFast", "KeyLeft", "KeyRight", "KeyUp", "KeyDown"
    };
    // IM_ASSERT(IM_ARRAYSIZE(names) == ImGuiNavInput_COUNT);
    // IM_ASSERT(n >= 0 && n < ImGuiNavInput_COUNT);
    return names[n];
}

// float GetNavInputAmount(ImGuiNavInput n, ImGuiNavReadMode mode)
pub fn get_nav_input_amount(g: &mut Context, n: NavInput, mode: NavReadMode)
{
    // ImGuiContext& g = *GImGui;
    if (mode == NavReadMode::Down)
        return g.io.NavInputs[n];                         // Instant, read analog input (0.0..1.0, as provided by user)

    let t = g.io.NavInputsDownDuration[n];
    if (t < 0.0 && mode == NavReadMode::Released)  // Return 1.0 when just released, no repeat, ignore analog input.
        return (g.io.NavInputsDownDurationPrev[n] >= 0.0 ? 1.0 : 0.0);
    if (t < 0.0)
        return 0.0;
    if (mode == NavReadMode::Pressed)               // Return 1.0 when just pressed, no repeat, ignore analog input.
        return (t == 0.0) ? 1.0 : 0.0;
    if (mode == NavReadMode::Repeat)
        return CalcTypematicRepeatAmount(t - g.io.delta_time, t, g.io.KeyRepeatDelay * 0.72, g.io.KeyRepeatRate * 0.80);
    if (mode == NavReadMode::RepeatSlow)
        return CalcTypematicRepeatAmount(t - g.io.delta_time, t, g.io.KeyRepeatDelay * 1.25, g.io.KeyRepeatRate * 2.00);
    if (mode == NavReadMode::RepeatFast)
        return CalcTypematicRepeatAmount(t - g.io.delta_time, t, g.io.KeyRepeatDelay * 0.72, g.io.KeyRepeatRate * 0.30);
    return 0.0;
}

// Vector2D get_nav_input_amount_2d(ImGuiNavDirSourceFlags dir_sources, ImGuiNavReadMode mode, float slow_factor, float fast_factor)
pub fn get_nav_input_amount_2d(g: &mut Context, dir_sources: &HashSet<NavDirSourceFlags>, mode: NavReadMode, slow_factor: f32, fast_factor: f32) -> Vector2D
{
    Vector2D delta(0.0, 0.0);
    if (dir_sources & NavDirSourceFlags::RawKeyboard)
        delta += Vector2D::new((float)IsKeyDown(ImGuiKey_RightArrow) - IsKeyDown(ImGuiKey_LeftArrow), IsKeyDown(ImGuiKey_DownArrow) - IsKeyDown(ImGuiKey_UpArrow));
    if (dir_sources & NavDirSourceFlags::Keyboard)
        delta += Vector2D::new(GetNavInputAmount(ImGuiNavInput_KeyRight_, mode)   - GetNavInputAmount(ImGuiNavInput_KeyLeft_,   mode), GetNavInputAmount(ImGuiNavInput_KeyDown_,   mode) - GetNavInputAmount(ImGuiNavInput_KeyUp_,   mode));
    if (dir_sources & NavDirSourceFlags::PadDPad)
        delta += Vector2D::new(GetNavInputAmount(ImGuiNavInput_DpadRight, mode)   - GetNavInputAmount(ImGuiNavInput_DpadLeft,   mode), GetNavInputAmount(ImGuiNavInput_DpadDown,   mode) - GetNavInputAmount(ImGuiNavInput_DpadUp,   mode));
    if (dir_sources & NavDirSourceFlags::PadLStick)
        delta += Vector2D::new(GetNavInputAmount(ImGuiNavInput_LStickRight, mode) - GetNavInputAmount(ImGuiNavInput_LStickLeft, mode), GetNavInputAmount(ImGuiNavInput_LStickDown, mode) - GetNavInputAmount(ImGuiNavInput_LStickUp, mode));
    if (slow_factor != 0.0 && IsNavInputDown(ImGuiNavInput_TweakSlow))
        delta *= slow_factor;
    if (fast_factor != 0.0 && IsNavInputDown(ImGuiNavInput_TweakFast))
        delta *= fast_factor;
    return delta;
}

// static void nav_update()
pub fn nav_update(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;

    io.WantSetMousePos = false;
    //if (g.nav_scoring_debug_count > 0) IMGUI_DEBUG_LOG_NAV("[nav] nav_scoring_debug_count %d for '%s' layer %d (Init:%d, Move:%d)\n", g.nav_scoring_debug_count, g.nav_window ? g.nav_window->name : "None", g.nav_layer, g.nav_init_request || g.nav_init_result_id != 0, g.NavMoveRequest);

    // Update Gamepad->Nav inputs mapping
    // Set input source as Gamepad when buttons are pressed (as some features differs when used with Gamepad vs Keyboard)
    const bool nav_gamepad_active = (io.config_flags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.backend_flags & BackendFlags::HasGamepad) != 0;
    if (nav_gamepad_active && g.io.BackendUsingLegacyNavInputArray == false)
    {
        for (int n = 0; n < ImGuiNavInput_COUNT; n += 1)
            // IM_ASSERT(io.NavInputs[n] == 0.0 && "Backend needs to either only use io.add_key_event()/io.add_key_analog_event(), either only fill legacy io.nav_inputs[]. Not both!");
        #define NAV_MAP_KEY(_KEY, _NAV_INPUT, _ACTIVATE_NAV)  do { io.NavInputs[_NAV_INPUT] = io.keys_data[_KEY - Key::KeysDataOffset].analog_value; if (_ACTIVATE_NAV && io.NavInputs[_NAV_INPUT] > 0.0) { g.nav_input_source = InputSource::Gamepad; } } while (0)
        NAV_MAP_KEY(ImGuiKey_GamepadFaceDown, ImGuiNavInput_Activate, true);
        NAV_MAP_KEY(ImGuiKey_GamepadFaceRight, ImGuiNavInput_Cancel, true);
        NAV_MAP_KEY(ImGuiKey_GamepadFaceLeft, ImGuiNavInput_Menu, true);
        NAV_MAP_KEY(ImGuiKey_GamepadFaceUp, ImGuiNavInput_Input, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadLeft, ImGuiNavInput_DpadLeft, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadRight, ImGuiNavInput_DpadRight, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadUp, ImGuiNavInput_DpadUp, true);
        NAV_MAP_KEY(ImGuiKey_GamepadDpadDown, ImGuiNavInput_DpadDown, true);
        NAV_MAP_KEY(ImGuiKey_GamepadL1, ImGuiNavInput_FocusPrev, false);
        NAV_MAP_KEY(ImGuiKey_GamepadR1, ImGuiNavInput_FocusNext, false);
        NAV_MAP_KEY(ImGuiKey_GamepadL1, ImGuiNavInput_TweakSlow, false);
        NAV_MAP_KEY(ImGuiKey_GamepadR1, ImGuiNavInput_TweakFast, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickLeft, ImGuiNavInput_LStickLeft, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickRight, ImGuiNavInput_LStickRight, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickUp, ImGuiNavInput_LStickUp, false);
        NAV_MAP_KEY(ImGuiKey_GamepadLStickDown, ImGuiNavInput_LStickDown, false);
        #undef NAV_MAP_KEY
    }

    // Update Keyboard->Nav inputs mapping
    const bool nav_keyboard_active = (io.config_flags & ConfigFlags::NavEnableKeyboard) != 0;
    if (nav_keyboard_active)
    {
        #define NAV_MAP_KEY(_KEY, _NAV_INPUT)  do { if (IsKeyDown(_KEY)) { io.NavInputs[_NAV_INPUT] = 1.0; g.nav_input_source = InputSource::Keyboard; } } while (0)
        NAV_MAP_KEY(ImGuiKey_Space,     ImGuiNavInput_Activate );
        NAV_MAP_KEY(ImGuiKey_Enter,     ImGuiNavInput_Input    );
        NAV_MAP_KEY(ImGuiKey_Escape,    ImGuiNavInput_Cancel   );
        NAV_MAP_KEY(ImGuiKey_LeftArrow, ImGuiNavInput_KeyLeft_ );
        NAV_MAP_KEY(ImGuiKey_RightArrow,ImGuiNavInput_KeyRight_);
        NAV_MAP_KEY(ImGuiKey_UpArrow,   ImGuiNavInput_KeyUp_   );
        NAV_MAP_KEY(ImGuiKey_DownArrow, ImGuiNavInput_KeyDown_ );
        if (io.key_ctrl)
            io.NavInputs[ImGuiNavInput_TweakSlow] = 1.0;
        if (io.key_shift)
            io.NavInputs[ImGuiNavInput_TweakFast] = 1.0;
        #undef NAV_MAP_KEY
    }
    memcpy(io.NavInputsDownDurationPrev, io.NavInputsDownDuration, sizeof(io.NavInputsDownDuration));
    for (int i = 0; i < IM_ARRAYSIZE(io.NavInputs); i += 1)
        io.NavInputsDownDuration[i] = (io.NavInputs[i] > 0.0) ? (io.NavInputsDownDuration[i] < 0.0 ? 0.0 : io.NavInputsDownDuration[i] + io.delta_time) : -1.0;

    // Process navigation init request (select first/default focus)
    if (g.NavInitResultId != 0)
        NavInitRequestApplyResult();
    g.NavInitRequest = false;
    g.NavInitRequestFromMove = false;
    g.NavInitResultId = 0;
    g.NavJustMovedToId = 0;

    // Process navigation move request
    if (g.NavMoveSubmitted)
        NavMoveRequestApplyResult();
    g.NavTabbingCounter = 0;
    g.NavMoveSubmitted = g.NavMoveScoringItems = false;

    // Schedule mouse position update (will be done at the bottom of this function, after 1) processing all move requests and 2) updating scrolling)
    bool set_mouse_pos = false;
    if (g.NavMousePosDirty && g.NavIdIsAlive)
        if (!g.nav_disable_highlight && g.nav_disable_mouse_hover && g.nav_window)
            set_mouse_pos = true;
    g.NavMousePosDirty = false;
    // IM_ASSERT(g.NavLayer == NavLayer::Main || g.NavLayer == NavLayer::Menu);

    // Store our return window (for returning from Menu Layer to Main Layer) and clear it as soon as we step back in our own Layer 0
    if (g.nav_window)
        NavSaveLastChildNavWindowIntoParent(g.nav_window);
    if (g.nav_window && g.nav_window.NavLastChildNavWindow != None && g.NavLayer == NavLayer::Main)
        g.nav_window.NavLastChildNavWindow = None;

    // Update CTRL+TAB and Windowing features (hold Square to move/resize/etc.)
    NavUpdateWindowing();

    // Set output flags for user application
    io.nav_active = (nav_keyboard_active || nav_gamepad_active) && g.nav_window && !(g.nav_window.flags & WindowFlags::NoNavInputs);
    io.NavVisible = (io.nav_active && g.nav_id != 0 && !g.nav_disable_highlight) || (g.nav_windowing_target != None);

    // Process NavCancel input (to close a popup, get back to parent, clear focus)
    NavUpdateCancelRequest();

    // Process manual activation request
    g.nav_activate_id = g.NavActivateDownId = g.NavActivatePressedId = g.NavActivateInputId = 0;
    g.NavActivateFlags = ImGuiActivateFlags_None;
    if (g.nav_id != 0 && !g.nav_disable_highlight && !g.nav_windowing_target && g.nav_window && !(g.nav_window.flags & WindowFlags::NoNavInputs))
    {
        bool activate_down = IsNavInputDown(ImGuiNavInput_Activate);
        bool input_down = IsNavInputDown(ImGuiNavInput_Input);
        bool activate_pressed = activate_down && IsNavInputTest(ImGuiNavInput_Activate, NavReadMode::Pressed);
        bool input_pressed = input_down && IsNavInputTest(ImGuiNavInput_Input, NavReadMode::Pressed);
        if (g.active_id == 0 && activate_pressed)
        {
            g.nav_activate_id = g.nav_id;
            g.NavActivateFlags = ImGuiActivateFlags_PreferTweak;
        }
        if ((g.active_id == 0 || g.active_id == g.nav_id) && input_pressed)
        {
            g.NavActivateInputId = g.nav_id;
            g.NavActivateFlags = ImGuiActivateFlags_PreferInput;
        }
        if ((g.active_id == 0 || g.active_id == g.nav_id) && activate_down)
            g.NavActivateDownId = g.nav_id;
        if ((g.active_id == 0 || g.active_id == g.nav_id) && activate_pressed)
            g.NavActivatePressedId = g.nav_id;
    }
    if (g.nav_window && (g.nav_window.flags & WindowFlags::NoNavInputs))
        g.nav_disable_highlight = true;
    if (g.nav_activate_id != 0)
        // IM_ASSERT(g.NavActivateDownId == g.nav_activate_id);

    // Process programmatic activation request
    // FIXME-NAV: Those should eventually be queued (unlike focus they don't cancel each others)
    if (g.NavNextActivateId != 0)
    {
        if (g.NavNextActivateFlags & ImGuiActivateFlags_PreferInput)
            g.NavActivateInputId = g.NavNextActivateId;
        else
            g.nav_activate_id = g.NavActivateDownId = g.NavActivatePressedId = g.NavNextActivateId;
        g.NavActivateFlags = g.NavNextActivateFlags;
    }
    g.NavNextActivateId = 0;

    // Process move requests
    NavUpdateCreateMoveRequest();
    if (g.NavMoveDir == Direction::None)
        NavUpdateCreateTabbingRequest();
    NavUpdateAnyRequestFlag();
    g.NavIdIsAlive = false;

    // Scrolling
    if (g.nav_window && !(g.nav_window.flags & WindowFlags::NoNavInputs) && !g.nav_windowing_target)
    {
        // *Fallback* manual-scroll with Nav directional keys when window has no navigable item
        ImGuiWindow* window = g.nav_window;
        let scroll_speed = IM_ROUND(window.CalcFontSize() * 100 * io.delta_time); // We need round the scrolling speed because sub-pixel scroll isn't reliably supported.
        const ImGuiDir move_dir = g.NavMoveDir;
        if (window.dc.nav_layers_active_mask == 0x00 && window.dc.nav_has_scroll && move_dir != Direction::None)
        {
            if (move_dir == Direction::Left || move_dir == Direction::Right)
                set_scroll_x(window, f32::floor(window.scroll.x + ((move_dir == Direction::Left) ? -1.0 : +1.0) * scroll_speed));
            if (move_dir == Direction::Up || move_dir == Direction::Down)
                set_scroll_y(window, f32::floor(window.scroll.y + ((move_dir == Direction::Up) ? -1.0 : +1.0) * scroll_speed));
        }

        // *Normal* Manual scroll with NavScrollXXX keys
        // Next movement request will clamp the nav_id reference rectangle to the visible area, so navigation will resume within those bounds.
        Vector2D scroll_dir = get_nav_input_amount_2d(NavDirSourceFlags::PadLStick, NavReadMode::Down, 1.0 / 10.0, 10.0);
        if (scroll_dir.x != 0.0 && window.scrollbar_x)
            set_scroll_x(window, f32::floor(window.scroll.x + scroll_dir.x * scroll_speed));
        if (scroll_dir.y != 0.0)
            set_scroll_y(window, f32::floor(window.scroll.y + scroll_dir.y * scroll_speed));
    }

    // Always prioritize mouse highlight if navigation is disabled
    if (!nav_keyboard_active && !nav_gamepad_active)
    {
        g.nav_disable_highlight = true;
        g.nav_disable_mouse_hover = set_mouse_pos = false;
    }

    // Update mouse position if requested
    // (This will take into account the possibility that a scroll was queued in the window to offset our absolute mouse position before scroll has been applied)
    if (set_mouse_pos && (io.config_flags & ImGuiConfigFlags_NavEnableSetMousePos) && (io.backend_flags & ImGuiBackendFlags_HasSetMousePos))
    {
        io.mouse_pos = io.mouse_pos_prev = NavCalcPreferredRefPos();
        io.WantSetMousePos = true;
        //IMGUI_DEBUG_LOG_IO("SetMousePos: (%.1,%.1)\n", io.mouse_pos.x, io.mouse_pos.y);
    }

    // [DEBUG]
    g.NavScoringDebugCount = 0;
// #if IMGUI_DEBUG_NAV_RECTS
    if (g.nav_window)
    {
        ImDrawList* draw_list = get_foreground_draw_list(g.nav_window);
        if (1) { for (int layer = 0; layer < 2; layer += 1) { Rect r = WindowRectRelToAbs(g.nav_window, g.nav_window.NavRectRel[layer]); draw_list.add_rect(r.min, r.max, IM_COL32(255,200,0,255)); } } // [DEBUG]
        if (1) { ImU32 col = (!g.nav_window.Hidden) ? IM_COL32(255,0,255,255) : IM_COL32(255,0,0,255); Vector2D p = NavCalcPreferredRefPos(); char buf[32]; ImFormatString(buf, 32, "%d", g.NavLayer); draw_list.AddCircleFilled(p, 3.0, col); draw_list.AddText(None, 13.0, p + Vector2D::new(8,-4), col, buf); }
    }

}

// void NavInitRequestApplyResult()
pub fn nav_init_request_apply_result(g: &mut Context)
{
    // In very rare cases g.nav_window may be null (e.g. clearing focus after requesting an init request, which does happen when releasing Alt while clicking on void)
    // ImGuiContext& g = *GImGui;
    if (!g.nav_window)
        return;

    // Apply result from previous navigation init request (will typically select the first item, unless SetItemDefaultFocus() has been called)
    // FIXME-NAV: On _NavFlattened windows, g.nav_window will only be updated during subsequent frame. Not a problem currently.
    IMGUI_DEBUG_LOG_NAV("[nav] nav_init_request: ApplyResult: NavID 0x%08X in Layer %d window \"%s\"\n", g.NavInitResultId, g.NavLayer, g.nav_window.name);
    SetNavID(g.NavInitResultId, g.NavLayer, 0, g.NavInitResultRectRel);
    g.NavIdIsAlive = true; // Mark as alive from previous frame as we got a result
    if (g.NavInitRequestFromMove)
        NavRestoreHighlightAfterMove();
}

// void NavUpdateCreateMoveRequest()
pub fn nav_update_create_move_request(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;
    ImGuiWindow* window = g.nav_window;

    if (g.NavMoveForwardToNextFrame && window != None)
    {
        // Forwarding previous request (which has been modified, e.g. wrap around menus rewrite the requests with a starting rectangle at the other side of the window)
        // (preserve most state, which were already set by the NavMoveRequestForward() function)
        // IM_ASSERT(g.NavMoveDir != Dir::None && g.NavMoveClipDir != Dir::None);
        // IM_ASSERT(g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded);
        IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequestForward %d\n", g.NavMoveDir);
    }
    else
    {
        // Initiate directional inputs request
        g.NavMoveDir = Direction::None;
        g.NavMoveFlags = ImGuiNavMoveFlags_None;
        g.NavMoveScrollFlags = ImGuiScrollFlags_None;
        if (window && !g.nav_windowing_target && !(window.flags & WindowFlags::NoNavInputs))
        {
            const ImGuiNavReadMode read_mode = NavReadMode::Repeat;
            if (!IsActiveIdUsingNavDir(Direction::Left)  && (IsNavInputTest(ImGuiNavInput_DpadLeft,  read_mode) || IsNavInputTest(ImGuiNavInput_KeyLeft_,  read_mode))) { g.NavMoveDir = Direction::Left; }
            if (!IsActiveIdUsingNavDir(Direction::Right) && (IsNavInputTest(ImGuiNavInput_DpadRight, read_mode) || IsNavInputTest(ImGuiNavInput_KeyRight_, read_mode))) { g.NavMoveDir = Direction::Right; }
            if (!IsActiveIdUsingNavDir(Direction::Up)    && (IsNavInputTest(ImGuiNavInput_DpadUp,    read_mode) || IsNavInputTest(ImGuiNavInput_KeyUp_,    read_mode))) { g.NavMoveDir = Direction::Up; }
            if (!IsActiveIdUsingNavDir(Direction::Down)  && (IsNavInputTest(ImGuiNavInput_DpadDown,  read_mode) || IsNavInputTest(ImGuiNavInput_KeyDown_,  read_mode))) { g.NavMoveDir = Direction::Down; }
        }
        g.NavMoveClipDir = g.NavMoveDir;
        g.NavScoringNoClipRect = Rect(+f32::MAX, +f32::MAX, -f32::MAX, -f32::MAX);
    }

    // Update PageUp/PageDown/Home/End scroll
    // FIXME-NAV: Consider enabling those keys even without the master ImGuiConfigFlags_NavEnableKeyboard flag?
    const bool nav_keyboard_active = (io.config_flags & ConfigFlags::NavEnableKeyboard) != 0;
    float scoring_rect_offset_y = 0.0;
    if (window && g.NavMoveDir == Direction::None && nav_keyboard_active)
        scoring_rect_offset_y = NavUpdatePageUpPageDown();
    if (scoring_rect_offset_y != 0.0)
    {
        g.NavScoringNoClipRect = window.inner_rect;
        g.NavScoringNoClipRect.TranslateY(scoring_rect_offset_y);
    }

    // [DEBUG] Always send a request
// #ifIMGUI_DEBUG_NAV_SCORING
    if (io.key_ctrl && IsKeyPressed(ImGuiKey_C))
        g.NavMoveDirForDebug = (ImGuiDir)((g.NavMoveDirForDebug + 1) & 3);
    if (io.key_ctrl && g.NavMoveDir == Direction::None)
    {
        g.NavMoveDir = g.NavMoveDirForDebug;
        g.NavMoveFlags |= ImGuiNavMoveFlags_DebugNoResult;
    }


    // Submit
    g.NavMoveForwardToNextFrame = false;
    if (g.NavMoveDir != Direction::None)
        NavMoveRequestSubmit(g.NavMoveDir, g.NavMoveClipDir, g.NavMoveFlags, g.NavMoveScrollFlags);

    // Moving with no reference triggers a init request (will be used as a fallback if the direction fails to find a match)
    if (g.NavMoveSubmitted && g.nav_id == 0)
    {
        IMGUI_DEBUG_LOG_NAV("[nav] nav_init_request: from move, window \"%s\", layer=%d\n", window ? window.name : "<None>", g.NavLayer);
        g.NavInitRequest = g.NavInitRequestFromMove = true;
        g.NavInitResultId = 0;
        g.nav_disable_highlight = false;
    }

    // When using gamepad, we project the reference nav bounding box into window visible area.
    // This is to allow resuming navigation inside the visible area after doing a large amount of scrolling, since with gamepad every movements are relative
    // (can't focus a visible object like we can with the mouse).
    if (g.NavMoveSubmitted && g.nav_input_source == InputSource::Gamepad && g.NavLayer == NavLayer::Main && window != None)// && (g.nav_move_flags & ImGuiNavMoveFlags_Forwarded))
    {
        bool clamp_x = (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapX)) == 0;
        bool clamp_y = (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopY | ImGuiNavMoveFlags_WrapY)) == 0;
        Rect inner_rect_rel = window_rect_abs_to_rel(window, Rect(window.inner_rect.min - Vector2D::new(1, 1), window.inner_rect.max + Vector2D::new(1, 1)));
        if ((clamp_x || clamp_y) && !inner_rect_rel.contains(window.NavRectRel[g.NavLayer]))
        {
            //IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: clamp nav_rect_rel for gamepad move\n");
            float pad_x = ImMin(inner_rect_rel.get_width(), window.CalcFontSize() * 0.5);
            float pad_y = ImMin(inner_rect_rel.get_height(), window.CalcFontSize() * 0.5); // Terrible approximation for the intent of starting navigation from first fully visible item
            inner_rect_rel.min.x = clamp_x ? (inner_rect_rel.min.x + pad_x) : -f32::MAX;
            inner_rect_rel.max.x = clamp_x ? (inner_rect_rel.max.x - pad_x) : +f32::MAX;
            inner_rect_rel.min.y = clamp_y ? (inner_rect_rel.min.y + pad_y) : -f32::MAX;
            inner_rect_rel.max.y = clamp_y ? (inner_rect_rel.max.y - pad_y) : +f32::MAX;
            window.NavRectRel[g.NavLayer].ClipWithFull(inner_rect_rel);
            g.nav_id = g.NavFocusScopeId = 0;
        }
    }

    // For scoring we use a single segment on the left side our current item bounding box (not touching the edge to avoid box overlap with zero-spaced items)
    Rect scoring_rect;
    if (window != None)
    {
        Rect nav_rect_rel = !window.NavRectRel[g.NavLayer].is_inverted() ? window.NavRectRel[g.NavLayer] : Rect(0, 0, 0, 0);
        scoring_rect = WindowRectRelToAbs(window, nav_rect_rel);
        scoring_rect.TranslateY(scoring_rect_offset_y);
        scoring_rect.min.x = ImMin(scoring_rect.min.x + 1.0, scoring_rect.max.x);
        scoring_rect.max.x = scoring_rect.min.x;
        // IM_ASSERT(!scoring_rect.is_inverted()); // Ensure if we have a finite, non-inverted bounding box here will allows us to remove extraneous f32::abs() calls in NavScoreItem().
        //GetForegroundDrawList()->add_rect(scoring_rect.min, scoring_rect.max, IM_COL32(255,200,0,255)); // [DEBUG]
        //if (!g.nav_scoring_no_clip_rect.is_inverted()) { GetForegroundDrawList()->add_rect(g.nav_scoring_no_clip_rect.min, g.nav_scoring_no_clip_rect.max, IM_COL32(255, 200, 0, 255)); } // [DEBUG]
    }
    g.NavScoringRect = scoring_rect;
    g.NavScoringNoClipRect.Add(scoring_rect);
}

// void NavUpdateCreateTabbingRequest()
pub fn nav_update_create_tabbing_request(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;
    // IM_ASSERT(g.NavMoveDir == Dir::None);
    if (window == None || g.nav_windowing_target != None || (window.flags & WindowFlags::NoNavInputs))
        return;

    const bool tab_pressed = IsKeyPressed(ImGuiKey_Tab, true) && !IsActiveIdUsingKey(ImGuiKey_Tab) && !g.io.key_ctrl && !g.io.key_alt;
    if (!tab_pressed)
        return;

    // Initiate tabbing request
    // (this is ALWAYS ENABLED, regardless of ImGuiConfigFlags_NavEnableKeyboard flag!)
    // Initially this was designed to use counters and modulo arithmetic, but that could not work with unsubmitted items (list clipper). Instead we use a strategy close to other move requests.
    // See NavProcessItemForTabbingRequest() for a description of the various forward/backward tabbing cases with and without wrapping.
    //// FIXME: We use (g.active_id == 0) but (g.NavDisableHighlight == false) might be righter once we can tab through anything
    g.NavTabbingDir = g.io.key_shift ? -1 : (g.active_id == 0) ? 0 : +1;
    ImGuiScrollFlags scroll_flags = window.Appearing ? ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleEdgeY;
    ImGuiDir clip_dir = (g.NavTabbingDir < 0) ? Direction::Up : Direction::Down;
    NavMoveRequestSubmit(Direction::None, clip_dir, ImGuiNavMoveFlags_Tabbing, scroll_flags); // FIXME-NAV: Once we refactor tabbing, add LegacyApi flag to not activate non-inputable.
    g.NavTabbingCounter = -1;
}

// Apply result from previous frame navigation directional move request. Always called from NavUpdate()
// void NavMoveRequestApplyResult()
pub fn nav_move_request_apply_result(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
// #ifIMGUI_DEBUG_NAV_SCORING
    if (g.NavMoveFlags & ImGuiNavMoveFlags_DebugNoResult) // [DEBUG] Scoring all items in nav_window at all times
        return;


    // Select which result to use
    ImGuiNavItemData* result = (g.NavMoveResultLocal.id != 0) ? &g.NavMoveResultLocal : (g.NavMoveResultOther.id != 0) ? &g.NavMoveResultOther : None;

    // Tabbing forward wrap
    if (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing)
        if ((g.NavTabbingCounter == 1 || g.NavTabbingDir == 0) && g.NavTabbingResultFirst.id)
            result = &g.NavTabbingResultFirst;

    // In a situation when there is no results but nav_id != 0, re-enable the Navigation highlight (because g.nav_id is not considered as a possible result)
    if (result == None)
    {
        if (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing)
            g.NavMoveFlags |= ImGuiNavMoveFlags_DontSetNavHighlight;
        if (g.nav_id != 0 && (g.NavMoveFlags & ImGuiNavMoveFlags_DontSetNavHighlight) == 0)
            NavRestoreHighlightAfterMove();
        return;
    }

    // PageUp/PageDown behavior first jumps to the bottom/top mostly visible item, _otherwise_ use the result from the previous/next page.
    if (g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet)
        if (g.NavMoveResultLocalVisible.id != 0 && g.NavMoveResultLocalVisible.id != g.nav_id)
            result = &g.NavMoveResultLocalVisible;

    // Maybe entering a flattened child from the outside? In this case solve the tie using the regular scoring rules.
    if (result != &g.NavMoveResultOther && g.NavMoveResultOther.id != 0 && g.NavMoveResultOther.Window.parent_window == g.nav_window)
        if ((g.NavMoveResultOther.DistBox < result.DistBox) || (g.NavMoveResultOther.DistBox == result.DistBox && g.NavMoveResultOther.DistCenter < result.DistCenter))
            result = &g.NavMoveResultOther;
    // IM_ASSERT(g.nav_window && result.Window);

    // scroll to keep newly navigated item fully into view.
    if (g.NavLayer == NavLayer::Main)
    {
        if (g.NavMoveFlags & ImGuiNavMoveFlags_ScrollToEdgeY)
        {
            // FIXME: Should remove this
            float scroll_target = (g.NavMoveDir == Direction::Up) ? result.Window->scroll_max.y : 0.0;
            set_scroll_y(result.Window, scroll_target);
        }
        else
        {
            Rect rect_abs = WindowRectRelToAbs(result.Window, result.RectRel);
            ScrollToRectEx(result.Window, rect_abs, g.NavMoveScrollFlags);
        }
    }

    if (g.nav_window != result.Window)
    {
        IMGUI_DEBUG_LOG_FOCUS("[focus] NavMoveRequest: SetNavWindow(\"%s\")\n", result.Window.name);
        g.nav_window = result.Window;
    }
    if (g.active_id != result.id)
        clear_active_id();
    if (g.nav_id != result.id)
    {
        // Don't set nav_just_moved_to_id if just landed on the same spot (which may happen with ImGuiNavMoveFlags_AllowCurrentNavId)
        g.NavJustMovedToId = result.id;
        g.NavJustMovedToFocusScopeId = result.FocusScopeId;
        g.NavJustMovedToKeyMods = g.NavMoveKeyMods;
    }

    // Focus
    IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: result NavID 0x%08X in Layer %d window \"%s\"\n", result.id, g.NavLayer, g.nav_window.name);
    SetNavID(result.id, g.NavLayer, result.FocusScopeId, result.RectRel);

    // Tabbing: Activates Inputable or Focus non-Inputable
    if ((g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) && (result.InFlags & ItemFlags::Inputable))
    {
        g.NavNextActivateId = result.id;
        g.NavNextActivateFlags = ImGuiActivateFlags_PreferInput | ImGuiActivateFlags_TryToPreserveState;
        g.NavMoveFlags |= ImGuiNavMoveFlags_DontSetNavHighlight;
    }

    // Activate
    if (g.NavMoveFlags & ImGuiNavMoveFlags_Activate)
    {
        g.NavNextActivateId = result.id;
        g.NavNextActivateFlags = ImGuiActivateFlags_None;
    }

    // Enable nav highlight
    if ((g.NavMoveFlags & ImGuiNavMoveFlags_DontSetNavHighlight) == 0)
        NavRestoreHighlightAfterMove();
}

// Process NavCancel input (to close a popup, get back to parent, clear focus)
// FIXME: In order to support e.g. Escape to clear a selection we'll need:
// - either to store the equivalent of active_id_using_key_input_mask for a FocusScope and test for it.
// - either to move most/all of those tests to the epilogue/end functions of the scope they are dealing with (e.g. exit child window in EndChild()) or in EndFrame(), to allow an earlier intercept
// static void NavUpdateCancelRequest()
pub fn nav_update_cancel_request(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    if (!IsNavInputTest(ImGuiNavInput_Cancel, NavReadMode::Pressed))
        return;

    IMGUI_DEBUG_LOG_NAV("[nav] ImGuiNavInput_Cancel\n");
    if (g.active_id != 0)
    {
        if (!IsActiveIdUsingNavInput(ImGuiNavInput_Cancel))
            clear_active_id();
    }
    else if (g.NavLayer != NavLayer::Main)
    {
        // Leave the "menu" layer
        NavRestoreLayer(NavLayer::Main);
        NavRestoreHighlightAfterMove();
    }
    else if (g.nav_window && g.nav_window != g.nav_window.root_window && !(g.nav_window.flags & WindowFlags::Popup) && g.nav_window.parent_window)
    {
        // Exit child window
        ImGuiWindow* child_window = g.nav_window;
        ImGuiWindow* parent_window = g.nav_window.parent_window;
        // IM_ASSERT(child_windowchild_id != 0);
        Rect child_rect = child_window.Rect();
        focus_window(parent_window);
        SetNavID(child_windowchild_id, NavLayer::Main, 0, window_rect_abs_to_rel(parent_window, child_rect));
        NavRestoreHighlightAfterMove();
    }
    else if (g.open_popup_stack.size > 0 && !(g.open_popup_stack.back().Window.flags & WindowFlags::Modal))
    {
        // Close open popup/menu
        ClosePopupToLevel(g.open_popup_stack.size - 1, true);
    }
    else
    {
        // clear NavLastId for popups but keep it for regular child window so we can leave one and come back where we were
        if (g.nav_window && ((g.nav_window.flags & WindowFlags::Popup) || !(g.nav_window.flags & WindowFlags::ChildWindow)))
            g.nav_window.nav_last_ids[0] = 0;
        g.nav_id = g.NavFocusScopeId = 0;
    }
}

// Handle PageUp/PageDown/Home/End keys
// Called from NavUpdateCreateMoveRequest() which will use our output to create a move request
// FIXME-NAV: This doesn't work properly with NavFlattened siblings as we use nav_window rectangle for reference
// FIXME-NAV: how to get Home/End to aim at the beginning/end of a 2D grid?
// static float NavUpdatePageUpPageDown()
pub fn nav_update_page_up_page_down(g: &mut Context) -> f32
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;
    if ((window.flags & WindowFlags::NoNavInputs) || g.nav_windowing_target != None)
        return 0.0;

    const bool page_up_held = IsKeyDown(ImGuiKey_PageUp) && !IsActiveIdUsingKey(ImGuiKey_PageUp);
    const bool page_down_held = IsKeyDown(ImGuiKey_PageDown) && !IsActiveIdUsingKey(ImGuiKey_PageDown);
    const bool home_pressed = IsKeyPressed(ImGuiKey_Home) && !IsActiveIdUsingKey(ImGuiKey_Home);
    const bool end_pressed = IsKeyPressed(ImGuiKey_End) && !IsActiveIdUsingKey(ImGuiKey_End);
    if (page_up_held == page_down_held && home_pressed == end_pressed) // Proceed if either (not both) are pressed, otherwise early out
        return 0.0;

    if (g.NavLayer != NavLayer::Main)
        NavRestoreLayer(NavLayer::Main);

    if (window.dc.nav_layers_active_mask == 0x00 && window.dc.nav_has_scroll)
    {
        // Fallback manual-scroll when window has no navigable item
        if (IsKeyPressed(ImGuiKey_PageUp, true))
            set_scroll_y(window, window.scroll.y - window.inner_rect.get_height());
        else if (IsKeyPressed(ImGuiKey_PageDown, true))
            set_scroll_y(window, window.scroll.y + window.inner_rect.get_height());
        else if (home_pressed)
            set_scroll_y(window, 0.0);
        else if (end_pressed)
            set_scroll_y(window, window.scroll_max.y);
    }
    else
    {
        Rect& nav_rect_rel = window.NavRectRel[g.NavLayer];
        let page_offset_y = ImMax(0.0, window.inner_rect.get_height() - window.CalcFontSize() * 1.0 + nav_rect_rel.get_height());
        float nav_scoring_rect_offset_y = 0.0;
        if (IsKeyPressed(ImGuiKey_PageUp, true))
        {
            nav_scoring_rect_offset_y = -page_offset_y;
            g.NavMoveDir = Direction::Down; // Because our scoring rect is offset up, we request the down direction (so we can always land on the last item)
            g.NavMoveClipDir = Direction::Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_AlsoScoreVisibleSet;
        }
        else if (IsKeyPressed(ImGuiKey_PageDown, true))
        {
            nav_scoring_rect_offset_y = +page_offset_y;
            g.NavMoveDir = Direction::Up; // Because our scoring rect is offset down, we request the up direction (so we can always land on the last item)
            g.NavMoveClipDir = Direction::Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_AlsoScoreVisibleSet;
        }
        else if (home_pressed)
        {
            // FIXME-NAV: handling of Home/End is assuming that the top/bottom most item will be visible with scroll.y == 0/scroll_max.y
            // Scrolling will be handled via the ImGuiNavMoveFlags_ScrollToEdgeY flag, we don't scroll immediately to avoid scrolling happening before nav result.
            // Preserve current horizontal position if we have any.
            nav_rect_rel.min.y = nav_rect_rel.max.y = 0.0;
            if (nav_rect_rel.is_inverted())
                nav_rect_rel.min.x = nav_rect_rel.max.x = 0.0;
            g.NavMoveDir = Direction::Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        else if (end_pressed)
        {
            nav_rect_rel.min.y = nav_rect_rel.max.y = window.ContentSize.y;
            if (nav_rect_rel.is_inverted())
                nav_rect_rel.min.x = nav_rect_rel.max.x = 0.0;
            g.NavMoveDir = Direction::Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        return nav_scoring_rect_offset_y;
    }
    return 0.0;
}

// static void NavEndFrame()
pub fn nav_end_frame(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;

    // Show CTRL+TAB list window
    if (g.nav_windowing_target != None)
        NavUpdateWindowingOverlay();

    // Perform wrap-around in menus
    // FIXME-NAV: Wrap may need to apply a weight bias on the other axis. e.g. 4x4 grid with 2 last items missing on last item won't handle LoopY/WrapY correctly.
    // FIXME-NAV: Wrap (not Loop) support could be handled by the scoring function and then WrapX would function without an extra frame.
    const ImGuiNavMoveFlags wanted_flags = ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY;
    if (g.nav_window && NavMoveRequestButNoResultYet() && (g.NavMoveFlags & wanted_flags) && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded) == 0)
        NavUpdateCreateWrappingRequest();
}

// static void NavUpdateCreateWrappingRequest()
pub fn nav_update_create_wrapping_request(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.nav_window;

    bool do_forward = false;
    Rect bb_rel = window.NavRectRel[g.NavLayer];
    ImGuiDir clip_dir = g.NavMoveDir;
    const ImGuiNavMoveFlags move_flags = g.NavMoveFlags;
    if (g.NavMoveDir == Direction::Left && (move_flags & (ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX)))
    {
        bb_rel.min.x = bb_rel.max.x = window.ContentSize.x + window.WindowPadding.x;
        if (move_flags & ImGuiNavMoveFlags_WrapX)
        {
            bb_rel.TranslateY(-bb_rel.get_height()); // Previous row
            clip_dir = Direction::Up;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == Direction::Right && (move_flags & (ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX)))
    {
        bb_rel.min.x = bb_rel.max.x = -window.WindowPadding.x;
        if (move_flags & ImGuiNavMoveFlags_WrapX)
        {
            bb_rel.TranslateY(+bb_rel.get_height()); // Next row
            clip_dir = Direction::Down;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == Direction::Up && (move_flags & (ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY)))
    {
        bb_rel.min.y = bb_rel.max.y = window.ContentSize.y + window.WindowPadding.y;
        if (move_flags & ImGuiNavMoveFlags_WrapY)
        {
            bb_rel.TranslateX(-bb_rel.get_width()); // Previous column
            clip_dir = Direction::Left;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == Direction::Down && (move_flags & (ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY)))
    {
        bb_rel.min.y = bb_rel.max.y = -window.WindowPadding.y;
        if (move_flags & ImGuiNavMoveFlags_WrapY)
        {
            bb_rel.TranslateX(+bb_rel.get_width()); // Next column
            clip_dir = Direction::Right;
        }
        do_forward = true;
    }
    if (!do_forward)
        return;
    window.NavRectRel[g.NavLayer] = bb_rel;
    NavMoveRequestForward(g.NavMoveDir, clip_dir, move_flags, g.NavMoveScrollFlags);
}

// static int FindWindowFocusIndex(ImGuiWindow* window)
pub fn find_window_focus_index(g: &mut Context, window: &mut Window) -> i32
{
    // ImGuiContext& g = *GImGui;
    IM_UNUSED(g);
    int order = window.focus_order;
    // IM_ASSERT(window.root_window == window); // No child window (not testing _ChildWindow because of docking)
    // IM_ASSERT(g.windows_focus_order[order] == window);
    return order;
}

// static ImGuiWindow* FindWindowNavFocusable(int i_start, int i_stop, int dir) // FIXME-OPT O(N)
pub fn find_window_nav_focusable(g: &mut Context, i_start: i32, i_stop: i32, dir: i32) -> &mut Window
{
    // ImGuiContext& g = *GImGui;
    for (int i = i_start; i >= 0 && i < g.windows_focus_order.size && i != i_stop; i += dir)
        if (IsWindowNavFocusable(g.windows_focus_order[i]))
            return g.windows_focus_order[i];
    return None;
}

// static void NavUpdateWindowingHighlightWindow(int focus_change_dir)
pub fn nav_update_windowing_highlight_window(g: &mut Context, focus_change_dir: i32)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.nav_windowing_target);
    if (g.nav_windowing_target.flags & WindowFlags::Modal)
        return;

    let i_current = FindWindowFocusIndex(g.nav_windowing_target);
    ImGuiWindow* window_target = FindWindowNavFocusable(i_current + focus_change_dir, -INT_MAX, focus_change_dir);
    if (!window_target)
        window_target = FindWindowNavFocusable((focus_change_dir < 0) ? (g.windows_focus_order.size - 1) : 0, i_current, focus_change_dir);
    if (window_target) // Don't reset windowing target if there's a single window in the list
        g.nav_windowing_target = g.NavWindowingTargetAnim = window_target;
    g.NavWindowingToggleLayer = false;
}

// Windowing management mode
// Keyboard: CTRL+Tab (change focus/move/resize), Alt (toggle menu layer)
// Gamepad:  Hold Menu/Square (change focus/move/resize), Tap Menu/Square (toggle menu layer)
// static void NavUpdateWindowing()
pub fn nav_update_windowing(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;

    ImGuiWindow* apply_focus_window = None;
    bool apply_toggle_layer = false;

    ImGuiWindow* modal_window = get_top_most_popup_modal();
    bool allow_windowing = (modal_window == None);
    if (!allow_windowing)
        g.nav_windowing_target = None;

    // Fade out
    if (g.NavWindowingTargetAnim && g.nav_windowing_target == None)
    {
        g.nav_windowing_highlight_alpha = ImMax(g.nav_windowing_highlight_alpha - io.delta_time * 10.0, 0.0);
        if (g.dim_bg_ration <= 0.0 && g.nav_windowing_highlight_alpha <= 0.0)
            g.NavWindowingTargetAnim = None;
    }

    // Start CTRL+Tab or Square+L/R window selection
    const bool start_windowing_with_gamepad = allow_windowing && !g.nav_windowing_target && IsNavInputTest(ImGuiNavInput_Menu, NavReadMode::Pressed);
    const bool start_windowing_with_keyboard = allow_windowing && !g.nav_windowing_target && io.key_ctrl && IsKeyPressed(ImGuiKey_Tab);
    if (start_windowing_with_gamepad || start_windowing_with_keyboard)
        if (ImGuiWindow* window = g.nav_window ? g.nav_window : FindWindowNavFocusable(g.windows_focus_order.size - 1, -INT_MAX, -1))
        {
            g.nav_windowing_target = g.NavWindowingTargetAnim = window.root_window;
            g.NavWindowingTimer = g.nav_windowing_highlight_alpha = 0.0;
            g.NavWindowingToggleLayer = start_windowing_with_gamepad ? true : false; // Gamepad starts toggling layer
            g.nav_input_source = start_windowing_with_keyboard ? InputSource::Keyboard : InputSource::Gamepad;
        }

    // Gamepad update
    g.NavWindowingTimer += io.delta_time;
    if (g.nav_windowing_target && g.nav_input_source == InputSource::Gamepad)
    {
        // Highlight only appears after a brief time holding the button, so that a fast tap on PadMenu (to toggle nav_layer) doesn't add visual noise
        g.nav_windowing_highlight_alpha = ImMax(g.nav_windowing_highlight_alpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05));

        // Select window to focus
        let focus_change_dir = IsNavInputTest(ImGuiNavInput_FocusPrev, NavReadMode::RepeatSlow) - IsNavInputTest(ImGuiNavInput_FocusNext, NavReadMode::RepeatSlow);
        if (focus_change_dir != 0)
        {
            NavUpdateWindowingHighlightWindow(focus_change_dir);
            g.nav_windowing_highlight_alpha = 1.0;
        }

        // Single press toggles nav_layer, long press with L/R apply actual focus on release (until then the window was merely rendered top-most)
        if (!IsNavInputDown(ImGuiNavInput_Menu))
        {
            g.NavWindowingToggleLayer &= (g.nav_windowing_highlight_alpha < 1.0); // Once button was held long enough we don't consider it a tap-to-toggle-layer press anymore.
            if (g.NavWindowingToggleLayer && g.nav_window)
                apply_toggle_layer = true;
            else if (!g.NavWindowingToggleLayer)
                apply_focus_window = g.nav_windowing_target;
            g.nav_windowing_target = None;
        }
    }

    // Keyboard: Focus
    if (g.nav_windowing_target && g.nav_input_source == InputSource::Keyboard)
    {
        // Visuals only appears after a brief time after pressing TAB the first time, so that a fast CTRL+TAB doesn't add visual noise
        g.nav_windowing_highlight_alpha = ImMax(g.nav_windowing_highlight_alpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05)); // 1.0
        if (IsKeyPressed(ImGuiKey_Tab, true))
            NavUpdateWindowingHighlightWindow(io.key_shift ? +1 : -1);
        if (!io.key_ctrl)
            apply_focus_window = g.nav_windowing_target;
    }

    // Keyboard: Press and Release ALT to toggle menu layer
    // - Testing that only Alt is tested prevents Alt+Shift or AltGR from toggling menu layer.
    // - AltGR is normally Alt+Ctrl but we can't reliably detect it (not all backends/systems/layout emit it as Alt+Ctrl). But even on keyboards without AltGR we don't want Alt+Ctrl to open menu anyway.
	const bool nav_keyboard_active = (io.config_flags & ConfigFlags::NavEnableKeyboard) != 0;
    if (nav_keyboard_active && IsKeyPressed(Key::ModAlt))
    {
        g.NavWindowingToggleLayer = true;
        g.nav_input_source = InputSource::Keyboard;
    }
    if (g.NavWindowingToggleLayer && g.nav_input_source == InputSource::Keyboard)
    {
        // We cancel toggling nav layer when any text has been typed (generally while holding Alt). (See #370)
        // We cancel toggling nav layer when other modifiers are pressed. (See #4439)
        if (io.input_queue_characters.size > 0 || io.key_ctrl || io.key_shift || io.key_super)
            g.NavWindowingToggleLayer = false;

        // Apply layer toggle on release
        // Important: as before version <18314 we lacked an explicit io event for focus gain/loss, we also compare mouse validity to detect old backends clearing mouse pos on focus loss.
        if (IsKeyReleased(Key::ModAlt) && g.NavWindowingToggleLayer)
            if (g.active_id == 0 || g.ActiveIdAllowOverlap)
                if (is_mouse_pos_valid(&io.mouse_pos) == is_mouse_pos_valid(&io.mouse_pos_prev))
                    apply_toggle_layer = true;
        if (!IsKeyDown(Key::ModAlt))
            g.NavWindowingToggleLayer = false;
    }

    // Move window
    if (g.nav_windowing_target && !(g.nav_windowing_target.flags & WindowFlags::NoMove))
    {
        Vector2D move_delta;
        if (g.nav_input_source == InputSource::Keyboard && !io.key_shift)
            move_delta = get_nav_input_amount_2d(NavDirSourceFlags::RawKeyboard, NavReadMode::Down);
        if (g.nav_input_source == InputSource::Gamepad)
            move_delta = get_nav_input_amount_2d(NavDirSourceFlags::PadLStick, NavReadMode::Down);
        if (move_delta.x != 0.0 || move_delta.y != 0.0)
        {
            let NAV_MOVE_SPEED = 800.0;
            let move_speed = f32::floor(NAV_MOVE_SPEED * io.delta_time * ImMin(io.display_frame_buffer_scale.x, io.display_frame_buffer_scale.y)); // FIXME: Doesn't handle variable framerate very well
            ImGuiWindow* moving_window = g.nav_windowing_target.root_window_dock_tree;
            set_window_pos(moving_window, moving_window.pos + move_delta * move_speed, Cond::Always);
            g.nav_disable_mouse_hover = true;
        }
    }

    // Apply final focus
    if (apply_focus_window && (g.nav_window == None || apply_focus_window != g.nav_window.root_window))
    {
        ImGuiViewport* previous_viewport = g.nav_window ? g.nav_window.viewport : None;
        clear_active_id();
        NavRestoreHighlightAfterMove();
        apply_focus_window = NavRestoreLastChildNavWindow(apply_focus_window);
        close_popups_over_window(apply_focus_window, false);
        focus_window(apply_focus_window);
        if (apply_focus_window.nav_last_ids[0] == 0)
            nav_init_window(apply_focus_window, false);

        // If the window has ONLY a menu layer (no main layer), select it directly
        // Use nav_layers_active_mask_next since windows didn't have a chance to be Begin()-ed on this frame,
        // so CTRL+Tab where the keys are only held for 1 frame will be able to use correct layers mask since
        // the target window as already been previewed once.
        // FIXME-NAV: This should be done in NavInit.. or in focus_window... However in both of those cases,
        // we won't have a guarantee that windows has been visible before and therefore nav_layers_active_mask*
        // won't be valid.
        if (apply_focus_window.dc.NavLayersActiveMaskNext == (1 << NavLayer::Menu))
            g.NavLayer = NavLayer::Menu;

        // Request OS level focus
        if (apply_focus_window.viewport != previous_viewport && g.platform_io.Platform_SetWindowFocus)
            g.platform_io.Platform_SetWindowFocus(apply_focus_window.viewport);
    }
    if (apply_focus_window)
        g.nav_windowing_target = None;

    // Apply menu/layer toggle
    if (apply_toggle_layer && g.nav_window)
    {
        clear_active_id();

        // Move to parent menu if necessary
        ImGuiWindow* new_nav_window = g.nav_window;
        while (new_nav_window.parent_window
            && (new_nav_window.dc.nav_layers_active_mask & (1 << NavLayer::Menu)) == 0
            && (new_nav_window.flags & WindowFlags::ChildWindow) != 0
            && (new_nav_window.flags & (WindowFlags::Popup | WindowFlags::ChildMenu)) == 0)
            new_nav_window = new_nav_window.parent_window;
        if (new_nav_window != g.nav_window)
        {
            ImGuiWindow* old_nav_window = g.nav_window;
            focus_window(new_nav_window);
            new_nav_window.NavLastChildNavWindow = old_nav_window;
        }

        // Toggle layer
        const ImGuiNavLayer new_nav_layer = (g.nav_window.DC.nav_layers_active_mask & (1 << NavLayer::Menu)) ? (ImGuiNavLayer)(g.NavLayer ^ 1) : NavLayer::Main;
        if (new_nav_layer != g.NavLayer)
        {
            // Reinitialize navigation when entering menu bar with the Alt key (FIXME: could be a properly of the layer?)
            const bool preserve_layer_1_nav_id = (new_nav_window.dock_node_as_host != None);
            if (new_nav_layer == NavLayer::Menu && !preserve_layer_1_nav_id)
                g.nav_window.nav_last_ids[new_nav_layer] = 0;
            NavRestoreLayer(new_nav_layer);
            NavRestoreHighlightAfterMove();
        }
    }
}

// window has already passed the IsWindowNavFocusable()
// static const char* GetFallbackWindowNameForWindowingList(ImGuiWindow* window)
pub fn get_fallback_window_name_for_windowing_list(g: &mut Context, window: &mut Window) -> String
{
    if (window.flags & WindowFlags::Popup)
        return "(Popup)";
    if ((window.flags & WindowFlags::MenuBar) && strcmp(window.name, "##MainMenuBar") == 0)
        return "(Main menu bar)";
    if (window.dock_node_as_host_id)
        return "(Dock node)";
    return "(Untitled)";
}

// Overlay displayed when using CTRL+TAB. Called by EndFrame().
// void NavUpdateWindowingOverlay()
pub fn nav_update_windowing_overlay(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.nav_windowing_target != None);

    if (g.NavWindowingTimer < NAV_WINDOWING_LIST_APPEAR_DELAY)
        return;

    if (g.nav_windowing_list_window == None)
        g.nav_windowing_list_window = find_window_by_name("###NavWindowingList");
    const ImGuiViewport* viewport = /*g.nav_window ? g.nav_window->viewport :*/ GetMainViewport();
    SetNextWindowSizeConstraints(Vector2D::new(viewport.size.x * 0.20, viewport.size.y * 0.20), Vector2D::new(f32::MAX, f32::MAX));
    set_next_window_pos(viewport.get_center(), Cond::Always, Vector2D::new(0.5, 0.5));
    push_style_var(StyleVar::WindowPadding, g.style.WindowPadding * 2.0);
    begin("###NavWindowingList", None, WindowFlags::NoTitleBar | WindowFlags::NoFocusOnAppearing | WindowFlags::NoResize | WindowFlags::NoMove | WindowFlags::NoInputs | WindowFlags::AlwaysAutoResize | WindowFlags::NoSavedSettings);
    for (int n = g.windows_focus_order.size - 1; n >= 0; n--)
    {
        ImGuiWindow* window = g.windows_focus_order[n];
        // IM_ASSERT(window != None); // Fix static analyzers
        if (!IsWindowNavFocusable(window))
            continue;
        const char* label = window.name;
        if (label == FindRenderedTextEnd(label))
            label = GetFallbackWindowNameForWindowingList(window);
        selectable(label, g.nav_windowing_target == window);
    }
    end();
    pop_style_var();
}
