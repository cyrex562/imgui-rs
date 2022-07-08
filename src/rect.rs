use crate::vec_nd::ImVec4;
use crate::math::{ImClampVec2, ImFloor, ImMaxVec2, ImMinVec2};
use crate::vec_nd::ImVec2;

// Helper: ImRect (2D axis aligned bounding-box)
// NB: we can't rely on ImVec2 math operators being available here!
#[derive(Default,Clone,Debug)]
pub struct DimgRect
{
    // ImVec2      Min;    // Upper-left
    pub Min: ImVec2,
    // ImVec2      Max;    // Lower-right
    pub Max: ImVec2,
}

impl DimgRect {
    //  constexpr ImRect()                                        : Min(0.0, 0.0), Max(0.0, 0.0)  {}
    pub fn new() -> Self {
        Self {
            Min: ImVec2::new2(),
            Max: ImVec2::new2()
        }
    }
    //     constexpr ImRect(const ImVec2& min, const ImVec2& max)    : Min(min), Max(max)                {}
    pub fn new2(min: &ImVec2, max: &ImVec2) -> Self {
        Self {
            Min: min.clone(),
            Max: max.clone()
        }
    }
    //     constexpr ImRect(const ImVec4& v)                         : Min(v.x, v.y), Max(v.z, v.w)      {}
    pub fn new3(v: &ImVec4) -> Self {
        Self {
            Min: ImVec2::new(v.y, v.y),
            Max: ImVec2::new(v.z,v.w),
        }
    }
    //     constexpr ImRect(float x1, float y1, float x2, float y2)  : Min(x1, y1), Max(x2, y2)          {}
    pub fn new4(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            Min: ImVec2::new(x1, y1),
            Max: ImVec2::new(x2,y2)
        }
    }
    //
    //     ImVec2      GetCenter() const                   { return ImVec2((Min.x + Max.x) * 0.5, (Min.y + Max.y) * 0.5); }
    pub fn GetCenter(&self) -> ImVec2 {
        ImVec2 {
            x: (self.Min.x + self.Max.x) * 0.5,
            y: (self.Min.y + self.Max.y) * 0.5
        }
    }
    //     ImVec2      GetSize() const                     { return ImVec2(Max.x - Min.x, Max.y - Min.y); }
    pub fn GetSize(&self) -> ImVec2 {
        ImVec2 {
            x: (self.Max.x - self.Min.x),
            y: (self.Max.y - self.Min.y)
        }
    }
    //     float       GetWidth() const                    { return Max.x - Min.x; }
    pub fn GetWidth(&self) -> f32 {
        self.Max.x - self.Min.x
    }
    //     float       GetHeight() const                   { return Max.y - Min.y; }
    pub fn GetHeight(&self) -> f32 {
        self.Max.y - self.Min.y
    }
    //     float       GetArea() const                     { return (Max.x - Min.x) * (Max.y - Min.y); }
    pub fn GetArea(&self) -> f32 {
        (self.Max.x - self.Min.x) * (self.Max.y - self.Min.y)
    }
    //     ImVec2      GetTL() const                       { return Min; }                   // Top-left
    pub fn GetTL(&self) -> ImVec2 {
        self.Min.clone()
    }
    //     ImVec2      GetTR() const                       { return ImVec2(Max.x, Min.y); }  // Top-right
    pub fn GetTR(&self) -> ImVec2 {
        ImVec2 {
            x: self.Max.x,
            y: self.Min.y
        }
    }
    //     ImVec2      GetBL() const                       { return ImVec2(Min.x, Max.y); }  // Bottom-left
    pub fn GetBL(&self) -> ImVec2 {
        ImVec2 {
            x: self.Min.x,
            y: self.Max.y
        }
    }
    //     ImVec2      GetBR() const                       { return Max; }                   // Bottom-right
    pub fn GetBR(&self) -> ImVec2 {
        self.Max.clone()
    }
    //     bool        Contains(const ImVec2& p) const     { return p.x     >= Min.x && p.y     >= Min.y && p.x     <  Max.x && p.y     <  Max.y; }
    pub fn Contains(&self, p: &ImVec2) -> bool {
        p.x >= self.Min.x && p.y >= self.Min.y && p.x < self.Max.x && p.y < self.Max.y
    }
    //     bool        Contains(const ImRect& r) const     { return r.Min.x >= Min.x && r.Min.y >= Min.y && r.Max.x <= Max.x && r.Max.y <= Max.y; }
    pub fn Contains2(&self, r: &Self) -> bool {
        r.Min.x >= self.Min.x && r.Min.y >= self.Min.y && r.Max.x <= self.max.x && r.Max.y <= self.Max.y
    }
    //     bool        Overlaps(const ImRect& r) const     { return r.Min.y <  Max.y && r.Max.y >  Min.y && r.Min.x <  Max.x && r.Max.x >  Min.x; }
    pub fn Overlaps(&self, r: &Self) -> bool {
        r.Min.y < self.Max.y && r.Max.y > self.Min.y && r.Min.x < self.Max.x && r.Max.x > self.Min.x
    }
    //     void        Add(const ImVec2& p)                { if (Min.x > p.x)     Min.x = p.x;     if (Min.y > p.y)     Min.y = p.y;     if (Max.x < p.x)     Max.x = p.x;     if (Max.y < p.y)     Max.y = p.y; }
    pub fn Add(&mut self, p: &ImVec2) {
        if self.Min.x > p.x {
            self.Min.x = p.x
        }
        if self.Min.y > p.y {
            self.Min.y = p.y
        }
        if self.Max.x < p.x {
            self.Max.x = p.x
        }
        if self.Max.y < p.y {
            self.Max.y = p.y
        }
    }
    //     void        Add(const ImRect& r)                { if (Min.x > r.Min.x) Min.x = r.Min.x; if (Min.y > r.Min.y) Min.y = r.Min.y; if (Max.x < r.Max.x) Max.x = r.Max.x; if (Max.y < r.Max.y) Max.y = r.Max.y; }
    pub fn Add2(&mut self, r: &Self) {
        if self.Min.x > r.Min.x {
            self.Min.x = r.Min.x
        }
        if self.Miny > r.Min.y {
            self.Min.y = r.Min.y
        }
        if self.Max.x < r.Max.x {
            self.Max.x = r.Max.x
        }
        if self.Max.y < r.Max.y {
            self.max.y = r.Max.y
        }
    }
    //     void        Expand(const float amount)          { Min.x -= amount;   Min.y -= amount;   Max.x += amount;   Max.y += amount; }
    pub fn Expand(&mut self, amount: f32) {
        self.Min.x -= amount;
        self.Min.y -= amount;
        self.Max.x += amount;
        self.Max.y += amount;
    }
    //     void        Expand(const ImVec2& amount)        { Min.x -= amount.x; Min.y -= amount.y; Max.x += amount.x; Max.y += amount.y; }
    pub fn Expand2(&mut self, amount: &ImVec2) {
        self.Min.x -= amount.x;
        self.Min.y -= amount.y;
        self.Max.x += amount.x;
        self.Max.y += amount.y;
    }
    //     void        Translate(const ImVec2& d)          { Min.x += d.x; Min.y += d.y; Max.x += d.x; Max.y += d.y; }
    pub fn Translate(&mut self, d: &ImVec2) {
        self.Min.x += d.x;
        self.Min.y += d.y;
        self.Max.x += d.x;
        self.Max.y += d.y;
    }
    //     void        TranslateX(float dx)                { Min.x += dx; Max.x += dx; }
    pub fn TranslateX(&mut self, dx: f32) {
        self.Min.x += dx;
        self.Max.x += dx;
    }
    //     void        TranslateY(float dy)                { Min.y += dy; Max.y += dy; }
    pub fn TranslateY(&mut self, dy: f32) {
        self.Min.y += dy;
        self.Max.y += dy;
    }
    //     void        ClipWith(const ImRect& r)           { Min = ImMax(Min, r.Min); Max = ImMin(Max, r.Max); }                   // Simple version, may lead to an inverted rectangle, which is fine for Contains/Overlaps test but not for display.
    pub fn ClipWidth(&mut self, r: &Self) {
        self.Min = ImMaxVec2(&self.Min, &r.Min);
        self.Max = ImMinVec2(&self.Max, &r.Max);
    }
    //     void        ClipWithFull(const ImRect& r)       { Min = ImClamp(Min, r.Min, r.Max); Max = ImClamp(Max, r.Min, r.Max); } // Full version, ensure both points are fully clipped.
    pub fn ClipWithFull(&mut self, r: &Self) {
        self.Min = ImClampVec2(&self.Min, &r.Min, &r.Max);
        self.Max = ImClampVec2(&self.Max, &r.Min, &r.Max);
    }
    //     void        Floor()                             { Min.x = IM_FLOOR(Min.x); Min.y = IM_FLOOR(Min.y); Max.x = IM_FLOOR(Max.x); Max.y = IM_FLOOR(Max.y); }
    pub fn Floor(&mut self) {
        self.Min.x = ImFloor(self.Min.x);
        self.Min.y = ImFloor(self.Min.y);
        self.Max.x = ImFloor(self.Max.x);
        self.Max.y = ImFloor(self.Max.y);
    }
    //     bool        IsInverted() const                  { return Min.x > Max.x || Min.y > Max.y; }
    pub fn IsInverted(&mut self) -> bool {
        self.Min.x > self.Max.x || self.Min.y > self.Max.y
    }
    //     ImVec4      ToVec4() const                      { return ImVec4(Min.x, Min.y, Max.x, Max.y); }
    pub fn ToVec4(&self) -> ImVec4 {
        ImVec4 {
            x: self.Min.x,
            y: self.Min.y,
            z: self.Max.x,
            w: self.Max.y
        }
    }
}
