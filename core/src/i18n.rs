use crate::models::{
    BehaviorResponse, BiasType, MotivationType, RepDim, StyleCategory, StyleType, ValueType,
};

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

pub struct ValueI18n {
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

impl ValueType {
    pub fn i18n(&self, lang: Lang) -> ValueI18n {
        match lang {
            Lang::Fr => match self {
                Self::Career => ValueI18n {
                    label: "Carrière",
                    desc: "Ambition professionnelle et priorités de vie au travail",
                },
                Self::Family => ValueI18n {
                    label: "Famille",
                    desc: "Priorités familiales et temps avec les proches",
                },
                Self::Health => ValueI18n {
                    label: "Santé",
                    desc: "Bien-être physique et mental",
                },
                Self::Wealth => ValueI18n {
                    label: "Richesse",
                    desc: "Sécurité financière et confort matériel",
                },
                Self::Stability => ValueI18n {
                    label: "Stabilité",
                    desc: "Prévisibilité, routine et faible incertitude",
                },
                Self::Adventure => ValueI18n {
                    label: "Aventure",
                    desc: "Nouveauté, risque et expériences nouvelles",
                },
                Self::Community => ValueI18n {
                    label: "Communauté",
                    desc: "Contribution sociale et appartenance à un groupe",
                },
                Self::Knowledge => ValueI18n {
                    label: "Savoir",
                    desc: "Apprentissage, expertise et compréhension",
                },
                Self::Faith => ValueI18n {
                    label: "Foi",
                    desc: "Croyances spirituelles et traditions",
                },
                Self::Loyalty => ValueI18n {
                    label: "Loyauté",
                    desc: "Fidélité, engagement et liens durables",
                },
            },
            Lang::En => match self {
                Self::Career => ValueI18n {
                    label: "Career",
                    desc: "Professional ambition and work-life priorities",
                },
                Self::Family => ValueI18n {
                    label: "Family",
                    desc: "Family priorities and time with loved ones",
                },
                Self::Health => ValueI18n {
                    label: "Health",
                    desc: "Physical and mental well-being",
                },
                Self::Wealth => ValueI18n {
                    label: "Wealth",
                    desc: "Financial security and material comfort",
                },
                Self::Stability => ValueI18n {
                    label: "Stability",
                    desc: "Predictability, routine and low uncertainty",
                },
                Self::Adventure => ValueI18n {
                    label: "Adventure",
                    desc: "Novelty, risk and new experiences",
                },
                Self::Community => ValueI18n {
                    label: "Community",
                    desc: "Social contribution and belonging to a group",
                },
                Self::Knowledge => ValueI18n {
                    label: "Knowledge",
                    desc: "Learning, expertise and understanding",
                },
                Self::Faith => ValueI18n {
                    label: "Faith",
                    desc: "Spiritual beliefs and traditions",
                },
                Self::Loyalty => ValueI18n {
                    label: "Loyalty",
                    desc: "Fidelity, commitment and long-term bonds",
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
                    desc: "Ne retient que les infos qui confirment ce qu'il croit déjà, en ignorant le reste. Ex. : relit surtout les sources qui lui donnent raison",
                },
                Self::Anchoring => BiasI18n {
                    label: "Ancrage cognitif",
                    desc: "Cale sa décision sur le premier chiffre ou le premier fait rencontré. Ex. : une première offre ancre ce qu'il acceptera ensuite",
                },
                Self::Availability => BiasI18n {
                    label: "Disponibilité",
                    desc: "Estime la probabilité d'un événement à la facilité d'en évoquer des exemples — le récent et le dramatique pèsent plus. Ex. : un accident spectaculaire lui fait croire que voler est dangereux",
                },
                Self::SunkCost => BiasI18n {
                    label: "Coût irrécupérable",
                    desc: "Poursuit une voie perdante à cause de ce qui a déjà été investi. Ex. : refuse d'abandonner un projet raté après des années d'efforts",
                },
                Self::DunningKruger => BiasI18n {
                    label: "Dunning-Kruger",
                    desc: "Surestime son propre niveau — classique excès de confiance des débutants. Ex. : un débutant se prend pour l'expert",
                },
                Self::Impostor => BiasI18n {
                    label: "Imposteur",
                    desc: "Sous-estime ses compétences réelles et se sent imposteur. Ex. : un excellent profil craint d'être démasqué",
                },
                Self::LossAversion => BiasI18n {
                    label: "Aversion aux pertes",
                    desc: "Craint plus les pertes qu'il ne goûte les gains équivalents. Ex. : refuse un pari à chances égales",
                },
                Self::SocialProof => BiasI18n {
                    label: "Preuve sociale",
                    desc: "Copie le comportement du groupe quand il est incertain. Ex. : change sa réponse pour rejoindre le consensus",
                },
                Self::Authority => BiasI18n {
                    label: "Autorité",
                    desc: "Se plie aux titres et au rang plutôt qu'à son propre jugement. Ex. : suit sans questionner une mauvaise décision de son supérieur",
                },
                Self::Recency => BiasI18n {
                    label: "Récence",
                    desc: "Pèse plus les informations récentes que la tendance de long terme. Ex. : un mauvais trimestre efface des années de bons résultats",
                },
                Self::InGroup => BiasI18n {
                    label: "Endogroupe",
                    desc: "Favorise le « nous » face au « eux » — son équipe ou sa famille bénéficie du doute. Ex. : défend son camp même quand il a clairement tort",
                },
                Self::Favoritism => BiasI18n {
                    label: "Favoritisme",
                    desc: "Traite certains individus mieux que les autres pour des raisons personnelles. Ex. : promeut toujours ses amis",
                },
            },
            Lang::En => match self {
                Self::Confirmation => BiasI18n {
                    label: "Confirmation bias",
                    desc: "Filters for info that backs what they already believe, ignoring the rest. E.g. only re-reads sources that agree with them",
                },
                Self::Anchoring => BiasI18n {
                    label: "Anchoring",
                    desc: "Hangs a decision on the first number or fact they see. E.g. a first offer anchors what they'll accept later",
                },
                Self::Availability => BiasI18n {
                    label: "Availability",
                    desc: "Judges how likely something is by how easily examples come to mind, so dramatic recent events loom large. E.g. one scary crash makes flying feel dangerous",
                },
                Self::SunkCost => BiasI18n {
                    label: "Sunk cost",
                    desc: "Keeps a losing course going because of what's already been invested. E.g. refuses to drop a failing project after sinking years into it",
                },
                Self::DunningKruger => BiasI18n {
                    label: "Dunning-Kruger",
                    desc: "Overrates their own skill, typically low-competence overconfidence. E.g. a beginner insists they're the expert",
                },
                Self::Impostor => BiasI18n {
                    label: "Impostor",
                    desc: "Downrates their proven skill and feels like a fraud. E.g. a top performer fears being 'found out'",
                },
                Self::LossAversion => BiasI18n {
                    label: "Loss aversion",
                    desc: "Fears losing more than they enjoy winning the same amount. E.g. refuses an even-odds bet",
                },
                Self::SocialProof => BiasI18n {
                    label: "Social proof",
                    desc: "Copies what the group does when unsure. E.g. changes their answer to match the room",
                },
                Self::Authority => BiasI18n {
                    label: "Authority",
                    desc: "Defers to titles and rank over their own judgment. E.g. follows a senior's bad call without question",
                },
                Self::Recency => BiasI18n {
                    label: "Recency",
                    desc: "Weights the latest news over the longer track record. E.g. last week's bad quarter outweighs years of solid results",
                },
                Self::InGroup => BiasI18n {
                    label: "In-group",
                    desc: "Favors 'us' over 'them' — their own team or family gets the benefit of the doubt. E.g. backs the home team even when clearly wrong",
                },
                Self::Favoritism => BiasI18n {
                    label: "Favoritism",
                    desc: "Treats certain individuals better than others for personal reasons. E.g. always promotes their friends",
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
                Self::HardworkerLazy => {
                    "Effort et diligence — 8+ mène ses tâches à terme au travail et finit ce qu'il entreprend à la maison ; 2 glande ou abandonne au premier obstacle."
                }
                Self::AuthoritativeSubmissive => {
                    "Aptitude à mener — 8+ prend les choses en main et délègue au travail, organise les sorties entre amis ; 2 attend les ordres et laisse les autres décider."
                }
                Self::HonestDeceitful => {
                    "Sincérité — 8+ reconnaît ses erreurs et tient parole au travail, dit la vérité à ses proches ; 2 ment ou dissimule."
                }
                Self::ReliableFlaky => {
                    "Respecte ses engagements — 8+ tient les échéances au travail, se présente aux plans entre amis ; 2 se désiste ou annule au dernier moment."
                }
                Self::HumbleArrogant => {
                    "Reconnaît ses limites et crédite les autres — 8+ valorise ses collègues au travail, remercie qui a cuisiné à la maison ; 2 s'attribue tout le mérite."
                }
                Self::CalmReactive => {
                    "Sang-froid sous pression — 8+ garde son calme en crise au travail, reste posé lors des disputes familiales ; 2 explose ou panique."
                }
                Self::DiplomaticBlunt => {
                    "Façon d'annoncer les messages difficiles — 8+ atténue les critiques au travail comme à la maison ; 2 dit les choses crues et coupantes à tous."
                }
                Self::GenerousSelfish => {
                    "Partage temps, ressources, mérite — 8+ aide ses collègues au travail, participe à la maison ; 2 garde tout pour lui."
                }
                Self::FairFavoritism => {
                    "Applique la même règle à tous — 8+ gère équipes et foyers sans favoritisme ; 2 favorise ses proches au détriment des autres."
                }
                Self::TrustingSuspicious => {
                    "Part du principe que les gens sont de bonne foi — 8+ croit ses collègues sur parole, fait confiance à ses amis ; 2 voit des intentions hostiles partout."
                }
                Self::AssertivePassive => {
                    "Fait valoir sa position — 8+ dit son opinion au travail, pose des limites en famille ; 2 se tait et s'efface."
                }
                Self::EmpatheticDetached => {
                    "Ressent ce que ressentent les autres — 8+ capte le moral de l'équipe au travail, remarque la mauvaise journée d'un ami ; 2 reste distant et froid."
                }
                Self::AdaptableRigid => {
                    "S'adapte quand les circonstances changent — 8+ adopte les nouveaux outils au travail, suit la nouvelle routine des enfants ; 2 s'accroche à l'existant."
                }
            },
            Lang::En => match self {
                Self::HardworkerLazy => {
                    "Effort and diligence — 8+ pushes tasks to done at work, and finishes what they start at home; 2 coasts or quits at the first obstacle."
                }
                Self::AuthoritativeSubmissive => {
                    "Willingness to lead — 8+ takes charge and delegates at work, and organizes weekend plans with friends; 2 waits for orders and lets others decide."
                }
                Self::HonestDeceitful => {
                    "Truthfulness — 8+ owns mistakes and keeps their word at work, and tells the truth to friends and family; 2 lies or covers up."
                }
                Self::ReliableFlaky => {
                    "Keeps commitments — 8+ meets deadlines at work, and shows up for plans with friends; 2 flakes or cancels at the last minute."
                }
                Self::HumbleArrogant => {
                    "Acknowledges limits and credits others — 8+ praises teammates at work, and thanks the cook instead of claiming the meal; 2 claims all the credit."
                }
                Self::CalmReactive => {
                    "Steadiness under pressure — 8+ stays level-headed in a crisis at work, and keeps cool during family arguments; 2 snaps or panics."
                }
                Self::DiplomaticBlunt => {
                    "How they deliver hard messages — 8+ softens criticism in meetings and at home; 2 says it straight and sharp to everyone."
                }
                Self::GenerousSelfish => {
                    "Shares time, resources, credit — 8+ helps colleagues at work, chips in at home; 2 keeps everything for themselves."
                }
                Self::FairFavoritism => {
                    "Applies the same standard to everyone — 8+ rules teams and households without favor; 2 favors friends and family over everyone."
                }
                Self::TrustingSuspicious => {
                    "Assumes people mean well — 8+ takes colleagues at their word, trusts friends; 2 sees hidden hostile intent everywhere."
                }
                Self::AssertivePassive => {
                    "Speaks up for their own position — 8+ voices opinions at work and sets boundaries with family; 2 stays silent and defers."
                }
                Self::EmpatheticDetached => {
                    "Feels what others feel — 8+ picks up on team morale at work, senses a friend's bad day at home; 2 stays aloof and cold."
                }
                Self::AdaptableRigid => {
                    "Adjusts when circumstances change — 8+ adopts new tools at work, tries the kids' new routine at home; 2 insists things stay as they were."
                }
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
            (Self::TrustStyle, Lang::En) => "🔗 Trust Style",
            (Self::TrustStyle, Lang::Fr) => "🔗 Style de confiance",
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
                // Trust style
                Self::ExtendsTrustFreely => "Fait confiance facilement",
                Self::EarnsTrustGradually => "Gagne la confiance progressivement",
                Self::VerifiesTrust => "Fait confiance mais vérifie",
                Self::Guarded => "Méfiant",
                Self::RepairsTrustActively => "Répare la confiance activement",
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
                // Trust style
                Self::ExtendsTrustFreely => "Extends Trust Freely",
                Self::EarnsTrustGradually => "Earns Trust Gradually",
                Self::VerifiesTrust => "Verifies Trust",
                Self::Guarded => "Guarded",
                Self::RepairsTrustActively => "Repairs Trust Actively",
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
                // Trust style
                Self::ExtendsTrustFreely => {
                    "Accorde sa confiance facilement, donne le bénéfice du doute"
                }
                Self::EarnsTrustGradually => "Construit la confiance par la fiabilité démontrée",
                Self::VerifiesTrust => "Fait confiance mais vérifie par les actions",
                Self::Guarded => "Prudent, a besoin de preuves avant de faire confiance",
                Self::RepairsTrustActively => "Recrée activement la confiance après une brèche",
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
                // Trust style
                Self::ExtendsTrustFreely => "Gives trust easily, offers benefit of the doubt",
                Self::EarnsTrustGradually => "Builds trust through demonstrated reliability",
                Self::VerifiesTrust => "Trusts but verifies through actions",
                Self::Guarded => "Cautious, needs proof before trusting",
                Self::RepairsTrustActively => "Proactively rebuilds trust after a breach",
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
