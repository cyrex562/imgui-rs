// the structure of this file:
//
// [SECTION] bezier curve helpers
// [SECTION] draw list helper
// [SECTION] ui state logic
// [SECTION] render helpers
// [SECTION] API implementation

#include "imnodes.h"
#include "imnodes_internal.h"

#define IMGUI_DEFINE_MATH_OPERATORS
#include "internal_h.rs"

// Check minimum ImGui version
#define MINIMUM_COMPATIBLE_IMGUI_VERSION 17400
// #ifIMGUI_VERSION_NUM < MINIMUM_COMPATIBLE_IMGUI_VERSION
#error "Minimum ImGui version requirement not met -- please use a newer version!"


// #include <limits.h>
// #include <math.h>
// #include <new>
// #include <stdint.h>
// #include <stdio.h> // for fwrite, ssprintf, sscanf
// #include <stdlib.h>
// #include <string.h> // strlen, strncmp

// Use secure CRT function variants to avoid MSVC compiler errors
// #ifdef _MSC_VER
// #define sscanf sscanf_s


ImNodesContext* GImNodes = None;

// namespace IMNODES_NAMESPACE
// {
// namespace
// {
// [SECTION] bezier curve helpers

use crate::Context;
use crate::types::DataType;

struct CubicBezier
{
    Vector2D P0, P1, P2, P3;
    int    NumSegments;
};

inline Vector2D EvalCubicBezier(
    let   t,
    const Vector2D& P0,
    const Vector2D& P1,
    const Vector2D& P2,
    const Vector2D& P3)
{
    // B(t) = (1-t)**3 p0 + 3(1 - t)**2 t P1 + 3(1-t)t**2 P2 + t**3 P3

    let u = 1.0 - t;
    let b0 = u * u * u;
    let b1 = 3 * u * u * t;
    let b2 = 3 * u * t * t;
    let b3 = t * t * t;
    return Vector2D::new(
        b0 * P0.x + b1 * P1.x + b2 * P2.x + b3 * P3.x,
        b0 * P0.y + b1 * P1.y + b2 * P2.y + b3 * P3.y);
}

// Calculates the closest point along each bezier curve segment.
Vector2D GetClosestPointOnCubicBezier(let num_segments, const Vector2D& p, const CubicBezier& cb)
{
    // IM_ASSERT(num_segments > 0);
    Vector2D p_last = cb.P0;
    Vector2D p_closest;
    let p_closest_dist =  f32::MAX;
    let t_step =  1.0 / num_segments;
    for (int i = 1; i <= num_segments;  += 1i)
    {
        Vector2D p_current = EvalCubicBezier(t_step * i, cb.P0, cb.P1, cb.P2, cb.P3);
        Vector2D p_line = ImLineClosestPoint(p_last, p_current, p);
        let dist =  ImLengthSqr(p - p_line);
        if (dist < p_closest_dist)
        {
            p_closest = p_line;
            p_closest_dist = dist;
        }
        p_last = p_current;
    }
    return p_closest;
}

inline float GetDistanceToCubicBezier(
    const Vector2D&      pos,
    const CubicBezier& cubic_bezier,
    let          num_segments)
{
    const Vector2D point_on_curve = GetClosestPointOnCubicBezier(num_segments, pos, cubic_bezier);

    const Vector2D to_curve = point_on_curve - pos;
    return ImSqrt(ImLengthSqr(to_curve));
}

inline Rect GetContainingRectForCubicBezier(const CubicBezier& cb)
{
    const Vector2D min = Vector2D::new(ImMin(cb.P0.x, cb.P3.x), ImMin(cb.P0.y, cb.P3.y));
    const Vector2D max = Vector2D::new(ImMax(cb.P0.x, cb.P3.x), ImMax(cb.P0.y, cb.P3.y));

    let hover_distance = GImNodes.Style.LinkHoverDistance;

    Rect rect(min, max);
    rect.Add(cb.P1);
    rect.Add(cb.P2);
    rect.Expand(Vector2D::new(hover_distance, hover_distance));

    return rect;
}

inline CubicBezier GetCubicBezier(
    Vector2D                     start,
    Vector2D                     end,
    const ImNodesAttributeType start_type,
    let                line_segments_per_length)
{
    // IM_ASSERT(
        (start_type == ImNodesAttributeType_Input) || (start_type == ImNodesAttributeType_Output));
    if (start_type == ImNodesAttributeType_Input)
    {
        ImSwap(start, end);
    }

    let  link_length = ImSqrt(ImLengthSqr(end - start));
    const Vector2D offset = Vector2D::new(0.25 * link_length, 0.f);
    CubicBezier  cubic_bezier;
    cubic_bezier.P0 = start;
    cubic_bezier.P1 = start + offset;
    cubic_bezier.P2 = end - offset;
    cubic_bezier.P3 = end;
    cubic_bezier.NumSegments = ImMax(static_cast<int>(link_length * line_segments_per_length), 1);
    return cubic_bezier;
}

inline float EvalImplicitLineEq(const Vector2D& p1, const Vector2D& p2, const Vector2D& p)
{
    return (p2.y - p1.y) * p.x + (p1.x - p2.x) * p.y + (p2.x * p1.y - p1.x * p2.y);
}

inline int Sign(float val) { return int(val > 0.0) - int(val < 0.0); }

inline bool RectangleOverlapsLineSegment(const Rect& rect, const Vector2D& p1, const Vector2D& p2)
{
    // Trivial case: rectangle contains an endpoint
    if (rect.contains(p1) || rect.contains(p2))
    {
        return true;
    }

    // Flip rectangle if necessary
    Rect flip_rect = rect;

    if (flip_rect.min.x > flip_rect.max.x)
    {
        ImSwap(flip_rect.min.x, flip_rect.max.x);
    }

    if (flip_rect.min.y > flip_rect.max.y)
    {
        ImSwap(flip_rect.min.y, flip_rect.max.y);
    }

    // Trivial case: line segment lies to one particular side of rectangle
    if ((p1.x < flip_rect.min.x && p2.x < flip_rect.min.x) ||
        (p1.x > flip_rect.max.x && p2.x > flip_rect.max.x) ||
        (p1.y < flip_rect.min.y && p2.y < flip_rect.min.y) ||
        (p1.y > flip_rect.max.y && p2.y > flip_rect.max.y))
    {
        return false;
    }

    let corner_signs[4] = {
        Sign(EvalImplicitLineEq(p1, p2, flip_rect.min)),
        Sign(EvalImplicitLineEq(p1, p2, Vector2D::new(flip_rect.max.x, flip_rect.min.y))),
        Sign(EvalImplicitLineEq(p1, p2, Vector2D::new(flip_rect.min.x, flip_rect.max.y))),
        Sign(EvalImplicitLineEq(p1, p2, flip_rect.max))};

    int sum = 0;
    int sum_abs = 0;

    for (int i = 0; i < 4;  += 1i)
    {
        sum += corner_signs[i];
        sum_abs += abs(corner_signs[i]);
    }

    // At least one corner of rectangle lies on a different side of line segment
    return abs(sum) != sum_abs;
}

inline bool RectangleOverlapsBezier(const Rect& rectangle, const CubicBezier& cubic_bezier)
{
    Vector2D current =
        EvalCubicBezier(0.f, cubic_bezier.P0, cubic_bezier.P1, cubic_bezier.P2, cubic_bezier.P3);
    let dt = 1.0 / cubic_bezier.NumSegments;
    for (int s = 0; s < cubic_bezier.NumSegments;  += 1s)
    {
        Vector2D next = EvalCubicBezier(
            static_cast<float>((s + 1) * dt),
            cubic_bezier.P0,
            cubic_bezier.P1,
            cubic_bezier.P2,
            cubic_bezier.P3);
        if (RectangleOverlapsLineSegment(rectangle, current, next))
        {
            return true;
        }
        current = next;
    }
    return false;
}

inline bool RectangleOverlapsLink(
    const Rect&              rectangle,
    const Vector2D&              start,
    const Vector2D&              end,
    const ImNodesAttributeType start_type)
{
    // First level: simple rejection test via rectangle overlap:

    Rect lrect = Rect(start, end);
    if (lrect.min.x > lrect.max.x)
    {
        ImSwap(lrect.min.x, lrect.max.x);
    }

    if (lrect.min.y > lrect.max.y)
    {
        ImSwap(lrect.min.y, lrect.max.y);
    }

    if (rectangle.Overlaps(lrect))
    {
        // First, check if either one or both endpoinds are trivially contained
        // in the rectangle

        if (rectangle.contains(start) || rectangle.contains(end))
        {
            return true;
        }

        // Second level of refinement: do a more expensive test against the
        // link

        const CubicBezier cubic_bezier =
            GetCubicBezier(start, end, start_type, GImNodes.Style.LinkLineSegmentsPerLength);
        return RectangleOverlapsBezier(rectangle, cubic_bezier);
    }

    return false;
}

// [SECTION] coordinate space conversion helpers

inline Vector2D ScreenSpaceToGridSpace(const ImNodesEditorContext& editor, const Vector2D& v)
{
    return v - GImNodes.CanvasOriginScreenSpace - editor.Panning;
}

inline Rect ScreenSpaceToGridSpace(const ImNodesEditorContext& editor, const Rect& r)
{
    return Rect(ScreenSpaceToGridSpace(editor, r.min), ScreenSpaceToGridSpace(editor, r.max));
}

inline Vector2D GridSpaceToScreenSpace(const ImNodesEditorContext& editor, const Vector2D& v)
{
    return v + GImNodes.CanvasOriginScreenSpace + editor.Panning;
}

inline Vector2D GridSpaceToEditorSpace(const ImNodesEditorContext& editor, const Vector2D& v)
{
    return v + editor.Panning;
}

inline Vector2D EditorSpaceToGridSpace(const ImNodesEditorContext& editor, const Vector2D& v)
{
    return v - editor.Panning;
}

inline Vector2D EditorSpaceToScreenSpace(const Vector2D& v)
{
    return GImNodes.CanvasOriginScreenSpace + v;
}

inline Vector2D MiniMapSpaceToGridSpace(const ImNodesEditorContext& editor, const Vector2D& v)
{
    return (v - editor.MiniMapContentScreenSpace.min) / editor.MiniMapScaling +
           editor.GridContentBounds.min;
}

inline Vector2D ScreenSpaceToMiniMapSpace(const ImNodesEditorContext& editor, const Vector2D& v)
{
    return (ScreenSpaceToGridSpace(editor, v) - editor.GridContentBounds.min) *
               editor.MiniMapScaling +
           editor.MiniMapContentScreenSpace.min;
}

inline Rect ScreenSpaceToMiniMapSpace(const ImNodesEditorContext& editor, const Rect& r)
{
    return Rect(
        ScreenSpaceToMiniMapSpace(editor, r.min), ScreenSpaceToMiniMapSpace(editor, r.max));
}

// [SECTION] draw list helper

void ImDrawListGrowChannels(ImDrawList* draw_list, let num_channels)
{
    ImDrawListSplitter& splitter = draw_list->_Splitter;

    if (splitter._Count == 1)
    {
        splitter.Split(draw_list, num_channels + 1);
        return;
    }

    // NOTE: this logic has been lifted from ImDrawListSplitter::split with slight modifications
    // to allow nested splits. The main modification is that we only create new ImDrawChannel
    // instances after splitter._count, instead of over the whole splitter._channels array like
    // the regular ImDrawListSplitter::split method does.

    let old_channel_capacity = splitter.channels.size;
    // NOTE: _channels is not resized down, and therefore _count <= _channels.size()!
    let old_channel_count = splitter._Count;
    let requested_channel_count = old_channel_count + num_channels;
    if (old_channel_capacity < old_channel_count + num_channels)
    {
        splitter.channels.resize(requested_channel_count);
    }

    splitter._Count = requested_channel_count;

    for (int i = old_channel_count; i < requested_channel_count;  += 1i)
    {
        ImDrawChannel& channel = splitter.channels[i];

        // If we're inside the old capacity region of the array, we need to reuse the existing
        // memory of the command and index buffers.
        if (i < old_channel_capacity)
        {
            channel._cmd_buffer.resize(0);
            channel._idx_buffer.resize(0);
        }
        // Else, we need to construct new draw channels.
        else
        {
            IM_PLACEMENT_NEW(&channel) ImDrawChannel();
        }

        {
            ImDrawCmd draw_cmd;
            draw_cmd.clip_rect = draw_list->clip_rect_stack.back();
            draw_cmd.texture_id = draw_list->texture_id_stack.back();
            channel._cmd_buffer.push_back(draw_cmd);
        }
    }
}

void ImDrawListSplitterSwapChannels(
    ImDrawListSplitter& splitter,
    let           lhs_idx,
    let           rhs_idx)
{
    if (lhs_idx == rhs_idx)
    {
        return;
    }

    // IM_ASSERT(lhs_idx >= 0 && lhs_idx < splitter._Count);
    // IM_ASSERT(rhs_idx >= 0 && rhs_idx < splitter._Count);

    ImDrawChannel& lhs_channel = splitter.channels[lhs_idx];
    ImDrawChannel& rhs_channel = splitter.channels[rhs_idx];
    lhs_channel._cmd_buffer.swap(rhs_channel._cmd_buffer);
    lhs_channel._idx_buffer.swap(rhs_channel._idx_buffer);

    let current_channel = splitter._Current;

    if (current_channel == lhs_idx)
    {
        splitter._Current = rhs_idx;
    }
    else if (current_channel == rhs_idx)
    {
        splitter._Current = lhs_idx;
    }
}

void DrawListSet(ImDrawList* window_draw_list)
{
    GImNodes.CanvasDrawList = window_draw_list;
    GImNodes.NodeIdxToSubmissionIdx.Clear();
    GImNodes.NodeIdxSubmissionOrder.clear();
}

// The draw list channels are structured as follows. First we have our base channel, the canvas grid
// on which we render the grid lines in BeginNodeEditor(). The base channel is the reason
// draw_list_submission_idx_to_background_channel_idx offsets the index by one. Each BeginNode()
// call appends two new draw channels, for the node background and foreground. The node foreground
// is the channel into which the node's ImGui content is rendered. Finally, in EndNodeEditor() we
// append one last draw channel for rendering the selection box and the incomplete link on top of
// everything else.
//
// +----------+----------+----------+----------+----------+----------+
// |          |          |          |          |          |          |
// |canvas    |node      |node      |...       |...       |click     |
// |grid      |background|foreground|          |          |interaction
// |          |          |          |          |          |          |
// +----------+----------+----------+----------+----------+----------+
//            |                     |
//            |   submission idx    |
//            |                     |
//            -----------------------

void DrawListAddNode(let node_idx)
{
    GImNodes.NodeIdxToSubmissionIdx.SetInt(
        static_cast<Id32>(node_idx), GImNodes.NodeIdxSubmissionOrder.size);
    GImNodes.NodeIdxSubmissionOrder.push_back(node_idx);
    ImDrawListGrowChannels(GImNodes.CanvasDrawList, 2);
}

void DrawListAppendClickInteractionChannel()
{
    // NOTE: don't use this function outside of EndNodeEditor. Using this before all nodes have been
    // added will screw up the node draw order.
    ImDrawListGrowChannels(GImNodes.CanvasDrawList, 1);
}

int DrawListSubmissionIdxToBackgroundChannelIdx(let submission_idx)
{
    // NOTE: the first channel is the canvas background, i.e. the grid
    return 1 + 2 * submission_idx;
}

int DrawListSubmissionIdxToForegroundChannelIdx(let submission_idx)
{
    return DrawListSubmissionIdxToBackgroundChannelIdx(submission_idx) + 1;
}

void DrawListActivateClickInteractionChannel()
{
    GImNodes.CanvasDrawList->_Splitter.SetCurrentChannel(
        GImNodes.CanvasDrawList, GImNodes.CanvasDrawList->_Splitter._Count - 1);
}

void DrawListActivateCurrentNodeForeground()
{
    let foreground_channel_idx =
        DrawListSubmissionIdxToForegroundChannelIdx(GImNodes.NodeIdxSubmissionOrder.size - 1);
    GImNodes.CanvasDrawList->_Splitter.SetCurrentChannel(
        GImNodes.CanvasDrawList, foreground_channel_idx);
}

void DrawListActivateNodeBackground(let node_idx)
{
    let submission_idx =
        GImNodes.NodeIdxToSubmissionIdx.GetInt(static_cast<Id32>(node_idx), -1);
    // There is a discrepancy in the submitted node count and the rendered node count! Did you call
    // one of the following functions
    // * EditorContextMoveToNode
    // * SetNodeScreenSpacePos
    // * SetNodeGridSpacePos
    // * SetNodeDraggable
    // after the BeginNode/EndNode function calls?
    // IM_ASSERT(submission_idx != -1);
    let background_channel_idx = DrawListSubmissionIdxToBackgroundChannelIdx(submission_idx);
    GImNodes.CanvasDrawList->_Splitter.SetCurrentChannel(
        GImNodes.CanvasDrawList, background_channel_idx);
}

void DrawListSwapSubmissionIndices(let lhs_idx, let rhs_idx)
{
    // IM_ASSERT(lhs_idx != rhs_idx);

    let lhs_foreground_channel_idx = DrawListSubmissionIdxToForegroundChannelIdx(lhs_idx);
    let lhs_background_channel_idx = DrawListSubmissionIdxToBackgroundChannelIdx(lhs_idx);
    let rhs_foreground_channel_idx = DrawListSubmissionIdxToForegroundChannelIdx(rhs_idx);
    let rhs_background_channel_idx = DrawListSubmissionIdxToBackgroundChannelIdx(rhs_idx);

    ImDrawListSplitterSwapChannels(
        GImNodes.CanvasDrawList->_Splitter,
        lhs_background_channel_idx,
        rhs_background_channel_idx);
    ImDrawListSplitterSwapChannels(
        GImNodes.CanvasDrawList->_Splitter,
        lhs_foreground_channel_idx,
        rhs_foreground_channel_idx);
}

void DrawListSortChannelsByDepth(const ImVector<int>& node_idx_depth_order)
{
    if (GImNodes.NodeIdxToSubmissionIdx.data.size < 2)
    {
        return;
    }

    // IM_ASSERT(node_idx_depth_order.size == GImNodes.NodeIdxSubmissionOrder.size);

    int start_idx = node_idx_depth_order.size - 1;

    while (node_idx_depth_order[start_idx] == GImNodes.NodeIdxSubmissionOrder[start_idx])
    {
        if (--start_idx == 0)
        {
            // early out if submission order and depth order are the same
            return;
        }
    }

    // TODO: this is an O(N^2) algorithm. It might be worthwhile revisiting this to see if the time
    // complexity can be reduced.

    for (int depth_idx = start_idx; depth_idx > 0; --depth_idx)
    {
        let node_idx = node_idx_depth_order[depth_idx];

        // Find the current index of the node_idx in the submission order array
        int submission_idx = -1;
        for (int i = 0; i < GImNodes.NodeIdxSubmissionOrder.size;  += 1i)
        {
            if (GImNodes.NodeIdxSubmissionOrder[i] == node_idx)
            {
                submission_idx = i;
                break;
            }
        }
        // IM_ASSERT(submission_idx >= 0);

        if (submission_idx == depth_idx)
        {
            continue;
        }

        for (int j = submission_idx; j < depth_idx;  += 1j)
        {
            DrawListSwapSubmissionIndices(j, j + 1);
            ImSwap(GImNodes.NodeIdxSubmissionOrder[j], GImNodes.NodeIdxSubmissionOrder[j + 1]);
        }
    }
}

// [SECTION] ui state logic

Vector2D GetScreenSpacePinCoordinates(
    const Rect&              node_rect,
    const Rect&              attribute_rect,
    const ImNodesAttributeType type)
{
    // IM_ASSERT(type == ImNodesAttributeType_Input || type == ImNodesAttributeType_Output);
    let x = type == ImNodesAttributeType_Input
                        ? (node_rect.min.x - GImNodes.Style.PinOffset)
                        : (node_rect.max.x + GImNodes.Style.PinOffset);
    return Vector2D::new(x, 0.5 * (attribute_rect.min.y + attribute_rect.max.y));
}

Vector2D GetScreenSpacePinCoordinates(const ImNodesEditorContext& editor, const ImPinData& pin)
{
    const Rect& parent_node_rect = editor.Nodes.Pool[pin.parent_node_idx].Rect;
    return GetScreenSpacePinCoordinates(parent_node_rect, pin.AttributeRect, pin.Type);
}

bool MouseInCanvas()
{
    // This flag should be true either when hovering or clicking something in the canvas.
    let is_window_hovered_or_focused = ImGui::is_window_hovered() || ImGui::IsWindowFocused();

    return is_window_hovered_or_focused &&
           GImNodes.CanvasRectScreenSpace.contains(ImGui::GetMousePos());
}

void BeginNodeSelection(ImNodesEditorContext& editor, let node_idx)
{
    // Don't start selecting a node if we are e.g. already creating and dragging
    // a new link! New link creation can happen when the mouse is clicked over
    // a node, but within the hover radius of a pin.
    if (editor.ClickInteraction.Type != ImNodesClickInteractionType_None)
    {
        return;
    }

    editor.ClickInteraction.Type = ImNodesClickInteractionType_Node;
    // If the node is not already contained in the selection, then we want only
    // the interaction node to be selected, effective immediately.
    //
    // If the multiple selection modifier is active, we want to add this node
    // to the current list of selected nodes.
    //
    // Otherwise, we want to allow for the possibility of multiple nodes to be
    // moved at once.
    if (!editor.SelectedNodeIndices.contains(node_idx))
    {
        editor.SelectedLinkIndices.clear();
        if (!GImNodes.MultipleSelectModifier)
        {
            editor.SelectedNodeIndices.clear();
        }
        editor.SelectedNodeIndices.push_back(node_idx);

        // Ensure that individually selected nodes get rendered on top
        ImVector<int>&   depth_stack = editor.NodeDepthOrder;
        let* const elem = depth_stack.find(node_idx);
        // IM_ASSERT(elem != depth_stack.end());
        depth_stack.erase(elem);
        depth_stack.push_back(node_idx);
    }
    // Deselect a previously-selected node
    else if (GImNodes.MultipleSelectModifier)
    {
        let* const node_ptr = editor.SelectedNodeIndices.find(node_idx);
        editor.SelectedNodeIndices.erase(node_ptr);

        // Don't allow dragging after deselecting
        editor.ClickInteraction.Type = ImNodesClickInteractionType_None;
    }

    // To support snapping of multiple nodes, we need to store the offset of
    // each node in the selection to the origin of the dragged node.
    const Vector2D ref_origin = editor.Nodes.Pool[node_idx].Origin;
    editor.PrimaryNodeOffset =
        ref_origin + GImNodes.CanvasOriginScreenSpace + editor.Panning - GImNodes.MousePos;

    editor.SelectedNodeOffsets.clear();
    for (int idx = 0; idx < editor.SelectedNodeIndices.size; idx += 1)
    {
        let    node = editor.SelectedNodeIndices[idx];
        const Vector2D node_origin = editor.Nodes.Pool[node].Origin - ref_origin;
        editor.SelectedNodeOffsets.push_back(node_origin);
    }
}

void BeginLinkSelection(ImNodesEditorContext& editor, let link_idx)
{
    editor.ClickInteraction.Type = ImNodesClickInteractionType_Link;
    // When a link is selected, clear all other selections, and insert the link
    // as the sole selection.
    editor.SelectedNodeIndices.clear();
    editor.SelectedLinkIndices.clear();
    editor.SelectedLinkIndices.push_back(link_idx);
}

void BeginLinkDetach(ImNodesEditorContext& editor, let link_idx, let detach_pin_idx)
{
    const ImLinkData&        link = editor.Links.Pool[link_idx];
    ImClickInteractionState& state = editor.ClickInteraction;
    state.Type = ImNodesClickInteractionType_LinkCreation;
    state.LinkCreation.EndPinIdx.Reset();
    state.LinkCreation.StartPinIdx =
        detach_pin_idx == link.StartPinIdx ? link.EndPinIdx : link.StartPinIdx;
    GImNodes.DeletedLinkIdx = link_idx;
}

void BeginLinkCreation(ImNodesEditorContext& editor, let hovered_pin_idx)
{
    editor.ClickInteraction.Type = ImNodesClickInteractionType_LinkCreation;
    editor.ClickInteraction.LinkCreation.StartPinIdx = hovered_pin_idx;
    editor.ClickInteraction.LinkCreation.EndPinIdx.Reset();
    editor.ClickInteraction.LinkCreation.Type = ImNodesLinkCreationType_Standard;
    GImNodes.ImNodesUIState |= ImNodesUIState_LinkStarted;
}

void BeginLinkInteraction(
    ImNodesEditorContext& editor,
    let             link_idx,
    const ImOptionalIndex pin_idx = ImOptionalIndex())
{
    // Check if we are clicking the link with the modifier pressed.
    // This will in a link detach via clicking.

    let modifier_pressed = GImNodes.Io.LinkDetachWithModifierClick.Modifier == None
                                      ? false
                                      : *GImNodes.Io.LinkDetachWithModifierClick.Modifier;

    if (modifier_pressed)
    {
        const ImLinkData& link = editor.Links.Pool[link_idx];
        const ImPinData&  start_pin = editor.Pins.Pool[link.StartPinIdx];
        const ImPinData&  end_pin = editor.Pins.Pool[link.EndPinIdx];
        const Vector2D&     mouse_pos = GImNodes.MousePos;
        let       dist_to_start = ImLengthSqr(start_pin.pos - mouse_pos);
        let       dist_to_end = ImLengthSqr(end_pin.pos - mouse_pos);
        let closest_pin_idx = dist_to_start < dist_to_end ? link.StartPinIdx : link.EndPinIdx;

        editor.ClickInteraction.Type = ImNodesClickInteractionType_LinkCreation;
        BeginLinkDetach(editor, link_idx, closest_pin_idx);
        editor.ClickInteraction.LinkCreation.Type = ImNodesLinkCreationType_FromDetach;
    }
    else
    {
        if (pin_idx.HasValue())
        {
            let hovered_pin_flags = editor.Pins.Pool[pin_idx.Value()].flags;

            // Check the 'click and drag to detach' case.
            if (hovered_pin_flags & ImNodesAttributeFlags_EnableLinkDetachWithDragClick)
            {
                BeginLinkDetach(editor, link_idx, pin_idx.Value());
                editor.ClickInteraction.LinkCreation.Type = ImNodesLinkCreationType_FromDetach;
            }
            else
            {
                BeginLinkCreation(editor, pin_idx.Value());
            }
        }
        else
        {
            BeginLinkSelection(editor, link_idx);
        }
    }
}

static inline bool IsMiniMapHovered();

void BeginCanvasInteraction(ImNodesEditorContext& editor)
{
    let any_ui_element_hovered =
        GImNodes.HoveredNodeIdx.HasValue() || GImNodes.HoveredLinkIdx.HasValue() ||
        GImNodes.HoveredPinIdx.HasValue() || ImGui::is_any_item_hovered();

    let mouse_not_in_canvas = !MouseInCanvas();

    if (editor.ClickInteraction.Type != ImNodesClickInteractionType_None ||
        any_ui_element_hovered || mouse_not_in_canvas)
    {
        return;
    }

    let started_panning = GImNodes.AltMouseClicked;

    if (started_panning)
    {
        editor.ClickInteraction.Type = ImNodesClickInteractionType_Panning;
    }
    else if (GImNodes.LeftMouseClicked)
    {
        editor.ClickInteraction.Type = ImNodesClickInteractionType_BoxSelection;
        editor.ClickInteraction.BoxSelector.Rect.min =
            ScreenSpaceToGridSpace(editor, GImNodes.MousePos);
    }
}

void BoxSelectorUpdateSelection(ImNodesEditorContext& editor, Rect box_rect)
{
    // Invert box selector coordinates as needed

    if (box_rect.min.x > box_rect.max.x)
    {
        ImSwap(box_rect.min.x, box_rect.max.x);
    }

    if (box_rect.min.y > box_rect.max.y)
    {
        ImSwap(box_rect.min.y, box_rect.max.y);
    }

    // update node selection

    editor.SelectedNodeIndices.clear();

    // Test for overlap against node rectangles

    for (int node_idx = 0; node_idx < editor.Nodes.Pool.size();  += 1node_idx)
    {
        if (editor.Nodes.InUse[node_idx])
        {
            ImNodeData& node = editor.Nodes.Pool[node_idx];
            if (box_rect.Overlaps(node.Rect))
            {
                editor.SelectedNodeIndices.push_back(node_idx);
            }
        }
    }

    // update link selection

    editor.SelectedLinkIndices.clear();

    // Test for overlap against links

    for (int link_idx = 0; link_idx < editor.Links.Pool.size();  += 1link_idx)
    {
        if (editor.Links.InUse[link_idx])
        {
            const ImLinkData& link = editor.Links.Pool[link_idx];

            const ImPinData& pin_start = editor.Pins.Pool[link.StartPinIdx];
            const ImPinData& pin_end = editor.Pins.Pool[link.EndPinIdx];
            const Rect&    node_start_rect = editor.Nodes.Pool[pin_start.parent_node_idx].Rect;
            const Rect&    node_end_rect = editor.Nodes.Pool[pin_end.parent_node_idx].Rect;

            const Vector2D start = GetScreenSpacePinCoordinates(
                node_start_rect, pin_start.AttributeRect, pin_start.Type);
            const Vector2D end =
                GetScreenSpacePinCoordinates(node_end_rect, pin_end.AttributeRect, pin_end.Type);

            // Test
            if (RectangleOverlapsLink(box_rect, start, end, pin_start.Type))
            {
                editor.SelectedLinkIndices.push_back(link_idx);
            }
        }
    }
}

Vector2D SnapOriginToGrid(Vector2D origin)
{
    if (GImNodes.Style.flags & NodesStyleFlags_GridSnapping)
    {
        let spacing = GImNodes.Style.GridSpacing;
        let spacing2 = spacing * 0.5;

        // Snap the origin to the nearest grid point in any direction
        let modx =  fmodf(fabsf(origin.x) + spacing2, spacing) - spacing2;
        let mody =  fmodf(fabsf(origin.y) + spacing2, spacing) - spacing2;
        origin.x += (origin.x < 0.f) ? modx : -modx;
        origin.y += (origin.y < 0.f) ? mody : -mody;
    }

    return origin;
}

void TranslateSelectedNodes(ImNodesEditorContext& editor)
{
    if (GImNodes.LeftMouseDragging)
    {
        // If we have grid snap enabled, don't start moving nodes until we've moved the mouse
        // slightly
        let shouldTranslate = (GImNodes.Style.flags & NodesStyleFlags_GridSnapping)
                                         ? ImGui::GetIO().mouse_drag_max_distance_sqr[0] > 5.0
                                         : true;

        const Vector2D origin = SnapOriginToGrid(
            GImNodes.MousePos - GImNodes.CanvasOriginScreenSpace - editor.Panning +
            editor.PrimaryNodeOffset);
        for (int i = 0; i < editor.SelectedNodeIndices.size();  += 1i)
        {
            const Vector2D node_rel = editor.SelectedNodeOffsets[i];
            let    node_idx = editor.SelectedNodeIndices[i];
            ImNodeData&  node = editor.Nodes.Pool[node_idx];
            if (node.Draggable && shouldTranslate)
            {
                node.Origin = origin + node_rel + editor.AutoPanningDelta;
            }
        }
    }
}

struct LinkPredicate
{
    bool operator()(const ImLinkData& lhs, const ImLinkData& rhs) const
    {
        // Do a unique compare by sorting the pins' addresses.
        // This catches duplicate links, whether they are in the
        // same direction or not.
        // Sorting by pin index should have the uniqueness guarantees as sorting
        // by id -- each unique id will get one slot in the link pool array.

        int lhs_start = lhs.StartPinIdx;
        int lhs_end = lhs.EndPinIdx;
        int rhs_start = rhs.StartPinIdx;
        int rhs_end = rhs.EndPinIdx;

        if (lhs_start > lhs_end)
        {
            ImSwap(lhs_start, lhs_end);
        }

        if (rhs_start > rhs_end)
        {
            ImSwap(rhs_start, rhs_end);
        }

        return lhs_start == rhs_start && lhs_end == rhs_end;
    }
};

ImOptionalIndex FindDuplicateLink(
    const ImNodesEditorContext& editor,
    let                   start_pin_idx,
    let                   end_pin_idx)
{
    ImLinkData test_link(0);
    test_link.StartPinIdx = start_pin_idx;
    test_link.EndPinIdx = end_pin_idx;
    for (int link_idx = 0; link_idx < editor.Links.Pool.size();  += 1link_idx)
    {
        const ImLinkData& link = editor.Links.Pool[link_idx];
        if (LinkPredicate()(test_link, link) && editor.Links.InUse[link_idx])
        {
            return ImOptionalIndex(link_idx);
        }
    }

    return ImOptionalIndex();
}

bool ShouldLinkSnapToPin(
    const ImNodesEditorContext& editor,
    const ImPinData&            start_pin,
    let                   hovered_pin_idx,
    const ImOptionalIndex       duplicate_link)
{
    const ImPinData& end_pin = editor.Pins.Pool[hovered_pin_idx];

    // The end pin must be in a different node
    if (start_pin.parent_node_idx == end_pin.parent_node_idx)
    {
        return false;
    }

    // The end pin must be of a different type
    if (start_pin.Type == end_pin.Type)
    {
        return false;
    }

    // The link to be created must not be a duplicate, unless it is the link which was created on
    // snap. In that case we want to snap, since we want it to appear visually as if the created
    // link remains snapped to the pin.
    if (duplicate_link.HasValue() && !(duplicate_link == GImNodes.SnapLinkIdx))
    {
        return false;
    }

    return true;
}

void ClickInteractionUpdate(ImNodesEditorContext& editor)
{
    switch (editor.ClickInteraction.Type)
    {
    case ImNodesClickInteractionType_BoxSelection:
    {
        editor.ClickInteraction.BoxSelector.Rect.max =
            ScreenSpaceToGridSpace(editor, GImNodes.MousePos);

        Rect box_rect = editor.ClickInteraction.BoxSelector.Rect;
        box_rect.min = GridSpaceToScreenSpace(editor, box_rect.min);
        box_rect.max = GridSpaceToScreenSpace(editor, box_rect.max);

        BoxSelectorUpdateSelection(editor, box_rect);

        const ImU32 box_selector_color = GImNodes.Style.colors[ImNodesCol_BoxSelector];
        const ImU32 box_selector_outline = GImNodes.Style.colors[ImNodesCol_BoxSelectorOutline];
        GImNodes.CanvasDrawList.add_rect_filled(box_rect.min, box_rect.max, box_selector_color);
        GImNodes.CanvasDrawList.add_rect(box_rect.min, box_rect.max, box_selector_outline);

        if (GImNodes.LeftMouseReleased)
        {
            ImVector<int>&       depth_stack = editor.NodeDepthOrder;
            const ImVector<int>& selected_idxs = editor.SelectedNodeIndices;

            // Bump the selected node indices, in order, to the top of the depth stack.
            // NOTE: this algorithm has worst case time complexity of O(N^2), if the node selection
            // is ~ N (due to selected_idxs.contains()).

            if ((selected_idxs.size > 0) && (selected_idxs.size < depth_stack.size))
            {
                int num_moved = 0; // The number of indices moved. Stop after selected_idxs.size
                for (int i = 0; i < depth_stack.size - selected_idxs.size;  += 1i)
                {
                    for (int node_idx = depth_stack[i]; selected_idxs.contains(node_idx);
                         node_idx = depth_stack[i])
                    {
                        depth_stack.erase(depth_stack.begin() + static_cast<size_t>(i));
                        depth_stack.push_back(node_idx);
                         += 1num_moved;
                    }

                    if (num_moved == selected_idxs.size)
                    {
                        break;
                    }
                }
            }

            editor.ClickInteraction.Type = ImNodesClickInteractionType_None;
        }
    }
    break;
    case ImNodesClickInteractionType_Node:
    {
        TranslateSelectedNodes(editor);

        if (GImNodes.LeftMouseReleased)
        {
            editor.ClickInteraction.Type = ImNodesClickInteractionType_None;
        }
    }
    break;
    case ImNodesClickInteractionType_Link:
    {
        if (GImNodes.LeftMouseReleased)
        {
            editor.ClickInteraction.Type = ImNodesClickInteractionType_None;
        }
    }
    break;
    case ImNodesClickInteractionType_LinkCreation:
    {
        const ImPinData& start_pin =
            editor.Pins.Pool[editor.ClickInteraction.LinkCreation.StartPinIdx];

        const ImOptionalIndex maybe_duplicate_link_idx =
            GImNodes.HoveredPinIdx.HasValue()
                ? FindDuplicateLink(
                      editor,
                      editor.ClickInteraction.LinkCreation.StartPinIdx,
                      GImNodes.HoveredPinIdx.Value())
                : ImOptionalIndex();

        let should_snap =
            GImNodes.HoveredPinIdx.HasValue() &&
            ShouldLinkSnapToPin(
                editor, start_pin, GImNodes.HoveredPinIdx.Value(), maybe_duplicate_link_idx);

        // If we created on snap and the hovered pin is empty or changed, then we need signal that
        // the link's state has changed.
        let snapping_pin_changed =
            editor.ClickInteraction.LinkCreation.EndPinIdx.HasValue() &&
            !(GImNodes.HoveredPinIdx == editor.ClickInteraction.LinkCreation.EndPinIdx);

        // Detach the link that was created by this link event if it's no longer in snap range
        if (snapping_pin_changed && GImNodes.SnapLinkIdx.HasValue())
        {
            BeginLinkDetach(
                editor,
                GImNodes.SnapLinkIdx.Value(),
                editor.ClickInteraction.LinkCreation.EndPinIdx.Value());
        }

        const Vector2D start_pos = GetScreenSpacePinCoordinates(editor, start_pin);
        // If we are within the hover radius of a receiving pin, snap the link
        // endpoint to it
        const Vector2D end_pos = should_snap
                                   ? GetScreenSpacePinCoordinates(
                                         editor, editor.Pins.Pool[GImNodes.HoveredPinIdx.Value()])
                                   : GImNodes.MousePos;

        const CubicBezier cubic_bezier = GetCubicBezier(
            start_pos, end_pos, start_pin.Type, GImNodes.Style.LinkLineSegmentsPerLength);
// #ifIMGUI_VERSION_NUM < 18000
        GImNodes.CanvasDrawList.AddBezierCurve(
#else
        GImNodes.CanvasDrawList.AddBezierCubic(

            cubic_bezier.P0,
            cubic_bezier.P1,
            cubic_bezier.P2,
            cubic_bezier.P3,
            GImNodes.Style.colors[ImNodesCol_Link],
            GImNodes.Style.LinkThickness,
            cubic_bezier.NumSegments);

        let link_creation_on_snap =
            GImNodes.HoveredPinIdx.HasValue() &&
            (editor.Pins.Pool[GImNodes.HoveredPinIdx.Value()].flags &
             ImNodesAttributeFlags_EnableLinkCreationOnSnap);

        if (!should_snap)
        {
            editor.ClickInteraction.LinkCreation.EndPinIdx.Reset();
        }

        let create_link =
            should_snap && (GImNodes.LeftMouseReleased || link_creation_on_snap);

        if (create_link && !maybe_duplicate_link_idx.HasValue())
        {
            // Avoid send OnLinkCreated() events every frame if the snap link is not saved
            // (only applies for EnableLinkCreationOnSnap)
            if (!GImNodes.LeftMouseReleased &&
                editor.ClickInteraction.LinkCreation.EndPinIdx == GImNodes.HoveredPinIdx)
            {
                break;
            }

            GImNodes.ImNodesUIState |= ImNodesUIState_LinkCreated;
            editor.ClickInteraction.LinkCreation.EndPinIdx = GImNodes.HoveredPinIdx.Value();
        }

        if (GImNodes.LeftMouseReleased)
        {
            editor.ClickInteraction.Type = ImNodesClickInteractionType_None;
            if (!create_link)
            {
                GImNodes.ImNodesUIState |= ImNodesUIState_LinkDropped;
            }
        }
    }
    break;
    case ImNodesClickInteractionType_Panning:
    {
        let dragging = GImNodes.AltMouseDragging;

        if (dragging)
        {
            editor.Panning += ImGui::GetIO().mouse_delta;
        }
        else
        {
            editor.ClickInteraction.Type = ImNodesClickInteractionType_None;
        }
    }
    break;
    case ImNodesClickInteractionType_ImGuiItem:
    {
        if (GImNodes.LeftMouseReleased)
        {
            editor.ClickInteraction.Type = ImNodesClickInteractionType_None;
        }
    }
    case ImNodesClickInteractionType_None:
        break;
    default:
        // IM_ASSERT(!"Unreachable code!");
        break;
    }
}

void ResolveOccludedPins(const ImNodesEditorContext& editor, ImVector<int>& occluded_pin_indices)
{
    const ImVector<int>& depth_stack = editor.NodeDepthOrder;

    occluded_pin_indices.resize(0);

    if (depth_stack.size < 2)
    {
        return;
    }

    // For each node in the depth stack
    for (int depth_idx = 0; depth_idx < (depth_stack.size - 1);  += 1depth_idx)
    {
        const ImNodeData& node_below = editor.Nodes.Pool[depth_stack[depth_idx]];

        // Iterate over the rest of the depth stack to find nodes overlapping the pins
        for (int next_depth_idx = depth_idx + 1; next_depth_idx < depth_stack.size;
              += 1next_depth_idx)
        {
            const Rect& rect_above = editor.Nodes.Pool[depth_stack[next_depth_idx]].Rect;

            // Iterate over each pin
            for (int idx = 0; idx < node_below.PinIndices.size;  += 1idx)
            {
                let     pin_idx = node_below.PinIndices[idx];
                const Vector2D& pin_pos = editor.Pins.Pool[pin_idx].pos;

                if (rect_above.contains(pin_pos))
                {
                    occluded_pin_indices.push_back(pin_idx);
                }
            }
        }
    }
}

ImOptionalIndex ResolveHoveredPin(
    const ImObjectPool<ImPinData>& pins,
    const ImVector<int>&           occluded_pin_indices)
{
    let smallest_distance =  f32::MAX;
    ImOptionalIndex pin_idx_with_smallest_distance;

    let hover_radius_sqr = GImNodes.Style.PinHoverRadius * GImNodes.Style.PinHoverRadius;

    for (int idx = 0; idx < pins.Pool.size;  += 1idx)
    {
        if (!pins.InUse[idx])
        {
            continue;
        }

        if (occluded_pin_indices.contains(idx))
        {
            continue;
        }

        const Vector2D& pin_pos = pins.Pool[idx].pos;
        let   distance_sqr = ImLengthSqr(pin_pos - GImNodes.MousePos);

        // TODO: GImNodes->style.PinHoverRadius needs to be copied into pin data and the pin-local
        // value used here. This is no longer called in BeginAttribute/EndAttribute scope and the
        // detected pin might have a different hover radius than what the user had when calling
        // BeginAttribute/EndAttribute.
        if (distance_sqr < hover_radius_sqr && distance_sqr < smallest_distance)
        {
            smallest_distance = distance_sqr;
            pin_idx_with_smallest_distance = idx;
        }
    }

    return pin_idx_with_smallest_distance;
}

ImOptionalIndex ResolveHoveredNode(const ImVector<int>& depth_stack)
{
    if (GImNodes.NodeIndicesOverlappingWithMouse.size() == 0)
    {
        return ImOptionalIndex();
    }

    if (GImNodes.NodeIndicesOverlappingWithMouse.size() == 1)
    {
        return ImOptionalIndex(GImNodes.NodeIndicesOverlappingWithMouse[0]);
    }

    int largest_depth_idx = -1;
    int node_idx_on_top = -1;

    for (int i = 0; i < GImNodes.NodeIndicesOverlappingWithMouse.size();  += 1i)
    {
        let node_idx = GImNodes.NodeIndicesOverlappingWithMouse[i];
        for (int depth_idx = 0; depth_idx < depth_stack.size();  += 1depth_idx)
        {
            if (depth_stack[depth_idx] == node_idx && (depth_idx > largest_depth_idx))
            {
                largest_depth_idx = depth_idx;
                node_idx_on_top = node_idx;
            }
        }
    }

    // IM_ASSERT(node_idx_on_top != -1);
    return ImOptionalIndex(node_idx_on_top);
}

ImOptionalIndex ResolveHoveredLink(
    const ImObjectPool<ImLinkData>& links,
    const ImObjectPool<ImPinData>&  pins)
{
    let smallest_distance =  f32::MAX;
    ImOptionalIndex link_idx_with_smallest_distance;

    // There are two ways a link can be detected as "hovered".
    // 1. The link is within hover distance to the mouse. The closest such link is selected as being
    // hovered over.
    // 2. If the link is connected to the currently hovered pin.
    //
    // The latter is a requirement for link detaching with drag click to work, as both a link and
    // pin are required to be hovered over for the feature to work.

    for (int idx = 0; idx < links.Pool.size;  += 1idx)
    {
        if (!links.InUse[idx])
        {
            continue;
        }

        const ImLinkData& link = links.Pool[idx];
        const ImPinData&  start_pin = pins.Pool[link.StartPinIdx];
        const ImPinData&  end_pin = pins.Pool[link.EndPinIdx];

        // If there is a hovered pin links can only be considered hovered if they use that pin
        if (GImNodes.HoveredPinIdx.HasValue())
        {
            if (GImNodes.HoveredPinIdx == link.StartPinIdx ||
                GImNodes.HoveredPinIdx == link.EndPinIdx)
            {
                return idx;
            }
            continue;
        }

        // TODO: the calculated CubicBeziers could be cached since we generate them again when
        // rendering the links

        const CubicBezier cubic_bezier = GetCubicBezier(
            start_pin.pos, end_pin.pos, start_pin.Type, GImNodes.Style.LinkLineSegmentsPerLength);

        // The distance test
        {
            const Rect link_rect = GetContainingRectForCubicBezier(cubic_bezier);

            // First, do a simple bounding box test against the box containing the link
            // to see whether calculating the distance to the link is worth doing.
            if (link_rect.contains(GImNodes.MousePos))
            {
                let distance = GetDistanceToCubicBezier(
                    GImNodes.MousePos, cubic_bezier, cubic_bezier.NumSegments);

                // TODO: GImNodes->style.LinkHoverDistance could be also copied into ImLinkData,
                // since we're not calling this function in the same scope as ImNodes::Link(). The
                // rendered/detected link might have a different hover distance than what the user
                // had specified when calling Link()
                if (distance < GImNodes.Style.LinkHoverDistance && distance < smallest_distance)
                {
                    smallest_distance = distance;
                    link_idx_with_smallest_distance = idx;
                }
            }
        }
    }

    return link_idx_with_smallest_distance;
}

// [SECTION] render helpers

inline Rect GetItemRect() { return Rect(ImGui::GetItemRectMin(), ImGui::GetItemRectMax()); }

inline Vector2D GetNodeTitleBarOrigin(const ImNodeData& node)
{
    return node.Origin + node.LayoutStyle.Padding;
}

inline Vector2D GetNodeContentOrigin(const ImNodeData& node)
{
    const Vector2D title_bar_height =
        Vector2D::new(0.f, node.TitleBarContentRect.get_height() + 2.0 * node.LayoutStyle.Padding.y);
    return node.Origin + title_bar_height + node.LayoutStyle.Padding;
}

inline Rect GetNodeTitleRect(const ImNodeData& node)
{
    Rect expanded_title_rect = node.TitleBarContentRect;
    expanded_title_rect.Expand(node.LayoutStyle.Padding);

    return Rect(
        expanded_title_rect.min,
        expanded_title_rect.min + Vector2D::new(node.Rect.get_width(), 0.f) +
            Vector2D::new(0.f, expanded_title_rect.get_height()));
}

void DrawGrid(ImNodesEditorContext& editor, const Vector2D& canvas_size)
{
    const Vector2D offset = editor.Panning;
    ImU32        line_color = GImNodes.Style.colors[ImNodesCol_GridLine];
    ImU32        line_color_prim = GImNodes.Style.colors[ImNodesCol_GridLinePrimary];
    bool         draw_primary = GImNodes.Style.flags & NodesStyleFlags_GridLinesPrimary;

    for (let x =  fmodf(offset.x, GImNodes.Style.GridSpacing); x < canvas_size.x;
         x += GImNodes.Style.GridSpacing)
    {
        GImNodes.CanvasDrawList.add_line(
            EditorSpaceToScreenSpace(Vector2D::new(x, 0.0)),
            EditorSpaceToScreenSpace(Vector2D::new(x, canvas_size.y)),
            offset.x - x == 0.f && draw_primary ? line_color_prim : line_color);
    }

    for (let y =  fmodf(offset.y, GImNodes.Style.GridSpacing); y < canvas_size.y;
         y += GImNodes.Style.GridSpacing)
    {
        GImNodes.CanvasDrawList.add_line(
            EditorSpaceToScreenSpace(Vector2D::new(0.0, y)),
            EditorSpaceToScreenSpace(Vector2D::new(canvas_size.x, y)),
            offset.y - y == 0.f && draw_primary ? line_color_prim : line_color);
    }
}

struct QuadOffsets
{
    Vector2D TopLeft, BottomLeft, BottomRight, TopRight;
};

QuadOffsets CalculateQuadOffsets(let side_length)
{
    let half_side = 0.5 * side_length;

    QuadOffsets offset;

    offset.TopLeft = Vector2D::new(-half_side, half_side);
    offset.BottomLeft = Vector2D::new(-half_side, -half_side);
    offset.BottomRight = Vector2D::new(half_side, -half_side);
    offset.TopRight = Vector2D::new(half_side, half_side);

    return offset;
}

struct TriangleOffsets
{
    Vector2D TopLeft, BottomLeft, Right;
};

TriangleOffsets CalculateTriangleOffsets(let side_length)
{
    // Calculates the Vec2 offsets from an equilateral triangle's midpoint to
    // its vertices. Here is how the left_offset and right_offset are
    // calculated.
    //
    // For an equilateral triangle of side length s, the
    // triangle's height, h, is h = s * sqrt(3) / 2.
    //
    // The length from the base to the midpoint is (1 / 3) * h. The length from
    // the midpoint to the triangle vertex is (2 / 3) * h.
    let sqrt_3 = sqrtf(3.0);
    let left_offset = -0.1666666666667 * sqrt_3 * side_length;
    let right_offset = 0.333333333333 * sqrt_3 * side_length;
    let vertical_offset = 0.5 * side_length;

    TriangleOffsets offset;
    offset.TopLeft = Vector2D::new(left_offset, vertical_offset);
    offset.BottomLeft = Vector2D::new(left_offset, -vertical_offset);
    offset.Right = Vector2D::new(right_offset, 0.f);

    return offset;
}

void DrawPinShape(const Vector2D& pin_pos, const ImPinData& pin, const ImU32 pin_color)
{
    static let CIRCLE_NUM_SEGMENTS = 8;

    switch (pin.Shape)
    {
    case ImNodesPinShape_Circle:
    {
        GImNodes.CanvasDrawList.AddCircle(
            pin_pos,
            GImNodes.Style.PinCircleRadius,
            pin_color,
            CIRCLE_NUM_SEGMENTS,
            GImNodes.Style.PinLineThickness);
    }
    break;
    case ImNodesPinShape_CircleFilled:
    {
        GImNodes.CanvasDrawList.AddCircleFilled(
            pin_pos, GImNodes.Style.PinCircleRadius, pin_color, CIRCLE_NUM_SEGMENTS);
    }
    break;
    case ImNodesPinShape_Quad:
    {
        const QuadOffsets offset = CalculateQuadOffsets(GImNodes.Style.PinQuadSideLength);
        GImNodes.CanvasDrawList.AddQuad(
            pin_pos + offset.TopLeft,
            pin_pos + offset.BottomLeft,
            pin_pos + offset.BottomRight,
            pin_pos + offset.TopRight,
            pin_color,
            GImNodes.Style.PinLineThickness);
    }
    break;
    case ImNodesPinShape_QuadFilled:
    {
        const QuadOffsets offset = CalculateQuadOffsets(GImNodes.Style.PinQuadSideLength);
        GImNodes.CanvasDrawList.AddQuadFilled(
            pin_pos + offset.TopLeft,
            pin_pos + offset.BottomLeft,
            pin_pos + offset.BottomRight,
            pin_pos + offset.TopRight,
            pin_color);
    }
    break;
    case ImNodesPinShape_Triangle:
    {
        const TriangleOffsets offset =
            CalculateTriangleOffsets(GImNodes.Style.PinTriangleSideLength);
        GImNodes.CanvasDrawList.AddTriangle(
            pin_pos + offset.TopLeft,
            pin_pos + offset.BottomLeft,
            pin_pos + offset.Right,
            pin_color,
            // NOTE: for some weird reason, the line drawn by add_triangle is
            // much thinner than the lines drawn by add_circle or add_quad.
            // Multiplying the line thickness by two seemed to solve the
            // problem at a few different thickness values.
            2.f * GImNodes.Style.PinLineThickness);
    }
    break;
    case ImNodesPinShape_TriangleFilled:
    {
        const TriangleOffsets offset =
            CalculateTriangleOffsets(GImNodes.Style.PinTriangleSideLength);
        GImNodes.CanvasDrawList.add_triangle_filled(
            pin_pos + offset.TopLeft,
            pin_pos + offset.BottomLeft,
            pin_pos + offset.Right,
            pin_color);
    }
    break;
    default:
        // IM_ASSERT(!"Invalid PinShape value!");
        break;
    }
}

void DrawPin(ImNodesEditorContext& editor, let pin_idx)
{
    ImPinData&    pin = editor.Pins.Pool[pin_idx];
    const Rect& parent_node_rect = editor.Nodes.Pool[pin.parent_node_idx].Rect;

    pin.pos = GetScreenSpacePinCoordinates(parent_node_rect, pin.AttributeRect, pin.Type);

    ImU32 pin_color = pin.ColorStyle.Background;

    if (GImNodes.HoveredPinIdx == pin_idx)
    {
        pin_color = pin.ColorStyle.Hovered;
    }

    DrawPinShape(pin.pos, pin, pin_color);
}

void DrawNode(ImNodesEditorContext& editor, let node_idx)
{
    const ImNodeData& node = editor.Nodes.Pool[node_idx];
    ImGui::SetCursorPos(node.Origin + editor.Panning);

    let node_hovered =
        GImNodes.HoveredNodeIdx == node_idx &&
        editor.ClickInteraction.Type != ImNodesClickInteractionType_BoxSelection;

    ImU32 node_background = node.ColorStyle.Background;
    ImU32 titlebar_background = node.ColorStyle.Titlebar;

    if (editor.SelectedNodeIndices.contains(node_idx))
    {
        node_background = node.ColorStyle.BackgroundSelected;
        titlebar_background = node.ColorStyle.TitlebarSelected;
    }
    else if (node_hovered)
    {
        node_background = node.ColorStyle.BackgroundHovered;
        titlebar_background = node.ColorStyle.TitlebarHovered;
    }

    {
        // node base
        GImNodes.CanvasDrawList.add_rect_filled(
            node.Rect.min, node.Rect.max, node_background, node.LayoutStyle.CornerRounding);

        // title bar:
        if (node.TitleBarContentRect.get_height() > 0.f)
        {
            Rect title_bar_rect = GetNodeTitleRect(node);

// #ifIMGUI_VERSION_NUM < 18200
            GImNodes.CanvasDrawList.add_rect_filled(
                title_bar_rect.min,
                title_bar_rect.max,
                titlebar_background,
                node.LayoutStyle.CornerRounding,
                ImDrawCornerFlags_Top);
#else
            GImNodes.CanvasDrawList.add_rect_filled(
                title_bar_rect.min,
                title_bar_rect.max,
                titlebar_background,
                node.LayoutStyle.CornerRounding,
                DrawFlags::RoundCornersTop);


        }

        if ((GImNodes.Style.flags & NodesStyleFlags_NodeOutline) != 0)
        {
// #ifIMGUI_VERSION_NUM < 18200
            GImNodes.CanvasDrawList.add_rect(
                node.Rect.min,
                node.Rect.max,
                node.ColorStyle.Outline,
                node.LayoutStyle.CornerRounding,
                ImDrawCornerFlags_All,
                node.LayoutStyle.BorderThickness);
#else
            GImNodes.CanvasDrawList.add_rect(
                node.Rect.min,
                node.Rect.max,
                node.ColorStyle.Outline,
                node.LayoutStyle.CornerRounding,
                DrawFlags::RoundCornersAll,
                node.LayoutStyle.BorderThickness);

        }
    }

    for (int i = 0; i < node.PinIndices.size();  += 1i)
    {
        DrawPin(editor, node.PinIndices[i]);
    }

    if (node_hovered)
    {
        GImNodes.HoveredNodeIdx = node_idx;
    }
}

void DrawLink(ImNodesEditorContext& editor, let link_idx)
{
    const ImLinkData& link = editor.Links.Pool[link_idx];
    const ImPinData&  start_pin = editor.Pins.Pool[link.StartPinIdx];
    const ImPinData&  end_pin = editor.Pins.Pool[link.EndPinIdx];

    const CubicBezier cubic_bezier = GetCubicBezier(
        start_pin.pos, end_pin.pos, start_pin.Type, GImNodes.Style.LinkLineSegmentsPerLength);

    let link_hovered =
        GImNodes.HoveredLinkIdx == link_idx &&
        editor.ClickInteraction.Type != ImNodesClickInteractionType_BoxSelection;

    if (link_hovered)
    {
        GImNodes.HoveredLinkIdx = link_idx;
    }

    // It's possible for a link to be deleted in begin_link_interaction. A user
    // may detach a link, resulting in the link wire snapping to the mouse
    // position.
    //
    // In other words, skip rendering the link if it was deleted.
    if (GImNodes.DeletedLinkIdx == link_idx)
    {
        return;
    }

    ImU32 link_color = link.ColorStyle.Base;
    if (editor.SelectedLinkIndices.contains(link_idx))
    {
        link_color = link.ColorStyle.Selected;
    }
    else if (link_hovered)
    {
        link_color = link.ColorStyle.Hovered;
    }

// #ifIMGUI_VERSION_NUM < 18000
    GImNodes.CanvasDrawList.AddBezierCurve(
#else
    GImNodes.CanvasDrawList.AddBezierCubic(

        cubic_bezier.P0,
        cubic_bezier.P1,
        cubic_bezier.P2,
        cubic_bezier.P3,
        link_color,
        GImNodes.Style.LinkThickness,
        cubic_bezier.NumSegments);
}

void BeginPinAttribute(
    let                  id,
    const ImNodesAttributeType type,
    const ImNodesPinShape      shape,
    let                  node_idx)
{
    // Make sure to call BeginNode() before calling
    // BeginAttribute()
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Node);
    GImNodes.CurrentScope = ImNodesScope_Attribute;

    ImGui::BeginGroup();
    ImGui::push_id(id);

    ImNodesEditorContext& editor = EditorContextGet();

    GImNodes.CurrentAttributeId = id;

    let pin_idx = ObjectPoolFindOrCreateIndex(editor.Pins, id);
    GImNodes.CurrentPinIdx = pin_idx;
    ImPinData& pin = editor.Pins.Pool[pin_idx];
    pin.Id = id;
    pin.parent_node_idx = node_idx;
    pin.Type = type;
    pin.Shape = shape;
    pin.flags = GImNodes.CurrentAttributeFlags;
    pin.ColorStyle.Background = GImNodes.Style.colors[ImNodesCol_Pin];
    pin.ColorStyle.Hovered = GImNodes.Style.colors[ImNodesCol_PinHovered];
}

void EndPinAttribute()
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Attribute);
    GImNodes.CurrentScope = ImNodesScope_Node;

    ImGui::pop_id();
    ImGui::EndGroup();

    if (ImGui::is_item_active())
    {
        GImNodes.ActiveAttribute = true;
        GImNodes.ActiveAttributeId = GImNodes.CurrentAttributeId;
    }

    ImNodesEditorContext& editor = EditorContextGet();
    ImPinData&            pin = editor.Pins.Pool[GImNodes.CurrentPinIdx];
    ImNodeData&           node = editor.Nodes.Pool[GImNodes.CurrentNodeIdx];
    pin.AttributeRect = GetItemRect();
    node.PinIndices.push_back(GImNodes.CurrentPinIdx);
}

void Initialize(ImNodesContext* context)
{
    context.CanvasOriginScreenSpace = Vector2D::new(0.0, 0.0);
    context.CanvasRectScreenSpace = Rect(Vector2D::new(0.f, 0.f), Vector2D::new(0.f, 0.f));
    context.CurrentScope = ImNodesScope_None;

    context.CurrentPinIdx = INT_MAX;
    context.CurrentNodeIdx = INT_MAX;

    context.DefaultEditorCtx = EditorContextCreate();
    context.EditorCtx = context.DefaultEditorCtx;

    context.CurrentAttributeFlags = ImNodesAttributeFlags_None;
    context.AttributeFlagStack.push_back(GImNodes.CurrentAttributeFlags);

    StyleColorsDark(&context.Style);
}

void Shutdown(ImNodesContext* g) { EditorContextFree(g.DefaultEditorCtx); }

// [SECTION] minimap

static inline bool IsMiniMapActive()
{
    ImNodesEditorContext& editor = EditorContextGet();
    return editor.MiniMapEnabled && editor.MiniMapSizeFraction > 0.0;
}

static inline bool IsMiniMapHovered()
{
    ImNodesEditorContext& editor = EditorContextGet();
    return IsMiniMapActive() &&
           ImGui::is_mouse_hovering_rect(
               editor.MiniMapRectScreenSpace.min, editor.MiniMapRectScreenSpace.max);
}

static inline void CalcMiniMapLayout()
{
    ImNodesEditorContext& editor = EditorContextGet();
    const Vector2D          offset = GImNodes.Style.MiniMapOffset;
    const Vector2D          border = GImNodes.Style.MiniMapPadding;
    const Rect          editor_rect = GImNodes.CanvasRectScreenSpace;

    // Compute the size of the mini-map area
    Vector2D mini_map_size;
    float  mini_map_scaling;
    {
        const Vector2D max_size =
            f32::floor(editor_rect.GetSize() * editor.MiniMapSizeFraction - border * 2.0);
        let  max_size_aspect_ratio = max_size.x / max_size.y;
        const Vector2D grid_content_size = editor.GridContentBounds.is_inverted()
                                             ? max_size
                                             : f32::floor(editor.GridContentBounds.GetSize());
        let grid_content_aspect_ratio = grid_content_size.x / grid_content_size.y;
        mini_map_size = f32::floor(
            grid_content_aspect_ratio > max_size_aspect_ratio
                ? Vector2D::new(max_size.x, max_size.x / grid_content_aspect_ratio)
                : Vector2D::new(max_size.y * grid_content_aspect_ratio, max_size.y));
        mini_map_scaling = mini_map_size.x / grid_content_size.x;
    }

    // Compute location of the mini-map
    Vector2D mini_map_pos;
    {
        Vector2D align;

        switch (editor.MiniMapLocation)
        {
        case ImNodesMiniMapLocation_BottomRight:
            align.x = 1.0;
            align.y = 1.0;
            break;
        case ImNodesMiniMapLocation_BottomLeft:
            align.x = 0.0;
            align.y = 1.0;
            break;
        case ImNodesMiniMapLocation_TopRight:
            align.x = 1.0;
            align.y = 0.0;
            break;
        case ImNodesMiniMapLocation_TopLeft: // [[fallthrough]]
        default:
            align.x = 0.0;
            align.y = 0.0;
            break;
        }

        const Vector2D top_left_pos = editor_rect.min + offset + border;
        const Vector2D bottom_right_pos = editor_rect.max - offset - border - mini_map_size;
        mini_map_pos = f32::floor(ImLerp(top_left_pos, bottom_right_pos, align));
    }

    editor.MiniMapRectScreenSpace =
        Rect(mini_map_pos - border, mini_map_pos + mini_map_size + border);
    editor.MiniMapContentScreenSpace = Rect(mini_map_pos, mini_map_pos + mini_map_size);
    editor.MiniMapScaling = mini_map_scaling;
}

static void MiniMapDrawNode(ImNodesEditorContext& editor, let node_idx)
{
    const ImNodeData& node = editor.Nodes.Pool[node_idx];

    const Rect node_rect = ScreenSpaceToMiniMapSpace(editor, node.Rect);

    // Round to near whole pixel value for corner-rounding to prevent visual glitches
    let mini_map_node_rounding =
        floorf(node.LayoutStyle.CornerRounding * editor.MiniMapScaling);

    ImU32 mini_map_node_background;

    if (editor.ClickInteraction.Type == ImNodesClickInteractionType_None &&
        ImGui::is_mouse_hovering_rect(node_rect.min, node_rect.max))
    {
        mini_map_node_background = GImNodes.Style.colors[ImNodesCol_MiniMapNodeBackgroundHovered];

        // Run user callback when hovering a mini-map node
        if (editor.MiniMapNodeHoveringCallback)
        {
            editor.MiniMapNodeHoveringCallback(node.Id, editor.MiniMapNodeHoveringCallbackUserData);
        }
    }
    else if (editor.SelectedNodeIndices.contains(node_idx))
    {
        mini_map_node_background = GImNodes.Style.colors[ImNodesCol_MiniMapNodeBackgroundSelected];
    }
    else
    {
        mini_map_node_background = GImNodes.Style.colors[ImNodesCol_MiniMapNodeBackground];
    }

    const ImU32 mini_map_node_outline = GImNodes.Style.colors[ImNodesCol_MiniMapNodeOutline];

    GImNodes.CanvasDrawList.add_rect_filled(
        node_rect.min, node_rect.max, mini_map_node_background, mini_map_node_rounding);

    GImNodes.CanvasDrawList.add_rect(
        node_rect.min, node_rect.max, mini_map_node_outline, mini_map_node_rounding);
}

static void MiniMapDrawLink(ImNodesEditorContext& editor, let link_idx)
{
    const ImLinkData& link = editor.Links.Pool[link_idx];
    const ImPinData&  start_pin = editor.Pins.Pool[link.StartPinIdx];
    const ImPinData&  end_pin = editor.Pins.Pool[link.EndPinIdx];

    const CubicBezier cubic_bezier = GetCubicBezier(
        ScreenSpaceToMiniMapSpace(editor, start_pin.pos),
        ScreenSpaceToMiniMapSpace(editor, end_pin.pos),
        start_pin.Type,
        GImNodes.Style.LinkLineSegmentsPerLength / editor.MiniMapScaling);

    // It's possible for a link to be deleted in begin_link_interaction. A user
    // may detach a link, resulting in the link wire snapping to the mouse
    // position.
    //
    // In other words, skip rendering the link if it was deleted.
    if (GImNodes.DeletedLinkIdx == link_idx)
    {
        return;
    }

    const ImU32 link_color =
        GImNodes.Style.colors
            [editor.SelectedLinkIndices.contains(link_idx) ? ImNodesCol_MiniMapLinkSelected
                                                           : ImNodesCol_MiniMapLink];

// #ifIMGUI_VERSION_NUM < 18000
    GImNodes.CanvasDrawList.AddBezierCurve(
#else
    GImNodes.CanvasDrawList.AddBezierCubic(

        cubic_bezier.P0,
        cubic_bezier.P1,
        cubic_bezier.P2,
        cubic_bezier.P3,
        link_color,
        GImNodes.Style.LinkThickness * editor.MiniMapScaling,
        cubic_bezier.NumSegments);
}

static void MiniMapUpdate()
{
    ImNodesEditorContext& editor = EditorContextGet();

    ImU32 mini_map_background;

    if (IsMiniMapHovered())
    {
        mini_map_background = GImNodes.Style.colors[ImNodesCol_MiniMapBackgroundHovered];
    }
    else
    {
        mini_map_background = GImNodes.Style.colors[ImNodesCol_MiniMapBackground];
    }

    // Create a child window bellow mini-map, so it blocks all mouse interaction on canvas.
    int flags = WindowFlags::NoBackground;
    ImGui::SetCursorScreenPos(editor.MiniMapRectScreenSpace.min);
    ImGui::begin_child("minimap", editor.MiniMapRectScreenSpace.GetSize(), false, flags);

    const Rect& mini_map_rect = editor.MiniMapRectScreenSpace;

    // Draw minimap background and border
    GImNodes.CanvasDrawList.add_rect_filled(
        mini_map_rect.min, mini_map_rect.max, mini_map_background);

    GImNodes.CanvasDrawList.add_rect(
        mini_map_rect.min, mini_map_rect.max, GImNodes.Style.colors[ImNodesCol_MiniMapOutline]);

    // Clip draw list items to mini-map rect (after drawing background/outline)
    GImNodes.CanvasDrawList.push_clip_rect(
        mini_map_rect.min, mini_map_rect.max, true /* intersect with editor clip-rect */);

    // Draw links first so they appear under nodes, and we can use the same draw channel
    for (int link_idx = 0; link_idx < editor.Links.Pool.size();  += 1link_idx)
    {
        if (editor.Links.InUse[link_idx])
        {
            MiniMapDrawLink(editor, link_idx);
        }
    }

    for (int node_idx = 0; node_idx < editor.Nodes.Pool.size();  += 1node_idx)
    {
        if (editor.Nodes.InUse[node_idx])
        {
            MiniMapDrawNode(editor, node_idx);
        }
    }

    // Draw editor canvas rect inside mini-map
    {
        const ImU32  canvas_color = GImNodes.Style.colors[ImNodesCol_MiniMapCanvas];
        const ImU32  outline_color = GImNodes.Style.colors[ImNodesCol_MiniMapCanvasOutline];
        const Rect rect = ScreenSpaceToMiniMapSpace(editor, GImNodes.CanvasRectScreenSpace);

        GImNodes.CanvasDrawList.add_rect_filled(rect.min, rect.max, canvas_color);
        GImNodes.CanvasDrawList.add_rect(rect.min, rect.max, outline_color);
    }

    // Have to pop mini-map clip rect
    GImNodes.CanvasDrawList.pop_clip_rect();

    bool mini_map_is_hovered = ImGui::is_window_hovered();

    ImGui::end_child();

    bool center_on_click = mini_map_is_hovered && ImGui::IsMouseDown(MouseButton::Left) &&
                           editor.ClickInteraction.Type == ImNodesClickInteractionType_None &&
                           !GImNodes.NodeIdxSubmissionOrder.empty();
    if (center_on_click)
    {
        Vector2D target = MiniMapSpaceToGridSpace(editor, ImGui::GetMousePos());
        Vector2D center = GImNodes.CanvasRectScreenSpace.GetSize() * 0.5;
        editor.Panning = f32::floor(center - target);
    }

    // Reset callback info after use
    editor.MiniMapNodeHoveringCallback = None;
    editor.MiniMapNodeHoveringCallbackUserData = None;
}

// [SECTION] selection helpers

template<typename T>
void SelectObject(const ImObjectPool<T>& objects, ImVector<int>& selected_indices, let id)
{
    let idx = ObjectPoolFind(objects, id);
    // IM_ASSERT(idx >= 0);
    // IM_ASSERT(selected_indices.find(idx) == selected_indices.end());
    selected_indices.push_back(idx);
}

template<typename T>
void ClearObjectSelection(
    const ImObjectPool<T>& objects,
    ImVector<int>&         selected_indices,
    let              id)
{
    let idx = ObjectPoolFind(objects, id);
    // IM_ASSERT(idx >= 0);
    // IM_ASSERT(selected_indices.find(idx) != selected_indices.end());
    selected_indices.find_erase_unsorted(idx);
}

template<typename T>
bool IsObjectSelected(const ImObjectPool<T>& objects, ImVector<int>& selected_indices, let id)
{
    let idx = ObjectPoolFind(objects, id);
    return selected_indices.find(idx) != selected_indices.end();
}

} // namespace
} // namespace IMNODES_NAMESPACE

// [SECTION] API implementation

ImNodesIO::EmulateThreeButtonMouse::EmulateThreeButtonMouse() : Modifier(None) {}

ImNodesIO::LinkDetachWithModifierClick::LinkDetachWithModifierClick() : Modifier(None) {}

ImNodesIO::MultipleSelectModifier::MultipleSelectModifier() : Modifier(None) {}

ImNodesIO::ImNodesIO()
    : EmulateThreeButtonMouse(), LinkDetachWithModifierClick(),
      AltMouseButton(MouseButton::Middle), AutoPanningSpeed(1000.0)
{
}

NodesStyle::NodesStyle()
    : GridSpacing(24.f), NodeCornerRounding(4.f), NodePadding(8.f, 8.f), NodeBorderThickness(1.f),
      LinkThickness(3.f), LinkLineSegmentsPerLength(0.1), LinkHoverDistance(10.f),
      PinCircleRadius(4.f), PinQuadSideLength(7.f), PinTriangleSideLength(9.5),
      PinLineThickness(1.f), PinHoverRadius(10.f), PinOffset(0.f), MiniMapPadding(8.0, 8.0),
      MiniMapOffset(4.0, 4.0), Flags(NodesStyleFlags_NodeOutline | NodesStyleFlags_GridLines),
      Colors()
{
}

namespace IMNODES_NAMESPACE
{
ImNodesContext* CreateContext()
{
    ImNodesContext* g = IM_NEW(ImNodesContext)();
    if (GImNodes == None)
        SetCurrentContext(g);
    Initialize(g);
    return g;
}

void DestroyContext(ImNodesContext* g)
{
    if (g == None)
        g = GImNodes;
    Shutdown(g);
    if (GImNodes == g)
        SetCurrentContext(None);
    IM_DELETE(g);
}

ImNodesContext* GetCurrentContext() { return GImNodes; }

void SetCurrentContext(ImNodesContext* g) { GImNodes = g; }

ImNodesEditorContext* EditorContextCreate()
{
    void* mem = ImGui::MemAlloc(sizeof(ImNodesEditorContext));
    new (mem) ImNodesEditorContext();
    return (ImNodesEditorContext*)mem;
}

void EditorContextFree(ImNodesEditorContext* g)
{
    g->~ImNodesEditorContext();
    ImGui::MemFree(g);
}

void EditorContextSet(ImNodesEditorContext* g) { GImNodes.EditorCtx = g; }

Vector2D EditorContextGetPanning()
{
    const ImNodesEditorContext& editor = EditorContextGet();
    return editor.Panning;
}

void EditorContextResetPanning(const Vector2D& pos)
{
    ImNodesEditorContext& editor = EditorContextGet();
    editor.Panning = pos;
}

void EditorContextMoveToNode(let node_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    ImNodeData&           node = ObjectPoolFindOrCreateObject(editor.Nodes, node_id);

    editor.Panning.x = -node.Origin.x;
    editor.Panning.y = -node.Origin.y;
}

void SetImGuiContext(ImGuiContext* g) { ImGui::SetCurrentContext(g); }

ImNodesIO& GetIO() { return GImNodes.Io; }

NodesStyle& GetStyle() { return GImNodes.Style; }

void StyleColorsDark(NodesStyle* dest)
{
    if (dest == nullptr)
    {
        dest = &GImNodes.Style;
    }

    dest.Colors[ImNodesCol_NodeBackground] = IM_COL32(50, 50, 50, 255);
    dest.Colors[ImNodesCol_NodeBackgroundHovered] = IM_COL32(75, 75, 75, 255);
    dest.Colors[ImNodesCol_NodeBackgroundSelected] = IM_COL32(75, 75, 75, 255);
    dest.Colors[ImNodesCol_NodeOutline] = IM_COL32(100, 100, 100, 255);
    // title bar colors match ImGui's titlebg colors
    dest.Colors[ImNodesCol_TitleBar] = IM_COL32(41, 74, 122, 255);
    dest.Colors[ImNodesCol_TitleBarHovered] = IM_COL32(66, 150, 250, 255);
    dest.Colors[ImNodesCol_TitleBarSelected] = IM_COL32(66, 150, 250, 255);
    // link colors match ImGui's slider grab colors
    dest.Colors[ImNodesCol_Link] = IM_COL32(61, 133, 224, 200);
    dest.Colors[ImNodesCol_LinkHovered] = IM_COL32(66, 150, 250, 255);
    dest.Colors[ImNodesCol_LinkSelected] = IM_COL32(66, 150, 250, 255);
    // pin colors match ImGui's button colors
    dest.Colors[ImNodesCol_Pin] = IM_COL32(53, 150, 250, 180);
    dest.Colors[ImNodesCol_PinHovered] = IM_COL32(53, 150, 250, 255);

    dest.Colors[ImNodesCol_BoxSelector] = IM_COL32(61, 133, 224, 30);
    dest.Colors[ImNodesCol_BoxSelectorOutline] = IM_COL32(61, 133, 224, 150);

    dest.Colors[ImNodesCol_GridBackground] = IM_COL32(40, 40, 50, 200);
    dest.Colors[ImNodesCol_GridLine] = IM_COL32(200, 200, 200, 40);
    dest.Colors[ImNodesCol_GridLinePrimary] = IM_COL32(240, 240, 240, 60);

    // minimap colors
    dest.Colors[ImNodesCol_MiniMapBackground] = IM_COL32(25, 25, 25, 150);
    dest.Colors[ImNodesCol_MiniMapBackgroundHovered] = IM_COL32(25, 25, 25, 200);
    dest.Colors[ImNodesCol_MiniMapOutline] = IM_COL32(150, 150, 150, 100);
    dest.Colors[ImNodesCol_MiniMapOutlineHovered] = IM_COL32(150, 150, 150, 200);
    dest.Colors[ImNodesCol_MiniMapNodeBackground] = IM_COL32(200, 200, 200, 100);
    dest.Colors[ImNodesCol_MiniMapNodeBackgroundHovered] = IM_COL32(200, 200, 200, 255);
    dest.Colors[ImNodesCol_MiniMapNodeBackgroundSelected] =
        dest.Colors[ImNodesCol_MiniMapNodeBackgroundHovered];
    dest.Colors[ImNodesCol_MiniMapNodeOutline] = IM_COL32(200, 200, 200, 100);
    dest.Colors[ImNodesCol_MiniMapLink] = dest.Colors[ImNodesCol_Link];
    dest.Colors[ImNodesCol_MiniMapLinkSelected] = dest.Colors[ImNodesCol_LinkSelected];
    dest.Colors[ImNodesCol_MiniMapCanvas] = IM_COL32(200, 200, 200, 25);
    dest.Colors[ImNodesCol_MiniMapCanvasOutline] = IM_COL32(200, 200, 200, 200);
}

void StyleColorsClassic(NodesStyle* dest)
{
    if (dest == nullptr)
    {
        dest = &GImNodes.Style;
    }

    dest.Colors[ImNodesCol_NodeBackground] = IM_COL32(50, 50, 50, 255);
    dest.Colors[ImNodesCol_NodeBackgroundHovered] = IM_COL32(75, 75, 75, 255);
    dest.Colors[ImNodesCol_NodeBackgroundSelected] = IM_COL32(75, 75, 75, 255);
    dest.Colors[ImNodesCol_NodeOutline] = IM_COL32(100, 100, 100, 255);
    dest.Colors[ImNodesCol_TitleBar] = IM_COL32(69, 69, 138, 255);
    dest.Colors[ImNodesCol_TitleBarHovered] = IM_COL32(82, 82, 161, 255);
    dest.Colors[ImNodesCol_TitleBarSelected] = IM_COL32(82, 82, 161, 255);
    dest.Colors[ImNodesCol_Link] = IM_COL32(255, 255, 255, 100);
    dest.Colors[ImNodesCol_LinkHovered] = IM_COL32(105, 99, 204, 153);
    dest.Colors[ImNodesCol_LinkSelected] = IM_COL32(105, 99, 204, 153);
    dest.Colors[ImNodesCol_Pin] = IM_COL32(89, 102, 156, 170);
    dest.Colors[ImNodesCol_PinHovered] = IM_COL32(102, 122, 179, 200);
    dest.Colors[ImNodesCol_BoxSelector] = IM_COL32(82, 82, 161, 100);
    dest.Colors[ImNodesCol_BoxSelectorOutline] = IM_COL32(82, 82, 161, 255);
    dest.Colors[ImNodesCol_GridBackground] = IM_COL32(40, 40, 50, 200);
    dest.Colors[ImNodesCol_GridLine] = IM_COL32(200, 200, 200, 40);
    dest.Colors[ImNodesCol_GridLinePrimary] = IM_COL32(240, 240, 240, 60);

    // minimap colors
    dest.Colors[ImNodesCol_MiniMapBackground] = IM_COL32(25, 25, 25, 100);
    dest.Colors[ImNodesCol_MiniMapBackgroundHovered] = IM_COL32(25, 25, 25, 200);
    dest.Colors[ImNodesCol_MiniMapOutline] = IM_COL32(150, 150, 150, 100);
    dest.Colors[ImNodesCol_MiniMapOutlineHovered] = IM_COL32(150, 150, 150, 200);
    dest.Colors[ImNodesCol_MiniMapNodeBackground] = IM_COL32(200, 200, 200, 100);
    dest.Colors[ImNodesCol_MiniMapNodeBackgroundSelected] =
        dest.Colors[ImNodesCol_MiniMapNodeBackgroundHovered];
    dest.Colors[ImNodesCol_MiniMapNodeBackgroundSelected] = IM_COL32(200, 200, 240, 255);
    dest.Colors[ImNodesCol_MiniMapNodeOutline] = IM_COL32(200, 200, 200, 100);
    dest.Colors[ImNodesCol_MiniMapLink] = dest.Colors[ImNodesCol_Link];
    dest.Colors[ImNodesCol_MiniMapLinkSelected] = dest.Colors[ImNodesCol_LinkSelected];
    dest.Colors[ImNodesCol_MiniMapCanvas] = IM_COL32(200, 200, 200, 25);
    dest.Colors[ImNodesCol_MiniMapCanvasOutline] = IM_COL32(200, 200, 200, 200);
}

void StyleColorsLight(NodesStyle* dest)
{
    if (dest == nullptr)
    {
        dest = &GImNodes.Style;
    }

    dest.Colors[ImNodesCol_NodeBackground] = IM_COL32(240, 240, 240, 255);
    dest.Colors[ImNodesCol_NodeBackgroundHovered] = IM_COL32(240, 240, 240, 255);
    dest.Colors[ImNodesCol_NodeBackgroundSelected] = IM_COL32(240, 240, 240, 255);
    dest.Colors[ImNodesCol_NodeOutline] = IM_COL32(100, 100, 100, 255);
    dest.Colors[ImNodesCol_TitleBar] = IM_COL32(248, 248, 248, 255);
    dest.Colors[ImNodesCol_TitleBarHovered] = IM_COL32(209, 209, 209, 255);
    dest.Colors[ImNodesCol_TitleBarSelected] = IM_COL32(209, 209, 209, 255);
    // original imgui values: 66, 150, 250
    dest.Colors[ImNodesCol_Link] = IM_COL32(66, 150, 250, 100);
    // original imgui values: 117, 138, 204
    dest.Colors[ImNodesCol_LinkHovered] = IM_COL32(66, 150, 250, 242);
    dest.Colors[ImNodesCol_LinkSelected] = IM_COL32(66, 150, 250, 242);
    // original imgui values: 66, 150, 250
    dest.Colors[ImNodesCol_Pin] = IM_COL32(66, 150, 250, 160);
    dest.Colors[ImNodesCol_PinHovered] = IM_COL32(66, 150, 250, 255);
    dest.Colors[ImNodesCol_BoxSelector] = IM_COL32(90, 170, 250, 30);
    dest.Colors[ImNodesCol_BoxSelectorOutline] = IM_COL32(90, 170, 250, 150);
    dest.Colors[ImNodesCol_GridBackground] = IM_COL32(225, 225, 225, 255);
    dest.Colors[ImNodesCol_GridLine] = IM_COL32(180, 180, 180, 100);
    dest.Colors[ImNodesCol_GridLinePrimary] = IM_COL32(120, 120, 120, 100);

    // minimap colors
    dest.Colors[ImNodesCol_MiniMapBackground] = IM_COL32(25, 25, 25, 100);
    dest.Colors[ImNodesCol_MiniMapBackgroundHovered] = IM_COL32(25, 25, 25, 200);
    dest.Colors[ImNodesCol_MiniMapOutline] = IM_COL32(150, 150, 150, 100);
    dest.Colors[ImNodesCol_MiniMapOutlineHovered] = IM_COL32(150, 150, 150, 200);
    dest.Colors[ImNodesCol_MiniMapNodeBackground] = IM_COL32(200, 200, 200, 100);
    dest.Colors[ImNodesCol_MiniMapNodeBackgroundSelected] =
        dest.Colors[ImNodesCol_MiniMapNodeBackgroundHovered];
    dest.Colors[ImNodesCol_MiniMapNodeBackgroundSelected] = IM_COL32(200, 200, 240, 255);
    dest.Colors[ImNodesCol_MiniMapNodeOutline] = IM_COL32(200, 200, 200, 100);
    dest.Colors[ImNodesCol_MiniMapLink] = dest.Colors[ImNodesCol_Link];
    dest.Colors[ImNodesCol_MiniMapLinkSelected] = dest.Colors[ImNodesCol_LinkSelected];
    dest.Colors[ImNodesCol_MiniMapCanvas] = IM_COL32(200, 200, 200, 25);
    dest.Colors[ImNodesCol_MiniMapCanvasOutline] = IM_COL32(200, 200, 200, 200);
}

void BeginNodeEditor()
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);
    GImNodes.CurrentScope = ImNodesScope_Editor;

    // Reset state from previous pass

    ImNodesEditorContext& editor = EditorContextGet();
    editor.AutoPanningDelta = Vector2D::new(0, 0);
    editor.GridContentBounds = Rect(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    editor.MiniMapEnabled = false;
    ObjectPoolReset(editor.Nodes);
    ObjectPoolReset(editor.Pins);
    ObjectPoolReset(editor.Links);

    GImNodes.HoveredNodeIdx.Reset();
    GImNodes.HoveredLinkIdx.Reset();
    GImNodes.HoveredPinIdx.Reset();
    GImNodes.DeletedLinkIdx.Reset();
    GImNodes.SnapLinkIdx.Reset();

    GImNodes.NodeIndicesOverlappingWithMouse.clear();

    GImNodes.ImNodesUIState = ImNodesUIState_None;

    GImNodes.MousePos = ImGui::GetIO().MousePos;
    GImNodes.LeftMouseClicked = ImGui::is_mouse_clicked(0);
    GImNodes.LeftMouseReleased = ImGui::is_mouse_released(0);
    GImNodes.LeftMouseDragging = ImGui::is_mouse_dragging(0, 0.0);
    GImNodes.AltMouseClicked =
        (GImNodes.Io.EmulateThreeButtonMouse.Modifier != None &&
         *GImNodes.Io.EmulateThreeButtonMouse.Modifier && GImNodes.LeftMouseClicked) ||
        ImGui::is_mouse_clicked(GImNodes.Io.AltMouseButton);
    GImNodes.AltMouseDragging =
        (GImNodes.Io.EmulateThreeButtonMouse.Modifier != None && GImNodes.LeftMouseDragging &&
         (*GImNodes.Io.EmulateThreeButtonMouse.Modifier)) ||
        ImGui::is_mouse_dragging(GImNodes.Io.AltMouseButton, 0.0);
    GImNodes.AltMouseScrollDelta = ImGui::GetIO().mouse_wheel;
    GImNodes.MultipleSelectModifier =
        (GImNodes.Io.MultipleSelectModifier.Modifier != None
             ? *GImNodes.Io.MultipleSelectModifier.Modifier
             : ImGui::GetIO().key_ctrl);

    GImNodes.ActiveAttribute = false;

    ImGui::BeginGroup();
    {
        ImGui::push_style_var(StyleVar::frame_padding, Vector2D::new(1.f, 1.f));
        ImGui::push_style_var(StyleVar::WindowPadding, Vector2D::new(0.f, 0.f));
        ImGui::push_style_color(StyleColor::ChildBg, GImNodes.Style.colors[ImNodesCol_GridBackground]);
        ImGui::begin_child(
            "scrolling_region",
            Vector2D::new(0.f, 0.f),
            true,
            WindowFlags::NoScrollbar | WindowFlags::NoMove |
                WindowFlags::NoScrollWithMouse);
        GImNodes.CanvasOriginScreenSpace = ImGui::GetCursorScreenPos();

        // NOTE: we have to fetch the canvas draw list *after* we call
        // BeginChild(), otherwise the ImGui UI elements are going to be
        // rendered into the parent window draw list.
        DrawListSet(ImGui::GetWindowDrawList());

        {
            const Vector2D canvas_size = ImGui::GetWindowSize();
            GImNodes.CanvasRectScreenSpace = Rect(
                EditorSpaceToScreenSpace(Vector2D::new(0.f, 0.f)), EditorSpaceToScreenSpace(canvas_size));

            if (GImNodes.Style.flags & NodesStyleFlags_GridLines)
            {
                DrawGrid(editor, canvas_size);
            }
        }
    }
}

void EndNodeEditor()
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Editor);
    GImNodes.CurrentScope = ImNodesScope_None;

    ImNodesEditorContext& editor = EditorContextGet();

    bool no_grid_content = editor.GridContentBounds.is_inverted();
    if (no_grid_content)
    {
        editor.GridContentBounds = ScreenSpaceToGridSpace(editor, GImNodes.CanvasRectScreenSpace);
    }

    // Detect ImGui interaction first, because it blocks interaction with the rest of the UI

    if (GImNodes.LeftMouseClicked && ImGui::IsAnyItemActive())
    {
        editor.ClickInteraction.Type = ImNodesClickInteractionType_ImGuiItem;
    }

    // Detect which UI element is being hovered over. Detection is done in a hierarchical fashion,
    // because a UI element being hovered excludes any other as being hovered over.

    // Don't do hovering detection for nodes/links/pins when interacting with the mini-map, since
    // its an *overlay* with its own interaction behavior and must have precedence during mouse
    // interaction.

    if ((editor.ClickInteraction.Type == ImNodesClickInteractionType_None ||
         editor.ClickInteraction.Type == ImNodesClickInteractionType_LinkCreation) &&
        MouseInCanvas() && !IsMiniMapHovered())
    {
        // Pins needs some special care. We need to check the depth stack to see which pins are
        // being occluded by other nodes.
        ResolveOccludedPins(editor, GImNodes.OccludedPinIndices);

        GImNodes.HoveredPinIdx = ResolveHoveredPin(editor.Pins, GImNodes.OccludedPinIndices);

        if (!GImNodes.HoveredPinIdx.HasValue())
        {
            // Resolve which node is actually on top and being hovered using the depth stack.
            GImNodes.HoveredNodeIdx = ResolveHoveredNode(editor.NodeDepthOrder);
        }

        // We don't check for hovered pins here, because if we want to detach a link by clicking and
        // dragging, we need to have both a link and pin hovered.
        if (!GImNodes.HoveredNodeIdx.HasValue())
        {
            GImNodes.HoveredLinkIdx = ResolveHoveredLink(editor.Links, editor.Pins);
        }
    }

    for (int node_idx = 0; node_idx < editor.Nodes.Pool.size();  += 1node_idx)
    {
        if (editor.Nodes.InUse[node_idx])
        {
            DrawListActivateNodeBackground(node_idx);
            DrawNode(editor, node_idx);
        }
    }

    // In order to render the links underneath the nodes, we want to first select the bottom draw
    // channel.
    GImNodes.CanvasDrawList.channels_set_current(0);

    for (int link_idx = 0; link_idx < editor.Links.Pool.size();  += 1link_idx)
    {
        if (editor.Links.InUse[link_idx])
        {
            DrawLink(editor, link_idx);
        }
    }

    // Render the click interaction UI elements (partial links, box selector) on top of everything
    // else.

    DrawListAppendClickInteractionChannel();
    DrawListActivateClickInteractionChannel();

    if (IsMiniMapActive())
    {
        CalcMiniMapLayout();
        MiniMapUpdate();
    }

    // Handle node graph interaction

    if (!IsMiniMapHovered())
    {
        if (GImNodes.LeftMouseClicked && GImNodes.HoveredLinkIdx.HasValue())
        {
            BeginLinkInteraction(editor, GImNodes.HoveredLinkIdx.Value(), GImNodes.HoveredPinIdx);
        }

        else if (GImNodes.LeftMouseClicked && GImNodes.HoveredPinIdx.HasValue())
        {
            BeginLinkCreation(editor, GImNodes.HoveredPinIdx.Value());
        }

        else if (GImNodes.LeftMouseClicked && GImNodes.HoveredNodeIdx.HasValue())
        {
            BeginNodeSelection(editor, GImNodes.HoveredNodeIdx.Value());
        }

        else if (
            GImNodes.LeftMouseClicked || GImNodes.LeftMouseReleased ||
            GImNodes.AltMouseClicked || GImNodes.AltMouseScrollDelta != 0.f)
        {
            BeginCanvasInteraction(editor);
        }

        bool should_auto_pan =
            editor.ClickInteraction.Type == ImNodesClickInteractionType_BoxSelection ||
            editor.ClickInteraction.Type == ImNodesClickInteractionType_LinkCreation ||
            editor.ClickInteraction.Type == ImNodesClickInteractionType_Node;
        if (should_auto_pan && !MouseInCanvas())
        {
            Vector2D mouse = ImGui::GetMousePos();
            Vector2D center = GImNodes.CanvasRectScreenSpace.get_center();
            Vector2D direction = (center - mouse);
            direction = direction * ImInvLength(direction, 0.0);

            editor.AutoPanningDelta =
                direction * ImGui::GetIO().delta_time * GImNodes.Io.AutoPanningSpeed;
            editor.Panning += editor.AutoPanningDelta;
        }
    }
    ClickInteractionUpdate(editor);

    // At this point, draw commands have been issued for all nodes (and pins). update the node pool
    // to detect unused node slots and remove those indices from the depth stack before sorting the
    // node draw commands by depth.
    ObjectPoolUpdate(editor.Nodes);
    ObjectPoolUpdate(editor.Pins);

    DrawListSortChannelsByDepth(editor.NodeDepthOrder);

    // After the links have been rendered, the link pool can be updated as well.
    ObjectPoolUpdate(editor.Links);

    // Finally, merge the draw channels
    GImNodes.CanvasDrawList.ChannelsMerge();

    // pop style
    ImGui::end_child();      // end scrolling region
    ImGui::pop_style_color(); // pop child window background color
    ImGui::pop_style_var();   // pop window padding
    ImGui::pop_style_var();   // pop frame padding
    ImGui::EndGroup();
}

void MiniMap(
    let                                      minimap_size_fraction,
    const ImNodesMiniMapLocation                     location,
    const ImNodesMiniMapNodeHoveringCallback         node_hovering_callback,
    const ImNodesMiniMapNodeHoveringCallbackUserData node_hovering_callback_data)
{
    // Check that editor size fraction is sane; must be in the range (0, 1]
    // IM_ASSERT(minimap_size_fraction > 0.f && minimap_size_fraction <= 1.f);

    // Remember to call before EndNodeEditor
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Editor);

    ImNodesEditorContext& editor = EditorContextGet();

    editor.MiniMapEnabled = true;
    editor.MiniMapSizeFraction = minimap_size_fraction;
    editor.MiniMapLocation = location;

    // Set node hovering callback information
    editor.MiniMapNodeHoveringCallback = node_hovering_callback;
    editor.MiniMapNodeHoveringCallbackUserData = node_hovering_callback_data;

    // Actual drawing/updating of the MiniMap is done in EndNodeEditor so that
    // mini map is draw over everything and all pin/link positions are updated
    // correctly relative to their respective nodes. Hence, we must store some of
    // of the state for the mini map in GImNodes for the actual drawing/updating
}

void BeginNode(let node_id)
{
    // Remember to call BeginNodeEditor before calling BeginNode
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Editor);
    GImNodes.CurrentScope = ImNodesScope_Node;

    ImNodesEditorContext& editor = EditorContextGet();

    let node_idx = ObjectPoolFindOrCreateIndex(editor.Nodes, node_id);
    GImNodes.CurrentNodeIdx = node_idx;

    ImNodeData& node = editor.Nodes.Pool[node_idx];
    node.ColorStyle.Background = GImNodes.Style.colors[ImNodesCol_NodeBackground];
    node.ColorStyle.BackgroundHovered = GImNodes.Style.colors[ImNodesCol_NodeBackgroundHovered];
    node.ColorStyle.BackgroundSelected = GImNodes.Style.colors[ImNodesCol_NodeBackgroundSelected];
    node.ColorStyle.Outline = GImNodes.Style.colors[ImNodesCol_NodeOutline];
    node.ColorStyle.Titlebar = GImNodes.Style.colors[ImNodesCol_TitleBar];
    node.ColorStyle.TitlebarHovered = GImNodes.Style.colors[ImNodesCol_TitleBarHovered];
    node.ColorStyle.TitlebarSelected = GImNodes.Style.colors[ImNodesCol_TitleBarSelected];
    node.LayoutStyle.CornerRounding = GImNodes.Style.NodeCornerRounding;
    node.LayoutStyle.Padding = GImNodes.Style.NodePadding;
    node.LayoutStyle.BorderThickness = GImNodes.Style.NodeBorderThickness;

    // ImGui::SetCursorPos sets the cursor position, local to the current widget
    // (in this case, the child object started in BeginNodeEditor). Use
    // ImGui::SetCursorScreenPos to set the screen space coordinates directly.
    ImGui::SetCursorPos(GridSpaceToEditorSpace(editor, GetNodeTitleBarOrigin(node)));

    DrawListAddNode(node_idx);
    DrawListActivateCurrentNodeForeground();

    ImGui::push_id(node.Id);
    ImGui::BeginGroup();
}

void EndNode()
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Node);
    GImNodes.CurrentScope = ImNodesScope_Editor;

    ImNodesEditorContext& editor = EditorContextGet();

    // The node's rectangle depends on the ImGui UI group size.
    ImGui::EndGroup();
    ImGui::pop_id();

    ImNodeData& node = editor.Nodes.Pool[GImNodes.CurrentNodeIdx];
    node.Rect = GetItemRect();
    node.Rect.Expand(node.LayoutStyle.Padding);

    editor.GridContentBounds.Add(node.Origin);
    editor.GridContentBounds.Add(node.Origin + node.Rect.GetSize());

    if (node.Rect.contains(GImNodes.MousePos))
    {
        GImNodes.NodeIndicesOverlappingWithMouse.push_back(GImNodes.CurrentNodeIdx);
    }
}

Vector2D GetNodeDimensions(int node_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    let             node_idx = ObjectPoolFind(editor.Nodes, node_id);
    // IM_ASSERT(node_idx != -1); // invalid node_id
    const ImNodeData& node = editor.Nodes.Pool[node_idx];
    return node.Rect.GetSize();
}

void BeginNodeTitleBar()
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Node);
    ImGui::BeginGroup();
}

void EndNodeTitleBar()
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Node);
    ImGui::EndGroup();

    ImNodesEditorContext& editor = EditorContextGet();
    ImNodeData&           node = editor.Nodes.Pool[GImNodes.CurrentNodeIdx];
    node.TitleBarContentRect = GetItemRect();

    ImGui::item_add(GetNodeTitleRect(node), ImGui::GetID("title_bar"));

    ImGui::SetCursorPos(GridSpaceToEditorSpace(editor, GetNodeContentOrigin(node)));
}

void BeginInputAttribute(let id, const ImNodesPinShape shape)
{
    BeginPinAttribute(id, ImNodesAttributeType_Input, shape, GImNodes.CurrentNodeIdx);
}

void EndInputAttribute() { EndPinAttribute(); }

void BeginOutputAttribute(let id, const ImNodesPinShape shape)
{
    BeginPinAttribute(id, ImNodesAttributeType_Output, shape, GImNodes.CurrentNodeIdx);
}

void EndOutputAttribute() { EndPinAttribute(); }

void BeginStaticAttribute(let id)
{
    // Make sure to call BeginNode() before calling BeginAttribute()
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Node);
    GImNodes.CurrentScope = ImNodesScope_Attribute;

    GImNodes.CurrentAttributeId = id;

    ImGui::BeginGroup();
    ImGui::push_id(id);
}

void EndStaticAttribute()
{
    // Make sure to call BeginNode() before calling BeginAttribute()
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Attribute);
    GImNodes.CurrentScope = ImNodesScope_Node;

    ImGui::pop_id();
    ImGui::EndGroup();

    if (ImGui::is_item_active())
    {
        GImNodes.ActiveAttribute = true;
        GImNodes.ActiveAttributeId = GImNodes.CurrentAttributeId;
    }
}

void PushAttributeFlag(const ImNodesAttributeFlags flag)
{
    GImNodes.CurrentAttributeFlags |= flag;
    GImNodes.AttributeFlagStack.push_back(GImNodes.CurrentAttributeFlags);
}

void PopAttributeFlag()
{
    // PopAttributeFlag called without a matching PushAttributeFlag!
    // The bottom value is always the default value, pushed in Initialize().
    // IM_ASSERT(GImNodes.AttributeFlagStack.size() > 1);

    GImNodes.AttributeFlagStack.pop_back();
    GImNodes.CurrentAttributeFlags = GImNodes.AttributeFlagStack.back();
}

void Link(let id, let start_attr_id, let end_attr_id)
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_Editor);

    ImNodesEditorContext& editor = EditorContextGet();
    ImLinkData&           link = ObjectPoolFindOrCreateObject(editor.Links, id);
    link.Id = id;
    link.StartPinIdx = ObjectPoolFindOrCreateIndex(editor.Pins, start_attr_id);
    link.EndPinIdx = ObjectPoolFindOrCreateIndex(editor.Pins, end_attr_id);
    link.ColorStyle.Base = GImNodes.Style.colors[ImNodesCol_Link];
    link.ColorStyle.Hovered = GImNodes.Style.colors[ImNodesCol_LinkHovered];
    link.ColorStyle.Selected = GImNodes.Style.colors[ImNodesCol_LinkSelected];

    // Check if this link was created by the current link event
    if ((editor.ClickInteraction.Type == ImNodesClickInteractionType_LinkCreation &&
         editor.Pins.Pool[link.EndPinIdx].flags & ImNodesAttributeFlags_EnableLinkCreationOnSnap &&
         editor.ClickInteraction.LinkCreation.StartPinIdx == link.StartPinIdx &&
         editor.ClickInteraction.LinkCreation.EndPinIdx == link.EndPinIdx) ||
        (editor.ClickInteraction.LinkCreation.StartPinIdx == link.EndPinIdx &&
         editor.ClickInteraction.LinkCreation.EndPinIdx == link.StartPinIdx))
    {
        GImNodes.SnapLinkIdx = ObjectPoolFindOrCreateIndex(editor.Links, id);
    }
}

void PushColorStyle(const ImNodesCol item, unsigned int color)
{
    GImNodes.ColorModifierStack.push_back(ImNodesColElement(GImNodes.Style.colors[item], item));
    GImNodes.Style.colors[item] = color;
}

void PopColorStyle()
{
    // IM_ASSERT(GImNodes.ColorModifierStack.size() > 0);
    const ImNodesColElement elem = GImNodes.ColorModifierStack.back();
    GImNodes.Style.colors[elem.Item] = elem.Color;
    GImNodes.ColorModifierStack.pop_back();
}

#[derive(Default,Debug,Clone)]
pub struct NodesStyleVarInfo
{
    // DataType Type;
    pub data_type: DataType,
    // ImU32         Count;
    pub count: usize,
    // ImU32         Offset;
    pub offset: usize
    // void* GetVarPtr(NodesStyle* style) const { return (void*)((unsigned char*)style + Offset); }
}

impl NodesStyleVarInfo {
    pub fn new(data_type: DataType, count: usize, offset: usize) -> Self {
        Self {
            data_type,
            count,
            offset
        }
    }
}

pub const STYLE_VAR_INFO: [NodesStyleVarInfo;15]  = [
    // NodesStyleVar_GridSpacing
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, GridSpacing)),
    // NodesStyleVar_NodeCornerRounding
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, NodeCornerRounding)),
    // NodesStyleVar_NodePadding
    NodesStyleVarInfo::new(DataType::Float, 2, IM_OFFSETOF(NodesStyle, NodePadding)),
    // NodesStyleVar_NodeBorderThickness
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, NodeBorderThickness)),
    // NodesStyleVar_LinkThickness
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, LinkThickness)),
    // NodesStyleVar_LinkLineSegmentsPerLength
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, LinkLineSegmentsPerLength)),
    // NodesStyleVar_LinkHoverDistance
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, LinkHoverDistance)),
    // NodesStyleVar_PinCircleRadius
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, PinCircleRadius)),
    // NodesStyleVar_PinQuadSideLength
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, PinQuadSideLength)),
    // NodesStyleVar_PinTriangleSideLength
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, PinTriangleSideLength)),
    // NodesStyleVar_PinLineThickness
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, PinLineThickness)),
    // NodesStyleVar_PinHoverRadius
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, PinHoverRadius)),
    // NodesStyleVar_PinOffset
    NodesStyleVarInfo::new(DataType::Float, 1, IM_OFFSETOF(NodesStyle, PinOffset)),
    // NodesStyleVar_MiniMapPadding
    NodesStyleVarInfo::new(DataType::Float, 2, IM_OFFSETOF(NodesStyle, MiniMapPadding)),
    // NodesStyleVar_MiniMapOffset
    NodesStyleVarInfo::new(DataType::Float, 2, IM_OFFSETOF(NodesStyle, MiniMapOffset)),
];

pub fn get_style_var_info(idx: NodesStyleVar) -> &NodesStyleVarInfo
{
    // IM_ASSERT(idx >= 0 && idx < NodesStyleVar_COUNT);
    // IM_ASSERT(IM_ARRAYSIZE(STYLE_VAR_INFO) == NodesStyleVar_COUNT);
    return STYLE_VAR_INFO[idx].borrow();
}

// void push_style_var(const NodesStyleVar item, let value)
pub fn push_style_float(g: &mut Context, item: NodesStyleVar, value: f32)
{
    // const NodesStyleVarInfo* var_info = get_style_var_info(item);
    let var_info = get_style_var_info(&item);
    if var_info.data_type == DataType::Float && var_info.count == 1
    {
        float& style_var = var_info.GetVarPtr(&GImNodes.Style);
        GImNodes.style_modifier_stack.push_back(NodesStyleVarElement(item, style_var));
        style_var = value;
        return;
    }
    // IM_ASSERT(0 && "Called PushStyleVar() float variant but variable is not a float!");
}

pub fn push_style_vector2d(g: &mut Context, item: NodesStyleVar, value: &Vector2D)
{
    let var_info = get_style_var_info(item);
    if var_info.Type == DataType::Float && var_info.count == 2
    {
        // Vector2D& style_var = *var_info.GetVarPtr(&GImNodes.Style);
        let style_var = var_info.get_var_vector2d(IM_NODES.style);
        IM_NODES.style_modifier_stack.push(NodesStyleVarElement::new(item, style_var));
        style_var = value;
        return;
    }
    // IM_ASSERT(0 && "Called PushStyleVar() Vector2D variant but variable is not a Vector2D!");
}

// void pop_style_var(int count)
pub fn pop_style_var(g: &mut Context, count: i32)
{
    while count > 0
    {
        // IM_ASSERT(GImNodes.StyleModifierStack.size() > 0);
        const NodesStyleVarElement style_backup = GImNodes.style_modifier_stack.back();
        GImNodes.style_modifier_stack.pop_back();
        const NodesStyleVarInfo* var_info = get_style_var_info(style_backup.Item);
        void*                      style_var = var_info.GetVarPtr(&GImNodes.Style);
        if (var_info.Type == DataType::Float && var_info.count == 1)
        {
            ((float*)style_var)[0] = style_backup.FloatValue[0];
        }
        else if (var_info.Type == DataType::Float && var_info.count == 2)
        {
            ((float*)style_var)[0] = style_backup.FloatValue[0];
            ((float*)style_var)[1] = style_backup.FloatValue[1];
        }
        count--;
    }
}

void SetNodeScreenSpacePos(let node_id, const Vector2D& screen_space_pos)
{
    ImNodesEditorContext& editor = EditorContextGet();
    ImNodeData&           node = ObjectPoolFindOrCreateObject(editor.Nodes, node_id);
    node.Origin = ScreenSpaceToGridSpace(editor, screen_space_pos);
}

void SetNodeEditorSpacePos(let node_id, const Vector2D& editor_space_pos)
{
    ImNodesEditorContext& editor = EditorContextGet();
    ImNodeData&           node = ObjectPoolFindOrCreateObject(editor.Nodes, node_id);
    node.Origin = EditorSpaceToGridSpace(editor, editor_space_pos);
}

void SetNodeGridSpacePos(let node_id, const Vector2D& grid_pos)
{
    ImNodesEditorContext& editor = EditorContextGet();
    ImNodeData&           node = ObjectPoolFindOrCreateObject(editor.Nodes, node_id);
    node.Origin = grid_pos;
}

void SetNodeDraggable(let node_id, let draggable)
{
    ImNodesEditorContext& editor = EditorContextGet();
    ImNodeData&           node = ObjectPoolFindOrCreateObject(editor.Nodes, node_id);
    node.Draggable = draggable;
}

Vector2D GetNodeScreenSpacePos(let node_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    let             node_idx = ObjectPoolFind(editor.Nodes, node_id);
    // IM_ASSERT(node_idx != -1);
    ImNodeData& node = editor.Nodes.Pool[node_idx];
    return GridSpaceToScreenSpace(editor, node.Origin);
}

Vector2D GetNodeEditorSpacePos(let node_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    let             node_idx = ObjectPoolFind(editor.Nodes, node_id);
    // IM_ASSERT(node_idx != -1);
    ImNodeData& node = editor.Nodes.Pool[node_idx];
    return GridSpaceToEditorSpace(editor, node.Origin);
}

Vector2D GetNodeGridSpacePos(let node_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    let             node_idx = ObjectPoolFind(editor.Nodes, node_id);
    // IM_ASSERT(node_idx != -1);
    ImNodeData& node = editor.Nodes.Pool[node_idx];
    return node.Origin;
}

void SnapNodeToGrid(int node_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    ImNodeData&           node = ObjectPoolFindOrCreateObject(editor.Nodes, node_id);
    node.Origin = SnapOriginToGrid(node.Origin);
}

bool IsEditorHovered() { return MouseInCanvas(); }

bool IsNodeHovered(int* const node_id)
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);
    // IM_ASSERT(node_id != None);

    let is_hovered = GImNodes.HoveredNodeIdx.HasValue();
    if (is_hovered)
    {
        const ImNodesEditorContext& editor = EditorContextGet();
        *node_id = editor.Nodes.Pool[GImNodes.HoveredNodeIdx.Value()].Id;
    }
    return is_hovered;
}

bool IsLinkHovered(int* const link_id)
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);
    // IM_ASSERT(link_id != None);

    let is_hovered = GImNodes.HoveredLinkIdx.HasValue();
    if (is_hovered)
    {
        const ImNodesEditorContext& editor = EditorContextGet();
        *link_id = editor.Links.Pool[GImNodes.HoveredLinkIdx.Value()].Id;
    }
    return is_hovered;
}

bool IsPinHovered(int* const attr)
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);
    // IM_ASSERT(attr != None);

    let is_hovered = GImNodes.HoveredPinIdx.HasValue();
    if (is_hovered)
    {
        const ImNodesEditorContext& editor = EditorContextGet();
        *attr = editor.Pins.Pool[GImNodes.HoveredPinIdx.Value()].Id;
    }
    return is_hovered;
}

int NumSelectedNodes()
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);
    const ImNodesEditorContext& editor = EditorContextGet();
    return editor.SelectedNodeIndices.size();
}

int NumSelectedLinks()
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);
    const ImNodesEditorContext& editor = EditorContextGet();
    return editor.SelectedLinkIndices.size();
}

void GetSelectedNodes(int* node_ids)
{
    // IM_ASSERT(node_ids != None);

    const ImNodesEditorContext& editor = EditorContextGet();
    for (int i = 0; i < editor.SelectedNodeIndices.size();  += 1i)
    {
        let node_idx = editor.SelectedNodeIndices[i];
        node_ids[i] = editor.Nodes.Pool[node_idx].Id;
    }
}

void GetSelectedLinks(int* link_ids)
{
    // IM_ASSERT(link_ids != None);

    const ImNodesEditorContext& editor = EditorContextGet();
    for (int i = 0; i < editor.SelectedLinkIndices.size();  += 1i)
    {
        let link_idx = editor.SelectedLinkIndices[i];
        link_ids[i] = editor.Links.Pool[link_idx].Id;
    }
}

void ClearNodeSelection()
{
    ImNodesEditorContext& editor = EditorContextGet();
    editor.SelectedNodeIndices.clear();
}

void ClearNodeSelection(int node_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    ClearObjectSelection(editor.Nodes, editor.SelectedNodeIndices, node_id);
}

void ClearLinkSelection()
{
    ImNodesEditorContext& editor = EditorContextGet();
    editor.SelectedLinkIndices.clear();
}

void ClearLinkSelection(int link_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    ClearObjectSelection(editor.Links, editor.SelectedLinkIndices, link_id);
}

void SelectNode(int node_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    SelectObject(editor.Nodes, editor.SelectedNodeIndices, node_id);
}

void SelectLink(int link_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    SelectObject(editor.Links, editor.SelectedLinkIndices, link_id);
}

bool IsNodeSelected(int node_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    return IsObjectSelected(editor.Nodes, editor.SelectedNodeIndices, node_id);
}

bool IsLinkSelected(int link_id)
{
    ImNodesEditorContext& editor = EditorContextGet();
    return IsObjectSelected(editor.Links, editor.SelectedLinkIndices, link_id);
}

bool IsAttributeActive()
{
    // IM_ASSERT((GImNodes.CurrentScope & ImNodesScope_Node) != 0);

    if (!GImNodes.ActiveAttribute)
    {
        return false;
    }

    return GImNodes.ActiveAttributeId == GImNodes.CurrentAttributeId;
}

bool IsAnyAttributeActive(int* const attribute_id)
{
    // IM_ASSERT((GImNodes.CurrentScope & (ImNodesScope_Node | ImNodesScope_Attribute)) == 0);

    if (!GImNodes.ActiveAttribute)
    {
        return false;
    }

    if (attribute_id != None)
    {
        *attribute_id = GImNodes.ActiveAttributeId;
    }

    return true;
}

bool IsLinkStarted(int* const started_at_id)
{
    // Call this function after EndNodeEditor()!
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);
    // IM_ASSERT(started_at_id != None);

    let is_started = (GImNodes.ImNodesUIState & ImNodesUIState_LinkStarted) != 0;
    if (is_started)
    {
        const ImNodesEditorContext& editor = EditorContextGet();
        let                   pin_idx = editor.ClickInteraction.LinkCreation.StartPinIdx;
        *started_at_id = editor.Pins.Pool[pin_idx].Id;
    }

    return is_started;
}

bool IsLinkDropped(int* const started_at_id, let including_detached_links)
{
    // Call this function after EndNodeEditor()!
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);

    const ImNodesEditorContext& editor = EditorContextGet();

    let link_dropped =
        (GImNodes.ImNodesUIState & ImNodesUIState_LinkDropped) != 0 &&
        (including_detached_links ||
         editor.ClickInteraction.LinkCreation.Type != ImNodesLinkCreationType_FromDetach);

    if (link_dropped && started_at_id)
    {
        let pin_idx = editor.ClickInteraction.LinkCreation.StartPinIdx;
        *started_at_id = editor.Pins.Pool[pin_idx].Id;
    }

    return link_dropped;
}

bool IsLinkCreated(
    int* const  started_at_pin_id,
    int* const  ended_at_pin_id,
    bool* const created_from_snap)
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);
    // IM_ASSERT(started_at_pin_id != None);
    // IM_ASSERT(ended_at_pin_id != None);

    let is_created = (GImNodes.ImNodesUIState & ImNodesUIState_LinkCreated) != 0;

    if (is_created)
    {
        const ImNodesEditorContext& editor = EditorContextGet();
        let                   start_idx = editor.ClickInteraction.LinkCreation.StartPinIdx;
        let        end_idx = editor.ClickInteraction.LinkCreation.EndPinIdx.Value();
        const ImPinData& start_pin = editor.Pins.Pool[start_idx];
        const ImPinData& end_pin = editor.Pins.Pool[end_idx];

        if (start_pin.Type == ImNodesAttributeType_Output)
        {
            *started_at_pin_id = start_pin.Id;
            *ended_at_pin_id = end_pin.Id;
        }
        else
        {
            *started_at_pin_id = end_pin.Id;
            *ended_at_pin_id = start_pin.Id;
        }

        if (created_from_snap)
        {
            *created_from_snap =
                editor.ClickInteraction.Type == ImNodesClickInteractionType_LinkCreation;
        }
    }

    return is_created;
}

bool IsLinkCreated(
    int*  started_at_node_id,
    int*  started_at_pin_id,
    int*  ended_at_node_id,
    int*  ended_at_pin_id,
    bool* created_from_snap)
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);
    // IM_ASSERT(started_at_node_id != None);
    // IM_ASSERT(started_at_pin_id != None);
    // IM_ASSERT(ended_at_node_id != None);
    // IM_ASSERT(ended_at_pin_id != None);

    let is_created = (GImNodes.ImNodesUIState & ImNodesUIState_LinkCreated) != 0;

    if (is_created)
    {
        const ImNodesEditorContext& editor = EditorContextGet();
        let                   start_idx = editor.ClickInteraction.LinkCreation.StartPinIdx;
        let         end_idx = editor.ClickInteraction.LinkCreation.EndPinIdx.Value();
        const ImPinData&  start_pin = editor.Pins.Pool[start_idx];
        const ImNodeData& start_node = editor.Nodes.Pool[start_pin.parent_node_idx];
        const ImPinData&  end_pin = editor.Pins.Pool[end_idx];
        const ImNodeData& end_node = editor.Nodes.Pool[end_pin.parent_node_idx];

        if (start_pin.Type == ImNodesAttributeType_Output)
        {
            *started_at_pin_id = start_pin.Id;
            *started_at_node_id = start_node.Id;
            *ended_at_pin_id = end_pin.Id;
            *ended_at_node_id = end_node.Id;
        }
        else
        {
            *started_at_pin_id = end_pin.Id;
            *started_at_node_id = end_node.Id;
            *ended_at_pin_id = start_pin.Id;
            *ended_at_node_id = start_node.Id;
        }

        if (created_from_snap)
        {
            *created_from_snap =
                editor.ClickInteraction.Type == ImNodesClickInteractionType_LinkCreation;
        }
    }

    return is_created;
}

bool IsLinkDestroyed(int* const link_id)
{
    // IM_ASSERT(GImNodes.CurrentScope == ImNodesScope_None);

    let link_destroyed = GImNodes.DeletedLinkIdx.HasValue();
    if (link_destroyed)
    {
        const ImNodesEditorContext& editor = EditorContextGet();
        let                   link_idx = GImNodes.DeletedLinkIdx.Value();
        *link_id = editor.Links.Pool[link_idx].Id;
    }

    return link_destroyed;
}

namespace
{
void NodeLineHandler(ImNodesEditorContext& editor, const char* const line)
{
    int id;
    int x, y;
    if (sscanf(line, "[node.%i", &id) == 1)
    {
        let node_idx = ObjectPoolFindOrCreateIndex(editor.Nodes, id);
        GImNodes.CurrentNodeIdx = node_idx;
        ImNodeData& node = editor.Nodes.Pool[node_idx];
        node.Id = id;
    }
    else if (sscanf(line, "origin=%i,%i", &x, &y) == 2)
    {
        ImNodeData& node = editor.Nodes.Pool[GImNodes.CurrentNodeIdx];
        node.Origin = SnapOriginToGrid(Vector2D::new(x, y));
    }
}

void EditorLineHandler(ImNodesEditorContext& editor, const char* const line)
{
    (void)sscanf(line, "panning=%f,%f", &editor.Panning.x, &editor.Panning.y);
}
} // namespace

const char* SaveCurrentEditorStateToIniString(size_t* const data_size)
{
    return SaveEditorStateToIniString(&EditorContextGet(), data_size);
}

const char* SaveEditorStateToIniString(
    const ImNodesEditorContext* const editor_ptr,
    size_t* const                     data_size)
{
    // IM_ASSERT(editor_ptr != None);
    const ImNodesEditorContext& editor = *editor_ptr;

    GImNodes.TextBuffer.clear();
    // TODO: check to make sure that the estimate is the upper bound of element
    GImNodes.TextBuffer.reserve(64 * editor.Nodes.Pool.size());

    GImNodes.TextBuffer.appendf(
        "[editor]\npanning=%i,%i\n", editor.Panning.x, editor.Panning.y);

    for (int i = 0; i < editor.Nodes.Pool.size(); i += 1)
    {
        if (editor.Nodes.InUse[i])
        {
            const ImNodeData& node = editor.Nodes.Pool[i];
            GImNodes.TextBuffer.appendf("\n[node.%d]\n", node.Id);
            GImNodes.TextBuffer.appendf("origin=%i,%i\n", node.Origin.x, node.Origin.y);
        }
    }

    if (data_size != None)
    {
        *data_size = GImNodes.TextBuffer.size();
    }

    return GImNodes.TextBuffer.c_str();
}

void LoadCurrentEditorStateFromIniString(const char* const data, const size_t data_size)
{
    LoadEditorStateFromIniString(&EditorContextGet(), data, data_size);
}

void LoadEditorStateFromIniString(
    ImNodesEditorContext* const editor_ptr,
    const char* const           data,
    const size_t                data_size)
{
    if (data_size == 0u)
    {
        return;
    }

    ImNodesEditorContext& editor = editor_ptr == None ? EditorContextGet() : *editor_ptr;

    char*       buf = (char*)ImGui::MemAlloc(data_size + 1);
    const char* buf_end = buf + data_size;
    memcpy(buf, data, data_size);
    buf[data_size] = 0;

    void (*line_handler)(ImNodesEditorContext&, const char*);
    line_handler = None;
    char* line_end = None;
    for (char* line = buf; line < buf_end; line = line_end + 1)
    {
        while (*line == '\n' || *line == '\r')
        {
            line += 1;
        }
        line_end = line;
        while (line_end < buf_end && *line_end != '\n' && *line_end != '\r')
        {
            line_end += 1;
        }
        line_end[0] = 0;

        if (*line == ';' || *line == '\0')
        {
            continue;
        }

        if (line[0] == '[' && line_end[-1] == ']')
        {
            line_end[-1] = 0;
            if (strncmp(line + 1, "node", 4) == 0)
            {
                line_handler = NodeLineHandler;
            }
            else if (strcmp(line + 1, "editor") == 0)
            {
                line_handler = EditorLineHandler;
            }
        }

        if (line_handler != None)
        {
            line_handler(editor, line);
        }
    }
    ImGui::MemFree(buf);
}

void SaveCurrentEditorStateToIniFile(const char* const file_name)
{
    SaveEditorStateToIniFile(&EditorContextGet(), file_name);
}

void SaveEditorStateToIniFile(const ImNodesEditorContext* const editor, const char* const file_name)
{
    size_t      data_size = 0u;
    const char* data = SaveEditorStateToIniString(editor, &data_size);
    FILE*       file = ImFileOpen(file_name, "wt");
    if (!file)
    {
        return;
    }

    fwrite(data, sizeof(char), data_size, file);
    fclose(file);
}

void LoadCurrentEditorStateFromIniFile(const char* const file_name)
{
    LoadEditorStateFromIniFile(&EditorContextGet(), file_name);
}

void LoadEditorStateFromIniFile(ImNodesEditorContext* const editor, const char* const file_name)
{
    size_t data_size = 0u;
    char*  file_data = (char*)ImFileLoadToMemory(file_name, "rb", &data_size);

    if (!file_data)
    {
        return;
    }

    LoadEditorStateFromIniString(editor, file_data, data_size);
    ImGui::MemFree(file_data);
}
} // namespace IMNODES_NAMESPACE
