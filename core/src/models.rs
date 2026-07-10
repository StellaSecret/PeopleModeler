use serde::{Deserialize, Deserializer, Serialize};

pub fn clamp_u8_1_10<'de, D: Deserializer<'de>>(d: D) -> Result<u8, D::Error> {
    let v = u8::deserialize(d)?.clamp(1, 10);
    Ok(v)
}

pub fn clamp_u8_opt_1_10<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u8>, D::Error> {
    let v = Option::<u8>::deserialize(d)?;
    Ok(v.map(|x| x.clamp(1, 10)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepDim {
    #[serde(alias = "HARDWORKER_LAZY")]
    HardworkerLazy,
    #[serde(alias = "AUTHORITATIVE_SUBMISSIVE")]
    AuthoritativeSubmissive,
    #[serde(alias = "HONEST_DECEITFUL")]
    HonestDeceitful,
    #[serde(alias = "RELIABLE_FLAKY")]
    ReliableFlaky,
    #[serde(alias = "HUMBLE_ARROGANT")]
    HumbleArrogant,
    #[serde(alias = "CALM_REACTIVE")]
    CalmReactive,
    #[serde(alias = "DIPLOMATIC_BLUNT")]
    DiplomaticBlunt,
    #[serde(alias = "GENEROUS_SELFISH")]
    GenerousSelfish,
}

impl std::fmt::Display for RepDim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl RepDim {
    pub const ALL: [Self; 8] = [
        Self::HardworkerLazy,
        Self::AuthoritativeSubmissive,
        Self::HonestDeceitful,
        Self::ReliableFlaky,
        Self::HumbleArrogant,
        Self::CalmReactive,
        Self::DiplomaticBlunt,
        Self::GenerousSelfish,
    ];

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::HardworkerLazy => "💪",
            Self::AuthoritativeSubmissive => "👑",
            Self::HonestDeceitful => "🫡",
            Self::ReliableFlaky => "🤝",
            Self::HumbleArrogant => "🌱",
            Self::CalmReactive => "🧘",
            Self::DiplomaticBlunt => "🤝",
            Self::GenerousSelfish => "🎁",
        }
    }

    pub fn pole_a_label(&self, lang: crate::i18n::Lang) -> &'static str {
        match lang {
            crate::i18n::Lang::Fr => match self {
                Self::HardworkerLazy => "Travailleur",
                Self::AuthoritativeSubmissive => "Autoritaire",
                Self::HonestDeceitful => "Honnête",
                Self::ReliableFlaky => "Fiable",
                Self::HumbleArrogant => "Humble",
                Self::CalmReactive => "Calme",
                Self::DiplomaticBlunt => "Diplomate",
                Self::GenerousSelfish => "Généreux",
            },
            crate::i18n::Lang::En => match self {
                Self::HardworkerLazy => "Hardworker",
                Self::AuthoritativeSubmissive => "Authoritative",
                Self::HonestDeceitful => "Honest",
                Self::ReliableFlaky => "Reliable",
                Self::HumbleArrogant => "Humble",
                Self::CalmReactive => "Calm",
                Self::DiplomaticBlunt => "Diplomatic",
                Self::GenerousSelfish => "Generous",
            },
        }
    }

    pub fn pole_b_label(&self, lang: crate::i18n::Lang) -> &'static str {
        match lang {
            crate::i18n::Lang::Fr => match self {
                Self::HardworkerLazy => "Paresseux",
                Self::AuthoritativeSubmissive => "Soumis",
                Self::HonestDeceitful => "Fourbe",
                Self::ReliableFlaky => "Inconstant",
                Self::HumbleArrogant => "Arrogant",
                Self::CalmReactive => "Réactif",
                Self::DiplomaticBlunt => "Direct",
                Self::GenerousSelfish => "Égoïste",
            },
            crate::i18n::Lang::En => match self {
                Self::HardworkerLazy => "Lazy",
                Self::AuthoritativeSubmissive => "Submissive",
                Self::HonestDeceitful => "Deceitful",
                Self::ReliableFlaky => "Flaky",
                Self::HumbleArrogant => "Arrogant",
                Self::CalmReactive => "Reactive",
                Self::DiplomaticBlunt => "Blunt",
                Self::GenerousSelfish => "Selfish",
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepScores {
    #[serde(default)]
    pub hardworker_lazy: Option<u8>,
    #[serde(default)]
    pub authoritative_submissive: Option<u8>,
    #[serde(default)]
    pub honest_deceitful: Option<u8>,
    #[serde(default)]
    pub reliable_flaky: Option<u8>,
    #[serde(default)]
    pub humble_arrogant: Option<u8>,
    #[serde(default)]
    pub calm_reactive: Option<u8>,
    #[serde(default)]
    pub diplomatic_blunt: Option<u8>,
    #[serde(default)]
    pub generous_selfish: Option<u8>,
}

impl RepScores {
    pub fn has_any(&self) -> bool {
        RepDim::ALL.iter().any(|d| self.score(*d).is_some())
    }

    /// Score for a given dimension: 0=pole B, 10=pole A
    pub fn score(&self, dim: RepDim) -> Option<u8> {
        match dim {
            RepDim::HardworkerLazy => self.hardworker_lazy,
            RepDim::AuthoritativeSubmissive => self.authoritative_submissive,
            RepDim::HonestDeceitful => self.honest_deceitful,
            RepDim::ReliableFlaky => self.reliable_flaky,
            RepDim::HumbleArrogant => self.humble_arrogant,
            RepDim::CalmReactive => self.calm_reactive,
            RepDim::DiplomaticBlunt => self.diplomatic_blunt,
            RepDim::GenerousSelfish => self.generous_selfish,
        }
    }
}

impl Default for RepScores {
    fn default() -> Self {
        Self {
            hardworker_lazy: None,
            authoritative_submissive: None,
            honest_deceitful: None,
            reliable_flaky: None,
            humble_arrogant: None,
            calm_reactive: None,
            diplomatic_blunt: None,
            generous_selfish: None,
        }
    }
}

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
    #[serde(deserialize_with = "clamp_u8_1_10")]
    pub intensity: u8,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bias {
    pub r#type: BiasType,
    #[serde(deserialize_with = "clamp_u8_1_10")]
    pub intensity: u8,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehavioralPattern {
    pub trigger: BehaviorTrigger,
    pub predicted_behavior: String,
    #[serde(deserialize_with = "clamp_u8_1_10")]
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
    #[serde(default, deserialize_with = "clamp_u8_opt_1_10")]
    pub openness: Option<u8>,
    #[serde(default, deserialize_with = "clamp_u8_opt_1_10")]
    pub conscientiousness: Option<u8>,
    #[serde(default, deserialize_with = "clamp_u8_opt_1_10")]
    pub extraversion: Option<u8>,
    #[serde(default, deserialize_with = "clamp_u8_opt_1_10")]
    pub agreeableness: Option<u8>,
    #[serde(default, deserialize_with = "clamp_u8_opt_1_10")]
    pub neuroticism: Option<u8>,
}

impl Default for OceanScores {
    fn default() -> Self {
        Self {
            openness: None,
            conscientiousness: None,
            extraversion: None,
            agreeableness: None,
            neuroticism: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationType {
    WorksWith,
    Manages,
    ReportsTo,
    Friends,
    Family,
    Partner,
    Mentors,
    Collaborates,
}

impl std::fmt::Display for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl RelationType {
    pub const ALL: [Self; 8] = [
        Self::WorksWith,
        Self::Manages,
        Self::ReportsTo,
        Self::Friends,
        Self::Family,
        Self::Partner,
        Self::Mentors,
        Self::Collaborates,
    ];
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub r#type: RelationType,
    pub notes: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    #[serde(default)]
    pub color: Option<String>,
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub role: String,
    pub context: String,
    pub avatar_emoji: String,
    pub tags: Vec<Tag>,
    pub notes: String,
    pub motivations: Vec<Motivation>,
    pub biases: Vec<Bias>,
    #[serde(default)]
    pub rep_scores: RepScores,
    pub behavioral_patterns: Vec<BehavioralPattern>,
    pub ocean: OceanScores,
    #[serde(default)]
    pub log: Vec<InteractionEntry>,
    pub predictions: Vec<Prediction>,
    #[serde(default = "default_confidence", deserialize_with = "clamp_u8_1_10")]
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
