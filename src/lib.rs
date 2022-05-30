mod color_stats;
mod quantize;
mod stats;
mod cmp;

use image::Rgb;

const EPSILON: f64 = 0.01;

pub fn perform_quantization(colors_bits: u32, input: &str, output: &str) {
    let colors_count = (1 << colors_bits) as u8;

    let img = image::open(input).expect("couldn't open an image");

    let (mut red_color_stats, mut green_color_stats, mut blue_color_stats) =
        color_stats::get_colors_stats(&img);

    red_color_stats.difference_means();
    green_color_stats.difference_means();
    blue_color_stats.difference_means();

    let red_variances = red_color_stats.variances();
    let green_variances = green_color_stats.variances();
    let blue_variances = blue_color_stats.variances();

    let red_variances = quantize::quantize_variances(colors_count, EPSILON, red_variances);
    let green_variances = quantize::quantize_variances(colors_count, EPSILON, green_variances);
    let blue_variances = quantize::quantize_variances(colors_count, EPSILON, blue_variances);

    let variances = red_variances
        .into_iter()
        .zip(green_variances)
        .zip(blue_variances)
        .map(|((r, g), b)| Rgb::from([r, g, b]))
        .collect::<Vec<_>>();

    red_color_stats.revert_difference_means();
    green_color_stats.revert_difference_means();
    blue_color_stats.revert_difference_means();

    let mut red_means_iter = red_color_stats.means().iter();
    let mut green_means_iter = green_color_stats.means().iter();
    let mut blue_means_iter = blue_color_stats.means().iter();

    let mut rgb_img = img.to_rgb8();
    let mut mut_pixels = rgb_img.pixels_mut();

    let mut variances_iter = variances.iter();

    {
        let first_pixel = mut_pixels.next().unwrap();

        let red_mean = red_means_iter.next();
        let green_mean = green_means_iter.next();
        let blue_mean = blue_means_iter.next();

        let variance = variances_iter.next().unwrap();

        let red_mean = red_mean.unwrap();
        let green_mean = green_mean.unwrap();
        let blue_mean = blue_mean.unwrap();

        let first_pixel_red = red_mean + variance[0];
        let first_pixel_green = green_mean + variance[1];
        let first_pixel_blue = blue_mean + variance[2];

        *first_pixel = Rgb::from([first_pixel_red, first_pixel_green, first_pixel_blue]);
    }

    loop {
        let first_pixel = mut_pixels.next();
        let second_pixel = mut_pixels.next();

        match (first_pixel, second_pixel) {
            (None, None) => break,
            (Some(last_pixel), None) => {
                let red_mean = red_means_iter.next();
                let green_mean = green_means_iter.next();
                let blue_mean = blue_means_iter.next();

                let variance = variances_iter.next().unwrap();

                let red_mean = red_mean.unwrap();
                let green_mean = green_mean.unwrap();
                let blue_mean = blue_mean.unwrap();

                let first_pixel_red = red_mean + variance[0];
                let first_pixel_green = green_mean + variance[1];
                let first_pixel_blue = blue_mean + variance[2];

                *last_pixel = Rgb::from([first_pixel_red, first_pixel_green, first_pixel_blue]);
            }
            (Some(first_pixel), Some(second_pixel)) => {
                let red_mean = red_means_iter.next();
                let green_mean = green_means_iter.next();
                let blue_mean = blue_means_iter.next();

                let variance = variances_iter.next().unwrap();

                let red_mean = red_mean.unwrap();
                let green_mean = green_mean.unwrap();
                let blue_mean = blue_mean.unwrap();

                let first_pixel_red = red_mean - variance[0];
                let first_pixel_green = green_mean - variance[1];
                let first_pixel_blue = blue_mean - variance[2];

                let second_pixel_red = red_mean + variance[0];
                let second_pixel_green = green_mean + variance[1];
                let second_pixel_blue = blue_mean + variance[2];

                *first_pixel = Rgb::from([first_pixel_red, first_pixel_green, first_pixel_blue]);
                *second_pixel =
                    Rgb::from([second_pixel_red, second_pixel_green, second_pixel_blue]);
            }
            _ => unreachable!(),
        }
    }

    rgb_img.save(output).expect("couldnt save image");

    cmp::print_mean_squared_and_noise(input, output);
}
