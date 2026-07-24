use crate::models::{BehaviorResponse, BiasType, MotivationType, RepDim, StyleCategory, StyleType};

#[derive(Clone, Copy, PartialEq)]
pub enum Lang {
    Fr,
    En,
}

pub struct MotI18n {
    pub label: &'static str,
    pub desc: &'static str,
}

pub struct BiasI18n {
    pub label: &'static str,
    pub desc: &'static str,
}

impl MotivationType {
    pub fn i18n(&self, lang: Lang) -> MotI18n {
        match lang {
            Lang::Fr => match self {
                Self::Power => MotI18n {
                    label: "Pouvoir",
                    desc: "Contrôle des décisions, influence et autorité",
                },
                Self::Achievement => MotI18n {
                    label: "Accomplissement",
                    desc: "Atteinte d'objectifs ambitieux et performance",
                },
                Self::Affiliation => MotI18n {
                    label: "Appartenance",
                    desc: "Relations harmonieuses et appartenance au groupe",
                },
                Self::Security => MotI18n {
                    label: "Sécurité",
                    desc: "Stabilité, prévisibilité et évitement des risques",
                },
                Self::Autonomy => MotI18n {
                    label: "Autonomie",
                    desc: "Indépendance et liberté d'action",
                },
                Self::Recognition => MotI18n {
                    label: "Reconnaissance",
                    desc: "Validation et estime des autres",
                },
                Self::Learning => MotI18n {
                    label: "Apprentissage",
                    desc: "Connaissances et développement personnel",
                },
                Self::Helping => MotI18n {
                    label: "Aider les autres",
                    desc: "Aider et soutenir les autres",
                },
                Self::Creativity => MotI18n {
                    label: "Créativité",
                    desc: "Innovation, expression et exploration créative",
                },
                Self::Fairness => MotI18n {
                    label: "Justice",
                    desc: "Équité, mérite et traitement juste des autres",
                },
            },
            Lang::En => match self {
                Self::Power => MotI18n {
                    label: "Power",
                    desc: "Control over decisions, influence and authority",
                },
                Self::Achievement => MotI18n {
                    label: "Achievement",
                    desc: "Reaching ambitious goals and high performance",
                },
                Self::Affiliation => MotI18n {
                    label: "Affiliation",
                    desc: "Harmonious relationships and group belonging",
                },
                Self::Security => MotI18n {
                    label: "Security",
                    desc: "Stability, predictability and risk avoidance",
                },
                Self::Autonomy => MotI18n {
                    label: "Autonomy",
                    desc: "Independence and freedom of action",
                },
                Self::Recognition => MotI18n {
                    label: "Recognition",
                    desc: "Validation and esteem from others",
                },
                Self::Learning => MotI18n {
                    label: "Learning",
                    desc: "Knowledge and personal development",
                },
                Self::Helping => MotI18n {
                    label: "Helping others",
                    desc: "Helping and supporting others",
                },
                Self::Creativity => MotI18n {
                    label: "Creativity",
                    desc: "Innovation, expression and creative exploration",
                },
                Self::Fairness => MotI18n {
                    label: "Fairness",
                    desc: "Justice, equity, and fair treatment of others",
                },
            },
        }
    }
}

impl BiasType {
    pub fn i18n(&self, lang: Lang) -> BiasI18n {
        match lang {
            Lang::Fr => match self {
                Self::Confirmation => BiasI18n {
                    label: "Biais de confirmation",
                    desc: "Cherche et interprète les infos qui confirment ses croyances",
                },
                Self::Anchoring => BiasI18n {
                    label: "Ancrage cognitif",
                    desc: "Se focalise sur la première information reçue",
                },
                Self::Availability => BiasI18n {
                    label: "Disponibilité",
                    desc: "Surestime la probabilité d'événements récents",
                },
                Self::SunkCost => BiasI18n {
                    label: "Coût irrécupérable",
                    desc: "Poursuit un investissement à cause des ressources déjà engagées",
                },
                Self::DunningKruger => BiasI18n {
                    label: "Dunning-Kruger",
                    desc: "Les incompétents surestiment leurs compétences, les experts les sous-estiment",
                },
                Self::LossAversion => BiasI18n {
                    label: "Aversion aux pertes",
                    desc: "Préfère éviter les pertes plutôt que chercher des gains",
                },
                Self::SocialProof => BiasI18n {
                    label: "Preuve sociale",
                    desc: "Se conforme aux comportements du groupe",
                },
                Self::Authority => BiasI18n {
                    label: "Autorité",
                    desc: "Confiance excessive aux figures d'autorité",
                },
                Self::Recency => BiasI18n {
                    label: "Récence",
                    desc: "Accorde plus d'importance aux informations récentes",
                },
                Self::InGroup => BiasI18n {
                    label: "Endogroupe",
                    desc: "Favorise les membres de son propre groupe",
                },
                Self::Favoritism => BiasI18n {
                    label: "Favoritisme",
                    desc: "Accorde un traitement préférentiel à certains individus",
                },
            },
            Lang::En => match self {
                Self::Confirmation => BiasI18n {
                    label: "Confirmation bias",
                    desc: "Seeks and interprets info that confirms existing beliefs",
                },
                Self::Anchoring => BiasI18n {
                    label: "Anchoring",
                    desc: "Fixes on the first piece of information received",
                },
                Self::Availability => BiasI18n {
                    label: "Availability",
                    desc: "Overestimates probability of recent events",
                },
                Self::SunkCost => BiasI18n {
                    label: "Sunk cost",
                    desc: "Continues investment because of resources already committed",
                },
                Self::DunningKruger => BiasI18n {
                    label: "Dunning-Kruger",
                    desc: "Incompetent overestimate skill, experts underestimate",
                },
                Self::LossAversion => BiasI18n {
                    label: "Loss aversion",
                    desc: "Prefers avoiding losses over acquiring gains",
                },
                Self::SocialProof => BiasI18n {
                    label: "Social proof",
                    desc: "Conforms to group behavior",
                },
                Self::Authority => BiasI18n {
                    label: "Authority",
                    desc: "Excessive trust in authority figures",
                },
                Self::Recency => BiasI18n {
                    label: "Recency",
                    desc: "Overweights recent information",
                },
                Self::InGroup => BiasI18n {
                    label: "In-group",
                    desc: "Favors members of own group",
                },
                Self::Favoritism => BiasI18n {
                    label: "Favoritism",
                    desc: "Shows preferential treatment toward certain individuals",
                },
            },
        }
    }
}

pub struct RepI18n {
    pub label_a: &'static str,
    pub label_b: &'static str,
    pub desc: &'static str,
}

impl RepDim {
    pub fn i18n(&self, lang: Lang) -> RepI18n {
        let (label_a, label_b) = match lang {
            Lang::Fr => match self {
                Self::HardworkerLazy => ("Travailleur", "Paresseux"),
                Self::AuthoritativeSubmissive => ("Autoritaire", "Soumis"),
                Self::HonestDeceitful => ("Honnête", "Fourbe"),
                Self::ReliableFlaky => ("Fiable", "Inconstant"),
                Self::HumbleArrogant => ("Humble", "Arrogant"),
                Self::CalmReactive => ("Calme", "Réactif"),
                Self::DiplomaticBlunt => ("Diplomate", "Direct"),
                Self::GenerousSelfish => ("Généreux", "Égoïste"),
                Self::FairFavoritism => ("Équitable", "Partial"),
                Self::TrustingSuspicious => ("Confiant", "Méfiant"),
                Self::AssertivePassive => ("Affirmé", "Passif"),
                Self::EmpatheticDetached => ("Empathique", "Détaché"),
                Self::AdaptableRigid => ("Flexible", "Rigide"),
            },
            Lang::En => match self {
                Self::HardworkerLazy => ("Hardworker", "Lazy"),
                Self::AuthoritativeSubmissive => ("Authoritative", "Submissive"),
                Self::HonestDeceitful => ("Honest", "Deceitful"),
                Self::ReliableFlaky => ("Reliable", "Flaky"),
                Self::HumbleArrogant => ("Humble", "Arrogant"),
                Self::CalmReactive => ("Calm", "Reactive"),
                Self::DiplomaticBlunt => ("Diplomatic", "Blunt"),
                Self::GenerousSelfish => ("Generous", "Selfish"),
                Self::FairFavoritism => ("Fair", "Favoritism"),
                Self::TrustingSuspicious => ("Trusting", "Suspicious"),
                Self::AssertivePassive => ("Assertive", "Passive"),
                Self::EmpatheticDetached => ("Empathetic", "Detached"),
                Self::AdaptableRigid => ("Adaptable", "Rigid"),
            },
        };
        let desc = match lang {
            Lang::Fr => match self {
                Self::HardworkerLazy => "Effort vs. fainéantise",
                Self::AuthoritativeSubmissive => "Commandement vs. obéissance",
                Self::HonestDeceitful => "Vérité vs. tromperie",
                Self::ReliableFlaky => "Constance vs. versatilité",
                Self::HumbleArrogant => "Modestie vs. orgueil",
                Self::CalmReactive => "Sérénité vs. réactivité émotionnelle",
                Self::DiplomaticBlunt => "Tact vs. franchise",
                Self::GenerousSelfish => "Altruisme vs. égoïsme",
                Self::FairFavoritism => "Justice vs. favoritisme",
                Self::TrustingSuspicious => "Confiance vs. méfiance",
                Self::AssertivePassive => "Affirmation vs. passivité",
                Self::EmpatheticDetached => "Empathie vs. détachement",
                Self::AdaptableRigid => "Flexibilité vs. rigidité",
            },
            Lang::En => match self {
                Self::HardworkerLazy => "Effort vs. laziness",
                Self::AuthoritativeSubmissive => "Command vs. obedience",
                Self::HonestDeceitful => "Truth vs. deception",
                Self::ReliableFlaky => "Consistency vs. unreliability",
                Self::HumbleArrogant => "Modesty vs. pride",
                Self::CalmReactive => "Serenity vs. emotional reactivity",
                Self::DiplomaticBlunt => "Tact vs. directness",
                Self::GenerousSelfish => "Altruism vs. selfishness",
                Self::FairFavoritism => "Justice vs. favoritism",
                Self::TrustingSuspicious => "Trust vs. suspicion",
                Self::AssertivePassive => "Assertion vs. passivity",
                Self::EmpatheticDetached => "Empathy vs. detachment",
                Self::AdaptableRigid => "Flexibility vs. rigidity",
            },
        };
        RepI18n {
            label_a,
            label_b,
            desc,
        }
    }
}

pub struct StyleI18n {
    pub label: &'static str,
    pub desc: &'static str,
}

impl StyleCategory {
    pub fn i18n_label(&self, lang: Lang) -> &'static str {
        match (self, lang) {
            (Self::Communication, Lang::En) => "💬 Communication",
            (Self::Communication, Lang::Fr) => "💬 Communication",
            (Self::ConflictResolution, Lang::En) => "🤝 Conflict Resolution",
            (Self::ConflictResolution, Lang::Fr) => "🤝 Résolution de conflit",
            (Self::DecisionMaking, Lang::En) => "🧠 Decision-Making",
            (Self::DecisionMaking, Lang::Fr) => "🧠 Prise de décision",
            (Self::Leadership, Lang::En) => "👥 Leadership",
            (Self::Leadership, Lang::Fr) => "👥 Leadership",
            (Self::TimeOrientation, Lang::En) => "⏰ Time Orientation",
            (Self::TimeOrientation, Lang::Fr) => "⏰ Orientation temporelle",
            (Self::MoralFramework, Lang::En) => "📜 Moral Framework",
            (Self::MoralFramework, Lang::Fr) => "📜 Cadre moral",
            (Self::InterpersonalConduct, Lang::En) => "🫂 Interpersonal Conduct",
            (Self::InterpersonalConduct, Lang::Fr) => "🫂 Conduite interpersonnelle",
        }
    }
}

impl StyleType {
    pub fn i18n_label(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::Fr => match self {
                Self::DirectCommunicator => "Direct",
                Self::DiplomaticCommunicator => "Diplomate",
                Self::ReservedCommunicator => "Réservé",
                Self::ExpressiveCommunicator => "Expressif",
                Self::Competing => "Compétitif",
                Self::Collaborating => "Collaboratif",
                Self::Compromising => "Compromis",
                Self::Avoiding => "Évitant",
                Self::Accommodating => "Accommodant",
                Self::Analytical => "Analytique",
                Self::Intuitive => "Intuitif",
                Self::Participatory => "Participatif",
                Self::Autocratic => "Autocratique",
                Self::ConsensusDriven => "Consensus",
                Self::Visionary => "Visionnaire",
                Self::Servant => "Serviteur",
                Self::Transactional => "Transactionnel",
                Self::Transformational => "Transformationnel",
                Self::Bureaucratic => "Bureaucrate",
                Self::PastOriented => "Orienté passé",
                Self::PresentOriented => "Orienté présent",
                Self::FutureOriented => "Orienté futur",
                Self::RuleBased => "Basé sur les règles",
                Self::OutcomeBased => "Basé sur les résultats",
                Self::VirtueBased => "Basé sur les vertus",
                Self::Relativist => "Relativiste",
                // Interpersonal conduct
                Self::Opportunistic => "Opportuniste",
                Self::Intrusive => "Intrusif",
                Self::Manipulative => "Manipulateur",
                Self::PassiveAggressive => "Agressif passif",
                Self::Controlling => "Contrôlant",
                Self::Detached => "Détaché",
                Self::Respectful => "Respectueux",
                Self::Empathetic => "Empathique",
                Self::Supportive => "Supportif",
                Self::Nurturing => "Bienveillant",
            },
            Lang::En => match self {
                Self::DirectCommunicator => "Direct",
                Self::DiplomaticCommunicator => "Diplomatic",
                Self::ReservedCommunicator => "Reserved",
                Self::ExpressiveCommunicator => "Expressive",
                Self::Competing => "Competing",
                Self::Collaborating => "Collaborating",
                Self::Compromising => "Compromising",
                Self::Avoiding => "Avoiding",
                Self::Accommodating => "Accommodating",
                Self::Analytical => "Analytical",
                Self::Intuitive => "Intuitive",
                Self::Participatory => "Participatory",
                Self::Autocratic => "Autocratic",
                Self::ConsensusDriven => "Consensus-Driven",
                Self::Visionary => "Visionary",
                Self::Servant => "Servant",
                Self::Transactional => "Transactional",
                Self::Transformational => "Transformational",
                Self::Bureaucratic => "Bureaucratic",
                Self::PastOriented => "Past-Oriented",
                Self::PresentOriented => "Present-Oriented",
                Self::FutureOriented => "Future-Oriented",
                Self::RuleBased => "Rule-Based",
                Self::OutcomeBased => "Outcome-Based",
                Self::VirtueBased => "Virtue-Based",
                Self::Relativist => "Relativist",
                // Interpersonal conduct
                Self::Opportunistic => "Opportunistic",
                Self::Intrusive => "Intrusive",
                Self::Manipulative => "Manipulative",
                Self::PassiveAggressive => "Passive-Aggressive",
                Self::Controlling => "Controlling",
                Self::Detached => "Detached",
                Self::Respectful => "Respectful",
                Self::Empathetic => "Empathetic",
                Self::Supportive => "Supportive",
                Self::Nurturing => "Nurturing",
            },
        }
    }

    pub fn i18n_desc(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::Fr => match self {
                Self::DirectCommunicator => "Parle franchement et va droit au but",
                Self::DiplomaticCommunicator => "Adoucit son langage pour ménager les autres",
                Self::ReservedCommunicator => "Parle peu, choisit ses mots avec soin",
                Self::ExpressiveCommunicator => "Partage ses pensées et émotions ouvertement",
                Self::Competing => "Cherche à gagner, confronte directement",
                Self::Collaborating => "Cherche une solution qui satisfait tout le monde",
                Self::Compromising => "Accepte des concessions mutuelles",
                Self::Avoiding => "Évite la confrontation, laisse faire",
                Self::Accommodating => "Cède pour préserver l'harmonie",
                Self::Analytical => "Décide après analyse approfondie des données",
                Self::Intuitive => "Décide par instinct et ressenti",
                Self::Participatory => "Implique les autres dans la décision",
                Self::Autocratic => "Décide seul, sans consultation",
                Self::ConsensusDriven => "Cherche l'accord unanime avant de décider",
                Self::Visionary => "Inspire avec une vision à long terme",
                Self::Servant => "Place les besoins de l'équipe en premier",
                Self::Transactional => "Gère par récompenses et sanctions",
                Self::Transformational => "Transforme et élève ses collaborateurs",
                Self::Bureaucratic => "Suit les procédures et la hiérarchie",
                Self::PastOriented => "Se réfère aux expériences passées",
                Self::PresentOriented => "Vit dans l'instant présent",
                Self::FutureOriented => "Planifie et anticipe l'avenir",
                Self::RuleBased => "Suit des principes moraux universels",
                Self::OutcomeBased => "Juge la moralité par les conséquences",
                Self::VirtueBased => "Cultive des qualités de caractère",
                Self::Relativist => "Adapte sa morale au contexte",
                // Interpersonal conduct
                Self::Opportunistic => "Exploite les situations et les gens pour son profit",
                Self::Intrusive => "Franchit les limites, s'impose aux autres",
                Self::Manipulative => "Orchestre les autres par la tromperie",
                Self::PassiveAggressive => "Résistance indirecte, sabotage subtil",
                Self::Controlling => "Domine et micro-gère les autres",
                Self::Detached => "Maintient une distance émotionnelle, objectif",
                Self::Respectful => "Respecte les limites et l'autonomie",
                Self::Empathetic => "Comprend et valide les émotions des autres",
                Self::Supportive => "Aide et encourage activement les autres",
                Self::Nurturing => "Investit dans la croissance des autres",
            },
            Lang::En => match self {
                Self::DirectCommunicator => "Speaks frankly, gets straight to the point",
                Self::DiplomaticCommunicator => "Softens language to spare others' feelings",
                Self::ReservedCommunicator => "Speaks little, chooses words carefully",
                Self::ExpressiveCommunicator => "Shares thoughts and emotions openly",
                Self::Competing => "Seeks to win, confronts directly",
                Self::Collaborating => "Seeks win-win solutions for everyone",
                Self::Compromising => "Accepts mutual concessions",
                Self::Avoiding => "Avoids confrontation, lets things slide",
                Self::Accommodating => "Yields to preserve harmony",
                Self::Analytical => "Decides after thorough data analysis",
                Self::Intuitive => "Decides by gut feeling and instinct",
                Self::Participatory => "Involves others in the decision",
                Self::Autocratic => "Decides alone without consultation",
                Self::ConsensusDriven => "Seeks unanimous agreement before deciding",
                Self::Visionary => "Inspires with a long-term vision",
                Self::Servant => "Puts team needs first",
                Self::Transactional => "Manages through rewards and sanctions",
                Self::Transformational => "Transforms and elevates collaborators",
                Self::Bureaucratic => "Follows procedures and hierarchy",
                Self::PastOriented => "References past experiences",
                Self::PresentOriented => "Lives in the present moment",
                Self::FutureOriented => "Plans and anticipates the future",
                Self::RuleBased => "Follows universal moral principles",
                Self::OutcomeBased => "Judges morality by consequences",
                Self::VirtueBased => "Cultivates character qualities",
                Self::Relativist => "Adapts morality to context",
                // Interpersonal conduct
                Self::Opportunistic => "Exploits situations and people for personal gain",
                Self::Intrusive => "Oversteps boundaries, imposes on others",
                Self::Manipulative => "Orchestrates others through deception",
                Self::PassiveAggressive => "Indirect resistance, subtle sabotage",
                Self::Controlling => "Dominates and micromanages others",
                Self::Detached => "Maintains emotional distance, objective",
                Self::Respectful => "Honors boundaries and autonomy",
                Self::Empathetic => "Understands and validates others' feelings",
                Self::Supportive => "Actively helps and encourages others",
                Self::Nurturing => "Invests in others' growth and wellbeing",
            },
        }
    }
}

impl BehaviorResponse {
    pub fn label(self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => self.label_en(),
            Lang::Fr => self.label_fr(),
        }
    }

    pub fn label_bare(self, lang: Lang) -> &'static str {
        let l = self.label(lang);
        // strip "<emoji> " prefix (2 unicode chars: emoji + trailing space)
        let byte_start = l.char_indices().nth(2).map(|(i, _)| i).unwrap_or(0);
        &l[byte_start..]
    }

    fn label_en(self) -> &'static str {
        match self {
            // Stress
            Self::RemainsCalm => "⭐ Remains calm (stays composed under pressure)",
            Self::SeeksSupport => "🔵 Seeks support (asks for help)",
            Self::StaysFocused => "🟢 Stays focused (channels stress into productivity)",
            Self::BecomesQuiet => "🟡 Becomes quiet (shuts down, goes silent)",
            Self::BecomesIrritable => "🟠 Becomes irritable (gets snappy)",
            Self::Overwhelmed => "🔴 Overwhelmed (shuts down)",
            Self::Panics => "⚫ Panics (loses control)",
            // Conflict
            Self::FacilitatesResolution => {
                "⭐ Facilitates resolution (mediates, finds common ground)"
            }
            Self::CommunicatesOpenly => {
                "🔵 Communicates openly (expresses feelings constructively)"
            }
            Self::SeeksCompromise => "🟢 Seeks compromise (meets halfway)",
            Self::StaysSilent => "🟡 Stays silent (avoids engagement)",
            Self::BecomesPassiveAggressive => "🟠 Becomes passive-aggressive (indirect digs)",
            Self::BecomesDefensive => "🔴 Becomes defensive (stonewalls, argues, deflects)",
            Self::Escalates => "⚫ Escalates (attacks personally)",
            // Success
            Self::CelebratesWithOthers => "⭐ Celebrates with others (shares joy, builds team)",
            Self::SharesCredit => "🔵 Shares credit (gives praise to others)",
            Self::SetsNewGoals => "🟢 Sets new goals (raises the bar)",
            Self::EnjoysQuietly => "🟡 Enjoys quietly (internal satisfaction)",
            Self::BecomesComplacent => "🟠 Becomes complacent (rests on laurels)",
            Self::BecomesOverconfident => "🔴 Becomes overconfident (arrogant, boasts)",
            Self::DismissesOthers => "⚫ Dismisses others (belittles contributions)",
            // Uncertainty
            Self::EmbracesAmbiguity => "⭐ Embraces ambiguity (thrives in unknown)",
            Self::AsksQuestions => "🔵 Asks questions (seeks clarity)",
            Self::SeeksData => "🟢 Seeks data (gathers facts)",
            Self::WaitsForClarity => "🟡 Waits for clarity (holds off)",
            Self::OverPlans => "🟠 Over-plans (tries to control the unknown)",
            Self::BecomesParalyzed => "🔴 Becomes paralyzed (unable to act)",
            Self::DeflectsResponsibility => "⚫ Deflects responsibility (blames ambiguity)",
            // Recognition
            Self::AppreciatesQuietly => {
                "⭐ Appreciates quietly (values recognition without display)"
            }
            Self::AppreciatesPraise => "🔵 Appreciates praise (accepts compliments gracefully)",
            Self::SharesAchievement => "🟢 Shares achievement (updates on progress)",
            Self::SeeksMore => "🟡 Seeks more validation (needs some approval)",
            Self::BecomesJealous => "🟠 Becomes jealous (resents others' recognition)",
            Self::DemandsAttention => "🔴 Demands attention (must be center)",
            Self::UnderminesOthers => "⚫ Undermines others (diminishes them to get ahead)",
            // Threatened
            Self::SeeksUnderstanding => "⭐ Seeks understanding (tries to understand the threat)",
            Self::SeeksAllies => "🔵 Seeks allies (builds support network)",
            Self::StandsGround => "🟢 Stands ground (calmly defends position)",
            Self::BecomesCautious => "🟡 Becomes cautious (withdraws to assess)",
            Self::DeflectsBlame => "🟠 Deflects blame (redirects responsibility)",
            Self::Counterattacks => "🔴 Counterattacks (strikes back)",
            Self::BecomesParanoid => "⚫ Becomes paranoid (sees threats everywhere)",
            // Change
            Self::EmbracesChange => "⭐ Embraces change (adapts quickly)",
            Self::PlansAhead => "🔵 Plans ahead (prepares, anticipates)",
            Self::AdaptsQuickly => "🟢 Adapts quickly (adjusts on the fly)",
            Self::ResistsChange => "🟡 Resists change (pushes back initially)",
            Self::NeedsReassurance => "🟠 Needs reassurance (requires support)",
            Self::BecomesDisoriented => "🔴 Becomes disoriented (can't keep up)",
            Self::Sabotages => "⚫ Sabotages (actively undermines)",
            // Feedback
            Self::SeeksFeedback => "⭐ Seeks feedback (proactively asks)",
            Self::AsksForDetails => "🔵 Asks for details (digs deeper, seeks specifics)",
            Self::Reflects => "🟢 Reflects thoughtfully (takes time to process)",
            Self::AcceptsResignedly => "🟡 Accepts resignedly (reluctant acceptance)",
            Self::RejectsFeedback => "🟠 Rejects feedback (dismisses)",
            Self::IgnoresCompletely => "⚫ Ignores completely (disregards entirely)",
            // Injustice
            Self::SeeksRestoration => "⭐ Seeks restoration (repairs and reconciles)",
            Self::ProtestsConstructively => {
                "🔵 Protests constructively (raises concerns productively)"
            }
            Self::ProtestsFirmly => "🟢 Protests firmly (advocates clearly)",
            Self::SeeksClarity => "🟡 Seeks clarity (investigates facts)",
            Self::WithdrawsFromInjustice => "🟠 Withdraws (disengages from injustice)",
            Self::ExploitsOpportunistically => "🔴 Exploits opportunistically (takes advantage)",
            Self::BecomesBitter => "⚫ Becomes bitter (resentful, cynical)",
        }
    }

    fn label_fr(self) -> &'static str {
        match self {
            // Stress
            Self::RemainsCalm => "⭐ Reste calme (garde son sang-froid sous pression)",
            Self::SeeksSupport => "🔵 Cherche du soutien (demande de l'aide)",
            Self::StaysFocused => "🟢 Reste concentré (canalise le stress en productivité)",
            Self::BecomesQuiet => "🟡 Devient silencieux (se ferme, se tait)",
            Self::BecomesIrritable => "🟠 Devient irritable (s'énerve facilement)",
            Self::Overwhelmed => "🔴 Submergé (se ferme complètement)",
            Self::Panics => "⚫ Panique (perd le contrôle)",
            // Conflict
            Self::FacilitatesResolution => {
                "⭐ Facilite la résolution (médie, trouve un terrain d'entente)"
            }
            Self::CommunicatesOpenly => {
                "🔵 Communique ouvertement (exprime ses sentiments avec constructivité)"
            }
            Self::SeeksCompromise => "🟢 Cherche un compromis (trouve un terrain d'entente)",
            Self::StaysSilent => "🟡 Reste silencieux (évite l'engagement)",
            Self::BecomesPassiveAggressive => "🟠 Devient passif-agressif (piques indirectes)",
            Self::BecomesDefensive => {
                "🔴 Devient défensif (fait obstruction, argumente, se justifie)"
            }
            Self::Escalates => "⚫ Escalade (attaque personnellement)",
            // Success
            Self::CelebratesWithOthers => {
                "⭐ Célèbre avec les autres (partage la joie, soude l'équipe)"
            }
            Self::SharesCredit => "🔵 Partage le crédit (félicite les autres)",
            Self::SetsNewGoals => "🟢 Se fixe de nouveaux objectifs (élève la barre)",
            Self::EnjoysQuietly => "🟡 Apprécie en silence (satisfaction intérieure)",
            Self::BecomesComplacent => "🟠 Devient complaisant (se repose sur ses lauriers)",
            Self::BecomesOverconfident => "🔴 Devient trop confiant (arrogant, se vante)",
            Self::DismissesOthers => "⚫ Dévalorise les autres (minimise leurs contributions)",
            // Uncertainty
            Self::EmbracesAmbiguity => "⭐ Embrasse l'ambiguïté (prospère dans l'incertain)",
            Self::AsksQuestions => "🔵 Pose des questions (cherche à comprendre)",
            Self::SeeksData => "🟢 Cherche des données (rassemble des faits)",
            Self::WaitsForClarity => "🟡 Attend des éclaircissements (temporise)",
            Self::OverPlans => "🟠 Planifie trop (essaie de contrôler l'incertain)",
            Self::BecomesParalyzed => "🔴 Devient paralysé (incapable d'agir)",
            Self::DeflectsResponsibility => "⚫ Esquive la responsabilité (blâme l'ambiguïté)",
            // Recognition
            Self::AppreciatesQuietly => {
                "⭐ Apprécie discrètement (valorise sans chercher la lumière)"
            }
            Self::AppreciatesPraise => {
                "🔵 Apprécie les éloges (accepte les compliments avec grâce)"
            }
            Self::SharesAchievement => "🟢 Partage ses réussites (informe des progrès)",
            Self::SeeksMore => "🟡 Cherche plus de validation (besoin d'approbation modéré)",
            Self::BecomesJealous => "🟠 Devient jaloux (ressent la reconnaissance des autres)",
            Self::DemandsAttention => "🔴 Exige l'attention (veut être le centre)",
            Self::UnderminesOthers => "⚫ Dénigre les autres (les rabaisse pour avancer)",
            // Threatened
            Self::SeeksUnderstanding => "⭐ Cherche à comprendre (essaie de cerner la menace)",
            Self::SeeksAllies => "🔵 Cherche des alliés (tisse des coalitions)",
            Self::StandsGround => "🟢 Tient bon (affirme sa position calmement)",
            Self::BecomesCautious => "🟡 Devient prudent (recule pour évaluer)",
            Self::DeflectsBlame => "🟠 Détourne le blâme (redirige la responsabilité)",
            Self::Counterattacks => "🔴 Contre-attaque (riposte)",
            Self::BecomesParanoid => "⚫ Devient paranoïaque (voit des menaces partout)",
            // Change
            Self::EmbracesChange => "⭐ Accepte le changement (s'adapte rapidement)",
            Self::PlansAhead => "🔵 Planifie à l'avance (se prépare, anticipe)",
            Self::AdaptsQuickly => "🟢 S'adapte rapidement (s'ajuste à la volée)",
            Self::ResistsChange => "🟡 Résiste au changement (rechigne initialement)",
            Self::NeedsReassurance => "🟠 A besoin de réassurance (demande du soutien)",
            Self::BecomesDisoriented => "🔴 Désorienté (n'arrive pas à suivre)",
            Self::Sabotages => "⚫ Sabote (compromet activement)",
            // Feedback
            Self::SeeksFeedback => "⭐ Cherche le feedback (le sollicite proactivement)",
            Self::AsksForDetails => "🔵 Demande des détails (creuse, cherche des précisions)",
            Self::Reflects => "🟢 Réfléchit avec soin (prend le temps d'analyser)",
            Self::AcceptsResignedly => "🟡 Accepte résigné (acceptation à contrecœur)",
            Self::RejectsFeedback => "🟠 Rejette le feedback (se braque, se ferme)",
            Self::IgnoresCompletely => "⚫ Ignore complètement (fait la sourde oreille)",
            // Injustice
            Self::SeeksRestoration => "⭐ Cherche la réparation (répare et réconcilie)",
            Self::ProtestsConstructively => {
                "🔵 Proteste avec constructivité (exprime ses préoccupations)"
            }
            Self::ProtestsFirmly => "🟢 Proteste fermement (défend ce qui est juste)",
            Self::SeeksClarity => "🟡 Cherche des éclaircissements (enquête sur les faits)",
            Self::WithdrawsFromInjustice => "🟠 Se retire (se désengage de l'injustice)",
            Self::ExploitsOpportunistically => {
                "🔴 Exploite opportunément (profite de la situation)"
            }
            Self::BecomesBitter => "⚫ Devient amer (ranceur, cynisme)",
        }
    }
}
