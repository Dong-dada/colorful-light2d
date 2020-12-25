pub struct SdfResult {
    // 带符号距离 signed distance
    pub sd: f64,

    // 自发光强度
    pub emissive: f64,
}

pub trait Shape {
    fn sdf(&self, x: f64, y: f64) -> SdfResult;
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
