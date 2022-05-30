use image::Rgb;

use crate::color_stats;

pub fn noise_part([r, g, b]: [u8; 3]) -> u32 {
    r as u32 + g as u32 + b as u32
}

pub fn get_colors_counts(pixels: &[&Rgb<u8>]) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut red_count = vec![0; 1 << 8];
    let mut green_count = vec![0; 1 << 8];
    let mut blue_count = vec![0; 1 << 8];

    for pixel in pixels {
        let [r, g, b] = color_stats::split_rgb_to_colors(pixel);

        red_count[r as usize] += 1;
        green_count[g as usize] += 1;
        blue_count[b as usize] += 1;
    }

    (red_count, green_count, blue_count)
}

pub fn print_mean_squared_and_noise(image1: &str, image2: &str) {
    let image1 = image::open(image1)
        .expect("couldn't open an image")
        .to_rgb8();
    let image2 = image::open(image2)
        .expect("couldn't open an image")
        .to_rgb8();

    let (image2_reds, image2_greens, image2_blues) =
        get_colors_counts(&image2.pixels().collect::<Vec<_>>()[..]);

    let mut image1_pixels = image1.pixels();
    let mut image2_pixels = image2.pixels();

    // let mut signal_to_noise;

    let mut red_distances = 0;
    let mut green_distances = 0;
    let mut blue_distances = 0;

    loop {
        let image1_pixel = image1_pixels.next();
        let image2_pixel = image2_pixels.next();

        if image1_pixel.is_none() || image2_pixel.is_none() {
            break;
        }

        let image1_pixel = image1_pixel.unwrap();
        let image2_pixel = image2_pixel.unwrap();

        let [img1_r, img1_g, img1_b] = color_stats::split_rgb_to_colors(image1_pixel);
        let [img2_r, img2_g, img2_b] = color_stats::split_rgb_to_colors(image2_pixel);

        red_distances +=
            crate::stats::dist_squared(img1_r, img2_r) as u32 * image2_reds[img2_r as usize];
        green_distances +=
            crate::stats::dist_squared(img1_g, img2_g) as u32 * image2_greens[img2_g as usize];
        blue_distances +=
            crate::stats::dist_squared(img1_b, img2_b) as u32 * image2_blues[img2_b as usize];

        // signal_to_noise += noise_part([img2_r, img2_g, img2_b]) * img_colors_counts[img_color];
    }

    let pixels_count = image2.pixels().len();

    let r_mean_squared_error = red_distances as f64 / pixels_count as f64;
    let g_mean_squared_error = green_distances as f64 / pixels_count as f64;
    let b_mean_squared_error = blue_distances as f64 / pixels_count as f64;

    println!(
        "R: {}\nG: {}\nB: {}",
        r_mean_squared_error, g_mean_squared_error, b_mean_squared_error
    );
}
