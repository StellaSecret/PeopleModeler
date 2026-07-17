use crate::models::{BehaviorResponse, BiasType, MotivationType, RepDim, StyleType};

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
            },
        }
    }
}

impl BehaviorResponse {
    pub fn label(self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => match self {
                Self::SeeksSupport => "🟢 Seeks support (asks for help)",
                Self::BecomesQuiet => "🟡 Becomes quiet (shuts down, goes silent)",
                Self::Withdraws => "🔴 Withdraws (panics, isolates, pulls away)",
                Self::CommunicatesOpenly => "🟢 Communicates openly (expresses feelings)",
                Self::SeeksCompromise => "🟡 Seeks compromise (meets halfway)",
                Self::BecomesDefensive => "🔴 Becomes defensive (stonewalls, argues, deflects)",
                Self::SharesCredit => "🟢 Shares credit (gives praise to others)",
                Self::SetsNewGoals => "🟡 Sets new goals (raises the bar)",
                Self::BecomesOverconfident => "🔴 Becomes overconfident (arrogant, boasts)",
                Self::AsksQuestions => "🟢 Asks questions (seeks clarity)",
                Self::SeeksData => "🟡 Seeks data (gathers facts)",
                Self::OverPlans => "🔴 Over-plans (analysis paralysis)",
                Self::AppreciatesPraise => "🟢 Appreciates praise (accepts compliments gracefully)",
                Self::SharesAchievement => "🟡 Shares achievement (updates on progress)",
                Self::SeeksMore => "🔴 Seeks more validation (needs constant approval)",
                Self::StandsGround => "🟢 Stands ground (asserts position calmly)",
                Self::SeeksAllies => "🟡 Seeks allies (builds coalitions)",
                Self::DeflectsBlame => {
                    "🔴 Deflects blame (freezes, points fingers, avoids responsibility)"
                }
                Self::EmbracesChange => "🟢 Embraces change (adapts quickly)",
                Self::PlansAhead => "🟡 Plans ahead (prepares, anticipates)",
                Self::ResistsChange => "🔴 Resists change (clings to status quo)",
                Self::AsksForDetails => "🟢 Asks for details (digs deeper, seeks specifics)",
                Self::Reflects => "🟡 Reflects thoughtfully (takes time to process)",
                Self::RejectsFeedback => "🔴 Rejects feedback (dismisses, gets defensive)",
                Self::ProtestsFirmly => "🟡 Protests firmly (stands up for what is right)",
                Self::AcceptsResignedly => "🔴 Accepts resignedly (gives in but resents it)",
                Self::SeeksRestoration => "🟢 Seeks restoration (repairs and reconciles)",
                Self::ExploitsOpportunistically => {
                    "🔴 Exploits opportunistically (takes advantage of unfair situation)"
                }
            },
            Lang::Fr => match self {
                Self::SeeksSupport => "🟢 Cherche du soutien (demande de l'aide)",
                Self::BecomesQuiet => "🟡 Devient silencieux (se ferme, se tait)",
                Self::Withdraws => "🔴 Se retire (panique, s'isole, se met à l'écart)",
                Self::CommunicatesOpenly => "🟢 Communique ouvertement (exprime ses sentiments)",
                Self::SeeksCompromise => "🟡 Cherche un compromis (trouve un terrain d'entente)",
                Self::BecomesDefensive => {
                    "🔴 Devient défensif (fait obstruction, argumente, se justifie)"
                }
                Self::SharesCredit => "🟢 Partage le crédit (félicite les autres)",
                Self::SetsNewGoals => "🟡 Se fixe de nouveaux objectifs (élève la barre)",
                Self::BecomesOverconfident => "🔴 Devient trop confiant (arrogant, se vante)",
                Self::AsksQuestions => "🟢 Pose des questions (cherche à comprendre)",
                Self::SeeksData => "🟡 Cherche des données (rassemble des faits)",
                Self::OverPlans => "🔴 Planifie trop (paralyse par l'analyse)",
                Self::AppreciatesPraise => {
                    "🟢 Apprécie les éloges (accepte les compliments avec grâce)"
                }
                Self::SharesAchievement => "🟡 Partage ses réussites (informe des progrès)",
                Self::SeeksMore => "🔴 Cherche plus de validation (besoin constant d'approbation)",
                Self::StandsGround => "🟢 Tient bon (affirme sa position calmement)",
                Self::SeeksAllies => "🟡 Cherche des alliés (tisse des coalitions)",
                Self::DeflectsBlame => {
                    "🔴 Détourne le blâme (se fige, montre du doigt, évite les responsabilités)"
                }
                Self::EmbracesChange => "🟢 Accepte le changement (s'adapte rapidement)",
                Self::PlansAhead => "🟡 Planifie à l'avance (se prépare, anticipe)",
                Self::ResistsChange => "🔴 Résiste au changement (s'accroche au statu quo)",
                Self::AsksForDetails => "🟢 Demande des détails (creuse, cherche des précisions)",
                Self::Reflects => "🟡 Réfléchit avec soin (prend le temps d'analyser)",
                Self::RejectsFeedback => "🔴 Rejette le feedback (se braque, se ferme)",
                Self::ProtestsFirmly => "🟡 Proteste fermement (défend ce qui est juste)",
                Self::AcceptsResignedly => {
                    "🔴 Accepte résigné (cède mais en ressent du ressentiment)"
                }
                Self::SeeksRestoration => "🟢 Cherche la réparation (répare et réconcilie)",
                Self::ExploitsOpportunistically => {
                    "🔴 Exploite opportunément (profite de l'injustice)"
                }
            },
        }
    }
}
