use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

const TWO_PI: f64 = 6.28318530718;
const EPSILON: f64 = 1e-6;

pub struct Scene {
    width: u32,
    height: u32,
    shapes: Vec<Circle>,
    sample_count: u8,
    max_step: usize,
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Scene {
        Scene {
            width,
            height,
            sample_count: 64,
            shapes: vec![],
            max_step: 10,
        }
    }

    pub fn add_shape(&mut self, shape: Circle) {
        self.shapes.push(shape);
    }

    pub fn render_to_file(&self, path: &str) {
        let mut image = vec![0u8; self.width as usize * self.height as usize * 3];

        for x in 0..self.width {
            for y in 0..self.height {
                let index = ((y * self.width + x) * 3) as usize;
                let value = self.sample(x as f64, y as f64);
                image[index] = value;
                image[index + 1] = value;
                image[index + 2] = value;
            }
        }

        self.save_to_file(&image, path);
    }

    // 对图片中的某个点进行采样
    // 也就是计算有多少光经过了这个点
    fn sample(&self, x: f64, y: f64) -> u8 {
        let mut rng = rand::thread_rng();

        let mut sum: f64 = 0.0;
        for i in 0..self.sample_count {
            let degree = TWO_PI * (i as f64 + rng.gen_range(0.0..1.0)) / self.sample_count as f64;
            sum += self.trace(x, y, degree.cos(), degree.sin());
        }

        let mut sum = sum / self.sample_count as f64 * 255.0;
        if sum >= 255.0 {
            sum = 255.0;
        }
        return sum as u8;
    }

    // 获取 (x, y) 点从 (dx, dy) 方向获取的光量
    fn trace(&self, x: f64, y: f64, dx: f64, dy: f64) -> f64 {
        let max_distance = ((self.width.pow(2) + self.width.pow(2)) as f64).sqrt();

        let mut distance: f64 = 0.0;
        for _ in 0..self.max_step {
            let result = self.sdf(x + (dx * distance), y + (dy * distance));
            if result.sd < EPSILON {
                return result.emissive;
            }
            distance += result.sd;
            if distance >= max_distance {
                break;
            }
        }
        return 0.0;
    }

    fn sdf(&self, x: f64, y: f64) -> SdfResult {
        let mut result = SdfResult {
            sd: f64::MAX,
            emissive: 0.0
        };
        for shape in self.shapes.iter() {
            result = Scene::union_sd(shape.sdf(x, y), result);
        }

        return result;
    }

    // 对两个形状做并集
    // 此时 sd 的结果应该是两个形状当中 sd 比较小的那个
    fn union_sd(result_a: SdfResult, result_b: SdfResult) -> SdfResult {
        return if result_a.sd < result_b.sd { result_a } else { result_b };
    }

    fn save_to_file(&self, image: &Vec<u8>, path: &str) {
        fs::remove_file(path).unwrap_or_default();
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(image.as_slice()).unwrap();
    }
}

struct SdfResult {
    // 带符号距离 signed distance
    sd: f64,

    // 自发光强度
    emissive: f64,
}

pub struct Circle {
    ox: f64,
    oy: f64,
    r: f64,
    emissive: f64,
}

impl Circle {
    pub fn new(ox: f64, oy: f64, r: f64, emissive: f64) -> Circle {
        Circle { ox, oy, r, emissive }
    }

    // 计算 (x, y) 点离这个圆的 SDF(也就是到这个圆的边的最近距离)
    fn sdf(&self, x: f64, y: f64) -> SdfResult {
        let ux = x - self.ox;
        let uy = y - self.oy;

        let sd = ((ux * ux + uy * uy) as f64).sqrt() - self.r as f64;
        return SdfResult {
            sd,
            emissive: self.emissive
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_circle() {
        let width: f64 = 512.0;
        let height: f64 = 384.0;
        let mut scene = Scene::new(width as u32, height as u32);
        scene.add_shape(Circle::new(width * 0.3, height * 0.3, width * 0.1, 2.0));
        scene.add_shape(Circle::new(width * 0.3, height * 0.7, width * 0.05, 0.8));
        scene.add_shape(Circle::new(width * 0.7, height * 0.5, width * 0.10, 0.8));
        scene.render_to_file("./image.png");
    }
}
