pub fn quantize_variances(num_colors: u8, epsilon: f64, variances: &[u8]) -> Vec<u8> {
    let mut quant_colors = crate::stats::gen_random_colors(num_colors);

    let mut curr_error = 0;

    let img_colors_counts = crate::stats::get_colors_counts(&variances); // for probability calculating

    let mut img_to_quant_color_map = vec![0; img_colors_counts.len()];

    loop {
        let mut quant_areas = vec![vec![]; num_colors as usize];

        for (img_color, _) in img_colors_counts
            .iter()
            .enumerate()
            .filter(|(_, &count)| count > 0)
        {
            let (_, img_color, quant_color_index) = quant_colors
                .iter()
                .enumerate()
                .map(|(quant_color_index, &quant_color)| {
                    (
                        crate::stats::dist(img_color as u8, quant_color),
                        img_color,
                        quant_color_index,
                    )
                })
                .min_by(|(d1, _, _), (d2, _, _)| d1.cmp(d2))
                .unwrap();

            quant_areas[quant_color_index].push(img_color);
            img_to_quant_color_map[img_color] = quant_colors[quant_color_index];
        }

        let mut sum_distances = 0;

        for area_index in 0..num_colors {
            for &img_color in &quant_areas[area_index as usize] {
                sum_distances +=
                    crate::stats::dist_squared(quant_colors[area_index as usize], img_color as u8)
                        as u32
                        * img_colors_counts[img_color];
            }
        }

        if sum_distances == 0 {
            sum_distances = 1;
        }

        let mean_squared_error =
            ((sum_distances as f64 - curr_error as f64) / sum_distances as f64).abs();

        curr_error = sum_distances;

        if mean_squared_error < epsilon {
            break;
        }

        for area_index in 0..num_colors {
            if quant_areas[area_index as usize].len() > 0 {
                quant_colors[area_index as usize] =
                    crate::stats::quant_area_avg(&quant_areas[area_index as usize]);
            }
        }
    }

    let final_colors = variances
        .iter()
        .map(|&img_color| img_to_quant_color_map[img_color as usize])
        .collect::<Vec<_>>();

    final_colors
}
