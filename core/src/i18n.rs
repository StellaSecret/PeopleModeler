use crate::models::{BehaviorResponse, BiasType, MotivationType, RepDim};

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
            },
        };
        RepI18n {
            label_a,
            label_b,
            desc,
        }
    }
}

impl BehaviorResponse {
    pub fn label(self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => match self {
                Self::SeeksSupport => "Seeks support",
                Self::BecomesQuiet => "Becomes quiet",
                Self::Withdraws => "Withdraws",
                Self::CommunicatesOpenly => "Communicates openly",
                Self::SeeksCompromise => "Seeks compromise",
                Self::BecomesDefensive => "Becomes defensive",
                Self::SharesCredit => "Shares credit",
                Self::SetsNewGoals => "Sets new goals",
                Self::BecomesOverconfident => "Becomes overconfident",
                Self::AsksQuestions => "Asks questions",
                Self::SeeksData => "Seeks data",
                Self::OverPlans => "Over-plans",
                Self::AppreciatesPraise => "Appreciates praise",
                Self::SharesAchievement => "Shares achievement",
                Self::SeeksMore => "Seeks more validation",
                Self::StandsGround => "Stands ground",
                Self::SeeksAllies => "Seeks allies",
                Self::DeflectsBlame => "Deflects blame",
                Self::EmbracesChange => "Embraces change",
                Self::PlansAhead => "Plans ahead",
                Self::ResistsChange => "Resists change",
                Self::AsksForDetails => "Asks for details",
                Self::Reflects => "Reflects thoughtfully",
                Self::RejectsFeedback => "Rejects feedback",
            },
            Lang::Fr => match self {
                Self::SeeksSupport => "Cherche du soutien",
                Self::BecomesQuiet => "Devient silencieux",
                Self::Withdraws => "Se retire",
                Self::CommunicatesOpenly => "Communique ouvertement",
                Self::SeeksCompromise => "Cherche un compromis",
                Self::BecomesDefensive => "Devient défensif",
                Self::SharesCredit => "Partage le crédit",
                Self::SetsNewGoals => "Se fixe de nouveaux objectifs",
                Self::BecomesOverconfident => "Devient trop confiant",
                Self::AsksQuestions => "Pose des questions",
                Self::SeeksData => "Cherche des données",
                Self::OverPlans => "Planifie trop",
                Self::AppreciatesPraise => "Apprécie les éloges",
                Self::SharesAchievement => "Partage ses réussites",
                Self::SeeksMore => "Cherche plus de validation",
                Self::StandsGround => "Tient bon",
                Self::SeeksAllies => "Cherche des alliés",
                Self::DeflectsBlame => "Détourne le blâme",
                Self::EmbracesChange => "Accepte le changement",
                Self::PlansAhead => "Planifie à l'avance",
                Self::ResistsChange => "Résiste au changement",
                Self::AsksForDetails => "Demande des détails",
                Self::Reflects => "Réfléchit avec soin",
                Self::RejectsFeedback => "Rejette le feedback",
            },
        }
    }
}
