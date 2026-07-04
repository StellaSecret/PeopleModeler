use crate::models::OceanScores;

pub struct TraitInterpretation {
    pub high: &'static str,
    pub low: &'static str,
}

pub fn interpret_openness(score: u8) -> &'static str {
    if score >= 7 {
        "très ouvert aux nouvelles idées, créatif et curieux"
    } else if score >= 4 {
        "équilibré entre tradition et innovation"
    } else {
        "pragmatique, préfère les routines et le concret"
    }
}

pub fn interpret_conscientiousness(score: u8) -> &'static str {
    if score >= 7 {
        "organisé, fiable, orienté résultats et détails"
    } else if score >= 4 {
        "niveau modéré de structure et de flexibilité"
    } else {
        "flexible et spontané, peut manquer de rigueur"
    }
}

pub fn interpret_extraversion(score: u8) -> &'static str {
    if score >= 7 {
        "extraverti, énergique, cherche la stimulation sociale"
    } else if score >= 4 {
        "équilibré entre solitude et vie sociale"
    } else {
        "introverti, réfléchi, préfère les interactions limitées"
    }
}

pub fn interpret_agreeableness(score: u8) -> &'static str {
    if score >= 7 {
        "coopératif, empathique, cherche l'harmonie"
    } else if score >= 4 {
        "équilibré entre affirmation de soi et diplomatie"
    } else {
        "direct voire abrasif, met ses objectifs avant les relations"
    }
}

pub fn interpret_neuroticism(score: u8) -> &'static str {
    if score >= 7 {
        "émotionnellement réactif, stressable, sensible aux critiques"
    } else if score >= 4 {
        "réactivité émotionnelle modérée"
    } else {
        "stable émotionnellement, calme sous pression"
    }
}

pub fn interpret_all(ocean: &OceanScores) -> String {
    format!(
        "O: {}\nC: {}\nE: {}\nA: {}\nN: {}",
        interpret_openness(ocean.openness),
        interpret_conscientiousness(ocean.conscientiousness),
        interpret_extraversion(ocean.extraversion),
        interpret_agreeableness(ocean.agreeableness),
        interpret_neuroticism(ocean.neuroticism),
    )
}
