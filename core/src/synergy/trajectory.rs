use crate::model_config::CFG;
use crate::models::Person;

use super::{Trajectory, Trend};

pub(crate) fn trajectory_from(entries: &[&crate::models::InteractionEntry]) -> Trajectory {
    let dated: Vec<&crate::models::InteractionEntry> = entries
        .iter()
        .filter(|e| e.valence.is_some())
        .copied()
        .collect();
    let sample = dated.len();
    if sample == 0 {
        return Trajectory {
            delta: 0,
            trend: Trend::Stable,
            sample: 0,
            level: 0.0,
        };
    }

    let t_max = dated.iter().map(|e| e.timestamp).max().unwrap_or(0) as f64;
    let mut w_sum = 0.0;
    let mut v_sum = 0.0;
    for e in &dated {
        let v = e.valence.unwrap() as f64 / CFG.trajectory.valence_scale;
        let age = (t_max - e.timestamp as f64).max(0.0);
        let w = (-age / CFG.trajectory.half_life_ms).exp();
        v_sum += v * w;
        w_sum += w;
    }
    let level = (v_sum / w_sum).clamp(-1.0, 1.0);

    let trend = if sample >= CFG.trajectory.min_samples {
        let mut sorted: Vec<&crate::models::InteractionEntry> = dated.clone();
        sorted.sort_by_key(|e| e.timestamp);
        let mid = sorted.len() / 2;
        let early: f64 = sorted[..mid]
            .iter()
            .map(|e| e.valence.unwrap() as f64 / CFG.trajectory.valence_scale)
            .sum::<f64>()
            / mid as f64;
        let recent: f64 = sorted[mid..]
            .iter()
            .map(|e| e.valence.unwrap() as f64 / CFG.trajectory.valence_scale)
            .sum::<f64>()
            / (sorted.len() - mid) as f64;
        let momentum = recent - early;
        if momentum > CFG.trajectory.momentum_threshold {
            Trend::Improving
        } else if momentum < -CFG.trajectory.momentum_threshold {
            Trend::Deteriorating
        } else if level > CFG.trajectory.level_threshold {
            Trend::Improving
        } else if level < -CFG.trajectory.level_threshold {
            Trend::Deteriorating
        } else {
            Trend::Stable
        }
    } else if level > CFG.trajectory.level_threshold {
        Trend::Improving
    } else if level < -CFG.trajectory.level_threshold {
        Trend::Deteriorating
    } else {
        Trend::Stable
    };

    Trajectory {
        delta: ((level * CFG.trajectory.delta_scale).round() as i8)
            .clamp(-CFG.trajectory.delta_clamp, CFG.trajectory.delta_clamp),
        trend,
        sample,
        level,
    }
}

/// Pair trajectory: only interactions between the two persons count.
pub fn pair_trajectory(a: &Person, b: &Person) -> Trajectory {
    let mut entries: Vec<&crate::models::InteractionEntry> = Vec::new();
    entries.extend(
        a.log
            .iter()
            .filter(|e| e.target_id.as_deref() == Some(&*b.id)),
    );
    entries.extend(
        b.log
            .iter()
            .filter(|e| e.target_id.as_deref() == Some(&*a.id)),
    );
    trajectory_from(&entries)
}

/// Personal trajectory: all of the person's own logged interactions.
pub fn personal_trajectory(p: &Person) -> Trajectory {
    let entries: Vec<&crate::models::InteractionEntry> = p.log.iter().collect();
    trajectory_from(&entries)
}

#[cfg(test)]
mod tests {
    use super::super::testutil::*;
    use super::*;
    use crate::models::*;

    #[test]
    fn test_trajectory_empty() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        let mut b = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
        b.id = "b".into();
        let t = pair_trajectory(&a, &b);
        assert_eq!(t.sample, 0);
        assert_eq!(t.delta, 0);
        assert_eq!(t.trend, Trend::Stable);
    }

    #[test]
    fn test_trajectory_positive_improving() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        a.log = vec![
            log_entry(1000, 3, None),
            log_entry(2000, 2, None),
            log_entry(3000, 1, None),
        ];
        let t = personal_trajectory(&a);
        assert_eq!(t.sample, 3);
        assert_eq!(t.trend, Trend::Improving);
        assert!(t.delta > 0, "positive log must yield a positive delta");
        assert!(t.level > 0.5);
    }

    #[test]
    fn test_trajectory_negative_deteriorating() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        a.log = vec![
            log_entry(1000, -1, None),
            log_entry(2000, -2, None),
            log_entry(3000, -3, None),
        ];
        let t = personal_trajectory(&a);
        assert_eq!(t.trend, Trend::Deteriorating);
        assert!(t.delta < 0, "negative log must yield a negative delta");
    }

    #[test]
    fn test_trajectory_recency_dominates() {
        let day = 86_400_000i64;
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        a.log = vec![
            log_entry(0, -3, None),
            log_entry(30 * day, -3, None),
            log_entry(59 * day, 2, None),
            log_entry(60 * day, 3, None),
        ];
        let t = personal_trajectory(&a);
        assert_eq!(
            t.trend,
            Trend::Improving,
            "recent positives dominate stale negatives"
        );
        assert!(t.level > 0.0);
    }

    #[test]
    fn test_trajectory_momentum_flips_trend() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        a.log = vec![
            log_entry(1000, 3, None),
            log_entry(2000, 3, None),
            log_entry(3000, -3, None),
            log_entry(4000, -3, None),
        ];
        let t = personal_trajectory(&a);
        assert_eq!(
            t.trend,
            Trend::Deteriorating,
            "recent half flips an earlier-good run"
        );
    }

    #[test]
    fn test_pair_trajectory_filters_by_target() {
        let mut a = make_person(Some(7), Some(8), Some(6), Some(5), Some(4));
        a.id = "a".into();
        let mut b = make_person(Some(6), Some(7), Some(8), Some(5), Some(3));
        b.id = "b".into();
        a.log = vec![
            log_entry(1000, 2, Some("b")),
            log_entry(2000, 3, Some("b")),
            log_entry(3000, -3, Some("c")),
        ];
        b.log = vec![log_entry(1500, 1, Some("a"))];
        let t = pair_trajectory(&a, &b);
        assert_eq!(t.sample, 3, "only entries targeting the other person count");
        assert!(t.delta > 0, "pair trajectory should be positive");
    }

    #[test]
    fn test_relationship_strength_serde_default() {
        let json = r#"{"id":"r1","source_id":"a","target_id":"b","type":"Friends","notes":"","created_at":0}"#;
        let r: Relationship = serde_json::from_str(json).unwrap();
        assert_eq!(r.strength, 5, "missing strength → default 5");
        assert_eq!(r.r#type, RelationType::Friends);
        let out = serde_json::to_string(&r).unwrap();
        assert!(out.contains("\"strength\":5"));
    }

    // --- trajectory_from: exact level and delta ---

    #[test]
    fn test_trajectory_from_single_positive() {
        use crate::models::InteractionEntry;
        let entries: Vec<InteractionEntry> = (0..2)
            .map(|i| InteractionEntry {
                id: format!("e{}", i),
                valence: Some(3),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 2);
        assert!(traj.level > 0.0, "positive valence → positive level");
        assert!(traj.delta > 0, "positive valence → positive delta");
    }

    #[test]
    fn test_trajectory_from_single_negative() {
        use crate::models::InteractionEntry;
        let entries: Vec<InteractionEntry> = (0..2)
            .map(|i| InteractionEntry {
                id: format!("e{}", i),
                valence: Some(-3),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 2);
        assert!(traj.level < 0.0, "negative valence → negative level");
        assert!(traj.delta < 0, "negative valence → negative delta");
    }

    #[test]
    fn test_trajectory_from_level_clamped() {
        use crate::models::InteractionEntry;
        let entries: Vec<InteractionEntry> = (0..2)
            .map(|i| InteractionEntry {
                id: format!("e{}", i),
                valence: Some(3),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.level <= 1.0, "level clamped to 1.0");
        assert!(traj.level >= -1.0, "level clamped to -1.0");
    }

    #[test]
    fn test_trajectory_from_delta_clamped() {
        use crate::models::InteractionEntry;
        let entries: Vec<InteractionEntry> = (0..6)
            .map(|i| InteractionEntry {
                id: format!("e{}", i),
                valence: Some(3),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.delta <= CFG.trajectory.delta_clamp);
        assert!(traj.delta >= -CFG.trajectory.delta_clamp);
    }

    #[test]
    fn test_trajectory_from_improving_trend() {
        use crate::models::InteractionEntry;
        let mut entries: Vec<InteractionEntry> = Vec::new();
        for i in 0..5 {
            entries.push(InteractionEntry {
                id: format!("e{}", i),
                valence: Some(if i < 2 { -2 } else { 2 }),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            });
        }
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.trend, Trend::Improving);
    }

    #[test]
    fn test_trajectory_from_deteriorating_trend() {
        use crate::models::InteractionEntry;
        let mut entries: Vec<InteractionEntry> = Vec::new();
        for i in 0..5 {
            entries.push(InteractionEntry {
                id: format!("e{}", i),
                valence: Some(if i < 2 { 2 } else { -2 }),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            });
        }
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.trend, Trend::Deteriorating);
    }

    #[test]
    fn test_trajectory_from_few_samples_level_fallback() {
        use crate::models::InteractionEntry;
        let entries: Vec<InteractionEntry> = (0..2)
            .map(|i| InteractionEntry {
                id: format!("e{}", i),
                valence: Some(3),
                timestamp: i * 1000,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect();
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.sample < CFG.trajectory.min_samples);
        assert!(traj.trend == Trend::Improving || traj.trend == Trend::Stable);
    }

    // =================================================================
    // trajectory_from: exact level / momentum / trend boundary tests
    // (catches lines 26-66 missed mutants)
    // =================================================================

    #[test]
    fn test_trajectory_valence_scale_division() {
        use crate::models::InteractionEntry;
        let entries = [
            InteractionEntry {
                id: "e0".into(),
                valence: Some(3),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                valence: Some(3),
                timestamp: 1,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        let expected_level = 3.0 / CFG.trajectory.valence_scale;
        assert!(
            (traj.level - expected_level).abs() < 0.001,
            "level must be valence/valence_scale: expected {expected_level}, got {}",
            traj.level
        );
    }

    #[test]
    fn test_trajectory_wsum_zero_guard() {
        use crate::models::InteractionEntry;
        let entries = [InteractionEntry {
            id: "e0".into(),
            valence: None,
            timestamp: 0,
            text: String::new(),
            trigger: None,
            target_id: None,
        }];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 0, "no valence → sample=0");
        assert_eq!(traj.level, 0.0);
    }

    #[test]
    fn test_trajectory_level_clamp_exact_upper() {
        use crate::models::InteractionEntry;
        let entries = [
            InteractionEntry {
                id: "e0".into(),
                valence: Some(3),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                valence: Some(3),
                timestamp: 1,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.level <= 1.0, "level must be <= 1.0");
        assert!(traj.level >= -1.0, "level must be >= -1.0");
    }

    // --- Sub-min-samples: level fallback trend (lines 64-66) ---

    #[test]
    fn test_trajectory_few_samples_high_level_improving() {
        use crate::models::InteractionEntry;
        let entries = [
            InteractionEntry {
                id: "e0".into(),
                valence: Some(2),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                valence: Some(2),
                timestamp: 1,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.sample < CFG.trajectory.min_samples);
        assert_eq!(
            traj.trend,
            Trend::Improving,
            "level > 0.5 → Improving (sub-min-samples fallback)"
        );
    }

    #[test]
    fn test_trajectory_few_samples_low_level_deteriorating() {
        use crate::models::InteractionEntry;
        let entries = [
            InteractionEntry {
                id: "e0".into(),
                valence: Some(-2),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                valence: Some(-2),
                timestamp: 1,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.sample < CFG.trajectory.min_samples);
        assert_eq!(
            traj.trend,
            Trend::Deteriorating,
            "level < -0.5 → Deteriorating (sub-min-samples fallback)"
        );
    }

    #[test]
    fn test_trajectory_few_samples_mid_level_stable() {
        use crate::models::InteractionEntry;
        let entries = [
            InteractionEntry {
                id: "e0".into(),
                valence: Some(1),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                valence: Some(-1),
                timestamp: 1,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.sample < CFG.trajectory.min_samples);
        assert_eq!(
            traj.trend,
            Trend::Stable,
            "level near 0 → Stable (sub-min-samples fallback)"
        );
    }

    #[test]
    fn test_trajectory_few_samples_exact_boundary_level() {
        use crate::models::InteractionEntry;
        let entries = [
            InteractionEntry {
                id: "e0".into(),
                valence: Some(1),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                valence: Some(2),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(traj.sample < CFG.trajectory.min_samples);
        let expected_level = (1.0 / 3.0 + 2.0 / 3.0) / 2.0;
        assert!(
            (traj.level - expected_level).abs() < 0.001,
            "level = {expected_level}, got {}",
            traj.level
        );
        assert_eq!(
            traj.trend,
            Trend::Stable,
            "level = 0.5 exactly → NOT > 0.5 → Stable"
        );
    }

    // --- At-min-samples: momentum / trend (lines 38-63) ---

    fn make_entries(vals: &[(i64, i8)]) -> Vec<crate::models::InteractionEntry> {
        vals.iter()
            .enumerate()
            .map(|(i, &(ts, v))| crate::models::InteractionEntry {
                id: format!("e{}", i),
                valence: Some(v),
                timestamp: ts,
                text: String::new(),
                trigger: None,
                target_id: None,
            })
            .collect()
    }

    #[test]
    fn test_trajectory_momentum_positive_above_threshold() {
        let entries = make_entries(&[(0, 0), (1, 0), (2, 1), (3, 1)]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 4);
        let early_avg = (0.0 / 3.0 + 0.0 / 3.0) / 2.0;
        let recent_avg = (1.0 / 3.0 + 1.0 / 3.0) / 2.0;
        let momentum = recent_avg - early_avg;
        assert!(
            momentum > CFG.trajectory.momentum_threshold,
            "momentum {momentum} should exceed threshold"
        );
        assert_eq!(traj.trend, Trend::Improving);
    }

    #[test]
    fn test_trajectory_momentum_negative_below_threshold() {
        let entries = make_entries(&[(0, 1), (1, 1), (2, 0), (3, 0)]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 4);
        let early_avg = (1.0 / 3.0 + 1.0 / 3.0) / 2.0;
        let recent_avg = (0.0 / 3.0 + 0.0 / 3.0) / 2.0;
        let momentum = recent_avg - early_avg;
        assert!(
            momentum < -CFG.trajectory.momentum_threshold,
            "momentum {momentum} should be below negative threshold"
        );
        assert_eq!(traj.trend, Trend::Deteriorating);
    }

    #[test]
    fn test_trajectory_momentum_stable_level_improving() {
        let entries = make_entries(&[(0, 2), (1, 2), (2, 2), (3, 2)]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 4);
        let early_avg = (2.0 / 3.0 + 2.0 / 3.0) / 2.0;
        let recent_avg = (2.0 / 3.0 + 2.0 / 3.0) / 2.0;
        let momentum: f64 = recent_avg - early_avg;
        assert!(
            momentum.abs() <= CFG.trajectory.momentum_threshold,
            "momentum should be stable"
        );
        assert_eq!(
            traj.trend,
            Trend::Improving,
            "level > 0.5 with stable momentum → Improving"
        );
    }

    #[test]
    fn test_trajectory_momentum_stable_level_deteriorating() {
        let entries = make_entries(&[(0, -2), (1, -2), (2, -2), (3, -2)]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        let early_avg = (-2.0 / 3.0 + -2.0 / 3.0) / 2.0;
        let recent_avg = (-2.0 / 3.0 + -2.0 / 3.0) / 2.0;
        let momentum: f64 = recent_avg - early_avg;
        assert!(momentum.abs() <= CFG.trajectory.momentum_threshold);
        assert_eq!(
            traj.trend,
            Trend::Deteriorating,
            "level < -0.5 with stable momentum → Deteriorating"
        );
    }

    #[test]
    fn test_trajectory_all_four_trends_covered() {
        let improving = make_entries(&[(0, 0), (1, 0), (2, 2), (3, 2)]);
        let deteriorating = make_entries(&[(0, 2), (1, 2), (2, 0), (3, 0)]);
        let stable_high = make_entries(&[(0, 2), (1, 2), (2, 2), (3, 2)]);
        let stable_mid = make_entries(&[(0, 1), (1, 0), (2, 0), (3, 1)]);

        let refs_i: Vec<&crate::models::InteractionEntry> = improving.iter().collect();
        let refs_d: Vec<&crate::models::InteractionEntry> = deteriorating.iter().collect();
        let refs_h: Vec<&crate::models::InteractionEntry> = stable_high.iter().collect();
        let refs_m: Vec<&crate::models::InteractionEntry> = stable_mid.iter().collect();

        assert_eq!(trajectory_from(&refs_i).trend, Trend::Improving);
        assert_eq!(trajectory_from(&refs_d).trend, Trend::Deteriorating);
        assert!(matches!(
            trajectory_from(&refs_h).trend,
            Trend::Improving | Trend::Stable
        ));
        assert_eq!(trajectory_from(&refs_m).trend, Trend::Stable);
    }

    #[test]
    fn test_trajectory_even_split_calculation() {
        let entries = make_entries(&[(0, 1), (1, 2), (2, 3), (3, 3)]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        let early = (1.0 / 3.0 + 2.0 / 3.0) / 2.0;
        let recent = (3.0 / 3.0 + 3.0 / 3.0) / 2.0;
        let expected_momentum = recent - early;
        assert!(expected_momentum > CFG.trajectory.momentum_threshold);
        assert_eq!(traj.trend, Trend::Improving);
    }

    #[test]
    fn test_trajectory_odd_split() {
        use crate::models::InteractionEntry;
        let entries = [
            InteractionEntry {
                id: "e0".into(),
                valence: Some(0),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                valence: Some(0),
                timestamp: 1,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e2".into(),
                valence: Some(0),
                timestamp: 2,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e3".into(),
                valence: Some(0),
                timestamp: 3,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e4".into(),
                valence: Some(3),
                timestamp: 4,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 5);
        assert_eq!(
            traj.trend,
            Trend::Improving,
            "5 entries: early=[0,0], mid=[0], recent=[0,3] → positive momentum"
        );
    }

    #[test]
    fn test_trajectory_delta_exact() {
        let entries = make_entries(&[(0, 3), (1, 3), (2, 3), (3, 3)]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        let expected_level = (3.0 / 3.0 + 3.0 / 3.0 + 3.0 / 3.0 + 3.0 / 3.0) / 4.0;
        let expected_delta = (expected_level * CFG.trajectory.delta_scale).round() as i8;
        let expected_delta =
            expected_delta.clamp(-CFG.trajectory.delta_clamp, CFG.trajectory.delta_clamp);
        assert_eq!(
            traj.delta, expected_delta,
            "delta = level * delta_scale, clamped"
        );
    }

    #[test]
    fn test_trajectory_pair_filters_by_target_id() {
        let mut a = make_person(Some(7), Some(7), Some(7), Some(7), Some(7));
        a.id = "a".into();
        a.log = vec![
            log_entry(1000, 3, Some("b")),
            log_entry(2000, 3, Some("b")),
            log_entry(3000, -3, Some("c")),
        ];
        let mut b = make_person(Some(7), Some(7), Some(7), Some(7), Some(7));
        b.id = "b".into();
        let traj = pair_trajectory(&a, &b);
        assert_eq!(traj.sample, 2, "only entries targeting 'b' count");
    }

    #[test]
    fn test_trajectory_personal_uses_all_entries() {
        let mut a = make_person(Some(7), Some(7), Some(7), Some(7), Some(7));
        a.id = "a".into();
        a.log = vec![
            log_entry(1000, 2, Some("b")),
            log_entry(2000, 2, Some("c")),
            log_entry(3000, 2, Some("d")),
            log_entry(4000, 2, Some("e")),
        ];
        let traj = personal_trajectory(&a);
        assert_eq!(traj.sample, 4, "personal uses all log entries");
    }

    #[test]
    fn test_trajectory_empty_log() {
        let mut a = make_person(Some(5), Some(5), Some(5), Some(5), Some(5));
        a.id = "a".into();
        let traj = personal_trajectory(&a);
        assert_eq!(traj.sample, 0);
        assert_eq!(traj.delta, 0);
        assert_eq!(traj.trend, Trend::Stable);
        assert_eq!(traj.level, 0.0);
    }

    #[test]
    fn test_trajectory_level_boundary_above() {
        use crate::models::InteractionEntry;
        let entries = [InteractionEntry {
            id: "e0".into(),
            valence: Some(2),
            timestamp: 0,
            text: String::new(),
            trigger: None,
            target_id: None,
        }];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        let expected = 2.0 / CFG.trajectory.valence_scale;
        assert!((traj.level - expected).abs() < 0.001);
        assert!(traj.level > CFG.trajectory.level_threshold);
    }

    #[test]
    fn test_trajectory_level_boundary_below() {
        use crate::models::InteractionEntry;
        let entries = [InteractionEntry {
            id: "e0".into(),
            valence: Some(-2),
            timestamp: 0,
            text: String::new(),
            trigger: None,
            target_id: None,
        }];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        let expected = -2.0 / CFG.trajectory.valence_scale;
        assert!((traj.level - expected).abs() < 0.001);
        assert!(traj.level < -CFG.trajectory.level_threshold);
    }

    // --- age subtraction vs division (line 27) ---

    #[test]
    fn test_trajectory_age_is_subtraction_not_division() {
        use crate::models::InteractionEntry;
        let entries = [
            InteractionEntry {
                id: "e0".into(),
                valence: Some(3),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                valence: Some(-3),
                timestamp: 1,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(
            traj.level.abs() < 0.1,
            "equal opposite valences with similar ages → level near 0, got {}",
            traj.level
        );
    }

    // --- odd split divisor (line 51) ---

    #[test]
    fn test_trajectory_odd_split_divisor_matters() {
        let entries = make_entries(&[(0, 0), (1, 0), (2, 0), (3, 1), (4, 1)]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 5);
        assert_eq!(
            traj.trend,
            Trend::Stable,
            "5 entries [0,0,0,1,1]: recent=[0,1,1], divisor 3 gives momentum < threshold"
        );
    }

    // --- momentum boundary (lines 53, 55) ---

    #[test]
    fn test_trajectory_momentum_exactly_at_threshold() {
        let entries = make_entries(&[
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 1),
            (5, 1),
            (6, 1),
            (7, 0),
        ]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 8);
        let early_avg = (0.0 / 3.0 + 0.0 / 3.0 + 0.0 / 3.0 + 0.0 / 3.0) / 4.0;
        let recent_avg = (1.0 / 3.0 + 1.0 / 3.0 + 1.0 / 3.0 + 0.0 / 3.0) / 4.0;
        let momentum = recent_avg - early_avg;
        assert!(
            (momentum - CFG.trajectory.momentum_threshold).abs() < 1e-9,
            "momentum must be exactly 0.25, got {momentum}"
        );
        assert_eq!(
            traj.trend,
            Trend::Stable,
            "momentum == threshold (not >) → Stable"
        );
    }

    #[test]
    fn test_trajectory_momentum_exactly_at_neg_threshold() {
        let entries = make_entries(&[
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
        ]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert_eq!(traj.sample, 8);
        let early_avg = (1.0 / 3.0 + 1.0 / 3.0 + 1.0 / 3.0 + 0.0 / 3.0) / 4.0;
        let recent_avg = (0.0 / 3.0 + 0.0 / 3.0 + 0.0 / 3.0 + 0.0 / 3.0) / 4.0;
        let momentum = recent_avg - early_avg;
        assert!(
            (momentum + CFG.trajectory.momentum_threshold).abs() < 1e-9,
            "momentum must be exactly -0.25, got {momentum}"
        );
        assert_eq!(
            traj.trend,
            Trend::Stable,
            "momentum == -threshold (not <) → Stable"
        );
    }

    // --- level boundary (lines 57, 59) ---

    #[test]
    fn test_trajectory_level_exactly_at_threshold() {
        let entries = make_entries(&[(0, 1), (0, 2), (0, 1), (0, 2)]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        let expected_level = (1.0 / 3.0 + 2.0 / 3.0 + 2.0 / 3.0 + 1.0 / 3.0) / 4.0;
        assert!(
            (traj.level - expected_level).abs() < 1e-9,
            "level must be exactly 0.5, got {}",
            traj.level
        );
        assert!(
            (traj.level - CFG.trajectory.level_threshold).abs() < 1e-9,
            "level == threshold"
        );
        assert_eq!(
            traj.trend,
            Trend::Stable,
            "level == threshold (not >) → Stable"
        );
    }

    #[test]
    fn test_trajectory_level_exactly_at_neg_threshold() {
        let entries = make_entries(&[(0, -1), (0, -2), (0, -1), (0, -2)]);
        let refs: Vec<&crate::models::InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        let expected_level = (-1.0 / 3.0 + -2.0 / 3.0 + -2.0 / 3.0 + -1.0 / 3.0) / 4.0;
        assert!(
            (traj.level - expected_level).abs() < 1e-9,
            "level must be exactly -0.5, got {}",
            traj.level
        );
        assert!(
            (traj.level + CFG.trajectory.level_threshold).abs() < 1e-9,
            "level == -threshold"
        );
        assert_eq!(
            traj.trend,
            Trend::Stable,
            "level == -threshold (not <) → Stable"
        );
    }

    // --- sub-min-samples level boundary (line 66) ---

    #[test]
    fn test_trajectory_submin_level_exactly_at_neg_threshold() {
        use crate::models::InteractionEntry;
        let entries = [
            InteractionEntry {
                id: "e0".into(),
                valence: Some(-1),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
            InteractionEntry {
                id: "e1".into(),
                valence: Some(-2),
                timestamp: 0,
                text: String::new(),
                trigger: None,
                target_id: None,
            },
        ];
        let refs: Vec<&InteractionEntry> = entries.iter().collect();
        let traj = trajectory_from(&refs);
        assert!(
            traj.sample < CFG.trajectory.min_samples,
            "sample must be < min_samples"
        );
        let expected_level = (-1.0 / 3.0 + -2.0 / 3.0) / 2.0;
        assert!(
            (traj.level - expected_level).abs() < 1e-9,
            "level must be exactly -0.5, got {}",
            traj.level
        );
        assert_eq!(
            traj.trend,
            Trend::Stable,
            "level == -threshold (not <) → Stable (sub-min path)"
        );
    }
}
