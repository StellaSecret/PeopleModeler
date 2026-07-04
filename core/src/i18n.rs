use crate::models::{BiasType, MotivationType};

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
