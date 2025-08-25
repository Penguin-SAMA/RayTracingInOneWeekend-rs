use std::f64::consts::PI;

pub const INFINITY: f64 = f64::INFINITY;

#[inline]
pub const fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod prelude;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use camera::Camera;
pub use color::{write_color, Color};
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use interval::Interval;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Point3, Vec3};
