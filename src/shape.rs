
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
        let t = ((vx * ux + vy * uy) / (ux * ux + uy * uy))
            .min(1.0)
            .max(0.0);
        let dx = vx - ux * t;
        let dy = vy - uy * t;
        let segment_sd = (dx * dx + dy * dy).sqrt();
        let capsule_sd = segment_sd - self.r;

        SdfResult {
            sd: capsule_sd,
            emissive: self.emissive,
        }
    }
}

pub struct Rect {
    // 矩形由中心点(cx, cy), 旋转角(theta), 半长(sx, sy) 组成
    cx: f64,
    cy: f64,
    theta: f64,
    sx: f64,
    sy: f64,
    emissive: f64,
    // 圆角矩形的半径
    r: f64,
}

impl Rect {
    pub fn new(cx: f64, cy: f64, theta: f64, sx: f64, sy: f64, emissive: f64) -> Rect {
        Rect {
            cx,
            cy,
            theta,
            sx,
            sy,
            emissive,
            r: 0.0,
        }
    }
}

impl Shape for Rect {
    fn sdf(&self, x: f64, y: f64) -> SdfResult {
        let sin_theta = self.theta.sin();
        let cos_theta = self.theta.cos();
        let dx = ((x - self.cx) * cos_theta + (y - self.cy) * sin_theta).abs() - self.sx;
        let dy = ((y - self.cy) * cos_theta - (x - self.cx) * sin_theta).abs() - self.sy;
        let ax = dx.max(0.0);
        let ay = dy.max(0.0);
        let sd = dx.max(dy).min(0.0) + (ax * ax + ay * ay).sqrt();
        return SdfResult {
            sd,
            emissive: self.emissive,
        };
    }
}

pub struct Triangle {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    cx: f64,
    cy: f64,
    emissive: f64,
    // 圆角三角形的半径
    r: f64,
}

impl Triangle {
    pub fn new(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, emissive: f64) -> Triangle {
        Triangle {
            ax,
            ay,
            bx,
            by,
            cx,
            cy,
            emissive,
            r: 0.0,
        }
    }

    fn segment_sdf(x: f64, y: f64, ax: f64, ay: f64, bx: f64, by:f64) -> f64 {
        let vx = x - ax;
        let vy = y - ay;
        let ux = bx - ax;
        let uy = by - ay;
        let t = ((vx * ux + vy * uy) / (ux * ux + uy * uy))
            .min(1.0)
            .max(0.0);
        let dx = vx - ux * t;
        let dy = vy - uy * t;
        return (dx * dx + dy * dy).sqrt();
    }
}

impl Shape for Triangle {
    fn sdf(&self, x: f64, y: f64) -> SdfResult {
        let result1 = Triangle::segment_sdf(x, y, self.ax, self.ay, self.bx, self.by);
        let result2 = Triangle::segment_sdf(x, y, self.bx, self.by, self.cx, self.cy);
        let result3 = Triangle::segment_sdf(x, y, self.cx, self.cy, self.ax, self.ay);

        // 三角形的 sd 是三个线段中距离最近的那个
        let mut sd = result1.min(result2).min(result3);

        // 如果在三角形内，那么返回 -sd
        if (self.bx - self.ax) * (y - self.ay) > (self.by - self.ay) * (x - self.ax) &&
            (self.cx - self.bx) * (y - self.by) > (self.cy - self.by) * (x - self.bx) &&
            (self.ax - self.cx) * (y - self.cy) > (self.ay - self.cy) * (x - self.cx) {
            sd = -sd;
        }

        return SdfResult {
            sd,
            emissive: self.emissive
        }
    }
}
