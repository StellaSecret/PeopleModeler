use crate::advice;
use crate::models::Person;
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
