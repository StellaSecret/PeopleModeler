use crate::advice;
use crate::models::{FacetKind, Person};
use crate::synergy::PersonProfile;
use crate::validation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum InsightContext {
    Decision,
    Team,
    Stress,
    Communication,
    Leadership,
    Growth,
}

impl InsightContext {
    pub const ALL: [Self; 6] = [
        Self::Decision,
        Self::Team,
        Self::Stress,
        Self::Communication,
        Self::Leadership,
        Self::Growth,
    ];
}

fn fmt_motivations(p: &Person) -> String {
    if p.motivations.is_empty() {
        return "• Aucune motivation définie\n".into();
    }
    p.motivations
        .iter()
        .map(|m| {
            format!(
                "• {} (intensité {}/10)",
                m.r#type.i18n(crate::i18n::Lang::Fr).label,
                m.intensity
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn fmt_biases(p: &Person) -> String {
    if p.biases.is_empty() {
        return "• Aucun biais défini\n".into();
    }
    p.biases
        .iter()
        .map(|b| {
            format!(
                "• {} (intensité {}/10)",
                b.r#type.i18n(crate::i18n::Lang::Fr).label,
                b.intensity
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn fmt_flags(flags: &[&str], lang: crate::i18n::Lang) -> String {
    if flags.is_empty() {
        return "• Aucun signal d'alerte\n".into();
    }
    flags
        .iter()
        .map(|f| format!("• ⚠ {}", advice::flag_action(f, lang)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn fmt_advice(advice_list: &[advice::FlagAdvice]) -> String {
    if advice_list.is_empty() {
        return "• Aucune recommandation spécifique\n".into();
    }
    advice_list
        .iter()
        .take(5)
        .map(|a| format!("• [{}] {}", a.category, a.action))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn generate_insight(ctx: InsightContext, p: &Person, lang: crate::i18n::Lang) -> String {
    let profile = crate::synergy::compute_person_profile(p);
    generate_insight_with_profile(ctx, p, &profile, lang)
}

/// Insight for a specific facet. A persona facet runs the whole pipeline on
/// the masked (persona-merged) profile and appends a mask-gap line when the
/// person has a persona for that arena. A facet the person has no persona for
/// is not a defined context: a "no persona" notice is returned instead of an
/// insight fabricated from the anchor profile.
pub fn generate_insight_facet(
    ctx: InsightContext,
    p: &Person,
    kind: FacetKind,
    lang: crate::i18n::Lang,
) -> String {
    let Some(pm) = p.facet_person(kind) else {
        let facet = match kind {
            FacetKind::Work => "professionnelle",
            FacetKind::Online => "en ligne",
            FacetKind::Base => "vie privée",
        };
        return format!(
            "Aucune persona {facet} n'est définie pour cette personne : son comportement dans ce contexte est inconnu."
        );
    };
    let out = generate_insight(ctx, &pm, lang);
    let mask_line = match kind {
        FacetKind::Base => None,
        FacetKind::Work => crate::synergy::mask_gap(p).map(|gap| {
            format!(
                "\n\n🎭 Masque professionnel : écart base/travail {:.0}% ({})",
                gap.gap * 100.0,
                mask_band_fr(gap.band)
            )
        }),
        FacetKind::Online => crate::synergy::mask_gap_for(p, kind).map(|gap| {
            format!(
                "\n\n🎭 Masque en ligne : écart base/en ligne {:.0}% ({})",
                gap.gap * 100.0,
                mask_band_fr(gap.band)
            )
        }),
    };
    if let Some(line) = mask_line {
        return format!("{out}{line}");
    }
    out
}

fn mask_band_fr(band: crate::synergy::MaskBand) -> &'static str {
    match band {
        crate::synergy::MaskBand::Low => "faible",
        crate::synergy::MaskBand::Moderate => "modéré",
        crate::synergy::MaskBand::High => "élevé",
    }
}

pub fn generate_insight_with_profile(
    ctx: InsightContext,
    p: &Person,
    profile: &PersonProfile,
    lang: crate::i18n::Lang,
) -> String {
    let top_mot = p
        .motivations
        .iter()
        .max_by_key(|m| m.intensity)
        .map(|m| m.r#type.i18n(lang).label)
        .unwrap_or("—");
    let top_bias = p
        .biases
        .iter()
        .max_by_key(|b| b.intensity)
        .map(|b| b.r#type.i18n(lang).label)
        .unwrap_or("—");
    let flags = validation::all_person_flags(p);
    let prioritized = advice::per_context_advice(p, profile, ctx, lang);
    let flag_count = flags.len();
    let advice_block = fmt_advice(&prioritized);
    let flag_block = fmt_flags(&flags, lang);

    let completeness_hint = if profile.completeness < 40 {
        "\n⚠ Profil incomplet — les recommandations sont moins fiables."
    } else if profile.completeness < 70 {
        "\n• Profil partiellement complété — complétez pour des conseils plus précis."
    } else {
        ""
    };

    match ctx {
        InsightContext::Decision => format!(
            "🧠 Analyse décisionnelle\n\n\
            Profil décisionnaire de {name} (complétude {comp}%)\n\
            • Style : influencé par {mot} (motivation principale) avec un biais de {bias}\n\
            • Score profil : {total}/100\n\
            • Signaux d'alerte : {flag_count}\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}\n\n\
            Alertes :\n{flag_block}\n\n\
            Recommandations prioritaires :\n{advice}{comp_hint}",
            name = p.name,
            comp = profile.completeness,
            mot = top_mot,
            bias = top_bias,
            total = profile.total,
            flag_count = flag_count,
            mots = fmt_motivations(p),
            biases = fmt_biases(p),
            flag_block = flag_block,
            advice = advice_block,
            comp_hint = completeness_hint,
        ),
        InsightContext::Team => format!(
            "👥 Dynamique d'équipe\n\n\
            {name} en contexte collectif (complétude {comp}%)\n\
            • Moteur principal : {mot}\n\
            • Risque relationnel : {bias}\n\
            • Score profil : {total}/100\n\
            • Signaux d'alerte : {flag_count}\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}\n\n\
            Alertes :\n{flag_block}\n\n\
            Recommandations prioritaires :\n{advice}{comp_hint}",
            name = p.name,
            comp = profile.completeness,
            mot = top_mot,
            bias = top_bias,
            total = profile.total,
            flag_count = flag_count,
            mots = fmt_motivations(p),
            biases = fmt_biases(p),
            flag_block = flag_block,
            advice = advice_block,
            comp_hint = completeness_hint,
        ),
        InsightContext::Stress => format!(
            "⚡ Gestion du stress\n\n\
            {name} sous pression (complétude {comp}%)\n\
            • Déclencheur principal : activation du biais de {bias}\n\
            • Comportement attendu : repli sur ses motivations fondamentales ({mot})\n\
            • Score profil : {total}/100\n\
            • Signaux d'alerte : {flag_count}\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}\n\n\
            Alertes :\n{flag_block}\n\n\
            Recommandations prioritaires :\n{advice}{comp_hint}",
            name = p.name,
            comp = profile.completeness,
            mot = top_mot,
            bias = top_bias,
            total = profile.total,
            flag_count = flag_count,
            mots = fmt_motivations(p),
            biases = fmt_biases(p),
            flag_block = flag_block,
            advice = advice_block,
            comp_hint = completeness_hint,
        ),
        InsightContext::Communication => format!(
            "💬 Style de communication\n\n\
            Communiquer avec {name} (complétude {comp}%)\n\
            • Canal privilégié : passer par sa motivation ({mot})\n\
            • Écueil à éviter : activer son biais de {bias}\n\
            • Score profil : {total}/100\n\
            • Signaux d'alerte : {flag_count}\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}\n\n\
            Alertes :\n{flag_block}\n\n\
            Recommandations prioritaires :\n{advice}{comp_hint}",
            name = p.name,
            comp = profile.completeness,
            mot = top_mot,
            bias = top_bias,
            total = profile.total,
            flag_count = flag_count,
            mots = fmt_motivations(p),
            biases = fmt_biases(p),
            flag_block = flag_block,
            advice = advice_block,
            comp_hint = completeness_hint,
        ),
        InsightContext::Leadership => format!(
            "🎯 Leadership & Management\n\n\
            Manager {name} (complétude {comp}%)\n\
            • Levier principal : {mot}\n\
            • Piège à éviter : {bias} dans vos feedbacks\n\
            • Score profil : {total}/100\n\
            • Signaux d'alerte : {flag_count}\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}\n\n\
            Alertes :\n{flag_block}\n\n\
            Recommandations prioritaires :\n{advice}{comp_hint}",
            name = p.name,
            comp = profile.completeness,
            mot = top_mot,
            bias = top_bias,
            total = profile.total,
            flag_count = flag_count,
            mots = fmt_motivations(p),
            biases = fmt_biases(p),
            flag_block = flag_block,
            advice = advice_block,
            comp_hint = completeness_hint,
        ),
        InsightContext::Growth => format!(
            "🌱 Développement personnel\n\n\
            Plan de progression pour {name} (complétude {comp}%)\n\
            • Point d'appui : sa motivation ({mot})\n\
            • Zone de progression : atténuer le biais de {bias}\n\
            • Score profil : {total}/100\n\
            • Signaux d'alerte : {flag_count}\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}\n\n\
            Alertes :\n{flag_block}\n\n\
            Recommandations prioritaires :\n{advice}{comp_hint}",
            name = p.name,
            comp = profile.completeness,
            mot = top_mot,
            bias = top_bias,
            total = profile.total,
            flag_count = flag_count,
            mots = fmt_motivations(p),
            biases = fmt_biases(p),
            flag_block = flag_block,
            advice = advice_block,
            comp_hint = completeness_hint,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;
    use crate::models::{FacetKind, Person};

    fn masked_person() -> Person {
        // Base profile + a work persona that differs on at least one bucket,
        // so the work facet yields a nonzero mask gap.
        serde_json::from_str(
            r#"{
                "id": "insight-test",
                "name": "Insight Test",
                "role": "Tester",
                "context": "test",
                "avatar_emoji": "🧑",
                "tags": [],
                "notes": "",
                "motivations": [{"type":"Power","intensity":8,"notes":"ambitious"}],
                "biases": [{"type":"Anchoring","intensity":7,"evidence":"sticky"}],
                "rep_scores": {"hardworker_lazy":6},
                "behavioral_patterns": [{"trigger":"Change","predicted_behavior":"embraces_change"}],
                "ocean": {"openness":7,"conscientiousness":6,"extraversion":8,"agreeableness":5,"neuroticism":4},
                "log": [],
                "predictions": [],
                "confidence": 5,
                "created_at": 0,
                "updated_at": 0,
                "persona": {
                    "ocean": {"openness":4}
                }
            }"#,
        )
        .unwrap()
    }

    #[test]
    fn facet_insight_base_outlines_profile_without_mask() {
        let p = masked_person();
        let out = generate_insight_facet(InsightContext::Decision, &p, FacetKind::Base, Lang::Fr);
        assert!(
            out.contains("Insight Test"),
            "base insight missing name: {out}"
        );
        assert!(
            !out.contains("🎭 Masque professionnel"),
            "base facet must not append the mask-gap line: {out}"
        );
    }

    #[test]
    fn facet_insight_work_appends_mask_gap() {
        let p = masked_person();
        let out = generate_insight_facet(InsightContext::Decision, &p, FacetKind::Work, Lang::Fr);
        assert!(
            out.contains("🎭 Masque professionnel"),
            "work facet must append the mask-gap line: {out}"
        );
    }

    #[test]
    fn facet_insight_work_without_persona_has_no_mask() {
        let mut p = masked_person();
        p.persona = None;
        let out = generate_insight_facet(InsightContext::Decision, &p, FacetKind::Work, Lang::Fr);
        assert!(
            !out.contains("🎭 Masque professionnel"),
            "no persona, no mask: {out}"
        );
    }

    #[test]
    fn mask_band_fr_maps_every_band() {
        assert_eq!(mask_band_fr(crate::synergy::MaskBand::Low), "faible");
        assert_eq!(mask_band_fr(crate::synergy::MaskBand::Moderate), "modéré");
        assert_eq!(mask_band_fr(crate::synergy::MaskBand::High), "élevé");
    }
}
