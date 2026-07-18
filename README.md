# 🧩 People Modeler

> Modéliser les gens comme des systèmes : motivations, biais, comportements.

[![Build](https://github.com/yourusername/people-modeler/actions/workflows/build.yml/badge.svg)](https://github.com/yourusername/people-modeler/actions)
[![Web](https://img.shields.io/badge/Web-GitHub%20Pages-blue)](https://yourusername.github.io/people-modeler)

---

## ⚠️ Note éthique

> Ce projet est un outil de **compréhension**, pas de manipulation.  
> Utilisez-le pour améliorer vos relations, votre leadership, votre empathie.  
> La connaissance des systèmes humains est une responsabilité.

---

## 📁 Structure du projet

```
PeopleModeler/
├── core/                       # Moteur Rust (WASM + JNI)
│   ├── src/
│   │   ├── lib.rs              # Point d'entrée, exports WASM
│   │   ├── models.rs           # Types: Person, Motivation, Bias, BehaviorPattern, StyleType...
│   │   ├── synergy.rs          # Score de synergie (OCEAN, Rep, Mot, Pat, Bias, Style)
│   │   ├── insights.rs         # Génération d'insights comportementaux
│   │   ├── predictions.rs      # Logique de prédictions
│   │   ├── ocean.rs            # Interprétation OCEAN
│   │   ├── i18n.rs             # Internationalisation (EN/FR)
│   │   ├── validation.rs       # Avertissements de cohérence
│   │   ├── wasm.rs             # Exports WebAssembly (JS)
│   │   └── android.rs          # Exports JNI (Kotlin, legacy)
│   └── Cargo.toml
│
├── app/                        # App Dioxus (Web WASM)
│   ├── src/
│   │   ├── main.rs             # Point d'entrée, routage
│   │   ├── i18n.rs             # Internationalisation app (EN/FR)
│   │   ├── db/mod.rs           # Stockage SQLite
│   │   ├── pages/
│   │   │   ├── person_list.rs      # Liste des personnes
│   │   │   ├── person_detail.rs    # Fiche personne (onglets)
│   │   │   ├── person_edit.rs      # Édition personne
│   │   │   ├── compare.rs          # Comparaison 2 profils
│   │   │   ├── insights.rs         # Insights globaux
│   │   │   ├── predictions.rs      # Prédictions
│   │   │   └── sync.rs             # Sync Google Drive
│   │   ├── drive.rs            # Backup Google Drive
│   │   ├── templates.rs        # Archétypes de personnes
│   │   └── theme.rs            # Thème
│   ├── assets/styles.css       # Styles
│   └── Cargo.toml
│
├── tests/                      # Tests E2E Playwright
├── scripts/
│   └── spa_server.py           # Serveur dev SPA
├── public/                     # Assets statiques (sw.js, manifest.json)
└── .github/workflows/build.yml # Pipeline CI/CD
```

---

## 🚀 Pipeline GitHub Actions

La pipeline `.github/workflows/build.yml` fait :

### 1. `rust-core` — Moteur Rust
- Compilation Rust stable (check + clippy + test)
- Bloque `deploy-web` si échec
- Exports WASM via `wasm-pack`

### 2. `web-build` — App Dioxus
- Build WASM avec `dx build --release`
- Upload artifact `web-static`
- Déploiement GitHub Pages (nécessite `rust-core` OK)

### 3. `release` — Sur tag `v*`
- Crée une GitHub Release
- Notes de release auto-générées

---

## 🌐 Déploiement Web (GitHub Pages)

1. Allez dans **Settings → Pages**
2. Source : **Deploy from a branch**
3. Branche : `gh-pages`
4. La pipeline déploie automatiquement à chaque push sur `main`

URL : `https://yourusername.github.io/people-modeler/`

---

## 📱 Android (legacy)

L'ancienne app Android (Kotlin + Room + MVVM) n'est plus maintenue.
Les fonctionnalités ont été migrées vers l'app Dioxus Web/WASM.

---

## 🌐 Fonctionnalités App (Dioxus Web/WASM)

### Pages
1. **Liste** — Recherche, cartes avec chips OCEAN/motivations/biais
2. **Fiche détail** — Profil complet avec onglets : Motivations, Biais, OCEAN, Réputation, Prédictions, Insights, Journal, Relations, Styles personnels
3. **Édition** — Formulaire complet : OCEAN, motivations, biais, réputation (13 dimensions), patterns comportementaux (9 déclencheurs, 28 réponses), styles personnels (6 catégories, 26 variantes)
4. **Comparaison** — Score de synergie avec décomposition par catégorie
5. **Prédictions** — Feedbacks et précision
6. **Insights** — Analyse globale et statistiques
7. **Sync** — Sauvegarde Google Drive

---

## 🛠️ Développement local

### App (Dioxus)
```bash
# Lancer en dev (hot-reload)
dx serve

# Build release WASM
dx build --release

# Tests
cargo test
cargo clippy
```

### Serveur SPA (pour tests E2E)
```bash
python3 scripts/spa_server.py
```

---

## 📦 Créer une release

```bash
git tag v1.0.0
git push origin v1.0.0
# → La pipeline crée automatiquement une GitHub Release avec l'APK
```

---

## 🔬 Modèle de données

```
Person
├── id, name, role, context, avatarEmoji
├── motivations[]        # type (enum 10), intensity (1-10), notes
├── biases[]             # type (enum 11), intensity (1-10), evidence
├── behavioralPatterns[] # trigger (enum 9), predictedBehavior (enum 28), intensity
├── styles[]             # type (enum 26), intensity (1-10), notes
├── ocean                # O, C, E, A, N (Option<u8>, 1-10)
├── rep_scores           # 13 dimensions Option<u8> (0-10), bipolar:
│                        #   Hardworker↔Lazy, Authoritative↔Submissive
│                        #   Honest↔Deceitful, Reliable↔Flaky
│                        #   Humble↔Arrogant, Calm↔Reactive
│                        #   Diplomatic↔Blunt, Generous↔Selfish
│                        #   Fair↔Favoritism, Trusting↔Suspicious
│                        #   Assertive↔Passive, Empathetic↔Detached
│                        #   Adaptable↔Rigid
│                        #   ≥5 = pole A, <5 = pole B, None = non-renseigné
├── tags[]
├── predictions[]        # context, predicted, actual, accuracy, resolvedAt
├── relationships[]      # sourceId, targetId, type, strength
├── log[]                # InteractionEntry: type, description, timestamp
└── confidence           # 1-10, fiabilité perçue du profil
```

### Score de synergie (comparaison 2 personnes)

Pondérations de base quand toutes les catégories ont des données :

```
OCEAN×17% + Réputation×26% + Motivation×19% + Patterns×14% + Biais×13% + Styles×11%
```

Si une catégorie n'a pas de données (ex: aucun pattern partagé), son poids est
redistribué proportionnellement aux autres catégories actives.

#### 1. OCEAN (17%)

Distance continue par trait (1-10) + bonus de complémentarité :

```
sim(x, y) = 1.0 - |x - y| / 10          → [0.0, 1.0] par trait

OC = (sim(O_A, O_B) + sim(C_A, C_B)) / 2
EA = (sim(E_A, E_B) + sim(A_A, A_B)) / 2
N  =  sim(N_A, N_B)

oc_bonus = 0.15 si (O_A≥7 ∧ C_B≥7) ∨ (O_B≥7 ∧ C_A≥7), sinon 0
ea_bonus = 0.15 si (E_A≥7 ∧ A_B≥7) ∨ (E_B≥7 ∧ A_A≥7), sinon 0

OCEAN_brut = (min(OC + oc_bonus, 1) + min(EA + ea_bonus, 1) + N) / 3
```

- `sim` remplace les anciens paliers (0.15/0.7/1.0) par une valeur continue
- `bonus` récompense la complémentarité O-C et E-A sans remplacer la distance

**Pénalités danger OCEAN** — combinaisons de traits connues pour générer des
frictions, **au sein d'une même personne** et **entre les deux** :

```
pénalité OCEAN = Σ(ci-dessous)

Intra-personne (chaque personne) :
  N ≥ 7 et A ≤ 4   → volatilité émotionnelle            +0.10
  N ≥ 7 et C ≤ 4   → impulsivité                        +0.05
  N ≥ 7 et O ≤ 4   → rigidité anxieuse                  +0.05

Inter-personnes (les deux) :
  Tous deux N ≥ 7   → contagion émotionnelle              +0.10
  Tous deux A ≤ 4   → antagonisme mutuel                   +0.15
  Tous deux C ≤ 4   → manque de fiabilité réciproque       +0.10
  Tous deux O ≤ 4   → rigidité partagée                    +0.05
```

OCEAN final après modulation et pénalité :

```
OCEAN_penalisé = max(OCEAN_brut - pénalité_OCEAN, 0)
OCEAN_final = min(OCEAN_penalisé × (1 + modulations_biais_OCEAN), 1)
```

#### 2. Réputation (26%)

Pour chaque dimension (13 bipolaires) où A et B ont une valeur :

```
similarité = 1.0 - |score_A - score_B| / 10   → [0.0, 1.0]
Rep_brut = Σ(similarité_dim × poids_dim) / Σ(poids_dim)
```

Les dimensions ont des poids différents selon leur impact relationnel :

| Dimension | Poids |
|---|---|---|---|
| Honnête ↔ Trompeur | 0.15 |
| Fiable ↔ Inconstant | 0.12 |
| Autoritaire ↔ Soumis | 0.12 |
| Humble ↔ Arrogant | 0.12 |
| Travailleur ↔ Paresseux | 0.07 |
| Calme ↔ Réactif | 0.07 |
| Diplomate ↔ Direct | 0.07 |
| Équitable ↔ Partial | 0.07 |
| Confiant ↔ Méfiant | 0.05 |
| Affirmé ↔ Passif | 0.05 |
| Empathique ↔ Détaché | 0.05 |
| Généreux ↔ Égoïste | 0.04 |
| Flexible ↔ Rigide | 0.04 |

> Somme = 1.02, normalisée à l'exécution par `total_active_w`.

- Si **aucune** dimension commune : catégorie inactive, poids redistribué

**Pénalités danger Réputation** — mêmes pôles extrêmes chez les deux :

Les scores de réputation sont bipolaires : `0 = pôle négatif`, `10 = pôle positif`.
Les seuils ci-dessous utilisent la valeur du score directement.

```
pénalité_Rep = Σ(ci-dessous)

Tous deux Autoritaire ≥ 8  → lutte de pouvoir            +0.10
Tous deux Direct ≤ 3       → brutalité, pas de diplomatie +0.10
Tous deux Réactif ≤ 3      → escalade mutuelle            +0.10
Tous deux Arrogant ≤ 3     → ni l'un ni l'autre ne cède   +0.10
Tous deux Paresseux ≤ 3    → passivité mutuelle           +0.05
Tous deux Trompeur ≤ 3     → effondrement de la confiance +0.10
Tous deux Inconstant ≤ 3   → manque de fiabilité mutuel   +0.08
Tous deux Méfiant ≤ 3      → suspicion mutuelle           +0.08
Tous deux Détaché ≤ 3      → froideur mutuelle            +0.08
Tous deux Favoritisme ≤ 3  → copinage                     +0.08
Tous deux Égoïste ≤ 3      → accaparement mutuel          +0.05
Tous deux Passif ≤ 3       → paralysie décisionnelle      +0.05
Tous deux Rigide ≤ 3       → blocage mutuel               +0.05
```

Réputation finale après modulation et pénalité :

```
Rep_penalisé = max(Rep_brut - pénalité_Rep, 0)
Rep_final = min(Rep_penalisé × (1 + modulations_biais_Rep), 1)
```

#### 3. Motivation (19%)

Paires pondérées par `intensity_A × intensity_B / 100`. Les paires neutres
(synergy = 0.0) sont ignorées pour éviter le biais de dilution. La moyenne
résultante est re-mappée de `[−0.3, +0.3]` vers `[0, 1]` :

```
avg = moyenne_pondérée(mot_synergy(type_A, type_B), poids, skip_neutral)
Mot_brut = (avg + 0.3) / 0.6   → clamp [0, 1]
```

Table `motivation_synergy(tA, tB)` :

🤝 Même type : selon la motivation — Power × Power = **−0.2** (compétition),
Recognition × Recognition = **−0.1** (lutte d'ego), Autonomy × Autonomy = **0.0**
(indépendance neutre), Security × Security = **0.0** (statu quo). Les autres
(Achievement, Affiliation, Learning, Helping, Creativity, Fairness) restent à **+0.2** (alignement).

🔄 Complémentarité : paires asymétriques productives — Power × Helping = **+0.1**
(l'un dirige, l'autre soutient), Achievement × Affiliation = **+0.1** (résultats + harmonie).

| tA \ tB | Power | Achieve | Affil | Security | Autonomy | Recogn | Learn | Helping | Creativ | Fairness |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **Power** | **−0.2** | +0.3 | −0.2 | −0.1 | +0.2 | +0.2 | 0.0 | +0.1 | −0.1 | −0.2 |
| **Achievement** | +0.3 | +0.2 | +0.1 | −0.2 | +0.2 | +0.3 | +0.3 | +0.2 | +0.2 | +0.2 |
| **Affiliation** | −0.2 | +0.1 | +0.2 | +0.2 | −0.1 | −0.1 | +0.2 | +0.3 | +0.2 | +0.2 |
| **Security** | −0.1 | −0.2 | +0.2 | 0.0 | −0.3 | 0.0 | +0.2 | +0.2 | −0.2 | +0.2 |
| **Autonomy** | +0.2 | +0.2 | −0.1 | −0.3 | 0.0 | 0.0 | +0.2 | 0.0 | +0.2 | +0.2 |
| **Recognition** | +0.2 | +0.3 | −0.1 | 0.0 | 0.0 | **−0.1** | +0.3 | 0.0 | +0.3 | −0.1 |
| **Learning** | 0.0 | +0.3 | +0.2 | +0.2 | +0.2 | +0.3 | +0.2 | +0.2 | +0.3 | +0.2 |
| **Helping** | +0.1 | +0.2 | +0.3 | +0.2 | 0.0 | 0.0 | +0.2 | +0.2 | −0.1 | +0.3 |
| **Creativity** | −0.1 | +0.2 | +0.2 | −0.2 | +0.2 | +0.3 | +0.3 | −0.1 | +0.2 | +0.2 |
| **Fairness** | −0.2 | +0.2 | +0.2 | +0.2 | +0.2 | −0.1 | +0.2 | +0.3 | +0.2 | +0.2 |

##### Ajustement vertus (profil individuel)

Avant le calcul de synergie, chaque profil individuel (`compute_person_profile`)
applique un ajustement moral à son score de motivation selon les vertus/vices :

| Motivation | ≥ 7 (vertu) | ≤ 3 ou absent (vice) |
|---|---|---|
| Équité (Fairness) | +0.08 | −0.08 |
| Entraide (Helping) | +0.06 | −0.06 |
| Apprentissage (Learning) | +0.04 | 0 |
| Créativité (Creativity) | +0.04 | 0 |
| Pouvoir (Power) | −0.08 (vice, seuil 7) | 0 |
| Sécurité (Security) | −0.05 (vice, seuil 7) | 0 |
| Reconnaissance (Recognition) | −0.03 (vice, seuil 9) | 0 |
| Autres (Achievement, Affiliation, Autonomy) | 0 | 0 |

**Pénalité de rareté** : si la personne a peu de motivations, son score est réduit :

| Nb motivations | Pénalité |
|---|---|
| 0 | −0.09 |
| 1 | −0.06 |
| 2 | −0.03 |
| 3+ | 0.0 |

Ces ajustements sont appliqués au score de motivation **dans le profil** avant
tout calcul de synergie cross-personne.

#### 4. Patterns (14%)

Paires pondérées par `conf_A × conf_B / 100`. Les paires neutres
(synergy = 0.0) sont ignorées (même logique que motivations).

```
avg = moyenne_pondérée(trigger_synergy(tA, tB), poids, skip_neutral)
Patterns_brut = (avg + 0.3) / 0.6   → clamp [0, 1]
```

Table `trigger_synergy(tA, tB)` :

| tA \ tB | Change | Feedback | Success | Conflict | Stress | Uncertainty | Recognition | Threatened | Injustice |
|---|---|---|---|---|---|---|---|---|---|---|
| **Change** | +0.3 | +0.3 | 0 | 0 | -0.2 | 0 | 0 | 0 | 0 |
| **Feedback** | +0.3 | +0.3 | 0 | 0 | 0 | 0 | +0.2 | 0 | 0 |
| **Success** | 0 | 0 | +0.3 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Conflict** | 0 | 0 | 0 | -0.3 | -0.3 | -0.2 | 0 | 0 | 0 |
| **Stress** | -0.2 | 0 | 0 | -0.3 | -0.2 | 0 | 0 | 0 | 0 |
| **Uncertainty** | 0 | 0 | 0 | -0.2 | 0 | 0 | 0 | 0 | 0 |
| **Recognition** | 0 | +0.2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Threatened** | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Injustice** | 0 | 0 | 0 | −0.1 | −0.1 | −0.1 | 0 | 0 | −0.2 |

**Pénalité danger Patterns** — lorsque les deux personnes n'ont **que** des
déclencheurs négatifs (Conflict, Stress, Threatened, Injustice), aucun pattern positif
n'équilibre la relation :

```
pénalité_Patterns = 0.05 si les deux n'ont que des déclencheurs négatifs
                     0.00 sinon
```

Patterns final après modulation et pénalité :

```
Patterns_penalisé = max(Patterns_brut - pénalité_Patterns, 0)
Patterns_final = min(Patterns_penalisé × (1 + modulations_biais_Patterns), 1)
```

#### 5. Biais (13%)

Les biais ne sont **pas** scorés directement. Chaque type de biais module une
autre catégorie du score quand il est partagé par les deux personnes :

```
biais_modificateur(type) → (cible, coefficient)

Anchoring     → OCEAN       +0.10  (ancrage des premières impressions)
Confirmation  → Réputation  +0.10  (recherche de confirmation)
Availability  → Patterns    +0.10  (poids des événements récents)
SunkCost      → Motivation  +0.10  (investissement passé)
DunningKruger → OCEAN       -0.10  (auto-évaluation distordue)
LossAversion  → Patterns    -0.10  (poids excessif du négatif)
SocialProof   → Réputation  +0.08  (influence du groupe)
Authority     → Motivation  +0.08  (déférence à l'autorité)
Recency       → Patterns    +0.08  (emphase sur le récent)
InGroup       → OCEAN       +0.08  (favoritisme endogroupe)
Favoritism    → Réputation  -0.08  (traitement préférentiel)
```

Pour chaque paire de biais **de même type** (partagés par A et B) :

```
modulation = coefficient × (intensity_A × intensity_B / 100)
score_cat_modulé = score_cat_brut × (1.0 + Σ_modulations)   → clamp [0, 1]
```

**Score de biais** pour le profil individuel : combiné d'une base
comptable, d'un ajustement d'intensité et d'un bonus de rareté.

Le **décompte des biais présents** inclut :
- les types **non définis** (absents du vecteur de la personne) — par défaut,
  un biais non renseigné est considéré comme présent
- les types définis avec une intensité **≥ 4** (modéré ou fort)

Les types avec intensité **0** (explicitement absent) ou **≤ 3** (faible) ne
sont pas comptés comme présents.

1. **Base** : `1.0 - nb_biais_présents / 11`
2. **Ajustement d'intensité** (`bias_adjustment`) :

   | Statut | Ajustement |
   |---|---|
   | Non défini (absent du vecteur) | 0 (mais compte dans la base) |
   | Intensité **0** (explicitement absent) | +0.02 |
   | ≤ 3 (faible) | +0.01 |
   | 4‑6 (modéré) | 0 |
   | ≥ 7 (fort) | −0.03 |

3. **Bonus de rareté** (`bias_count_bonus`) — basé sur `nb_biais_présents` :

   | Nb biais présents | Bonus |
   |---|---|
   | 0 | +0.09 |
   | 1 | +0.06 |
   | 2 | +0.03 |
   | 3+ | 0.0 |

```
biais_score_profil = (base + ajustement + bonus).clamp(0, 1)
```

Ces ajustements sont appliqués au score de biais **dans le profil** avant tout
calcul de comparaison cross-personne.

**Score de biais cross-personne** (comparaison) : fraction des types de biais
partagés :

```
biais_score = shared_types / max(len(A_types), len(B_types))
             → 0.5 si aucun biais renseigné
qualité_Biais(P) = 1 - nb_biais / 11  (11 types de biais)
```

- Biais partagé = les deux personnes ont le même biais → modulation appliquée
- Biais non partagé = pas d'effet (ni bonus, ni malus)
- Plus les biais partagés sont intenses, plus la modulation est forte
- Remplace l'ancien système `bias_pair_synergy` (même=-0.2, différent=+0.2)

#### 6. Styles personnels (11%)

Les styles personnels mesurent la compatibilité des modes de fonctionnement
préférentiels dans 6 catégories :

| Catégorie | Variantes |
|---|---|
| 💬 Communication | Direct, Diplomatic, Analytical, Expressive, Reserved |
| 🤝 Résolution conflit | Collaborative, Competitive, Avoidant, Accommodating, Compromising |
| 🧠 Prise de décision | Rational, Intuitive, Consultative, Decisive |
| 👥 Leadership | Autocratic, Democratic, Transformational, Transactional, Bureaucratic, LaissezFaire, Servant, Coach |
| ⏰ Orientation temporelle | PastOriented, PresentOriented, FutureOriented |
| 📜 Cadre moral | RuleBased, OutcomeBased, VirtueBased, Relativist |

Pour chaque catégorie où les deux personnes ont un style renseigné :

```
sim_style(cat) = 1.0 si même variante
                 0.5 si variante différente
styles_brut = moyenne des sim_style sur les catégories partagées
              0.5 si aucune catégorie en commun
```

#### 7. Facteur historique (traque les angles morts)

Si les deux personnes ont ≥ 3 prédictions résolues, leur **précision moyenne**
(< 5/10) indique une auto-évaluation peu fiable :

```
pénalité_historique =
  0.05 si les deux ont avg < 5
  0.03 si l'une des deux a avg < 5
  0.00 sinon
```

#### Agrégation finale (poids dynamiques)

Le score de base (catégories compatibilité) et les scores asymétriques utilisent
les mêmes poids fixes redistribués dynamiquement :

```
poids_OCEAN   = 0.17
poids_Rep     = 0.26
poids_Mot     = 0.19
poids_Patterns = 0.14
poids_Biais   = 0.13
poids_Styles  = 0.11
```

Quand une catégorie manque de données → elle est exclue et son poids est réparti
proportionnellement sur les catégories restantes. La Motivation (poids 0.19) est
**toujours active** — même sans données, le poids 0.19 est conservé (une pénalité
de rareté s'applique alors, voir §3).

#### Score asymétrique (bénéfice individuel)

Chaque personne reçoit son propre score (`a_score` / `b_score`) reflétant ce
qu'elle *bénéficie* de l'autre, calculé par catégorie :

- **OCEAN** : qualité du partenaire pondérée par similarité. Pour chaque trait,
  la contribution est `B_qualité × sim(A, B)` où
  `sim(A, B) = 1 - |A/10 - B/10|`. Asymétrique car `B × sim ≠ A × sim`
  quand les niveaux de traits diffèrent. Résultat : moyenne des 5 traits.

- **Réputation** : la qualité brute de l'autre
  (`qualité_base_Rep(P) = moyenne pondérée des scores / 10`).

- **Biais** : l'absence de biais chez l'autre
  (`qualité_base_Biais(P) = 1 - nb_biais / 10`).

- **Motivation / Patterns / Styles** : synergie mutuelle (identique pour les deux).

```
poids actif = Σ(poids_cat) pour chaque catégorie active
a_raw = score_OCEAN_a × 0.17 + qual_Rep_B × 0.26 + synergie_Mot × 0.19
      + synergie_Patterns × 0.14 + qual_Biais_B × 0.13 + synergie_Styles × 0.11
b_raw = score_OCEAN_b × 0.17 + qual_Rep_A × 0.26 + synergie_Mot × 0.19
      + synergie_Patterns × 0.14 + qual_Biais_A × 0.13 + synergie_Styles × 0.11

a_score = round(a_raw / poids_actif × 100) → clamp [0, 100]
b_score = round(b_raw / poids_actif × 100) → clamp [0, 100]
```

Le **score total** est la moyenne des deux, réduite des pénalités danger :

```
total = round((a_score + b_score) / 2) - danger_pts
danger_pts = round(danger / poids_actif × 100)
```

`danger` est la somme pondérée des pénalités OCEAN, Réputation, Patterns,
historique :

```
danger = pénalité_OCEAN × 0.17 + pénalité_Rep × 0.26
       + pénalité_Patterns × 0.14 + pénalité_historique × 0.10
```

Le champ `danger` du `SynergyBreakdown` expose cette valeur pour transparence
(sans double soustraction).

L'interface affiche les trois scores : `{A}% – {total}% – {B}%` avec des
flèches directionnelles indiquant qui bénéficie le plus.

---

## 📄 Licence

MIT — Libre d'utilisation, modification et distribution.

> 🧩 Construit avec Kotlin, HTML/CSS/JS vanilla, et GitHub Actions.
