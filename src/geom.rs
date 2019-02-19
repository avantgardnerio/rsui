pub use vecmath::vec2_add as add;

pub type Point = [f64; 2];

pub type Color = [f32; 4];

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub origin: Point,
    pub size: Point
}

impl Rect {
    pub fn min() -> Rect {
        let origin: Point = [std::f64::INFINITY, std::f64::INFINITY];
        let size: Point = [std::f64::NEG_INFINITY, std::f64::NEG_INFINITY];
        return Rect { origin, size };
    }

    pub fn translate(&self, p: Point) -> Rect {
        return Rect {
            origin: add(self.origin, p),
            size: self.size
        }
    }

    pub fn to_u32(&self) -> [u32; 4] {
        return [
            self.origin[0] as u32,
            self.origin[1] as u32,
            self.size[0] as u32,
            self.size[1] as u32
        ];
    }

    pub fn union(a: Rect, b: Rect) -> Rect {
        let w = f64::min(a.origin[0], b.origin[0]);
        let n = f64::min(a.origin[1], b.origin[1]);
        let e = f64::max(a.origin[0] + a.size[0], b.origin[0] + b.size[0]);
        let s = f64::max(a.origin[1] + a.size[1], b.origin[1] + b.size[1]);
        let origin: Point = [w, n];
        let size: Point = [e - w, s - n];
        return Rect {origin, size};
    }

    pub fn intersection(a: Rect, b: Rect) -> Rect {
        let w = f64::max(a.origin[0], b.origin[0]);
        let n = f64::max(a.origin[1], b.origin[1]);
        let e = f64::min(a.origin[0] + a.size[0], b.origin[0] + b.size[0]);
        let s = f64::min(a.origin[1] + a.size[1], b.origin[1] + b.size[1]);
        let origin: Point = [w, n];
        let size: Point = [e - w, s - n];
        return Rect {origin, size};
    }
}
