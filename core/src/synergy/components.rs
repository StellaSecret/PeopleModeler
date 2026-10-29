use std::collections::HashSet;

use crate::model_config::CFG;
use crate::models::{
    BehaviorResponse, BehaviorTrigger, BehavioralPattern, Bias, BiasType, Motivation,
    MotivationType, PersonalStyle,
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

/// Number of distinct triggers a pattern set records. Duplicate answers for
/// the same trigger count once.
pub fn distinct_trigger_count(patterns: &[BehavioralPattern]) -> usize {
    let defined: HashSet<BehaviorTrigger> = patterns.iter().map(|p| p.trigger).collect();
    defined.len()
}

/// Coverage adjustment: evidence is only as good as what was recorded.
///
/// Each side's record covers `distinct_trigger_count` triggers; the pair's
/// evidence is their mean. Every trigger without evidence costs a little
/// credit. For a self-comparison this is exactly the personal completeness
/// rule (both sides are the same record).
pub fn pattern_coverage_adjustment(pa: &[BehavioralPattern], pb: &[BehavioralPattern]) -> f64 {
    let evidence = (distinct_trigger_count(pa) + distinct_trigger_count(pb)) as f64 / 2.0;
    let missing = (BehaviorTrigger::ALL.len() as f64 - evidence).max(0.0);
    -(missing * CFG.patterns.undefined_penalty)
}

/// How one person behaves on one trigger.
///
/// A trigger can carry several recorded answers (same trigger, different
/// notes), so the behavior is the *mean* quality of those answers; the
/// trigger counts as maladaptive only when every answer for it is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TriggerBehavior {
    /// Mean response quality over the recorded answers, in [-1, 1].
    pub quality: f64,
    /// True when every recorded answer for the trigger is maladaptive.
    pub all_maladaptive: bool,
}

impl TriggerBehavior {
    /// Behavior of a single recorded answer.
    pub fn of(response: BehaviorResponse) -> Self {
        Self {
            quality: response.quality_unit(),
            all_maladaptive: response.is_maladaptive(),
        }
    }

    /// Behavior recorded for a trigger, or `None` when nothing was recorded
    /// for it.
    pub fn recorded(patterns: &[BehavioralPattern], trigger: BehaviorTrigger) -> Option<Self> {
        let mut sum = 0.0;
        let mut n = 0.0;
        let mut all_maladaptive = true;
        for p in patterns.iter().filter(|p| p.trigger == trigger) {
            sum += p.predicted_behavior.quality_unit();
            n += 1.0;
            all_maladaptive &= p.predicted_behavior.is_maladaptive();
        }
        if n == 0.0 {
            None
        } else {
            Some(Self {
                quality: sum / n,
                all_maladaptive,
            })
        }
    }
}

/// Value of a trigger **both** people have, in synergy units.
///
/// The recorded behavior drives the value: two proactive answers credit the
/// pair, two maladaptive answers are friction, and a mixed pair is neutral.
/// Two maladaptive answers on the same trigger get an extra penalty — mutual
/// escalation is worse than two unrelated bad habits.
pub fn shared_trigger_value(a: TriggerBehavior, b: TriggerBehavior) -> f64 {
    let avg = (a.quality + b.quality) / 2.0;
    let mut value = CFG.patterns.response_gain * avg;
    if a.all_maladaptive && b.all_maladaptive {
        value -= CFG.patterns.maladaptive_pair_penalty;
    }
    value
}

/// Value of a trigger only **one** of the two people has: the individual's own
/// behavior counts, at a discount — an unshared situation is not shared
/// friction, but a maladaptive answer still drags the pair down.
pub fn solo_trigger_value(behavior: TriggerBehavior) -> f64 {
    CFG.patterns.response_gain * behavior.quality * CFG.patterns.solo_weight
}

/// Behavioral synergy of two people.
///
/// Scored trigger by trigger over the union of both pattern sets: a trigger
/// both have is a *shared* situation, a trigger only one has is *solo*. Each
/// trigger counts once, so recording more behavior never dilutes the score.
pub fn pattern_synergy(pa: &[BehavioralPattern], pb: &[BehavioralPattern]) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0f64;
    for trigger in BehaviorTrigger::ALL {
        let a = TriggerBehavior::recorded(pa, trigger);
        let b = TriggerBehavior::recorded(pb, trigger);
        match (a, b) {
            (Some(x), Some(y)) => {
                sum += shared_trigger_value(x, y);
                count += 1.0;
            }
            (Some(x), None) => {
                sum += solo_trigger_value(x);
                count += 1.0;
            }
            (None, Some(y)) => {
                sum += solo_trigger_value(y);
                count += 1.0;
            }
            (None, None) => {}
        }
    }
    if count == 0.0 {
        CFG.patterns.default
    } else {
        ((sum / count + CFG.patterns.norm_offset) / CFG.patterns.norm_scale).clamp(0.0, 1.0)
    }
}

/// Final pattern score for a pair: the response-driven union score adjusted
/// for how much of the trigger set was actually recorded.
///
/// Without this, one shared proactive answer would reach 1.00 even though
/// eight situations were never observed. For a person compared to themselves
/// the result equals the personal profile pattern score.
pub fn pattern_score(pa: &[BehavioralPattern], pb: &[BehavioralPattern]) -> f64 {
    (pattern_synergy(pa, pb) + pattern_coverage_adjustment(pa, pb)).clamp(0.0, 1.0)
}

/// Whether a mean response quality reads as a constructive, mixed, or
/// maladaptive mindset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MindsetTone {
    Constructive,
    Mixed,
    Maladaptive,
}

/// Mean response quality needs to leave the neutral tier's neighbourhood
/// before it is read as a clear mindset. Tiers are spaced by 1/3, so the
/// threshold sits between tier 3 (adaptive) and tier 4 (neutral).
pub const MINDSET_CONSTRUCTIVE_MIN: f64 = 1.0 / 3.0;
/// See [`MINDSET_CONSTRUCTIVE_MIN`]; between tier 4 (neutral) and tier 5
/// (mild friction).
pub const MINDSET_MALADAPTIVE_MAX: f64 = -1.0 / 3.0;

pub fn mindset_tone(quality: f64) -> MindsetTone {
    if quality >= MINDSET_CONSTRUCTIVE_MIN {
        MindsetTone::Constructive
    } else if quality <= MINDSET_MALADAPTIVE_MAX {
        MindsetTone::Maladaptive
    } else {
        MindsetTone::Mixed
    }
}

/// Response quality read from the recorded patterns, grouped by situation
/// class. This is a **readout**, not a scored input: it reuses the answers
/// already recorded and never feeds the synergy score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MindsetReadout {
    /// Mean quality over adversity triggers, in [-1, 1].
    pub adversity: Option<f64>,
    /// Mean quality over positive-situation triggers, in [-1, 1].
    pub positive: Option<f64>,
    /// Mean quality over every recorded trigger, in [-1, 1].
    pub overall: Option<f64>,
    /// Number of distinct triggers with a recorded answer.
    pub samples: usize,
}

impl MindsetReadout {
    pub fn adversity_tone(&self) -> Option<MindsetTone> {
        self.adversity.map(mindset_tone)
    }

    pub fn positive_tone(&self) -> Option<MindsetTone> {
        self.positive.map(mindset_tone)
    }

    pub fn overall_tone(&self) -> Option<MindsetTone> {
        self.overall.map(mindset_tone)
    }
}

/// Derive the mindset readout from recorded patterns.
///
/// A trigger's quality is the mean of its answers (duplicates allowed), and
/// each trigger counts once. Triggers with no answer are simply absent; they
/// are not treated as neutral.
pub fn mindset_readout(patterns: &[BehavioralPattern]) -> MindsetReadout {
    let mut adversity_sum = 0.0;
    let mut adversity_n = 0.0;
    let mut positive_sum = 0.0;
    let mut positive_n = 0.0;
    let mut overall_sum = 0.0;
    let mut overall_n = 0.0;

    for trigger in BehaviorTrigger::ALL {
        let Some(behavior) = TriggerBehavior::recorded(patterns, trigger) else {
            continue;
        };
        overall_sum += behavior.quality;
        overall_n += 1.0;
        if trigger.is_adversity() {
            adversity_sum += behavior.quality;
            adversity_n += 1.0;
        } else if trigger.is_positive_situation() {
            positive_sum += behavior.quality;
            positive_n += 1.0;
        }
    }

    let mean = |sum: f64, n: f64| if n > 0.0 { Some(sum / n) } else { None };
    MindsetReadout {
        adversity: mean(adversity_sum, adversity_n),
        positive: mean(positive_sum, positive_n),
        overall: mean(overall_sum, overall_n),
        samples: overall_n as usize,
    }
}

/// Behavioral divergence between two pattern sets (0 = same answers, 1 = one
/// proactive and the other maladaptive). Triggers only one side recorded are
/// missing evidence, not divergent behavior, so they are skipped.
pub fn pattern_divergence(pa: &[BehavioralPattern], pb: &[BehavioralPattern]) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0f64;
    for trigger in BehaviorTrigger::ALL {
        let a = TriggerBehavior::recorded(pa, trigger);
        let b = TriggerBehavior::recorded(pb, trigger);
        if let (Some(x), Some(y)) = (a, b) {
            let diff = (x.quality - y.quality).abs();
            sum += diff / 2.0;
            count += 1.0;
        }
    }
    if count == 0.0 {
        0.0
    } else {
        (sum / count).clamp(0.0, 1.0)
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
