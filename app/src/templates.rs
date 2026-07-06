use peoplemodeler_core::models::{
    Bias, BiasType, Motivation, MotivationType, OceanScores,
};

pub struct Archetype {
    pub name: &'static str,
    pub emoji: &'static str,
    pub ocean: OceanScores,
    pub motivations: Vec<Motivation>,
    pub biases: Vec<Bias>,
}

pub fn all() -> Vec<Archetype> {
    vec![
        Archetype {
            name: "The Leader",
            emoji: "👑",
            ocean: OceanScores { openness: 8, conscientiousness: 8, extraversion: 9, agreeableness: 3, neuroticism: 4 },
            motivations: vec![
                Motivation { r#type: MotivationType::Power, intensity: 9, notes: String::new() },
                Motivation { r#type: MotivationType::Achievement, intensity: 8, notes: String::new() },
                Motivation { r#type: MotivationType::Recognition, intensity: 7, notes: String::new() },
            ],
            biases: vec![
                Bias { r#type: BiasType::Confirmation, intensity: 6, evidence: String::new() },
                Bias { r#type: BiasType::DunningKruger, intensity: 7, evidence: String::new() },
            ],
        },
        Archetype {
            name: "The Analyst",
            emoji: "🔬",
            ocean: OceanScores { openness: 4, conscientiousness: 9, extraversion: 3, agreeableness: 5, neuroticism: 5 },
            motivations: vec![
                Motivation { r#type: MotivationType::Autonomy, intensity: 8, notes: String::new() },
                Motivation { r#type: MotivationType::Learning, intensity: 9, notes: String::new() },
                Motivation { r#type: MotivationType::Security, intensity: 6, notes: String::new() },
            ],
            biases: vec![
                Bias { r#type: BiasType::Anchoring, intensity: 7, evidence: String::new() },
                Bias { r#type: BiasType::SunkCost, intensity: 5, evidence: String::new() },
            ],
        },
        Archetype {
            name: "The People Person",
            emoji: "🤝",
            ocean: OceanScores { openness: 6, conscientiousness: 5, extraversion: 9, agreeableness: 9, neuroticism: 3 },
            motivations: vec![
                Motivation { r#type: MotivationType::Affiliation, intensity: 9, notes: String::new() },
                Motivation { r#type: MotivationType::Helping, intensity: 8, notes: String::new() },
                Motivation { r#type: MotivationType::Recognition, intensity: 6, notes: String::new() },
            ],
            biases: vec![
                Bias { r#type: BiasType::SocialProof, intensity: 8, evidence: String::new() },
                Bias { r#type: BiasType::InGroup, intensity: 7, evidence: String::new() },
            ],
        },
        Archetype {
            name: "The Creative",
            emoji: "🎨",
            ocean: OceanScores { openness: 10, conscientiousness: 3, extraversion: 6, agreeableness: 6, neuroticism: 6 },
            motivations: vec![
                Motivation { r#type: MotivationType::Autonomy, intensity: 9, notes: String::new() },
                Motivation { r#type: MotivationType::Learning, intensity: 8, notes: String::new() },
                Motivation { r#type: MotivationType::Achievement, intensity: 5, notes: String::new() },
            ],
            biases: vec![
                Bias { r#type: BiasType::Availability, intensity: 7, evidence: String::new() },
                Bias { r#type: BiasType::Recency, intensity: 6, evidence: String::new() },
            ],
        },
    ]
}
