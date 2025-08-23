use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    // indicatif
    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green}    [{wide_bar:.cyan/blue}]      {pos}/{len} ETA {eta}           ",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    for j in 0..image_height {
        for i in 0..image_width {
            let image_width_f64 = image_width as f64;
            let image_height_f64 = image_height as f64;

            let r = i as f64 / image_width_f64;
            let g = j as f64 / image_height_f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}");
        }
        pb.inc(1);
    }
    pb.finish_with_message("Done");
}
