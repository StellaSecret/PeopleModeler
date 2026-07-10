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
people-modeler/
├── core/                       # Moteur Rust (WASM + JNI)
│   ├── src/
│   │   ├── lib.rs              # Point d'entrée, exports WASM/JNI
│   │   ├── models.rs           # Types partagés (Person, Prediction, BehaviorTrigger)
│   │   ├── synergy.rs          # Score de synergie (OCEAN, Rep, Mot, Pat, Bias)
│   │   ├── insights.rs         # Génération d'insights comportementaux
│   │   ├── ocean.rs            # Interprétation OCEAN
│   │   ├── wasm.rs             # Exports WebAssembly (JS)
│   │   └── android.rs          # Exports JNI (Kotlin)
│   └── Cargo.toml
│
├── android/                    # App Android (Kotlin + Room + MVVM)
│   ├── app/
│   │   ├── src/main/
│   │   │   ├── java/com/peoplemodeler/
│   │   │   │   ├── core/           # JNI bridge vers Rust
│   │   │   │   ├── data/
│   │   │   │   │   ├── models/      # Person, Motivation, Bias, BehaviorPattern
│   │   │   │   │   └── repository/  # Room DB, DAOs, Repository
│   │   │   │   ├── ui/
│   │   │   │   │   ├── screens/     # Fragments (List, Detail, Edit, Predictions, Insights)
│   │   │   │   │   └── components/  # RecyclerView Adapters
│   │   │   │   └── viewmodels/      # PersonViewModel
│   │   │   └── res/                 # Layouts, navigation, menus, colors
│   │   └── build.gradle
│   └── build.gradle
│
├── web/                        # Site web statique
│   ├── index.html              # Landing page
│   ├── person.html             # Fiche personne interactive
│   ├── compare.html            # Comparaison de deux profils
│   ├── css/
│   │   ├── main.css            # Design system + landing
│   │   ├── person.css          # Page fiche
│   │   └── compare.css         # Page comparaison
│   └── js/
│       ├── data.js             # Données, constantes, storage
│       ├── i18n.js             # Traductions FR/EN
│       ├── wasm-bridge.js      # Pont WASM → Rust core
│       ├── main.js             # Animations landing
│       └── person.js           # Logique interactive fiche
│
└── .github/
    ├── dependabot.yml          # Màj automatiques npm, gradle, cargo, actions
    └── workflows/
        └── build.yml           # Pipeline CI/CD complète
```

---

## 🚀 Pipeline GitHub Actions

La pipeline `.github/workflows/build.yml` fait :

### 1. `rust-core` — Moteur Rust
- Compilation Rust stable (check + clippy + test)
- Bloque `deploy-web` et `build-android` si échec
- Les exports WASM (`wasm-pack`) et JNI (`cargo ndk`) sont optionnels

### 2. `web-build` — Site web
- Validation HTML
- Upload artifact `web-static`
- Déploiement GitHub Pages (nécessite `rust-core` OK)

### 3. `android-build` — APK
- Build Debug APK
- Build Release APK (unsigned)
- Signature optionnelle avec secrets GitHub
- Upload artifacts APK (debug 7j, release 30j)
- Bloqué si `rust-core` échoue

### 4. `release` — Sur tag `v*`
- Télécharge les APK release
- Crée une GitHub Release avec les APKs joints
- Notes de release auto-générées

### ⚙️ Secrets nécessaires (optionnels)

| Secret | Description |
|--------|-------------|
| `SIGNING_KEY` | Keystore en base64 |
| `SIGNING_KEY_ALIAS` | Alias de la clé |
| `SIGNING_STORE_PASSWORD` | Mot de passe du keystore |
| `SIGNING_KEY_PASSWORD` | Mot de passe de la clé |

---

## 🌐 Déploiement Web (GitHub Pages)

1. Allez dans **Settings → Pages**
2. Source : **Deploy from a branch**
3. Branche : `gh-pages`
4. La pipeline déploie automatiquement à chaque push sur `main`

URL : `https://yourusername.github.io/people-modeler/`

---

## 📱 Fonctionnalités Android

### Architecture
- **Pattern** : MVVM + Repository
- **DB** : Room (SQLite locale, aucun cloud)
- **UI** : Fragments + Navigation Component + Material 3
- **State** : LiveData + Coroutines

### Écrans
1. **Liste** — Recherche, cartes avec chips motivation/biais, OCEAN mini-bars
2. **Fiche détail** — Profil complet avec onglets
3. **Édition** — Formulaire complet avec sliders OCEAN
4. **Prédictions** — Toutes les prédictions en attente
5. **Insights** — Statistiques globales

---

## 🌐 Fonctionnalités Web

### Pages
1. **index.html** — Landing page avec hero animé
2. **person.html** — Fiche interactive complète :
   - Onglets : Motivations / Biais / OCEAN / Prédictions / Insights
   - Sliders OCEAN interactifs avec interprétation live
   - Ajout/suppression de motivations et biais
   - Système de prédictions avec feedback loop
   - Analyse comportementale par contexte (6 triggers)
   - Persistance localStorage
3. **compare.html** — Comparaison de deux profils

---

## 🛠️ Développement local

### Web
```bash
# Ouvrir directement dans le navigateur
open web/index.html

# Ou avec un serveur local
npx serve web/
```

### Android
```bash
cd android
./gradlew assembleDebug
# APK dans app/build/outputs/apk/debug/
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
├── motivations[]        # type (enum), intensity (1-10), notes
├── biases[]             # type (enum), intensity (1-10), evidence
├── behavioralPatterns[] # trigger, predictedBehavior, confidence
├── ocean                # O, C, E, A, N (1-10 each)
├── rep_scores           # 8 dimensions Option<u8> (0-10), bipolar:
│                        #   Hardworker↔Lazy, Authoritative↔Submissive
│                        #   Honest↔Deceitful, Reliable↔Flaky
│                        #   Humble↔Arrogant, Calm↔Reactive
│                        #   Diplomatic↔Blunt, Generous↔Selfish
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
OCEAN×19% + Réputation×29% + Motivation×21% + Patterns×16% + Biais×15%
```

Si une catégorie n'a pas de données (ex: aucun pattern partagé), son poids est
redistribué proportionnellement aux autres catégories actives.

#### 1. OCEAN (19%)

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

#### 2. Réputation (29%)

Pour chaque dimension (8 bipolaires) où A et B ont une valeur :

```
similarité = 1.0 - |score_A - score_B| / 10   → [0.0, 1.0]
Rep_brut = moyenne des similarités sur les dimensions partagées
```

- Si **aucune** dimension commune : catégorie inactive, poids redistribué

#### 3. Motivation (21%)

Toutes les paires pondérées par `intensity_A × intensity_B / 100` :

```
Mot_brut = 0.5 + moyenne_pondérée(mot_synergy(type_A, type_B))   → clamp [0, 1]
```

Table `motivation_synergy(tA, tB)` :

| tA \ tB | Power | Achieve | Affil | Security | Autonomy | Recogn | Learn | Helping |
|---|---|---|---|---|---|---|---|---|
| **Power** | +0.2 | +0.3 | -0.2 | -0.1 | +0.2 | +0.2 | 0 | 0 |
| **Achievement** | +0.3 | +0.2 | 0 | -0.2 | +0.2 | +0.3 | +0.3 | 0 |
| **Affiliation** | -0.2 | 0 | +0.2 | +0.2 | -0.1 | -0.1 | 0 | +0.3 |
| **Security** | -0.1 | -0.2 | +0.2 | +0.2 | -0.3 | 0 | 0 | +0.2 |
| **Autonomy** | +0.2 | +0.2 | -0.1 | -0.3 | +0.2 | 0 | +0.2 | 0 |
| **Recognition** | +0.2 | +0.3 | -0.1 | 0 | 0 | +0.2 | 0 | 0 |
| **Learning** | 0 | +0.3 | 0 | 0 | +0.2 | 0 | +0.2 | +0.2 |
| **Helping** | 0 | 0 | +0.3 | +0.2 | 0 | 0 | +0.2 | +0.2 |

#### 4. Patterns (16%)

Toutes les paires pondérées par `conf_A × conf_B / 100` :

```
Patterns_brut = 0.5 + moyenne_pondérée(trigger_synergy(tA, tB))   → clamp [0, 1]
```

Table `trigger_synergy(tA, tB)` :

| tA \ tB | Change | Feedback | Success | Conflict | Stress | Uncertainty | Recognition | Threatened |
|---|---|---|---|---|---|---|---|---|
| **Change** | +0.3 | +0.3 | 0 | 0 | -0.2 | 0 | 0 | 0 |
| **Feedback** | +0.3 | +0.3 | 0 | 0 | 0 | 0 | +0.2 | 0 |
| **Success** | 0 | 0 | +0.3 | 0 | 0 | 0 | 0 | 0 |
| **Conflict** | 0 | 0 | 0 | -0.3 | -0.3 | -0.2 | 0 | 0 |
| **Stress** | -0.2 | 0 | 0 | -0.3 | -0.2 | 0 | 0 | 0 |
| **Uncertainty** | 0 | 0 | 0 | -0.2 | 0 | 0 | 0 | 0 |
| **Recognition** | 0 | +0.2 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Threatened** | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

#### 5. Biais (15%)

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
```

Pour chaque paire de biais **de même type** (partagés par A et B) :

```
modulation = coefficient × (intensity_A × intensity_B / 100)
score_cat_modulé = score_cat_brut × (1.0 + Σ_modulations)   → clamp [0, 1]
```

**Score de biais** pour l'affichage : fraction des types de biais partagés :

```
biais_score = shared_types / max(len(A_types), len(B_types))
             → 0.5 si aucun biais renseigné
```

- Biais partagé = les deux personnes ont le même biais → modulation appliquée
- Biais non partagé = pas d'effet (ni bonus, ni malus)
- Plus les biais partagés sont intenses, plus la modulation est forte
- Remplace l'ancien système `bias_pair_synergy` (même=-0.2, différent=+0.2)

#### Agrégation finale (poids dynamiques)

```
raw = Σ(score_cat × poids_cat)  pour chaque catégorie active
total_w = Σ(poids_cat)
score = round(raw / total_w × 100)   → [0, 100] (plus de plafond)
```

Quand une catégorie manque de données (ex: aucun pattern, aucune dimension
de réputation commune) → elle est exclue et son poids est réparti
proportionnellement sur les catégories restantes.

---

## 📄 Licence

MIT — Libre d'utilisation, modification et distribution.

> 🧩 Construit avec Kotlin, HTML/CSS/JS vanilla, et GitHub Actions.
