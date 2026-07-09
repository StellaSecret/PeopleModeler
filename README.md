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
Synergy = OCEAN×30% + Réputation×30% + Motivation×20% + Patterns×12% + Biais×8%
```

- **OCEAN**: complémentarité O-C, E-A, similarité N (distance ≤3)
- **Réputation**: moyenne des similarités sur les dimensions partagées
  `1.0 - |a - b| / 10` par dimension, moyenne des dimensions renseignées
- **Motivation**: bonus si types différents, pondéré par intensité min
- **Patterns**: complémentarité des triggers, pondéré par confiance
- **Biais**: bonus si types différents
- Plafond \[25, 98\] — jamais 0% ou 100%

---

## 📄 Licence

MIT — Libre d'utilisation, modification et distribution.

> 🧩 Construit avec Kotlin, HTML/CSS/JS vanilla, et GitHub Actions.
