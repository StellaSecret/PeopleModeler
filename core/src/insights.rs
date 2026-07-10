use crate::models::Person;

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

pub fn generate_insight(ctx: InsightContext, p: &Person) -> String {
    let top_mot = p
        .motivations
        .iter()
        .max_by_key(|m| m.intensity)
        .map(|m| m.r#type.i18n(crate::i18n::Lang::Fr).label)
        .unwrap_or("—");
    let top_bias = p
        .biases
        .iter()
        .max_by_key(|b| b.intensity)
        .map(|b| b.r#type.i18n(crate::i18n::Lang::Fr).label)
        .unwrap_or("—");
    match ctx {
        InsightContext::Decision => format!(
            "🧠 Analyse décisionnelle\n\n\
            Profil décisionnaire de {name}\n\
            • Style : influencé par {mot} (motivation principale) avec un biais de {bias}\n\
            • À tendance à : prendre des décisions alignées sur ses besoins profonds\n\
            • Recommandation : confronter ses décisions à des données objectives\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}",
            name = p.name,
            mot = top_mot,
            bias = top_bias,
            mots = fmt_motivations(p),
            biases = fmt_biases(p)
        ),
        InsightContext::Team => format!(
            "👥 Dynamique d'équipe\n\n\
            {name} en contexte collectif\n\
            • Moteur principal : {mot}\n\
            • Risque relationnel : {bias}\n\
            • Son apport : complète l'équipe par sa perspective unique\n\
            • À surveiller : les situations qui activent ses biais\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}",
            name = p.name,
            mot = top_mot,
            bias = top_bias,
            mots = fmt_motivations(p),
            biases = fmt_biases(p)
        ),
        InsightContext::Stress => format!(
            "⚡ Gestion du stress\n\n\
            {name} sous pression\n\
            • Déclencheur principal : activation du biais de {bias}\n\
            • Comportement attendu : repli sur ses motivations fondamentales ({mot})\n\
            • Seuil de stress : basé sur l'intensité de ses drivers\n\
            • Recommandation : créer un environnement prévisible pour réduire l'anxiété\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}",
            name = p.name,
            mot = top_mot,
            bias = top_bias,
            mots = fmt_motivations(p),
            biases = fmt_biases(p)
        ),
        InsightContext::Communication => format!(
            "💬 Style de communication\n\n\
            Communiquer avec {name}\n\
            • Canal privilégié : passer par sa motivation ({mot})\n\
            • Écueil à éviter : activer son biais de {bias}\n\
            • Approche : utiliser des arguments qui résonnent avec ses drivers\n\
            • Langage : adapter le niveau de détail à son profil OCEAN\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}",
            name = p.name,
            mot = top_mot,
            bias = top_bias,
            mots = fmt_motivations(p),
            biases = fmt_biases(p)
        ),
        InsightContext::Leadership => format!(
            "🎯 Leadership & Management\n\n\
            Manager {name}\n\
            • Levier principal : {mot}\n\
            • Piège à éviter : {bias} dans vos feedbacks\n\
            • Style de management recommandé : adapter votre approche à ses drivers\n\
            • Objectif : transformer ses biais en forces via la prise de conscience\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}",
            name = p.name,
            mot = top_mot,
            bias = top_bias,
            mots = fmt_motivations(p),
            biases = fmt_biases(p)
        ),
        InsightContext::Growth => format!(
            "🌱 Développement personnel\n\n\
            Plan de progression pour {name}\n\
            • Point d'appui : sa motivation ({mot})\n\
            • Zone de progression : atténuer le biais de {bias}\n\
            • Piste : des exercices de perspective-taking pour relativiser\n\
            • Objectif long terme : équilibrer ses drivers pour des décisions plus objectives\n\n\
            Motivation(s) active(s) :\n{mots}\n\n\
            Biais cognitif(s) détecté(s) :\n{biases}",
            name = p.name,
            mot = top_mot,
            bias = top_bias,
            mots = fmt_motivations(p),
            biases = fmt_biases(p)
        ),
    }
}
