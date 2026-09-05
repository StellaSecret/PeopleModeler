use std::collections::HashSet;

use crate::model_config::CFG;
use crate::models::{
    BehaviorTrigger, BehavioralPattern, Bias, BiasType, Motivation, MotivationType, PersonalStyle,
};

/// Confidence band width (± points) from relationship strength (1-10).
pub fn strength_band(strength: u8) -> u8 {
    match strength {
        1..=4 => CFG.bands.wide,
        5..=7 => CFG.bands.mid,
        _ => CFG.bands.narrow,
    }
}

/// Confidence band width (± points) from profile confidence (1-10).
pub fn confidence_band(conf: u8) -> u8 {
    match conf {
        1..=4 => CFG.bands.wide,
        5..=7 => CFG.bands.mid,
        _ => CFG.bands.narrow,
    }
}

/// Formula: `1 - |a-b|/trait_scale`, clamped to [0.0, 1.0].
/// Returns the neutral value when either value is missing.
pub fn sim(a: Option<u8>, b: Option<u8>) -> f64 {
    match (a, b) {
        (Some(a), Some(b)) => 1.0 - (a.abs_diff(b) as f64) / CFG.similarity.trait_scale,
        _ => CFG.similarity.neutral,
    }
}

/// Scale band thresholds derived from the [`sim`] formula.
///
/// `score = sim * 100 = (1 - |a-b|/trait_scale) * 100`.
/// Trait-diff boundaries map to score thresholds via the same formula.
pub fn synergy_bands() -> [(u8, u8); 5] {
    // Trait-diff boundaries mapped through sim formula → score thresholds
    let mut thresh = [0u8; 4];
    for (i, &d) in CFG.similarity.diff_bounds.iter().enumerate() {
        thresh[i] = ((1.0 - d / CFG.similarity.trait_scale) * 100.0).round() as u8;
    }
    [
        (0, thresh[3] - 1),         // Tension
        (thresh[3], thresh[2] - 1), // Friction
        (thresh[2], thresh[1] - 1), // Moderate
        (thresh[1], thresh[0] - 1), // Good
        (thresh[0], 100),           // Strong
    ]
}

pub fn motivation_synergy(a: MotivationType, b: MotivationType) -> f64 {
    CFG.motivation_synergy(a, b)
}

pub fn motivation_synergy_score(ma: &[Motivation], mb: &[Motivation]) -> f64 {
    let mut sum = 0.0;
    let mut total_w = 0.0;
    for a in ma {
        for b in mb {
            let syn = motivation_synergy(a.r#type, b.r#type);
            if syn == 0.0 {
                continue;
            }
            let w = a.intensity as f64 * b.intensity as f64;
            sum += syn * w;
            total_w += w;
        }
    }
    if total_w == 0.0 {
        CFG.motivation.default
    } else {
        ((sum / total_w + CFG.motivation.norm_offset) / CFG.motivation.norm_scale).clamp(0.0, 1.0)
    }
}

pub fn virtue_adjustment(motivations: &[Motivation]) -> f64 {
    use crate::models::MotivationType::*;
    let v = CFG.motivation.virtue;
    let mut sum = 0.0;
    for &t in &MotivationType::ALL {
        let mot = motivations.iter().find(|m| m.r#type == t);
        let intensity = mot.map(|m| m.intensity);
        match (t, intensity) {
            (Fairness, Some(i)) if i >= v.high => sum += v.fairness,
            (Fairness, Some(i)) if i <= v.low => sum -= v.fairness,
            (Fairness, None) => sum -= v.fairness,
            (Helping, Some(i)) if i >= v.high => sum += v.helping,
            (Helping, Some(i)) if i <= v.low => sum -= v.helping,
            (Helping, None) => sum -= v.helping,
            (Learning, Some(i)) if i >= v.high => sum += v.learning,
            (Creativity, Some(i)) if i >= v.high => sum += v.creativity,
            (Power, Some(i)) if i >= v.high => sum -= v.power,
            (Security, Some(i)) if i >= v.high => sum -= v.security,
            (Recognition, Some(i)) if i >= v.recognition_high => sum -= v.recognition,
            _ => {}
        }
    }
    sum
}

pub(crate) fn motivation_count_penalty(n: usize) -> f64 {
    if n >= CFG.motivation.count.min {
        0.0
    } else {
        (CFG.motivation.count.min - n) as f64 * CFG.motivation.count.per_missing
    }
}

pub fn bias_adjustment(biases: &[Bias]) -> f64 {
    let b = CFG.bias;
    let mut sum = 0.0;
    for &t in &BiasType::ALL {
        match biases.iter().find(|b| b.r#type == t).map(|b| b.intensity) {
            Some(0) => sum += b.absent_bonus, // explicitly absent → bonus
            Some(i) if i <= b.mild_max => sum += b.mild_bonus, // mild → small bonus
            Some(i) if i >= b.strong_min => sum -= b.strong_penalty, // strong → penalty
            _ => {}                           // moderate or undefined → neutral
        }
    }
    sum
}

pub(crate) fn bias_count_bonus(n: usize) -> f64 {
    CFG.bias.count_bonus.get(n).copied().unwrap_or(0.0)
}

pub fn pattern_adjustment(patterns: &[BehavioralPattern]) -> f64 {
    let mut adj = 0.0;
    let mut defined: HashSet<BehaviorTrigger> = HashSet::new();
    for p in patterns {
        defined.insert(p.trigger);
        adj += p.predicted_behavior.score();
    }
    for t in BehaviorTrigger::ALL {
        if !defined.contains(&t) {
            adj -= CFG.patterns.undefined_penalty;
        }
    }
    adj
}

pub fn trigger_synergy(a: BehaviorTrigger, b: BehaviorTrigger) -> f64 {
    CFG.trigger_synergy(a, b)
}

pub fn pattern_synergy(pa: &[BehavioralPattern], pb: &[BehavioralPattern]) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0f64;
    for a in pa {
        for b in pb {
            let syn = trigger_synergy(a.trigger, b.trigger);
            if syn == 0.0 {
                continue;
            }
            sum += syn;
            count += 1.0;
        }
    }
    if count == 0.0 {
        CFG.patterns.default
    } else {
        ((sum / count + CFG.patterns.norm_offset) / CFG.patterns.norm_scale).clamp(0.0, 1.0)
    }
}

/// Style synergy: for each of the 6 style categories, if both persons have a
/// style in that category, score same if identical choice, different otherwise.
/// Average over categories where both have data. Returns default if no overlap.
pub fn style_synergy(a: &[PersonalStyle], b: &[PersonalStyle]) -> f64 {
    use crate::models::StyleCategory;
    let cats = StyleCategory::ALL;
    let mut sum = 0.0;
    let mut n = 0;
    for cat in &cats {
        let a_style = a.iter().find(|s| s.r#type.category() == *cat);
        let b_style = b.iter().find(|s| s.r#type.category() == *cat);
        if let (Some(sa), Some(sb)) = (a_style, b_style) {
            if sa.r#type == sb.r#type {
                sum += CFG.styles.same_score;
            } else {
                sum += CFG.styles.different_score;
            }
            n += 1;
        }
    }
    if n == 0 {
        CFG.styles.default
    } else {
        sum / n as f64
    }
}

/// Value-alignment similarity (Phase 6): distance-weighted overlap over the 10
/// value dimensions. Missing values default to neutral (0.5); weight per dim is
/// the max priority on either side (0 if both absent → skip).
pub fn value_similarity(a: &[crate::models::Value], b: &[crate::models::Value]) -> f64 {
    use crate::models::ValueType;
    let mut num = 0.0;
    let mut den = 0.0;
    for vt in &ValueType::ALL {
        let ai = a
            .iter()
            .find(|v| v.r#type == *vt)
            .map(|v| v.intensity as f64 / 10.0);
        let bi = b
            .iter()
            .find(|v| v.r#type == *vt)
            .map(|v| v.intensity as f64 / 10.0);
        let ap = a
            .iter()
            .find(|v| v.r#type == *vt)
            .map(|v| v.priority as f64 / 10.0)
            .unwrap_or(0.0);
        let bp = b
            .iter()
            .find(|v| v.r#type == *vt)
            .map(|v| v.priority as f64 / 10.0)
            .unwrap_or(0.0);
        let w = ap.max(bp);
        if w == 0.0 {
            continue;
        }
        let av = ai.unwrap_or(0.5);
        let bv = bi.unwrap_or(0.5);
        num += (1.0 - (av - bv).abs()) * w;
        den += w;
    }
    if den == 0.0 {
        0.5
    } else {
        (num / den).clamp(0.0, 1.0)
    }
}

/// Self-alignment score for a person's values: mean of
/// (intensity/10 + priority/10)/2 across the value set. Empty set → 0
/// (unfilled data earns no credit, mirroring the OCEAN bucket).
pub fn value_self_score(values: &[crate::models::Value]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let sum: f64 = values
        .iter()
        .map(|v| (v.intensity as f64 / 10.0 + v.priority as f64 / 10.0) / 2.0)
        .sum();
    (sum / values.len() as f64).clamp(0.0, 1.0)
}
