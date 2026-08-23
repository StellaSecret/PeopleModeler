/// Per-relation-type bucket weights. All rows sum to 1.0 so the mutual score is
/// directly comparable across contexts. The 7 buckets reuse the existing dynamic
/// redistribution path (only the constants change).
pub fn rel_weights(t: crate::models::RelationType) -> (f64, f64, f64, f64, f64, f64, f64) {
    let w = crate::model_config::CFG.relation_weights(t);
    (w[0], w[1], w[2], w[3], w[4], w[5], w[6])
}
