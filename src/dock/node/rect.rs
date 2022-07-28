use crate::axis::Axis;
use crate::rect::Rect;
use crate::types::Direction;
use crate::vectors::two_d::Vector2D;
use crate::vectors::ImLengthSqr;
use crate::Context;

// void DockNodeCalcSplitRects(Vector2D& pos_old, Vector2D& size_old, Vector2D& pos_new, Vector2D& size_new, ImGuiDir dir, Vector2D size_new_desired)
pub fn dock_node_calc_split_rects(
    g: &mut Context,
    pos_old: &Vector2D,
    size_old: &Vector2D,
    size_new: &Vector2D,
    pos_new: &Vector2D,
    dir: Direction,
    size_new_desired: Vector2D,
) {
    // ImGuiContext& g = *GImGui;
    // let dock_spacing = g.style.item_inner_spacing.x;
    let dock_spacing = g.style.item_inner_spacing.x;
    // const ImGuiAxis axis = (dir == Direction::Left || dir == Direction::Right) ? Axis::X : Axis::Y;
    let axis = if dir == Direction::Left || dir == Direction::Right {
        Axis::X
    } else {
        Axis::Y
    };
    pos_new[&axis ^ 1] = pos_old[&axis ^ 1];
    size_new[&axis ^ 1] = size_old[&axis ^ 1];

    // Distribute size on given axis (with a desired size or equally)
    // let w_avail = size_old[axis] - dock_spacing;
    let w_avail = size_old[&axis] - dock_spacing;
    if size_new_desired[&axis] > 0.0 && size_new_desired[&axis] <= w_avail * 0.5 {
        size_new[&axis] = size_new_desired[&axis];
        size_old[&axis] = f32::floor(w_avail - size_new[&axis]);
    } else {
        size_new[&axis] = f32::floor(w_avail * 0.5);
        size_old[&axis] = f32::floor(w_avail - size_new[&axis]);
    }

    // Position each node
    if dir == Direction::Right || dir == Direction::Down {
        pos_new[&axis] = pos_old[&axis] + size_old[&axis] + dock_spacing;
    } else if dir == Direction::Left || dir == Direction::Up {
        pos_new[&axis] = pos_old[&axis];
        pos_old[&axis] = pos_new[&axis] + size_new[&axis] + dock_spacing;
    }
}

// Retrieve the drop rectangles for a given direction or for the center + perform hit testing.
// bool DockNodeCalcDropRectsAndTestMousePos(const Rect& parent, ImGuiDir dir, Rect& out_r, bool outer_docking, Vector2D* test_mouse_pos)
pub fn dock_node_calc_drop_rects_and_test_mouse_pos(
    g: &mut Context,
    parent: &Rect,
    dir: &Direction,
    out_r: &mut Rect,
    outer_docking: bool,
    test_mouse_pos: Option<&mut Vector2D>,
) -> bool {
    // ImGuiContext& g = *GImGui;

    // let parent_smaller_axis = ImMin(parent.get_width(), parent.get_height());
    let parent_smaller_axis = f32::min(parent.get_width(), parent.get_height());
    let hs_for_central_nodes = f32::min(
        g.font_size * 1.5,
        f32::max(g.font_size * 0.5, parent_smaller_axis / 8.0),
    );
    let mut hs_w = 0f32; // Half-size, longer axis
    // float hs_h; // Half-size, smaller axis
    let mut hs_h = 0f32;
    // Vector2D off; // Distance from edge or center
    let mut off = Vector2D::default();
    if outer_docking {
        //hs_w = f32::floor(ImClamp(parent_smaller_axis - hs_for_central_nodes * 4.0, g.font_size * 0.5, g.font_size * 8.0));
        //hs_h = f32::floor(hs_w * 0.15);
        //off = Vector2D(f32::floor(parent.get_width() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h), f32::floor(parent.get_height() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h));
        hs_w = f32::floor(hs_for_central_nodes * 1.50);
        hs_h = f32::floor(hs_for_central_nodes * 0.80);
        off = Vector2D::new(
            f32::floor(parent.get_width() * 0.5 - hs_h),
            f32::floor(parent.get_height() * 0.5 - hs_h),
        );
    } else {
        hs_w = f32::floor(hs_for_central_nodes);
        hs_h = f32::floor(hs_for_central_nodes * 0.90);
        off = Vector2D::new(f32::floor(hs_w * 2.40), f32::floor(hs_w * 2.40));
    }

    let c = Vector2D::floor(parent.get_center());
    if dir == Direction::None {
        *out_r = Rect::from((c.x - hs_w, c.y - hs_w, c.x + hs_w, c.y + hs_w));
    } else if dir == Direction::Up {
        *out_r = Rect::from((
            c.x - hs_w,
            c.y - off.y - hs_h,
            c.x + hs_w,
            c.y - off.y + hs_h,
        ));
    } else if dir == Direction::Down {
        *out_r = Rect::from((
            c.x - hs_w,
            c.y + off.y - hs_h,
            c.x + hs_w,
            c.y + off.y + hs_h,
        ));
    } else if dir == Direction::Left {
        *out_r = Rect::from((
            c.x - off.x - hs_h,
            c.y - hs_w,
            c.x - off.x + hs_h,
            c.y + hs_w,
        ));
    } else if dir == Direction::Right {
        *out_r = Rect::from((
            c.x + off.x - hs_h,
            c.y - hs_w,
            c.x + off.x + hs_h,
            c.y + hs_w,
        ));
    }

    if test_mouse_pos.is_none() {
        return false;
    }

    let hit_r = out_r;
    if !outer_docking {
        // Custom hit testing for the 5-way selection, designed to reduce flickering when moving diagonally between sides
        hit_r.Expand(f32::floor(hs_w * 0.30));
        let mouse_delta = (test_mouse_pos.unwrap() - c);
        let mouse_delta_len2 = ImLengthSqr(mouse_delta);
        let r_threshold_center = hs_w * 1.4;
        let r_threshold_sides = hs_w * (1.4 + 1.2);
        if mouse_delta_len2 < r_threshold_center * r_threshold_center {
            return dir == Direction::None;
        }
        if mouse_delta_len2 < r_threshold_sides * r_threshold_sides {
            return dir == ImGetDirQuadrantFromDelta(mouse_delta.x, mouse_delta.y);
        }
    }
    return hit_r.contains(test_mouse_pos.unwrap());
}
