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
            // forward to serde's enum visitor
            BehaviorResponse::deserialize(de::value::StrDeserializer::<E>::new(v))
                .or(Ok(BehaviorResponse::SeeksSupport))
        }
    }
    d.deserialize_str(Brv)
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
    pub const ALL: [Self; 11] = [
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
        Self::Favoritism,
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
    pub text: String,
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

impl Person {
    pub fn top_motivation(&self) -> Option<&Motivation> {
        self.motivations.iter().max_by_key(|m| m.intensity)
    }
    pub fn top_bias(&self) -> Option<&Bias> {
        self.biases.iter().max_by_key(|b| b.intensity)
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

pub const AVATAR_EMOJIS: &[&str] = &[
    "🧑", "👩", "👨", "🧠", "🎯", "💼", "🦁", "🦊", "🐺", "🌟", "🔥", "💎", "🎸", "🧬", "🌊", "🏔️",
    // Chinese Zodiac (十二生肖)
    "🐀", "🐂", "🐅", "🐇", "🐉", "🐍", "🐎", "🐐", "🐒", "🐓", "🐕", "🐖", // Animals
    "🐶", "🐱", "🐻", "🐼", "🐸", "🦄", "🐧", "🦉", "🐨", "🦋", "🐙", "🦥", "🦜",
];
