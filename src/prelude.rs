use rand::Rng;
use std::f64::consts::PI;

pub const INFINITY: f64 = f64::INFINITY;

// Utility functions
#[inline]
pub const fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}

#[inline]
pub fn random_f64() -> f64 {
    rand::rng().random_range(0.0..1.0)
}

#[inline]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}

// re-exports
pub use crate::camera::Camera;
pub use crate::color::{write_color, Color};
pub use crate::hittable::{HitRecord, Hittable};
pub use crate::hittable_list::HittableList;
pub use crate::interval::Interval;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::{Point3, Vec3};
