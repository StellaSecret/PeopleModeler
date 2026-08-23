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
    let level = if w_sum > 0.0 {
        (v_sum / w_sum).clamp(-1.0, 1.0)
    } else {
        0.0
    };

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
}
