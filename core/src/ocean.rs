use crate::models::OceanScores;

pub struct TraitInterpretation {
    pub high: &'static str,
    pub low: &'static str,
}

pub fn interpret_openness(score: u8) -> &'static str {
    if score >= 7 {
        "très ouvert aux nouvelles idées, créatif et curieux"
    } else if score >= 4 {
        "équilibré entre tradition et innovation"
    } else {
        "pragmatique, préfère les routines et le concret"
    }
}

pub fn interpret_conscientiousness(score: u8) -> &'static str {
    if score >= 7 {
        "organisé, fiable, orienté résultats et détails"
    } else if score >= 4 {
        "niveau modéré de structure et de flexibilité"
    } else {
        "flexible et spontané, peut manquer de rigueur"
    }
}

pub fn interpret_extraversion(score: u8) -> &'static str {
    if score >= 7 {
        "extraverti, énergique, cherche la stimulation sociale"
    } else if score >= 4 {
        "équilibré entre solitude et vie sociale"
    } else {
        "introverti, réfléchi, préfère les interactions limitées"
    }
}

pub fn interpret_agreeableness(score: u8) -> &'static str {
    if score >= 7 {
        "coopératif, empathique, cherche l'harmonie"
    } else if score >= 4 {
        "équilibré entre affirmation de soi et diplomatie"
    } else {
        "direct voire abrasif, met ses objectifs avant les relations"
    }
}

pub fn interpret_neuroticism(score: u8) -> &'static str {
    if score >= 7 {
        "émotionnellement réactif, stressable, sensible aux critiques"
    } else if score >= 4 {
        "réactivité émotionnelle modérée"
    } else {
        "stable émotionnellement, calme sous pression"
    }
}

fn fmt_trait(val: Option<u8>, f: impl FnOnce(u8) -> &'static str) -> String {
    val.map(f).unwrap_or("—").to_string()
}

pub fn interpret_all(ocean: &OceanScores) -> String {
    format!(
        "O: {}\nC: {}\nE: {}\nA: {}\nN: {}",
        fmt_trait(ocean.openness, interpret_openness),
        fmt_trait(ocean.conscientiousness, interpret_conscientiousness),
        fmt_trait(ocean.extraversion, interpret_extraversion),
        fmt_trait(ocean.agreeableness, interpret_agreeableness),
        fmt_trait(ocean.neuroticism, interpret_neuroticism),
    )
}

pub fn active_band_index(score: u8, band_ranges: &[(u8, u8)], default: usize) -> usize {
    band_ranges
        .iter()
        .position(|&(lo, hi)| score >= lo && score <= hi)
        .unwrap_or(default)
}

pub fn avg_ocean_score(scores: &[Option<u8>]) -> u8 {
    let total: u8 = scores.iter().filter_map(|s| *s).sum();
    let count = scores.iter().filter(|s| s.is_some()).count().max(1);
    total / count as u8
}

pub fn scores_to_percentages(scores: &[f64]) -> Vec<u8> {
    scores.iter().map(|v| (*v * 100.0) as u8).collect()
}

pub fn radar_data_point(i: usize, score: Option<u8>, cx: f64, cy: f64, r: f64) -> (f64, f64) {
    use std::f64::consts::PI;
    let a = (-90.0 + i as f64 * 72.0) * PI / 180.0;
    let pr = r * score.unwrap_or(0) as f64 / 10.0;
    (cx + pr * a.cos(), cy + pr * a.sin())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_band_index_in_range() {
        let ranges = [(0, 3), (4, 6), (7, 10)];
        assert_eq!(active_band_index(2, &ranges, 2), 0);
        assert_eq!(active_band_index(5, &ranges, 2), 1);
        assert_eq!(active_band_index(8, &ranges, 2), 2);
    }

    #[test]
    fn active_band_index_boundary() {
        let ranges = [(0, 3), (4, 6), (7, 10)];
        assert_eq!(active_band_index(0, &ranges, 2), 0);
        assert_eq!(active_band_index(3, &ranges, 2), 0);
        assert_eq!(active_band_index(4, &ranges, 2), 1);
        assert_eq!(active_band_index(6, &ranges, 2), 1);
        assert_eq!(active_band_index(7, &ranges, 2), 2);
        assert_eq!(active_band_index(10, &ranges, 2), 2);
    }

    #[test]
    fn active_band_index_out_of_range() {
        let ranges = [(0, 3), (4, 6), (7, 10)];
        assert_eq!(active_band_index(99, &ranges, 2), 2);
    }

    #[test]
    fn avg_ocean_score_all_some() {
        assert_eq!(avg_ocean_score(&[Some(4), Some(6), Some(8)]), 6);
    }

    #[test]
    fn avg_ocean_score_with_none() {
        assert_eq!(avg_ocean_score(&[Some(6), None, Some(8)]), 7);
    }

    #[test]
    fn avg_ocean_score_all_none() {
        assert_eq!(avg_ocean_score(&[None, None]), 0);
    }

    #[test]
    fn avg_ocean_score_single() {
        assert_eq!(avg_ocean_score(&[Some(7)]), 7);
    }

    #[test]
    fn scores_to_percentages_basic() {
        assert_eq!(scores_to_percentages(&[0.5, 1.0, 0.0]), vec![50, 100, 0]);
    }

    #[test]
    fn scores_to_percentages_empty() {
        let result: Vec<u8> = scores_to_percentages(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn radar_data_point_first_axis() {
        let (x, y) = radar_data_point(0, Some(10), 200.0, 200.0, 150.0);
        assert!((x - 200.0).abs() < 0.1);
        assert!((y - 50.0).abs() < 0.1);
    }

    #[test]
    fn radar_data_point_zero_score() {
        let (x, y) = radar_data_point(0, Some(0), 200.0, 200.0, 150.0);
        assert!((x - 200.0).abs() < 0.1);
        assert!((y - 200.0).abs() < 0.1);
    }

    #[test]
    fn radar_data_point_none_score() {
        let (x, y) = radar_data_point(0, None, 200.0, 200.0, 150.0);
        assert!((x - 200.0).abs() < 0.1);
        assert!((y - 200.0).abs() < 0.1);
    }

    #[test]
    fn radar_data_point_all_five_axes() {
        for i in 0..5 {
            let (x, y) = radar_data_point(i, Some(5), 200.0, 200.0, 150.0);
            let dist = ((x - 200.0).powi(2) + (y - 200.0).powi(2)).sqrt();
            assert!((dist - 75.0).abs() < 1.0, "axis {i}: dist={dist}");
        }
    }

    #[test]
    fn radar_data_point_second_axis_exact() {
        // Pin the exact coordinate for a non-zero axis: the angle math at
        // ocean.rs:92 (`-90.0 + i as f64 * 72.0`) and the x-axis sign at
        // ocean.rs:94 (`cx + pr * a.cos()`) are only exercised when i != 0
        // and the literal expected value is asserted.
        let (x, y) = radar_data_point(1, Some(10), 200.0, 200.0, 150.0);
        let a = (-90.0 + 72.0) * std::f64::consts::PI / 180.0;
        let (ex, ey) = (200.0 + 150.0 * a.cos(), 200.0 + 150.0 * a.sin());
        assert!((x - ex).abs() < 1e-9, "x={x}, ex={ex}");
        assert!((y - ey).abs() < 1e-9, "y={y}, ey={ey}");
    }
}
