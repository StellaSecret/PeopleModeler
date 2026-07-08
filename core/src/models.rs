use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotivationType {
    #[serde(alias = "POWER")]
    Power,
    #[serde(alias = "ACHIEVEMENT")]
    Achievement,
    #[serde(alias = "AFFILIATION")]
    Affiliation,
    #[serde(alias = "SECURITY")]
    Security,
    #[serde(alias = "AUTONOMY")]
    Autonomy,
    #[serde(alias = "RECOGNITION")]
    Recognition,
    #[serde(alias = "LEARNING")]
    Learning,
    #[serde(alias = "HELPING")]
    Helping,
}

impl std::fmt::Display for MotivationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
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

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Power => "👑",
            Self::Achievement => "🏆",
            Self::Affiliation => "🤝",
            Self::Security => "🛡️",
            Self::Autonomy => "🦅",
            Self::Recognition => "⭐",
            Self::Learning => "📚",
            Self::Helping => "❤️",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiasType {
    #[serde(alias = "CONFIRMATION")]
    Confirmation,
    #[serde(alias = "ANCHORING")]
    Anchoring,
    #[serde(alias = "AVAILABILITY")]
    Availability,
    #[serde(alias = "SUNK_COST")]
    SunkCost,
    #[serde(alias = "DUNNING_KRUGER")]
    DunningKruger,
    #[serde(alias = "LOSS_AVERSION")]
    LossAversion,
    #[serde(alias = "SOCIAL_PROOF")]
    SocialProof,
    #[serde(alias = "AUTHORITY")]
    Authority,
    #[serde(alias = "RECENCY")]
    Recency,
    #[serde(alias = "IN_GROUP")]
    InGroup,
}

impl std::fmt::Display for BiasType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
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

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Confirmation => "🔄",
            Self::Anchoring => "⚓",
            Self::Availability => "📱",
            Self::SunkCost => "💸",
            Self::DunningKruger => "🎭",
            Self::LossAversion => "😰",
            Self::SocialProof => "👥",
            Self::Authority => "🎖️",
            Self::Recency => "⏰",
            Self::InGroup => "🏠",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BehaviorTrigger {
    #[serde(alias = "STRESS")]
    Stress,
    #[serde(alias = "CONFLICT")]
    Conflict,
    #[serde(alias = "SUCCESS")]
    Success,
    #[serde(alias = "UNCERTAINTY")]
    Uncertainty,
    #[serde(alias = "RECOGNITION")]
    Recognition,
    #[serde(alias = "THREATENED")]
    Threatened,
    #[serde(alias = "CHANGE")]
    Change,
    #[serde(alias = "FEEDBACK")]
    Feedback,
}

impl std::fmt::Display for BehaviorTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl BehaviorTrigger {
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Stress => "😰",
            Self::Conflict => "⚔️",
            Self::Success => "🏆",
            Self::Uncertainty => "❓",
            Self::Recognition => "⭐",
            Self::Threatened => "🛡️",
            Self::Change => "🔄",
            Self::Feedback => "💬",
        }
    }
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
pub struct InteractionEntry {
    pub id: String,
    pub timestamp: i64,
    pub text: String,
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
        Self {
            openness: 5,
            conscientiousness: 5,
            extraversion: 5,
            agreeableness: 5,
            neuroticism: 5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub r#type: String,
    pub notes: String,
    pub created_at: i64,
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
    #[serde(default)]
    pub log: Vec<InteractionEntry>,
    pub predictions: Vec<Prediction>,
    #[serde(default = "default_confidence")]
    pub confidence: u8,
    pub created_at: i64,
    pub updated_at: i64,
}

fn default_confidence() -> u8 {
    5
}

impl Person {
    pub fn top_motivation(&self) -> Option<&Motivation> {
        self.motivations.iter().max_by_key(|m| m.intensity)
    }

    pub fn top_bias(&self) -> Option<&Bias> {
        self.biases.iter().max_by_key(|b| b.intensity)
    }
}

pub const AVATAR_EMOJIS: &[&str] = &[
    "🧑", "👩", "👨", "🧠", "🎯", "💼", "🦁", "🦊", "🐺", "🌟", "🔥", "💎", "🎸", "🧬", "🌊", "🏔️",
    // Chinese Zodiac (十二生肖)
    "🐀", "🐂", "🐅", "🐇", "🐉", "🐍", "🐎", "🐐", "🐒", "🐓", "🐕", "🐖",
];
