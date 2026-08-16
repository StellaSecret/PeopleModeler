//! Central configuration table for every tunable model coefficient.
//!
//! All synergy weights, penalties, danger thresholds, similarity formulas and
//! validation thresholds live here so the model can be tuned without touching
//! engine code. This file must stay a **pure refactor**: the values below are
//! verbatim copies of the constants previously inlined across `synergy.rs` and
//! `validation.rs`.

use crate::models::{BehaviorTrigger, BiasType, MotivationType, RelationType};

/// Everything the model tunes, in one place.
pub const CFG: ModelConfig = ModelConfig {
    base_weights: BaseWeights {
        ocean: 0.17,
        reputation: 0.26,
        motivation: 0.19,
        patterns: 0.14,
        bias: 0.13,
        style: 0.11,
        history: 0.10,
    },
    bands: BandConfig {
        narrow_max: 4,
        wide_max: 7,
        wide: 12,
        mid: 8,
        narrow: 4,
    },
    trajectory: TrajectoryConfig {
        valence_scale: 3.0,
        half_life_ms: 30.0 * 24.0 * 3600.0 * 1000.0,
        momentum_threshold: 0.25,
        level_threshold: 0.5,
        delta_scale: 10.0,
        delta_clamp: 10,
        min_samples: 4,
    },
    similarity: SimilarityConfig {
        trait_scale: 10.0,
        neutral: 0.5,
        asym_dimensions: 5.0,
        diff_bounds: [3.0, 5.0, 7.0, 8.5],
    },
    ocean: OceanConfig {
        complement_min: 7,
        complement_bonus: 0.15,
        neutral_default: 0.5,
        danger: OceanDangerConfig {
            high: 7,
            low: 4,
            within_volatile: 0.10,
            within_impulsive: 0.05,
            within_rigid: 0.05,
            contagion: 0.10,
            antagonism: 0.15,
            unreliability: 0.10,
            rigidity: 0.05,
        },
    },
    reputation: ReputationConfig {
        // Indexed by RepDim::ALL order.
        dim_weights: [
            0.07, // HardworkerLazy
            0.12, // AuthoritativeSubmissive
            0.15, // HonestDeceitful
            0.12, // ReliableFlaky
            0.12, // HumbleArrogant
            0.07, // CalmReactive
            0.07, // DiplomaticBlunt
            0.04, // GenerousSelfish
            0.07, // FairFavoritism
            0.05, // TrustingSuspicious
            0.05, // AssertivePassive
            0.05, // EmpatheticDetached
            0.04, // AdaptableRigid
        ],
        adjust: RepAdjustConfig {
            extreme_low: 2,
            extreme_high: 9,
            mid_low: 4,
            mid_high: 6,
            context_extreme: 0.04,
            context_mid: 0.02,
            non_context_low: 0.05,
            non_context_high: 0.03,
            missing: 0.02,
        },
        danger: RepDangerConfig {
            high: 8,
            low: 3,
            power_struggle: 0.10,
            brutal: 0.10,
            escalation: 0.10,
            no_concede: 0.10,
            passivity: 0.05,
            suspicion: 0.08,
            coldness: 0.08,
            trust_collapse: 0.10,
            unreliability: 0.08,
            cronyism: 0.08,
            hoarding: 0.05,
            paralysis: 0.05,
            gridlock: 0.05,
        },
    },
    motivation: MotivationConfig {
        // Indexed by MotivationType::ALL order:
        // [Power, Achievement, Affiliation, Security, Autonomy, Recognition,
        //  Learning, Helping, Creativity, Fairness]
        synergy: [
            [-0.2, 0.3, -0.2, -0.1, 0.2, 0.2, 0.0, 0.1, -0.1, -0.2], // Power
            [0.3, 0.2, 0.1, -0.2, 0.2, 0.3, 0.3, 0.2, 0.2, 0.2],     // Achievement
            [-0.2, 0.1, 0.2, 0.2, -0.1, -0.1, 0.2, 0.3, 0.2, 0.2],   // Affiliation
            [-0.1, -0.2, 0.2, 0.0, -0.3, 0.0, 0.2, 0.2, -0.2, 0.2],  // Security
            [0.2, 0.2, -0.1, -0.3, 0.0, 0.0, 0.2, 0.0, 0.2, 0.2],    // Autonomy
            [0.2, 0.3, -0.1, 0.0, 0.0, -0.1, 0.3, 0.0, 0.3, -0.1],   // Recognition
            [0.0, 0.3, 0.2, 0.2, 0.2, 0.3, 0.2, 0.2, 0.3, 0.2],      // Learning
            [0.1, 0.2, 0.3, 0.2, 0.0, 0.0, 0.2, 0.2, -0.1, 0.3],     // Helping
            [-0.1, 0.2, 0.2, -0.2, 0.2, 0.3, 0.3, -0.1, 0.2, 0.2],   // Creativity
            [-0.2, 0.2, 0.2, 0.2, 0.2, -0.1, 0.2, 0.3, 0.2, 0.2],    // Fairness
        ],
        norm_offset: 0.3,
        norm_scale: 0.6,
        default: 0.5,
        intensity_scale: 100.0,
        virtue: VirtueConfig {
            high: 7,
            low: 3,
            recognition_high: 9,
            fairness: 0.08,
            helping: 0.06,
            learning: 0.04,
            creativity: 0.04,
            power: 0.08,
            security: 0.05,
            recognition: 0.03,
        },
        count: MotivationCountConfig {
            min: 3,
            per_missing: 0.03,
        },
    },
    patterns: PatternsConfig {
        // Indexed by BehaviorTrigger::ALL order:
        // [Stress, Conflict, Success, Uncertainty, Recognition, Threatened,
        //  Change, Feedback, Injustice]
        trigger_synergy: [
            [-0.2, -0.3, 0.0, 0.0, 0.0, 0.0, -0.2, 0.0, -0.1], // Stress
            [-0.3, -0.3, 0.0, -0.2, 0.0, 0.0, 0.0, 0.0, -0.1], // Conflict
            [0.0, 0.0, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],     // Success
            [0.0, -0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.1],   // Uncertainty
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.0],     // Recognition
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],     // Threatened
            [-0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 0.0],    // Change
            [0.0, 0.0, 0.0, 0.0, 0.2, 0.0, 0.3, 0.3, 0.0],     // Feedback
            [-0.1, -0.1, 0.0, -0.1, 0.0, 0.0, 0.0, 0.0, -0.2], // Injustice
        ],
        norm_offset: 0.3,
        norm_scale: 0.6,
        default: 0.5,
        pair_weight: 1.0,
        undefined_penalty: 0.02,
        only_negative_penalty: 0.05,
    },
    bias: BiasConfig {
        // Indexed by BiasType::ALL order.
        modulation: [
            Some((BiasTarget::Reputation, 0.10)),  // Confirmation
            Some((BiasTarget::Ocean, 0.10)),       // Anchoring
            Some((BiasTarget::Patterns, 0.10)),    // Availability
            Some((BiasTarget::Motivation, 0.10)),  // SunkCost
            Some((BiasTarget::Ocean, -0.10)),      // DunningKruger
            Some((BiasTarget::Ocean, 0.10)),       // Impostor
            Some((BiasTarget::Patterns, -0.10)),   // LossAversion
            Some((BiasTarget::Reputation, 0.08)),  // SocialProof
            Some((BiasTarget::Motivation, 0.08)),  // Authority
            Some((BiasTarget::Patterns, 0.08)),    // Recency
            Some((BiasTarget::Ocean, 0.08)),       // InGroup
            Some((BiasTarget::Reputation, -0.08)), // Favoritism
        ],
        intensity_scale: 100.0,
        default: 0.5,
        absent_bonus: 0.02,
        mild_bonus: 0.01,
        strong_penalty: 0.03,
        mild_max: 3,
        strong_min: 7,
        moderate_min: 4,
        count_bonus: [0.09, 0.06, 0.03],
    },
    styles: StylesConfig {
        same_score: 1.0,
        different_score: 0.5,
        default: 0.5,
        contradiction_cap: 0.5,
    },
    history: HistoryConfig {
        min_samples: 3,
        low_accuracy: 5.0,
        both_low_penalty: 0.05,
        single_low_penalty: 0.03,
    },
    relationship: RelationshipConfig {
        // Indexed by RelationType::ALL order:
        // [WorksWith, Manages, ReportsTo, Friends, Family, Partner, Mentors, Collaborates]
        // weights order: (ocean, reputation, motivation, patterns, bias, style)
        weights: [
            [0.20, 0.28, 0.16, 0.16, 0.12, 0.08], // WorksWith
            [0.15, 0.30, 0.15, 0.18, 0.13, 0.09], // Manages
            [0.15, 0.30, 0.15, 0.18, 0.13, 0.09], // ReportsTo
            [0.18, 0.18, 0.20, 0.12, 0.12, 0.20], // Friends
            [0.14, 0.22, 0.24, 0.12, 0.12, 0.16], // Family
            [0.16, 0.20, 0.22, 0.14, 0.10, 0.18], // Partner
            [0.20, 0.18, 0.20, 0.14, 0.12, 0.16], // Mentors
            [0.18, 0.28, 0.16, 0.16, 0.13, 0.09], // Collaborates
        ],
        power_friction_mod: -0.08,
        power_intensity_min: 7,
        hierarchy_bonus: 0.04,
        hierarchy_rep_diff_min: 3,
    },
    completeness: CompletenessConfig {
        motivation_cap: 3,
        bias_cap: 11,
        style_cap: 8,
        pattern_cap: 5,
        denominator: 45.0,
    },
    profile: ProfileConfig {
        default_total: 50,
        default_motivation: 0.5,
        default_patterns: 0.5,
        default_style: 0.5,
        contradiction_cap: 0.5,
    },
    flags: FlagConfig {
        self_report: 0.20,
        stated_perceived: 0.30,
        evidence: 0.40,
        malus_cap: 0.50,
    },
    validation: ValidationConfig {
        high: 8,
        low: 3,
        motivation_high: 6,
        bias_high: 7,
        style_high: 6,
    },
};

pub struct ModelConfig {
    pub base_weights: BaseWeights,
    pub bands: BandConfig,
    pub trajectory: TrajectoryConfig,
    pub similarity: SimilarityConfig,
    pub ocean: OceanConfig,
    pub reputation: ReputationConfig,
    pub motivation: MotivationConfig,
    pub patterns: PatternsConfig,
    pub bias: BiasConfig,
    pub styles: StylesConfig,
    pub history: HistoryConfig,
    pub relationship: RelationshipConfig,
    pub completeness: CompletenessConfig,
    pub profile: ProfileConfig,
    pub flags: FlagConfig,
    pub validation: ValidationConfig,
}

impl ModelConfig {
    pub fn relation_weights(&self, rtype: RelationType) -> [f64; 6] {
        let i = RelationType::ALL
            .iter()
            .position(|&t| t == rtype)
            .expect("unknown relation type");
        self.relationship.weights[i]
    }

    pub fn motivation_synergy(&self, a: MotivationType, b: MotivationType) -> f64 {
        let i = MotivationType::ALL
            .iter()
            .position(|&t| t == a)
            .expect("unknown motivation type");
        let j = MotivationType::ALL
            .iter()
            .position(|&t| t == b)
            .expect("unknown motivation type");
        self.motivation.synergy[i][j]
    }

    pub fn trigger_synergy(&self, a: BehaviorTrigger, b: BehaviorTrigger) -> f64 {
        let i = BehaviorTrigger::ALL
            .iter()
            .position(|&t| t == a)
            .expect("unknown trigger");
        let j = BehaviorTrigger::ALL
            .iter()
            .position(|&t| t == b)
            .expect("unknown trigger");
        self.patterns.trigger_synergy[i][j]
    }

    pub fn bias_modulation(&self, ty: BiasType) -> Option<(BiasTarget, f64)> {
        let i = BiasType::ALL
            .iter()
            .position(|&t| t == ty)
            .expect("unknown bias");
        self.bias.modulation[i]
    }
}

pub struct BaseWeights {
    pub ocean: f64,
    pub reputation: f64,
    pub motivation: f64,
    pub patterns: f64,
    pub bias: f64,
    pub style: f64,
    /// Weight of the history (prediction-accuracy) danger factor.
    pub history: f64,
}

pub struct BandConfig {
    /// Highest strength/confidence value in the "narrow" band (1..=narrow_max).
    pub narrow_max: u8,
    /// Highest strength/confidence value in the "mid" band (narrow_max+1..=wide_max).
    pub wide_max: u8,
    /// ± band width for the low (1-4) band.
    pub wide: u8,
    /// ± band width for the mid (5-7) band.
    pub mid: u8,
    /// ± band width for the high (8-10) band.
    pub narrow: u8,
}

pub struct TrajectoryConfig {
    pub valence_scale: f64,
    pub half_life_ms: f64,
    pub momentum_threshold: f64,
    pub level_threshold: f64,
    pub delta_scale: f64,
    pub delta_clamp: i8,
    pub min_samples: usize,
}

pub struct SimilarityConfig {
    pub trait_scale: f64,
    pub neutral: f64,
    pub asym_dimensions: f64,
    pub diff_bounds: [f64; 4],
}

pub struct OceanConfig {
    /// Trait level that qualifies for the O/C and E/A complement bonus.
    pub complement_min: u8,
    pub complement_bonus: f64,
    pub neutral_default: f64,
    pub danger: OceanDangerConfig,
}

pub struct OceanDangerConfig {
    pub high: u8,
    pub low: u8,
    pub within_volatile: f64,
    pub within_impulsive: f64,
    pub within_rigid: f64,
    pub contagion: f64,
    pub antagonism: f64,
    pub unreliability: f64,
    pub rigidity: f64,
}

pub struct ReputationConfig {
    pub dim_weights: [f64; 13],
    pub adjust: RepAdjustConfig,
    pub danger: RepDangerConfig,
}

pub struct RepAdjustConfig {
    pub extreme_low: u8,
    pub extreme_high: u8,
    pub mid_low: u8,
    pub mid_high: u8,
    pub context_extreme: f64,
    pub context_mid: f64,
    pub non_context_low: f64,
    pub non_context_high: f64,
    pub missing: f64,
}

pub struct RepDangerConfig {
    pub high: u8,
    pub low: u8,
    pub power_struggle: f64,
    pub brutal: f64,
    pub escalation: f64,
    pub no_concede: f64,
    pub passivity: f64,
    pub suspicion: f64,
    pub coldness: f64,
    pub trust_collapse: f64,
    pub unreliability: f64,
    pub cronyism: f64,
    pub hoarding: f64,
    pub paralysis: f64,
    pub gridlock: f64,
}

pub struct MotivationConfig {
    pub synergy: [[f64; 10]; 10],
    pub norm_offset: f64,
    pub norm_scale: f64,
    pub default: f64,
    pub intensity_scale: f64,
    pub virtue: VirtueConfig,
    pub count: MotivationCountConfig,
}

pub struct VirtueConfig {
    pub high: u8,
    pub low: u8,
    pub recognition_high: u8,
    pub fairness: f64,
    pub helping: f64,
    pub learning: f64,
    pub creativity: f64,
    pub power: f64,
    pub security: f64,
    pub recognition: f64,
}

pub struct MotivationCountConfig {
    pub min: usize,
    pub per_missing: f64,
}

pub struct PatternsConfig {
    pub trigger_synergy: [[f64; 9]; 9],
    pub norm_offset: f64,
    pub norm_scale: f64,
    pub default: f64,
    pub pair_weight: f64,
    pub undefined_penalty: f64,
    pub only_negative_penalty: f64,
}

/// Which component a shared bias type modulates.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BiasTarget {
    Ocean,
    Reputation,
    Motivation,
    Patterns,
}

pub struct BiasConfig {
    pub modulation: [Option<(BiasTarget, f64)>; 12],
    pub intensity_scale: f64,
    pub default: f64,
    pub absent_bonus: f64,
    pub mild_bonus: f64,
    pub strong_penalty: f64,
    pub mild_max: u8,
    pub strong_min: u8,
    pub moderate_min: u8,
    pub count_bonus: [f64; 3],
}

pub struct StylesConfig {
    pub same_score: f64,
    pub different_score: f64,
    pub default: f64,
    pub contradiction_cap: f64,
}

pub struct HistoryConfig {
    pub min_samples: usize,
    pub low_accuracy: f64,
    pub both_low_penalty: f64,
    pub single_low_penalty: f64,
}

pub struct RelationshipConfig {
    pub weights: [[f64; 6]; 8],
    pub power_friction_mod: f64,
    pub power_intensity_min: u8,
    pub hierarchy_bonus: f64,
    pub hierarchy_rep_diff_min: i16,
}

pub struct CompletenessConfig {
    pub motivation_cap: usize,
    pub bias_cap: usize,
    pub style_cap: usize,
    pub pattern_cap: usize,
    pub denominator: f64,
}

pub struct ProfileConfig {
    pub default_total: u8,
    pub default_motivation: f64,
    pub default_patterns: f64,
    pub default_style: f64,
    pub contradiction_cap: f64,
}

pub struct FlagConfig {
    pub self_report: f64,
    pub stated_perceived: f64,
    pub evidence: f64,
    pub malus_cap: f64,
}

pub struct ValidationConfig {
    pub high: u8,
    pub low: u8,
    pub motivation_high: u8,
    pub bias_high: u8,
    pub style_high: u8,
}
