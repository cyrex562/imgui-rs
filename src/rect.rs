#![allow(non_snake_case)]

use libc::c_float;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;

// Helper: ImRect (2D axis aligned bounding-box)
// NB: we can't rely on ImVec2 math operators being available here!
#[derive(Debug,Clone,Default)]
pub struct  ImRect
{
    // ImVec2      Min;    // Upper-left
pub Min: ImVec2,
// ImVec2      Max;    // Lower-right
pub Max: ImVec2

}

impl ImRect {
    // constexpr ImRect::new()                                        : Min(0f32, 0f32), Max(0f32, 0f32)  {}
    pub fn new() -> Self {
        Self {
            Min: ImVec2::default(),
            Max: ImVec2::default()
        }
    }

    // constexpr ImRect::new(const min: &ImVec2, const max: &ImVec2)    : Min(min), Max(max)                {}
    pub fn new2(min: &ImVec2, max: &ImVec2) -> Self {
        Self {
            Min: min.clone(),
            Max: max.clone()
        }
    }

    // constexpr ImRect::new(const ImVec4& v)                         : Min(v.x, v.y), Max(v.z, v.w)      {}
    pub fn new3(v: &ImVec4) -> Self {
        Self {
            Min: ImVec2::new2(v.x, v.y),
            Max: ImVec2::new2(v.z, v.w)
        }
    }

    // constexpr ImRect::new(x1: c_float, y1: c_float, x2: c_float, y2: c_float)  : Min(x1, y1), Max(x2, y2)          {}
    pub fn new4(x1: c_float, y1: c_float, x2: c_float, y2: c_float) -> Self {
        Self {
            Min: ImVec2::new2(x1,y1),
            Max: ImVec2::new2(x2,y2),
        }
    }

    // ImVec2      GetCenter() const                   { return ImVec2::new((Min.x + Max.x) * 0.5f32, (Min.y + Max.y) * 0.5f32); }
    pub fn GetCenter(&mut self) -> ImVec2 {
        ImVec2::new2((self.Min.x + self.Max.x) *0.5f32, (self.Min.y + Self.Max.y) * 0.5f32)
    }

    // ImVec2      GetSize() const                     { return ImVec2::new(Max.x - Min.x, Max.y - Min.y); }
    pub fn GetSize(&mut self) -> ImVec2 {
        ImVec2::new2(self.Max.x - self.Min.x, self.Max.y - self.Min.y)
    }

    // c_float       GetWidth() const                    { return Max.x - Min.x; }
    pub fn GetWidth(&mut self) -> c_float {
        return self.Max.x - self.Min.x
    }

    // c_float       GetHeight() const                   { return Max.y - Min.y; }
    pub fn GetHeight(&mut self) -> c_float {
        return self.Max.y - self.Min.y
    }

    // c_float       GetArea() const                     { return (Max.x - Min.x) * (Max.y - Min.y); }
    pub fn GetArea(&mut self) -> c_float {
        (self.Max.x - self.Min.x) * (self.Max.y - self.Min.y)
    }


    // ImVec2      GetTL() const                       { return Min; }                   // Top-left
    pub fn GetTL(&mut self) -> ImVec2 {
        self.Min.clone()
    }

    // ImVec2      GetTR() const                       { return ImVec2::new(Max.x, Min.y); }  // Top-right
    pub fn GetTR(&mut self) -> ImVec2 {
        ImVec2::new2(self.Max.x, self.Min.y)
    }

    // ImVec2      GetBL() const                       { return ImVec2::new(Min.x, Max.y); }  // Bottom-left
    pub fn GetBL(&mut self) -> ImVec2 {
        ImVec2::new2(self.Min.x, self.Max.y)
    }

    // ImVec2      GetBR() const                       { return Max; }                   // Bottom-right
    pub fn GetBR(&mut self) -> ImVec2 {
        self.Max.clone()
    }

    // bool        Contains(const p: &ImVec2) const     { return p.x     >= Min.x && p.y     >= Min.y && p.x     <  Max.x && p.y     <  Max.y; }
    pub fn Contains(&mut self, p: &ImVec2) -> bool {
        p.x >= self.Min.x && p.y >= self.Min.y && p.x < self.Max.x && p.x < self.Max.y
    }

    // bool        Contains(r: &ImRect) const     { return r.Min.x >= Min.x && r.Min.y >= Min.y && r.Max.x <= Max.x && r.Max.y <= Max.y; }
    pub fn Contains2(&mut self, r: &Self) -> bool {
        r.Min.x >= self.Min.x && r.Min.y >= self.Min.y && r.Max.x <= self.Max.x && r.Max.y <= self.Max.y
    }

    // bool        Overlaps(r: &ImRect) const     { return r.Min.y <  Max.y && r.Max.y >  Min.y && r.Min.x <  Max.x && r.Max.x >  Min.x; }
    pub fn Overlaps(&mut self, r: &Self) -> bool {
        return r.Min.y < self.Max.y && r.Max.y > self.Min.y && r.Min.x < self.Max.x && r.Max.x > self.Min.x
    }

    // void        Add(const p: &ImVec2)                { if (Min.x > p.x)     Min.x = p.x;     if (Min.y > p.y)     Min.y = p.y;     if (Max.x < p.x)     Max.x = p.x;     if (Max.y < p.y)     Max.y = p.y; }
    pub fn Add(&mut self, p: &ImVec2) {
        if self.Min.x > p.x {
            self.Min.x = p.x;
        }
        if self.Min.y > p.y {
            self.Min.y = p.y;
        }
        if self.Max.x < p.x {
            self.Max.x = p.x
        }
        if self.Max.y < p.y {
            self.Max.y = p.y
        }
    }

    // void        Add(r: &ImRect)                { if (Min.x > r.Min.x) Min.x = r.Min.x; if (Min.y > r.Min.y) Min.y = r.Min.y; if (Max.x < r.Max.x) Max.x = r.Max.x; if (Max.y < r.Max.y) Max.y = r.Max.y; }
    pub fn Add2(&mut self, r: &ImRect) {
        if self.Min.x > r.Min.x {
            self.Min.x = r.Min.x
        }
        if self.Min.y > r.Min.y {
            self.Min.y = r.Min.y
        }
        if self.Max.x < r.Max.x {
            self.Max.x = r.Max.x
        }
        if self.Max.y < r.Max.y {
            self.Max.y = r.Max.y
        }
    }

    // void        Expand(const amount: c_float)          { Min.x -= amount;   Min.y -= amount;   Max.x += amount;   Max.y += amount; }
    pub fn Expand(&mut self, amount: c_float) {
        self.Min.x -= amount;
        self.Min.y -= amount;
        self.Max.x += amount;
        self.Max.y += amount;
    }

    // void        Expand(const amount: &ImVec2)        { Min.x -= amount.x; Min.y -= amount.y; Max.x += amount.x; Max.y += amount.y; }
    pub fn Expand2(&mut self, amount: &ImVec2) {
        self.Min.x -= amount.x;
        self.Min.y -= amount.y;
        self.Max.x += amount.x;
        self.Max.y += amount.y;
    }

    // void        Translate(const d: &ImVec2)          { Min.x += d.x; Min.y += d.y; Max.x += d.x; Max.y += d.y; }
    pub fn Translate(&mut self, d: &ImVec2) {
        self.Min.x += d.x;
        self.Min.y += d.y;
        self.Max.x += d.x;
        self.Max.y += d.y;
    }

    // void        TranslateX(dx: c_float)                { Min.x += dx; Max.x += dx; }
    pub fn TranslateX(&mut self, dx: c_float) {
        self.Min.x += dx;
        self.Max.x += dx;
    }

    // void        TranslateY(dy: c_float)                { Min.y += dy; Max.y += dy; }
    pub fn TranslateY(&mut self, dy: c_float) {
        self.Min.y += dy;
        self.Max.y += dy;
    }

    // void        ClipWith(r: &ImRect)           { Min = ImMax(Min, r.Min); Max = ImMin(Max, r.Max); }                   // Simple version, may lead to an inverted rectangle, which is fine for Contains/Overlaps test but not for display.
    pub fn ClipWidth(&mut self, r: &ImRect) {
        self.Min = ImMax(self.Min.clone(), r.Min.clone());
        self.Max = ImMax(self.Max.clone(), r.Max.clone())
    }

    // void        ClipWithFull(r: &ImRect)       { Min = ImClamp(Min, r.Min, r.Max); Max = ImClamp(Max, r.Min, r.Max); } // Full version, ensure both points are fully clipped.
    pub fn ClipWithFull(&mut self, r: &ImRect) {
        self.Min = ImClamp(self.Min.clone(), r.Min.clone(), r.Max.clone());
        self.Max = ImClamp(self.Max.clone(), r.Min.clone(), r.Max.clone());
    }

    // void        Floor()                             { Min.x = IM_FLOOR(Min.x); Min.y = IM_FLOOR(Min.y); Max.x = IM_FLOOR(Max.x); Max.y = IM_FLOOR(Max.y); }
    pub fn Floor(&mut self) {
        self.Min.x = f32::floor(self.Min.x);
        self.Min.y = f32::floor(self.Min.y);
        self.Max.x = f32::floor(self.Max.x);
        self.Max.y = f32::floor(self.Max.y);
    }

    // bool        IsInverted() const                  { return Min.x > Max.x || Min.y > Max.y; }
    pub fn IsInverted(&self) -> bool {
        self.Min.x > self.Max.x || self.Min.y > self.Max.y
    }

    // ImVec4      ToVec4() const                      { return ImVec4(Min.x, Min.y, Max.x, Max.y); }
    pub fn ToVec4(&self) -> ImVec4 {
        ImVec4::new2(self.Min.x, self.Min.y, self.Max.x, self.Max.y)
    }
}
