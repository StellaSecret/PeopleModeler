use peoplemodeler_core::models::{
    Bias, BiasType, Motivation, MotivationType, OceanScores, PersonalStyle, RepScores, Value,
};

#[allow(dead_code)]
pub struct Archetype {
    pub name: &'static str,
    pub emoji: &'static str,
    pub ocean: OceanScores,
    pub motivations: Vec<Motivation>,
    pub biases: Vec<Bias>,
    pub rep_scores: RepScores,
    pub styles: Vec<PersonalStyle>,
    pub values: Vec<Value>,
}

pub fn all() -> Vec<Archetype> {
    vec![
        Archetype {
            name: "The Leader",
            emoji: "👑",
            ocean: OceanScores {
                openness: Some(8),
                conscientiousness: Some(8),
                extraversion: Some(9),
                agreeableness: Some(3),
                neuroticism: Some(4),
            },
            motivations: vec![
                Motivation {
                    r#type: MotivationType::Power,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Achievement,
                    intensity: 8,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Recognition,
                    intensity: 7,
                    notes: String::new(),
                },
            ],
            biases: vec![
                Bias {
                    r#type: BiasType::Confirmation,
                    intensity: 6,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::DunningKruger,
                    intensity: 7,
                    evidence: String::new(),
                },
            ],
            rep_scores: RepScores::default(),
            styles: vec![],
            values: vec![],
        },
        Archetype {
            name: "The Analyst",
            emoji: "🔬",
            ocean: OceanScores {
                openness: Some(4),
                conscientiousness: Some(9),
                extraversion: Some(3),
                agreeableness: Some(5),
                neuroticism: Some(5),
            },
            motivations: vec![
                Motivation {
                    r#type: MotivationType::Autonomy,
                    intensity: 8,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Learning,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Security,
                    intensity: 6,
                    notes: String::new(),
                },
            ],
            biases: vec![
                Bias {
                    r#type: BiasType::Anchoring,
                    intensity: 7,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::SunkCost,
                    intensity: 5,
                    evidence: String::new(),
                },
            ],
            rep_scores: RepScores::default(),
            styles: vec![],
            values: vec![],
        },
        Archetype {
            name: "The People Person",
            emoji: "🤝",
            ocean: OceanScores {
                openness: Some(6),
                conscientiousness: Some(5),
                extraversion: Some(9),
                agreeableness: Some(9),
                neuroticism: Some(3),
            },
            motivations: vec![
                Motivation {
                    r#type: MotivationType::Affiliation,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Helping,
                    intensity: 8,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Recognition,
                    intensity: 6,
                    notes: String::new(),
                },
            ],
            biases: vec![
                Bias {
                    r#type: BiasType::SocialProof,
                    intensity: 8,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::InGroup,
                    intensity: 7,
                    evidence: String::new(),
                },
            ],
            rep_scores: RepScores::default(),
            styles: vec![],
            values: vec![],
        },
        Archetype {
            name: "The Creative",
            emoji: "🎨",
            ocean: OceanScores {
                openness: Some(10),
                conscientiousness: Some(3),
                extraversion: Some(6),
                agreeableness: Some(6),
                neuroticism: Some(6),
            },
            motivations: vec![
                Motivation {
                    r#type: MotivationType::Autonomy,
                    intensity: 9,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Learning,
                    intensity: 8,
                    notes: String::new(),
                },
                Motivation {
                    r#type: MotivationType::Achievement,
                    intensity: 5,
                    notes: String::new(),
                },
            ],
            biases: vec![
                Bias {
                    r#type: BiasType::Availability,
                    intensity: 7,
                    evidence: String::new(),
                },
                Bias {
                    r#type: BiasType::Recency,
                    intensity: 6,
                    evidence: String::new(),
                },
            ],
            rep_scores: RepScores::default(),
            styles: vec![],
            values: vec![],
        },
    ]
}
