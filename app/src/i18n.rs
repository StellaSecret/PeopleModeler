#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lang {
    Fr,
    En,
}

fn detect_from_strings(
    stored: Option<&str>,
    navigator: Option<&str>,
    env_lang: Option<&str>,
) -> Lang {
    if let Some(l) = stored {
        return if l == "en" { Lang::En } else { Lang::Fr };
    }
    if let Some(nav) = navigator
        && nav.starts_with("en")
    {
        return Lang::En;
    }
    if let Some(l) = env_lang
        && l.starts_with("en")
    {
        return Lang::En;
    }
    Lang::Fr
}

impl Lang {
    pub fn detect() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let stored = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
                .and_then(|s| s.get_item("pm_lang").ok())
                .flatten();
            let nav = web_sys::window().and_then(|w| w.navigator().language());
            detect_from_strings(stored.as_deref(), nav.as_deref(), None)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let env_lang = std::env::var("LANG").ok();
            detect_from_strings(None, None, env_lang.as_deref())
        }
    }

    pub fn persist(self) {
        let s = match self {
            Lang::En => "en",
            Lang::Fr => "fr",
        };
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
            {
                let _ = storage.set_item("pm_lang", s);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = std::fs::write(
                std::env::current_dir()
                    .unwrap_or_else(|_| ".".into())
                    .join(".pm_lang"),
                s,
            );
        }
    }
}

/// Bridge from the app's UI language to the core engine's language enum used
/// by facet/persona label mapping.
pub fn core_lang(lang: Lang) -> peoplemodeler_core::i18n::Lang {
    match lang {
        Lang::Fr => peoplemodeler_core::i18n::Lang::Fr,
        Lang::En => peoplemodeler_core::i18n::Lang::En,
    }
}

/// Translated string identifier. Variants mirror the snake_case i18n
/// keys; `tr!` and `tr_str` map to them. `Unknown` is the fallback for
/// unrecognized runtime keys (see [`tr_str`] and [`tr_danger_details`]).
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, strum::EnumString, strum::EnumIter, strum::Display,
)]
#[strum(serialize_all = "snake_case")]
pub enum Key {
    AddBtn,
    AriaAddBias,
    AriaAddMotivation,
    AriaAddPattern,
    AriaAddStyle,
    AriaAddValue,
    AriaAvatarPrefix,
    AriaDeleteBias,
    AriaDeleteLogEntry,
    AriaDeleteMotivation,
    AriaDeletePattern,
    AriaDeleteStyle,
    AriaDeleteValue,
    AriaDiscardPrefix,
    AriaEditBias,
    AriaEditMotivation,
    AriaEditPattern,
    AriaEditStyle,
    AriaEditValue,
    AriaMoveBiasDown,
    AriaMoveBiasUp,
    AriaMoveMotivationDown,
    AriaMoveMotivationUp,
    AriaMovePatternDown,
    AriaMovePatternUp,
    AriaMoveStyleDown,
    AriaMoveStyleUp,
    AriaMoveValueDown,
    AriaMoveValueUp,
    AriaSkipToContent,
    AriaUpdateBias,
    AriaUpdateMotivation,
    AriaUpdatePattern,
    AriaUpdateStyle,
    AriaUpdateValue,
    BiasScaleHint,
    BiasUndefinedWarning,
    BiasesTitle,
    BucketInheritsBase,
    BucketOverride,
    CommonAdd,
    CommonBack,
    CommonCancel,
    CommonDelete,
    CommonEdit,
    CommonFinish,
    CommonNext,
    CommonSave,
    CommonSkip,
    CompareAnalysisTitle,
    CompareAsymmetric,
    CompareBalanced,
    CompareBandHint,
    CompareBenefitMore,
    CompareBiasMain,
    CompareBreakdown,
    CompareBtn,
    CompareCatBias,
    CompareCatMotivation,
    CompareCatOcean,
    CompareCatPatterns,
    CompareCatReputation,
    CompareCatStyles,
    CompareCatValues,
    CompareCtxTitle,
    CompareEthics,
    CompareFacetUnavailable,
    CompareFriction,
    CompareOcean,
    CompareRelNone,
    CompareRelStrength,
    CompareRelTitle,
    CompareRiskMitigation,
    CompareStrategy,
    CompareSub,
    CompareSynergies,
    CompareTitle,
    CompareTopMot,
    CompareVs,
    ConfidenceHint,
    ConfidenceLabel,
    ConfirmDelete,
    ConfirmDeleteLog,
    ConfirmDeletePred,
    ConfirmDeleteTeam,
    CtxChange,
    CtxCommunication,
    CtxConflict,
    CtxDecision,
    CtxFeedback,
    CtxGrowth,
    CtxInjustice,
    CtxLeadership,
    CtxRecognition,
    CtxStress,
    CtxSuccess,
    CtxTeam,
    CtxThreatened,
    CtxUncertainty,
    DeleteBtn,
    EditBiases,
    EditBtn,
    EditDiscardSection,
    EditEvidencePlaceholder,
    EditMotivations,
    EditNotesPlaceholder,
    EditPatterns,
    EditPriority,
    EditReputation,
    EditStyles,
    EditUpdateBtn,
    EditValues,
    FlagAffiliationCold,
    FlagAffiliationDistrustful,
    FlagAmbitionLazy,
    FlagAnchoringOpen,
    FlagAuthorityDominant,
    FlagAutonomySubmissive,
    FlagAvailabilityCalm,
    FlagBiasConfirmationOpen,
    FlagBiasFavoritismFairness,
    FlagCalmNeurotic,
    FlagClaimsCalmReactive,
    FlagCreativityClosed,
    FlagCreativityRigid,
    FlagDisciplineFlaky,
    FlagDisciplineLazy,
    FlagDunningKrugerHumble,
    FlagFairnessRhetoric,
    FlagHelpingCold,
    FlagHelpingSelfish,
    FlagHighELowA,
    FlagHighNLowC,
    FlagHighOLowC,
    FlagHonestFavoritist,
    FlagHonestSelfish,
    FlagImpostorArrogant,
    FlagLearningArrogant,
    FlagLearningRigid,
    FlagLossAversionRisky,
    FlagOpenRigid,
    FlagPatternAchievementComplacent,
    FlagPatternAssertiveQuiet,
    FlagPatternCalmVolatile,
    FlagPatternClaimedCalmVolatile,
    FlagPatternDiplomatEscalator,
    FlagPatternDisciplineShirker,
    FlagPatternEmpathDismissive,
    FlagPatternExtravertQuiet,
    FlagPatternFairExploiter,
    FlagPatternFairnessExploiter,
    FlagPatternFlexibleResister,
    FlagPatternGenerousExploiter,
    FlagPatternHardworkerComplacent,
    FlagPatternHelpingExploiter,
    FlagPatternHonestExploiter,
    FlagPatternHumbleDismissive,
    FlagPatternLearningResister,
    FlagPatternOpenResister,
    FlagPatternPassiveBlowup,
    FlagPatternRecognitionDismissive,
    FlagPatternReliableShirker,
    FlagPatternTrustingParanoid,
    FlagPatternWarmthDismissive,
    FlagPowerPassive,
    FlagRecencyReliable,
    FlagResilientHides,
    FlagResilientReactive,
    FlagRiskAppetiteAmbition,
    FlagSecurityGullible,
    FlagSecurityRisky,
    FlagSocialProofOpen,
    FlagStyleCompetingPassive,
    FlagStyleConsensusAuthoritative,
    FlagStyleControlling,
    FlagStyleDetached,
    FlagStyleDiplomaticBlunt,
    FlagStyleDirectDiplomatic,
    FlagStyleDominantSubmissive,
    FlagStyleEmpatheticCold,
    FlagStyleGuardedTrusting,
    FlagStyleManipulative,
    FlagStyleManipulativeHonest,
    FlagStylePassiveAggressive,
    FlagStyleRepairsTrustDeceitful,
    FlagStyleRulebasedFavoritist,
    FlagStyleServantAuthoritative,
    FlagStyleTrustsFreelySuspicious,
    FlagStyleVirtuebasedDeceitful,
    FlagSunkCostFlexible,
    FlagValueAdventureStability,
    FlagValueCareerFamily,
    FlagValueCommunitySelfish,
    FlagValueFaithDeceitful,
    FlagValueFamilyFuture,
    FlagValueHealthRisky,
    FlagValueKnowledgeArrogant,
    FlagValueLoyaltyGuarded,
    FlagValueStabilityRisk,
    FlagValueWealthGenerous,
    FlagWarmthBlunt,
    FlagWarmthCold,
    FlagWarmthSelfish,
    FormAvatar,
    FormCancel,
    FormConfidence,
    FormContext,
    FormEditTitle,
    FormName,
    FormNewTitle,
    FormNotes,
    FormOceanTitle,
    FormResilience,
    FormRiskAppetite,
    FormRole,
    FormSave,
    FormTags,
    InsightsObserved,
    InsightsSelectPerson,
    InsightsTitle,
    LogAdd,
    LogEmpty,
    LogNoTarget,
    LogNoTrigger,
    LogPlaceholder,
    LogTarget,
    LogTitle,
    LogTrigger,
    LogValence,
    MoreRecs,
    MotUndefinedWarning,
    MotivationsTitle,
    NavPeople,
    NavRelationships,
    NavSync,
    NavTeams,
    NavTimeline,
    NoBiases,
    NoMotivations,
    NoPatterns,
    NoPeopleInsights,
    NoPeopleYet,
    NoReputation,
    NoSearchResults,
    NoValues,
    OceanA,
    OceanAHigh,
    OceanALow,
    OceanAgreeableness,
    OceanC,
    OceanCHigh,
    OceanCLow,
    OceanConscientiousness,
    OceanE,
    OceanEHigh,
    OceanELow,
    OceanExtraversion,
    OceanN,
    OceanNHigh,
    OceanNLow,
    OceanNeuroticism,
    OceanO,
    OceanOHigh,
    OceanOLow,
    OceanOpenness,
    OceanTitle,
    PatternHelperChange,
    PatternHelperConflict,
    PatternHelperFeedback,
    PatternHelperInjustice,
    PatternHelperRecognition,
    PatternHelperStress,
    PatternHelperSuccess,
    PatternHelperThreat,
    PatternHelperUncertainty,
    PatternsTitle,
    PersonNotFound,
    PersonSelfScore,
    PlName,
    PredAccuracyLabel,
    PredActualLabel,
    PredActualPlaceholder,
    PredAddBtn,
    PredAllTitle,
    PredCancelBtn,
    PredContextPlaceholder,
    PredDeleteBtn,
    PredFor,
    PredNone,
    PredOutcomePlaceholder,
    PredPredictedLabel,
    PredResolveBtn,
    PredResolveSubmit,
    PredTitle,
    ProfileCompleteness,
    RelCloseAdd,
    RelConfirmDelete,
    RelNone,
    RelNotes,
    RelOpenAdd,
    RelPersonRel,
    RelSearchPlaceholder,
    RelStrength,
    RelTitle,
    ReliabilityTitle,
    RepScaleHint,
    RepUndefinedWarning,
    ReputationTitle,
    ResilienceLabel,
    RiskAppetiteLabel,
    ScaleFriction,
    ScaleGood,
    ScaleModerate,
    ScaleStrong,
    ScaleTension,
    ScoreBand,
    SearchPlaceholder,
    StrategyChangeDisciplineRhetoric,
    StrategyChangeFallback,
    StrategyChangeHighC,
    StrategyChangeHighN,
    StrategyChangeHighO,
    StrategyChangeLabel,
    StrategyChangeLowE,
    StrategyChangeLowN,
    StrategyConflictAffiliationRhetoric,
    StrategyConflictAffiliationTrustRhetoric,
    StrategyConflictFallback,
    StrategyConflictHighA,
    StrategyConflictHighC,
    StrategyConflictHighE,
    StrategyConflictHighN,
    StrategyConflictLabel,
    StrategyConflictLowA,
    StrategyConflictLowE,
    StrategyFeedbackFallback,
    StrategyFeedbackHelpingRhetoric,
    StrategyFeedbackHighC,
    StrategyFeedbackHighN,
    StrategyFeedbackLabel,
    StrategyFeedbackLowA,
    StrategyFeedbackLowE,
    StrategyFeedbackLowN,
    StrategyFeedbackWarmthRhetoric,
    StrategyInjusticeAmbitionRhetoric,
    StrategyInjusticeFairness,
    StrategyInjusticeFairnessRhetoric,
    StrategyInjusticeFallback,
    StrategyInjusticeHighA,
    StrategyInjusticeHighN,
    StrategyInjusticeLabel,
    StrategyInjusticePower,
    StrategyRecognitionFallback,
    StrategyRecognitionHigh,
    StrategyRecognitionHighE,
    StrategyRecognitionLabel,
    StrategyRecognitionLow,
    StrategyRecognitionLowE,
    StrategyRecognitionMid,
    StrategyStressAmbitionRhetoric,
    StrategyStressFallback,
    StrategyStressHighC,
    StrategyStressHighE,
    StrategyStressHighN,
    StrategyStressHighO,
    StrategyStressLabel,
    StrategyStressLowA,
    StrategyStressLowC,
    StrategyStressLowE,
    StrategyStressPower,
    StrategyStressSecurity,
    StrategyStressSecurityRhetoric,
    StrategySuccessAmbitionRhetoric,
    StrategySuccessFallback,
    StrategySuccessHighA,
    StrategySuccessHighC,
    StrategySuccessHighO,
    StrategySuccessLabel,
    StrategySuccessLowE,
    StrategySuccessPower,
    StrategySuccessRecognition,
    StrategyThreatFallback,
    StrategyThreatHighA,
    StrategyThreatHighN,
    StrategyThreatLabel,
    StrategyThreatLowA,
    StrategyThreatPower,
    StrategyUncertaintyFallback,
    StrategyUncertaintyHighC,
    StrategyUncertaintyHighE,
    StrategyUncertaintyHighN,
    StrategyUncertaintyHighO,
    StrategyUncertaintyLabel,
    StrategyUncertaintyLowN,
    StrategyUncertaintyLowO,
    StrategyWhen,
    StyleNoStyles,
    StylePanelTitle,
    SyncBackedUp,
    SyncBackingUp,
    SyncBackupBtn,
    SyncClearBtn,
    SyncExportBtn,
    SyncExported,
    SyncGdriveTitle,
    SyncImportBtn,
    SyncLastBackedUp,
    SyncLocalDesc,
    SyncLocalTitle,
    SyncNetworkError,
    SyncNoDataWarn,
    SyncNoToken,
    SyncNotConfigured,
    SyncPassphraseHide,
    SyncPassphraseLabel,
    SyncPassphrasePlaceholder,
    SyncPassphraseShow,
    SyncPastePlaceholder,
    SyncReauth,
    SyncRestoreBtn,
    SyncRestored,
    SyncRestoring,
    SyncSaveTokenBtn,
    SyncSignIn,
    SyncTitle,
    SyncTokenCleared,
    SyncTokenExpired,
    SyncTokenInstruction1,
    SyncTokenInstruction2,
    SyncTokenInstruction3,
    SyncTokenInstruction4,
    SyncTokenLoaded,
    SyncTokenSaved,
    SyncViewBackup,
    SyncWrongPassphrase,
    TeamAllNoEdit,
    TeamAvgDanger,
    TeamAvgScore,
    TeamCtxAvg,
    TeamEdit,
    TeamEmpty,
    TeamIcon,
    TeamMaxDanger,
    TeamMembersCount,
    TeamNoDanger,
    TeamPairs,
    TeamRename,
    TeamSize,
    TeamStrongest,
    TeamTabMembers,
    TeamTabSynergy,
    TeamTitle,
    TeamWeakest,
    TeamsAll,
    TeamsCreate,
    TeamsDelete,
    TeamsMembers,
    TeamsTitle,
    TemplateBlank,
    TemplateTitle,
    TlEmpty,
    TlTitle,
    ToastDeleted,
    ToastError,
    ToastSaved,
    TrendDeteriorating,
    TrendHint,
    TrendImproving,
    TrendStable,
    TutCompareBody,
    TutCompareTitle,
    TutCreateBody,
    TutCreateTitle,
    TutDoneBody,
    TutDoneTitle,
    TutMotBiasBody,
    TutMotBiasTitle,
    TutOceanBody,
    TutOceanTitle,
    TutPeopleBody,
    TutPeopleTitle,
    TutRepPatternBody,
    TutRepPatternTitle,
    TutStep,
    TutWelcomeBody,
    TutWelcomeTitle,
    ValuesTitle,
    ValueIntensityHelper,
    ValuePriorityHelper,
    PersonaSection,
    PersonaHint,
    PersonaCopyBase,
    PersonaClear,
    PersonaBalanceTitle,
    PersonaOnlineSection,
    PersonaBaseSection,
    FacetBase,
    FacetWork,
    FacetOnline,
    FacetAuto,
    FacetMain,
    FacetMainSuffix,
    MainContextLabel,
    PersonaContextPrompt,
    PersonaContextChange,
    MaskGapLow,
    MaskGapModerate,
    MaskGapHigh,
    TeamFacetToggle,
    #[strum(serialize = "OCEAN volatility")]
    OceanVolatility,
    #[strum(serialize = "Rep power struggle")]
    RepPowerStruggle,
    #[strum(serialize = "Only negative patterns")]
    OnlyNegativePatterns,
    #[strum(serialize = "Low prediction accuracy")]
    LowPredictionAccuracy,
    Unknown,
}

pub fn tr(key: Key, lang: Lang) -> &'static str {
    match lang {
        Lang::Fr => fr(key),
        Lang::En => en(key),
    }
}

/// Runtime-safe variant of [`tr`]: resolves a snake_case key string to a
/// [`Key`] and translates it, falling back to the raw string when it does
/// not map to any known key.
pub fn tr_str(key: &str, lang: Lang) -> String {
    match key.parse::<Key>() {
        Ok(Key::Unknown) | Err(_) => String::from(key),
        Ok(k) => tr(k, lang).to_string(),
    }
}

pub fn tr_danger_details(details: &str, lang: Lang) -> String {
    if details.is_empty() {
        return String::new();
    }
    fn key(s: &str) -> Key {
        match s {
            "OCEAN volatility" => Key::OceanVolatility,
            "Rep power struggle" => Key::RepPowerStruggle,
            "Only negative patterns" => Key::OnlyNegativePatterns,
            "Low prediction accuracy" => Key::LowPredictionAccuracy,
            _ => Key::Unknown,
        }
    }
    details
        .split(", ")
        .map(|d| tr(key(d), lang))
        .collect::<Vec<_>>()
        .join(", ")
}

fn en(key: Key) -> &'static str {
    match key {
        Key::AddBtn => "＋",
        Key::AriaAddBias => "Add bias",
        Key::AriaAddMotivation => "Add motivation",
        Key::AriaAddPattern => "Add pattern",
        Key::AriaAddStyle => "Add style",
        Key::AriaAddValue => "Add value",
        Key::AriaAvatarPrefix => "Avatar",
        Key::AriaDeleteBias => "Delete bias",
        Key::AriaDeleteLogEntry => "Delete log entry",
        Key::AriaDeleteMotivation => "Delete motivation",
        Key::AriaDeletePattern => "Delete pattern",
        Key::AriaDeleteStyle => "Delete style",
        Key::AriaDeleteValue => "Delete value",
        Key::AriaDiscardPrefix => "Discard",
        Key::AriaEditBias => "Edit bias",
        Key::AriaEditMotivation => "Edit motivation",
        Key::AriaEditPattern => "Edit pattern",
        Key::AriaEditStyle => "Edit style",
        Key::AriaEditValue => "Edit value",
        Key::AriaMoveBiasDown => "Move bias down",
        Key::AriaMoveBiasUp => "Move bias up",
        Key::AriaMoveMotivationDown => "Move motivation down",
        Key::AriaMoveMotivationUp => "Move motivation up",
        Key::AriaMovePatternDown => "Move pattern down",
        Key::AriaMovePatternUp => "Move pattern up",
        Key::AriaMoveStyleDown => "Move style down",
        Key::AriaMoveStyleUp => "Move style up",
        Key::AriaMoveValueDown => "Move value down",
        Key::AriaMoveValueUp => "Move value up",
        Key::AriaSkipToContent => "Skip to content",
        Key::AriaUpdateBias => "Update bias",
        Key::AriaUpdateMotivation => "Update motivation",
        Key::AriaUpdatePattern => "Update pattern",
        Key::AriaUpdateStyle => "Update style",
        Key::AriaUpdateValue => "Update value",
        Key::BiasScaleHint => "0 = this bias is absent, 10 = it shapes most decisions.",
        Key::BiasUndefinedWarning => "Undefined biases count as present. Set 0 to mark as absent.",
        Key::BiasesTitle => "Biases",
        Key::BucketInheritsBase => "Inherits base",
        Key::BucketOverride => "Override",
        Key::CommonAdd => "Add",
        Key::CommonBack => "← Back",
        Key::CommonCancel => "Cancel",
        Key::CommonDelete => "Delete",
        Key::CommonEdit => "Edit",
        Key::CommonFinish => "Finish",
        Key::CommonNext => "Next →",
        Key::CommonSave => "Save",
        Key::CommonSkip => "Skip",
        Key::CompareAnalysisTitle => "Dynamic Analysis",
        Key::CompareAsymmetric => "Mutual benefit",
        Key::CompareBalanced => "Balanced",
        Key::CompareBandHint => "±{}% (relationship + profile confidence)",
        Key::CompareBenefitMore => "benefits more",
        Key::CompareBiasMain => "Main Bias",
        Key::CompareBreakdown => "Breakdown",
        Key::CompareBtn => "Compare",
        Key::CompareCatBias => "Bias",
        Key::CompareCatMotivation => "Motivation",
        Key::CompareCatOcean => "OCEAN",
        Key::CompareCatPatterns => "Patterns",
        Key::CompareCatReputation => "Reputation",
        Key::CompareCatStyles => "Styles",
        Key::CompareCatValues => "Values",
        Key::CompareCtxTitle => "By situation",
        Key::CompareEthics => {
            "These are probabilistic models, not absolute truths. Use them to understand better, never to manipulate."
        }
        Key::CompareFacetUnavailable => "Not scoreable in this context — a persona is missing",
        Key::CompareFriction => "Friction Points",
        Key::CompareOcean => "OCEAN Profile",
        Key::CompareRelNone => "General (no context)",
        Key::CompareRelStrength => "Strength",
        Key::CompareRelTitle => "Relationship Context",
        Key::CompareRiskMitigation => "Risks & Mitigations",
        Key::CompareStrategy => "Interaction Strategy",
        Key::CompareSub => "Identify synergies and friction points between two people",
        Key::CompareSynergies => "Synergies",
        Key::CompareTitle => "Compare Persons",
        Key::CompareTopMot => "Top Motivation",
        Key::CompareVs => "VS",
        Key::ConfidenceHint => {
            "How reliable is this profile? 1 = rough sketch, 10 = built from real observations."
        }
        Key::ConfidenceLabel => "Profile confidence",
        Key::ConfirmDelete => "Delete this person?",
        Key::ConfirmDeleteLog => "Delete this entry?",
        Key::ConfirmDeletePred => "Delete this prediction?",
        Key::ConfirmDeleteTeam => "Delete this team?",
        Key::CtxChange => "Change",
        Key::CtxCommunication => "Communication",
        Key::CtxConflict => "Conflict",
        Key::CtxDecision => "Decision",
        Key::CtxFeedback => "Feedback",
        Key::CtxGrowth => "Growth",
        Key::CtxInjustice => "Injustice",
        Key::CtxLeadership => "Leadership",
        Key::CtxRecognition => "Recognition",
        Key::CtxStress => "Stress",
        Key::CtxSuccess => "Success",
        Key::CtxTeam => "Team",
        Key::CtxThreatened => "Threatened",
        Key::CtxUncertainty => "Uncertainty",
        Key::DeleteBtn => "🗑 Delete",
        Key::EditBiases => "Biases",
        Key::EditBtn => "✏ Edit",
        Key::EditDiscardSection => "Discard",
        Key::EditEvidencePlaceholder => "Evidence",
        Key::EditMotivations => "Motivations",
        Key::EditNotesPlaceholder => "Notes",
        Key::EditPatterns => "Behavioral Patterns",
        Key::EditPriority => "P",
        Key::EditReputation => "Reputation",
        Key::EditStyles => "Personal Styles",
        Key::EditUpdateBtn => "💾",
        Key::EditValues => "Values",
        Key::FlagAffiliationCold => {
            "Values closeness but is perceived as cold and detached — do as I say, not as I do."
        }
        Key::FlagAffiliationDistrustful => {
            "Values closeness but is perceived as suspicious — do as I say, not as I do."
        }
        Key::FlagAmbitionLazy => {
            "Aspires to power, success, or recognition but is perceived as lazy — do as I say, not as I do."
        }
        Key::FlagAnchoringOpen => "Claims open-mindedness yet clings to first impressions.",
        Key::FlagAuthorityDominant => "Perceived as a leader yet blindly defers to authority.",
        Key::FlagAutonomySubmissive => {
            "Preaches independence yet is perceived as submissive — do as I say, not as I do."
        }
        Key::FlagAvailabilityCalm => "Perceived as unflappable yet overweights dramatic events.",
        Key::FlagBiasConfirmationOpen => {
            "Claims open-mindedness yet only seeks confirming information — they don't know themselves."
        }
        Key::FlagBiasFavoritismFairness => {
            "Preaches fairness yet shows favoritism or in-group bias — do as I say, not as I do."
        }
        Key::FlagCalmNeurotic => {
            "Reported as calm under pressure but OCEAN indicates high reactivity — review for consistency."
        }
        Key::FlagClaimsCalmReactive => {
            "Claims to be calm and stable but is perceived as reactive — they don't know themselves."
        }
        Key::FlagCreativityClosed => {
            "Preaches creativity yet self-reports little openness to novelty."
        }
        Key::FlagCreativityRigid => {
            "Preaches creativity yet is perceived as rigid — do as I say, not as I do."
        }
        Key::FlagDisciplineFlaky => {
            "Sees themselves as disciplined but comes across as flaky — they don't know themselves."
        }
        Key::FlagDisciplineLazy => {
            "Self-image of discipline contradicted by a lazy reputation — they don't know themselves."
        }
        Key::FlagDunningKrugerHumble => "Overestimates their competence yet is seen as humble.",
        Key::FlagFairnessRhetoric => {
            "Talks about fairness and justice but practices favoritism — do as I say, not as I do."
        }
        Key::FlagHelpingCold => "Preaches helpfulness yet reads as emotionally cold.",
        Key::FlagHelpingSelfish => {
            "Preaches helpfulness but is perceived as selfish — do as I say, not as I do."
        }
        Key::FlagHighELowA => {
            "Very outgoing but low agreeableness — may be assertive to the point of abrasiveness."
        }
        Key::FlagHighNLowC => {
            "High emotional reactivity with low conscientiousness — may struggle with structure under stress."
        }
        Key::FlagHighOLowC => {
            "Highly creative but unstructured — may have many ideas with difficulty following through."
        }
        Key::FlagHonestFavoritist => {
            "Principled honesty paired with perceived favoritism — may enforce fairness only for some."
        }
        Key::FlagHonestSelfish => {
            "Principled honesty paired with low generosity — may indicate a rigid moral stance."
        }
        Key::FlagImpostorArrogant => "Underestimates their competence yet is seen as arrogant.",
        Key::FlagLearningArrogant => "Preaches growth yet is too arrogant to take advice.",
        Key::FlagLearningRigid => {
            "Preaches growth and learning yet is perceived as rigid — do as I say, not as I do."
        }
        Key::FlagLossAversionRisky => "Claims a taste for risk yet is loss-averse.",
        Key::FlagOpenRigid => {
            "Thinks they're open-minded but comes across as rigid — they don't know themselves."
        }
        Key::FlagPatternAchievementComplacent => {
            "Aspires to achievement yet recorded patterns rest on laurels."
        }
        Key::FlagPatternAssertiveQuiet => "Perceived as assertive yet goes quiet when it counts.",
        Key::FlagPatternCalmVolatile => {
            "Perceived as calm under pressure, but recorded patterns show volatility — the calm may be an act."
        }
        Key::FlagPatternClaimedCalmVolatile => {
            "Self-reports calm yet recorded patterns show volatility."
        }
        Key::FlagPatternDiplomatEscalator => "Perceived as diplomatic yet escalates conflict.",
        Key::FlagPatternDisciplineShirker => {
            "Self-image of discipline yet recorded patterns dodge accountability."
        }
        Key::FlagPatternEmpathDismissive => "Perceived as empathetic yet puts others down.",
        Key::FlagPatternExtravertQuiet => {
            "Self-image of extraversion yet recorded patterns go quiet."
        }
        Key::FlagPatternFairExploiter => {
            "Perceived as fair yet exploits injustice for personal gain."
        }
        Key::FlagPatternFairnessExploiter => {
            "Preaches fairness yet recorded patterns exploit injustice."
        }
        Key::FlagPatternFlexibleResister => {
            "Perceived as flexible yet resists change and feedback."
        }
        Key::FlagPatternGenerousExploiter => "Perceived as generous yet exploits others.",
        Key::FlagPatternHardworkerComplacent => {
            "Perceived as hardworking yet rests on past laurels."
        }
        Key::FlagPatternHelpingExploiter => {
            "Preaches helpfulness yet recorded patterns show exploitation."
        }
        Key::FlagPatternHonestExploiter => {
            "Perceived as honest, but recorded patterns show exploitation or blame-shifting — do as I say, not as I do."
        }
        Key::FlagPatternHumbleDismissive => "Perceived as humble yet puts others down.",
        Key::FlagPatternLearningResister => {
            "Preaches learning yet recorded patterns reject feedback."
        }
        Key::FlagPatternOpenResister => "Claims openness yet recorded patterns resist change.",
        Key::FlagPatternPassiveBlowup => "Perceived as passive yet blows up under pressure.",
        Key::FlagPatternRecognitionDismissive => {
            "Seeks recognition yet puts others down to win it."
        }
        Key::FlagPatternReliableShirker => "Perceived as reliable yet dodges accountability.",
        Key::FlagPatternTrustingParanoid => {
            "Perceived as trusting yet turns paranoid under threat."
        }
        Key::FlagPatternWarmthDismissive => {
            "Self-image of warmth yet recorded patterns put others down."
        }
        Key::FlagPowerPassive => "Aspires to power yet is perceived as a pushover.",
        Key::FlagRecencyReliable => "Perceived as steady yet swings with the latest news.",
        Key::FlagResilientHides => "Admits fragility yet appears unflappable — they hide it.",
        Key::FlagResilientReactive => {
            "Claims high resilience but is perceived as reactive — they don't know themselves."
        }
        Key::FlagRiskAppetiteAmbition => "Aspires to power or achievement yet avoids all risk.",
        Key::FlagSecurityGullible => {
            "Claims to value security yet is perceived as gullibly trusting — do as I say, not as I do."
        }
        Key::FlagSecurityRisky => {
            "Preaches caution and security yet self-reports a taste for risk — do as I say, not as I do."
        }
        Key::FlagSocialProofOpen => {
            "Claims independent thinking yet follows the herd — do as I say, not as I do."
        }
        Key::FlagStyleCompetingPassive => "Claims a competitive style yet comes across as passive.",
        Key::FlagStyleConsensusAuthoritative => {
            "Claims consensus-driven yet comes across as a dictator."
        }
        Key::FlagStyleControlling => {
            "Controls and micromanages — perceived as domineering, not trusting."
        }
        Key::FlagStyleDetached => "Openly detached and perceived as cold/distant.",
        Key::FlagStyleDiplomaticBlunt => "Claims a diplomatic style yet comes across as blunt.",
        Key::FlagStyleDirectDiplomatic => "Claims to be direct yet comes across as diplomatic.",
        Key::FlagStyleDominantSubmissive => {
            "Claims an autocratic style yet comes across as submissive."
        }
        Key::FlagStyleEmpatheticCold => "Claims empathy yet comes across as cold.",
        Key::FlagStyleGuardedTrusting => "Claims to be guarded yet comes across as trusting.",
        Key::FlagStyleManipulative => "Admits a manipulative style and is perceived as deceitful.",
        Key::FlagStyleManipulativeHonest => "Claims to play dirty yet comes across as honest.",
        Key::FlagStylePassiveAggressive => "Openly passive-aggressive and perceived as reactive.",
        Key::FlagStyleRepairsTrustDeceitful => {
            "Claims to repair trust yet comes across as deceitful."
        }
        Key::FlagStyleRulebasedFavoritist => "Claims a rules-based approach yet plays favorites.",
        Key::FlagStyleServantAuthoritative => {
            "Claims servant leadership yet comes across as a commander."
        }
        Key::FlagStyleTrustsFreelySuspicious => {
            "Claims to trust freely yet comes across as suspicious."
        }
        Key::FlagStyleVirtuebasedDeceitful => {
            "Claims a virtue-based approach yet comes across as deceitful."
        }
        Key::FlagSunkCostFlexible => "Perceived as flexible yet clings to sunk costs.",
        Key::FlagValueAdventureStability => {
            "Values both adventure and stability — opposing drivers."
        }
        Key::FlagValueCareerFamily => {
            "Both career and family rated as top priorities — expect tension."
        }
        Key::FlagValueCommunitySelfish => {
            "Values community yet is perceived as selfish — contradictory."
        }
        Key::FlagValueFaithDeceitful => {
            "Values faith yet is perceived as deceitful — contradictory."
        }
        Key::FlagValueFamilyFuture => {
            "Values family highly yet decides through a future-oriented lens."
        }
        Key::FlagValueHealthRisky => {
            "Values health yet has a very high risk appetite — contradictory."
        }
        Key::FlagValueKnowledgeArrogant => {
            "Values knowledge yet comes across as arrogant — contradictory."
        }
        Key::FlagValueLoyaltyGuarded => {
            "Values loyalty yet adopts a guarded, distrustful trust style."
        }
        Key::FlagValueStabilityRisk => {
            "Craves stability yet has a very high risk appetite — contradictory."
        }
        Key::FlagValueWealthGenerous => {
            "Values wealth yet is perceived as generous — contradictory."
        }
        Key::FlagWarmthBlunt => {
            "Self-image of warmth contradicted by a blunt reputation — they don't know themselves."
        }
        Key::FlagWarmthCold => {
            "Thinks they're warm-hearted but comes across cold — they don't know themselves."
        }
        Key::FlagWarmthSelfish => "Claims warmth yet is perceived as selfish.",
        Key::FormAvatar => "Avatar",
        Key::FormCancel => "Cancel",
        Key::FormConfidence => "Profile confidence (1-10)",
        Key::FormContext => "Context",
        Key::FormEditTitle => "Edit Person",
        Key::FormName => "Name",
        Key::FormNewTitle => "New Person",
        Key::FormNotes => "Notes",
        Key::FormOceanTitle => "OCEAN Scores (1-10)",
        Key::FormResilience => "Resilience (1-10)",
        Key::FormRiskAppetite => "Risk appetite (1-10)",
        Key::FormRole => "Role",
        Key::FormSave => "💾 Save",
        Key::FormTags => "Tags (comma separated)",
        Key::InsightsObserved => "Observed Patterns",
        Key::InsightsSelectPerson => "Select a person to view behavioral insights.",
        Key::InsightsTitle => "📊 Insights",
        Key::LogAdd => "Add entry",
        Key::LogEmpty => "No entries yet.",
        Key::LogNoTarget => "Self note (no target)",
        Key::LogNoTrigger => "No trigger",
        Key::LogPlaceholder => "What happened?",
        Key::LogTarget => "With",
        Key::LogTitle => "📋 Log",
        Key::LogTrigger => "Trigger",
        Key::LogValence => "Valence",
        Key::MoreRecs => "More recommendations",
        Key::MotUndefinedWarning => {
            "Fewer than 3 motivations penalizes (−0.03 each). Missing Fairness/Helping also hurts."
        }
        Key::MotivationsTitle => "Motivations",
        Key::NavPeople => "People",
        Key::NavRelationships => "Relationships",
        Key::NavSync => "Sync",
        Key::NavTeams => "Teams",
        Key::NavTimeline => "Timeline",
        Key::NoBiases => "No biases recorded.",
        Key::NoMotivations => "No motivations recorded.",
        Key::NoPatterns => "No behavioral patterns recorded.",
        Key::NoPeopleInsights => "No persons yet. Add someone to see insights.",
        Key::NoPeopleYet => "No people yet. Tap + to add someone.",
        Key::NoReputation => "No reputation traits recorded.",
        Key::NoSearchResults => "No results for \"{0}\".",
        Key::NoValues => "No values defined",
        Key::OceanA => "A — Agreeableness",
        Key::OceanAHigh => "cooperative, empathetic, seeks harmony",
        Key::OceanALow => "direct or abrasive, puts goals before relationships",
        Key::OceanAgreeableness => "Agreeableness",
        Key::OceanC => "C — Conscientiousness",
        Key::OceanCHigh => "organized, reliable, results and detail-oriented",
        Key::OceanCLow => "flexible and spontaneous, may lack rigor",
        Key::OceanConscientiousness => "Conscientiousness",
        Key::OceanE => "E — Extraversion",
        Key::OceanEHigh => "extraverted, energetic, seeks social stimulation",
        Key::OceanELow => "introverted, thoughtful, prefers limited interactions",
        Key::OceanExtraversion => "Extraversion",
        Key::OceanN => "N — Neuroticism",
        Key::OceanNHigh => "emotionally reactive, prone to stress, sensitive to criticism",
        Key::OceanNLow => "emotionally stable, calm under pressure",
        Key::OceanNeuroticism => "Neuroticism",
        Key::OceanO => "O — Openness",
        Key::OceanOHigh => "very open to new ideas, creative and curious",
        Key::OceanOLow => "pragmatic, prefers routines and concrete things",
        Key::OceanOpenness => "Openness",
        Key::OceanTitle => "OCEAN Scores",
        Key::PatternHelperChange => {
            "How they adapt to transitions and new situations — at work: a reorg or a new role; in everyday life: a move or a new routine"
        }
        Key::PatternHelperConflict => {
            "How they handle disagreements and confrontation — at work: a clash in a meeting; in everyday life: a heavy argument at home"
        }
        Key::PatternHelperFeedback => {
            "How they receive and process input from others — at work: a post-project review; in everyday life: a friend pointing out a blind spot"
        }
        Key::PatternHelperInjustice => {
            "How they react when treated unfairly or witnessing unfairness — at work: an unfairly skipped promotion; in everyday life: seeing someone treated unfairly"
        }
        Key::PatternHelperRecognition => {
            "How they seek and respond to acknowledgment — at work: being praised by a manager; in everyday life: being appreciated by friends"
        }
        Key::PatternHelperStress => {
            "How they react under pressure or tight deadlines — at work: a looming deadline; in everyday life: a jam-packed day"
        }
        Key::PatternHelperSuccess => {
            "How they respond to achievements and wins — at work: closing a big deal; in everyday life: finishing a personal milestone"
        }
        Key::PatternHelperThreat => {
            "How they defend themselves when feeling attacked — at work: criticized during a review; in everyday life: cornered in a heated talk"
        }
        Key::PatternHelperUncertainty => {
            "How they navigate ambiguity and unknown outcomes — at work: an unclear project scope; in everyday life: waiting on an uncertain outcome"
        }
        Key::PatternsTitle => "Behavioral Patterns",
        Key::PersonNotFound => "Person not found",
        Key::PersonSelfScore => "Profile Score",
        Key::PlName => "Name",
        Key::PredAccuracyLabel => "Accuracy",
        Key::PredActualLabel => "Actual",
        Key::PredActualPlaceholder => "Actual outcome...",
        Key::PredAddBtn => "Add",
        Key::PredAllTitle => "All Predictions",
        Key::PredCancelBtn => "Cancel",
        Key::PredContextPlaceholder => "Context...",
        Key::PredDeleteBtn => "Delete",
        Key::PredFor => "🔮 Predictions for",
        Key::PredNone => "No predictions yet.",
        Key::PredOutcomePlaceholder => "Predicted outcome...",
        Key::PredPredictedLabel => "Predicted",
        Key::PredResolveBtn => "Resolve",
        Key::PredResolveSubmit => "✓ Resolve",
        Key::PredTitle => "Predictions",
        Key::ProfileCompleteness => "Compl.",
        Key::RelCloseAdd => "− Cancel",
        Key::RelConfirmDelete => "Delete this relationship?",
        Key::RelNone => "No relationships yet.",
        Key::RelNotes => "Notes",
        Key::RelOpenAdd => "＋ Add",
        Key::RelPersonRel => "Relationships",
        Key::RelSearchPlaceholder => "Search person…",
        Key::RelStrength => "Strength",
        Key::RelTitle => "Relationships",
        Key::ReliabilityTitle => "Data quality",
        Key::RepScaleHint => {
            "0 = the negative pole, 10 = the positive pole — the ✗ toggle leaves the dimension unknown."
        }
        Key::RepUndefinedWarning => {
            "Undefined traits penalize reputation. Extreme values (≤2 or ≥8) trigger adjustments."
        }
        Key::ReputationTitle => "Reputation",
        Key::ResilienceLabel => "Resilience",
        Key::RiskAppetiteLabel => "Risk appetite",
        Key::ScaleFriction => "Friction",
        Key::ScaleGood => "Good",
        Key::ScaleModerate => "Moderate",
        Key::ScaleStrong => "Strong",
        Key::ScaleTension => "Tension",
        Key::ScoreBand => "±{}",
        Key::SearchPlaceholder => "Search people...",
        Key::StrategyChangeDisciplineRhetoric => {
            "They see themselves as disciplined yet are perceived as lazy — don't appeal to their organized self-image; check actual output."
        }
        Key::StrategyChangeFallback => "Communicate the why and involve them in the transition.",
        Key::StrategyChangeHighC => "High conscientiousness — needs a clear transition roadmap.",
        Key::StrategyChangeHighN => {
            "High neuroticism — may resist change; provide stability anchors."
        }
        Key::StrategyChangeHighO => {
            "High openness — embrace change; give them a role in shaping it."
        }
        Key::StrategyChangeLabel => "Facing change",
        Key::StrategyChangeLowE => "Low extraversion — needs time to process change privately.",
        Key::StrategyChangeLowN => "Low neuroticism — adapts well; leverage as change champion.",
        Key::StrategyConflictAffiliationRhetoric => {
            "They value closeness yet come across cold — don't appeal to their stated need for connection; address the detachment directly."
        }
        Key::StrategyConflictAffiliationTrustRhetoric => {
            "They value closeness yet come across distrustful — don't appeal to their stated need for connection; earn credibility before seeking rapport."
        }
        Key::StrategyConflictFallback => "Mediate with balanced communication.",
        Key::StrategyConflictHighA => {
            "High agreeableness — soften confrontation, focus on harmony."
        }
        Key::StrategyConflictHighC => {
            "High conscientiousness — may rigidly insist on rules and procedures."
        }
        Key::StrategyConflictHighE => "High extraversion — let them talk it through.",
        Key::StrategyConflictHighN => {
            "High neuroticism — de-escalate and provide emotional safety."
        }
        Key::StrategyConflictLabel => "In conflict",
        Key::StrategyConflictLowA => "Low agreeableness — address conflict directly with facts.",
        Key::StrategyConflictLowE => {
            "Low extraversion — may withdraw or stonewall instead of engaging."
        }
        Key::StrategyFeedbackFallback => {
            "Balance praise and constructive input with specific examples."
        }
        Key::StrategyFeedbackHelpingRhetoric => {
            "They preach helpfulness yet are perceived as selfish — don't frame feedback around helping others; name the self-interest behind the advice."
        }
        Key::StrategyFeedbackHighC => {
            "High conscientiousness — values detailed, actionable feedback."
        }
        Key::StrategyFeedbackHighN => {
            "High neuroticism — may take feedback personally; use gentle framing."
        }
        Key::StrategyFeedbackLabel => "Receiving feedback",
        Key::StrategyFeedbackLowA => "Low agreeableness — may reject feedback; focus on data.",
        Key::StrategyFeedbackLowE => "Low extraversion — prefers private, written feedback.",
        Key::StrategyFeedbackLowN => "Low neuroticism — handles critical feedback well; be direct.",
        Key::StrategyFeedbackWarmthRhetoric => {
            "They see themselves as warm yet are perceived as blunt — don't rely on soft delivery; be clear and specific about the behavior."
        }
        Key::StrategyInjusticeAmbitionRhetoric => {
            "They talk ambition but are perceived as lazy — don't expect them to fight for the cause; frame the outcome as serving their status instead."
        }
        Key::StrategyInjusticeFairness => {
            "Fairness-driven — will fight for what they believe is right, even at personal cost."
        }
        Key::StrategyInjusticeFairnessRhetoric => {
            "Speaks of fairness but acts with favoritism — don't appeal to their justice rhetoric; address the real driver instead."
        }
        Key::StrategyInjusticeFallback => {
            "Acknowledge their concern and clarify the path to resolution."
        }
        Key::StrategyInjusticeHighA => {
            "High agreeableness — may feel personally wounded by unfairness."
        }
        Key::StrategyInjusticeHighN => {
            "High neuroticism — may ruminate and escalate perceived slights."
        }
        Key::StrategyInjusticeLabel => "Facing injustice",
        Key::StrategyInjusticePower => {
            "Power-driven — may leverage authority to correct the perceived wrong."
        }
        Key::StrategyRecognitionFallback => "Match recognition style to their comfort level.",
        Key::StrategyRecognitionHigh => {
            "Strong recognition drive — give frequent, specific praise."
        }
        Key::StrategyRecognitionHighE => "High extraversion — public recognition is effective.",
        Key::StrategyRecognitionLabel => "Seeking recognition",
        Key::StrategyRecognitionLow => "Low recognition need — avoid over-praising.",
        Key::StrategyRecognitionLowE => {
            "Low extraversion — prefer private, written acknowledgment."
        }
        Key::StrategyRecognitionMid => {
            "Moderate recognition drive — acknowledge contributions regularly."
        }
        Key::StrategyStressAmbitionRhetoric => {
            "They talk ambition but are perceived as lazy — don't reward the rhetoric; focus on effort and follow-through."
        }
        Key::StrategyStressFallback => "Monitor stress signals and adjust environment.",
        Key::StrategyStressHighC => {
            "High conscientiousness — break problems into actionable steps."
        }
        Key::StrategyStressHighE => "High extraversion — allow verbal processing of stress.",
        Key::StrategyStressHighN => "High neuroticism — provide reassurance and clear structure.",
        Key::StrategyStressHighO => {
            "High openness — may overthink and spiral into worst-case scenarios."
        }
        Key::StrategyStressLabel => "Under stress",
        Key::StrategyStressLowA => {
            "Low agreeableness — may become short or irritable under pressure."
        }
        Key::StrategyStressLowC => "Low conscientiousness — may become disorganized or avoidant.",
        Key::StrategyStressLowE => "Low extraversion — give quiet space to decompress.",
        Key::StrategyStressPower => "Power-driven — let them regain control in one domain.",
        Key::StrategyStressSecurity => "Security-driven — reinforce stability and routine.",
        Key::StrategyStressSecurityRhetoric => {
            "They claim to value security yet are gullibly trusting — don't rely on their stated caution; verify safeguards yourself."
        }
        Key::StrategySuccessAmbitionRhetoric => {
            "They talk ambition but are perceived as lazy — don't celebrate their plans; require delivery."
        }
        Key::StrategySuccessFallback => "Celebrate success and identify growth areas.",
        Key::StrategySuccessHighA => {
            "High agreeableness — may deflect credit to avoid standing out."
        }
        Key::StrategySuccessHighC => {
            "High conscientiousness — leverage success as validation of process."
        }
        Key::StrategySuccessHighO => {
            "High openness — channel success into new creative challenges."
        }
        Key::StrategySuccessLabel => "In success",
        Key::StrategySuccessLowE => "Low extraversion — may feel overwhelmed by public attention.",
        Key::StrategySuccessPower => "Power-driven — give them ownership of the next initiative.",
        Key::StrategySuccessRecognition => {
            "Recognition-driven — publicly acknowledge their achievement."
        }
        Key::StrategyThreatFallback => "Listen actively and validate their concerns.",
        Key::StrategyThreatHighA => {
            "High agreeableness — they may concede too easily; check true feelings."
        }
        Key::StrategyThreatHighN => {
            "High neuroticism — perceived threats are amplified; offer reassurance."
        }
        Key::StrategyThreatLabel => "Feeling threatened",
        Key::StrategyThreatLowA => {
            "Low agreeableness — they may push back; address concerns calmly."
        }
        Key::StrategyThreatPower => {
            "Power-driven — threat to status is serious; involve them in decisions."
        }
        Key::StrategyUncertaintyFallback => {
            "Acknowledge uncertainty and provide available information."
        }
        Key::StrategyUncertaintyHighC => {
            "High conscientiousness — needs a concrete plan immediately."
        }
        Key::StrategyUncertaintyHighE => {
            "High extraversion — may over-socialize to cope with ambiguity."
        }
        Key::StrategyUncertaintyHighN => {
            "High neuroticism — provide clear timelines and frequent updates."
        }
        Key::StrategyUncertaintyHighO => "High openness — frame uncertainty as opportunity.",
        Key::StrategyUncertaintyLabel => "In uncertainty",
        Key::StrategyUncertaintyLowN => {
            "Low neuroticism — they handle ambiguity well; trust their resilience."
        }
        Key::StrategyUncertaintyLowO => {
            "Low openness — provide concrete examples and familiar frameworks."
        }
        Key::StrategyWhen => "When {name} is {trigger}:\n\n{advice}",
        Key::StyleNoStyles => "No personal styles recorded.",
        Key::StylePanelTitle => "Personal Styles",
        Key::SyncBackedUp => "✅ Backed up",
        Key::SyncBackingUp => "Backing up...",
        Key::SyncBackupBtn => "☁ Backup to Drive",
        Key::SyncClearBtn => "Clear",
        Key::SyncExportBtn => "📥 Export JSON",
        Key::SyncExported => "✅ Exported",
        Key::SyncGdriveTitle => "Google Drive Sync",
        Key::SyncImportBtn => "📤 Import JSON",
        Key::SyncLastBackedUp => "Last backed up: ",
        Key::SyncLocalDesc => "Export all data as JSON or import from a previous backup.",
        Key::SyncLocalTitle => "Local Backup",
        Key::SyncNetworkError => "❌ Network error — check your connection and try again.",
        Key::SyncNoDataWarn => "No people data to back up. Add people first!",
        Key::SyncNoToken => "No token. Sign in first.",
        Key::SyncNotConfigured => {
            "Google Drive backup not configured at build time. Set GOOGLE_CLIENT_ID env var before building."
        }
        Key::SyncPassphraseHide => "Hide",
        Key::SyncPassphraseLabel => "Encrypt backup with passphrase (optional)",
        Key::SyncPassphrasePlaceholder => "Enter passphrase...",
        Key::SyncPassphraseShow => "Show",
        Key::SyncPastePlaceholder => "Paste the full redirect URL here",
        Key::SyncReauth => "🔐 Sign in again",
        Key::SyncRestoreBtn => "☁ Restore from Drive",
        Key::SyncRestored => "✅ Restored",
        Key::SyncRestoring => "Restoring...",
        Key::SyncSaveTokenBtn => "Save Token",
        Key::SyncSignIn => "🔐 Sign in with Google",
        Key::SyncTitle => "☁ Sync & Backup",
        Key::SyncTokenCleared => "Token cleared",
        Key::SyncTokenExpired => "Your Google sign-in has expired. Sign in again to keep syncing.",
        Key::SyncTokenInstruction1 => "1. Tap 'Sign in with Google' — opens your browser",
        Key::SyncTokenInstruction2 => "2. Sign in and grant access",
        Key::SyncTokenInstruction3 => {
            "3. Browser redirects to the web app — copy the token from the address bar before the page loads"
        }
        Key::SyncTokenInstruction4 => "4. Paste the URL below and tap Save",
        Key::SyncTokenLoaded => "✓ Token loaded",
        Key::SyncTokenSaved => "✅ Token saved",
        Key::SyncViewBackup => "🔎 View backups in your browser (appDataFolder Browser)",
        Key::SyncWrongPassphrase => "❌ Wrong passphrase or corrupted data",
        Key::TeamAllNoEdit => "All People includes everyone automatically",
        Key::TeamAvgDanger => "Avg danger",
        Key::TeamAvgScore => "Avg score",
        Key::TeamCtxAvg => "Average by situation",
        Key::TeamEdit => "Edit",
        Key::TeamEmpty => "Add at least 2 people to see team synergy.",
        Key::TeamIcon => "Icon",
        Key::TeamMaxDanger => "Max danger",
        Key::TeamMembersCount => "{0} members",
        Key::TeamNoDanger => "None",
        Key::TeamPairs => "All pairs",
        Key::TeamRename => "Rename",
        Key::TeamSize => "Team size",
        Key::TeamStrongest => "Strongest link",
        Key::TeamTabMembers => "Members",
        Key::TeamTabSynergy => "Synergy",
        Key::TeamTitle => "Team Synergy",
        Key::TeamWeakest => "Weakest link",
        Key::TeamsAll => "All People",
        Key::TeamsCreate => "New Team",
        Key::TeamsDelete => "Delete team?",
        Key::TeamsMembers => "{0} members",
        Key::TeamsTitle => "Teams",
        Key::TemplateBlank => "Blank (start from scratch)",
        Key::TemplateTitle => "Quick Template",
        Key::TlEmpty => "No interaction entries yet.",
        Key::TlTitle => "Timeline",
        Key::ToastDeleted => "Deleted",
        Key::ToastError => "Something went wrong",
        Key::ToastSaved => "Saved",
        Key::TrendDeteriorating => "Deteriorating",
        Key::TrendHint => "From recent logged interactions",
        Key::TrendImproving => "Improving",
        Key::TrendStable => "Stable",
        Key::TutCompareBody => {
            "Once you have at least two people, you can compare them side by side to see their synergy score, friction points, and interaction strategies.\n\nYou can also track predictions (guess an outcome, then check if you were right), build a relationship map, and log interactions on a timeline."
        }
        Key::TutCompareTitle => "Comparisons & More",
        Key::TutCreateBody => {
            "The person form is divided into sections: basic info (name, role, context), OCEAN personality scores, motivations, cognitive biases, reputation dimensions, and behavioral patterns.\n\nEach section captures a different facet of someone's personality — fill in what you know, leave the rest blank."
        }
        Key::TutCreateTitle => "Creating a Person",
        Key::TutDoneBody => {
            "You can replay this tutorial anytime from the navigation bar.\n\nQuick tips:\n• Create at least two people to unlock comparisons\n• Use the Sync page to back up your data\n• Tag people to organise them by group\n\nGo ahead and start modeling the people in your world!"
        }
        Key::TutDoneTitle => "You're Ready!",
        Key::TutMotBiasBody => {
            "Motivations capture what drives a person — their goals, fears, and values (Achievement, Power, Affiliation, Security, Autonomy, etc.).\n\nBiases represent mental shortcuts that shape their decisions (Confirmation bias, Anchoring, Overconfidence, etc.). Together they give you a deeper understanding of why people act the way they do."
        }
        Key::TutMotBiasTitle => "Motivations & Biases",
        Key::TutOceanBody => {
            "OCEAN measures personality across five dimensions from 1 to 10:\n• Openness — curiosity vs. caution\n• Conscientiousness — organization vs. flexibility\n• Extraversion — sociability vs. solitude\n• Agreeableness — cooperation vs. competition\n• Neuroticism — sensitivity vs. emotional stability\n\nThese scores power the comparison engine and help predict behaviour."
        }
        Key::TutOceanTitle => "OCEAN Model (Big Five)",
        Key::TutPeopleBody => {
            "The main page shows everyone you've created. Use the search bar to find someone, sort by name / recent / OCEAN score, and click the + button to add someone new."
        }
        Key::TutPeopleTitle => "Your People",
        Key::TutRepPatternBody => {
            "Reputation scores capture how others perceive this person across bipolar scales (hardworking vs. lazy, honest vs. deceitful, etc.).\n\nBehavioral patterns let you record how they typically react to specific triggers (stress, criticism, success, conflict, etc.). This helps anticipate their responses in future situations."
        }
        Key::TutRepPatternTitle => "Reputation & Patterns",
        Key::TutStep => "Step",
        Key::TutWelcomeBody => {
            "This app helps you model and understand the people in your life using personality frameworks like OCEAN (Big Five), motivations, cognitive biases, and behavioral patterns.\n\nYou can compare people side by side, track predictions over time, map relationships, and explore synergy scores."
        }
        Key::TutWelcomeTitle => "Welcome to PeopleModeler!",
        Key::ValuesTitle => "Values",
        Key::ValueIntensityHelper => "Intensity (I): how strongly they hold this value.",
        Key::ValuePriorityHelper => "Priority (P): importance relative to their other values.",
        Key::PersonaSection => "Work Persona",
        Key::PersonaHint => {
            "Define a different work persona. OCEAN and reputation scores can diverge from the base profile."
        }
        Key::PersonaCopyBase => "Copy from base profile",
        Key::PersonaClear => "Clear mask",
        Key::PersonaBalanceTitle => "Resilience & Risk Appetite",
        Key::PersonaOnlineSection => "Online Persona",
        Key::PersonaBaseSection => "Personal life Persona",
        Key::FacetBase => "Personal life",
        Key::FacetWork => "At work",
        Key::FacetOnline => "Online",
        Key::FacetAuto => "Automatic",
        Key::FacetMain => "Main",
        Key::FacetMainSuffix => " (main)",
        Key::MainContextLabel => "Main context:",
        Key::PersonaContextPrompt => "Where do you know this person from?",
        Key::PersonaContextChange => "Change context",
        Key::MaskGapLow => "No mask",
        Key::MaskGapModerate => "Moderate mask",
        Key::MaskGapHigh => "Strong mask",
        Key::TeamFacetToggle => "Filter by facet",
        Key::OceanVolatility => "OCEAN volatility",
        Key::RepPowerStruggle => "Power struggle (Reputation)",
        Key::OnlyNegativePatterns => "Only negative patterns",
        Key::LowPredictionAccuracy => "Low prediction accuracy",
        Key::Unknown => "Unknown",
    }
}

fn fr(key: Key) -> &'static str {
    match key {
        Key::AddBtn => "＋",
        Key::AriaAddBias => "Ajouter un biais",
        Key::AriaAddMotivation => "Ajouter une motivation",
        Key::AriaAddPattern => "Ajouter un pattern",
        Key::AriaAddStyle => "Ajouter un style",
        Key::AriaAddValue => "Ajouter une valeur",
        Key::AriaAvatarPrefix => "Avatar",
        Key::AriaDeleteBias => "Supprimer le biais",
        Key::AriaDeleteLogEntry => "Supprimer l'entrée du journal",
        Key::AriaDeleteMotivation => "Supprimer la motivation",
        Key::AriaDeletePattern => "Supprimer le pattern",
        Key::AriaDeleteStyle => "Supprimer le style",
        Key::AriaDeleteValue => "Supprimer la valeur",
        Key::AriaDiscardPrefix => "Annuler",
        Key::AriaEditBias => "Modifier le biais",
        Key::AriaEditMotivation => "Modifier la motivation",
        Key::AriaEditPattern => "Modifier le pattern",
        Key::AriaEditStyle => "Modifier le style",
        Key::AriaEditValue => "Modifier la valeur",
        Key::AriaMoveBiasDown => "Descendre le biais",
        Key::AriaMoveBiasUp => "Monter le biais",
        Key::AriaMoveMotivationDown => "Descendre la motivation",
        Key::AriaMoveMotivationUp => "Monter la motivation",
        Key::AriaMovePatternDown => "Descendre le pattern",
        Key::AriaMovePatternUp => "Monter le pattern",
        Key::AriaMoveStyleDown => "Descendre le style",
        Key::AriaMoveStyleUp => "Monter le style",
        Key::AriaMoveValueDown => "Descendre la valeur",
        Key::AriaMoveValueUp => "Monter la valeur",
        Key::AriaSkipToContent => "Aller au contenu",
        Key::AriaUpdateBias => "Modifier le biais",
        Key::AriaUpdateMotivation => "Modifier la motivation",
        Key::AriaUpdatePattern => "Modifier le pattern",
        Key::AriaUpdateStyle => "Modifier le style",
        Key::AriaUpdateValue => "Modifier la valeur",
        Key::BiasScaleHint => {
            "0 = ce biais est absent, 10 = il influence la plupart des décisions."
        }
        Key::BiasUndefinedWarning => {
            "Les biais non définis comptent comme présents. Mettez 0 pour les marquer absents."
        }
        Key::BiasesTitle => "Biais",
        Key::BucketInheritsBase => "Hérite de la base",
        Key::BucketOverride => "Remplacer",
        Key::CommonAdd => "Ajouter",
        Key::CommonBack => "← Retour",
        Key::CommonCancel => "Annuler",
        Key::CommonDelete => "Supprimer",
        Key::CommonEdit => "Modifier",
        Key::CommonFinish => "Terminer",
        Key::CommonNext => "Suivant →",
        Key::CommonSave => "Enregistrer",
        Key::CommonSkip => "Passer",
        Key::CompareAnalysisTitle => "Analyse dynamique",
        Key::CompareAsymmetric => "Bénéfice mutuel",
        Key::CompareBalanced => "Équilibré",
        Key::CompareBandHint => "±{}% (relation + fiabilité du profil)",
        Key::CompareBenefitMore => "bénéficie plus",
        Key::CompareBiasMain => "Biais principal",
        Key::CompareBreakdown => "Détail",
        Key::CompareBtn => "Comparer",
        Key::CompareCatBias => "Biais",
        Key::CompareCatMotivation => "Motivation",
        Key::CompareCatOcean => "OCÉAN",
        Key::CompareCatPatterns => "Patterns",
        Key::CompareCatReputation => "Réputation",
        Key::CompareCatStyles => "Styles",
        Key::CompareCatValues => "Valeurs",
        Key::CompareCtxTitle => "Par situation",
        Key::CompareEthics => {
            "Ce sont des modèles probabilistes, pas des vérités absolues. Utilisez-les pour mieux comprendre, jamais pour manipuler."
        }
        Key::CompareFacetUnavailable => "Non calculable dans ce contexte — un persona manque",
        Key::CompareFriction => "Points de friction",
        Key::CompareOcean => "Profil OCEAN",
        Key::CompareRelNone => "Général (sans contexte)",
        Key::CompareRelStrength => "Intensité",
        Key::CompareRelTitle => "Contexte de relation",
        Key::CompareRiskMitigation => "Risques & Mitigations",
        Key::CompareStrategy => "Stratégie d'interaction",
        Key::CompareSub => "Identifiez synergies et points de friction entre deux personnes",
        Key::CompareSynergies => "Synergies",
        Key::CompareTitle => "Comparer des personnes",
        Key::CompareTopMot => "Motivation principale",
        Key::CompareVs => "VS",
        Key::ConfidenceHint => {
            "À quel point ce profil est fiable ? 1 = ébauche, 10 = fondé sur des observations réelles."
        }
        Key::ConfidenceLabel => "Fiabilité du profil",
        Key::ConfirmDelete => "Supprimer cette personne ?",
        Key::ConfirmDeleteLog => "Supprimer cette entrée ?",
        Key::ConfirmDeletePred => "Supprimer cette prédiction ?",
        Key::ConfirmDeleteTeam => "Supprimer cette équipe ?",
        Key::CtxChange => "Changement",
        Key::CtxCommunication => "Communication",
        Key::CtxConflict => "Conflit",
        Key::CtxDecision => "Décision",
        Key::CtxFeedback => "Feedback",
        Key::CtxGrowth => "Croissance",
        Key::CtxInjustice => "Injustice",
        Key::CtxLeadership => "Leadership",
        Key::CtxRecognition => "Reconnaissance",
        Key::CtxStress => "Stress",
        Key::CtxSuccess => "Réussite",
        Key::CtxTeam => "Équipe",
        Key::CtxThreatened => "Menacé",
        Key::CtxUncertainty => "Incertitude",
        Key::DeleteBtn => "🗑 Supprimer",
        Key::EditBiases => "Biais",
        Key::EditBtn => "✏ Modifier",
        Key::EditDiscardSection => "Rétablir",
        Key::EditEvidencePlaceholder => "Preuve",
        Key::EditMotivations => "Motivations",
        Key::EditNotesPlaceholder => "Notes",
        Key::EditPatterns => "Patterns comportementaux",
        Key::EditPriority => "P",
        Key::EditReputation => "Réputation",
        Key::EditStyles => "Styles personnels",
        Key::EditUpdateBtn => "💾",
        Key::EditValues => "Valeurs",
        Key::FlagAffiliationCold => {
            "Revendique la proximité mais est perçu comme froid et distant — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagAffiliationDistrustful => {
            "Revendique la proximité mais est perçu comme méfiant — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagAmbitionLazy => {
            "Aspire au pouvoir, au succès ou à la reconnaissance mais est perçu comme paresseux — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagAnchoringOpen => {
            "Se dit ouvert d'esprit mais s'accroche aux premières impressions."
        }
        Key::FlagAuthorityDominant => {
            "Perçu comme un leader mais se soumet aveuglément à l'autorité."
        }
        Key::FlagAutonomySubmissive => {
            "Prêche l'indépendance mais est perçu comme soumis — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagAvailabilityCalm => {
            "Perçu comme imperturbable mais surpondère les événements dramatiques."
        }
        Key::FlagBiasConfirmationOpen => {
            "Se dit ouvert d'esprit mais ne cherche que des informations qui confirment ses vues — ne se connaît pas."
        }
        Key::FlagBiasFavoritismFairness => {
            "Prêche l'équité mais montre un biais de favoritisme ou de groupe — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagCalmNeurotic => {
            "Décrit comme calme sous pression mais l'OCEAN indique une forte réactivité — à vérifier."
        }
        Key::FlagClaimsCalmReactive => {
            "Se prétend calme et stable mais est perçu comme réactif — ne se connaît pas."
        }
        Key::FlagCreativityClosed => {
            "Prêche la créativité mais se dit peu ouvert à la nouveauté — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagCreativityRigid => {
            "Prêche la créativité mais est perçu comme rigide — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagDisciplineFlaky => {
            "Se voit discipliné mais paraît inconstant — ne se connaît pas."
        }
        Key::FlagDisciplineLazy => {
            "Image de soi disciplinée contredite par une réputation de paresse — ne se connaît pas."
        }
        Key::FlagDunningKrugerHumble => "Surestime ses compétences mais paraît humble.",
        Key::FlagFairnessRhetoric => {
            "Parle d'équité et de justice mais pratique le favoritisme — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagHelpingCold => "Prêche l'aide aux autres mais paraît émotionnellement froid.",
        Key::FlagHelpingSelfish => {
            "Prêche l'entraide mais est perçu comme égoïste — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagHighELowA => {
            "Très extraverti mais faible agréabilité — peut être assertif jusqu'à l'abrasivité."
        }
        Key::FlagHighNLowC => {
            "Réactivité émotionnelle élevée avec faible conscience — peut avoir du mal sous stress."
        }
        Key::FlagHighOLowC => {
            "Très créatif mais désorganisé — beaucoup d'idées mais difficulté à les concrétiser."
        }
        Key::FlagHonestFavoritist => {
            "Honnêteté de principe associée à un favoritisme perçu — l'équité ne vaut peut-être que pour certains."
        }
        Key::FlagHonestSelfish => {
            "Honnêteté de principe associée à une faible générosité — peut indiquer une position morale rigide."
        }
        Key::FlagImpostorArrogant => "Sous-estime ses compétences mais paraît arrogant.",
        Key::FlagLearningArrogant => {
            "Prêche la croissance mais est trop arrogant pour écouter les conseils."
        }
        Key::FlagLearningRigid => {
            "Prêche l'apprentissage et la croissance mais est perçu comme rigide — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagLossAversionRisky => "Se dit amateur de risque mais est averse à la perte.",
        Key::FlagOpenRigid => "Se croit ouvert d'esprit mais paraît rigide — ne se connaît pas.",
        Key::FlagPatternAchievementComplacent => {
            "Aspire à la réussite mais les patterns se reposent sur les lauriers."
        }
        Key::FlagPatternAssertiveQuiet => "Perçu comme affirmé mais se tait quand il le faut.",
        Key::FlagPatternCalmVolatile => {
            "Perçu comme calme sous pression, mais les schémas enregistrés montrent de la volatilité — ce calme n'est peut-être qu'un masque."
        }
        Key::FlagPatternClaimedCalmVolatile => {
            "Se dit calme mais les patterns montrent de la volatilité."
        }
        Key::FlagPatternDiplomatEscalator => "Perçu comme diplomate mais escalade les conflits.",
        Key::FlagPatternDisciplineShirker => {
            "Image de discipline mais les patterns esquivent les responsabilités."
        }
        Key::FlagPatternEmpathDismissive => "Perçu comme empathique mais rabaisse les autres.",
        Key::FlagPatternExtravertQuiet => "Image d'extraversion mais les patterns se taisent.",
        Key::FlagPatternFairExploiter => {
            "Perçu comme équitable mais exploite l'injustice à son profit."
        }
        Key::FlagPatternFairnessExploiter => {
            "Prêche l'équité mais les patterns exploitent l'injustice."
        }
        Key::FlagPatternFlexibleResister => {
            "Perçu comme flexible mais résiste au changement et au feedback."
        }
        Key::FlagPatternGenerousExploiter => "Perçu comme généreux mais exploite les autres.",
        Key::FlagPatternHardworkerComplacent => {
            "Perçu comme travailleur mais se repose sur ses lauriers."
        }
        Key::FlagPatternHelpingExploiter => {
            "Prêche l'aide aux autres mais les patterns montrent l'exploitation."
        }
        Key::FlagPatternHonestExploiter => {
            "Perçu comme honnête, mais les schémas montrent de l'exploitation ou des rejets de responsabilité — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagPatternHumbleDismissive => "Perçu comme humble mais rabaisse les autres.",
        Key::FlagPatternLearningResister => {
            "Prêche l'apprentissage mais les patterns rejettent le feedback."
        }
        Key::FlagPatternOpenResister => "Se dit ouvert mais les patterns résistent au changement.",
        Key::FlagPatternPassiveBlowup => "Perçu comme passif mais explose sous la pression.",
        Key::FlagPatternRecognitionDismissive => {
            "Cherche la reconnaissance mais rabaisse les autres pour la gagner."
        }
        Key::FlagPatternReliableShirker => "Perçu comme fiable mais esquive ses responsabilités.",
        Key::FlagPatternTrustingParanoid => {
            "Perçu comme confiant mais devient paranoïaque sous la menace."
        }
        Key::FlagPatternWarmthDismissive => {
            "Image de chaleur mais les patterns rabaissent les autres."
        }
        Key::FlagPowerPassive => "Aspire au pouvoir mais est perçu comme une carpette.",
        Key::FlagRecencyReliable => "Perçu comme stable mais ballotté par l'actualité.",
        Key::FlagResilientHides => "Admet sa fragilité mais paraît imperturbable — il la cache.",
        Key::FlagResilientReactive => {
            "Se dit très résilient mais est perçu comme réactif — ne se connaît pas."
        }
        Key::FlagRiskAppetiteAmbition => {
            "Aspire au pouvoir ou à la réussite mais évite tout risque."
        }
        Key::FlagSecurityGullible => {
            "Revendique un besoin de sécurité mais est perçu comme naïvement confiant — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagSecurityRisky => {
            "Prêche la prudence et la sécurité mais déclare aimer le risque — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagSocialProofOpen => {
            "Se dit indépendant d'esprit mais suit le troupeau — fait ce que je dis, pas ce que je fais."
        }
        Key::FlagStyleCompetingPassive => "Se dit compétitif mais passe pour passif.",
        Key::FlagStyleConsensusAuthoritative => {
            "Se dit axé consensus mais passe pour un dictateur."
        }
        Key::FlagStyleControlling => {
            "Contrôle et micro-gère — perçu comme dominateur, pas confiant."
        }
        Key::FlagStyleDetached => "S'avoue détaché et passe pour froid/distant.",
        Key::FlagStyleDiplomaticBlunt => "Se dit diplomate mais passe pour brutal.",
        Key::FlagStyleDirectDiplomatic => "Se dit direct mais passe pour un diplomate.",
        Key::FlagStyleDominantSubmissive => "Se dit autocratique mais passe pour soumis.",
        Key::FlagStyleEmpatheticCold => "Se dit empathique mais paraît froid.",
        Key::FlagStyleGuardedTrusting => "Se dit méfiant mais paraît confiant.",
        Key::FlagStyleManipulative => "Adopte un style manipulateur et passe pour malhonnête.",
        Key::FlagStyleManipulativeHonest => "Se dit roublard mais passe pour honnête.",
        Key::FlagStylePassiveAggressive => "S'avoue passif-agressif et passe pour réactif.",
        Key::FlagStyleRepairsTrustDeceitful => {
            "Se dit réparateur de confiance mais passe pour trompeur."
        }
        Key::FlagStyleRulebasedFavoritist => "Se dit basé sur des règles mais joue les favoris.",
        Key::FlagStyleServantAuthoritative => {
            "Se dit leader serviteur mais passe pour un commandant."
        }
        Key::FlagStyleTrustsFreelySuspicious => "Se dit confiant mais passe pour méfiant.",
        Key::FlagStyleVirtuebasedDeceitful => "Se dit basé sur la vertu mais passe pour trompeur.",
        Key::FlagSunkCostFlexible => {
            "Perçu comme flexible mais s'accroche aux coûts irrécupérables."
        }
        Key::FlagValueAdventureStability => {
            "Valorise l'aventure et la stabilité à la fois — forces opposées."
        }
        Key::FlagValueCareerFamily => {
            "Carrière et famille tous deux en priorité — attendez-vous à des tensions."
        }
        Key::FlagValueCommunitySelfish => {
            "Valorise la communauté mais passe pour égoïste — contradictoire."
        }
        Key::FlagValueFaithDeceitful => {
            "Valorise la foi mais passe pour malhonnête — contradictoire."
        }
        Key::FlagValueFamilyFuture => {
            "Valorise la famille mais décide avec une orientation tournée vers l'avenir."
        }
        Key::FlagValueHealthRisky => {
            "Valorise la santé mais a un appétit de risque très élevé — contradictoire."
        }
        Key::FlagValueKnowledgeArrogant => {
            "Valorise le savoir mais passe pour arrogant — contradictoire."
        }
        Key::FlagValueLoyaltyGuarded => {
            "Valorise la loyauté mais adopte un style de confiance défiant."
        }
        Key::FlagValueStabilityRisk => {
            "Aspire à la stabilité mais a un très fort appétit pour le risque — contradictoire."
        }
        Key::FlagValueWealthGenerous => {
            "Valorise la richesse mais passe pour généreux — contradictoire."
        }
        Key::FlagWarmthBlunt => {
            "Image de soi chaleureuse contredite par une réputation de franchise brutale — ne se connaît pas."
        }
        Key::FlagWarmthCold => "Se croit chaleureux mais paraît froid — ne se connaît pas.",
        Key::FlagWarmthSelfish => "Se dit chaleureux mais est perçu comme égoïste.",
        Key::FormAvatar => "Avatar",
        Key::FormCancel => "Annuler",
        Key::FormConfidence => "Fiabilité du profil (1-10)",
        Key::FormContext => "Contexte",
        Key::FormEditTitle => "Modifier la personne",
        Key::FormName => "Nom",
        Key::FormNewTitle => "Nouvelle personne",
        Key::FormNotes => "Notes",
        Key::FormOceanTitle => "Scores OCEAN (1-10)",
        Key::FormResilience => "Résilience (1-10)",
        Key::FormRiskAppetite => "Appétence pour le risque (1-10)",
        Key::FormRole => "Rôle",
        Key::FormSave => "💾 Enregistrer",
        Key::FormTags => "Tags (séparés par des virgules)",
        Key::InsightsObserved => "Patterns observés",
        Key::InsightsSelectPerson => {
            "Sélectionnez une personne pour voir les analyses comportementales."
        }
        Key::InsightsTitle => "📊 Analyses",
        Key::LogAdd => "Ajouter",
        Key::LogEmpty => "Aucune entrée.",
        Key::LogNoTarget => "Note perso (sans cible)",
        Key::LogNoTrigger => "Aucun déclencheur",
        Key::LogPlaceholder => "Que s'est-il passé ?",
        Key::LogTarget => "Avec",
        Key::LogTitle => "📋 Journal",
        Key::LogTrigger => "Déclencheur",
        Key::LogValence => "Valence",
        Key::MoreRecs => "Plus de recommandations",
        Key::MotUndefinedWarning => {
            "Moins de 3 motivations pénalise (−0.03 chaque). L'absence de Justice/Aide aussi."
        }
        Key::MotivationsTitle => "Motivations",
        Key::NavPeople => "Personnes",
        Key::NavRelationships => "Relations",
        Key::NavSync => "Sync",
        Key::NavTeams => "Équipes",
        Key::NavTimeline => "Chrono",
        Key::NoBiases => "Aucun biais enregistré.",
        Key::NoMotivations => "Aucune motivation enregistrée.",
        Key::NoPatterns => "Aucun pattern comportemental enregistré.",
        Key::NoPeopleInsights => {
            "Aucune personne encore. Ajoutez quelqu'un pour voir les analyses."
        }
        Key::NoPeopleYet => "Aucune personne. Appuyez sur + pour ajouter.",
        Key::NoReputation => "Aucun trait de réputation enregistré.",
        Key::NoSearchResults => "Aucun résultat pour «{0}».",
        Key::NoValues => "Aucune valeur définie",
        Key::OceanA => "A — Agréabilité",
        Key::OceanAHigh => "coopératif, empathique, cherche l'harmonie",
        Key::OceanALow => "direct voire abrasif, met ses objectifs avant les relations",
        Key::OceanAgreeableness => "Agréabilité",
        Key::OceanC => "C — Conscienciosité",
        Key::OceanCHigh => "organisé, fiable, orienté résultats et détails",
        Key::OceanCLow => "flexible et spontané, peut manquer de rigueur",
        Key::OceanConscientiousness => "Conscienciosité",
        Key::OceanE => "E — Extraversion",
        Key::OceanEHigh => "extraverti, énergique, cherche la stimulation sociale",
        Key::OceanELow => "introverti, réfléchi, préfère les interactions limitées",
        Key::OceanExtraversion => "Extraversion",
        Key::OceanN => "N — Névrosisme",
        Key::OceanNHigh => "émotionnellement réactif, stressable, sensible aux critiques",
        Key::OceanNLow => "stable émotionnellement, calme sous pression",
        Key::OceanNeuroticism => "Névrosisme",
        Key::OceanO => "O — Ouverture",
        Key::OceanOHigh => "très ouvert aux nouvelles idées, créatif et curieux",
        Key::OceanOLow => "pragmatique, préfère les routines et le concret",
        Key::OceanOpenness => "Ouverture",
        Key::OceanTitle => "Scores OCEAN",
        Key::PatternHelperChange => {
            "Comment il s'adapte aux transitions et nouveautés — au travail : une réorganisation ou un nouveau poste ; dans la vie : un déménagement ou une nouvelle routine"
        }
        Key::PatternHelperConflict => {
            "Comment il gère les désaccords et confrontations — au travail : un clash en réunion ; dans la vie : une dispute familiale"
        }
        Key::PatternHelperFeedback => {
            "Comment il reçoit et traite les retours des autres — au travail : le bilan d'un projet ; dans la vie : un ami qui signale un angle mort"
        }
        Key::PatternHelperInjustice => {
            "Comment il réagit face à l'injustice ou au traitement inéquitable — au travail : une promotion injustement ignorée ; dans la vie : voir quelqu'un être maltraité"
        }
        Key::PatternHelperRecognition => {
            "Comment il cherche et réagit à la reconnaissance — au travail : les éloges d'un supérieur ; dans la vie : l'appréciation de ses amis"
        }
        Key::PatternHelperStress => {
            "Comment il réagit sous pression ou délais serrés — au travail : une échéance imminente ; dans la vie : une journée surchargée"
        }
        Key::PatternHelperSuccess => {
            "Comment il répond aux réussites et victoires — au travail : la conclusion d'un gros contrat ; dans la vie : l'aboutissement d'un projet personnel"
        }
        Key::PatternHelperThreat => {
            "Comment il se défend quand il se sent attaqué — au travail : critiqué lors d'un entretien ; dans la vie : pris à partie dans une discussion vive"
        }
        Key::PatternHelperUncertainty => {
            "Comment il navigue l'ambiguïté et l'incertain — au travail : un projet au périmètre flou ; dans la vie : l'attente d'un résultat incertain"
        }
        Key::PatternsTitle => "Patterns comportementaux",
        Key::PersonNotFound => "Personne introuvable",
        Key::PersonSelfScore => "Score de profil",
        Key::PlName => "Nom",
        Key::PredAccuracyLabel => "Précision",
        Key::PredActualLabel => "Réel",
        Key::PredActualPlaceholder => "Résultat réel...",
        Key::PredAddBtn => "Ajouter",
        Key::PredAllTitle => "Toutes les prédictions",
        Key::PredCancelBtn => "Annuler",
        Key::PredContextPlaceholder => "Contexte...",
        Key::PredDeleteBtn => "Supprimer",
        Key::PredFor => "🔮 Prédictions pour",
        Key::PredNone => "Aucune prédiction.",
        Key::PredOutcomePlaceholder => "Comportement prédit...",
        Key::PredPredictedLabel => "Prédit",
        Key::PredResolveBtn => "Résoudre",
        Key::PredResolveSubmit => "✓ Résoudre",
        Key::PredTitle => "Prédictions",
        Key::ProfileCompleteness => "Compl.",
        Key::RelCloseAdd => "− Annuler",
        Key::RelConfirmDelete => "Supprimer cette relation ?",
        Key::RelNone => "Aucune relation.",
        Key::RelNotes => "Notes",
        Key::RelOpenAdd => "＋ Ajouter",
        Key::RelPersonRel => "Relations",
        Key::RelSearchPlaceholder => "Rechercher une personne…",
        Key::RelStrength => "Intensité",
        Key::RelTitle => "Relations",
        Key::ReliabilityTitle => "Qualité des données",
        Key::RepScaleHint => {
            "0 = pôle négatif, 10 = pôle positif — la case ✗ laisse la dimension inconnue."
        }
        Key::RepUndefinedWarning => {
            "Les traits non définis pénalisent la réputation. Les valeurs extrêmes (≤2 ou ≥8) déclenchent des ajustements."
        }
        Key::ReputationTitle => "Réputation",
        Key::ResilienceLabel => "Résilience",
        Key::RiskAppetiteLabel => "Appétence risque",
        Key::ScaleFriction => "Friction",
        Key::ScaleGood => "Bon",
        Key::ScaleModerate => "Moyen",
        Key::ScaleStrong => "Fort",
        Key::ScaleTension => "Tension",
        Key::ScoreBand => "±{}",
        Key::SearchPlaceholder => "Rechercher...",
        Key::StrategyChangeDisciplineRhetoric => {
            "Il se voit discipliné mais est perçu comme paresseux — ne faites pas appel à son image organisée ; vérifiez la production réelle."
        }
        Key::StrategyChangeFallback => "Expliquez le pourquoi et impliquez-les dans la transition.",
        Key::StrategyChangeHighC => {
            "Haute conscienciosité — a besoin d'une feuille de route claire."
        }
        Key::StrategyChangeHighN => {
            "Névrosisme élevé — peut résister au changement ; offrez des points d'ancrage."
        }
        Key::StrategyChangeHighO => {
            "Haute ouverture — embrasse le changement ; donnez-lui un rôle actif."
        }
        Key::StrategyChangeLabel => "Face au changement",
        Key::StrategyChangeLowE => {
            "Faible extraversion — a besoin de temps pour digérer le changement en privé."
        }
        Key::StrategyChangeLowN => {
            "Faible névrosisme — s'adapte bien ; exploitez comme champion du changement."
        }
        Key::StrategyConflictAffiliationRhetoric => {
            "Il revendique la proximité mais paraît froid — ne faites pas appel à son besoin déclaré de connexion ; traitez directement la distance."
        }
        Key::StrategyConflictAffiliationTrustRhetoric => {
            "Il revendique la proximité mais se montre méfiant — ne faites pas appel à son besoin déclaré de connexion ; gagnez sa confiance avant de chercher la complicité."
        }
        Key::StrategyConflictFallback => "Médiateur avec une communication équilibrée.",
        Key::StrategyConflictHighA => {
            "Haute agréabilité — adoucissez la confrontation, concentrez-vous sur l'harmonie."
        }
        Key::StrategyConflictHighC => {
            "Haute conscienciosité — peut insister rigidement sur les règles."
        }
        Key::StrategyConflictHighE => "Extraversion élevée — laissez-les parler pour évacuer.",
        Key::StrategyConflictHighN => {
            "Névrosisme élevé — désamorcez et offrez un espace de sécurité émotionnelle."
        }
        Key::StrategyConflictLabel => "En conflit",
        Key::StrategyConflictLowA => {
            "Faible agréabilité — abordez le conflit directement avec des faits."
        }
        Key::StrategyConflictLowE => "Faible extraversion — peut se retirer au lieu de s'engager.",
        Key::StrategyFeedbackFallback => {
            "Équilibrez éloges et critiques constructives avec des exemples précis."
        }
        Key::StrategyFeedbackHelpingRhetoric => {
            "Il prêche l'entraide mais est perçu comme égoïste — ne formulez pas le retour autour de l'aide aux autres ; nommez l'intérêt personnel derrière le conseil."
        }
        Key::StrategyFeedbackHighC => {
            "Haute conscienciosité — apprécie un feedback détaillé et actionnable."
        }
        Key::StrategyFeedbackHighN => {
            "Névrosisme élevé — peut prendre le feedback personnellement ; utilisez un ton doux."
        }
        Key::StrategyFeedbackLabel => "Recevoir du feedback",
        Key::StrategyFeedbackLowA => {
            "Faible agréabilité — peut rejeter le feedback ; basez-vous sur des faits."
        }
        Key::StrategyFeedbackLowE => "Faible extraversion — préfère un feedback écrit et privé.",
        Key::StrategyFeedbackLowN => "Faible névrosisme — gère bien les critiques ; soyez direct.",
        Key::StrategyFeedbackWarmthRhetoric => {
            "Il se voit chaleureux mais est perçu comme brutal — ne comptez pas sur un ton doux ; soyez clair et précis sur le comportement."
        }
        Key::StrategyInjusticeAmbitionRhetoric => {
            "Il parle d'ambition mais est perçu comme paresseux — n'attendez pas qu'il se batte pour la cause ; présentez l'issue comme servant son statut."
        }
        Key::StrategyInjusticeFairness => {
            "Motivé par l'équité — se battra pour ce qu'il croit juste, même à titre personnel."
        }
        Key::StrategyInjusticeFairnessRhetoric => {
            "Parle d'équité mais agit avec favoritisme — ne faites pas appel à son discours sur la justice ; adressez-vous au vrai moteur."
        }
        Key::StrategyInjusticeFallback => {
            "Reconnaissez leur préoccupation et clarifiez la voie vers la résolution."
        }
        Key::StrategyInjusticeHighA => {
            "Haute agréabilité — peut se sentir personnellement blessé par l'injustice."
        }
        Key::StrategyInjusticeHighN => {
            "Névrosisme élevé — peut ruminer et amplifier les affronts perçus."
        }
        Key::StrategyInjusticeLabel => "Face à l'injustice",
        Key::StrategyInjusticePower => {
            "Motivé par le pouvoir — peut utiliser son autorité pour corriger le tort perçu."
        }
        Key::StrategyRecognitionFallback => {
            "Adaptez le style de reconnaissance à leur niveau de confort."
        }
        Key::StrategyRecognitionHigh => {
            "Fort besoin de reconnaissance — donnez des éloges fréquents et spécifiques."
        }
        Key::StrategyRecognitionHighE => {
            "Extraversion élevée — la reconnaissance publique est efficace."
        }
        Key::StrategyRecognitionLabel => "Cherchant la reconnaissance",
        Key::StrategyRecognitionLow => {
            "Faible besoin de reconnaissance — évitez les éloges excessifs."
        }
        Key::StrategyRecognitionLowE => {
            "Faible extraversion — préférez une reconnaissance privée et écrite."
        }
        Key::StrategyRecognitionMid => {
            "Besoin modéré de reconnaissance — reconnaissez les contributions régulièrement."
        }
        Key::StrategyStressAmbitionRhetoric => {
            "Il parle d'ambition mais est perçu comme paresseux — ne récompensez pas le discours ; concentrez-vous sur l'effort et la concrétisation."
        }
        Key::StrategyStressFallback => {
            "Surveillez les signaux de stress et ajustez l'environnement."
        }
        Key::StrategyStressHighC => {
            "Conscienciosité élevée — décomposez les problèmes en étapes actionnables."
        }
        Key::StrategyStressHighE => {
            "Extraversion élevée — permettez l'expression verbale du stress."
        }
        Key::StrategyStressHighN => {
            "Névrosisme élevé — offrez du soutien émotionnel avant les solutions."
        }
        Key::StrategyStressHighO => "Haute ouverture — peut trop réfléchir et imaginer le pire.",
        Key::StrategyStressLabel => "Sous stress",
        Key::StrategyStressLowA => "Faible agréabilité — peut devenir irritable sous pression.",
        Key::StrategyStressLowC => "Faible conscienciosité — peut devenir désorganisé ou éviter.",
        Key::StrategyStressLowE => "Faible extraversion — laissez de l'espace pour décompresser.",
        Key::StrategyStressPower => {
            "Motivé par le pouvoir — laissez-lui reprendre le contrôle sur un domaine."
        }
        Key::StrategyStressSecurity => {
            "Motivé par la sécurité — renforcez la stabilité et la routine."
        }
        Key::StrategyStressSecurityRhetoric => {
            "Il revendique la sécurité mais est naïvement confiant — ne vous fiez pas à sa prudence affichée ; vérifiez vous-même les garde-fous."
        }
        Key::StrategySuccessAmbitionRhetoric => {
            "Il parle d'ambition mais est perçu comme paresseux — ne célébrez pas ses plans ; exigez des résultats."
        }
        Key::StrategySuccessFallback => "Célébrez le succès et identifiez les axes de croissance.",
        Key::StrategySuccessHighA => {
            "Haute agréabilité — peut détourner le crédit pour éviter de se démarquer."
        }
        Key::StrategySuccessHighC => {
            "Haute conscienciosité — utilisez le succès comme validation du processus."
        }
        Key::StrategySuccessHighO => {
            "Haute ouverture — canalisez le succès vers de nouveaux défis créatifs."
        }
        Key::StrategySuccessLabel => "En réussite",
        Key::StrategySuccessLowE => {
            "Faible extraversion — peut se sentir submergé par l'attention publique."
        }
        Key::StrategySuccessPower => {
            "Motivé par le pouvoir — donnez-leur la propriété de la prochaine initiative."
        }
        Key::StrategySuccessRecognition => {
            "Motivé par la reconnaissance — reconnaissez publiquement leur accomplissement."
        }
        Key::StrategyThreatFallback => "Écoutez activement et validez ses préoccupations.",
        Key::StrategyThreatHighA => {
            "Haute agréabilité — peut céder trop facilement ; vérifiez les vrais sentiments."
        }
        Key::StrategyThreatHighN => {
            "Névrosisme élevé — les menaces perçues sont amplifiées ; offrez du réconfort."
        }
        Key::StrategyThreatLabel => "Se sentant menacé",
        Key::StrategyThreatLowA => {
            "Faible agréabilité — peut réagir ; abordez les préoccupations calmement."
        }
        Key::StrategyThreatPower => {
            "Motivé par le pouvoir — la menace au statut est sérieuse ; impliquez-le dans les décisions."
        }
        Key::StrategyUncertaintyFallback => {
            "Reconnaissez l'incertitude et fournissez les informations disponibles."
        }
        Key::StrategyUncertaintyHighC => {
            "Haute conscienciosité — a besoin d'un plan concret immédiatement."
        }
        Key::StrategyUncertaintyHighE => {
            "Haute extraversion — peut trop socialiser pour gérer l'ambiguïté."
        }
        Key::StrategyUncertaintyHighN => {
            "Névrosisme élevé — fournissez des échéances claires et des mises à jour fréquentes."
        }
        Key::StrategyUncertaintyHighO => {
            "Haute ouverture — cadrez l'incertitude comme une opportunité."
        }
        Key::StrategyUncertaintyLabel => "Dans l'incertitude",
        Key::StrategyUncertaintyLowN => {
            "Faible névrosisme — gère bien l'ambiguïté ; faites confiance à sa résilience."
        }
        Key::StrategyUncertaintyLowO => {
            "Faible ouverture — fournissez des exemples concrets et des cadres familiers."
        }
        Key::StrategyWhen => "Quand {name} est {trigger} :\n\n{advice}",
        Key::StyleNoStyles => "Aucun style personnel enregistré.",
        Key::StylePanelTitle => "Styles personnels",
        Key::SyncBackedUp => "✅ Sauvegardé",
        Key::SyncBackingUp => "Sauvegarde en cours...",
        Key::SyncBackupBtn => "☁ Sauvegarder sur Drive",
        Key::SyncClearBtn => "Effacer",
        Key::SyncExportBtn => "📥 Exporter JSON",
        Key::SyncExported => "✅ Exporté",
        Key::SyncGdriveTitle => "Synchronisation Google Drive",
        Key::SyncImportBtn => "📤 Importer JSON",
        Key::SyncLastBackedUp => "Dernière sauvegarde : ",
        Key::SyncLocalDesc => {
            "Exportez toutes les données en JSON ou importez depuis une sauvegarde."
        }
        Key::SyncLocalTitle => "Sauvegarde locale",
        Key::SyncNetworkError => "❌ Erreur réseau — vérifiez votre connexion et réessayez.",
        Key::SyncNoDataWarn => "Aucune personne à sauvegarder. Ajoutez des personnes d'abord !",
        Key::SyncNoToken => "Aucun jeton. Connectez-vous d'abord.",
        Key::SyncNotConfigured => {
            "Sauvegarde Google Drive non configurée. Définissez GOOGLE_CLIENT_ID avant de compiler."
        }
        Key::SyncPassphraseHide => "Masquer",
        Key::SyncPassphraseLabel => "Chiffrer la sauvegarde avec une phrase de passe (optionnel)",
        Key::SyncPassphrasePlaceholder => "Entrez la phrase de passe...",
        Key::SyncPassphraseShow => "Afficher",
        Key::SyncPastePlaceholder => "Collez l'URL de redirection complète ici",
        Key::SyncReauth => "🔐 Se reconnecter",
        Key::SyncRestoreBtn => "☁ Restaurer depuis Drive",
        Key::SyncRestored => "✅ Restauré",
        Key::SyncRestoring => "Restauration en cours...",
        Key::SyncSaveTokenBtn => "Enregistrer",
        Key::SyncSignIn => "🔐 Connexion Google",
        Key::SyncTitle => "☁ Sync & Sauvegarde",
        Key::SyncTokenCleared => "Jeton effacé",
        Key::SyncTokenExpired => {
            "Votre connexion Google a expiré. Reconnectez-vous pour continuer la synchronisation."
        }
        Key::SyncTokenInstruction1 => "1. Appuyez sur « Connexion Google » — le navigateur s'ouvre",
        Key::SyncTokenInstruction2 => "2. Connectez-vous et autorisez l'accès",
        Key::SyncTokenInstruction3 => {
            "3. Le navigateur redirige vers l'app web — copiez le jeton depuis la barre d'adresse avant que la page ne charge"
        }
        Key::SyncTokenInstruction4 => "4. Collez l'URL ci-dessous et appuyez sur Enregistrer",
        Key::SyncTokenLoaded => "✓ Jeton chargé",
        Key::SyncTokenSaved => "✅ Jeton enregistré",
        Key::SyncViewBackup => "🔎 Voir les sauvegardes dans le navigateur (appDataFolder Browser)",
        Key::SyncWrongPassphrase => "❌ Mauvaise phrase de passe ou données corrompues",
        Key::TeamAllNoEdit => "Toutes les personnes inclut tout le monde automatiquement",
        Key::TeamAvgDanger => "Danger moyen",
        Key::TeamAvgScore => "Score moyen",
        Key::TeamCtxAvg => "Moyenne par situation",
        Key::TeamEdit => "Modifier",
        Key::TeamEmpty => "Ajoutez au moins 2 personnes pour voir la synergie d'équipe.",
        Key::TeamIcon => "Icône",
        Key::TeamMaxDanger => "Danger max",
        Key::TeamMembersCount => "{0} membres",
        Key::TeamNoDanger => "Aucun",
        Key::TeamPairs => "Toutes les paires",
        Key::TeamRename => "Renommer",
        Key::TeamSize => "Taille",
        Key::TeamStrongest => "Lien le plus fort",
        Key::TeamTabMembers => "Membres",
        Key::TeamTabSynergy => "Synergie",
        Key::TeamTitle => "Synergie d'équipe",
        Key::TeamWeakest => "Lien le plus faible",
        Key::TeamsAll => "Toutes les personnes",
        Key::TeamsCreate => "Nouvelle équipe",
        Key::TeamsDelete => "Supprimer l'équipe ?",
        Key::TeamsMembers => "{0} membres",
        Key::TeamsTitle => "Équipes",
        Key::TemplateBlank => "Vierge (commencer de zéro)",
        Key::TemplateTitle => "Modèle rapide",
        Key::TlEmpty => "Aucune entrée d'interaction.",
        Key::TlTitle => "Chronologie",
        Key::ToastDeleted => "Supprimé",
        Key::ToastError => "Une erreur est survenue",
        Key::ToastSaved => "Enregistré",
        Key::TrendDeteriorating => "Détérioration",
        Key::TrendHint => "Basé sur les interactions journalisées récentes",
        Key::TrendImproving => "Amélioration",
        Key::TrendStable => "Stable",
        Key::TutCompareBody => {
            "Une fois que vous avez au moins deux personnes, vous pouvez les comparer côte à côte pour voir leur score de synergie, leurs points de friction et leurs stratégies d'interaction.\n\nVous pouvez aussi suivre des prédictions (devinez un résultat, puis vérifiez si vous aviez raison), construire une carte des relations et journaliser les interactions sur une chronologie."
        }
        Key::TutCompareTitle => "Comparaisons & Plus",
        Key::TutCreateBody => {
            "Le formulaire personne est divisé en sections : infos de base (nom, rôle, contexte), scores de personnalité OCEAN, motivations, biais cognitifs, dimensions de réputation et schémas comportementaux.\n\nChaque section capture une facette différente de la personnalité — remplissez ce que vous savez, laissez le reste vide."
        }
        Key::TutCreateTitle => "Créer une Personne",
        Key::TutDoneBody => {
            "Vous pouvez rejouer ce tutoriel à tout moment depuis la barre de navigation.\n\nConseils rapides :\n• Créez au moins deux personnes pour débloquer les comparaisons\n• Utilisez la page Sync pour sauvegarder vos données\n• Utilisez les tags pour organiser les personnes par groupe\n\nAllez-y et commencez à modéliser les personnes de votre monde !"
        }
        Key::TutDoneTitle => "Prêt à Commencer !",
        Key::TutMotBiasBody => {
            "Les motivations capturent ce qui anime une personne — ses objectifs, ses peurs et ses valeurs (Réussite, Pouvoir, Affiliation, Sécurité, Autonomie, etc.).\n\nLes biais représentent des raccourcis mentaux qui influencent ses décisions (biais de confirmation, ancrage, excès de confiance, etc.). Ensemble, ils vous donnent une compréhension plus profonde de pourquoi les gens agissent comme ils le font."
        }
        Key::TutMotBiasTitle => "Motivations & Biais",
        Key::TutOceanBody => {
            "L'OCEAN mesure la personnalité sur cinq dimensions de 1 à 10 :\n• Ouverture — curiosité vs. prudence\n• Conscience — organisation vs. flexibilité\n• Extraversion — sociabilité vs. solitude\n• Agréabilité — coopération vs. compétition\n• Névrosisme — sensibilité vs. stabilité émotionnelle\n\nCes scores alimentent le moteur de comparaison et aident à prédire le comportement."
        }
        Key::TutOceanTitle => "Modèle OCEAN (Big Five)",
        Key::TutPeopleBody => {
            "La page principale montre toutes les personnes que vous avez créées. Utilisez la barre de recherche pour trouver quelqu'un, triez par nom / récent / score OCEAN, et cliquez sur le bouton + pour ajouter une nouvelle personne."
        }
        Key::TutPeopleTitle => "Vos Personnes",
        Key::TutRepPatternBody => {
            "Les scores de réputation capturent comment les autres perçoivent cette personne sur des échelles bipolaires (travailleur vs. paresseux, honnête vs. trompeur, etc.).\n\nLes schémas comportementaux vous permettent d'enregistrer comment elle réagit typiquement à des déclencheurs spécifiques (stress, critique, succès, conflit, etc.). Cela aide à anticiper ses réponses dans des situations futures."
        }
        Key::TutRepPatternTitle => "Réputation & Schémas",
        Key::TutStep => "Étape",
        Key::TutWelcomeBody => {
            "Cette application vous aide à modéliser et comprendre les personnes de votre vie en utilisant des cadres de personnalité comme l'OCEAN (Big Five), les motivations, les biais cognitifs et les schémas comportementaux.\n\nVous pouvez comparer des personnes côte à côte, suivre des prédictions dans le temps, cartographier les relations et explorer les scores de synergie."
        }
        Key::TutWelcomeTitle => "Bienvenue sur PeopleModeler !",
        Key::ValuesTitle => "Valeurs",
        Key::ValueIntensityHelper => "Intensité (I) : à quel point cette valeur est ancrée.",
        Key::ValuePriorityHelper => "Priorité (P) : importance par rapport aux autres valeurs.",
        Key::PersonaSection => "Persona de travail",
        Key::PersonaHint => {
            "Définissez une persona de travail différente. Les scores OCEAN et réputation peuvent diverger du profil de base."
        }
        Key::PersonaCopyBase => "Copier depuis le profil de base",
        Key::PersonaClear => "Effacer le masque",
        Key::PersonaBalanceTitle => "Résilience & Appétence Risque",
        Key::PersonaOnlineSection => "Persona en ligne",
        Key::PersonaBaseSection => "Persona de la vie privée",
        Key::FacetBase => "Vie privée",
        Key::FacetWork => "Au travail",
        Key::FacetOnline => "En ligne",
        Key::FacetAuto => "Automatique",
        Key::FacetMain => "Principal",
        Key::FacetMainSuffix => " (principal)",
        Key::MainContextLabel => "Contexte principal :",
        Key::PersonaContextPrompt => "D'où connaissez-vous cette personne ?",
        Key::PersonaContextChange => "Changer de contexte",
        Key::MaskGapLow => "Pas de masque",
        Key::MaskGapModerate => "Masque modéré",
        Key::MaskGapHigh => "Fort masque",
        Key::TeamFacetToggle => "Filtrer par facette",
        Key::OceanVolatility => "Volatilité OCEAN",
        Key::RepPowerStruggle => "Lutte de pouvoir (réputation)",
        Key::OnlyNegativePatterns => "Patterns négatifs uniquement",
        Key::LowPredictionAccuracy => "Faible précision prédictive",
        Key::Unknown => "Unknown",
    }
}

/// Compile-time-checked translation lookup: the macro argument must be a
/// `Key` variant identifier (or `Key::path`), which the type system
/// enforces at compile time.
#[macro_export]
macro_rules! tr {
    ($key:ident, $lang:expr) => {{ $crate::i18n::tr($crate::i18n::Key::$key, $lang) }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn core_lang_maps_both_branches() {
        assert!(matches!(
            core_lang(Lang::Fr),
            peoplemodeler_core::i18n::Lang::Fr
        ));
        assert!(matches!(
            core_lang(Lang::En),
            peoplemodeler_core::i18n::Lang::En
        ));
    }

    const IDENTITY: &[Key] = &[
        Key::OceanVolatility,
        Key::OnlyNegativePatterns,
        Key::LowPredictionAccuracy,
    ];

    #[test]
    fn all_keys_translate_en() {
        for key in Key::iter() {
            if key == Key::Unknown {
                continue;
            }
            let result = tr(key, Lang::En);
            assert!(!result.is_empty(), "tr({key:?}, En) returned empty");
            if !IDENTITY.contains(&key) {
                assert_ne!(
                    result,
                    key.to_string(),
                    "tr({key:?}, En) returned key itself (arm deleted?)"
                );
            }
            assert_ne!(result, "xyzzy", "tr({key:?}, En) returned sentinel 'xyzzy'");
        }
    }

    #[test]
    fn all_keys_translate_fr() {
        for key in Key::iter() {
            if key == Key::Unknown {
                continue;
            }
            let result = tr(key, Lang::Fr);
            assert!(!result.is_empty(), "tr({key:?}, Fr) returned empty");
            assert_ne!(
                result,
                key.to_string(),
                "tr({key:?}, Fr) returned key itself (arm deleted?)"
            );
            assert_ne!(result, "xyzzy", "tr({key:?}, Fr) returned sentinel 'xyzzy'");
        }
    }

    #[test]
    fn key_roundtrip_snake_case() {
        assert_eq!("facet_base".parse::<Key>(), Ok(Key::FacetBase));
        assert_eq!("nav_people".parse::<Key>(), Ok(Key::NavPeople));
        assert_eq!(Key::FacetBase.to_string(), "facet_base");
        assert_eq!(Key::NavPeople.to_string(), "nav_people");
    }

    #[test]
    fn key_from_str_danger_identities() {
        assert_eq!("OCEAN volatility".parse::<Key>(), Ok(Key::OceanVolatility));
        assert_eq!(
            "Rep power struggle".parse::<Key>(),
            Ok(Key::RepPowerStruggle)
        );
        assert_eq!(
            "Only negative patterns".parse::<Key>(),
            Ok(Key::OnlyNegativePatterns)
        );
        assert_eq!(
            "Low prediction accuracy".parse::<Key>(),
            Ok(Key::LowPredictionAccuracy)
        );
    }

    #[test]
    fn key_from_str_unknown_maps_to_unknown() {
        // Unrecognized strings fail to parse (no strum default variant);
        // tr_str then falls back to the raw string, preserving the old
        // `_ => key` behavior.
        assert!(
            "this_key_does_not_exist_at_all_______"
                .parse::<Key>()
                .is_err()
        );
        assert_eq!(
            tr_str("this_key_does_not_exist_at_all_______", Lang::En),
            "this_key_does_not_exist_at_all_______"
        );
        assert_eq!(tr_str("facet_base", Lang::Fr), "Vie privée");
    }

    #[test]
    fn tr_empty_details() {
        assert_eq!(tr_danger_details("", Lang::En), "");
        assert_eq!(tr_danger_details("", Lang::Fr), "");
    }

    #[test]
    fn tr_danger_details_en_individual() {
        assert_eq!(
            tr_danger_details("OCEAN volatility", Lang::En),
            "OCEAN volatility"
        );
        assert_eq!(
            tr_danger_details("Rep power struggle", Lang::En),
            "Power struggle (Reputation)"
        );
        assert_eq!(
            tr_danger_details("Only negative patterns", Lang::En),
            "Only negative patterns"
        );
        assert_eq!(
            tr_danger_details("Low prediction accuracy", Lang::En),
            "Low prediction accuracy"
        );
    }

    #[test]
    fn tr_danger_details_fr_individual() {
        assert_eq!(
            tr_danger_details("OCEAN volatility", Lang::Fr),
            "Volatilité OCEAN"
        );
        assert_eq!(
            tr_danger_details("Rep power struggle", Lang::Fr),
            "Lutte de pouvoir (réputation)"
        );
        assert_eq!(
            tr_danger_details("Only negative patterns", Lang::Fr),
            "Patterns négatifs uniquement"
        );
        assert_eq!(
            tr_danger_details("Low prediction accuracy", Lang::Fr),
            "Faible précision prédictive"
        );
    }

    #[test]
    fn tr_danger_details_multi() {
        assert_eq!(
            tr_danger_details("OCEAN volatility, Rep power struggle", Lang::En),
            "OCEAN volatility, Power struggle (Reputation)"
        );
        assert_eq!(
            tr_danger_details("OCEAN volatility, Rep power struggle", Lang::Fr),
            "Volatilité OCEAN, Lutte de pouvoir (réputation)"
        );
    }

    #[test]
    fn tr_danger_details_unknown() {
        assert_eq!(
            tr_danger_details("Some unknown detail", Lang::En),
            "Unknown"
        );
        assert_eq!(
            tr_danger_details("Some unknown detail", Lang::Fr),
            "Unknown"
        );
    }

    #[test]
    fn lang_detect_nonwasm() {
        let lang = Lang::detect();
        assert!(lang == Lang::En || lang == Lang::Fr);
    }

    #[test]
    fn lang_display() {
        assert_eq!(format!("{:?}", Lang::En), "En");
        assert_eq!(format!("{:?}", Lang::Fr), "Fr");
    }

    #[test]
    fn lang_persist_writes_file() {
        let _lock = crate::CWD_LOCK.lock().unwrap();
        let path = std::env::current_dir().unwrap().join(".pm_lang");
        let _ = std::fs::remove_file(&path);
        Lang::En.persist();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "en");
        Lang::Fr.persist();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "fr");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn lang_equality() {
        assert_eq!(Lang::En, Lang::En);
        assert_eq!(Lang::Fr, Lang::Fr);
        assert_ne!(Lang::En, Lang::Fr);
    }

    #[test]
    fn lang_clone_copy() {
        let a = Lang::En;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn detect_from_strings_stored_en() {
        assert_eq!(detect_from_strings(Some("en"), None, None), Lang::En);
    }

    #[test]
    fn detect_from_strings_stored_fr() {
        assert_eq!(detect_from_strings(Some("fr"), None, None), Lang::Fr);
    }

    #[test]
    fn detect_from_strings_stored_other() {
        assert_eq!(detect_from_strings(Some("de"), None, None), Lang::Fr);
    }

    #[test]
    fn detect_from_strings_navigator_en() {
        assert_eq!(detect_from_strings(None, Some("en-CA"), None), Lang::En);
    }

    #[test]
    fn detect_from_strings_navigator_fr() {
        assert_eq!(detect_from_strings(None, Some("fr-CA"), None), Lang::Fr);
    }

    #[test]
    fn detect_from_strings_env_en() {
        assert_eq!(
            detect_from_strings(None, None, Some("en_US.UTF-8")),
            Lang::En
        );
    }

    #[test]
    fn detect_from_strings_env_fr() {
        assert_eq!(
            detect_from_strings(None, None, Some("fr_CA.UTF-8")),
            Lang::Fr
        );
    }

    #[test]
    fn detect_from_strings_all_none() {
        assert_eq!(detect_from_strings(None, None, None), Lang::Fr);
    }

    #[test]
    fn detect_from_strings_stored_takes_priority() {
        assert_eq!(
            detect_from_strings(Some("fr"), Some("en-CA"), Some("en_US.UTF-8")),
            Lang::Fr
        );
    }
}
