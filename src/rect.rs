use crate::math::{ImClampVec2, ImFloor, ImMaxVec2, ImMinVec2};
use crate::vectors::vector_2d::Vector2D;
use crate::vectors::Vector4D;

// Helper: ImRect (2D axis aligned bounding-box)
// NB: we can't rely on Vector2D math operators being available here!
#[derive(Default, Clone, Debug)]
pub struct Rect {
    // upper left
    pub min: Vector2D,
    // lower right
    pub max: Vector2D,
}

impl From<[&Vector2D; 2]> for Rect {
    fn from(vectors: [&Vector2D; 2]) -> Self {
        Self {
            min: vectors[0].clone(),
            max: vectors[1].clone(),
        }
    }
}

impl From<[f32; 4]> for Rect {
    fn from(floats: [f32; 4]) -> Self {
        Self {
            min: Vector2D::new(floats[0], floats[1]),
            max: Vector2D::new(floats[2], floats[3]),
        }
    }
}

impl From<&Vector4d> for Rect {
    fn from(param: &Vector4D) -> Self {
        Self {
            min: Vector2D::new(param.x, param.y),
            max: Vector2D::new(param.z, param.w),
        }
    }
}

impl Rect {
    pub fn new(min: &Vector2D, max: &Vector2D) -> Self {
        Self {
            min: min.clone(),
            max: max.clone(),
        }
    }

    pub fn center(&self) -> Vector2D {
        Vector2D {
            x: (self.min.x + self.max.x) * 0.5,
            y: (self.min.y + self.max.y) * 0.5,
        }
    }

    pub fn size(&self) -> Vector2D {
        Vector2D {
            x: (self.max.x - self.min.x),
            y: (self.max.y - self.min.y),
        }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn area(&self) -> f32 {
        (self.max.x - self.min.x) * (self.max.y - self.min.y)
    }

    pub fn top_left(&self) -> Vector2D {
        self.min.clone()
    }

    pub fn top_right(&self) -> Vector2D {
        Vector2D {
            x: self.max.x,
            y: self.min.y,
        }
    }

    pub fn bottom_left(&self) -> Vector2D {
        Vector2D {
            x: self.min.x,
            y: self.max.y,
        }
    }
    //     Vector2D      get_br() const                       { return max; }                   // Bottom-right
    pub fn bottom_right(&self) -> Vector2D {
        self.max.clone()
    }
    //     bool        contains(const Vector2D& p) const     { return p.x     >= min.x && p.y     >= min.y && p.x     <  max.x && p.y     <  max.y; }
    pub fn contains_vector(&self, p: &Vector2D) -> bool {
        p.x >= self.min.x && p.y >= self.min.y && p.x < self.max.x && p.y < self.max.y
    }
    //     bool        contains(const ImRect& r) const     { return r.min.x >= min.x && r.min.y >= min.y && r.max.x <= max.x && r.max.y <= max.y; }
    pub fn contains_rect(&self, r: &Self) -> bool {
        r.min.x >= self.min.x && r.min.y >= self.min.y && r.max.x <= self.max.x && r.max.y <= self.max.y
    }
    //     bool        Overlaps(const ImRect& r) const     { return r.min.y <  max.y && r.max.y >  min.y && r.min.x <  max.x && r.max.x >  min.x; }
    pub fn overlaps_rect(&self, r: &Self) -> bool {
        r.min.y < self.max.y && r.max.y > self.min.y && r.min.x < self.max.x && r.max.x > self.min.x
    }
    //     void        Add(const Vector2D& p)                { if (min.x > p.x)     min.x = p.x;     if (min.y > p.y)     min.y = p.y;     if (max.x < p.x)     max.x = p.x;     if (max.y < p.y)     max.y = p.y; }
    pub fn add_vector(&mut self, p: &Vector2D) {
        if self.min.x > p.x {
            self.min.x = p.x
        }
        if self.min.y > p.y {
            self.min.y = p.y
        }
        if self.max.x < p.x {
            self.max.x = p.x
        }
        if self.max.y < p.y {
            self.max.y = p.y
        }
    }
    //     void        Add(const ImRect& r)                { if (min.x > r.min.x) min.x = r.min.x; if (min.y > r.min.y) min.y = r.min.y; if (max.x < r.max.x) max.x = r.max.x; if (max.y < r.max.y) max.y = r.max.y; }
    pub fn add_rect(&mut self, r: &Self) {
        if self.min.x > r.min.x {
            self.min.x = r.min.x
        }
        if self.Miny > r.min.y {
            self.min.y = r.min.y
        }
        if self.max.x < r.max.x {
            self.max.x = r.max.x
        }
        if self.max.y < r.max.y {
            self.max.y = r.max.y
        }
    }
    //     void        Expand(let amount)          { min.x -= amount;   min.y -= amount;   max.x += amount;   max.y += amount; }
    pub fn expand_float(&mut self, amount: f32) {
        self.min.x -= amount;
        self.min.y -= amount;
        self.max.x += amount;
        self.max.y += amount;
    }
    //     void        Expand(const Vector2D& amount)        { min.x -= amount.x; min.y -= amount.y; max.x += amount.x; max.y += amount.y; }
    pub fn expand_vector(&mut self, amount: &Vector2D) {
        self.min.x -= amount.x;
        self.min.y -= amount.y;
        self.max.x += amount.x;
        self.max.y += amount.y;
    }
    //     void        Translate(const Vector2D& d)          { min.x += d.x; min.y += d.y; max.x += d.x; max.y += d.y; }
    pub fn translate_vector(&mut self, d: &Vector2D) {
        self.min.x += d.x;
        self.min.y += d.y;
        self.max.x += d.x;
        self.max.y += d.y;
    }
    //     void        TranslateX(float dx)                { min.x += dx; max.x += dx; }
    pub fn translate_x_f32(&mut self, dx: f32) {
        self.min.x += dx;
        self.max.x += dx;
    }
    //     void        TranslateY(float dy)                { min.y += dy; max.y += dy; }
    pub fn translate_y_f32(&mut self, dy: f32) {
        self.min.y += dy;
        self.max.y += dy;
    }
    //     void        ClipWith(const ImRect& r)           { min = ImMax(min, r.min); max = ImMin(max, r.max); }                   // Simple version, may lead to an inverted rectangle, which is fine for contains/Overlaps test but not for display.
    pub fn clip_width(&mut self, r: &Self) {
        self.min = ImMaxVec2(&self.min, &r.min);
        self.max = ImMinVec2(&self.max, &r.max);
    }
    //     void        ClipWithFull(const ImRect& r)       { min = ImClamp(min, r.min, r.max); max = ImClamp(max, r.min, r.max); } // Full version, ensure both points are fully clipped.
    pub fn clip_width_full(&mut self, r: &Self) {
        self.min = ImClampVec2(&self.min, &r.min, &r.max);
        self.max = ImClampVec2(&self.max, &r.min, &r.max);
    }
    //     void        Floor()                             { min.x = IM_FLOOR(min.x); min.y = IM_FLOOR(min.y); max.x = IM_FLOOR(max.x); max.y = IM_FLOOR(max.y); }
    pub fn floor(&mut self) {
        self.min.x = f32::floor(self.min.x);
        self.min.y = f32::floor(self.min.y);
        self.max.x = f32::floor(self.max.x);
        self.max.y = f32::floor(self.max.y);
    }
    //     bool        is_inverted() const                  { return min.x > max.x || min.y > max.y; }
    pub fn is_inverted(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y
    }
    //     Vector4D      ToVec4() const                      { return Vector4D(min.x, min.y, max.x, max.y); }
    pub fn to_vector_4d(&self) -> Vector4D {
        Vector4D {
            x: self.min.x,
            y: self.min.y,
            z: self.max.x,
            w: self.max.y,
        }
    }
}

impl Into<Vector4D> for Rect {
    fn into(&self) -> Vector4D {
        Vector4D {
            x: self.min.x,
            y: self.min.y,
            z: self.max.x,
            w: self.max.y,
        }
    }
}
