use std::io::BufWriter;

use indicatif::{ProgressBar, ProgressStyle};

mod color;
use color::Color;
mod vec3;

fn main() {
    // Image

    let image_width: usize = 256;
    let image_height: usize = 256;

    // let inv_w = 1.0 / (image_width - 1) as f64;
    // let inv_h = 1.0 / (image_height - 1) as f64;

    // indicatif
    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green}    [{wide_bar:.cyan/blue}]      {pos}/{len} ETA {eta}           ",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    // stdout
    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    // Render

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.,
            );
            color::write_color(&mut out, pixel_color).unwrap();
        }
        pb.inc(1);
    }
    pb.finish_with_message("Done");
}
