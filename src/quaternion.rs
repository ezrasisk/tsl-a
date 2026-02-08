use std::f64::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Quaternion { w, x, y, z }
    }

    pub fn norm(&self) -> f64 {
        (self.w.powi(2) + self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let n = self.norm();
        if n == 0.0 {
            Quaternion::new(1.0, 0.0, 0.0, 0.0)
        } else {
            Quaternion::new(self.w / n, self.x / n, self.y / n, self.z / n)
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        Quaternion::new(
            self.w*other.w - self.x*other.x - self.y*other.y - self.z*other.z,
            self.w*other.x + self.x*other.w + self.y*other.z - self.z*other.y,
            self.w*other.y - self.x*other.z + self.y*other.w + self.z*other.x,
            self.w*other.z + self.x*other.y - self.y*other.x + self.z*other.w,
        )
    }

    pub fn scale(&self, s: f64) -> Self {
        Quaternion::new(self.w * s, self.x * s, self.y * s, self.z * s)
    }

    pub fn add(&self, other: &Self) -> Self {
        Quaternion::new(self.w + other.w, self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn rotation_angle_deg(&self) -> f64 {
        let cos_half = self.w.clamp(-1.0, 1.0);
        2.0 * cos_half.acos() * 180.0 / PI
    }
}
