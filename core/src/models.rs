use serde::de;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

pub fn clamp_u8_1_10<'de, D: Deserializer<'de>>(d: D) -> Result<u8, D::Error> {
    let v = u8::deserialize(d)?.clamp(1, 10);
    Ok(v)
}

pub fn clamp_u8_0_10<'de, D: Deserializer<'de>>(d: D) -> Result<u8, D::Error> {
    let v = u8::deserialize(d)?.clamp(0, 10);
    Ok(v)
}

pub fn deserialize_behavior_response<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<BehaviorResponse, D::Error> {
    struct Brv;
    impl<'de> de::Visitor<'de> for Brv {
        type Value = BehaviorResponse;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a behavior response variant")
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<BehaviorResponse, E> {
            if v.is_empty() {
                return Ok(BehaviorResponse::SeeksSupport);
            }
            BehaviorResponse::deserialize(de::value::StrDeserializer::<E>::new(v))
        }
    }
    d.deserialize_str(Brv)
}

pub fn clamp_u8_opt_1_10<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u8>, D::Error> {
    let v = Option::<u8>::deserialize(d)?;
    Ok(v.map(|x| x.clamp(1, 10)))
}

pub fn clamp_i8_opt_neg3_3<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i8>, D::Error> {
    let v = Option::<i8>::deserialize(d)?;
    Ok(v.map(|x| x.clamp(-3, 3)))
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
    #[serde(alias = "FAIR_FAVORITISM")]
    FairFavoritism,
    #[serde(alias = "TRUSTING_SUSPICIOUS")]
    TrustingSuspicious,
    #[serde(alias = "ASSERTIVE_PASSIVE")]
    AssertivePassive,
    #[serde(alias = "EMPATHETIC_DETACHED")]
    EmpatheticDetached,
    #[serde(alias = "ADAPTABLE_RIGID")]
    AdaptableRigid,
}

impl std::fmt::Display for RepDim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl RepDim {
    pub const ALL: [Self; 13] = [
        Self::HardworkerLazy,
        Self::AuthoritativeSubmissive,
        Self::HonestDeceitful,
        Self::ReliableFlaky,
        Self::HumbleArrogant,
        Self::CalmReactive,
        Self::DiplomaticBlunt,
        Self::GenerousSelfish,
        Self::FairFavoritism,
        Self::TrustingSuspicious,
        Self::AssertivePassive,
        Self::EmpatheticDetached,
        Self::AdaptableRigid,
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
            Self::FairFavoritism => "⚖️",
            Self::TrustingSuspicious => "🤗",
            Self::AssertivePassive => "📢",
            Self::EmpatheticDetached => "💗",
            Self::AdaptableRigid => "🌿",
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
                Self::FairFavoritism => "Équitable",
                Self::TrustingSuspicious => "Confiant",
                Self::AssertivePassive => "Affirmé",
                Self::EmpatheticDetached => "Empathique",
                Self::AdaptableRigid => "Flexible",
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
                Self::FairFavoritism => "Fair",
                Self::TrustingSuspicious => "Trusting",
                Self::AssertivePassive => "Assertive",
                Self::EmpatheticDetached => "Empathetic",
                Self::AdaptableRigid => "Adaptable",
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
                Self::FairFavoritism => "Partial",
                Self::TrustingSuspicious => "Méfiant",
                Self::AssertivePassive => "Passif",
                Self::EmpatheticDetached => "Détaché",
                Self::AdaptableRigid => "Rigide",
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
                Self::FairFavoritism => "Favoritism",
                Self::TrustingSuspicious => "Suspicious",
                Self::AssertivePassive => "Passive",
                Self::EmpatheticDetached => "Detached",
                Self::AdaptableRigid => "Rigid",
            },
        }
    }

    pub fn is_context_dependent(&self) -> bool {
        matches!(
            self,
            Self::AuthoritativeSubmissive
                | Self::DiplomaticBlunt
                | Self::TrustingSuspicious
                | Self::AssertivePassive
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(default)]
    pub fair_favoritism: Option<u8>,
    #[serde(default)]
    pub trusting_suspicious: Option<u8>,
    #[serde(default)]
    pub assertive_passive: Option<u8>,
    #[serde(default)]
    pub empathetic_detached: Option<u8>,
    #[serde(default)]
    pub adaptable_rigid: Option<u8>,
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
            RepDim::FairFavoritism => self.fair_favoritism,
            RepDim::TrustingSuspicious => self.trusting_suspicious,
            RepDim::AssertivePassive => self.assertive_passive,
            RepDim::EmpatheticDetached => self.empathetic_detached,
            RepDim::AdaptableRigid => self.adaptable_rigid,
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
    #[serde(alias = "CREATIVITY")]
    Creativity,
    #[serde(alias = "FAIRNESS")]
    Fairness,
}

impl std::fmt::Display for MotivationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl MotivationType {
    pub const ALL: [Self; 10] = [
        Self::Power,
        Self::Achievement,
        Self::Affiliation,
        Self::Security,
        Self::Autonomy,
        Self::Recognition,
        Self::Learning,
        Self::Helping,
        Self::Creativity,
        Self::Fairness,
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
            Self::Creativity => "🎨",
            Self::Fairness => "⚖️",
        }
    }
}

// ---------- Values (Phase 6) ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
    #[serde(alias = "CAREER")]
    Career,
    #[serde(alias = "FAMILY")]
    Family,
    #[serde(alias = "HEALTH")]
    Health,
    #[serde(alias = "WEALTH")]
    Wealth,
    #[serde(alias = "STABILITY")]
    Stability,
    #[serde(alias = "ADVENTURE")]
    Adventure,
    #[serde(alias = "COMMUNITY")]
    Community,
    #[serde(alias = "KNOWLEDGE")]
    Knowledge,
    #[serde(alias = "FAITH")]
    Faith,
    #[serde(alias = "LOYALTY")]
    Loyalty,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl ValueType {
    pub const ALL: [Self; 10] = [
        Self::Career,
        Self::Family,
        Self::Health,
        Self::Wealth,
        Self::Stability,
        Self::Adventure,
        Self::Community,
        Self::Knowledge,
        Self::Faith,
        Self::Loyalty,
    ];

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Career => "💼",
            Self::Family => "👨‍👩‍👧",
            Self::Health => "💪",
            Self::Wealth => "💰",
            Self::Stability => "🏠",
            Self::Adventure => "🧭",
            Self::Community => "🌍",
            Self::Knowledge => "🔬",
            Self::Faith => "🕊️",
            Self::Loyalty => "🔗",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value {
    pub r#type: ValueType,
    #[serde(deserialize_with = "clamp_u8_1_10")]
    pub intensity: u8,
    #[serde(deserialize_with = "clamp_u8_1_10")]
    pub priority: u8,
    pub notes: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    #[serde(alias = "IMPOSTOR")]
    Impostor,
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
    #[serde(alias = "FAVORITISM")]
    Favoritism,
}

impl std::fmt::Display for BiasType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl BiasType {
    pub const ALL: [Self; 12] = [
        Self::Confirmation,
        Self::Anchoring,
        Self::Availability,
        Self::SunkCost,
        Self::DunningKruger,
        Self::Impostor,
        Self::LossAversion,
        Self::SocialProof,
        Self::Authority,
        Self::Recency,
        Self::InGroup,
        Self::Favoritism,
    ];

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Confirmation => "🔄",
            Self::Anchoring => "⚓",
            Self::Availability => "📱",
            Self::SunkCost => "💸",
            Self::DunningKruger => "🎭",
            Self::Impostor => "🫥",
            Self::LossAversion => "😰",
            Self::SocialProof => "👥",
            Self::Authority => "🎖️",
            Self::Recency => "⏰",
            Self::InGroup => "🏠",
            Self::Favoritism => "🎯",
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
    #[serde(alias = "INJUSTICE")]
    Injustice,
}

impl std::fmt::Display for BehaviorTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl BehaviorTrigger {
    pub const ALL: [Self; 9] = [
        Self::Stress,
        Self::Conflict,
        Self::Success,
        Self::Uncertainty,
        Self::Recognition,
        Self::Threatened,
        Self::Change,
        Self::Feedback,
        Self::Injustice,
    ];

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
            Self::Injustice => "⚖️",
        }
    }

    /// Returns true for maladaptive triggers: Conflict, Stress, Threatened, Injustice.
    pub fn is_negative(self) -> bool {
        matches!(
            self,
            Self::Conflict | Self::Stress | Self::Threatened | Self::Injustice
        )
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
    #[serde(deserialize_with = "clamp_u8_0_10")]
    pub intensity: u8,
    pub evidence: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorResponse {
    // Stress
    RemainsCalm,
    SeeksSupport,
    StaysFocused,
    BecomesQuiet,
    BecomesIrritable,
    Overwhelmed,
    Panics,
    // Conflict
    FacilitatesResolution,
    CommunicatesOpenly,
    SeeksCompromise,
    StaysSilent,
    BecomesPassiveAggressive,
    BecomesDefensive,
    Escalates,
    // Success
    CelebratesWithOthers,
    SharesCredit,
    SetsNewGoals,
    EnjoysQuietly,
    BecomesComplacent,
    BecomesOverconfident,
    DismissesOthers,
    // Uncertainty
    EmbracesAmbiguity,
    AsksQuestions,
    SeeksData,
    WaitsForClarity,
    OverPlans,
    BecomesParalyzed,
    DeflectsResponsibility,
    // Recognition
    AppreciatesQuietly,
    AppreciatesPraise,
    SharesAchievement,
    SeeksMore,
    BecomesJealous,
    DemandsAttention,
    UnderminesOthers,
    // Threatened
    SeeksUnderstanding,
    SeeksAllies,
    StandsGround,
    BecomesCautious,
    DeflectsBlame,
    Counterattacks,
    BecomesParanoid,
    // Change
    EmbracesChange,
    PlansAhead,
    AdaptsQuickly,
    ResistsChange,
    NeedsReassurance,
    BecomesDisoriented,
    Sabotages,
    // Feedback
    SeeksFeedback,
    AsksForDetails,
    Reflects,
    AcceptsResignedly,
    RejectsFeedback,
    IgnoresCompletely,
    // Injustice
    SeeksRestoration,
    ProtestsConstructively,
    ProtestsFirmly,
    SeeksClarity,
    WithdrawsFromInjustice,
    ExploitsOpportunistically,
    BecomesBitter,
}

impl BehaviorResponse {
    pub fn serde_name(self) -> &'static str {
        match self {
            Self::RemainsCalm => "remains_calm",
            Self::SeeksSupport => "seeks_support",
            Self::StaysFocused => "stays_focused",
            Self::BecomesQuiet => "becomes_quiet",
            Self::BecomesIrritable => "becomes_irritable",
            Self::Overwhelmed => "overwhelmed",
            Self::Panics => "panics",
            Self::FacilitatesResolution => "facilitates_resolution",
            Self::CommunicatesOpenly => "communicates_openly",
            Self::SeeksCompromise => "seeks_compromise",
            Self::StaysSilent => "stays_silent",
            Self::BecomesPassiveAggressive => "becomes_passive_aggressive",
            Self::BecomesDefensive => "becomes_defensive",
            Self::Escalates => "escalates",
            Self::CelebratesWithOthers => "celebrates_with_others",
            Self::SharesCredit => "shares_credit",
            Self::SetsNewGoals => "sets_new_goals",
            Self::EnjoysQuietly => "enjoys_quietly",
            Self::BecomesComplacent => "becomes_complacent",
            Self::BecomesOverconfident => "becomes_overconfident",
            Self::DismissesOthers => "dismisses_others",
            Self::EmbracesAmbiguity => "embraces_ambiguity",
            Self::AsksQuestions => "asks_questions",
            Self::SeeksData => "seeks_data",
            Self::WaitsForClarity => "waits_for_clarity",
            Self::OverPlans => "over_plans",
            Self::BecomesParalyzed => "becomes_paralyzed",
            Self::DeflectsResponsibility => "deflects_responsibility",
            Self::AppreciatesQuietly => "appreciates_quietly",
            Self::AppreciatesPraise => "appreciates_praise",
            Self::SharesAchievement => "shares_achievement",
            Self::SeeksMore => "seeks_more",
            Self::BecomesJealous => "becomes_jealous",
            Self::DemandsAttention => "demands_attention",
            Self::UnderminesOthers => "undermines_others",
            Self::SeeksUnderstanding => "seeks_understanding",
            Self::SeeksAllies => "seeks_allies",
            Self::StandsGround => "stands_ground",
            Self::BecomesCautious => "becomes_cautious",
            Self::DeflectsBlame => "deflects_blame",
            Self::Counterattacks => "counterattacks",
            Self::BecomesParanoid => "becomes_paranoid",
            Self::EmbracesChange => "embraces_change",
            Self::PlansAhead => "plans_ahead",
            Self::AdaptsQuickly => "adapts_quickly",
            Self::ResistsChange => "resists_change",
            Self::NeedsReassurance => "needs_reassurance",
            Self::BecomesDisoriented => "becomes_disoriented",
            Self::Sabotages => "sabotages",
            Self::SeeksFeedback => "seeks_feedback",
            Self::AsksForDetails => "asks_for_details",
            Self::Reflects => "reflects",
            Self::AcceptsResignedly => "accepts_resignedly",
            Self::RejectsFeedback => "rejects_feedback",
            Self::IgnoresCompletely => "ignores_completely",
            Self::SeeksRestoration => "seeks_restoration",
            Self::ProtestsConstructively => "protests_constructively",
            Self::ProtestsFirmly => "protests_firmly",
            Self::SeeksClarity => "seeks_clarity",
            Self::WithdrawsFromInjustice => "withdraws_from_injustice",
            Self::ExploitsOpportunistically => "exploits_opportunistically",
            Self::BecomesBitter => "becomes_bitter",
        }
    }

    pub fn score(self) -> f64 {
        match self {
            // Tier 1 — Proactive (+0.03)
            Self::RemainsCalm
            | Self::FacilitatesResolution
            | Self::CelebratesWithOthers
            | Self::EmbracesAmbiguity
            | Self::AppreciatesQuietly
            | Self::SeeksUnderstanding
            | Self::EmbracesChange
            | Self::SeeksFeedback
            | Self::SeeksRestoration => 0.03,
            // Tier 2 — Constructive (+0.02)
            Self::SeeksSupport
            | Self::CommunicatesOpenly
            | Self::SharesCredit
            | Self::AsksQuestions
            | Self::AppreciatesPraise
            | Self::SeeksAllies
            | Self::PlansAhead
            | Self::AsksForDetails
            | Self::ProtestsConstructively => 0.02,
            // Tier 3 — Adaptive (+0.01)
            Self::StaysFocused
            | Self::SeeksCompromise
            | Self::SetsNewGoals
            | Self::SeeksData
            | Self::SharesAchievement
            | Self::StandsGround
            | Self::AdaptsQuickly
            | Self::Reflects
            | Self::ProtestsFirmly => 0.01,
            // Tier 4 — Neutral (0.00)
            Self::BecomesQuiet
            | Self::StaysSilent
            | Self::EnjoysQuietly
            | Self::WaitsForClarity
            | Self::SeeksMore
            | Self::BecomesCautious
            | Self::ResistsChange
            | Self::AcceptsResignedly
            | Self::SeeksClarity => 0.0,
            // Tier 5 — Mild friction (−0.01)
            Self::BecomesIrritable
            | Self::BecomesPassiveAggressive
            | Self::BecomesComplacent
            | Self::OverPlans
            | Self::BecomesJealous
            | Self::NeedsReassurance
            | Self::RejectsFeedback
            | Self::WithdrawsFromInjustice => -0.01,
            // Tier 6 — Bad (−0.02)
            Self::Overwhelmed
            | Self::BecomesDefensive
            | Self::BecomesOverconfident
            | Self::BecomesParalyzed
            | Self::DemandsAttention
            | Self::DeflectsBlame
            | Self::Counterattacks
            | Self::BecomesDisoriented
            | Self::ExploitsOpportunistically => -0.02,
            // Tier 7 — Maladaptive (−0.03)
            Self::Panics
            | Self::Escalates
            | Self::DismissesOthers
            | Self::DeflectsResponsibility
            | Self::UnderminesOthers
            | Self::BecomesParanoid
            | Self::Sabotages
            | Self::IgnoresCompletely
            | Self::BecomesBitter => -0.03,
        }
    }

    pub fn options_for(t: BehaviorTrigger) -> &'static [Self] {
        match t {
            BehaviorTrigger::Stress => &[
                Self::RemainsCalm,
                Self::SeeksSupport,
                Self::StaysFocused,
                Self::BecomesQuiet,
                Self::BecomesIrritable,
                Self::Overwhelmed,
                Self::Panics,
            ],
            BehaviorTrigger::Conflict => &[
                Self::FacilitatesResolution,
                Self::CommunicatesOpenly,
                Self::SeeksCompromise,
                Self::StaysSilent,
                Self::BecomesPassiveAggressive,
                Self::BecomesDefensive,
                Self::Escalates,
            ],
            BehaviorTrigger::Success => &[
                Self::CelebratesWithOthers,
                Self::SharesCredit,
                Self::SetsNewGoals,
                Self::EnjoysQuietly,
                Self::BecomesComplacent,
                Self::BecomesOverconfident,
                Self::DismissesOthers,
            ],
            BehaviorTrigger::Uncertainty => &[
                Self::EmbracesAmbiguity,
                Self::AsksQuestions,
                Self::SeeksData,
                Self::WaitsForClarity,
                Self::OverPlans,
                Self::BecomesParalyzed,
                Self::DeflectsResponsibility,
            ],
            BehaviorTrigger::Recognition => &[
                Self::AppreciatesQuietly,
                Self::AppreciatesPraise,
                Self::SharesAchievement,
                Self::SeeksMore,
                Self::BecomesJealous,
                Self::DemandsAttention,
                Self::UnderminesOthers,
            ],
            BehaviorTrigger::Threatened => &[
                Self::SeeksUnderstanding,
                Self::SeeksAllies,
                Self::StandsGround,
                Self::BecomesCautious,
                Self::DeflectsBlame,
                Self::Counterattacks,
                Self::BecomesParanoid,
            ],
            BehaviorTrigger::Change => &[
                Self::EmbracesChange,
                Self::PlansAhead,
                Self::AdaptsQuickly,
                Self::ResistsChange,
                Self::NeedsReassurance,
                Self::BecomesDisoriented,
                Self::Sabotages,
            ],
            BehaviorTrigger::Feedback => &[
                Self::SeeksFeedback,
                Self::AsksForDetails,
                Self::Reflects,
                Self::AcceptsResignedly,
                Self::RejectsFeedback,
                Self::BecomesDefensive,
                Self::IgnoresCompletely,
            ],
            BehaviorTrigger::Injustice => &[
                Self::SeeksRestoration,
                Self::ProtestsConstructively,
                Self::ProtestsFirmly,
                Self::SeeksClarity,
                Self::WithdrawsFromInjustice,
                Self::ExploitsOpportunistically,
                Self::BecomesBitter,
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehavioralPattern {
    pub trigger: BehaviorTrigger,
    #[serde(deserialize_with = "deserialize_behavior_response")]
    pub predicted_behavior: BehaviorResponse,
    #[serde(default)]
    pub notes: String,
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
    #[serde(default)]
    pub text: String,
    /// Valence of the interaction on -3..+3. `None` for legacy free-text entries.
    #[serde(
        default,
        deserialize_with = "clamp_i8_opt_neg3_3",
        skip_serializing_if = "Option::is_none"
    )]
    pub valence: Option<i8>,
    /// Trigger context of the interaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<BehaviorTrigger>,
    /// The other person this interaction is about. `None` = self-observation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FacetKind {
    /// The anchor profile: the person's primary context (their most-known
    /// persona). Always the fallback facet.
    #[default]
    Base,
    /// The arena where work-type relationships are scored.
    Work,
    /// The arena where the person is active online (social media, messaging...).
    Online,
}

impl FacetKind {
    pub const ALL: [Self; 3] = [Self::Base, Self::Work, Self::Online];

    pub fn label(&self, lang: crate::i18n::Lang) -> &'static str {
        match (self, lang) {
            (Self::Base, crate::i18n::Lang::Fr) => "Vie privée",
            (Self::Base, crate::i18n::Lang::En) => "Personal life",
            (Self::Work, crate::i18n::Lang::Fr) => "Au travail",
            (Self::Work, crate::i18n::Lang::En) => "At work",
            (Self::Online, crate::i18n::Lang::Fr) => "En ligne",
            (Self::Online, crate::i18n::Lang::En) => "Online",
        }
    }
}

/// Optional arena persona: the mask a person wears in a given arena (work,
/// online...). Each field is a delta over the anchor profile (which represents
/// the person's primary context, see `Person::primary_facet`). `None` on a
/// bucket means "same as the anchor".
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PersonaMask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocean: Option<OceanScores>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rep_scores: Option<RepScores>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motivations: Option<Vec<Motivation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biases: Option<Vec<Bias>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavioral_patterns: Option<Vec<BehavioralPattern>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<PersonalStyle>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Value>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "clamp_u8_opt_1_10"
    )]
    pub resilience: Option<u8>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "clamp_u8_opt_1_10"
    )]
    pub risk_appetite: Option<u8>,
}

/// Effective view of a person's behavior channels under a given facet. When a
/// work-persona bucket is unset, it inherits from base.
#[derive(Debug, Clone, PartialEq)]
pub struct FacetView {
    pub kind: FacetKind,
    pub ocean: OceanScores,
    pub rep_scores: RepScores,
    pub motivations: Vec<Motivation>,
    pub biases: Vec<Bias>,
    pub behavioral_patterns: Vec<BehavioralPattern>,
    pub styles: Vec<PersonalStyle>,
    pub values: Vec<Value>,
    pub resilience: Option<u8>,
    pub risk_appetite: Option<u8>,
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

    pub fn label(&self, lang: crate::i18n::Lang) -> &'static str {
        match lang {
            crate::i18n::Lang::Fr => match self {
                Self::WorksWith => "Travaille avec",
                Self::Manages => "Dirige",
                Self::ReportsTo => "Rend compte à",
                Self::Friends => "Amis",
                Self::Family => "Famille",
                Self::Partner => "Partenaire",
                Self::Mentors => "Mentore",
                Self::Collaborates => "Collabore",
            },
            crate::i18n::Lang::En => match self {
                Self::WorksWith => "Works With",
                Self::Manages => "Manages",
                Self::ReportsTo => "Reports To",
                Self::Friends => "Friends",
                Self::Family => "Family",
                Self::Partner => "Partner",
                Self::Mentors => "Mentors",
                Self::Collaborates => "Collaborates",
            },
        }
    }

    pub fn facet(&self) -> FacetKind {
        match self {
            Self::WorksWith
            | Self::Manages
            | Self::ReportsTo
            | Self::Mentors
            | Self::Collaborates => FacetKind::Work,
            Self::Friends | Self::Family | Self::Partner => FacetKind::Base,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub r#type: RelationType,
    #[serde(default = "default_strength", deserialize_with = "clamp_u8_1_10")]
    pub strength: u8,
    pub notes: String,
    pub created_at: i64,
}

fn default_strength() -> u8 {
    5
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
    #[serde(default)]
    pub tags: Vec<Tag>,
    pub notes: String,
    pub motivations: Vec<Motivation>,
    pub biases: Vec<Bias>,
    #[serde(default)]
    pub rep_scores: RepScores,
    pub behavioral_patterns: Vec<BehavioralPattern>,
    #[serde(default)]
    pub styles: Vec<PersonalStyle>,
    #[serde(default)]
    pub values: Vec<Value>,
    #[serde(default)]
    pub persona: Option<PersonaMask>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_persona: Option<PersonaMask>,
    /// The mask the person wears in their personal life. Only meaningful when
    /// `primary_facet` is a non-base context (the inline anchor is then work
    /// or online); present only then.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_persona: Option<PersonaMask>,
    /// The context the inline anchor fields represent (the person's primary /
    /// most-known persona, not necessarily their private life). The
    /// `PersonaMask` deltas above are always relative to this anchor.
    #[serde(default)]
    pub primary_facet: FacetKind,
    pub ocean: OceanScores,
    #[serde(default, deserialize_with = "clamp_u8_opt_1_10")]
    pub resilience: Option<u8>,
    #[serde(default, deserialize_with = "clamp_u8_opt_1_10")]
    pub risk_appetite: Option<u8>,
    #[serde(default)]
    pub log: Vec<InteractionEntry>,
    #[serde(default = "default_confidence", deserialize_with = "clamp_u8_1_10")]
    pub confidence: u8,
    pub created_at: i64,
    pub updated_at: i64,
}

fn default_confidence() -> u8 {
    5
}

/// Scalar fallback used when capturing a context's view as a full persona
/// mask (mirrors the app's "copy from base profile" seeding).
const CAPTURE_DEFAULT: u8 = 5;

impl Person {
    pub fn top_motivation(&self) -> Option<&Motivation> {
        self.motivations.iter().max_by_key(|m| m.intensity)
    }
    pub fn top_bias(&self) -> Option<&Bias> {
        self.biases.iter().max_by_key(|b| b.intensity)
    }

    /// Whether `kind` is a defined context for this person: either the anchor
    /// (primary facet — always known) or an explicitly-stored arena mask. A
    /// non-primary arena with no mask is an undefined persona: the person only
    /// exists in their primary context.
    pub fn has_facet(&self, kind: FacetKind) -> bool {
        kind == self.primary_facet || self.mask_for(kind).is_some()
    }

    pub fn facet_view(&self, kind: FacetKind) -> Option<FacetView> {
        if !self.has_facet(kind) {
            return None;
        }
        Some(FacetView {
            kind,
            ocean: self
                .mask_for(kind)
                .and_then(|m| m.ocean.clone())
                .unwrap_or_else(|| self.ocean.clone()),
            rep_scores: self
                .mask_for(kind)
                .and_then(|m| m.rep_scores.clone())
                .unwrap_or_else(|| self.rep_scores.clone()),
            motivations: self
                .mask_for(kind)
                .and_then(|m| m.motivations.clone())
                .unwrap_or_else(|| self.motivations.clone()),
            biases: self
                .mask_for(kind)
                .and_then(|m| m.biases.clone())
                .unwrap_or_else(|| self.biases.clone()),
            behavioral_patterns: self
                .mask_for(kind)
                .and_then(|m| m.behavioral_patterns.clone())
                .unwrap_or_else(|| self.behavioral_patterns.clone()),
            styles: self
                .mask_for(kind)
                .and_then(|m| m.styles.clone())
                .unwrap_or_else(|| self.styles.clone()),
            values: self
                .mask_for(kind)
                .and_then(|m| m.values.clone())
                .unwrap_or_else(|| self.values.clone()),
            resilience: self
                .mask_for(kind)
                .and_then(|m| m.resilience)
                .or(self.resilience),
            risk_appetite: self
                .mask_for(kind)
                .and_then(|m| m.risk_appetite)
                .or(self.risk_appetite),
        })
    }

    /// The arena persona mask for `kind` — `None` for the context the inline
    /// fields anchor (`primary_facet`), since that context is always "base".
    fn mask_for(&self, kind: FacetKind) -> Option<&PersonaMask> {
        if kind == self.primary_facet {
            return None;
        }
        match kind {
            FacetKind::Base => self.private_persona.as_ref(),
            FacetKind::Work => self.persona.as_ref(),
            FacetKind::Online => self.online_persona.as_ref(),
        }
    }

    /// A clone with the behavior channels resolved under `kind`. Persona
    /// buckets that are `None` inherit the anchor channel, so the merged
    /// person is what every downstream computation (profile, insights, flags)
    /// would read for that facet. The clone carries no persona, which keeps
    /// those computations persona-agnostic. `None` when the facet is not a
    /// defined context for this person (no mask and not the anchor).
    pub fn facet_person(&self, kind: FacetKind) -> Option<Person> {
        if !self.has_facet(kind) {
            return None;
        }
        if kind == self.primary_facet {
            return Some(self.clone());
        }
        let v = self.facet_view(kind)?;
        Some(Person {
            persona: None,
            online_persona: None,
            private_persona: None,
            ocean: v.ocean,
            rep_scores: v.rep_scores,
            motivations: v.motivations,
            biases: v.biases,
            behavioral_patterns: v.behavioral_patterns,
            styles: v.styles,
            values: v.values,
            resilience: v.resilience,
            risk_appetite: v.risk_appetite,
            ..self.clone()
        })
    }

    /// Full view of a context during a primary swap. An undefined context
    /// materializes its anchor view: the swap itself is the act that defines
    /// the target as the new anchor. The anchor is always defined.
    fn capture_view(&self, kind: FacetKind) -> FacetView {
        self.facet_view(kind).unwrap_or_else(|| {
            self.facet_view(self.primary_facet)
                .expect("anchor is defined")
        })
    }

    /// Re-point the inline anchor fields at another context. The new anchor
    /// becomes the target's materialized view; every other context is kept as
    /// a full-capture mask of its own pre-swap view, so no context's data
    /// changes. No-op when `new` is already the primary facet.
    pub fn set_primary_facet(&mut self, new: FacetKind) {
        if new == self.primary_facet {
            return;
        }
        let before = self.clone();
        for kind in FacetKind::ALL {
            let view = before.capture_view(kind);
            if kind == new {
                self.apply_inline_view(&view);
            } else {
                let mask = PersonaMask {
                    ocean: Some(view.ocean.clone()),
                    rep_scores: Some(view.rep_scores.clone()),
                    motivations: Some(view.motivations.clone()),
                    biases: Some(view.biases.clone()),
                    behavioral_patterns: Some(view.behavioral_patterns.clone()),
                    styles: Some(view.styles.clone()),
                    values: Some(view.values.clone()),
                    resilience: Some(view.resilience.unwrap_or(CAPTURE_DEFAULT)),
                    risk_appetite: Some(view.risk_appetite.unwrap_or(CAPTURE_DEFAULT)),
                };
                self.set_persona_slot(kind, Some(mask));
            }
        }
        self.primary_facet = new;
    }

    fn apply_inline_view(&mut self, v: &FacetView) {
        self.ocean = v.ocean.clone();
        self.rep_scores = v.rep_scores.clone();
        self.motivations = v.motivations.clone();
        self.biases = v.biases.clone();
        self.behavioral_patterns = v.behavioral_patterns.clone();
        self.styles = v.styles.clone();
        self.values = v.values.clone();
        self.resilience = v.resilience;
        self.risk_appetite = v.risk_appetite;
    }

    /// Assign a persona mask to a context slot regardless of which context is
    /// primary (the anchor context simply stores it as a mask).
    pub fn set_persona_slot(&mut self, kind: FacetKind, mask: Option<PersonaMask>) {
        match kind {
            FacetKind::Base => self.private_persona = mask,
            FacetKind::Work => self.persona = mask,
            FacetKind::Online => self.online_persona = mask,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StyleCategory {
    Communication,
    ConflictResolution,
    DecisionMaking,
    Leadership,
    TimeOrientation,
    MoralFramework,
    InterpersonalConduct,
    TrustStyle,
}

impl StyleCategory {
    pub const ALL: [Self; 8] = [
        Self::Communication,
        Self::ConflictResolution,
        Self::DecisionMaking,
        Self::Leadership,
        Self::TimeOrientation,
        Self::MoralFramework,
        Self::InterpersonalConduct,
        Self::TrustStyle,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StyleType {
    // Communication
    DirectCommunicator,
    DiplomaticCommunicator,
    ReservedCommunicator,
    ExpressiveCommunicator,
    // Conflict resolution
    Competing,
    Collaborating,
    Compromising,
    Avoiding,
    Accommodating,
    // Decision making
    Analytical,
    Intuitive,
    Participatory,
    Autocratic,
    ConsensusDriven,
    // Leadership
    Visionary,
    Servant,
    Transactional,
    Transformational,
    Bureaucratic,
    // Time orientation
    PastOriented,
    PresentOriented,
    FutureOriented,
    // Moral framework
    RuleBased,
    OutcomeBased,
    VirtueBased,
    Relativist,
    // Interpersonal conduct
    Opportunistic,
    Intrusive,
    Manipulative,
    PassiveAggressive,
    Controlling,
    Detached,
    Respectful,
    Empathetic,
    Supportive,
    Nurturing,
    // Trust style
    ExtendsTrustFreely,
    EarnsTrustGradually,
    VerifiesTrust,
    Guarded,
    RepairsTrustActively,
}

impl fmt::Display for StyleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl StyleType {
    pub const ALL: [Self; 41] = [
        Self::DirectCommunicator,
        Self::DiplomaticCommunicator,
        Self::ReservedCommunicator,
        Self::ExpressiveCommunicator,
        Self::Competing,
        Self::Collaborating,
        Self::Compromising,
        Self::Avoiding,
        Self::Accommodating,
        Self::Analytical,
        Self::Intuitive,
        Self::Participatory,
        Self::Autocratic,
        Self::ConsensusDriven,
        Self::Visionary,
        Self::Servant,
        Self::Transactional,
        Self::Transformational,
        Self::Bureaucratic,
        Self::PastOriented,
        Self::PresentOriented,
        Self::FutureOriented,
        Self::RuleBased,
        Self::OutcomeBased,
        Self::VirtueBased,
        Self::Relativist,
        Self::Opportunistic,
        Self::Intrusive,
        Self::Manipulative,
        Self::PassiveAggressive,
        Self::Controlling,
        Self::Detached,
        Self::Respectful,
        Self::Empathetic,
        Self::Supportive,
        Self::Nurturing,
        Self::ExtendsTrustFreely,
        Self::EarnsTrustGradually,
        Self::VerifiesTrust,
        Self::Guarded,
        Self::RepairsTrustActively,
    ];

    pub fn category(&self) -> StyleCategory {
        use StyleCategory::*;
        match self {
            Self::DirectCommunicator
            | Self::DiplomaticCommunicator
            | Self::ReservedCommunicator
            | Self::ExpressiveCommunicator => Communication,
            Self::Competing
            | Self::Collaborating
            | Self::Compromising
            | Self::Avoiding
            | Self::Accommodating => ConflictResolution,
            Self::Analytical
            | Self::Intuitive
            | Self::Participatory
            | Self::Autocratic
            | Self::ConsensusDriven => DecisionMaking,
            Self::Visionary
            | Self::Servant
            | Self::Transactional
            | Self::Transformational
            | Self::Bureaucratic => Leadership,
            Self::PastOriented | Self::PresentOriented | Self::FutureOriented => TimeOrientation,
            Self::RuleBased | Self::OutcomeBased | Self::VirtueBased | Self::Relativist => {
                MoralFramework
            }
            Self::Opportunistic
            | Self::Intrusive
            | Self::Manipulative
            | Self::PassiveAggressive
            | Self::Controlling
            | Self::Detached
            | Self::Respectful
            | Self::Empathetic
            | Self::Supportive
            | Self::Nurturing => InterpersonalConduct,
            Self::ExtendsTrustFreely
            | Self::EarnsTrustGradually
            | Self::VerifiesTrust
            | Self::Guarded
            | Self::RepairsTrustActively => TrustStyle,
        }
    }

    pub fn options_for(cat: StyleCategory) -> &'static [Self] {
        match cat {
            StyleCategory::Communication => &[
                Self::DirectCommunicator,
                Self::DiplomaticCommunicator,
                Self::ReservedCommunicator,
                Self::ExpressiveCommunicator,
            ],
            StyleCategory::ConflictResolution => &[
                Self::Competing,
                Self::Collaborating,
                Self::Compromising,
                Self::Avoiding,
                Self::Accommodating,
            ],
            StyleCategory::DecisionMaking => &[
                Self::Analytical,
                Self::Intuitive,
                Self::Participatory,
                Self::Autocratic,
                Self::ConsensusDriven,
            ],
            StyleCategory::Leadership => &[
                Self::Visionary,
                Self::Servant,
                Self::Transactional,
                Self::Transformational,
                Self::Bureaucratic,
            ],
            StyleCategory::TimeOrientation => &[
                Self::PastOriented,
                Self::PresentOriented,
                Self::FutureOriented,
            ],
            StyleCategory::MoralFramework => &[
                Self::RuleBased,
                Self::OutcomeBased,
                Self::VirtueBased,
                Self::Relativist,
            ],
            StyleCategory::InterpersonalConduct => &[
                Self::Opportunistic,
                Self::Intrusive,
                Self::Manipulative,
                Self::PassiveAggressive,
                Self::Controlling,
                Self::Detached,
                Self::Respectful,
                Self::Empathetic,
                Self::Supportive,
                Self::Nurturing,
            ],
            StyleCategory::TrustStyle => &[
                Self::ExtendsTrustFreely,
                Self::EarnsTrustGradually,
                Self::VerifiesTrust,
                Self::Guarded,
                Self::RepairsTrustActively,
            ],
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self.category() {
            StyleCategory::Communication => "💬",
            StyleCategory::ConflictResolution => "🤝",
            StyleCategory::DecisionMaking => "🧠",
            StyleCategory::Leadership => "👥",
            StyleCategory::TimeOrientation => "⏰",
            StyleCategory::MoralFramework => "📜",
            StyleCategory::InterpersonalConduct => "🫂",
            StyleCategory::TrustStyle => "🔗",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalStyle {
    pub r#type: StyleType,
    #[serde(deserialize_with = "clamp_u8_1_10")]
    pub intensity: u8,
    pub notes: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub member_ids: Vec<String>,
    pub created_at: i64,
}

pub const AVATAR_EMOJIS: &[&str] = &[
    "🧑", "👩", "👨", "🧠", "🎯", "💼", "🦁", "🦊", "🐺", "🌟", "🔥", "💎", "🎸", "🧬", "🌊", "🏔️",
    // Chinese Zodiac (十二生肖)
    "🐀", "🐂", "🐅", "🐇", "🐉", "🐍", "🐎", "🐐", "🐒", "🐓", "🐕", "🐖", // Animals
    "🐶", "🐱", "🐻", "🐼", "🐸", "🦄", "🐧", "🦉", "🐨", "🦋", "🐙", "🦥", "🦜",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behavior_pattern_bad_predicted_behavior_error_uses_expecting() {
        // A wrong-typed `predicted_behavior` reaches the custom visitor's
        // `expecting`, so its message must be written (models.rs:22). A
        // mutated `expecting` that returns `Ok(Default::default())` produces
        // no message and this assertion fails, killing the mutant.
        let err = serde_json::from_str::<BehavioralPattern>(
            r#"{"trigger":"SUCCESS","predicted_behavior":42}"#,
        )
        .unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("a behavior response variant"), "got: {msg}");
    }

    #[test]
    fn facet_kind_label_covers_all_modes() {
        assert_eq!(FacetKind::Base.label(crate::i18n::Lang::Fr), "Vie privée");
        assert_eq!(
            FacetKind::Base.label(crate::i18n::Lang::En),
            "Personal life"
        );
        assert_eq!(FacetKind::Work.label(crate::i18n::Lang::Fr), "Au travail");
        assert_eq!(FacetKind::Work.label(crate::i18n::Lang::En), "At work");
        assert_eq!(FacetKind::Online.label(crate::i18n::Lang::Fr), "En ligne");
        assert_eq!(FacetKind::Online.label(crate::i18n::Lang::En), "Online");
    }

    #[test]
    fn primary_facet_defaults_to_base_on_legacy_backup() {
        let raw = r#"{"id":"p","name":"n","role":"","context":"","avatar_emoji":"a",
            "notes":"","motivations":[],"biases":[],"behavioral_patterns":[],
            "ocean":{},"created_at":0,"updated_at":0}"#;
        let p: Person = serde_json::from_str(raw).unwrap();
        assert_eq!(p.primary_facet, FacetKind::Base);
    }

    #[test]
    fn primary_facet_survives_round_trip() {
        let raw = r#"{"id":"p","name":"n","role":"","context":"","avatar_emoji":"a",
            "notes":"","motivations":[],"biases":[],"behavioral_patterns":[],
            "ocean":{},"created_at":0,"updated_at":0,"primary_facet":"Work"}"#;
        let p: Person = serde_json::from_str(raw).unwrap();
        assert_eq!(p.primary_facet, FacetKind::Work);
        let back: Person = serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        assert_eq!(back.primary_facet, FacetKind::Work);
    }

    #[test]
    fn facet_person_merges_persona_buckets() {
        let base = Person {
            id: "p1".into(),
            primary_facet: FacetKind::Base,
            name: "Base".into(),
            role: "r".into(),
            context: "c".into(),
            avatar_emoji: "🧑".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![Motivation {
                r#type: MotivationType::Power,
                intensity: 8,
                notes: "base".into(),
            }],
            biases: vec![Bias {
                r#type: BiasType::Anchoring,
                intensity: 7,
                evidence: "base".into(),
            }],
            rep_scores: RepScores {
                hardworker_lazy: Some(5),
                ..RepScores::default()
            },
            behavioral_patterns: vec![BehavioralPattern {
                trigger: BehaviorTrigger::Change,
                predicted_behavior: BehaviorResponse::StaysFocused,
                notes: String::new(),
            }],
            styles: vec![PersonalStyle {
                r#type: StyleType::DirectCommunicator,
                intensity: 6,
                notes: String::new(),
            }],
            values: vec![Value {
                r#type: ValueType::Health,
                intensity: 5,
                priority: 3,
                notes: String::new(),
            }],
            persona: Some(PersonaMask {
                ocean: Some(OceanScores {
                    openness: Some(3),
                    conscientiousness: Some(9),
                    extraversion: Some(2),
                    agreeableness: Some(8),
                    neuroticism: Some(6),
                }),
                rep_scores: Some(RepScores {
                    hardworker_lazy: Some(1),
                    ..RepScores::default()
                }),
                motivations: Some(vec![Motivation {
                    r#type: MotivationType::Security,
                    intensity: 9,
                    notes: "work".into(),
                }]),
                biases: Some(vec![Bias {
                    r#type: BiasType::Confirmation,
                    intensity: 9,
                    evidence: "work".into(),
                }]),
                behavioral_patterns: Some(vec![BehavioralPattern {
                    trigger: BehaviorTrigger::Conflict,
                    predicted_behavior: BehaviorResponse::Escalates,
                    notes: String::new(),
                }]),
                styles: Some(vec![PersonalStyle {
                    r#type: StyleType::ExpressiveCommunicator,
                    intensity: 9,
                    notes: String::new(),
                }]),
                values: Some(vec![Value {
                    r#type: ValueType::Career,
                    intensity: 7,
                    priority: 8,
                    notes: String::new(),
                }]),
                resilience: Some(4),
                risk_appetite: Some(6),
            }),
            online_persona: None,
            private_persona: None,
            ocean: OceanScores {
                openness: Some(7),
                conscientiousness: Some(6),
                extraversion: Some(8),
                agreeableness: Some(5),
                neuroticism: Some(4),
            },
            resilience: None,
            risk_appetite: None,
            log: vec![],
            confidence: 5,
            created_at: 1,
            updated_at: 2,
        };

        let merged = base.facet_person(FacetKind::Work).unwrap();
        assert_eq!(merged.id, "p1", "identity must be kept");
        assert_eq!(merged.name, "Base");
        assert_eq!(
            merged.persona, None,
            "merged clone must be persona-agnostic"
        );
        assert_eq!(
            merged.online_persona, None,
            "merged clone must drop the online mask too"
        );
        assert_eq!(
            merged.ocean,
            base.persona.as_ref().unwrap().ocean.clone().unwrap()
        );
        assert_eq!(
            merged.rep_scores,
            base.persona.as_ref().unwrap().rep_scores.clone().unwrap()
        );
        assert_eq!(
            merged.motivations,
            base.persona.as_ref().unwrap().motivations.clone().unwrap()
        );
        assert_eq!(
            merged.biases,
            base.persona.as_ref().unwrap().biases.clone().unwrap()
        );
        assert_eq!(
            merged.behavioral_patterns,
            base.persona
                .as_ref()
                .unwrap()
                .behavioral_patterns
                .clone()
                .unwrap()
        );
        assert_eq!(
            merged.styles,
            base.persona.as_ref().unwrap().styles.clone().unwrap()
        );
        assert_eq!(
            merged.values,
            base.persona.as_ref().unwrap().values.clone().unwrap()
        );
        assert_eq!(merged.resilience, Some(4));
        assert_eq!(merged.risk_appetite, Some(6));
    }

    #[test]
    fn facet_person_base_ignores_persona() {
        let base = Person {
            id: "p2".into(),
            primary_facet: FacetKind::Base,
            name: "Base".into(),
            role: "r".into(),
            context: "c".into(),
            avatar_emoji: "🧑".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![Motivation {
                r#type: MotivationType::Power,
                intensity: 8,
                notes: "base".into(),
            }],
            biases: vec![],
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            persona: Some(PersonaMask {
                ocean: Some(OceanScores {
                    openness: Some(3),
                    ..OceanScores::default()
                }),
                ..PersonaMask::default()
            }),
            private_persona: None,
            online_persona: Some(PersonaMask {
                ocean: Some(OceanScores {
                    openness: Some(4),
                    ..OceanScores::default()
                }),
                ..PersonaMask::default()
            }),
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
            log: vec![],
            confidence: 5,
            created_at: 1,
            updated_at: 2,
        };

        let merged = base.facet_person(FacetKind::Base).unwrap();
        assert_eq!(merged, base, "base facet must return an identical clone");
        assert_eq!(merged.persona, base.persona.clone());
        assert_eq!(merged.online_persona, base.online_persona.clone());
    }

    #[test]
    fn facet_person_without_work_mask_is_undefined() {
        // No work persona, but an online persona set. The work facet is not a
        // defined context for this person: the merge guard must return `None`
        // instead of resolving the anchor (which would fabricate a work
        // persona from the base profile). This is the case that catches an
        // `||` sneaking into the `&&` in the `has_facet` guard (an undefined
        // arena must not fall back to the clone).
        let base = Person {
            id: "p3".into(),
            primary_facet: FacetKind::Base,
            name: "No Work Mask".into(),
            role: "r".into(),
            context: "c".into(),
            avatar_emoji: "🧑".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            persona: None,
            private_persona: None,
            online_persona: Some(PersonaMask {
                ocean: Some(OceanScores {
                    openness: Some(4),
                    ..OceanScores::default()
                }),
                ..PersonaMask::default()
            }),
            ocean: OceanScores {
                openness: Some(7),
                ..OceanScores::default()
            },
            resilience: None,
            risk_appetite: None,
            log: vec![],
            confidence: 5,
            created_at: 1,
            updated_at: 2,
        };

        assert_eq!(
            base.facet_person(FacetKind::Work),
            None,
            "undefined work facet"
        );
        assert!(
            !base.has_facet(FacetKind::Work),
            "no work persona means no work facet"
        );
        let anchor = base.facet_person(FacetKind::Base).unwrap();
        assert_eq!(
            anchor, base,
            "anchor returns the raw clone with masks intact"
        );
        assert_eq!(anchor.online_persona, base.online_persona);
    }

    #[test]
    fn facet_person_online_merges_online_persona() {
        let base = Person {
            id: "p3".into(),
            primary_facet: FacetKind::Base,
            name: "Base".into(),
            role: "r".into(),
            context: "c".into(),
            avatar_emoji: "🧑".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![Motivation {
                r#type: MotivationType::Power,
                intensity: 8,
                notes: "base".into(),
            }],
            biases: vec![],
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            persona: Some(PersonaMask {
                ocean: Some(OceanScores {
                    openness: Some(2),
                    ..OceanScores::default()
                }),
                ..PersonaMask::default()
            }),
            private_persona: None,
            online_persona: Some(PersonaMask {
                ocean: Some(OceanScores {
                    openness: Some(9),
                    ..OceanScores::default()
                }),
                ..PersonaMask::default()
            }),
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
            log: vec![],
            confidence: 5,
            created_at: 1,
            updated_at: 2,
        };

        let merged = base.facet_person(FacetKind::Online).unwrap();
        assert_eq!(
            merged.persona, None,
            "merged clone must be persona-agnostic"
        );
        assert_eq!(
            merged.online_persona, None,
            "merged clone must drop the online mask"
        );
        assert_eq!(
            merged.ocean,
            base.online_persona.as_ref().unwrap().ocean.clone().unwrap()
        );
        assert_eq!(
            base.facet_person(FacetKind::Work).unwrap().ocean,
            base.persona.as_ref().unwrap().ocean.clone().unwrap(),
            "work facet is independent of the online mask"
        );
    }

    #[test]
    fn private_persona_is_omitted_when_none_and_round_trips() {
        let p = Person {
            id: "p".into(),
            primary_facet: FacetKind::Work,
            ..blank_person()
        };
        let raw = serde_json::to_string(&p).unwrap();
        assert!(!raw.contains("private_persona"));
        let back: Person = serde_json::from_str(&raw).unwrap();
        assert_eq!(back.primary_facet, FacetKind::Work);
        assert_eq!(back.private_persona, None);
        assert_eq!(back, p, "omitted private mask round-trips as None");
    }

    #[test]
    fn mask_for_gates_by_primary_facet() {
        let mut p = blank_person();
        p.primary_facet = FacetKind::Work;
        p.persona = Some(PersonaMask {
            ocean: Some(OceanScores {
                openness: Some(2),
                ..OceanScores::default()
            }),
            ..PersonaMask::default()
        });
        p.private_persona = Some(PersonaMask {
            ocean: Some(OceanScores {
                openness: Some(9),
                extraversion: Some(8),
                ..OceanScores::default()
            }),
            ..PersonaMask::default()
        });
        // The anchor context (Work) has no mask: it IS the inline fields.
        assert_eq!(p.mask_for(FacetKind::Work), None);
        // Personal life is the new mask slot under a work-primary person.
        assert_eq!(
            p.facet_view(FacetKind::Base).unwrap().ocean.openness,
            Some(9),
            "base facet reads the private persona mask"
        );
        assert_eq!(
            p.facet_view(FacetKind::Work).unwrap().ocean.openness,
            None,
            "work facet reads the anchor inline values"
        );
    }

    #[test]
    fn facet_person_base_merges_private_persona_for_non_base_primary() {
        let mut p = blank_person();
        p.primary_facet = FacetKind::Work;
        p.private_persona = Some(PersonaMask {
            ocean: Some(OceanScores {
                openness: Some(9),
                ..OceanScores::default()
            }),
            ..PersonaMask::default()
        });
        let personal = p.facet_person(FacetKind::Base).unwrap();
        assert_eq!(personal.ocean.openness, Some(9), "merged inline value");
        assert_eq!(personal.persona, None);
        assert_eq!(personal.online_persona, None);
        assert_eq!(
            personal.private_persona, None,
            "merged clone must be persona-agnostic"
        );
        // The anchor facet remains a raw clone carrying its masks (same as the
        // legacy base-facet behavior for base-primary people).
        let anchor = p.facet_person(FacetKind::Work).unwrap();
        assert_eq!(anchor.persona, p.persona);
        assert_eq!(anchor.private_persona, p.private_persona);
    }

    #[test]
    fn set_primary_facet_preserves_every_context_view() {
        let mut p = blank_person();
        p.primary_facet = FacetKind::Base;
        p.ocean = OceanScores {
            openness: Some(7),
            ..OceanScores::default()
        };
        // Concrete scalars so the full-capture normalization (None -> 5) never
        // has anything to normalize: every view round-trips exactly.
        p.resilience = Some(6);
        p.risk_appetite = Some(7);
        p.persona = Some(PersonaMask {
            ocean: Some(OceanScores {
                openness: Some(3),
                ..OceanScores::default()
            }),
            ..PersonaMask::default()
        });
        p.online_persona = Some(PersonaMask {
            ocean: Some(OceanScores {
                extraversion: Some(4),
                ..OceanScores::default()
            }),
            ..PersonaMask::default()
        });
        p.primary_facet = FacetKind::Base;
        let before: Vec<_> = FacetKind::ALL
            .into_iter()
            .map(|k| p.facet_view(k).expect("all contexts defined"))
            .collect();

        p.set_primary_facet(FacetKind::Work);
        p.set_primary_facet(FacetKind::Base); // round-trip back
        let after: Vec<_> = FacetKind::ALL
            .into_iter()
            .map(|k| p.facet_view(k).expect("all contexts defined"))
            .collect();

        assert_eq!(p.primary_facet, FacetKind::Base);
        for (b, a) in before.iter().zip(&after) {
            assert_eq!(b, a, "context view must survive the round-trip");
        }
        assert_eq!(
            p.persona
                .as_ref()
                .and_then(|m| m.ocean.as_ref())
                .and_then(|o| o.openness),
            Some(3),
            "work regained its original data as a mask after the round-trip"
        );
    }

    #[test]
    fn set_primary_facet_is_noop_for_current_primary() {
        let mut p = blank_person();
        p.primary_facet = FacetKind::Online;
        let snapshot = p.clone();
        p.set_primary_facet(FacetKind::Online);
        assert_eq!(p, snapshot);
    }

    // Minimal bare person for new facade tests, so another new field never
    // means touching every struct literal again.
    fn blank_person() -> Person {
        Person {
            id: "p".into(),
            name: "n".into(),
            role: String::new(),
            context: String::new(),
            avatar_emoji: "🧑".into(),
            tags: vec![],
            notes: String::new(),
            motivations: vec![],
            biases: vec![],
            rep_scores: RepScores::default(),
            behavioral_patterns: vec![],
            styles: vec![],
            values: vec![],
            persona: None,
            online_persona: None,
            private_persona: None,
            primary_facet: FacetKind::Base,
            ocean: OceanScores::default(),
            resilience: None,
            risk_appetite: None,
            log: vec![],
            confidence: 5,
            created_at: 0,
            updated_at: 0,
        }
    }
}
