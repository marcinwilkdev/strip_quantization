use image::{Rgb, DynamicImage, GenericImageView};

pub struct ColorStats {
    means: Vec<u8>,
    variances: Vec<u8>,
}

impl ColorStats {
    pub fn new(size: usize) -> ColorStats {
        ColorStats {
            means: Vec::with_capacity(size),
            variances: Vec::with_capacity(size),
        }
    }

    pub fn add_colors(&mut self, first_pixel: u8, second_pixel: u8) {
        let mut mean = ((first_pixel as u16 + second_pixel as u16) / 2) as u8;
        let variance = second_pixel - mean;

        match (first_pixel % 2, second_pixel % 2) {
            (1, 0) | (0, 1) => mean += 1,
            (1, 1) => mean += 2,
            _ => (),
        }

        self.means.push(mean);
        self.variances.push(variance);
    }

    pub fn add_color(&mut self, pixel: u8) {
        self.means.push(pixel);
        self.variances.push(0);
    }

    pub fn difference_means(&mut self) {
        let mut last_value = self.means[0];

        for i in 1..self.means.len() {
            let tmp = self.means[i];

            self.means[i] = tmp - last_value;

            last_value = tmp;
        }
    }

    pub fn revert_difference_means(&mut self) {
        for i in 1..self.means.len() {
            self.means[i] = self.means[i - 1] + self.means[i];
        }
    }

    pub fn means(&self) -> &[u8] {
        &self.means
    }

    pub fn variances(&self) -> &[u8] {
        &self.variances
    }
}

#[inline]
pub fn split_rgb_to_colors(rgb: &Rgb<u8>) -> [u8; 3] {
    [rgb[0], rgb[1], rgb[2]]
}

pub fn add_pixel_to_colors(
    pixel: &Rgb<u8>,
    red_color_stats: &mut ColorStats,
    green_color_stats: &mut ColorStats,
    blue_color_stats: &mut ColorStats,
) {
    let [red, green, blue] = split_rgb_to_colors(pixel);

    red_color_stats.add_color(red);
    green_color_stats.add_color(green);
    blue_color_stats.add_color(blue);
}

pub fn add_pixels_to_colors(
    first_pixel: &Rgb<u8>,
    second_pixel: &Rgb<u8>,
    red_color_stats: &mut ColorStats,
    green_color_stats: &mut ColorStats,
    blue_color_stats: &mut ColorStats,
) {
    let [first_red, first_green, first_blue] = split_rgb_to_colors(first_pixel);
    let [second_red, second_green, second_blue] = split_rgb_to_colors(second_pixel);

    red_color_stats.add_colors(first_red, second_red);
    green_color_stats.add_colors(first_green, second_green);
    blue_color_stats.add_colors(first_blue, second_blue);
}

pub fn get_colors_stats(img: &DynamicImage) -> (ColorStats, ColorStats, ColorStats) {
    let rgb_img = img.to_rgb8();

    let (width, height) = img.dimensions();
    let expected_pixels_count = (width * height) as usize;

    let mut red_color_stats = ColorStats::new(expected_pixels_count);
    let mut green_color_stats = ColorStats::new(expected_pixels_count);
    let mut blue_color_stats = ColorStats::new(expected_pixels_count);

    let mut pixels = rgb_img.pixels();

    let first_pixel = pixels.next().unwrap();

    add_pixel_to_colors(
        first_pixel,
        &mut red_color_stats,
        &mut green_color_stats,
        &mut blue_color_stats,
    );

    loop {
        let first_pixel = pixels.next();
        let second_pixel = pixels.next();

        match (first_pixel, second_pixel) {
            (None, None) => break,
            (Some(pixel), None) => {
                add_pixel_to_colors(
                    pixel,
                    &mut red_color_stats,
                    &mut green_color_stats,
                    &mut blue_color_stats,
                );
            }
            (Some(first_pixel), Some(second_pixel)) => {
                add_pixels_to_colors(
                    first_pixel,
                    second_pixel,
                    &mut red_color_stats,
                    &mut green_color_stats,
                    &mut blue_color_stats,
                );
            }
            _ => unreachable!(),
        }
    }

    (red_color_stats, green_color_stats, blue_color_stats)
}
