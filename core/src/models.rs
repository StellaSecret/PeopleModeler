use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotivationType {
    Power,
    Achievement,
    Affiliation,
    Security,
    Autonomy,
    Recognition,
    Learning,
    Helping,
}

impl MotivationType {
    pub const ALL: [Self; 8] = [
        Self::Power,
        Self::Achievement,
        Self::Affiliation,
        Self::Security,
        Self::Autonomy,
        Self::Recognition,
        Self::Learning,
        Self::Helping,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiasType {
    Confirmation,
    Anchoring,
    Availability,
    SunkCost,
    DunningKruger,
    LossAversion,
    SocialProof,
    Authority,
    Recency,
    InGroup,
}

impl BiasType {
    pub const ALL: [Self; 10] = [
        Self::Confirmation,
        Self::Anchoring,
        Self::Availability,
        Self::SunkCost,
        Self::DunningKruger,
        Self::LossAversion,
        Self::SocialProof,
        Self::Authority,
        Self::Recency,
        Self::InGroup,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BehaviorTrigger {
    Stress,
    Conflict,
    Success,
    Uncertainty,
    Recognition,
    Threatened,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motivation {
    pub r#type: MotivationType,
    pub intensity: u8,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bias {
    pub r#type: BiasType,
    pub intensity: u8,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehavioralPattern {
    pub trigger: BehaviorTrigger,
    pub predicted_behavior: String,
    pub confidence: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prediction {
    pub id: String,
    pub person_id: String,
    pub context: String,
    pub predicted_outcome: String,
    pub actual_outcome: Option<String>,
    pub accuracy: Option<u8>,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
    pub resolved: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OceanScores {
    pub openness: u8,
    pub conscientiousness: u8,
    pub extraversion: u8,
    pub agreeableness: u8,
    pub neuroticism: u8,
}

impl Default for OceanScores {
    fn default() -> Self {
        Self { openness: 5, conscientiousness: 5, extraversion: 5, agreeableness: 5, neuroticism: 5 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub role: String,
    pub context: String,
    pub avatar_emoji: String,
    pub tags: Vec<String>,
    pub notes: String,
    pub motivations: Vec<Motivation>,
    pub biases: Vec<Bias>,
    pub behavioral_patterns: Vec<BehavioralPattern>,
    pub ocean: OceanScores,
    pub predictions: Vec<Prediction>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Person {
    pub fn top_motivation(&self) -> Option<&Motivation> {
        self.motivations.iter().max_by_key(|m| m.intensity)
    }

    pub fn top_bias(&self) -> Option<&Bias> {
        self.biases.iter().max_by_key(|b| b.intensity)
    }
}
