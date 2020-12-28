use std::panic::resume_unwind;
use std::cmp::{min, max};

pub struct SdfResult {
    // 带符号距离 signed distance
    pub sd: f64,

    // 自发光强度
    pub emissive: f64,
}

pub trait Shape {
    fn sdf(&self, x: f64, y: f64) -> SdfResult;
}

pub struct UnionShape {
    shape1: Box<dyn Shape>,
    shape2: Box<dyn Shape>,
}

impl Shape for UnionShape {
    fn sdf(&self, x: f64, y: f64) -> SdfResult {
        let result1 = self.shape1.sdf(x, y);
        let result2 = self.shape2.sdf(x, y);

        return if result1.sd < result2.sd {
            result1
        } else {
            result2
        };
    }
}

pub struct IntersectShape {
    shape1: Box<dyn Shape>,
    shape2: Box<dyn Shape>,
}

impl Shape for IntersectShape {
    fn sdf(&self, x: f64, y: f64) -> SdfResult {
        let mut result1 = self.shape1.sdf(x, y);
        let mut result2 = self.shape2.sdf(x, y);

        return if result1.sd > result2.sd {
            result2.sd = result1.sd;
            result2
        } else {
            result1.sd = result2.sd;
            result1
        };
    }
}

pub struct SubtractShape {
    shape1: Box<dyn Shape>,
    shape2: Box<dyn Shape>,
}

impl Shape for SubtractShape {
    fn sdf(&self, x: f64, y: f64) -> SdfResult {
        let mut result1 = self.shape1.sdf(x, y);
        let result2 = self.shape2.sdf(x, y);
        let sd = if result1.sd > -result2.sd {
            result1.sd
        } else {
            -result2.sd
        };
        result1.sd = sd;

        return result1;
    }
}

pub struct Shapes;

impl Shapes {
    pub fn union(shape1: Box<dyn Shape>, shape2: Box<dyn Shape>) -> Box<UnionShape> {
        Box::new(UnionShape { shape1, shape2 })
    }

    pub fn intersect(shape1: Box<dyn Shape>, shape2: Box<dyn Shape>) -> Box<IntersectShape> {
        Box::new(IntersectShape { shape1, shape2 })
    }

    pub fn subtract(shape1: Box<dyn Shape>, shape2: Box<dyn Shape>) -> Box<SubtractShape> {
        Box::new(SubtractShape { shape1, shape2 })
    }
}

pub struct Circle {
    ox: f64,
    oy: f64,
    r: f64,
    emissive: f64,
}

impl Circle {
    pub fn new(ox: f64, oy: f64, r: f64, emissive: f64) -> Circle {
        Circle {
            ox,
            oy,
            r,
            emissive,
        }
    }
}

impl Shape for Circle {
    // 计算 (x, y) 点离这个圆的 SDF(也就是到这个圆的边的最近距离)
    fn sdf(&self, x: f64, y: f64) -> SdfResult {
        let ux = x - self.ox;
        let uy = y - self.oy;

        let sd = ((ux * ux + uy * uy) as f64).sqrt() - self.r as f64;
        return SdfResult {
            sd,
            emissive: self.emissive,
        };
    }
}

pub struct Plane {
    // 用一个点和法线来确定一个平面
    px: f64,
    py: f64,
    nx: f64,
    ny: f64,
    emissive: f64,
}

impl Plane {
    pub fn new(px: f64, py: f64, nx: f64, ny: f64, emissive: f64) -> Plane {
        Plane {
            px,
            py,
            nx,
            ny,
            emissive,
        }
    }
}

impl Shape for Plane {
    fn sdf(&self, x: f64, y: f64) -> SdfResult {
        return SdfResult {
            sd: (x - self.px) * self.nx + (y - self.py) * self.ny,
            emissive: self.emissive,
        };
    }
}

pub struct Capsule {
    // 用两个点和半径来表示胶囊
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    r: f64,
    emissive: f64,
}

impl Capsule {
    pub fn new(ax: f64, ay: f64, bx: f64, by: f64, r: f64, emissive: f64) -> Capsule {
        Capsule {
            ax,
            ay,
            bx,
            by,
            r,
            emissive,
        }
    }
}

impl Shape for Capsule {
    fn sdf(&self, x: f64, y: f64) -> SdfResult {
        let vx = x - self.ax;
        let vy = y - self.ay;
        let ux = self.bx - self.ax;
        let uy = self.by - self.ay;
        let t = ((vx * ux + vy * uy) / (ux * ux + uy * uy)).min(1.0).max(0.0);
        let dx = vx - ux * t;
        let dy = vy - uy * t;
        let segment_sd = (dx * dx + dy * dy).sqrt();
        let capsule_sd = segment_sd - self.r;

        SdfResult {
            sd: capsule_sd,
            emissive: self.emissive
        }
    }
}
