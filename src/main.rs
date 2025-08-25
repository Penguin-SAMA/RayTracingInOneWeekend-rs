#![allow(unused)]
use rtow_rs::*;

fn main() {
    let mut world = HittableList::default();

    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.0), 100.0)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.render(&world);
}
