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

```
Synergy = OCEAN×20% + Réputation×31% + Motivation×23% + Patterns×17% + Biais×9%
```

Chaque catégorie produit un score brut dans [0, 1], puis on applique la pondération
et on plafonne le résultat final dans [25, 98].

#### 1. OCEAN (20%)

Trois sous-scores indépendants, chacun dépendant des valeurs OCEAN (1-10) de A et B :

| Sous-score | Condition | Valeur | Logique |
|---|---|---|---|
| **OC** (O-C) | `O≥7 ∧ C≥7` (un des deux a O haut, l'autre C haut) | 1.0 | Complémentarité Ouverture / Conscience |
| | `\|O_A - O_B\| ≤ 3 ∧ \|C_A - C_B\| ≤ 3` | 0.7 | Profils similaires |
| | Sinon | 0.15 | Pas de synergie particulière |
| **EA** (E-A) | `E≥7 ∧ A≥7` (un E haut, l'autre A haut) | 1.0 | Complémentarité Extraversion / Agréabilité |
| | `\|E_A - E_B\| ≤ 3 ∧ \|A_A - A_B\| ≤ 3` | 0.7 | Profils similaires |
| | Sinon | 0.15 | Pas de synergie particulière |
| **N** (Névrosisme) | `\|N_A - N_B\| ≤ 2` | 0.8 | Niveaux proches (stabilité similaire) |
| | `\|N_A - N_B\| ≤ 4` | 0.5 | Modérément proches |
| | Sinon | 0.1 | Niveaux opposés |

```
OCEAN_brut = (OC + EA + N) / 3       → plage ~[0.13, 0.93]
```

#### 2. Réputation (31%)

Pour chaque dimension de réputation (8 dimensions bipolaires : Autorité, Chaleur,
Compétence, Intégrité, Sociabilité, Dominance, Fiabilité, Prestige) :

- Si A et B ont tous deux renseigné la dimension :
  ```
  similarité = 1.0 - |score_A - score_B| / 10   → [0.0, 1.0]
  ```
- Si aucune dimension commune : score = 0.5 (neutre).

```
Rep_brut = moyenne des similarités sur les dimensions partagées   → [0.0, 1.0]
```

#### 3. Motivation (23%)

Seule la motivation principale (`top_motivation()`, intensité max) de chaque personne
est utilisée.

```
w = min(intensité_A, intensité_B) / 10        → [0.1, 1.0]  (pondération)
base = 0.6 si types différents, 0.3 si identiques
Mot_brut = base + 0.4 × w                     → [0.34, 1.0]
```

Si l'un des deux n'a pas de motivation : score = 0.5.

#### 4. Patterns (17%)

Toutes les paires de patterns sont combinées. Chaque pattern a un `trigger` (parmi
8 valeurs) et une `confidence` (1-10).

```
pour chaque paire (pattern_A, pattern_B) :
    poids = confidence_A × confidence_B / 100        → [0.01, 1.0]
    score = trigger_synergy(trigger_A, trigger_B) × poids

Patterns_brut = (somme des scores) / (somme des poids) + 0.5   → clamp [0.0, 1.0]
```

La table `trigger_synergy(tA, tB)` :

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

(Si aucune paire : score = 0.5 neutre)

#### 5. Biais (9%)

Seul le biais principal (`top_bias()`, intensité max) de chaque personne est utilisé.

```
bias_raw = 0.3 si types de biais différents, 0.0 si identiques ou absents
w = min(intensité_A, intensité_B) / 10                 → [0.1, 1.0]
Biais_brut = (0.5 + bias_raw × w).clamp(0, 1)         → [0.5, 0.8]
```

#### Agrégation finale

```
raw  = OCEAN_brut × 0.20
     + Rep_brut    × 0.31
     + Mot_brut    × 0.23
     + Pat_brut    × 0.17
     + Biais_brut  × 0.09

score = round(raw × 100).max(25).min(98)   → [25, 98]
```

Le plafond évite les extrêmes absolus (jamais 0% ou 100% de compatibilité).

---

## 📄 Licence

MIT — Libre d'utilisation, modification et distribution.

> 🧩 Construit avec Kotlin, HTML/CSS/JS vanilla, et GitHub Actions.
