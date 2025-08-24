#![allow(unused)]
use std::io::BufWriter;

use indicatif::{ProgressBar, ProgressStyle};
use rtow_rs::*;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = vec3::unit(r.direction());
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // indicatif
    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green}    [{wide_bar:.cyan/blue}]      {pos}/{len} ETA {eta}           ",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    // World
    let mut world = HittableList::default();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::default();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // stdout
    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    // Render

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(&mut out, pixel_color).unwrap();
        }
        pb.inc(1);
    }
    pb.finish_with_message("Done");
}
