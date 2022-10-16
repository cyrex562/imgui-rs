use std::ptr::{null, null_mut};
use libc::{c_char, c_float, c_int, INT_MAX};
use crate::activate_flags::{ImGuiActivateFlags_None, ImGuiActivateFlags_PreferInput, ImGuiActivateFlags_PreferTweak, ImGuiActivateFlags_TryToPreserveState};
use crate::config_flags::{ImGuiConfigFlags_NavEnableGamepad, ImGuiConfigFlags_NavEnableKeyboard, ImGuiConfigFlags_NavEnableSetMousePos};
use crate::constants::{NAV_WINDOWING_HIGHLIGHT_DELAY, NAV_WINDOWING_LIST_APPEAR_DELAY};
use crate::direction::{ImGuiDir, ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_None, ImGuiDir_Right, ImGuiDir_Up};
use crate::{GImGui, ImGuiViewport};
use crate::axis::{ImGuiAxis, ImGuiAxis_X};
use crate::backend_flags::{ImGuiBackendFlags_HasGamepad, ImGuiBackendFlags_HasSetMousePos};
use crate::color::IM_COL32;
use crate::condition::ImGuiCond_Always;
use crate::draw_list::ImDrawList;
use crate::draw_list_ops::GetForegroundDrawList;
use crate::id_ops::ClearActiveID;
use crate::imgui_cpp::{NavProcessItemForTabbingRequest, SetNavWindow};
use crate::input_flags::{ImGuiInputFlags_Repeat, ImGuiInputFlags_RepeatRateNavMove, ImGuiInputFlags_RepeatRateNavTweak};
use crate::input_ops::{GetKeyPressedAmount, GetTypematicRepeatRate, IsKeyDown, IsKeyPressed, IsKeyPressedEx, IsMouseHoveringRect, IsMousePosValid};
use crate::input_source::{ImGuiInputSource_Gamepad, ImGuiInputSource_Keyboard, ImGuiInputSource_Nav};
use crate::io::ImGuiIO;
use crate::item_flags::{ImGuiItemFlags, ImGuiItemFlags_Disabled, ImGuiItemFlags_Inputable, ImGuiItemFlags_NoNav, ImGuiItemFlags_NoNavDefaultFocus, ImGuiItemFlags_NoTabStop};
use crate::key::{ImGuiKey, ImGuiKey_C, ImGuiKey_DownArrow, ImGuiKey_End, ImGuiKey_Enter, ImGuiKey_Escape, ImGuiKey_GamepadDpadDown, ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadUp, ImGuiKey_GamepadFaceDown, ImGuiKey_GamepadFaceLeft, ImGuiKey_GamepadFaceRight, ImGuiKey_GamepadFaceUp, ImGuiKey_GamepadL1, ImGuiKey_GamepadLStickDown, ImGuiKey_GamepadLStickLeft, ImGuiKey_GamepadLStickRight, ImGuiKey_GamepadLStickUp, ImGuiKey_GamepadR1, ImGuiKey_Home, ImGuiKey_LeftArrow, ImGuiKey_ModAlt, ImGuiKey_NavGamepadActivate, ImGuiKey_NavGamepadCancel, ImGuiKey_NavGamepadInput, ImGuiKey_NavGamepadMenu, ImGuiKey_NavGamepadTweakFast, ImGuiKey_NavGamepadTweakSlow, ImGuiKey_None, ImGuiKey_PageDown, ImGuiKey_PageUp, ImGuiKey_RightArrow, ImGuiKey_Space, ImGuiKey_Tab, ImGuiKey_UpArrow};
use crate::math_ops::{ImClamp, ImFabs, ImLerp, ImMax, ImMin};
use crate::nav_item_data::ImGuiNavItemData;
use crate::nav_layer::{ImGuiNavLayer, ImGuiNavLayer_Main, ImGuiNavLayer_Menu};
use crate::nav_move_flags::{ImGuiNavMoveFlags, ImGuiNavMoveFlags_Activate, ImGuiNavMoveFlags_AllowCurrentNavId, ImGuiNavMoveFlags_AlsoScoreVisibleSet, ImGuiNavMoveFlags_DebugNoResult, ImGuiNavMoveFlags_DontSetNavHighlight, ImGuiNavMoveFlags_FocusApi, ImGuiNavMoveFlags_Forwarded, ImGuiNavMoveFlags_LoopX, ImGuiNavMoveFlags_LoopY, ImGuiNavMoveFlags_None, ImGuiNavMoveFlags_ScrollToEdgeY, ImGuiNavMoveFlags_Tabbing, ImGuiNavMoveFlags_WrapX, ImGuiNavMoveFlags_WrapY};
use crate::popup_ops::{ClosePopupsOverWindow, ClosePopupToLevel, GetTopMostPopupModal};
use crate::popup_position_policy::ImGuiPopupPositionPolicy;
use crate::rect::ImRect;
use crate::render_ops::FindRenderedTextEnd;
use crate::scroll_flags::{ImGuiScrollFlags, ImGuiScrollFlags_AlwaysCenterY, ImGuiScrollFlags_KeepVisibleEdgeX, ImGuiScrollFlags_KeepVisibleEdgeY, ImGuiScrollFlags_None};
use crate::scrolling_ops::{CalcNextScrollFromScrollTargetAndClamp, ScrollToRectEx, SetScrollX, SetScrollY};
use crate::string_ops::{ImFormatString, str_to_const_c_char_ptr};
use crate::text_ops::CalcTextSize;
use crate::type_defs::ImGuiID;
use crate::utils::{flag_clear, flag_set, is_not_null, is_null};
use crate::vec2::ImVec2;
use crate::window::find::FindWindowByName;
use crate::window::focus::FocusWindow;
use crate::window::ImGuiWindow;
use crate::window::ops::{Begin, End};
use crate::window::props::{IsWindowNavFocusable, SetNextWindowPos, SetNextWindowSizeConstraints, SetWindowPos};
use crate::window::rect::{WindowRectAbsToRel, WindowRectRelToAbs};
use crate::window::window_flags::{ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoFocusOnAppearing, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoNavInputs, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup};

// We get there when either NavId == id, or when g.NavAnyRequest is set (which is updated by NavUpdateAnyRequestFlag above)
// This is called after LastItemData is set.
pub unsafe fn NavProcessItem() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut id: ImGuiID = g.LastItemData.ID;
    let nav_bb: ImRect = g.LastItemData.NavRect;
    let mut item_flags: ImGuiItemFlags = g.LastItemData.InFlags;

    // Process Init Request
    if g.NavInitRequest && g.NavLayer == window.DC.NavLayerCurrent && (item_flags & ImGuiItemFlags_Disabled) == 0 {
        // Even if 'ImGuiItemFlags_NoNavDefaultFocus' is on (typically collapse/close button) we record the first ResultId so they can be used as a fallback
        let candidate_for_nav_default_focus: bool = (item_flags & ImGuiItemFlags_NoNavDefaultFocus) == 0;
        if candidate_for_nav_default_focus || g.NavInitResultId == 0 {
            g.NavInitResultId = id;
            g.NavInitResultRectRel = WindowRectAbsToRel(window, &nav_bb);
        }
        if candidate_for_nav_default_focus {
            g.NavInitRequest = false; // Found a match, clear request
            NavUpdateAnyRequestFlag();
        }
    }

    // Process Move Request (scoring for navigation)
    // FIXME-NAV: Consider policy for double scoring (scoring from NavScoringRect + scoring from a rect wrapped according to current wrapping policy)
    if (g.NavMoveScoringItems) {
        let is_tab_stop: bool = flag_set(item_flags, ImGuiItemFlags_Inputable) && flag_clear(item_flags, (ImGuiItemFlags_NoTabStop | ImGuiItemFlags_Disabled));
        let is_tabbing: bool = (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) != 0;
        if (is_tabbing) {
            if (is_tab_stop || flag_set(g.NavMoveFlags, ImGuiNavMoveFlags_FocusApi)) {
                NavProcessItemForTabbingRequest(id);
            }
        } else if ((g.NavId != id || flag_set(g.NavMoveFlags, ImGuiNavMoveFlags_AllowCurrentNavId)) && flag_clear(item_flags, (ImGuiItemFlags_Disabled | ImGuiItemFlags_NoNav))) {
            ImGuiNavItemData * result = if (window == g.NavWindow) { &g.NavMoveResultLocal } else { &g.NavMoveResultOther };
            if (!is_tabbing) {
                if (NavScoreItem(result)) {
                    NavApplyItemToResult(result);
                }

                // Features like PageUp/PageDown need to maintain a separate score for the visible set of items.
                let VISIBLE_RATIO: c_float = 0.70f32;
                if (g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet) && window.ClipRect.Overlaps(nav_bb) {
                    if ImClamp(nav_bb.Max.y, window.ClipRect.Min.y, window.ClipRect.Max.y) - ImClamp(nav_bb.Min.y, window.ClipRect.Min.y, window.ClipRect.Max.y) >= (nav_bb.Max.y - nav_bb.Min.y) * VISIBLE_RATIO {
                        if NavScoreItem(&mut g.NavMoveResultLocalVisible) {
                            NavApplyItemToResult(&mut g.NavMoveResultLocalVisible);
                        }
                    }
                }
            }
        }
    }

    // Update window-relative bounding box of navigated item
    if g.NavId == id {
        if g.NavWindow != window {
            SetNavWindow(window);
        }// Always refresh g.NavWindow, because some operations such as FocusItem() may not have a window.
        g.NavLayer = window.DC.NavLayerCurrent;
        g.NavFocusScopeId = window.DC.NavFocusScopeIdCurrent;
        g.NavIdIsAlive = true;
        window.NavRectRel[window.DC.NavLayerCurrent] = WindowRectAbsToRel(window, &nav_bb);    // Store item bounding box (relative to window position)
    }
}



// FIXME-NAV: The existence of SetNavID vs SetFocusID vs FocusWindow() needs to be clarified/reworked.
// In our terminology those should be interchangeable, yet right now this is super confusing.
// Those two functions are merely a legacy artifact, so at minimum naming should be clarified.

pub unsafe fn SetNavWindow(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.NavWindow != window
    {
        // IMGUI_DEBUG_LOG_FOCUS("[focus] SetNavWindow(\"%s\")\n", window ? window.Name : "<NULL>");
        g.NavWindow = window;
    }
    g.NavInitRequest = false;g.NavMoveSubmitted = false;g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

pub unsafe fn SetNavID(id: ImGuiID, nav_layer: ImGuiNavLayer, focus_scope_id: ImGuiID, rect_rel: &ImRect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindow != NULL);
    // IM_ASSERT(nav_layer == ImGuiNavLayer_Main || nav_layer == ImGuiNavLayer_Menu);
    g.NavId = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = focus_scope_id;
    g.NavWindow.NavLastIds[nav_layer] = id;
    g.NavWindow.NavRectRel[nav_layer] = rect_rel;
}

pub unsafe fn SetFocusID(id: ImGuiID, window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(id != 0);

    if (g.NavWindow != window) {
        SetNavWindow(window);
    }

    // Assume that SetFocusID() is called in the context where its window.DC.NavLayerCurrent and window.DC.NavFocusScopeIdCurrent are valid.
    // Note that window may be != g.CurrentWindow (e.g. SetFocusID call in InputTextEx for multi-line text)
    const nav_layer: ImGuiNavLayer = window.DC.NavLayerCurrent;
    g.NavId = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = window.DC.NavFocusScopeIdCurrent;
    window.NavLastIds[nav_layer] = id;
    if (g.LastItemData.ID == id) {
        window.NavRectRel[nav_layer] = WindowRectAbsToRel(window, &g.LastItemData.NavRect);
    }

    if (g.ActiveIdSource == ImGuiInputSource_Nav) {
        g.NavDisableMouseHover = true;
    }
    else {
        g.NavDisableHighlight = true;
    }
}

pub unsafe fn ImGetDirQuadrantFromDelta(dx: c_float,dy: c_float) -> ImGuiDir
{
    if ImFabs(dx) > ImFabs(dy) {
        return if dx > 0.0 {
            ImGuiDir_Right
        } else { ImGuiDir_Left };
    }
    return if dy > 0.0 { ImGuiDir_Down } else { ImGuiDir_Up };
}

pub unsafe fn NavScoreItemDistInterval(a0: c_float,a1: c_float,b0: c_float,b1: c_float) -> c_float
{
    if (a1 < b0) {
        return a1 - b0;
    }
    if (b1 < a0) {
        return a0 - b1;
    }
    return 0.0;
}

pub unsafe fn NavClampRectToVisibleAreaForMoveDir(move_dir: ImGuiDir, r: &mut ImRect, clip_rect: &ImRect)
{
    if (move_dir == ImGuiDir_Left || move_dir == ImGuiDir_Right)
    {
        r.Min.y = ImClamp(r.Min.y, clip_rect.Min.y, clip_rect.Max.y);
        r.Max.y = ImClamp(r.Max.y, clip_rect.Min.y, clip_rect.Max.y);
    }
    else // FIXME: PageUp/PageDown are leaving move_dir == None
    {
        r.Min.x = ImClamp(r.Min.x, clip_rect.Min.x, clip_rect.Max.x);
        r.Max.x = ImClamp(r.Max.x, clip_rect.Min.x, clip_rect.Max.x);
    }
}

// Scoring function for gamepad/keyboard directional navigation. Based on https://gist.github.com/rygorous/6981057
pub unsafe fn NavScoreItem(result: *mut ImGuiNavItemData) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (g.NavLayer != window.DC.NavLayerCurrent) {
        return false;
    }

    // FIXME: Those are not good variables names
    let mut cand: ImRect =  g.LastItemData.NavRect;   // Current item nav rectangle
    let curr: ImRect =  g.NavScoringRect;   // Current modified source rect (NB: we've applied Max.x = Min.x in NavUpdate() to inhibit the effect of having varied item width)
    g.NavScoringDebugCount+= 1;

    // When entering through a NavFlattened border, we consider child window items as fully clipped for scoring
    if (window.ParentWindow == g.NavWindow)
    {
        // IM_ASSERT((window.Flags | g.Navwindow.Flags) & ImGuiWindowFlags_NavFlattened);
        if (!window.ClipRect.Overlaps(cand)) {
            return false;
        }
        cand.ClipWithFull(&ImRect::from_vec4(&window.ClipRect)); // This allows the scored item to not overlap other candidates in the parent window
    }

    // We perform scoring on items bounding box clipped by the current clipping rectangle on the other axis (clipping on our movement axis would give us equal scores for all clipped items)
    // For example, this ensure that items in one column are not reached when moving vertically from items in another column.
    NavClampRectToVisibleAreaForMoveDir(g.NavMoveClipDir, &mut cand, &ImRect::from_vec4(&window.ClipRect));

    // Compute distance between boxes
    // FIXME-NAV: Introducing biases for vertical navigation, needs to be removed.
    let mut dbx: c_float =  NavScoreItemDistInterval(cand.Min.x, cand.Max.x, curr.Min.x, curr.Max.x);
    let dby: c_float =  NavScoreItemDistInterval(ImLerp(cand.Min.y, cand.Max.y, 0.20f32), ImLerp(cand.Min.y, cand.Max.y, 0.80f32), ImLerp(curr.Min.y, curr.Max.y, 0.20f32), ImLerp(curr.Min.y, curr.Max.y, 0.80f32)); // Scale down on Y to keep using box-distance for vertically touching items
    if dby != 0.0 && dbx != 0.0 {
        dbx = (dbx / 1000f32) + (if dbx > 0.0 {
            1.0
        }else { -1.0 });
    }
    let dist_box: c_float =  ImFabs(dbx) + ImFabs(dby);

    // Compute distance between centers (this is off by a factor of 2, but we only compare center distances with each other so it doesn't matter)
    let dcx: c_float =  (cand.Min.x + cand.Max.x) - (curr.Min.x + curr.Max.x);
    let dcy: c_float =  (cand.Min.y + cand.Max.y) - (curr.Min.y + curr.Max.y);
    let dist_center: c_float =  ImFabs(dcx) + ImFabs(dcy); // L1 metric (need this for our connectedness guarantee)

    // Determine which quadrant of 'curr' our candidate item 'cand' lies in based on distance
    quadrant: ImGuiDir;
    let mut dax: c_float =  0.0;
    let mut day = 0.0;
    let mut dist_axial = 0.0;
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
        quadrant = if g.LastItemData.ID < g.NavId { ImGuiDir_Left } else { ImGuiDir_Right };
    }

// #if IMGUI_DEBUG_NAV_SCORING
    let mut buf: [c_char;128] = [0;128];
    if (IsMouseHoveringRect(&cand.Min, &cand.Max, false))
    {
        // ImFormatString(buf, buf.len(), "dbox (%.2f,%.2f->%.40f32)\ndcen (%.2f,%.2f->%.40f32)\nd (%.2f,%.2f->%.40f32)\nnav %c, quadrant %c", dbx, dby, dist_box, dcx, dcy, dist_center, dax, day, dist_axial, "WENS"[g.NavMoveDir], "WENS"[quadrant]);
        let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(window.Viewport);
        draw_list.AddRect(&curr.Min, &curr.Max, IM_COL32(255, 200, 0, 100), 0.0, 0, 0.0);
        draw_list.AddRect(&cand.Min, &cand.Max, IM_COL32(255, 255, 0, 200), 0.0, 0, 0.0);
        draw_list.AddRectFilled(cand.Max - ImVec2::new(4.0, 4.0), cand.Max + CalcTextSize(buf.as_ptr(), null(), false, 0.0) + ImVec2::new(4.0, 4.0), IM_COL32(40, 0, 0, 150), 0.0, 0);
        draw_list.AddText(&cand.Max, !0, buf.as_ptr(), null_mut());
    }
    else if (g.IO.KeyCtrl) // Hold to preview score in matching quadrant. Press C to rotate.
    {
        if (quadrant == g.NavMoveDir)
        {
            // ImFormatString(buf, buf.len(), "%.0f/%.0f", dist_box, dist_center);
            let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(window.Viewport);
            draw_list.AddRectFilled(&cand.Min, &cand.Max, IM_COL32(255, 0, 0, 200), 0.0, 0);
            draw_list.AddText(&cand.Min, IM_COL32(255, 255, 255, 255), buf.as_ptr(), null_mut());
        }
    }
// #endif

    // Is it in the quadrant we're interesting in moving to?
    let mut new_best: bool =  false;
    const move_dir: ImGuiDir = g.NavMoveDir;
    if quadrant == move_dir
    {
        // Does it beat the current best candidate?
        if dist_box < result.DistBox
        {
            result.DistBox = dist_box;
            result.DistCenter = dist_center;
            return true;
        }
        if dist_box == result.DistBox
        {
            // Try using distance between center points to break ties
            if dist_center < result.DistCenter
            {
                result.DistCenter = dist_center;
                new_best = true;
            }
            else if dist_center == result.DistCenter
            {
                // Still tied! we need to be extra-careful to make sure everything gets linked properly. We consistently break ties by symbolically moving "later" items
                // (with higher index) to the right/downwards by an infinitesimal amount since we the current "best" button already (so it must have a lower index),
                // this is fairly easy. This rule ensures that all buttons with dx==dy==0 will end up being linked in order of appearance along the x axis.
                if (if move_dir == ImGuiDir_Up || move_dir == ImGuiDir_Down { dby } else { dbx }) < 0.0 { // moving bj to the right/down decreases distance
                    new_best = true;
                }
            }
        }
    }

    // Axial check: if 'curr' has no link at all in some direction and 'cand' lies roughly in that direction, add a tentative link. This will only be kept if no "real" matches
    // are found, so it only augments the graph produced by the above method using extra links. (important, since it doesn't guarantee strong connectedness)
    // This is just to avoid buttons having no links in a particular direction when there's a suitable neighbor. you get good graphs without this too.
    // 2017/09/29: FIXME: This now currently only enabled inside menu bars, ideally we'd disable it everywhere. Menus in particular need to catch failure. For general navigation it feels awkward.
    // Disabling it may lead to disconnected graphs when nodes are very spaced out on different axis. Perhaps consider offering this as an option?
    if result.DistBox == f32::MAX && dist_axial < result.DistAxial { // Check axial match
        if g.NavLayer == ImGuiNavLayer_Menu && flag_clear(g.NavWindow.Flags, ImGuiWindowFlags_ChildMenu) {
            if (move_dir == ImGuiDir_Left && dax < 0.0) || (move_dir == ImGuiDir_Right && dax > 0.0) || (move_dir == ImGuiDir_Up && day < 0.0) || (move_dir == ImGuiDir_Down && day > 0.0) {
                result.DistAxial = dist_axial;
                new_best = true;
            }
        }
    }

    return new_best;
}

pub unsafe fn NavApplyItemToResult(result: *mut ImGuiNavItemData)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    result.Window = window;
    result.ID = g.LastItemData.ID;
    result.FocusScopeId = window.DC.NavFocusScopeIdCurrent;
    result.InFlags = g.LastItemData.InFlags;
    result.RectRel = WindowRectAbsToRel(window, &g.LastItemData.NavRect);
}

// Handle "scoring" of an item for a tabbing/focusing request initiated by NavUpdateCreateTabbingRequest().
// Note that SetKeyboardFocusHere() API calls are considered tabbing requests!
// - Case 1: no nav/active id:    set result to first eligible item, stop storing.
// - Case 2: tab forward:         on ref id set counter, on counter elapse store result
// - Case 3: tab forward wrap:    set result to first eligible item (preemptively), on ref id set counter, on next frame if counter hasn't elapsed store result. // FIXME-TABBING: Could be done as a next-frame forwarded request
// - Case 4: tab backward:        store all results, on ref id pick prev, stop storing
// - Case 5: tab backward wrap:   store all results, on ref id if no result keep storing until last // FIXME-TABBING: Could be done as next-frame forwarded requested
pub unsafe fn NavProcessItemForTabbingRequest(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Always store in NavMoveResultLocal (unlike directional request which uses NavMoveResultOther on sibling/flattened windows)
    result: *mut ImGuiNavItemData = &mut g.NavMoveResultLocal;
    if (g.NavTabbingDir == 1)
    {
        // Tab Forward or SetKeyboardFocusHere() with >= 0
        if (g.NavTabbingResultFirst.ID == 0) {
            NavApplyItemToResult(&mut g.NavTabbingResultFirst);
        }
        if (--g.NavTabbingCounter == 0) {
            NavMoveRequestResolveWithLastItem(result);
        }
        else if (g.NavId == id) {
            g.NavTabbingCounter = 1;
        }
    }
    else if (g.NavTabbingDir == -1)
    {
        // Tab Backward
        if (g.NavId == id)
        {
            if (result.ID)
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
        if (g.NavTabbingResultFirst.ID == 0) {
            NavMoveRequestResolveWithLastItem(&mut g.NavTabbingResultFirst);
        }
    }
}

pub unsafe fn NavMoveRequestButNoResultYet() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.NavMoveScoringItems && g.NavMoveResultLocal.ID == 0 && g.NavMoveResultOther.ID == 0;
}

// FIXME: ScoringRect is not set
pub unsafe fn NavMoveRequestSubmit(move_dir: ImGuiDir, clip_dir: ImGuiDir, mut move_flags: ImGuiNavMoveFlags, scroll_flags: ImGuiScrollFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindow != NULL);

    if flag_set(move_flags, ImGuiNavMoveFlags_Tabbing) {
        move_flags |= ImGuiNavMoveFlags_AllowCurrentNavId;
    }

    g.NavMoveSubmitted = true;
    g.NavMoveScoringItems = true;
    g.NavMoveDir = move_dir;
    g.NavMoveDirForDebug = move_dir;
    g.NavMoveClipDir = clip_dir;
    g.NavMoveFlags = move_flags;
    g.NavMoveScrollFlags = scroll_flags;
    g.NavMoveForwardToNextFrame = false;
    g.NavMoveKeyMods = g.IO.KeyMods;
    g.NavMoveResultLocal.Clear();
    g.NavMoveResultLocalVisible.Clear();
    g.NavMoveResultOther.Clear();
    g.NavTabbingCounter = 0;
    g.NavTabbingResultFirst.Clear();
    NavUpdateAnyRequestFlag();
}

pub unsafe fn NavMoveRequestResolveWithLastItem(result: *mut ImGuiNavItemData)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavMoveScoringItems = false; // Ensure request doesn't need more processing
    NavApplyItemToResult(result);
    NavUpdateAnyRequestFlag();
}

pub unsafe fn NavMoveRequestCancel()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavMoveSubmitted = false;
    g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

// Forward will reuse the move request again on the next frame (generally with modifications done to it)
pub unsafe fn NavMoveRequestForward(move_dir: ImGuiDir, clip_dir: ImGuiDir, move_flags: ImGuiNavMoveFlags, scroll_flags: ImGuiScrollFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
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
pub unsafe fn NavMoveRequestTryWrapping(window: *mut ImGuiWindow, wrap_flags: ImGuiNavMoveFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(wrap_flags != 0); // Call with _WrapX, _WrapY, _LoopX, _LoopY
    // In theory we should test for NavMoveRequestButNoResultYet() but there's no point doing it, NavEndFrame() will do the same test
    if (g.NavWindow == window && g.NavMoveScoringItems && g.NavLayer == ImGuiNavLayer_Main) {
        g.NavMoveFlags |= wrap_flags;
    }
}

// FIXME: This could be replaced by updating a frame number in each window when (window == NavWindow) and (NavLayer == 0).
// This way we could find the last focused window among our children. It would be much less confusing this way?
pub unsafe fn NavSaveLastChildNavWindowIntoParent(nav_window: *mut ImGuiWindow)
{
    let mut parent: *mut ImGuiWindow =  nav_window;
    while (is_not_null(parent) && parent.RootWindow != parent && flag_clear(parent.Flags, (ImGuiWindowFlags_Popup | ImGuiWindowFlags_ChildMenu))) {
        parent = parent.ParentWindow;
    }
    if is_not_null(parent) && parent != nav_window {
        parent.NavLastChildNavWindow = nav_window;
    }
}

// Restore the last focused child.
// Call when we are expected to land on the Main Layer (0) after FocusWindow()
pub unsafe fn NavRestoreLastChildNavWindow(window: *mut ImGuiWindow) -> *mut ImGuiWindow
{
    if window.NavLastChildNavWindow && window.NavLastChildNavwindow.WasActive {
        return window.NavLastChildNavWindow;
    }
    if is_not_null(window.DockNodeAsHost) && is_not_null(window.DockNodeAsHost.TabBar)
    {
        let tab = TabBarFindMostRecentlySelectedTabForActiveWindow(window.DockNodeAsHost.TabBar);
        if is_not_null(tab) {
            return tab.Window;
        }
    }
    return window;
}

pub unsafe fn NavRestoreLayer(layer: ImGuiNavLayer)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (layer == ImGuiNavLayer_Main)
    {
        let mut prev_nav_window: *mut ImGuiWindow =  g.NavWindow;
        g.NavWindow = NavRestoreLastChildNavWindow(g.NavWindow);    // FIXME-NAV: Should clear ongoing nav requests?
        if (prev_nav_window) {}
            // IMGUI_DEBUG_LOG_FOCUS("[focus] NavRestoreLayer: from \"%s\" to SetNavWindow(\"%s\")\n", prev_nav_window.Name, g.NavWindow.Name);
    }
    let mut window: *mut ImGuiWindow =  g.NavWindow;
    if (window.NavLastIds[layer] != 0)
    {
        SetNavID(window.NavLastIds[layer], layer, 0, window.NavRectRel[layer]);
    }
    else
    {
        g.NavLayer = layer;
        NavInitWindow(window, true);
    }
}

pub unsafe fn NavRestoreHighlightAfterMove()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavDisableHighlight = false;
    g.NavDisableMouseHover = true; g.NavMousePosDirty = true;
}

pub unsafe fn NavUpdateAnyRequestFlag()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavAnyRequest = g.NavMoveScoringItems || g.NavInitRequest || (IMGUI_DEBUG_NAV_SCORING && g.NavWindow != null_mut());
    if (g.NavAnyRequest) {}
        // IM_ASSERT(g.NavWindow != NULL);
}

// This needs to be called before we submit any widget (aka in or before Begin)
pub unsafe fn NavInitWindow(window: *mut ImGuiWindow, force_reinit: bool)
{
    // FIXME: ChildWindow test here is wrong for docking
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window == g.NavWindow);

    if flag_set(window.Flags, ImGuiWindowFlags_NoNavInputs)
    {
        g.NavId = 0;g.NavFocusScopeId = 0;
        return;
    }

    let mut init_for_nav: bool =  false;
    if (window == window.RootWindow || flag_set(window.Flags, ImGuiWindowFlags_Popup) || (window.NavLastIds[0] == 0) || force_reinit) {
        init_for_nav = true;
    }
    IMGUI_DEBUG_LOG_NAV("[nav] NavInitRequest: from NavInitWindow(), init_for_nav=%d, window=\"%s\", layer=%d\n", init_for_nav, window.Name, g.NavLayer);
    if (init_for_nav)
    {
        SetNavID(0, g.NavLayer, 0, ImRect());
        g.NavInitRequest = true;
        g.NavInitRequestFromMove = false;
        g.NavInitResultId = 0;
        g.NavInitResultRectRel = ImRect();
        NavUpdateAnyRequestFlag();
    }
    else
    {
        g.NavId = window.NavLastIds[0];
        g.NavFocusScopeId = 0;
    }
}

pub unsafe fn NavCalcPreferredRefPos() -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  g.NavWindow;
    if g.NavDisableHighlight || !g.NavDisableMouseHover || !is_not_null(window)
    {
        // Mouse (we need a fallback in case the mouse becomes invalid after being used)
        // The +1.0 offset when stored by OpenPopupEx() allows reopening this or another popup (same or another mouse button) while not moving the mouse, it is pretty standard.
        // In theory we could move that +1.0 offset in OpenPopupEx()
        let p: ImVec2 = if IsMousePosValid(&g.IO.MousePos) { g.IO.MousePos } else { g.MouseLastValidPos };
        return ImVec2::new(p.x + 1.0, p.y);
    }
    else
    {
        // When navigation is active and mouse is disabled, pick a position around the bottom left of the currently navigated item
        // Take account of upcoming scrolling (maybe set mouse pos should be done in EndFrame?)
        let mut rect_rel: ImRect =  WindowRectRelToAbs(window, window.NavRectRel[g.NavLayer]);
        if (window.LastFrameActive != g.FrameCount && (window.ScrollTarget.x != f32::MAX || window.ScrollTarget.y != f32::MAX))
        {
            let next_scroll: ImVec2 = CalcNextScrollFromScrollTargetAndClamp(window);
            rect_rel.Translate(window.Scroll - next_scroll);
        }
        let pos: ImVec2 = ImVec2::new(rect_rel.Min.x + ImMin(g.Style.FramePadding.x * 4, rect_rel.GetWidth()), rect_rel.Max.y - ImMin(g.Style.FramePadding.y, rect_rel.GetHeight()));
        let viewport = window.Viewport;
        return ImFloor(ImClamp(pos, viewport.Pos, viewport.Pos + viewport.Size)); // ImFloor() is important because non-integer mouse position application in backend might be lossy and result in undesirable non-zero delta.
    }
}

pub unsafe fn GetNavTweakPressedAmount(axis: ImGuiAxis) -> c_int
{
    let g = GImGui; // ImGuiContext& g = *GImGui;repeat_delay: c_float, repeat_rate;
    GetTypematicRepeatRate(ImGuiInputFlags_RepeatRateNavTweak, &mut repeat_delay, &mut repeat_rate);

    // ImGuiKey key_less, key_more;
    let mut key_less = ImGuiKey_None;
    let mut key_more = ImGuiKey_None;
    if g.NavInputSource == ImGuiInputSource_Gamepad
    {
        key_less = if axis == ImGuiAxis_X { ImGuiKey_GamepadDpadLeft } else { ImGuiKey_GamepadDpadUp };
        key_more = if axis == ImGuiAxis_X { ImGuiKey_GamepadDpadRight} else { ImGuiKey_GamepadDpadDown};
    }
    else
    {
        key_less = if axis == ImGuiAxis_X { ImGuiKey_LeftArrow} else { ImGuiKey_UpArrow};
        key_more = if axis == ImGuiAxis_X { ImGuiKey_RightArrow} else { ImGuiKey_DownArrow};
    }
    let mut amount =  GetKeyPressedAmount(key_more, repeat_delay, repeat_rate) - GetKeyPressedAmount(key_less, repeat_delay, repeat_rate);
    if amount != 0 && IsKeyDown(key_less) && IsKeyDown(key_more) { // Cancel when opposite directions are held, regardless of repeat phase
        amount = 0;
    }
    return amount;
}

pub unsafe fn NavUpdate()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut io = &mut g.IO;

    io.WantSetMousePos = false;
    //if (g.NavScoringDebugCount > 0) IMGUI_DEBUG_LOG_NAV("[nav] NavScoringDebugCount %d for '%s' layer %d (Init:%d, Move:%d)\n", g.NavScoringDebugCount, g.NavWindow ? g.Navwindow.Name : "NULL", g.NavLayer, g.NavInitRequest || g.NavInitResultId != 0, g.NavMoveRequest);

    // Set input source based on which keys are last pressed (as some features differs when used with Gamepad vs Keyboard)
    // FIXME-NAV: Now that keys are separated maybe we can get rid of NavInputSource?
    let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    let nav_gamepad_keys_to_change_source:[ImGuiKey;8] = [ ImGuiKey_GamepadFaceRight, ImGuiKey_GamepadFaceLeft, ImGuiKey_GamepadFaceUp, ImGuiKey_GamepadFaceDown, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadUp, ImGuiKey_GamepadDpadDown ];
    if nav_gamepad_active {
        // for (ImGuiKey key : nav_gamepad_keys_to_change_source)
        for key in nav_gamepad_keys_to_change_source
        {
            if IsKeyDown(key) {
                g.NavInputSource = ImGuiInputSource_Gamepad;
            }
        }
    }
    let nav_keyboard_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;
    let nav_keyboard_keys_to_change_source: [ImGuiKey;7] = [ ImGuiKey_Space, ImGuiKey_Enter, ImGuiKey_Escape, ImGuiKey_RightArrow, ImGuiKey_LeftArrow, ImGuiKey_UpArrow, ImGuiKey_DownArrow ];
    if nav_keyboard_active {
        // for (ImGuiKey key : nav_keyboard_keys_to_change_source)
        for key in nav_keyboard_keys_to_change_source
        {
            if IsKeyDown(key) {
                g.NavInputSource = ImGuiInputSource_Keyboard;
            }
        }
    }

    // Process navigation init request (select first/default focus)
    if g.NavInitResultId != 0 {
        NavInitRequestApplyResult();
    }
    g.NavInitRequest = false;
    g.NavInitRequestFromMove = false;
    g.NavInitResultId = 0;
    g.NavJustMovedToId = 0;

    // Process navigation move request
    if (g.NavMoveSubmitted) {
        NavMoveRequestApplyResult();
    }
    g.NavTabbingCounter = 0;
    g.NavMoveSubmitted = false;
    g.NavMoveScoringItems = false;

    // Schedule mouse position update (will be done at the bottom of this function, after 1) processing all move requests and 2) updating scrolling)
    let mut set_mouse_pos: bool =  false;
    if (g.NavMousePosDirty && g.NavIdIsAlive) {
        if (!g.NavDisableHighlight && g.NavDisableMouseHover && is_not_null(g.NavWindow)) {
            set_mouse_pos = true;
        }
    }
    g.NavMousePosDirty = false;
    // IM_ASSERT(g.NavLayer == ImGuiNavLayer_Main || g.NavLayer == ImGuiNavLayer_Menu);

    // Store our return window (for returning from Menu Layer to Main Layer) and clear it as soon as we step back in our own Layer 0
    if (g.NavWindow) {
        NavSaveLastChildNavWindowIntoParent(g.NavWindow);
    }
    if (is_not_null(g.NavWindow) && g.NavWindow.NavLastChildNavWindow != null_mut() && g.NavLayer == ImGuiNavLayer_Main) {
        g.NavWindow.NavLastChildNavWindow = null_mut();
    }

    // Update CTRL+TAB and Windowing features (hold Square to move/resize/etc.)
    NavUpdateWindowing();

    // Set output flags for user application
    io.NavActive = (nav_keyboard_active || nav_gamepad_active) && is_not_null(g.NavWindow) && flag_clear(g.NavWindow.Flags , ImGuiWindowFlags_NoNavInputs);
    io.NavVisible = (io.NavActive && g.NavId != 0 && !g.NavDisableHighlight) || (g.NavWindowingTarget != null_mut());

    // Process NavCancel input (to close a popup, get back to parent, clear focus)
    NavUpdateCancelRequest();

    // Process manual activation request
    g.NavActivateId = 0;
    g.NavActivateDownId = 0;
    g.NavActivatePressedId = 0;
    g.NavActivateInputId = 0;
    g.NavActivateFlags = ImGuiActivateFlags_None;
    if g.NavId != 0 && !g.NavDisableHighlight && is_null(g.NavWindowingTarget) && is_not_null(g.NavWindow) && flag_clear(g.NavWindow.Flags, ImGuiWindowFlags_NoNavInputs)
    {
        let activate_down: bool = (nav_keyboard_active && IsKeyDown(ImGuiKey_Space)) || (nav_gamepad_active && IsKeyDown(ImGuiKey_NavGamepadActivate));
        let activate_pressed: bool = activate_down && ((nav_keyboard_active && IsKeyPressed(ImGuiKey_Space, false)) || (nav_gamepad_active && IsKeyPressed(ImGuiKey_NavGamepadActivate, false)));
        let input_down: bool = (nav_keyboard_active && IsKeyDown(ImGuiKey_Enter)) || (nav_gamepad_active && IsKeyDown(ImGuiKey_NavGamepadInput));
        let input_pressed: bool = input_down && ((nav_keyboard_active && IsKeyPressed(ImGuiKey_Enter, false)) || (nav_gamepad_active && IsKeyPressed(ImGuiKey_NavGamepadInput, false)));
        if g.ActiveId == 0 && activate_pressed
        {
            g.NavActivateId = g.NavId;
            g.NavActivateFlags = ImGuiActivateFlags_PreferTweak;
        }
        if (g.ActiveId == 0 || g.ActiveId == g.NavId) && input_pressed
        {
            g.NavActivateInputId = g.NavId;
            g.NavActivateFlags = ImGuiActivateFlags_PreferInput;
        }
        if (g.ActiveId == 0 || g.ActiveId == g.NavId) && activate_down {
            g.NavActivateDownId = g.NavId;
        }
        if (g.ActiveId == 0 || g.ActiveId == g.NavId) && activate_pressed {
            g.NavActivatePressedId = g.NavId;
        }
    }
    if is_not_null(g.NavWindow) && flag_set(g.NavWindow.Flags, ImGuiWindowFlags_NoNavInputs) {
        g.NavDisableHighlight = true;
    }
    if g.NavActivateId != 0 {}
        // IM_ASSERT(g.NavActivateDownId == g.NavActivateId);

    // Process programmatic activation request
    // FIXME-NAV: Those should eventually be queued (unlike focus they don't cancel each others)
    if (g.NavNextActivateId != 0)
    {
        if (g.NavNextActivateFlags & ImGuiActivateFlags_PreferInput) {
            g.NavActivateInputId = g.NavNextActivateId;
        }
        else {
            g.NavActivateId = g.NavNextActivateId;g.NavActivateDownId = g.NavNextActivateId;g.NavActivatePressedId = g.NavNextActivateId;
        }
        g.NavActivateFlags = g.NavNextActivateFlags;
    }
    g.NavNextActivateId = 0;

    // Process move requests
    NavUpdateCreateMoveRequest();
    if g.NavMoveDir == ImGuiDir_None {
        NavUpdateCreateTabbingRequest();
    }
    NavUpdateAnyRequestFlag();
    g.NavIdIsAlive = false;

    // Scrolling
    if (is_not_null(g.NavWindow) && flag_clear(g.NavWindow.Flags, ImGuiWindowFlags_NoNavInputs) && is_null(g.NavWindowingTarget))
    {
        // *Fallback* manual-scroll with Nav directional keys when window has no navigable item
        let mut window: *mut ImGuiWindow =  g.NavWindow;
        let scroll_speed: c_float =  IM_ROUND(window.CalcFontSize() * 100 * io.DeltaTime); // We need round the scrolling speed because sub-pixel scroll isn't reliably supported.
        const move_dir: ImGuiDir = g.NavMoveDir;
        if (window.DC.NavLayersActiveMask == 0x00 && window.DC.NavHasScroll && move_dir != ImGuiDir_None)
        {
            if (move_dir == ImGuiDir_Left || move_dir == ImGuiDir_Right) {
                SetScrollX(window, ImFloor(window.Scroll.x + (if move_dir == ImGuiDir_Left { -1.0 } else { 1.0 }) * scroll_speed));
            }
            if (move_dir == ImGuiDir_Up || move_dir == ImGuiDir_Down) {
                SetScrollY(window, ImFloor(window.Scroll.y + (if move_dir == ImGuiDir_Up { -1.0 } else { 1.0 }) * scroll_speed));
            }
        }

        // *Normal* Manual scroll with LStick
        // Next movement request will clamp the NavId reference rectangle to the visible area, so navigation will resume within those bounds.
        if (nav_gamepad_active)
        {
            let scroll_dir: ImVec2 = GetKeyVector2d(ImGuiKey_GamepadLStickLeft, ImGuiKey_GamepadLStickRight, ImGuiKey_GamepadLStickUp, ImGuiKey_GamepadLStickDown);
            let tweak_factor: c_float =  if IsKeyDown(ImGuiKey_NavGamepadTweakSlow) { 1.0 / 10f32 } else {
                if IsKeyDown(ImGuiKey_NavGamepadTweakFast) {
                    10f32
                } else { 1.0 }
            };
            if (scroll_dir.x != 0.0 && window.ScrollbarX) {
                SetScrollX(window, ImFloor(window.Scroll.x + scroll_dir.x * scroll_speed * tweak_factor));
            }
            if (scroll_dir.y != 0.0) {
                SetScrollY(window, ImFloor(window.Scroll.y + scroll_dir.y * scroll_speed * tweak_factor));
            }
        }
    }

    // Always prioritize mouse highlight if navigation is disabled
    if (!nav_keyboard_active && !nav_gamepad_active)
    {
        g.NavDisableHighlight = true;
        g.NavDisableMouseHover = false; set_mouse_pos = false;
    }

    // Update mouse position if requested
    // (This will take into account the possibility that a Scroll was queued in the window to offset our absolute mouse position before scroll has been applied)
    if (set_mouse_pos && flag_set(io.ConfigFlags, ImGuiConfigFlags_NavEnableSetMousePos) && flag_set(io.BackendFlags, ImGuiBackendFlags_HasSetMousePos))
    {
        io.MousePos = NavCalcPreferredRefPos();io.MousePosPrev = NavCalcPreferredRefPos();
        io.WantSetMousePos = true;
        //IMGUI_DEBUG_LOG_IO("SetMousePos: (%.1f,%.10f32)\n", io.MousePos.x, io.MousePos.y);
    }

    // [DEBUG]
    g.NavScoringDebugCount = 0;
// #if IMGUI_DEBUG_NAV_RECTS
    if (g.NavWindow)
    {
        let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(g.NavWindow.Viewport);

            // for (let layer: c_int = 0; layer < 2; layer++)
        for layer in 0 .. 2
            {
            let r: ImRect =  WindowRectRelToAbs(g.NavWindow, &g.NavWindow.NavRectRel[layer]);
            draw_list.AddRect(&r.Min, &r.Max, IM_COL32(255, 200, 0, 255), 0.0, 0, 0.0);
        }

        let col: u32 = if (!g.NavWindow.Hidden) { IM_COL32(255, 0, 255, 255) } else { IM_COL32(255, 0, 0, 255) };
        let mut p: ImVec2 =  NavCalcPreferredRefPos();
        let mut buf: [c_char;32] = [0;32];
        // ImFormatString(buf, 32, "%d", g.NavLayer);
        draw_list.AddCircleFilled(&p, 3.0.0, col, 0);
        draw_list.AddText2(null_mut(), 13.0.0, p + ImVec2::new(8.0, -4.0), col, buf.as_ptr(), null(), 0.0, null());
    }
// #endif
}

pub unsafe fn NavInitRequestApplyResult()
{
    // In very rare cases g.NavWindow may be null (e.g. clearing focus after requesting an init request, which does happen when releasing Alt while clicking on void)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.NavWindow) {
        return;
    }

    // Apply result from previous navigation init request (will typically select the first item, unless SetItemDefaultFocus() has been called)
    // FIXME-NAV: On _NavFlattened windows, g.NavWindow will only be updated during subsequent frame. Not a problem currently.
    IMGUI_DEBUG_LOG_NAV("[nav] NavInitRequest: ApplyResult: NavID 0x%08X in Layer %d Window \"%s\"\n", g.NavInitResultId, g.NavLayer, g.NavWindow.Name);
    SetNavID(g.NavInitResultId, g.NavLayer, 0, &g.NavInitResultRectRel);
    g.NavIdIsAlive = true; // Mark as alive from previous frame as we got a result
    if (g.NavInitRequestFromMove) {
        NavRestoreHighlightAfterMove();
    }
}

pub unsafe fn NavUpdateCreateMoveRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let io = &mut g.IO;
    let mut window: *mut ImGuiWindow =  g.NavWindow;
    let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    let nav_keyboard_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;

    if (g.NavMoveForwardToNextFrame && window != null_mut())
    {
        // Forwarding previous request (which has been modified, e.g. wrap around menus rewrite the requests with a starting rectangle at the other side of the window)
        // (preserve most state, which were already set by the NavMoveRequestForward() function)
        // IM_ASSERT(g.NavMoveDir != ImGuiDir_None && g.NavMoveClipDir != ImGuiDir_None);
        // IM_ASSERT(g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded);
        IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequestForward %d\n", g.NavMoveDir);
    }
    else
    {
        // Initiate directional inputs request
        g.NavMoveDir = ImGuiDir_None;
        g.NavMoveFlags = ImGuiNavMoveFlags_None;
        g.NavMoveScrollFlags = ImGuiScrollFlags_None;
        if is_not_null(window) && is_null(g.NavWindowingTarget) && flag_clear(window.Flags , ImGuiWindowFlags_NoNavInputs)
        {
            let repeat_mode = ImGuiInputFlags_Repeat | ImGuiInputFlags_RepeatRateNavMove;
            if !IsActiveIdUsingNavDir(ImGuiDir_Left)  && ((nav_gamepad_active && IsKeyPressedEx(ImGuiKey_GamepadDpadLeft, repeat_mode)) || (nav_keyboard_active && IsKeyPressedEx(ImGuiKey_LeftArrow, repeat_mode))) { g.NavMoveDir = ImGuiDir_Left; }
            if !IsActiveIdUsingNavDir(ImGuiDir_Right) && ((nav_gamepad_active && IsKeyPressedEx(ImGuiKey_GamepadDpadRight, repeat_mode)) || (nav_keyboard_active && IsKeyPressedEx(ImGuiKey_RightArrow, repeat_mode))) { g.NavMoveDir = ImGuiDir_Right; }
            if !IsActiveIdUsingNavDir(ImGuiDir_Up)    && ((nav_gamepad_active && IsKeyPressedEx(ImGuiKey_GamepadDpadUp, repeat_mode)) || (nav_keyboard_active && IsKeyPressedEx(ImGuiKey_UpArrow, repeat_mode))) { g.NavMoveDir = ImGuiDir_Up; }
            if !IsActiveIdUsingNavDir(ImGuiDir_Down)  && ((nav_gamepad_active && IsKeyPressedEx(ImGuiKey_GamepadDpadDown, repeat_mode)) || (nav_keyboard_active && IsKeyPressedEx(ImGuiKey_DownArrow, repeat_mode))) { g.NavMoveDir = ImGuiDir_Down; }
        }
        g.NavMoveClipDir = g.NavMoveDir;
        g.NavScoringNoClipRect = ImRect(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    }

    // Update PageUp/PageDown/Home/End scroll
    // FIXME-NAV: Consider enabling those keys even without the master ImGuiConfigFlags_NavEnableKeyboard flag?
    let mut scoring_rect_offset_y: c_float =  0.0;
    if is_not_null(window) && g.NavMoveDir == ImGuiDir_None && nav_keyboard_active {
        scoring_rect_offset_y = NavUpdatePageUpPageDown();
    }
    if scoring_rect_offset_y != 0.0
    {
        g.NavScoringNoClipRect = window.InnerRect;
        g.NavScoringNoClipRect.TranslateY(scoring_rect_offset_y);
    }

    // [DEBUG] Always send a request
// #if IMGUI_DEBUG_NAV_SCORING
    if io.KeyCtrl && IsKeyPressed(ImGuiKey_C, false) {
        g.NavMoveDirForDebug = (ImGuiDir)((g.NavMoveDirForDebug + 1) & 3);
    }
    if io.KeyCtrl && g.NavMoveDir == ImGuiDir_None
    {
        g.NavMoveDir = g.NavMoveDirForDebug;
        g.NavMoveFlags |= ImGuiNavMoveFlags_DebugNoResult;
    }
// #endif

    // Submit
    g.NavMoveForwardToNextFrame = false;
    if g.NavMoveDir != ImGuiDir_None {
        NavMoveRequestSubmit(g.NavMoveDir, g.NavMoveClipDir, g.NavMoveFlags, g.NavMoveScrollFlags);
    }

    // Moving with no reference triggers a init request (will be used as a fallback if the direction fails to find a match)
    if g.NavMoveSubmitted && g.NavId == 0
    {
        // IMGUI_DEBUG_LOG_NAV("[nav] NavInitRequest: from move, window \"%s\", layer=%d\n", window ? window.Name : "<NULL>", g.NavLayer);
        g.NavInitRequest = true;
        g.NavInitRequestFromMove = true;
        g.NavInitResultId = 0;
        g.NavDisableHighlight = false;
    }

    // When using gamepad, we project the reference nav bounding box into window visible area.
    // This is to allow resuming navigation inside the visible area after doing a large amount of scrolling, since with gamepad every movements are relative
    // (can't focus a visible object like we can with the mouse).
    if g.NavMoveSubmitted && g.NavInputSource == ImGuiInputSource_Gamepad && g.NavLayer == ImGuiNavLayer_Main && window != null_mut()// && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded))
    {
        let mut clamp_x: bool =  (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapX)) == 0;
        let mut clamp_y: bool =  (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopY | ImGuiNavMoveFlags_WrapY)) == 0;
        let mut inner_rect_rel: ImRect =  WindowRectAbsToRel(window, ImRect(window.InnerRect.Min - ImVec2::new(1.0, 1.0), window.InnerRect.Max + ImVec2::new(1.0, 1.0)));
        if (clamp_x || clamp_y) && !inner_rect_rel.Contains(window.NavRectRel[g.NavLayer])
        {
            //IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: clamp NavRectRel for gamepad move\n");
            let pad_x: c_float =  ImMin(inner_rect_rel.GetWidth(), window.CalcFontSize() * 0.5);
            let pad_y: c_float =  ImMin(inner_rect_rel.GetHeight(), window.CalcFontSize() * 0.5); // Terrible approximation for the intent of starting navigation from first fully visible item
            inner_rect_rel.Min.x = if clamp_x { (inner_rect_rel.Min.x + pad_x) } else { -f32::MAX };
            inner_rect_rel.Max.x = if clamp_x { (inner_rect_rel.Max.x - pad_x) } else { f32::MAX };
            inner_rect_rel.Min.y = if clamp_y { (inner_rect_rel.Min.y + pad_y) } else { -f32::MAX };
            inner_rect_rel.Max.y = if clamp_y { (inner_rect_rel.Max.y - pad_y) } else { f32::MAX };
            window.NavRectRel[g.NavLayer].ClipWithFull(inner_rect_rel);
            g.NavId = 0;
            g.NavFocusScopeId = 0;
        }
    }

    // For scoring we use a single segment on the left side our current item bounding box (not touching the edge to avoid box overlap with zero-spaced items)
    let mut scoring_rect: ImRect = ImRect::default();
    if (window != null_mut())
    {
        let nav_rect_rel: ImRect =  if !window.NavRectRel[g.NavLayer].IsInverted() { window.NavRectRel[g.NavLayer] } else { ImRect::from_floats(0.0, 0.0, 0.0, 0.0) };
        scoring_rect = WindowRectRelToAbs(window, &nav_rect_rel);
        scoring_rect.TranslateY(scoring_rect_offset_y);
        scoring_rect.Min.x = ImMin(scoring_rect.Min.x + 1.0, scoring_rect.Max.x);
        scoring_rect.Max.x = scoring_rect.Min.x;
        // IM_ASSERT(!scoring_rect.IsInverted()); // Ensure if we have a finite, non-inverted bounding box here will allows us to remove extraneous ImFabs() calls in NavScoreItem().
        //GetForegroundDrawList()->AddRect(scoring_rect.Min, scoring_rect.Max, IM_COL32(255,200,0,255)); // [DEBUG]
        //if (!g.NavScoringNoClipRect.IsInverted()) { GetForegroundDrawList()->AddRect(g.NavScoringNoClipRect.Min, g.NavScoringNoClipRect.Max, IM_COL32(255, 200, 0, 255)); } // [DEBUG]
    }
    g.NavScoringRect = scoring_rect;
    g.NavScoringNoClipRect.Add(&scoring_rect.GetSize());
}

pub unsafe fn NavUpdateCreateTabbingRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  g.NavWindow;
    // IM_ASSERT(g.NavMoveDir == ImGuiDir_None);
    if window == null_mut() || g.NavWindowingTarget != null_mut() || flag_set(window.Flags, ImGuiWindowFlags_NoNavInputs) {
        return;
    }

    let tab_pressed: bool = IsKeyPressed(ImGuiKey_Tab, true) && !IsActiveIdUsingKey(ImGuiKey_Tab) && !g.IO.KeyCtrl && !g.IO.KeyAlt;
    if (!tab_pressed) {
        return;
    }

    // Initiate tabbing request
    // (this is ALWAYS ENABLED, regardless of ImGuiConfigFlags_NavEnableKeyboard flag!)
    // Initially this was designed to use counters and modulo arithmetic, but that could not work with unsubmitted items (list clipper). Instead we use a strategy close to other move requests.
    // See NavProcessItemForTabbingRequest() for a description of the various forward/backward tabbing cases with and without wrapping.
    //// FIXME: We use (g.ActiveId == 0) but (g.NavDisableHighlight == false) might be righter once we can tab through anything
    g.NavTabbingDir = if  g.IO.KeyShift { -1 } else {
        if g.ActiveId == 0 {
            0
        } else { 1 }
    };
    scroll_flags: ImGuiScrollFlags = if window.Appearing { ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_AlwaysCenterY } else { ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleEdgeY };
    clip_dir: ImGuiDir = if g.NavTabbingDir < 0 { ImGuiDir_Up} else { ImGuiDir_Down};
    NavMoveRequestSubmit(ImGuiDir_None, clip_dir, ImGuiNavMoveFlags_Tabbing, scroll_flags); // FIXME-NAV: Once we refactor tabbing, add LegacyApi flag to not activate non-inputable.
    g.NavTabbingCounter = -1;
}

// Apply result from previous frame navigation directional move request. Always called from NavUpdate()
pub unsafe fn NavMoveRequestApplyResult()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
// #if IMGUI_DEBUG_NAV_SCORING
    if g.NavMoveFlags & ImGuiNavMoveFlags_DebugNoResult {// [DEBUG] Scoring all items in NavWindow at all times
        return;
    }
// #endif

    // Select which result to use
    let mut result: *mut ImGuiNavItemData = if g.NavMoveResultLocal.ID != 0 {
        &mut g.NavMoveResultLocal
    } else {
    if g.NavMoveResultOther.ID != 0
    { &g.NavMoveResultOther } else { null_mut() }};

    // Tabbing forward wrap
    if g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing {
        if (g.NavTabbingCounter == 1 || g.NavTabbingDir == 0) && g.NavTabbingResultFirst.ID != 0 {
            result = &mut g.NavTabbingResultFirst;
        }
    }

    // In a situation when there is no results but NavId != 0, re-enable the Navigation highlight (because g.NavId is not considered as a possible result)
    if result == null_mut()
    {
        if g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing {
            g.NavMoveFlags |= ImGuiNavMoveFlags_DontSetNavHighlight;
        }
        if g.NavId != 0 && (g.NavMoveFlags & ImGuiNavMoveFlags_DontSetNavHighlight) == 0 {
            NavRestoreHighlightAfterMove();
        }
        return;
    }

    // PageUp/PageDown behavior first jumps to the bottom/top mostly visible item, _otherwise_ use the result from the previous/next page.
    if g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet {
        {
            if g.NavMoveResultLocalVisible.ID != 0 && g.NavMoveResultLocalVisible.ID != g.NavId {
                result = &mut g.NavMoveResultLocalVisible;
            }
        }
    }

    // Maybe entering a flattened child from the outside? In this case solve the tie using the regular scoring rules.
    if result != &mut g.NavMoveResultOther && g.NavMoveResultOther.ID != 0 && g.NavMoveResultOther.window.ParentWindow == g.NavWindow {
        if (g.NavMoveResultOther.DistBox < result.DistBox) || (g.NavMoveResultOther.DistBox == result.DistBox && g.NavMoveResultOther.DistCenter < result.DistCenter) {
            result = &mut g.NavMoveResultOther;
        }
    }
    // IM_ASSERT(g.NavWindow && result.Window);

    // Scroll to keep newly navigated item fully into view.
    if g.NavLayer == ImGuiNavLayer_Main
    {
        if g.NavMoveFlags & ImGuiNavMoveFlags_ScrollToEdgeY
        {
            // FIXME: Should remove this
            let scroll_target: c_float =  if g.NavMoveDir == ImGuiDir_Up { result.window.ScrollMax.y } else { 0.0 };
            SetScrollY(result.Window, scroll_target);
        }
        else
        {
            let mut rect_abs: ImRect =  WindowRectRelToAbs(result.Window, &result.RectRel);
            ScrollToRectEx(result.Window, &mut rect_abs, g.NavMoveScrollFlags);
        }
    }

    if g.NavWindow != result.Window
    {
        // IMGUI_DEBUG_LOG_FOCUS("[focus] NavMoveRequest: SetNavWindow(\"%s\")\n", result.window.Name);
        g.NavWindow = result.Window;
    }
    if g.ActiveId != result.ID {
        ClearActiveID();
    }
    if g.NavId != result.ID
    {
        // Don't set NavJustMovedToId if just landed on the same spot (which may happen with ImGuiNavMoveFlags_AllowCurrentNavId)
        g.NavJustMovedToId = result.ID;
        g.NavJustMovedToFocusScopeId = result.FocusScopeId;
        g.NavJustMovedToKeyMods = g.NavMoveKeyMods;
    }

    // Focus
    // IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: result NavID 0x%08X in Layer %d Window \"%s\"\n", result.ID, g.NavLayer, g.NavWindow.Name);
    SetNavID(result.ID, g.NavLayer, result.FocusScopeId, &result.RectRel);

    // Tabbing: Activates Inputable or Focus non-Inputable
    if ((g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) && (result.InFlags & ImGuiItemFlags_Inputable))
    {
        g.NavNextActivateId = result.ID;
        g.NavNextActivateFlags = ImGuiActivateFlags_PreferInput | ImGuiActivateFlags_TryToPreserveState;
        g.NavMoveFlags |= ImGuiNavMoveFlags_DontSetNavHighlight;
    }

    // Activate
    if (g.NavMoveFlags & ImGuiNavMoveFlags_Activate)
    {
        g.NavNextActivateId = result.ID;
        g.NavNextActivateFlags = ImGuiActivateFlags_None;
    }

    // Enable nav highlight
    if ((g.NavMoveFlags & ImGuiNavMoveFlags_DontSetNavHighlight) == 0) {
        NavRestoreHighlightAfterMove();
    }
}

// Process NavCancel input (to close a popup, get back to parent, clear focus)
// FIXME: In order to support e.g. Escape to clear a selection we'll need:
// - either to store the equivalent of ActiveIdUsingKeyInputMask for a FocusScope and test for it.
// - either to move most/all of those tests to the epilogue/end functions of the scope they are dealing with (e.g. exit child window in EndChild()) or in EndFrame(), to allow an earlier intercept
pub unsafe fn NavUpdateCancelRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let nav_gamepad_active: bool = (g.IO.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (g.IO.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    let nav_keyboard_active: bool = (g.IO.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;
    if !(nav_keyboard_active && IsKeyPressed(ImGuiKey_Escape, false)) && !(nav_gamepad_active && IsKeyPressed(ImGuiKey_NavGamepadCancel, false)) {
        return;
    }

    IMGUI_DEBUG_LOG_NAV("[nav] NavUpdateCancelRequest()\n");
    if (g.ActiveId != 0)
    {
        if (!IsActiveIdUsingKey(ImGuiKey_Escape) && !IsActiveIdUsingKey(ImGuiKey_NavGamepadCancel)) {
            ClearActiveID();
        }
    }
    else if (g.NavLayer != ImGuiNavLayer_Main)
    {
        // Leave the "menu" layer
        NavRestoreLayer(ImGuiNavLayer_Main);
        NavRestoreHighlightAfterMove();
    }
    else if is_not_null(g.NavWindow) && g.NavWindow != g.NavWindow.RootWindow && flag_clear(g.NavWindow.Flags, ImGuiWindowFlags_Popup) && is_not_null(g.NavWindow.ParentWindow)
    {
        // Exit child window
        let mut child_window: *mut ImGuiWindow =  g.NavWindow;
        let mut parent_window: *mut ImGuiWindow =  g.NavWindow.ParentWindow;
        // IM_ASSERT(child_window.ChildId != 0);
        let child_rect: ImRect =  child_window.Rect();
        FocusWindow(parent_window);
        SetNavID(child_window.ChildId, ImGuiNavLayer_Main, 0, &WindowRectAbsToRel(parent_window, &child_rect));
        NavRestoreHighlightAfterMove();
    }
    else if g.OpenPopupStack.len() > 0 && !(g.OpenPopupStack.last().unwrap().window.Flags & ImGuiWindowFlags_Modal)
    {
        // Close open popup/menu
        ClosePopupToLevel(g.OpenPopupStack.len() - 1, true);
    }
    else
    {
        // Clear NavLastId for popups but keep it for regular child window so we can leave one and come back where we were
        if is_not_null(g.NavWindow) && (flag_set(g.NavWindow.Flags, ImGuiWindowFlags_Popup) || flag_clear(g.NavWindow.Flags, ImGuiWindowFlags_ChildWindow)) {
            g.NavWindow.NavLastIds[0] = 0;
        }
        g.NavId = 0;
        g.NavFocusScopeId = 0;
    }
}

// Handle PageUp/PageDown/Home/End keys
// Called from NavUpdateCreateMoveRequest() which will use our output to create a move request
// FIXME-NAV: This doesn't work properly with NavFlattened siblings as we use NavWindow rectangle for reference
// FIXME-NAV: how to get Home/End to aim at the beginning/end of a 2D grid?
pub unsafe fn NavUpdatePageUpPageDown() -> c_float
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  g.NavWindow;
    if flag_set(window.Flags, ImGuiWindowFlags_NoNavInputs) || g.NavWindowingTarget != null_mut() {
        return 0.0;
    }

    let page_up_held: bool = IsKeyDown(ImGuiKey_PageUp) && !IsActiveIdUsingKey(ImGuiKey_PageUp);
    let page_down_held: bool = IsKeyDown(ImGuiKey_PageDown) && !IsActiveIdUsingKey(ImGuiKey_PageDown);
    let home_pressed: bool = IsKeyPressed(ImGuiKey_Home, false) && !IsActiveIdUsingKey(ImGuiKey_Home);
    let end_pressed: bool = IsKeyPressed(ImGuiKey_End, false) && !IsActiveIdUsingKey(ImGuiKey_End);
    if (page_up_held == page_down_held && home_pressed == end_pressed) {// Proceed if either (not both) are pressed, otherwise early out
        return 0.0;
    }

    if (g.NavLayer != ImGuiNavLayer_Main) {
        NavRestoreLayer(ImGuiNavLayer_Main);
    }

    if (window.DC.NavLayersActiveMask == 0x00 && window.DC.NavHasScroll)
    {
        // Fallback manual-scroll when window has no navigable item
        if (IsKeyPressed(ImGuiKey_PageUp, true)) {
            SetScrollY(window, window.Scroll.y - window.InnerRect.GetHeight());
        }
        else if (IsKeyPressed(ImGuiKey_PageDown, true)) {
            SetScrollY(window, window.Scroll.y + window.InnerRect.GetHeight());
        }
        else if (home_pressed) {
            SetScrollY(window, 0.0);
        }
        else if (end_pressed) {
            SetScrollY(window, window.ScrollMax.y);
        }
    }
    else
    {
        nav_rect_rel: &mut ImRect = window.NavRectRel[g.NavLayer];
        let page_offset_y: c_float =  ImMax(0.0, window.InnerRect.GetHeight() - window.CalcFontSize() * 1.0 + nav_rect_rel.GetHeight());
        let mut nav_scoring_rect_offset_y: c_float =  0.0;
        if IsKeyPressed(ImGuiKey_PageUp, true)
        {
            nav_scoring_rect_offset_y = -page_offset_y;
            g.NavMoveDir = ImGuiDir_Down; // Because our scoring rect is offset up, we request the down direction (so we can always land on the last item)
            g.NavMoveClipDir = ImGuiDir_Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_AlsoScoreVisibleSet;
        }
        else if IsKeyPressed(ImGuiKey_PageDown, true)
        {
            nav_scoring_rect_offset_y = page_offset_y;
            g.NavMoveDir = ImGuiDir_Up; // Because our scoring rect is offset down, we request the up direction (so we can always land on the last item)
            g.NavMoveClipDir = ImGuiDir_Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_AlsoScoreVisibleSet;
        }
        else if home_pressed
        {
            // FIXME-NAV: handling of Home/End is assuming that the top/bottom most item will be visible with Scroll.y == 0/ScrollMax.y
            // Scrolling will be handled via the ImGuiNavMoveFlags_ScrollToEdgeY flag, we don't scroll immediately to avoid scrolling happening before nav result.
            // Preserve current horizontal position if we have any.
            nav_rect_rel.Min.y = nav_rect_rel.Max.y = 0.0;
            if nav_rect_rel.IsInverted() {
                nav_rect_rel.Min.x = nav_rect_rel.Max.x = 0.0;
            }
            g.NavMoveDir = ImGuiDir_Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        else if end_pressed
        {
            nav_rect_rel.Min.y = nav_rect_rel.Max.y = window.ContentSize.y;
            if nav_rect_rel.IsInverted() {
                nav_rect_rel.Min.x = nav_rect_rel.Max.x = 0.0;
            }
            g.NavMoveDir = ImGuiDir_Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        return nav_scoring_rect_offset_y;
    }
    return 0.0;
}

pub unsafe fn NavEndFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Show CTRL+TAB list window
    if g.NavWindowingTarget != null_mut() {
        NavUpdateWindowingOverlay();
    }

    // Perform wrap-around in menus
    // FIXME-NAV: Wrap may need to apply a weight bias on the other axis. e.g. 4x4 grid with 2 last items missing on last item won't handle LoopY/WrapY correctly.
    // FIXME-NAV: Wrap (not Loop) support could be handled by the scoring function and then WrapX would function without an extra frame.
    const wanted_flags: ImGuiNavMoveFlags = ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY;
    if is_not_null(g.NavWindow) && NavMoveRequestButNoResultYet() && flag_set(g.NavMoveFlags, wanted_flags) && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded) == 0 {
        NavUpdateCreateWrappingRequest();
    }
}

pub unsafe fn NavUpdateCreateWrappingRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  g.NavWindow;

    let mut do_forward: bool =  false;
    let mut bb_rel: ImRect =  window.NavRectRel[g.NavLayer];
    clip_dir: ImGuiDir = g.NavMoveDir;
    let move_flags: ImGuiNavMoveFlags = g.NavMoveFlags;
    if g.NavMoveDir == ImGuiDir_Left && flag_set(move_flags, (ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX))
    {
        bb_rel.Min.x = window.ContentSize.x + window.WindowPadding.x;
        bb_rel.Max.x = window.ContentSize.x + window.WindowPadding.x;
        if move_flags & ImGuiNavMoveFlags_WrapX
        {
            bb_rel.TranslateY(-bb_rel.GetHeight()); // Previous row
            clip_dir = ImGuiDir_Up;
        }
        do_forward = true;
    }
    if g.NavMoveDir == ImGuiDir_Right && flag_set(move_flags, (ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX))
    {
        bb_rel.Min.x = -window.WindowPadding.x;
        bb_rel.Max.x = -window.WindowPadding.x;
        if move_flags & ImGuiNavMoveFlags_WrapX
        {
            bb_rel.TranslateY(bb_rel.GetHeight()); // Next row
            clip_dir = ImGuiDir_Down;
        }
        do_forward = true;
    }
    if g.NavMoveDir == ImGuiDir_Up && flag_set(move_flags, (ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY))
    {
        bb_rel.Min.y =  window.ContentSize.y + window.WindowPadding.y;
        bb_rel.Max.y = window.ContentSize.y + window.WindowPadding.y;
        if move_flags & ImGuiNavMoveFlags_WrapY
        {
            bb_rel.TranslateX(-bb_rel.GetWidth()); // Previous column
            clip_dir = ImGuiDir_Left;
        }
        do_forward = true;
    }
    if g.NavMoveDir == ImGuiDir_Down && flag_set(move_flags, (ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY))
    {
        bb_rel.Min.y =  -window.WindowPadding.y;
        bb_rel.Max.y = -window.WindowPadding.y;
        if move_flags & ImGuiNavMoveFlags_WrapY
        {
            bb_rel.TranslateX(bb_rel.GetWidth()); // Next column
            clip_dir = ImGuiDir_Right;
        }
        do_forward = true;
    }
    if (!do_forward) {
        return;
    }
    window.NavRectRel[g.NavLayer] = bb_rel;
    NavMoveRequestForward(g.NavMoveDir, clip_dir, move_flags, g.NavMoveScrollFlags);
}

pub unsafe fn FindWindowFocusIndex(window: *mut ImGuiWindow) -> c_int
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let order: c_int = window.FocusOrder as c_int;
    // IM_ASSERT(window.RootWindow == window); // No child window (not testing _ChildWindow because of docking)
    // IM_ASSERT(g.WindowsFocusOrder[order] == window);
    return order;
}

pub unsafe fn FindWindowNavFocusable(i_start: c_int, i_stop: c_int, dir: c_int) -> *mut ImGuiWindow // FIXME-OPT O(N)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (let i: c_int = i_start; i >= 0 && i < g.WindowsFocusOrder.Size && i != i_stop; i += dir)
    let mut i = i_start;
    while i > 0 && i < g.WindowsFocusOrder.len() as c_int && i != i_stop
    {
        if IsWindowNavFocusable(g.WindowsFocusOrder[i]) {
            return g.WindowsFocusOrder[i];
        }
        i += dir;
    }
    return null_mut();
}

pub unsafe fn NavUpdateWindowingHighlightWindow(focus_change_dir: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindowingTarget);
    if g.NavWindowingTarget.Flags & ImGuiWindowFlags_Modal {
        return;
    }

    let i_current: c_int = FindWindowFocusIndex(g.NavWindowingTarget);
    let mut window_target: *mut ImGuiWindow =  FindWindowNavFocusable(i_current + focus_change_dir, -INT_MAX, focus_change_dir);
    if !window_target {
        window_target = FindWindowNavFocusable(if focus_change_dir < 0 { g.WindowsFocusOrder.Size - 1 } else { 0 }, i_current, focus_change_dir);
    }
    if window_target // Don't reset windowing target if there's a single window in the list
    {
        g.NavWindowingTarget = window_target;
        g.NavWindowingTargetAnim = window_target;
        g.NavWindowingAccumDeltaPos = ImVec2::new(0.0, 0.0);
        g.NavWindowingAccumDeltaSize = ImVec2::new(0.0, 0.0);
    }
    g.NavWindowingToggleLayer = false;
}

// Windowing management mode
// Keyboard: CTRL+Tab (change focus/move/resize), Alt (toggle menu layer)
// Gamepad:  Hold Menu/Square (change focus/move/resize), Tap Menu/Square (toggle menu layer)
pub unsafe fn NavUpdateWindowing()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut io = &mut g.IO;

    let mut apply_focus_window: *mut ImGuiWindow =  null_mut();
    let mut apply_toggle_layer: bool =  false;

    let mut modal_window: *mut ImGuiWindow =  GetTopMostPopupModal();
    let mut allow_windowing: bool =  (modal_window == null_mut());
    if !allow_windowing {
        g.NavWindowingTarget = null_mut();
    }

    // Fade out
    if is_not_null(g.NavWindowingTargetAnim) && g.NavWindowingTarget == null_mut()
    {
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha - io.DeltaTime * 10f32, 0.0);
        if g.DimBgRatio <= 0.0 && g.NavWindowingHighlightAlpha <= 0.0 {
            g.NavWindowingTargetAnim = null_mut();
        }
    }

    // Start CTRL+Tab or Square+L/R window selection
    let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    let nav_keyboard_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;
    let start_windowing_with_gamepad: bool = allow_windowing && nav_gamepad_active && is_null(g.NavWindowingTarget) && IsKeyPressed(ImGuiKey_NavGamepadMenu, false);
    let start_windowing_with_keyboard: bool = allow_windowing && is_null(g.NavWindowingTarget) && io.KeyCtrl && IsKeyPressed(ImGuiKey_Tab, false); // Note: enabled even without NavEnableKeyboard!
    if start_windowing_with_gamepad || start_windowing_with_keyboard {
        let mut window: *mut ImGuiWindow = if is_not_null(g.NavWindow) {
            g.NavWindow
        } else { FindWindowNavFocusable(g.WindowsFocusOrder.Size - 1, -INT_MAX, -1) };
        if is_not_null(window)
        {
        g.NavWindowingTarget = null_mut();
            g.NavWindowingTargetAnim = window.RootWindow;
            g.NavWindowingTimer = 0.0;
            g.NavWindowingHighlightAlpha = 0.0;
            g.NavWindowingAccumDeltaPos = ImVec2::new(0.0, 0.0);
            g.NavWindowingAccumDeltaSize = ImVec2::new(0.0, 0.0);
            g.NavWindowingToggleLayer = if start_windowing_with_gamepad { true } else { false }; // Gamepad starts toggling layer
        g.NavInputSource = if start_windowing_with_keyboard { ImGuiInputSource_Keyboard } else { ImGuiInputSource_Gamepad };
        }
    }

    // Gamepad update
    g.NavWindowingTimer += io.DeltaTime;
    if is_not_null(g.NavWindowingTarget) && g.NavInputSource == ImGuiInputSource_Gamepad
    {
        // Highlight only appears after a brief time holding the button, so that a fast tap on PadMenu (to toggle NavLayer) doesn't add visual noise
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05f32));

        // Select window to focus
        let focus_change_dir: c_int = IsKeyPressed(ImGuiKey_GamepadL1, false) - IsKeyPressed(ImGuiKey_GamepadR1, false);
        if focus_change_dir != 0
        {
            NavUpdateWindowingHighlightWindow(focus_change_dir);
            g.NavWindowingHighlightAlpha = 1.0;
        }

        // Single press toggles NavLayer, long press with L/R apply actual focus on release (until then the window was merely rendered top-most)
        if !IsKeyDown(ImGuiKey_NavGamepadMenu)
        {
            g.NavWindowingToggleLayer &= (g.NavWindowingHighlightAlpha < 1.0); // Once button was held long enough we don't consider it a tap-to-toggle-layer press anymore.
            if g.NavWindowingToggleLayer && is_not_null(g.NavWindow) {
                apply_toggle_layer = true;
            }
            else if !g.NavWindowingToggleLayer {
                apply_focus_window = g.NavWindowingTarget;
            }
            g.NavWindowingTarget= null_mut();
        }
    }

    // Keyboard: Focus
    if is_not_null(g.NavWindowingTarget) && g.NavInputSource == ImGuiInputSource_Keyboard
    {
        // Visuals only appears after a brief time after pressing TAB the first time, so that a fast CTRL+TAB doesn't add visual noise
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05f32)); // 1.0f
        if IsKeyPressed(ImGuiKey_Tab, true) {
            NavUpdateWindowingHighlightWindow(if io.KeyShift { 1 }else { -1 });
        }
        if !io.KeyCtrl {
            apply_focus_window = g.NavWindowingTarget;
        }
    }

    // Keyboard: Press and Release ALT to toggle menu layer
    // - Testing that only Alt is tested prevents Alt+Shift or AltGR from toggling menu layer.
    // - AltGR is normally Alt+Ctrl but we can't reliably detect it (not all backends/systems/layout emit it as Alt+Ctrl). But even on keyboards without AltGR we don't want Alt+Ctrl to open menu anyway.
    if nav_keyboard_active && IsKeyPressed(ImGuiKey_ModAlt, false)
    {
        g.NavWindowingToggleLayer = true;
        g.NavInputSource = ImGuiInputSource_Keyboard;
    }
    if g.NavWindowingToggleLayer && g.NavInputSource == ImGuiInputSource_Keyboard
    {
        // We cancel toggling nav layer when any text has been typed (generally while holding Alt). (See #370)
        // We cancel toggling nav layer when other modifiers are pressed. (See #4439)
        if io.InputQueueCharacters.Size > 0 || io.KeyCtrl || io.KeyShift || io.KeySuper {
            g.NavWindowingToggleLayer = false;
        }

        // Apply layer toggle on release
        // Important: as before version <18314 we lacked an explicit IO event for focus gain/loss, we also compare mouse validity to detect old backends clearing mouse pos on focus loss.
        if IsKeyReleased(ImGuiKey_ModAlt) && g.NavWindowingToggleLayer {
            if g.ActiveId == 0 || g.ActiveIdAllowOverlap {
                if IsMousePosValid(&io.MousePos) == IsMousePosValid(&io.MousePosPrev) {
                    apply_toggle_layer = true;
                }
            }
        }
        if !IsKeyDown(ImGuiKey_ModAlt) {
            g.NavWindowingToggleLayer = false;
        }
    }

    // Move window
    if is_not_null(g.NavWindowingTarget) && flag_clear(g.NavWindowingTarget.Flags , ImGuiWindowFlags_NoMove)
    {
        nav_move_dir: ImVec2;
        if g.NavInputSource == ImGuiInputSource_Keyboard && !io.KeyShift {
            nav_move_dir = GetKeyVector2d(ImGuiKey_LeftArrow, ImGuiKey_RightArrow, ImGuiKey_UpArrow, ImGuiKey_DownArrow);
        }
        if g.NavInputSource == ImGuiInputSource_Gamepad {
            nav_move_dir = GetKeyVector2d(ImGuiKey_GamepadLStickLeft, ImGuiKey_GamepadLStickRight, ImGuiKey_GamepadLStickUp, ImGuiKey_GamepadLStickDown);
        }
        if nav_move_dir.x != 0.0 || nav_move_dir.y != 0.0
        {
            let NAV_MOVE_SPEED: c_float =  800f32;
            let move_step: c_float =  NAV_MOVE_SPEED * io.DeltaTime * ImMin(io.DisplayFramebufferScale.x, io.DisplayFramebufferScale.y);
            g.NavWindowingAccumDeltaPos += nav_move_dir * move_step;
            g.NavDisableMouseHover = true;
            let accum_floored: ImVec2 = ImFloor(g.NavWindowingAccumDeltaPos);
            if accum_floored.x != 0.0 || accum_floored.y != 0.0
            {
                let mut moving_window: *mut ImGuiWindow =  g.NavWindowingTarget.RootWindowDockTree;
                SetWindowPos(moving_window, moving_window.Pos + accum_floored, ImGuiCond_Always);
                g.NavWindowingAccumDeltaPos -= accum_floored;
            }
        }
    }

    // Apply final focus
    if is_not_null(apply_focus_window) && (g.NavWindow == null_mut() || apply_focus_window != g.NavWindow.RootWindow)
    {
        previous_viewport: *mut ImGuiViewport = if is_not_null(g.NavWindow) { g.NavWindow.Viewport } else { null_mut() };
        ClearActiveID();
        NavRestoreHighlightAfterMove();
        apply_focus_window = NavRestoreLastChildNavWindow(apply_focus_window);
        ClosePopupsOverWindow(apply_focus_window, false);
        FocusWindow(apply_focus_window);
        if apply_focus_window.NavLastIds[0] == 0 {
            NavInitWindow(apply_focus_window, false);
        }

        // If the window has ONLY a menu layer (no main layer), select it directly
        // Use NavLayersActiveMaskNext since windows didn't have a chance to be Begin()-ed on this frame,
        // so CTRL+Tab where the keys are only held for 1 frame will be able to use correct layers mask since
        // the target window as already been previewed once.
        // FIXME-NAV: This should be done in NavInit.. or in FocusWindow... However in both of those cases,
        // we won't have a guarantee that windows has been visible before and therefore NavLayersActiveMask*
        // won't be valid.
        if apply_focus_window.DC.NavLayersActiveMaskNext == (1 << ImGuiNavLayer_Menu) {
            g.NavLayer = ImGuiNavLayer_Menu;
        }

        // Request OS level focus
        // && is_not_null(g.PlatformIO.Platform_SetWindowFocus)
        if apply_focus_window.Viewport != previous_viewport  {
            g.PlatformIO.Platform_SetWindowFocus(apply_focus_window.Viewport);
        }
    }
    if (apply_focus_window) {
        g.NavWindowingTarget = null_mut();
    }

    // Apply menu/layer toggle
    if (apply_toggle_layer && is_not_null(g.NavWindow))
    {
        ClearActiveID();

        // Move to parent menu if necessary
        let mut new_nav_window: *mut ImGuiWindow =  g.NavWindow;
        while is_not_null(new_nav_window.ParentWindow)
            && (new_nav_window.DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) == 0
            && (new_nav_window.Flags & ImGuiWindowFlags_ChildWindow) != 0
            && (new_nav_window.Flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_ChildMenu)) == 0 {
            new_nav_window = new_nav_window.ParentWindow;
        }
        if new_nav_window != g.NavWindow
        {
            let mut old_nav_window: *mut ImGuiWindow =  g.NavWindow;
            FocusWindow(new_nav_window);
            new_nav_window.NavLastChildNavWindow = old_nav_window;
        }

        // Toggle layer
        let new_nav_layer: ImGuiNavLayer = if g.NavWindow.DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu) { (g.NavLayer ^ 1)} else { ImGuiNavLayer_Main};
        if new_nav_layer != g.NavLayer
        {
            // Reinitialize navigation when entering menu bar with the Alt key (FIXME: could be a properly of the layer?)
            let preserve_layer_1_nav_id: bool = (new_nav_window.DockNodeAsHost != null_mut());
            if new_nav_layer == ImGuiNavLayer_Menu && !preserve_layer_1_nav_id {
                g.NavWindow.NavLastIds[new_nav_layer] = 0;
            }
            NavRestoreLayer(new_nav_layer);
            NavRestoreHighlightAfterMove();
        }
    }
}

// Window has already passed the IsWindowNavFocusable()
pub unsafe fn GetFallbackWindowNameForWindowingList(window: *mut ImGuiWindow) ->  *const c_char
{
    // if (window.Flags & ImGuiWindowFlags_Popup) {
    //     return "(Popup)";
    // }
    // if ((window.Flags & ImGuiWindowFlags_MenuBar) && strcmp(window.Name, "##MainMenuBar") == 0) {
    //     return "(Main menu bar)";
    // }
    // if (window.DockNodeAsHost) {
    //     return "(Dock node)";
    // }
    // return "(Untitled)";
    todo!()
}

// Overlay displayed when using CTRL+TAB. Called by EndFrame().
pub unsafe fn NavUpdateWindowingOverlay()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindowingTarget != NULL);

    if g.NavWindowingTimer < NAV_WINDOWING_LIST_APPEAR_DELAY {
        return;
    }

    if g.NavWindowingListWindow == null_mut() {
        g.NavWindowingListWindow = FindWindowByName(str_to_const_c_char_ptr("###NavWindowingList"));
    }
    let viewport: *const ImGuiViewport = /*g.NavWindow ? g.Navwindow.Viewport :*/ GetMainViewport();
    SetNextWindowSizeConstraints(&ImVec2::new(viewport.Size.x * 0.20f32, viewport.Size.y * 0.200f32), &ImVec2::new(f32::MAX, f32::MAX), (), null_mut());
    SetNextWindowPos(&viewport.GetCenter(), ImGuiCond_Always, &ImVec2::new(0.5, 0.5));
    PushStyleVar(ImGuiStyleVar_WindowPadding, g.Style.WindowPadding * 2.00f32);
    Begin(str_to_const_c_char_ptr("###NavWindowingList"), null_mut(), ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoFocusOnAppearing | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoInputs | ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoSavedSettings);
    // for (let n: c_int = g.WindowsFocusOrder.Size - 1; n >= 0; n--)
    for n in g.WindowsFocusOrder.len() - 1 .. 0
    {
        let mut window: *mut ImGuiWindow =  g.WindowsFocusOrder[n];
        // IM_ASSERT(window != NULL); // Fix static analyzers
        if !IsWindowNavFocusable(window) {
            continue;
        }
        let mut  label: *const c_char = window.Name;
        if label == FindRenderedTextEnd(label, null()) {
            label = GetFallbackWindowNameForWindowingList(window);
        }
        Selectable(label, g.NavWindowingTarget == window);
    }
    End();
    PopStyleVar();
}
