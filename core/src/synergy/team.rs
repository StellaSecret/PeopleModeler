use super::scoring::compute_synergy_score_ctx;
use super::{RelContext, SynergyBreakdown};
use crate::insights::InsightContext;
use crate::models::{Person, Prediction, Relationship};

/// A single pair result inside the team matrix.
#[derive(serde::Serialize, Clone)]
pub struct PairResult {
    pub id_a: String,
    pub id_b: String,
    pub person_a: String,
    pub person_b: String,
    pub breakdown: SynergyBreakdown,
}

/// Team-level aggregation over all pairwise synergies.
#[derive(serde::Serialize)]
pub struct TeamSynergy {
    /// All pairwise results (unordered, i < j).
    pub pairs: Vec<PairResult>,
    /// Number of persons in the team.
    pub team_size: usize,
    /// Average total score across all pairs.
    pub avg_score: u8,
    /// Weakest link: (person_a, person_b, score).
    pub weakest: Option<(String, String, u8)>,
    /// Strongest link: (person_a, person_b, score).
    pub strongest: Option<(String, String, u8)>,
    /// Maximum pairwise danger across all pairs.
    pub max_danger: f64,
    /// Average danger across all pairs.
    pub avg_danger: f64,
    /// Per-context team averages: (context, avg_score).
    pub context_averages: Vec<(InsightContext, u8)>,
}

/// Compute team-level synergy over all persons, using relationship data to
/// build per-pair `RelContext`.
///
/// `rels` is the full relationship list (typically `db::all_relationships()`).
/// `preds` maps person_id → predictions for that person.
/// Pairs without a matching Relationship are scored context-free.
pub fn compute_team_synergy(
    persons: &[Person],
    rels: &[Relationship],
    preds: &std::collections::HashMap<String, Vec<Prediction>>,
) -> Option<TeamSynergy> {
    if persons.len() < 2 {
        return None;
    }

    // Build a quick lookup: (min_id, max_id) → Relationship
    let mut rel_map: std::collections::HashMap<(&str, &str), &Relationship> =
        std::collections::HashMap::new();
    for r in rels {
        let (a, b) = if r.source_id <= r.target_id {
            (&r.source_id, &r.target_id)
        } else {
            (&r.target_id, &r.source_id)
        };
        rel_map.insert((a, b), r);
    }

    let empty_preds: Vec<Prediction> = Vec::new();

    let mut pairs: Vec<PairResult> = Vec::new();

    for i in 0..persons.len() {
        for j in (i + 1)..persons.len() {
            let a = &persons[i];
            let b = &persons[j];

            let (id_a, id_b) = if a.id <= b.id {
                (&a.id, &b.id)
            } else {
                (&b.id, &a.id)
            };

            let ctx = rel_map
                .get(&(id_a.as_str(), id_b.as_str()))
                .map(|r| RelContext {
                    rtype: r.r#type,
                    strength: r.strength.clamp(1, 10),
                });

            let a_preds = preds.get(&a.id).unwrap_or(&empty_preds);
            let b_preds = preds.get(&b.id).unwrap_or(&empty_preds);

            let breakdown = compute_synergy_score_ctx(a, b, ctx.as_ref(), a_preds, b_preds);

            pairs.push(PairResult {
                id_a: a.id.clone(),
                id_b: b.id.clone(),
                person_a: a.name.clone(),
                person_b: b.name.clone(),
                breakdown,
            });
        }
    }

    if pairs.is_empty() {
        return None;
    }

    // Aggregate
    let team_size = persons.len();
    let n = pairs.len() as f64;
    let avg_score = (pairs.iter().map(|p| p.breakdown.total as f64).sum::<f64>() / n).round() as u8;

    let weakest = pairs
        .iter()
        .min_by_key(|p| p.breakdown.total)
        .map(|p| (p.person_a.clone(), p.person_b.clone(), p.breakdown.total));

    let strongest = pairs
        .iter()
        .max_by_key(|p| p.breakdown.total)
        .map(|p| (p.person_a.clone(), p.person_b.clone(), p.breakdown.total));

    let max_danger = pairs
        .iter()
        .map(|p| p.breakdown.danger)
        .fold(0.0f64, f64::max);

    let avg_danger = pairs.iter().map(|p| p.breakdown.danger).sum::<f64>() / n;

    // Per-context averages across all pairs
    let context_averages: Vec<(InsightContext, u8)> = InsightContext::ALL
        .iter()
        .map(|ctx| {
            let sum: u32 = pairs
                .iter()
                .filter_map(|p| {
                    p.breakdown
                        .per_context
                        .iter()
                        .find(|(c, _)| c == ctx)
                        .map(|(_, s)| *s as u32)
                })
                .sum();
            let count = pairs
                .iter()
                .filter(|p| p.breakdown.per_context.iter().any(|(c, _)| c == ctx))
                .count();
            let avg = if count > 0 {
                (sum / count as u32) as u8
            } else {
                avg_score
            };
            (*ctx, avg)
        })
        .collect();

    Some(TeamSynergy {
        pairs,
        team_size,
        avg_score,
        weakest,
        strongest,
        max_danger,
        avg_danger,
        context_averages,
    })
}
