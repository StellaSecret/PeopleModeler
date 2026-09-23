use super::components::{
    motivation_synergy_score, pattern_synergy, sim, style_synergy, value_similarity,
};
use crate::model_config::CFG;
use crate::models::{FacetKind, Person, RepDim};

/// How strong an arena persona diverges from the anchor (primary) profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MaskBand {
    /// The mask is thin or absent: anchor and arena behavior are near-identical.
    Low,
    /// Noticeable but partial differences; some channels shift in the arena.
    Moderate,
    /// The person wears a strong mask: arena behavior differs sharply.
    High,
}

impl MaskBand {
    pub fn from_gap(gap: f64) -> Self {
        if gap < CFG.mask.low_max {
            Self::Low
        } else if gap < CFG.mask.moderate_max {
            Self::Moderate
        } else {
            Self::High
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaskGap {
    /// Mean divergence (0..1) over the seven behavior channels, weighted by the
    /// anchor weights. 0 = the arena facet is identical to the anchor profile.
    pub gap: f64,
    pub band: MaskBand,
}

/// Divergence between the anchor (primary) and work (mask) facets of a
/// person. Returns `None` when the person has no work persona.
pub fn mask_gap(p: &Person) -> Option<MaskGap> {
    mask_gap_for(p, FacetKind::Work)
}

/// Divergence between the anchor facet and the arena persona (`Work` or
/// `Online`) of a person. Returns `None` when the person has no mask for that
/// arena.
///
/// Per-channel gaps reuse the engine's own similarity functions, so a gap of 0
/// means "the exact behavior the engine would score"; the result composes with
/// synergy scoring (a masked pair can look very compatible in one arena and
/// incompatible in another, or the reverse).
pub fn mask_gap_for(p: &Person, kind: FacetKind) -> Option<MaskGap> {
    if kind == p.primary_facet {
        return None;
    }
    match kind {
        FacetKind::Base => p.private_persona.as_ref()?,
        FacetKind::Work => p.persona.as_ref()?,
        FacetKind::Online => p.online_persona.as_ref()?,
    };
    let base = p.facet_view(p.primary_facet);
    let arena = p.facet_view(kind);

    let mut num = 0.0;
    let mut wsum = 0.0;
    let mut channel = |gap: f64, weight: f64, active: bool| {
        if active {
            num += gap * weight;
            wsum += weight;
        }
    };

    // OCEAN: average sim over dimensions defined on both facets. Inherited
    // dims are identical → 0 gap, so only persona-defined dims contribute.
    let mut ocean_gap = 0.0;
    let mut ocean_dims = 0;
    for dim in [
        ("o", base.ocean.openness, arena.ocean.openness),
        (
            "c",
            base.ocean.conscientiousness,
            arena.ocean.conscientiousness,
        ),
        ("e", base.ocean.extraversion, arena.ocean.extraversion),
        ("a", base.ocean.agreeableness, arena.ocean.agreeableness),
        ("n", base.ocean.neuroticism, arena.ocean.neuroticism),
    ] {
        if let (Some(b), Some(w)) = (dim.1, dim.2) {
            ocean_gap += 1.0 - sim(Some(b), Some(w));
            ocean_dims += 1;
        }
    }
    if ocean_dims > 0 {
        channel(ocean_gap / ocean_dims as f64, CFG.base_weights.ocean, true);
    }

    // Reputation: weighted distance per shared dimension (mirrors scoring.rs).
    let mut rep_gap_num = 0.0;
    let mut rep_w = 0.0;
    for (d, w) in RepDim::ALL.iter().zip(&CFG.reputation.dim_weights) {
        if let (Some(bv), Some(wv)) = (base.rep_scores.score(*d), arena.rep_scores.score(*d)) {
            let dist = bv.abs_diff(wv) as f64 / CFG.similarity.trait_scale;
            rep_gap_num += dist * w;
            rep_w += w;
        }
    }
    if rep_w > 0.0 {
        channel(rep_gap_num / rep_w, CFG.base_weights.reputation, true);
    }

    channel(
        if base.motivations.is_empty() && arena.motivations.is_empty() {
            0.0
        } else {
            1.0 - motivation_synergy_score(&base.motivations, &arena.motivations)
        },
        CFG.base_weights.motivation,
        !base.motivations.is_empty() || !arena.motivations.is_empty(),
    );

    channel(
        if base.behavioral_patterns.is_empty() && arena.behavioral_patterns.is_empty() {
            0.0
        } else {
            1.0 - pattern_synergy(&base.behavioral_patterns, &arena.behavioral_patterns)
        },
        CFG.base_weights.patterns,
        !base.behavioral_patterns.is_empty() || !arena.behavioral_patterns.is_empty(),
    );

    channel(
        if base.styles.is_empty() && arena.styles.is_empty() {
            0.0
        } else {
            1.0 - style_synergy(&base.styles, &arena.styles)
        },
        CFG.base_weights.style,
        !base.styles.is_empty() || !arena.styles.is_empty(),
    );

    channel(
        if base.values.is_empty() && arena.values.is_empty() {
            0.0
        } else {
            1.0 - value_similarity(&base.values, &arena.values)
        },
        CFG.base_weights.values,
        !base.values.is_empty() || !arena.values.is_empty(),
    );

    // Biases: shared-type proportion mirrors scoring's bias_score.
    let base_types: Vec<_> = base.biases.iter().map(|b| b.r#type).collect();
    let arena_types: Vec<_> = arena.biases.iter().map(|b| b.r#type).collect();
    if !base_types.is_empty() || !arena_types.is_empty() {
        let shared = base_types
            .iter()
            .filter(|t| arena_types.contains(t))
            .count();
        let max = base_types.len().max(arena_types.len());
        let similarity = shared as f64 / max as f64;
        channel(1.0 - similarity, CFG.base_weights.bias, true);
    }

    if wsum == 0.0 {
        return Some(MaskGap {
            gap: 0.0,
            band: MaskBand::Low,
        });
    }
    let gap = num / wsum;
    Some(MaskGap {
        gap,
        band: MaskBand::from_gap(gap),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{OceanScores, Person, PersonaMask};

    fn moon() -> Person {
        let mut p =
            crate::synergy::testutil::make_person(Some(9), Some(9), Some(9), Some(9), Some(2));
        p.ocean = OceanScores {
            openness: Some(7),
            conscientiousness: Some(4),
            extraversion: Some(8),
            agreeableness: Some(6),
            neuroticism: Some(3),
        };
        p
    }

    #[test]
    fn no_persona_yields_none() {
        assert!(mask_gap(&moon()).is_none());
    }

    #[test]
    fn empty_persona_is_zero_gap() {
        let mut p = moon();
        p.persona = Some(PersonaMask::default());
        let g = mask_gap(&p).unwrap();
        assert_eq!(g.gap, 0.0);
        assert_eq!(g.band, MaskBand::Low);
    }

    #[test]
    fn persona_overrides_drive_gap_and_band() {
        let mut p = moon();
        p.persona = Some(PersonaMask {
            // Flip every OCEAN dim far from base → large ocean divergence.
            ocean: Some(OceanScores {
                openness: Some(1),
                conscientiousness: Some(10),
                extraversion: Some(1),
                agreeableness: Some(10),
                neuroticism: Some(9),
            }),
            ..PersonaMask::default()
        });
        let g = mask_gap(&p).unwrap();
        assert!(g.gap > CFG.mask.moderate_max, "gap = {}", g.gap);
        assert_eq!(g.band, MaskBand::High);
        // N flipped 3→9 while others distance 6/10: only ocean active here.
        assert!(g.gap < 0.7, "gap = {}", g.gap);
    }

    #[test]
    fn online_persona_gap_is_computed_independently_of_work() {
        let mut p = moon();
        p.online_persona = Some(PersonaMask {
            ocean: Some(OceanScores {
                openness: Some(10),
                ..OceanScores::default()
            }),
            ..PersonaMask::default()
        });
        assert!(
            mask_gap(&p).is_none(),
            "a lone online mask must not leak into the work gap"
        );
        assert!(mask_gap_for(&p, FacetKind::Base).is_none());
        let g = mask_gap_for(&p, FacetKind::Online).unwrap();
        assert!(g.gap > 0.0, "gap = {}", g.gap);
    }

    #[test]
    fn work_primary_reports_personal_life_gap() {
        let mut p = moon();
        p.primary_facet = FacetKind::Work;
        p.private_persona = Some(PersonaMask {
            ocean: Some(OceanScores {
                openness: Some(1),
                ..OceanScores::default()
            }),
            ..PersonaMask::default()
        });
        assert!(
            mask_gap(&p).is_none(),
            "a work-primary person has no work mask: work gap is None"
        );
        let g = mask_gap_for(&p, FacetKind::Base).unwrap();
        assert!(
            g.gap > 0.0,
            "gap between the work anchor and the personal-life mask: {}",
            g.gap
        );
    }
}
