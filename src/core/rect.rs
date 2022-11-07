#![allow(non_snake_case)]

use crate::core::math_ops::{ImClamp, ImMax};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::window::ImguiWindow;
use crate::GImGui;
use libc::c_float;

// Helper: ImRect (2D axis aligned bounding-box)
// NB: we can't rely on math: ImVec2 operators being available here!
#[derive(Debug, Clone, Default, Copy)]
pub struct ImRect {
    // ImVec2      Min;    // Upper-left
    pub min: ImVec2,
    // ImVec2      Max;    // Lower-right
    pub max: ImVec2,
}

impl ImRect {
    // constexpr ImRect::default()                                        : Min(0.0, 0.0), Max(0.0, 0.0)  {}
    // pub fn new() -> Self {
    //     Self {
    //         Min: ImVec2::default(),
    //         Max: ImVec2::default()
    //     }
    // }

    // constexpr ImRect(const min: &mut ImVec2, const max: &mut ImVec2)    : Min(min), Max(max)                {}
    pub fn from_vec2(min: &ImVec2, max: &ImVec2) -> Self {
        Self {
            min: min.clone(),
            max: max.clone(),
        }
    }

    // constexpr ImRect(v: &ImVec4)                         : Min(v.x, v.y), Max(v.z, v.w)      {}
    pub fn from_vec4(v: &ImVec4) -> Self {
        Self {
            min: ImVec2::from_floats(v.x, v.y),
            max: ImVec2::from_floats(v.z, v.w),
        }
    }

    // constexpr ImRect(c_float x1, c_float y1, c_float x2, c_float y2)  : Min(x1, y1), Max(x2, y2)          {}
    pub fn from_floats(x1: c_float, y1: c_float, x2: c_float, y2: c_float) -> Self {
        Self {
            min: ImVec2::from_floats(x1, y1),
            max: ImVec2::from_floats(x2, y2),
        }
    }

    // ImVec2      GetCenter() const                   { return ImVec2::new((Min.x + Max.x) * 0.5, (Min.y + Max.y) * 0.5); }
    pub fn GetCenter(&mut self) -> ImVec2 {
        ImVec2::from_floats(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + Self.max.y) * 0.5,
        )
    }

    // ImVec2      GetSize() const                     { return ImVec2::new(Max.x - Min.x, Max.y - Min.y); }
    pub fn GetSize(&mut self) -> ImVec2 {
        ImVec2::from_floats(self.max.x - self.min.x, self.max.y - self.min.y)
    }

    // c_float       GetWidth() const                    { return Max.x - Min.x; }
    pub fn GetWidth(&mut self) -> f32 {
        return self.max.x - self.min.x;
    }

    // c_float       GetHeight() const                   { return Max.y - Min.y; }
    pub fn GetHeight(&mut self) -> f32 {
        return self.max.y - self.min.y;
    }

    // c_float       GetArea() const                     { return (Max.x - Min.x) * (Max.y - Min.y); }
    pub fn GetArea(&mut self) -> f32 {
        (self.max.x - self.min.x) * (self.max.y - self.min.y)
    }

    // ImVec2      GetTL() const                       { return Min; }                   // Top-left
    pub fn GetTL(&mut self) -> ImVec2 {
        self.min.clone()
    }

    // ImVec2      GetTR() const                       { return ImVec2::new(Max.x, Min.y); }  // Top-right
    pub fn GetTR(&mut self) -> ImVec2 {
        ImVec2::from_floats(self.max.x, self.min.y)
    }

    // ImVec2      GetBL() const                       { return ImVec2::new(Min.x, Max.y); }  // Bottom-left
    pub fn GetBL(&mut self) -> ImVec2 {
        ImVec2::from_floats(self.min.x, self.max.y)
    }

    // ImVec2      GetBR() const                       { return Max; }                   // Bottom-right
    pub fn GetBR(&mut self) -> ImVec2 {
        self.max.clone()
    }

    // bool        Contains(const p: &mut ImVec2) const     { return p.x     >= Min.x && p.y     >= Min.y && p.x     <  Max.x && p.y     <  Max.y; }
    pub fn Contains(&mut self, p: &ImVec2) -> bool {
        p.x >= self.min.x && p.y >= self.min.y && p.x < self.max.x && p.x < self.max.y
    }

    // bool        Contains(const ImRect& r) const     { return r.Min.x >= Min.x && r.Min.y >= Min.y && r.Max.x <= Max.x && r.Max.y <= Max.y; }
    pub fn Contains2(&mut self, r: &Self) -> bool {
        r.min.x >= self.min.x
            && r.min.y >= self.min.y
            && r.max.x <= self.max.x
            && r.max.y <= self.max.y
    }

    // bool        Overlaps(const ImRect& r) const     { return r.Min.y <  Max.y && r.Max.y >  Min.y && r.Min.x <  Max.x && r.Max.x >  Min.x; }
    pub fn Overlaps(&mut self, r: &Self) -> bool {
        return r.min.y < self.max.y
            && r.max.y > self.min.y
            && r.min.x < self.max.x
            && r.max.x > self.min.x;
    }

    // void        Add(const p: &mut ImVec2)                { if (Min.x > p.x)     Min.x = p.x;     if (Min.y > p.y)     Min.y = p.y;     if (Max.x < p.x)     Max.x = p.x;     if (Max.y < p.y)     Max.y = p.y; }
    pub fn Add(&mut self, p: &ImVec2) {
        if self.min.x > p.x {
            self.min.x = p.x;
        }
        if self.min.y > p.y {
            self.min.y = p.y;
        }
        if self.max.x < p.x {
            self.max.x = p.x
        }
        if self.max.y < p.y {
            self.max.y = p.y
        }
    }

    // void        Add(const ImRect& r)                { if (Min.x > r.Min.x) Min.x = r.Min.x; if (Min.y > r.Min.y) Min.y = r.Min.y; if (Max.x < r.Max.x) Max.x = r.Max.x; if (Max.y < r.Max.y) Max.y = r.Max.y; }
    pub fn Add2(&mut self, r: &ImRect) {
        if self.min.x > r.min.x {
            self.min.x = r.min.x
        }
        if self.min.y > r.min.y {
            self.min.y = r.min.y
        }
        if self.max.x < r.max.x {
            self.max.x = r.max.x
        }
        if self.max.y < r.max.y {
            self.max.y = r.max.y
        }
    }

    // void        Expand(const c_float amount)          { Min.x -= amount;   Min.y -= amount;   Max.x += amount;   Max.y += amount; }
    pub fn Expand(&mut self, amount: c_float) {
        self.min.x -= amount;
        self.min.y -= amount;
        self.max.x += amount;
        self.max.y += amount;
    }

    // void        Expand(const amount: &mut ImVec2)        { Min.x -= amount.x; Min.y -= amount.y; Max.x += amount.x; Max.y += amount.y; }
    pub fn expand_from_vec(&mut self, amount: &ImVec2) {
        self.min.x -= amount.x;
        self.min.y -= amount.y;
        self.max.x += amount.x;
        self.max.y += amount.y;
    }

    // void        Translate(const d: &mut ImVec2)          { Min.x += d.x; Min.y += d.y; Max.x += d.x; Max.y += d.y; }
    pub fn Translate(&mut self, d: &ImVec2) {
        self.min.x += d.x;
        self.min.y += d.y;
        self.max.x += d.x;
        self.max.y += d.y;
    }

    // void        TranslateX(c_float dx)                { Min.x += dx; Max.x += dx; }
    pub fn TranslateX(&mut self, dx: c_float) {
        self.min.x += dx;
        self.max.x += dx;
    }

    // void        TranslateY(c_float dy)                { Min.y += dy; Max.y += dy; }
    pub fn TranslateY(&mut self, dy: c_float) {
        self.min.y += dy;
        self.max.y += dy;
    }

    // void        ClipWith(const ImRect& r)           { Min = ImMax(Min, r.Min); Max = ImMin(Max, r.Max); }                   // Simple version, may lead to an inverted rectangle, which is fine for Contains/Overlaps test but not for display.
    pub fn ClipWidth(&mut self, r: &ImRect) {
        self.min = ImMax(self.min.clone(), r.min.clone());
        self.max = ImMax(self.max.clone(), r.max.clone())
    }

    // void        ClipWithFull(const ImRect& r)       { Min = ImClamp(Min, r.Min, r.Max); Max = ImClamp(Max, r.Min, r.Max); } // Full version, ensure both points are fully clipped.
    pub fn ClipWithFull(&mut self, r: &ImRect) {
        self.min = ImClamp(self.min.clone(), r.min.clone(), r.max.clone());
        self.max = ImClamp(self.max.clone(), r.min.clone(), r.max.clone());
    }

    // void        Floor()                             { Min.x = IM_FLOOR(Min.x); Min.y = IM_FLOOR(Min.y); Max.x = IM_FLOOR(Max.x); Max.y = IM_FLOOR(Max.y); }
    pub fn Floor(&mut self) {
        self.min.x = f32::floor(self.min.x);
        self.min.y = f32::floor(self.min.y);
        self.max.x = f32::floor(self.max.x);
        self.max.y = f32::floor(self.max.y);
    }

    // bool        IsInverted() const                  { return Min.x > Max.x || Min.y > Max.y; }
    pub fn IsInverted(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y
    }

    // ImVec4      ToVec4() const                      { return ImVec4(Min.x, Min.y, Max.x, Max.y); }
    pub fn ToVec4(&self) -> ImVec4 {
        ImVec4::from_floats(self.min.x, self.min.y, self.max.x, self.max.y)
    }
}

// IsRectVisible: bool(size: &ImVec2)
pub unsafe fn IsRectVisible(size: &ImVec2) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    return window.ClipRect.Overlaps(ImRect::from_vec2(
        &window.dc.cursor_pos,
        window.dc.cursor_pos + size,
    ));
}

// IsRectVisible: bool(rect_min: &ImVec2, rect_max: &ImVec2)
pub unsafe fn IsRectVisible2(rect_min: &ImVec2, rect_max: &ImVec2) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    return window
        .ClipRect
        .Overlaps(ImRect::from_vec2(rect_min, rect_max));
}
