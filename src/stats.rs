use rand::prelude::*;

pub fn gen_random_colors(num_colors: u8) -> Vec<u8> {
    let mut color = vec![false; 1 << 8];
    let mut generated_colors = 0;

    while generated_colors < num_colors {
        let generated = thread_rng().gen::<u8>();

        if !color[generated as usize] {
            color[generated as usize] = true;
            generated_colors += 1;
        }
    }

    color
        .iter()
        .enumerate()
        .filter(|(_, &c)| c)
        .map(|(i, _)| i as u8)
        .collect()
}

pub fn get_colors_counts(original: &[u8]) -> Vec<u32> {
    let mut colors_count = vec![0; 1 << 8];

    for &number in original {
        colors_count[number as usize] += 1;
    }

    colors_count
}

pub fn dist(p1: u8, p2: u8) -> u8 {
    if p1 < p2 {
        p2 - p1
    } else {
        p1 - p2
    }
}

pub fn dist_squared(p1: u8, p2: u8) -> u8 {
    let dist = dist(p1, p2);

    dist * dist
}

pub fn quant_area_avg(group: &[usize]) -> u8 {
    let mut sum = 0;

    for &color in group {
        let color = color as u32;

        sum += color;
    }

    sum /= group.len() as u32;

    sum as u8
}
