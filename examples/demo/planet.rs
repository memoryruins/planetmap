use noise::NoiseFn;

use planetmap::chunk::Face;

pub struct Planet {
    noise: noise::Fbm,
    radius: f64,
}

impl Planet {
    pub fn new() -> Self {
        Self {
            noise: Default::default(),
            radius: 6371e3,
        }
    }

    /// Radial heightmap function
    pub fn height_at(&self, dir: &na::Unit<na::Vector3<f64>>) -> f64 {
        self.sample(self.noise_space(dir))
    }

    pub fn normal_at(&self, dir: &na::Unit<na::Vector3<f64>>) -> na::Unit<na::Vector3<f32>> {
        let basis = Face::from_vector(dir).basis::<f64>();
        let center = self.noise_space(dir);
        let perp = basis.matrix().index((.., 1));
        let x = dir.into_inner().cross(&perp);
        let y = dir.into_inner().cross(&x);
        let h = 1e-8;
        let x_off = x * h;
        let y_off = y * h;
        let x_0 = self.sample(center - x_off);
        let x_1 = self.sample(center + x_off);
        let y_0 = self.sample(center - y_off);
        let y_1 = self.sample(center + y_off);
        let dx = na::Vector3::new(2.0*h, 0.0, x_1 - x_0);
        let dy = na::Vector3::new(0.0, 2.0*h, y_1 - y_0);
        na::Unit::new_normalize(na::convert(dx.cross(&dy)))
    }

    pub fn color_at(&self, dir: &na::Unit<na::Vector3<f64>>) -> [u8; 4] {
        let height = self.height_at(dir);
        if height == 0.0 {
            [0, 0, 128, 255]
        } else if height < 2000.0 {
            [0, 128, 0, 255]
        } else {
            [255, 255, 255, 255]
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    fn sample(&self, x: na::Point3<f64>) -> f64 {
        let point: [f64; 3] = x.coords.into();
        let h = ((1.0 + self.noise.get(point)).powi(3) - 1.0) * 1000.0;
        if h < 0.0 { 0.0 } else { h }
    }

    fn noise_space(&self, dir: &na::Unit<na::Vector3<f64>>) -> na::Point3<f64> {
        na::Point3::from(dir.into_inner() * self.radius * 5e-5)
    }
}
