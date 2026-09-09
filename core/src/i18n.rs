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
                    desc: "Contrôle des décisions et influence — au travail : prend les rênes et mène ; dans la vie : aime être consulté et décider",
                },
                Self::Achievement => MotI18n {
                    label: "Accomplissement",
                    desc: "Objectifs ambitieux et haute performance — au travail : se fixe des cibles exigeantes ; dans la vie : se mesure via sports, loisirs ou jalons personnels",
                },
                Self::Affiliation => MotI18n {
                    label: "Appartenance",
                    desc: "Relations harmonieuses et appartenance — au travail : valorise une équipe soudée et évite les conflits ; dans la vie : s'entoure d'amis proches",
                },
                Self::Security => MotI18n {
                    label: "Sécurité",
                    desc: "Stabilité, prévisibilité et évitement des risques — au travail : préfère un rôle stable et des règles claires ; dans la vie : tient aux habitudes, budgets et routines",
                },
                Self::Autonomy => MotI18n {
                    label: "Autonomie",
                    desc: "Indépendance et liberté d'action — au travail : veut gérer ses tâches à sa façon ; dans la vie : organise librement son temps et son emploi du temps",
                },
                Self::Recognition => MotI18n {
                    label: "Reconnaissance",
                    desc: "Validation et estime des autres — au travail : veut que son travail soit vu et loué ; dans la vie : partage ses réussites et apprécie les compliments",
                },
                Self::Learning => MotI18n {
                    label: "Apprentissage",
                    desc: "Connaissances et développement personnel — au travail : suit des formations et creuse les nouveautés ; dans la vie : lit, explore et continue d'apprendre",
                },
                Self::Helping => MotI18n {
                    label: "Aider les autres",
                    desc: "Aider et soutenir les autres — au travail : assiste ses collègues et prend des tâches de service ; dans la vie : soutient famille, amis et communauté",
                },
                Self::Creativity => MotI18n {
                    label: "Créativité",
                    desc: "Innovation, expression et exploration créative — au travail : invente des approches et des idées neuves ; dans la vie : écrit, fabrique ou bricole",
                },
                Self::Fairness => MotI18n {
                    label: "Justice",
                    desc: "Équité et traitement juste des autres — au travail : dénonce le favoritisme et partage équitablement ; dans la vie : défend la justice et le mérite individuel",
                },
            },
            Lang::En => match self {
                Self::Power => MotI18n {
                    label: "Power",
                    desc: "Control over decisions and influence — at work: takes charge and leads; in everyday life: likes being consulted and deciding",
                },
                Self::Achievement => MotI18n {
                    label: "Achievement",
                    desc: "Ambitious goals and high performance — at work: sets demanding targets and chases mastery; in everyday life: competes in sports, hobbies, or personal milestones",
                },
                Self::Affiliation => MotI18n {
                    label: "Affiliation",
                    desc: "Harmonious relationships and belonging — at work: values a friendly team and avoids conflict; in everyday life: surrounds themselves with close friends",
                },
                Self::Security => MotI18n {
                    label: "Security",
                    desc: "Stability, predictability and risk avoidance — at work: prefers a steady role and clear rules; in everyday life: sticks to habits, budgets, and routines",
                },
                Self::Autonomy => MotI18n {
                    label: "Autonomy",
                    desc: "Independence and freedom of action — at work: wants to run their tasks their own way; in everyday life: manages their time and schedule freely",
                },
                Self::Recognition => MotI18n {
                    label: "Recognition",
                    desc: "Validation and esteem from others — at work: wants others to see and praise their work; in everyday life: shares wins and appreciates compliments",
                },
                Self::Learning => MotI18n {
                    label: "Learning",
                    desc: "Knowledge and personal development — at work: takes courses and digs into new topics; in everyday life: reads, explores, and keeps growing",
                },
                Self::Helping => MotI18n {
                    label: "Helping others",
                    desc: "Helping and supporting others — at work: assists colleagues and takes on service tasks; in everyday life: supports family, friends, and community",
                },
                Self::Creativity => MotI18n {
                    label: "Creativity",
                    desc: "Innovation, expression and creative exploration — at work: invents novel approaches and new ideas; in everyday life: writes, makes, or crafts",
                },
                Self::Fairness => MotI18n {
                    label: "Fairness",
                    desc: "Equity and fair treatment of others — at work: speaks up against favoritism and shares fairly; in everyday life: defends justice and individual merit",
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
                    desc: "Ambition professionnelle — au travail : vise promotions et responsabilités ; dans la vie : mesure sa réussite à sa carrière",
                },
                Self::Family => ValueI18n {
                    label: "Famille",
                    desc: "Temps passé avec les proches — au travail : aménage son agenda autour des moments en famille ; dans la vie : privilégie dîners, appels et vacances ensemble",
                },
                Self::Health => ValueI18n {
                    label: "Santé",
                    desc: "Bien-être physique et mental — au travail : protège l'équilibre vie pro/vie perso ; dans la vie : s'investit dans l'exercice, le sommeil et le stress",
                },
                Self::Wealth => ValueI18n {
                    label: "Richesse",
                    desc: "Sécurité financière et confort matériel — au travail : cherche revenus et promotions ; dans la vie : épargne, investit et planifie la stabilité",
                },
                Self::Stability => ValueI18n {
                    label: "Stabilité",
                    desc: "Prévisibilité et faible incertitude — au travail : préfère des tâches claires et des routines stables ; dans la vie : évite surprises et changements brusques",
                },
                Self::Adventure => ValueI18n {
                    label: "Aventure",
                    desc: "Nouveauté, risque et expériences — au travail : se porte volontaire pour des projets ambitieux ; dans la vie : voyage et essaie de nouveaux loisirs",
                },
                Self::Community => ValueI18n {
                    label: "Communauté",
                    desc: "Appartenance et contribution à un groupe — au travail : s'implique dans son équipe et ses causes ; dans la vie : fait du bénévolat ou rejoint des groupes locaux",
                },
                Self::Knowledge => ValueI18n {
                    label: "Savoir",
                    desc: "Apprentissage, expertise et compréhension — au travail : étudie son domaine en profondeur ; dans la vie : lit beaucoup et suit ses curiosités",
                },
                Self::Faith => ValueI18n {
                    label: "Foi",
                    desc: "Croyances spirituelles et traditions — au travail : cherche du sens et une culture compatible ; dans la vie : pratique rituels et communauté",
                },
                Self::Loyalty => ValueI18n {
                    label: "Loyauté",
                    desc: "Fidélité et liens durables — au travail : reste fidèle aux équipes et défend ses collègues ; dans la vie : valorise les amitiés et relations qui durent",
                },
            },
            Lang::En => match self {
                Self::Career => ValueI18n {
                    label: "Career",
                    desc: "Professional ambition — at work: pushes for promotions and bigger responsibilities; in life: measures success by their career path",
                },
                Self::Family => ValueI18n {
                    label: "Family",
                    desc: "Time with loved ones — at work: schedules around family moments; in life: prioritizes dinners, calls, and holidays together",
                },
                Self::Health => ValueI18n {
                    label: "Health",
                    desc: "Physical and mental well-being — at work: protects work-life balance; in life: commits to exercise, sleep, and managing stress",
                },
                Self::Wealth => ValueI18n {
                    label: "Wealth",
                    desc: "Financial security and material comfort — at work: seeks income and promotions; in life: saves, invests, and plans for stability",
                },
                Self::Stability => ValueI18n {
                    label: "Stability",
                    desc: "Predictability and low uncertainty — at work: prefers clear tasks and steady routines; in life: avoids surprises and sudden change",
                },
                Self::Adventure => ValueI18n {
                    label: "Adventure",
                    desc: "Novelty, risk and new experiences — at work: volunteers for stretch projects; in life: travels and tries new hobbies",
                },
                Self::Community => ValueI18n {
                    label: "Community",
                    desc: "Social contribution and belonging — at work: engages in teams and causes; in life: volunteers or joins local groups",
                },
                Self::Knowledge => ValueI18n {
                    label: "Knowledge",
                    desc: "Learning, expertise and understanding — at work: studies their field deeply; in life: reads widely and follows curiosities",
                },
                Self::Faith => ValueI18n {
                    label: "Faith",
                    desc: "Spiritual beliefs and traditions — at work: seeks meaning and a compatible culture; in life: practices rituals and community",
                },
                Self::Loyalty => ValueI18n {
                    label: "Loyalty",
                    desc: "Fidelity and long-term bonds — at work: stays loyal to teams and defends colleagues; in life: values friendships and relationships that last",
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
                    desc: "Ne retient que les infos qui confirment ce qu'il croit déjà, en ignorant le reste. Au travail : relit surtout les sources qui lui donnent raison ; dans la vie : ne suit que des avis conformes à ses convictions",
                },
                Self::Anchoring => BiasI18n {
                    label: "Ancrage cognitif",
                    desc: "Cale sa décision sur le premier chiffre ou le premier fait rencontré. Au travail : la première estimation d'un budget scelle ce qu'il acceptera ; dans la vie : juge un achat au premier prix vu",
                },
                Self::Availability => BiasI18n {
                    label: "Disponibilité",
                    desc: "Estime la probabilité d'un événement à la facilité d'en évoquer des exemples — le récent et le dramatique pèsent plus. Au travail : un bug récent lui fait croire que le système est fragile ; dans la vie : un accident spectaculaire lui fait éviter l'avion",
                },
                Self::SunkCost => BiasI18n {
                    label: "Coût irrécupérable",
                    desc: "Poursuit une voie perdante à cause de ce qui a déjà été investi. Au travail : refuse d'abandonner un projet raté après des années d'efforts ; dans la vie : finit un livre ou un abonnement qu'il n'aime plus parce qu'il a payé",
                },
                Self::DunningKruger => BiasI18n {
                    label: "Dunning-Kruger",
                    desc: "Surestime son propre niveau — classique excès de confiance des débutants. Au travail : un débutant se prend pour l'expert et surcharge sa mission ; dans la vie : donne des conseils assurés dans des domaines qu'il maîtrise peu",
                },
                Self::Impostor => BiasI18n {
                    label: "Imposteur",
                    desc: "Sous-estime ses compétences réelles et se sent imposteur. Au travail : un excellent profil craint d'être démasqué à chaque promo ; dans la vie : minimise ses réussites personnelles",
                },
                Self::LossAversion => BiasI18n {
                    label: "Aversion aux pertes",
                    desc: "Craint plus les pertes qu'il ne goûte les gains équivalents. Au travail : refuse un pari à chances égales ; dans la vie : garde un placement perdant plutôt que d'assumer la perte",
                },
                Self::SocialProof => BiasI18n {
                    label: "Preuve sociale",
                    desc: "Copie le comportement du groupe quand il est incertain. Au travail : change sa réponse pour rejoindre le consensus ; dans la vie : choisit un restaurant parce qu'il est bondé",
                },
                Self::Authority => BiasI18n {
                    label: "Autorité",
                    desc: "Se plie aux titres et au rang plutôt qu'à son propre jugement. Au travail : suit sans questionner une mauvaise décision de son supérieur ; dans la vie : se range à l'avis d'un spécialiste médiatique",
                },
                Self::Recency => BiasI18n {
                    label: "Récence",
                    desc: "Pèse plus les informations récentes que la tendance de long terme. Au travail : un mauvais trimestre efface des années de bons résultats ; dans la vie : décide selon son humeur récente",
                },
                Self::InGroup => BiasI18n {
                    label: "Endogroupe",
                    desc: "Favorise le « nous » face au « eux ». Au travail : défend son équipe même quand elle a clairement tort ; dans la vie : son camp ou sa famille bénéficie toujours du doute",
                },
                Self::Favoritism => BiasI18n {
                    label: "Favoritisme",
                    desc: "Traite certains individus mieux que les autres pour des raisons personnelles. Au travail : promeut toujours ses amis ; dans la vie : prête plus facilement à ses proches",
                },
            },
            Lang::En => match self {
                Self::Confirmation => BiasI18n {
                    label: "Confirmation bias",
                    desc: "Filters for info that backs what they already believe, ignoring the rest. At work: re-reads only the sources that agree with them; in everyday life: follows only views aligned with their opinions",
                },
                Self::Anchoring => BiasI18n {
                    label: "Anchoring",
                    desc: "Hangs a decision on the first number or fact they see. At work: a budget's first estimate seals what they'll accept; in everyday life: judges a purchase by the first price they saw",
                },
                Self::Availability => BiasI18n {
                    label: "Availability",
                    desc: "Judges how likely something is by how easily examples come to mind, so dramatic recent events loom large. At work: one recent bug makes the whole system feel fragile; in everyday life: a scary crash makes flying feel dangerous",
                },
                Self::SunkCost => BiasI18n {
                    label: "Sunk cost",
                    desc: "Keeps a losing course going because of what's already been invested. At work: refuses to drop a failing project after sinking years into it; in everyday life: finishes a dull book or membership they already paid for",
                },
                Self::DunningKruger => BiasI18n {
                    label: "Dunning-Kruger",
                    desc: "Overrates their own skill, typically low-competence overconfidence. At work: a beginner insists they're the expert and overpromises; in everyday life: gives confident advice on topics they know little about",
                },
                Self::Impostor => BiasI18n {
                    label: "Impostor",
                    desc: "Downrates their proven skill and feels like a fraud. At work: a top performer fears being found out at every promotion; in everyday life: plays down their personal achievements",
                },
                Self::LossAversion => BiasI18n {
                    label: "Loss aversion",
                    desc: "Fears losing more than they enjoy winning the same amount. At work: refuses an even-odds bet; in everyday life: holds a losing investment rather than taking the loss",
                },
                Self::SocialProof => BiasI18n {
                    label: "Social proof",
                    desc: "Copies what the group does when unsure. At work: changes their answer to match the room; in everyday life: picks the busy restaurant",
                },
                Self::Authority => BiasI18n {
                    label: "Authority",
                    desc: "Defers to titles and rank over their own judgment. At work: follows a senior's bad call without question; in everyday life: defers to a media pundit",
                },
                Self::Recency => BiasI18n {
                    label: "Recency",
                    desc: "Weights the latest news over the longer track record. At work: last week's bad quarter outweighs years of solid results; in everyday life: decides based on recent moods",
                },
                Self::InGroup => BiasI18n {
                    label: "In-group",
                    desc: "Favors 'us' over 'them'. At work: defends their own team even when it's clearly wrong; in everyday life: their camp or family always gets the benefit of the doubt",
                },
                Self::Favoritism => BiasI18n {
                    label: "Favoritism",
                    desc: "Treats certain individuals better than others for personal reasons. At work: always promotes their friends; in everyday life: lends more easily to close ones",
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
                Self::DirectCommunicator => {
                    "Parle franchement et va droit au but — au travail : donne un retour direct en réunion ; dans la vie : dit ce qu'il pense sans adoucir"
                }
                Self::DiplomaticCommunicator => {
                    "Adoucit son langage pour ménager les autres — au travail : nuance sa critique quand il présente ; dans la vie : annonce les mauvaises nouvelles avec tact"
                }
                Self::ReservedCommunicator => {
                    "Parle peu, choisit ses mots avec soin — au travail : contribue en quelques phrases pesées ; dans la vie : écoute plus qu'il ne parle"
                }
                Self::ExpressiveCommunicator => {
                    "Partage ses pensées et émotions ouvertement — au travail : exprime sentiments et idées ; dans la vie : raconte des histoires avec vivacité"
                }
                Self::Competing => {
                    "Cherche à gagner, confronte directement — au travail : vise les objectifs et bat ses concurrents ; dans la vie : transforme jeux et débats en compétition"
                }
                Self::Collaborating => {
                    "Cherche une solution qui satisfait tout le monde — au travail : fusionne les idées en solution commune ; dans la vie : organise des sorties où chacun est inclus"
                }
                Self::Compromising => {
                    "Accepte des concessions mutuelles — au travail : partage la différence pour avancer ; dans la vie : fait des compromis quand il planifie avec ses proches"
                }
                Self::Avoiding => {
                    "Évite la confrontation, laisse faire — au travail : fuit les conversations difficiles ; dans la vie : change de sujet plutôt que se disputer"
                }
                Self::Accommodating => {
                    "Cède pour préserver l'harmonie — au travail : laisse tomber son avis pour éviter les frictions ; dans la vie : laisse les autres choisir le resto, le film, le plan"
                }
                Self::Analytical => {
                    "Décide après analyse approfondie des données — au travail : construit tableaux et modèles avant d'agir ; dans la vie : lit les avis et compare avant d'acheter"
                }
                Self::Intuitive => {
                    "Décide par instinct et ressenti — au travail : suit son intuition plutôt que les données ; dans la vie : se fie à son feeling sur les gens et les choix"
                }
                Self::Participatory => {
                    "Implique les autres dans la décision — au travail : consulte l'équipe avant de trancher ; dans la vie : demande l'avis de tous avant de choisir"
                }
                Self::Autocratic => {
                    "Décide seul, sans consultation — au travail : fixe la direction et attend l'exécution ; dans la vie : planifie l'agenda familial seul"
                }
                Self::ConsensusDriven => {
                    "Cherche l'accord unanime avant de décider — au travail : attend que tout le monde soit d'accord ; dans la vie : attend l'approbation de toute la famille"
                }
                Self::Visionary => {
                    "Inspire avec une vision à long terme — au travail : dessine la direction de l'équipe ; dans la vie : projette des plans de vie à long terme"
                }
                Self::Servant => {
                    "Place les besoins de l'équipe en premier — au travail : soutient et débloque ses collègues ; dans la vie : prend les tâches ménagères pour que les autres se reposent"
                }
                Self::Transactional => {
                    "Gère par récompenses et sanctions — au travail : associe primes et résultats ; dans la vie : traite les services rendus comme à rembourser"
                }
                Self::Transformational => {
                    "Transforme et élève ses collaborateurs — au travail : coache les gens pour qu'ils se dépassent ; dans la vie : pousse ses amis vers leurs objectifs"
                }
                Self::Bureaucratic => {
                    "Suit les procédures et la hiérarchie — au travail : s'en tient au process et à l'organigramme ; dans la vie : aime les règles et les horaires clairs"
                }
                Self::PastOriented => {
                    "Se réfère aux expériences passées — au travail : réutilise ce qui a marché avant ; dans la vie : aime traditions, souvenirs et routines habituelles"
                }
                Self::PresentOriented => {
                    "Vit dans l'instant présent — au travail : se concentre sur les tâches du jour ; dans la vie : profite de l'ici et maintenant sans planifier"
                }
                Self::FutureOriented => {
                    "Planifie et anticipe l'avenir — au travail : révise feuilles de route et prévisions ; dans la vie : épargne et projette des années en avance"
                }
                Self::RuleBased => {
                    "Suit des principes moraux universels — au travail : applique les mêmes règles à tous ; dans la vie : tient bon sur ce qu'il juge juste pour tous"
                }
                Self::OutcomeBased => {
                    "Juge la moralité par les conséquences — au travail : pèse les résultats plus que le process ; dans la vie : juge juste ce dont l'issue est bénéfique"
                }
                Self::VirtueBased => {
                    "Cultive des qualités de caractère — au travail : pratique intégrité et assiduité ; dans la vie : s'efforce d'être honnête, gentil et discipliné"
                }
                Self::Relativist => {
                    "Adapte sa morale au contexte — au travail : assouplit les règles quand la situation le demande ; dans la vie : juge chaque cas sur sa propre valeur"
                }
                // Interpersonal conduct
                Self::Opportunistic => {
                    "Exploite les situations et les gens pour son profit — au travail : saisit les occasions aux dépens des autres ; dans la vie : se met en premier quand ça rapporte"
                }
                Self::Intrusive => {
                    "Franchit les limites, s'impose aux autres — au travail : s'immisce dans les tâches et agendas des autres ; dans la vie : lit les messages, donne des avis non demandés"
                }
                Self::Manipulative => {
                    "Orchestre les autres par la tromperie — au travail : influence les gens avec des demi-vérités ; dans la vie : joue sur les sentiments pour orienter les gens"
                }
                Self::PassiveAggressive => {
                    "Résistance indirecte, sabotage subtil — au travail : bloque discrètement les projets au lieu de protester ; dans la vie : boude et fait des remarques pointues"
                }
                Self::Controlling => {
                    "Domine et micro-gère les autres — au travail : contrôle chaque détail du travail de l'équipe ; dans la vie : décide et supervise le quotidien à la maison"
                }
                Self::Detached => {
                    "Maintient une distance émotionnelle, objectif — au travail : reste clinique même dans les moments tendus ; dans la vie : semble distant dans les affaires personnelles"
                }
                Self::Respectful => {
                    "Respecte les limites et l'autonomie — au travail : demande avant de toucher au travail des autres ; dans la vie : respecte la vie privée et les choix"
                }
                Self::Empathetic => {
                    "Comprend et valide les émotions des autres — au travail : perçoit l'humeur et les tensions de l'équipe ; dans la vie : sent quand un ami va mal et demande"
                }
                Self::Supportive => {
                    "Aide et encourage activement les autres — au travail : aide et encourage ses collègues ; dans la vie : se rend disponible pour ses amis en difficulté"
                }
                Self::Nurturing => {
                    "Investit dans la croissance des autres — au travail : développe le talent des gens dans la durée ; dans la vie : veille sur les progrès de famille et amis"
                }
                // Trust style
                Self::ExtendsTrustFreely => {
                    "Accorde sa confiance facilement, donne le bénéfice du doute — au travail : délègue sans vérifier ; dans la vie : fait confiance aux gens jusqu'à preuve du contraire"
                }
                Self::EarnsTrustGradually => {
                    "Construit la confiance par la fiabilité démontrée — au travail : gagne la confiance par la constance ; dans la vie : s'ouvre quand l'historique le justifie"
                }
                Self::VerifiesTrust => {
                    "Fait confiance mais vérifie par les actions — au travail : contrôle le travail délégué ; dans la vie : croit les paroles mais observe les actes"
                }
                Self::Guarded => {
                    "Prudent, a besoin de preuves avant de faire confiance — au travail : doute des promesses tant que les résultats ne sont pas là ; dans la vie : met du temps à laisser les gens approcher"
                }
                Self::RepairsTrustActively => {
                    "Recrée activement la confiance après une brèche — au travail : assume ses erreurs et répare les relations ; dans la vie : s'excuse et s'efforce de se racheter"
                }
            },
            Lang::En => match self {
                Self::DirectCommunicator => {
                    "Speaks frankly and goes straight to the point — at work: gives direct feedback in meetings; in everyday life: says what they think without softening"
                }
                Self::DiplomaticCommunicator => {
                    "Softens language to spare others' feelings — at work: rounds off a critical point when presenting; in everyday life: phrases bad news kindly"
                }
                Self::ReservedCommunicator => {
                    "Speaks little, chooses words carefully — at work: contributes measured remarks in meetings; in everyday life: listens more than they talk"
                }
                Self::ExpressiveCommunicator => {
                    "Shares thoughts and emotions openly — at work: speaks up about feelings and ideas; in everyday life: tells stories with animation"
                }
                Self::Competing => {
                    "Seeks to win, confronts directly — at work: drives to beat targets and rivals; in everyday life: turns games and debates into contests"
                }
                Self::Collaborating => {
                    "Seeks win-win solutions for everyone — at work: combines ideas into shared solutions; in everyday life: plans group outings where everyone is included"
                }
                Self::Compromising => {
                    "Accepts mutual concessions — at work: splits differences to keep projects moving; in everyday life: trades off when planning with loved ones"
                }
                Self::Avoiding => {
                    "Avoids confrontation, lets things slide — at work: ducks tough conversations; in everyday life: changes the subject rather than argue"
                }
                Self::Accommodating => {
                    "Yields to preserve harmony — at work: waves their own view to avoid friction; in everyday life: lets others choose the restaurant, the film, the plan"
                }
                Self::Analytical => {
                    "Decides after thorough data analysis — at work: builds spreadsheets and models before acting; in everyday life: reads reviews and compares before buying"
                }
                Self::Intuitive => {
                    "Decides by gut feeling and instinct — at work: trusts a hunch over deep data; in everyday life: goes with a feeling about people and choices"
                }
                Self::Participatory => {
                    "Involves others in the decision — at work: polls the team before deciding; in everyday life: asks everyone's opinion before choosing"
                }
                Self::Autocratic => {
                    "Decides alone without consultation — at work: sets direction and expects follow-through; in everyday life: plans the family agenda alone"
                }
                Self::ConsensusDriven => {
                    "Seeks unanimous agreement before deciding — at work: holds out until everyone agrees; in everyday life: waits for full family approval"
                }
                Self::Visionary => {
                    "Inspires with a long-term vision — at work: paints where the team is heading; in everyday life: dreams up long-term life plans"
                }
                Self::Servant => {
                    "Puts team needs first — at work: supports and unblocks colleagues; in everyday life: takes on the chores so others can rest"
                }
                Self::Transactional => {
                    "Manages through rewards and sanctions — at work: ties bonuses to results; in everyday life: treats favors as things to be returned"
                }
                Self::Transformational => {
                    "Transforms and elevates those around them — at work: coaches people to exceed themselves; in everyday life: pushes friends to reach their goals"
                }
                Self::Bureaucratic => {
                    "Follows procedures and hierarchy — at work: sticks to the process and the org chart; in everyday life: loves clear rules and schedules"
                }
                Self::PastOriented => {
                    "References past experiences — at work: reuses what worked before; in everyday life: enjoys traditions, memories, and usual routines"
                }
                Self::PresentOriented => {
                    "Lives in the present moment — at work: focuses on today's tasks; in everyday life: enjoys the here and now without planning"
                }
                Self::FutureOriented => {
                    "Plans and anticipates the future — at work: reviews roadmaps and forecasts; in everyday life: saves and projects years ahead"
                }
                Self::RuleBased => {
                    "Follows universal moral principles — at work: applies the same rules to everyone; in everyday life: holds firm on what they think is right for all"
                }
                Self::OutcomeBased => {
                    "Judges morality by consequences — at work: weighs results over process; in everyday life: calls something right when the outcome is beneficial"
                }
                Self::VirtueBased => {
                    "Cultivates character qualities — at work: practices integrity and diligence; in everyday life: strives to be honest, kind, and disciplined"
                }
                Self::Relativist => {
                    "Adapts morality to context — at work: bends the rules when the situation asks; in everyday life: judges each case on its own, not by fixed rules"
                }
                // Interpersonal conduct
                Self::Opportunistic => {
                    "Exploits situations and people for personal gain — at work: jumps on chances at others' expense; in everyday life: puts themselves first when it pays"
                }
                Self::Intrusive => {
                    "Oversteps boundaries, imposes on others — at work: barges into others' tasks and schedules; in everyday life: reads messages, gives unsolicited advice"
                }
                Self::Manipulative => {
                    "Orchestrates others through deception — at work: sways people with half-truths; in everyday life: plays on feelings to steer people"
                }
                Self::PassiveAggressive => {
                    "Indirect resistance, subtle sabotage — at work: quietly blocks projects instead of objecting; in everyday life: sulks and makes pointed remarks"
                }
                Self::Controlling => {
                    "Dominates and micromanages others — at work: checks every detail of the team's work; in everyday life: decides and oversees how things are done at home"
                }
                Self::Detached => {
                    "Maintains emotional distance, objective — at work: stays clinical even in tense moments; in everyday life: seems distant in personal matters"
                }
                Self::Respectful => {
                    "Honors boundaries and autonomy — at work: asks before touching others' work; in everyday life: respects others' privacy and choices"
                }
                Self::Empathetic => {
                    "Understands and validates others' feelings — at work: picks up on team mood and tensions; in everyday life: senses when friends are off and asks"
                }
                Self::Supportive => {
                    "Actively helps and encourages others — at work: helps and cheers colleagues on; in everyday life: shows up for friends in need"
                }
                Self::Nurturing => {
                    "Invests in others' growth and wellbeing — at work: develops people's talent over time; in everyday life: cares deeply about family and friends' progress"
                }
                // Trust style
                Self::ExtendsTrustFreely => {
                    "Gives trust easily, offers benefit of the doubt — at work: delegates without checking; in everyday life: trusts people until proven otherwise"
                }
                Self::EarnsTrustGradually => {
                    "Builds trust through demonstrated reliability — at work: wins confidence with consistency; in everyday life: opens up once the track record shows they can"
                }
                Self::VerifiesTrust => {
                    "Trusts but verifies through actions — at work: checks in on delegated work; in everyday life: trusts words but watches behavior"
                }
                Self::Guarded => {
                    "Cautious, needs proof before trusting — at work: doubts promises until results appear; in everyday life: slow to let people close"
                }
                Self::RepairsTrustActively => {
                    "Proactively rebuilds trust after a breach — at work: owns mistakes and mends relations; in everyday life: apologizes and works to make it up"
                }
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

    pub fn desc(self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => self.desc_en(),
            Lang::Fr => self.desc_fr(),
        }
    }

    fn desc_en(self) -> &'static str {
        match self {
            // Stress
            Self::RemainsCalm => {
                "Stays composed under pressure — at work: keeps working calmly as a deadline slips; in everyday life: stays steady when plans fall apart"
            }
            Self::SeeksSupport => {
                "Reaches out for help when overwhelmed — at work: asks colleagues for support on a heavy workload; in everyday life: leans on family and friends during hard times"
            }
            Self::StaysFocused => {
                "Channels stress into productivity — at work: powers through the busy period with a clear task list; in everyday life: picks a task or project to stay anchored"
            }
            Self::BecomesQuiet => {
                "Clams up and withdraws — at work: goes silent in meetings when tension rises; in everyday life: shuts down instead of talking it through"
            }
            Self::BecomesIrritable => {
                "Gets easily annoyed — at work: snaps at small interruptions during crunch time; in everyday life: gets testy at home when things press on them"
            }
            Self::Overwhelmed => {
                "Shuts down completely — at work: freezes in the face of a big backlog; in everyday life: gets stuck by mounting chores and worries"
            }
            Self::Panics => {
                "Loses control — at work: overreacts to an unexpected failure; in everyday life: catastrophizes when something goes wrong"
            }
            // Conflict
            Self::FacilitatesResolution => {
                "Mediates and finds common ground — at work: steps in to defuse team clashes; in everyday life: brokers peace between arguing relatives"
            }
            Self::CommunicatesOpenly => {
                "Expresses feelings constructively — at work: says what bothers them in a calm direct way; in everyday life: talks issues through instead of holding grudges"
            }
            Self::SeeksCompromise => {
                "Looks for a middle ground — at work: splits the difference on a disputed plan; in everyday life: suggests trade-offs so everyone gets something"
            }
            Self::StaysSilent => {
                "Avoids taking sides — at work: sits out the debate without taking a position; in everyday life: stays quiet in family disputes"
            }
            Self::BecomesPassiveAggressive => {
                "Uses pointed digs instead of honesty — at work: makes snide comments rather than raising the issue; in everyday life: gives the silent treatment and bitter remarks"
            }
            Self::BecomesDefensive => {
                "Blocks, argues, justifies — at work: answers criticism with excuses; in everyday life: turns disagreements into arguments to win"
            }
            Self::Escalates => {
                "Attacks personally — at work: makes the disagreement personal and heated; in everyday life: raises old grudges in a fight"
            }
            // Success
            Self::CelebratesWithOthers => {
                "Shares the joy and brings people together — at work: organizes a team celebration for the win; in everyday life: throws a party for a milestone"
            }
            Self::SharesCredit => {
                "Praises others' contributions — at work: names everyone on the project in the recap; in everyday life: tells people they made it possible"
            }
            Self::SetsNewGoals => {
                "Raises the bar — at work: immediately aims for the next target; in everyday life: signs up for a new personal challenge"
            }
            Self::EnjoysQuietly => {
                "Internal satisfaction without display — at work: lets results speak and skips the loud win; in everyday life: savors accomplishments privately"
            }
            Self::BecomesComplacent => {
                "Rests on their laurels — at work: coasts after hitting a big target; in everyday life: stops pushing once a goal is reached"
            }
            Self::BecomesOverconfident => {
                "Arrogant and boastful — at work: takes on more than they can deliver after a win; in everyday life: lectures others on their success"
            }
            Self::DismissesOthers => {
                "Belittles others' contributions — at work: claims the success while minimizing teammates' input; in everyday life: plays down others' achievements"
            }
            // Uncertainty
            Self::EmbracesAmbiguity => {
                "Thrives in the unknown — at work: jumps into vaguely scoped projects; in everyday life: enjoys open-ended plans and surprises"
            }
            Self::AsksQuestions => {
                "Seeks to understand — at work: clarifies goals and expectations; in everyday life: gathers details before deciding"
            }
            Self::SeeksData => {
                "Gathers facts — at work: pulls metrics before acting; in everyday life: researches before a purchase"
            }
            Self::WaitsForClarity => {
                "Temporizes — at work: holds decisions until direction is clear; in everyday life: postpones choices until the situation clears"
            }
            Self::OverPlans => {
                "Tries to control the uncertain — at work: over-schedules and over-documents; in everyday life: plans every step of a trip or event"
            }
            Self::BecomesParalyzed => {
                "Unable to act — at work: stalls on an ambiguous task; in everyday life: can't choose when options are left open"
            }
            Self::DeflectsResponsibility => {
                "Blames the ambiguity — at work: attributes delays to unclear instructions; in everyday life: blames circumstances rather than themselves"
            }
            // Recognition
            Self::AppreciatesQuietly => {
                "Values recognition without display — at work: is content with a discreet thank-you; in everyday life: values genuine praise over fanfare"
            }
            Self::AppreciatesPraise => {
                "Accepts compliments gracefully — at work: smiles and thanks the team for feedback; in everyday life: receives compliments warmly"
            }
            Self::SharesAchievement => {
                "Updates on progress — at work: keeps stakeholders posted on wins; in everyday life: tells close ones about their successes"
            }
            Self::SeeksMore => {
                "Needs some approval — at work: checks in for validation after tasks; in everyday life: wants reassurance from family and friends"
            }
            Self::BecomesJealous => {
                "Resents others' recognition — at work: begrudges a colleague's praise; in everyday life: envies friends' achievements"
            }
            Self::DemandsAttention => {
                "Must be the center — at work: steers conversations back to themselves; in everyday life: needs to be noticed in a group"
            }
            Self::UnderminesOthers => {
                "Diminishes others to get ahead — at work: downplays teammates' wins; in everyday life: cuts others down socially"
            }
            // Threatened
            Self::SeeksUnderstanding => {
                "Tries to understand the threat — at work: investigates the source of an attack; in everyday life: analyzes motives before reacting"
            }
            Self::SeeksAllies => {
                "Builds a support network — at work: lines up support from other teams; in everyday life: rallies friends when they feel attacked"
            }
            Self::StandsGround => {
                "Calmly defends their position — at work: holds their ground on a choice without escalating; in everyday life: stays firm but polite in arguments"
            }
            Self::BecomesCautious => {
                "Withdraws to assess — at work: pulls back to evaluate the situation; in everyday life: keeps distance while weighing a threat"
            }
            Self::DeflectsBlame => {
                "Redirects responsibility — at work: points to process or others when accused; in everyday life: finds someone else to carry the fault"
            }
            Self::Counterattacks => {
                "Strikes back — at work: retaliates against criticism; in everyday life: fires back in a dispute"
            }
            Self::BecomesParanoid => {
                "Sees threats everywhere — at work: reads attacks into neutral messages; in everyday life: suspects hidden motives in others"
            }
            // Change
            Self::EmbracesChange => {
                "Adapts quickly — at work: adopts new tools and processes readily; in everyday life: welcomes changes in plans and routines"
            }
            Self::PlansAhead => {
                "Prepares and anticipates — at work: sets up contingencies for transitions; in everyday life: plans well ahead of a big change"
            }
            Self::AdaptsQuickly => {
                "Adjusts on the fly — at work: pivots smoothly when priorities shift; in everyday life: rolls with unexpected changes"
            }
            Self::ResistsChange => {
                "Pushes back initially — at work: questions new processes before adopting; in everyday life: is wary of changes to routines"
            }
            Self::NeedsReassurance => {
                "Requires support — at work: asks for repeated confirmation during transitions; in everyday life: seeks reassurance through change"
            }
            Self::BecomesDisoriented => {
                "Can't keep up — at work: lags when processes shift fast; in everyday life: feels lost when routines change abruptly"
            }
            Self::Sabotages => {
                "Actively undermines — at work: quietly blocks the new direction; in everyday life: spoils plans they didn't want"
            }
            // Feedback
            Self::SeeksFeedback => {
                "Proactively asks — at work: requests regular reviews of their work; in everyday life: asks friends how they come across"
            }
            Self::AsksForDetails => {
                "Digs deeper, seeks specifics — at work: asks for concrete examples in reviews; in everyday life: probes to understand exactly what went wrong"
            }
            Self::Reflects => {
                "Takes time to process — at work: sleeps on feedback before responding; in everyday life: mulls over criticism before reacting"
            }
            Self::AcceptsResignedly => {
                "Reluctant acceptance — at work: takes feedback without agreement or pushback; in everyday life: shrugs off criticism with resignation"
            }
            Self::RejectsFeedback => {
                "Dismisses — at work: pushes back hard on any negative review; in everyday life: refuses to hear criticism from others"
            }
            Self::IgnoresCompletely => {
                "Disregards entirely — at work: tunes out the feedback session; in everyday life: carries on regardless of others' input"
            }
            // Injustice
            Self::SeeksRestoration => {
                "Repairs and reconciles — at work: pushes to mend the unfair situation and relations; in everyday life: seeks to restore fairness and peace"
            }
            Self::ProtestsConstructively => {
                "Raises concerns productively — at work: formalizes unfairness through the right channels; in everyday life: speaks up with reason and composure"
            }
            Self::ProtestsFirmly => {
                "Advocates clearly — at work: challenges the unfair decision head-on; in everyday life: defends the wronged person loudly"
            }
            Self::SeeksClarity => {
                "Investigates the facts — at work: gathers evidence before judging; in everyday life: verifies both sides before taking a stance"
            }
            Self::WithdrawsFromInjustice => {
                "Disengages — at work: distances themselves from the unfair environment; in everyday life: walks away from unjust situations"
            }
            Self::ExploitsOpportunistically => {
                "Takes advantage — at work: profits from a loophole or unfair situation; in everyday life: benefits from others' bad luck"
            }
            Self::BecomesBitter => {
                "Resentful, cynical — at work: harbors resentment over past unfairness; in everyday life: grows cynical about people and systems"
            }
        }
    }

    fn desc_fr(self) -> &'static str {
        match self {
            // Stress
            Self::RemainsCalm => {
                "Garde son sang-froid sous pression — au travail : continue de travailler calmement alors qu'une échéance glisse ; dans la vie : reste stable quand les plans s'effondrent"
            }
            Self::SeeksSupport => {
                "Demande de l'aide quand il est dépassé — au travail : sollicite ses collègues sur une charge lourde ; dans la vie : s'appuie sur famille et amis dans les moments difficiles"
            }
            Self::StaysFocused => {
                "Canalise le stress en productivité — au travail : traverse la période chargée avec une liste claire ; dans la vie : se raccroche à une tâche ou un projet"
            }
            Self::BecomesQuiet => {
                "Se ferme et se retire — au travail : se tait en réunion quand la tension monte ; dans la vie : se renferme au lieu d'en parler"
            }
            Self::BecomesIrritable => {
                "S'énerve facilement — au travail : s'agace des petites interruptions en période de rush ; dans la vie : devient grognon à la maison sous pression"
            }
            Self::Overwhelmed => {
                "Se ferme complètement — au travail : fige devant un gros backlog ; dans la vie : reste bloqué par l'accumulation de tâches et de soucis"
            }
            Self::Panics => {
                "Perd le contrôle — au travail : surréagit à un échec inattendu ; dans la vie : dramatise quand quelque chose tourne mal"
            }
            // Conflict
            Self::FacilitatesResolution => {
                "Médie et trouve un terrain d'entente — au travail : s'interpose pour apaiser les clashs d'équipe ; dans la vie : joue les médiateurs entre proches en désaccord"
            }
            Self::CommunicatesOpenly => {
                "Exprime ses sentiments avec constructivité — au travail : dit ce qui le dérange calmement et directement ; dans la vie : discute des problèmes au lieu de garder rancune"
            }
            Self::SeeksCompromise => {
                "Cherche un terrain d'entente — au travail : partage la différence sur un plan contesté ; dans la vie : propose des concessions pour que chacun y gagne"
            }
            Self::StaysSilent => {
                "Évite de prendre parti — au travail : reste en retrait du débat sans se positionner ; dans la vie : se tait lors des disputes familiales"
            }
            Self::BecomesPassiveAggressive => {
                "Utilise des piques au lieu de la franchise — au travail : lance des remarques perfides plutôt que de soulever le problème ; dans la vie : boude et fait des réflexions amères"
            }
            Self::BecomesDefensive => {
                "Bloque, argumente, se justifie — au travail : répond aux critiques par des excuses ; dans la vie : transforme les désaccords en disputes à gagner"
            }
            Self::Escalates => {
                "Attaque personnellement — au travail : rend le désaccord personnel et virulent ; dans la vie : ressasse de vieilles rancunes pendant une dispute"
            }
            // Success
            Self::CelebratesWithOthers => {
                "Partage la joie et soude le groupe — au travail : organise une célébration d'équipe pour la victoire ; dans la vie : fête une étape importante avec les proches"
            }
            Self::SharesCredit => {
                "Félicite les contributions des autres — au travail : cite tout le monde dans le bilan du projet ; dans la vie : rappelle aux gens leur part du mérite"
            }
            Self::SetsNewGoals => {
                "Relève la barre — au travail : vise immédiatement l'objectif suivant ; dans la vie : se lance un nouveau défi personnel"
            }
            Self::EnjoysQuietly => {
                "Satisfaction intérieure sans démonstration — au travail : laisse les résultats parler, sans fanfare ; dans la vie : savoure ses réussites en privé"
            }
            Self::BecomesComplacent => {
                "Se repose sur ses lauriers — au travail : navigue à vue après avoir atteint un gros objectif ; dans la vie : arrête de progresser une fois le but atteint"
            }
            Self::BecomesOverconfident => {
                "Arrogant et vantard — au travail : prend plus que ce qu'il peut livrer après une victoire ; dans la vie : donne des leçons sur son succès"
            }
            Self::DismissesOthers => {
                "Dévalorise les contributions des autres — au travail : s'attribue le succès en minimisant l'apport de l'équipe ; dans la vie : minimise les réussites des autres"
            }
            // Uncertainty
            Self::EmbracesAmbiguity => {
                "Prospère dans l'inconnu — au travail : se lance dans des projets au périmètre flou ; dans la vie : aime les plans ouverts et les surprises"
            }
            Self::AsksQuestions => {
                "Cherche à comprendre — au travail : clarifie objectifs et attentes ; dans la vie : rassemble les détails avant de décider"
            }
            Self::SeeksData => {
                "Rassemble des faits — au travail : sort les indicateurs avant d'agir ; dans la vie : fait des recherches avant un achat"
            }
            Self::WaitsForClarity => {
                "Temporise — au travail : diffère les décisions tant que la direction n'est pas claire ; dans la vie : reporte les choix jusqu'à ce que ça s'éclaircisse"
            }
            Self::OverPlans => {
                "Tente de contrôler l'incertain — au travail : surplanifie et surdocumente ; dans la vie : planifie chaque étape d'un voyage ou d'un événement"
            }
            Self::BecomesParalyzed => {
                "Incapable d'agir — au travail : cale sur une tâche ambiguë ; dans la vie : n'arrive pas à choisir face à des options ouvertes"
            }
            Self::DeflectsResponsibility => {
                "Blâme l'ambiguïté — au travail : attribue les retards à des consignes floues ; dans la vie : blâme les circonstances plutôt que lui-même"
            }
            // Recognition
            Self::AppreciatesQuietly => {
                "Valorise sans chercher la lumière — au travail : se contente d'un merci discret ; dans la vie : apprécie les vrais éloges sans fanfare"
            }
            Self::AppreciatesPraise => {
                "Accepte les compliments avec grâce — au travail : sourit et remercie pour le retour ; dans la vie : reçoit les compliments chaleureusement"
            }
            Self::SharesAchievement => {
                "Informe des progrès — au travail : tient les parties prenantes au courant des victoires ; dans la vie : partage ses réussites avec ses proches"
            }
            Self::SeeksMore => {
                "Besoin d'approbation modéré — au travail : vérifie que son travail est validé ; dans la vie : cherche la réassurance de famille et amis"
            }
            Self::BecomesJealous => {
                "Ressent la reconnaissance des autres — au travail : en veut à un collègue loué ; dans la vie : envie les succès de ses amis"
            }
            Self::DemandsAttention => {
                "Veut être le centre — au travail : ramène les conversations vers lui ; dans la vie : a besoin d'être remarqué dans un groupe"
            }
            Self::UnderminesOthers => {
                "Rabaisse les autres pour avancer — au travail : minimise les réussites de l'équipe ; dans la vie : rabaisse socialement les autres"
            }
            // Threatened
            Self::SeeksUnderstanding => {
                "Tente de cerner la menace — au travail : recherche la source d'une attaque ; dans la vie : analyse les motivations avant de réagir"
            }
            Self::SeeksAllies => {
                "Tisse des coalitions — au travail : s'appuie sur d'autres équipes ; dans la vie : mobilise ses amis quand il se sent attaqué"
            }
            Self::StandsGround => {
                "Affirme sa position calmement — au travail : tient sa position sur un choix sans envenimer ; dans la vie : reste ferme mais poli dans les débats"
            }
            Self::BecomesCautious => {
                "Recule pour évaluer — au travail : prend du recul pour analyser ; dans la vie : garde ses distances en pesant la menace"
            }
            Self::DeflectsBlame => {
                "Redirige la responsabilité — au travail : pointe le process ou les autres quand il est accusé ; dans la vie : trouve quelqu'un d'autre à qui imputer la faute"
            }
            Self::Counterattacks => {
                "Riposte — au travail : contre-attaque face aux critiques ; dans la vie : réplique vivement dans une dispute"
            }
            Self::BecomesParanoid => {
                "Voit des menaces partout — au travail : lit des attaques dans des messages neutres ; dans la vie : soupçonne des intentions cachées chez les autres"
            }
            // Change
            Self::EmbracesChange => {
                "S'adapte rapidement — au travail : adopte outils et process nouveaux sans résistance ; dans la vie : accueille volontiers les changements de plans"
            }
            Self::PlansAhead => {
                "Se prépare, anticipe — au travail : prépare des plans B pour les transitions ; dans la vie : planifie largement à l'avance un grand changement"
            }
            Self::AdaptsQuickly => {
                "S'ajuste à la volée — au travail : pivote facilement quand les priorités changent ; dans la vie : suit les imprévus avec souplesse"
            }
            Self::ResistsChange => {
                "Rechigne initialement — au travail : questionne tout nouveau process avant de l'adopter ; dans la vie : se méfie des changements de routine"
            }
            Self::NeedsReassurance => {
                "Demande du soutien — au travail : réclame des confirmations répétées pendant les transitions ; dans la vie : cherche à être rassuré dans le changement"
            }
            Self::BecomesDisoriented => {
                "N'arrive pas à suivre — au travail : décroche quand les process changent vite ; dans la vie : se sent perdu quand les routines chamboulent"
            }
            Self::Sabotages => {
                "Compromet activement — au travail : bloque en douce la nouvelle direction ; dans la vie : fait échouer les plans qu'il ne voulait pas"
            }
            // Feedback
            Self::SeeksFeedback => {
                "Le sollicite proactivement — au travail : demande des bilans réguliers de son travail ; dans la vie : demande aux amis l'image qu'il renvoie"
            }
            Self::AsksForDetails => {
                "Creuse, cherche des précisions — au travail : réclame des exemples concrets dans les bilans ; dans la vie : cherche à comprendre précisément ce qui a échoué"
            }
            Self::Reflects => {
                "Prend le temps d'analyser — au travail : dort sur le feedback avant de répondre ; dans la vie : réfléchit avant de réagir à une critique"
            }
            Self::AcceptsResignedly => {
                "Acceptation à contrecœur — au travail : encaisse le feedback sans adhérer ni protester ; dans la vie : hausse les épaules face aux critiques"
            }
            Self::RejectsFeedback => {
                "Se braque, se ferme — au travail : rejette vivement toute évaluation négative ; dans la vie : refuse d'écouter les critiques"
            }
            Self::IgnoresCompletely => {
                "Fait la sourde oreille — au travail : n'écoute pas la séance de feedback ; dans la vie : continue sans tenir compte des avis"
            }
            // Injustice
            Self::SeeksRestoration => {
                "Répare et réconcilie — au travail : s'emploie à réparer la situation et les relations ; dans la vie : cherche à rétablir justice et paix"
            }
            Self::ProtestsConstructively => {
                "Exprime ses préoccupations avec constructivité — au travail : remonte l'injustice par les bons canaux ; dans la vie : s'exprime avec calme et raison"
            }
            Self::ProtestsFirmly => {
                "Défend ce qui est juste — au travail : conteste frontalement la décision injuste ; dans la vie : défend haut et fort la personne lésée"
            }
            Self::SeeksClarity => {
                "Enquête sur les faits — au travail : rassemble les preuves avant de juger ; dans la vie : vérifie les deux versions avant de prendre parti"
            }
            Self::WithdrawsFromInjustice => {
                "Se désengage — au travail : prend ses distances avec un environnement injuste ; dans la vie : s'écarte des situations injustes"
            }
            Self::ExploitsOpportunistically => {
                "Profite de la situation — au travail : tire parti d'une faille ou d'une injustice ; dans la vie : bénéficie du malheur des autres"
            }
            Self::BecomesBitter => {
                "Amertume, cynisme — au travail : nourrit de la rancœur sur les injustices passées ; dans la vie : devient cynique envers les gens et les systèmes"
            }
        }
    }
}
