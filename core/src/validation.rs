use crate::models::OceanScores;
use crate::models::RepScores;

pub fn ocean_rep_flags(ocean: &OceanScores, rep: &RepScores) -> Vec<&'static str> {
    let mut flags = Vec::new();

    if ocean.extraversion >= Some(8) && ocean.agreeableness <= Some(3) {
        flags.push("flag_high_e_low_a");
    }
    if ocean.neuroticism >= Some(8) && ocean.conscientiousness <= Some(3) {
        flags.push("flag_high_n_low_c");
    }
    if ocean.openness >= Some(8) && ocean.conscientiousness <= Some(3) {
        flags.push("flag_high_o_low_c");
    }
    if rep.calm_reactive >= Some(8) && ocean.neuroticism >= Some(8) {
        flags.push("flag_calm_neurotic");
    }
    if rep.honest_deceitful >= Some(8) && rep.generous_selfish <= Some(3) {
        flags.push("flag_honest_selfish");
    }

    flags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{OceanScores, RepScores};

    #[test]
    fn test_no_flags_default() {
        let o = OceanScores::default();
        let r = RepScores::default();
        assert!(ocean_rep_flags(&o, &r).is_empty());
    }

    #[test]
    fn test_high_e_low_a() {
        let o = OceanScores {
            extraversion: Some(8),
            agreeableness: Some(2),
            ..Default::default()
        };
        let r = RepScores::default();
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_high_e_low_a"));
    }

    #[test]
    fn test_high_n_low_c() {
        let o = OceanScores {
            neuroticism: Some(9),
            conscientiousness: Some(2),
            ..Default::default()
        };
        let r = RepScores::default();
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_high_n_low_c"));
    }

    #[test]
    fn test_calm_neurotic() {
        let o = OceanScores {
            neuroticism: Some(9),
            ..Default::default()
        };
        let r = RepScores {
            calm_reactive: Some(8),
            ..Default::default()
        };
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_calm_neurotic"));
    }

    #[test]
    fn test_honest_selfish() {
        let o = OceanScores::default();
        let r = RepScores {
            honest_deceitful: Some(9),
            generous_selfish: Some(2),
            ..Default::default()
        };
        let flags = ocean_rep_flags(&o, &r);
        assert!(flags.contains(&"flag_honest_selfish"));
    }

    #[test]
    fn test_threshold_bounds() {
        let o = OceanScores {
            extraversion: Some(7),
            agreeableness: Some(4),
            ..Default::default()
        };
        let r = RepScores::default();
        assert!(ocean_rep_flags(&o, &r).is_empty());
    }
}
