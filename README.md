# 🧩 People Modeler

> Model people as systems: motivations, biases, behaviors.

[![Build](https://github.com/stellasecret/peoplemodeler/actions/workflows/build.yml/badge.svg)](https://github.com/stellasecret/peoplemodeler/actions)
[![Web](https://img.shields.io/badge/Web-GitHub%20Pages-blue)](https://stellasecret.github.io/PeopleModeler)

---

## ⚠️ Ethical Note

> This project is a tool for **understanding**, not manipulation.
> Use it to improve your relationships, leadership, and empathy.
> Knowledge of human systems is a responsibility.

---

## 📁 Project Structure

```
PeopleModeler/
├── core/                       # Rust engine (WASM + JNI)
│   ├── src/
│   │   ├── lib.rs              # Entry point, WASM exports
│   │   ├── models.rs           # Types: Person, Motivation, Bias, BehaviorPattern, StyleType...
│   │   ├── synergy.rs          # Synergy scoring (OCEAN, Rep, Mot, Pat, Bias, Style)
│   │   ├── insights.rs         # Behavioral insight generation
│   │   ├── predictions.rs      # Prediction logic
│   │   ├── ocean.rs            # OCEAN interpretation
│   │   ├── i18n.rs             # Internationalization (EN/FR)
│   │   ├── validation.rs       # Consistency warnings
│   │   ├── wasm.rs             # WebAssembly exports (JS)
│   │   └── android.rs          # JNI exports (Kotlin, legacy)
│   └── Cargo.toml
│
├── app/                        # Dioxus app (Web WASM)
│   ├── src/
│   │   ├── main.rs             # Entry point, routing
│   │   ├── i18n.rs             # App i18n (EN/FR)
│   │   ├── db/mod.rs           # SQLite storage
│   │   ├── pages/
│   │   │   ├── person_list.rs      # Person list
│   │   │   ├── person_detail.rs    # Person detail (tabs)
│   │   │   ├── person_edit.rs      # Person edit
│   │   │   ├── compare.rs          # Compare 2 profiles
│   │   │   ├── insights.rs         # Global insights
│   │   │   ├── predictions.rs      # Predictions
│   │   │   └── sync.rs             # Google Drive sync
│   │   ├── drive.rs            # Google Drive backup
│   │   ├── templates.rs        # Person archetypes
│   │   └── theme.rs            # Theme
│   ├── assets/styles.css       # Styles
│   └── Cargo.toml
│
├── tests/                      # E2E Playwright tests
├── scripts/
│   └── spa_server.py           # Dev SPA server
├── public/                     # Static assets (sw.js, manifest.json)
└── .github/workflows/build.yml # CI/CD pipeline
```

---

## 🚀 GitHub Actions Pipeline

The `.github/workflows/build.yml` pipeline does:

### 1. `rust-core` — Rust Engine
- Rust stable compile (check + clippy + test)
- Blocks `deploy-web` on failure
- WASM exports via `wasm-pack`

### 2. `web-build` — Dioxus App
- WASM build with `dx build --release`
- Uploads `web-static` artifact
- GitHub Pages deployment (requires `rust-core` OK)

### 3. `release` — On tag `v*`
- Creates a GitHub Release
- Auto-generated release notes

---

## 🌐 Web Deployment (GitHub Pages)

1. Go to **Settings → Pages**
2. Source: **Deploy from a branch**
3. Branch: `gh-pages`
4. The pipeline auto-deploys on every push to `main`

URL: `https://stellasecret.github.io/PeopleModeler/`

---

## 📱 Android (legacy)

The old Android app (Kotlin + Room + MVVM) is no longer maintained.
Features have been migrated to the Dioxus Web/WASM app.

---

## 🌐 App Features (Dioxus Web/WASM)

### Pages
1. **List** — Search, cards with OCEAN/motivations/biases chips
2. **Detail** — Full profile with tabs: Motivations, Biases, OCEAN, Reputation, Predictions, Insights, Journal, Relationships, Personal Styles
3. **Edit** — Full form: OCEAN, motivations, biases, reputation (13 dimensions), behavioral patterns (9 triggers, 28 responses), personal styles (8 categories, 41 variants), resilience (1-10), risk appetite (1-10)
4. **Compare** — Synergy score with per-category breakdown
5. **Predictions** — Feedback and accuracy
6. **Insights** — Global analysis and statistics
7. **Sync** — Google Drive backup

---

## 🛠️ Local Development

### App (Dioxus)
```bash
# Run in dev (hot-reload)
dx serve

# Release WASM build
dx build --release

# Tests
cargo test
cargo clippy
```

### SPA Server (for E2E tests)
```bash
python3 scripts/spa_server.py
```

---

## 📦 Creating a Release

```bash
git tag v1.0.0
git push origin v1.0.0
# → Pipeline auto-creates a GitHub Release with the APK
```

---

## 🔬 Data Model

```
Person
├── id, name, role, context, avatarEmoji
├── motivations[]        # type (enum 10), intensity (1-10), notes
├── biases[]             # type (enum 11), intensity (1-10), evidence
├── behavioralPatterns[] # trigger (enum 9), predictedBehavior (enum 55), notes
├── styles[]             # type (enum 41), intensity (1-10), notes
├── ocean                # O, C, E, A, N (Option<u8>, 1-10)
├── resilience           # Option<u8> 1-10, recovery capacity
├── risk_appetite        # Option<u8> 1-10, comfort with uncertainty
├── rep_scores           # 13 dimensions Option<u8> (0-10), bipolar:
│                        #   Hardworker↔Lazy, Authoritative↔Submissive
│                        #   Honest↔Deceitful, Reliable↔Flaky
│                        #   Humble↔Arrogant, Calm↔Reactive
│                        #   Diplomatic↔Blunt, Generous↔Selfish
│                        #   Fair↔Favoritism, Trusting↔Suspicious
│                        #   Assertive↔Passive, Empathetic↔Detached
│                        #   Adaptable↔Rigid
│                        #   ≥5 = pole A, <5 = pole B, None = not set
├── tags[]
├── predictions[]        # context, predicted, actual, accuracy, resolvedAt
├── relationships[]      # sourceId, targetId, type, strength
├── log[]                # InteractionEntry: type, description, timestamp
└── confidence           # 1-10, profile reliability
```

### Synergy Score (comparing 2 people)

Base weights when all categories have data:

```
OCEAN×17% + Reputation×26% + Motivation×19% + Patterns×14% + Bias×13% + Styles×11%
```

If a category has no data (e.g. no shared pattern), its weight is redistributed
proportionally to the other active categories.

#### 1. OCEAN (17%)

Continuous distance per trait (1-10) + complementarity bonus:

```
sim(x, y) = 1.0 - |x - y| / 10          → [0.0, 1.0] per trait

OC = (sim(O_A, O_B) + sim(C_A, C_B)) / 2
EA = (sim(E_A, E_B) + sim(A_A, A_B)) / 2
N  =  sim(N_A, N_B)

oc_bonus = 0.15 if (O_A≥7 ∧ C_B≥7) ∨ (O_B≥7 ∧ C_A≥7), else 0
ea_bonus = 0.15 if (E_A≥7 ∧ A_B≥7) ∨ (E_B≥7 ∧ A_A≥7), else 0

OCEAN_raw = (min(OC + oc_bonus, 1) + min(EA + ea_bonus, 1) + N) / 3
```

- `sim` replaces the old thresholds (0.15/0.7/1.0) with a continuous value
- `bonus` rewards O-C and E-A complementarity without overriding distance

**OCEAN Danger Penalties** — trait combinations known to generate friction,
**within the same person** and **between both**:

```
OCEAN penalty = Σ(below)

Intra-person (each person):
  N ≥ 7 and A ≤ 4   → emotional volatility                +0.10
  N ≥ 7 and C ≤ 4   → impulsiveness                       +0.05
  N ≥ 7 and O ≤ 4   → anxious rigidity                    +0.05

Inter-person (both):
  Both N ≥ 7        → emotional contagion                 +0.10
  Both A ≤ 4        → mutual antagonism                   +0.15
  Both C ≤ 4        → reciprocal unreliability             +0.10
  Both O ≤ 4        → shared rigidity                     +0.05
```

OCEAN final after modulation and penalty:

```
OCEAN_penalized = max(OCEAN_raw - OCEAN_penalty, 0)
OCEAN_final = min(OCEAN_penalized × (1 + bias_modulations_OCEAN), 1)
```

#### 2. Reputation (26%)

For each dimension (13 bipolar) where A and B have a value:

```
similarity = 1.0 - |score_A - score_B| / 10   → [0.0, 1.0]
Rep_raw = Σ(similarity_dim × weight_dim) / Σ(weight_dim)
```

Dimensions have different weights based on relational impact:

| Dimension | Weight |
|---|---|
| Honest ↔ Deceitful | 0.15 |
| Reliable ↔ Flaky | 0.12 |
| Authoritative ↔ Submissive | 0.12 |
| Humble ↔ Arrogant | 0.12 |
| Hardworker ↔ Lazy | 0.07 |
| Calm ↔ Reactive | 0.07 |
| Diplomatic ↔ Blunt | 0.07 |
| Fair ↔ Favoritism | 0.07 |
| Trusting ↔ Suspicious | 0.05 |
| Assertive ↔ Passive | 0.05 |
| Empathetic ↔ Detached | 0.05 |
| Generous ↔ Selfish | 0.04 |
| Adaptable ↔ Rigid | 0.04 |

> Sum = 1.02, normalized at runtime by `total_active_w`.

- If **no** shared dimension: category inactive, weight redistributed

**Reputation Danger Penalties** — same extreme poles in both:

Reputation scores are bipolar: `0 = negative pole`, `10 = positive pole`.
Thresholds below use the raw score value.

```
Rep_penalty = Σ(below)

Both Authoritative ≥ 8  → power struggle                 +0.10
Both Blunt ≤ 3          → brutality, no diplomacy          +0.10
Both Reactive ≤ 3       → mutual escalation                +0.10
Both Arrogant ≤ 3       → neither backs down              +0.10
Both Lazy ≤ 3           → mutual passivity                 +0.05
Both Deceitful ≤ 3      → trust collapse                   +0.10
Both Flaky ≤ 3          → mutual unreliability             +0.08
Both Suspicious ≤ 3     → mutual suspicion                 +0.08
Both Detached ≤ 3       → mutual coldness                  +0.08
Both Favoritism ≤ 3     → cronyism                         +0.08
Both Selfish ≤ 3        → mutual hoarding                  +0.05
Both Passive ≤ 3        → decision paralysis               +0.05
Both Rigid ≤ 3          → mutual blockage                  +0.05
```

Reputation final after modulation and penalty:

```
Rep_penalized = max(Rep_raw - Rep_penalty, 0)
Rep_final = min(Rep_penalized × (1 + bias_modulations_Rep), 1)
```

**Reputation Adjustment** — applied to the individual score (*compute_person_profile*):

Each undefined dimension penalizes (−0.02). Extreme values adjust based on dimension type:

| Type | Condition | Adjustment |
|---|---|---|
| **Good pole** (Honest, Reliable, Humble, Hardworker, Calm, Generous, Fair, Empathetic, Adaptable) | ≤ 2 | −0.05 |
| | ≥ 9 | +0.03 |
| **Contextual** (Authoritative/Submissive, Diplomatic/Blunt, Trusting/Suspicious, Assertive/Passive) | ≤ 2 or ≥ 9 | −0.04 |
| | 4‑6 | +0.02 |
| **Any dimension** | undefined | −0.02 |

```text
Rep_adjusted = clamp(base_rep_quality(person) + rep_adjustment(&person.rep_scores), 0, 1)
```

> Contextual dimensions have no universally positive/negative pole: extremes are penalized, center (4-6) is rewarded.

#### 3. Motivation (19%)

Pairs weighted by `intensity_A × intensity_B / 100`. Neutral pairs
(synergy = 0.0) are skipped to avoid dilution bias. The resulting
average is remapped from `[−0.3, +0.3]` to `[0, 1]`:

```
avg = weighted_average(mot_synergy(type_A, type_B), weights, skip_neutral)
Mot_raw = (avg + 0.3) / 0.6   → clamp [0, 1]
```

Table `motivation_synergy(tA, tB)`:

🤝 Same type: depends on motivation — Power × Power = **−0.2** (competition),
Recognition × Recognition = **−0.1** (ego battle), Autonomy × Autonomy = **0.0**
(neutral independence), Security × Security = **0.0** (status quo). Others
(Achievement, Affiliation, Learning, Helping, Creativity, Fairness) stay at **+0.2** (alignment).

🔄 Complementarity: productive asymmetric pairs — Power × Helping = **+0.1**
(one leads, the other supports), Achievement × Affiliation = **+0.1** (results + harmony).

| tA \ tB | Power | Achieve | Affil | Security | Autonomy | Recogn | Learn | Helping | Creativ | Fairness |
|---|---|---|---|---|---|---|---|---|---|---|
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

##### Virtue adjustment (individual profile)

Before synergy computation, each individual profile (`compute_person_profile`)
applies a moral adjustment to its motivation score based on virtues/vices:

| Motivation | ≥ 7 (virtue) | ≤ 3 or absent (vice) |
|---|---|---|
| Fairness | +0.08 | −0.08 |
| Helping | +0.06 | −0.06 |
| Learning | +0.04 | 0 |
| Creativity | +0.04 | 0 |
| Power | −0.08 (vice, threshold 7) | 0 |
| Security | −0.05 (vice, threshold 7) | 0 |
| Recognition | −0.03 (vice, threshold 9) | 0 |
| Others (Achievement, Affiliation, Autonomy) | 0 | 0 |

**Count Penalty**: if the person has few motivations, their score is reduced:

| Motivations | Penalty |
|---|---|
| 0 | −0.09 |
| 1 | −0.06 |
| 2 | −0.03 |
| 3+ | 0.0 |

These adjustments are applied to the motivation score **in the profile** before
any cross-person synergy computation.

#### 4. Patterns (14%)

Pairs weighted by `conf_A × conf_B / 100`. Neutral pairs
(synergy = 0.0) are skipped (same logic as motivations).

```
avg = weighted_average(trigger_synergy(tA, tB), weights, skip_neutral)
Patterns_raw = (avg + 0.3) / 0.6   → clamp [0, 1]
```

Table `trigger_synergy(tA, tB)`:

| tA \ tB | Change | Feedback | Success | Conflict | Stress | Uncertainty | Recognition | Threatened | Injustice |
|---|---|---|---|---|---|---|---|---|---|
| **Change** | +0.3 | +0.3 | 0 | 0 | -0.2 | 0 | 0 | 0 | 0 |
| **Feedback** | +0.3 | +0.3 | 0 | 0 | 0 | 0 | +0.2 | 0 | 0 |
| **Success** | 0 | 0 | +0.3 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Conflict** | 0 | 0 | 0 | -0.3 | -0.3 | -0.2 | 0 | 0 | 0 |
| **Stress** | -0.2 | 0 | 0 | -0.3 | -0.2 | 0 | 0 | 0 | 0 |
| **Uncertainty** | 0 | 0 | 0 | -0.2 | 0 | 0 | 0 | 0 | 0 |
| **Recognition** | 0 | +0.2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Threatened** | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Injustice** | 0 | 0 | 0 | −0.1 | −0.1 | −0.1 | 0 | 0 | −0.2 |

##### Pattern Adjustment (individual profile)

Before cross-person comparison, the individual pattern score is adjusted
based on the quality of the **chosen response** (not the trigger or intensity).
Each `BehaviorResponse` variant has a built-in score from −0.03 to +0.03
in 7 tiers:

| Tier | Score | Color | Example responses |
|---|---|---|---|
| 1 | **+0.03** | ⭐ Proactive | RemainsCalm, FacilitatesResolution, EmbracesChange |
| 2 | **+0.02** | 🟢 Constructive | SeeksSupport, CommunicatesOpenly, PlansAhead |
| 3 | **+0.01** | 🟢 Adaptive | StaysFocused, SeeksCompromise, Reflects |
| 4 | **0.00** | 🟡 Neutral | BecomesQuiet, WaitsForClarity, ResistsChange |
| 5 | **−0.01** | 🟠 Mild friction | BecomesIrritable, OverPlans, RejectsFeedback |
| 6 | **−0.02** | 🔴 Bad | Overwhelmed, BecomesDefensive, BecomesOverconfident |
| 7 | **−0.03** | 🔴 Maladaptive | Panics, Escalates, Sabotages |

Each undefined trigger (not present in the person's vector) incurs −0.02.

```
Pat_adjusted = clamp(raw_pat + pattern_adjustment(&person.behavioral_patterns), 0, 1)
```

The score is stored on `BehaviorResponse::score()` (see `core/src/models.rs`).

**Patterns Danger Penalty** — when both persons have **only** negative
triggers (Conflict, Stress, Threatened, Injustice), no positive pattern
balances the relationship:

```
Patterns_penalty = 0.05 if both have only negative triggers
                   0.00 otherwise
```

Patterns final after modulation and penalty:

```
Patterns_penalized = max(Patterns_raw - Patterns_penalty, 0)
Patterns_final = min(Patterns_penalized × (1 + bias_modulations_Patterns), 1)
```

#### 5. Bias (13%)

Biases are **not** scored directly. Each bias type modulates another
category of the score when shared by both persons:

```
bias_modifier(type) → (target, coefficient)

Anchoring     → OCEAN       +0.10  (anchoring of first impressions)
Confirmation  → Reputation  +0.10  (confirmation seeking)
Availability  → Patterns    +0.10  (weight of recent events)
SunkCost      → Motivation  +0.10  (past investment)
DunningKruger → OCEAN       -0.10  (distorted self-assessment)
LossAversion  → Patterns    -0.10  (excessive weight on negatives)
SocialProof   → Reputation  +0.08  (group influence)
Authority     → Motivation  +0.08  (deference to authority)
Recency       → Patterns    +0.08  (emphasis on recent)
InGroup       → OCEAN       +0.08  (in-group favoritism)
Favoritism    → Reputation  -0.08  (preferential treatment)
```

For each bias pair **of the same type** (shared by A and B):

```
modulation = coefficient × (intensity_A × intensity_B / 100)
modulated_cat_score = raw_cat_score × (1.0 + Σ_modulations)   → clamp [0, 1]
```

**Bias score** for the individual profile: combination of a counting
base, an intensity adjustment, and a rarity bonus.

The **present bias count** includes:
- **undefined** types (absent from the person's vector) — by default,
  an unset bias is considered present
- defined types with intensity **≥ 4** (moderate or strong)

Types with intensity **0** (explicitly absent) or **≤ 3** (mild) are
not counted as present.

1. **Base**: `1.0 - present_bias_count / 11`
2. **Intensity adjustment** (`bias_adjustment`):

   | Status | Adjustment |
   |---|---|
   | Undefined (absent from vector) | 0 (but counted in base) |
   | Intensity **0** (explicitly absent) | +0.02 |
   | ≤ 3 (mild) | +0.01 |
   | 4‑6 (moderate) | 0 |
   | ≥ 7 (strong) | −0.03 |

3. **Rarity bonus** (`bias_count_bonus`) — based on `present_bias_count`:

   | Present biases | Bonus |
   |---|---|
   | 0 | +0.09 |
   | 1 | +0.06 |
   | 2 | +0.03 |
   | 3+ | 0.0 |

```
profile_bias_score = (base + adjustment + bonus).clamp(0, 1)
```

These adjustments are applied to the bias score **in the profile** before any
cross-person comparison computation.

**Cross-person bias score** (comparison): fraction of shared bias types:

```
bias_score = shared_types / max(len(A_types), len(B_types))
             → 0.5 if no biases set
quality_Bias(P) = 1 - bias_count / 11  (11 bias types)
```

- Shared bias = both persons have the same bias → modulation applied
- Unshared bias = no effect (neither bonus nor penalty)
- The more intense the shared biases, the stronger the modulation
- Replaces the old `bias_pair_synergy` system (same=-0.2, different=+0.2)

#### Profile Completeness

The profile is 100% complete when all following fields are filled:

| Category | Max | Detail |
|---|---|---|
| OCEAN | 5 | 5 Big Five traits filled |
| Motivations | 3 | capped at 3 (beyond doesn't help) |
| Biases | 11 | 11 bias types in the vector |
| Reputation | 13 | 13 bipolar dimensions enabled |
| Styles | 8 | 8 style categories (1 per category) |
| Patterns | 5 | capped at 5 behavioral patterns |
| **Total** | **45** | |

```rust
completion = filled / 45   → [0, 1]
```

Displayed as a percentage in the detail page and the person list.

#### 6. Personal Styles (11%)

Personal styles measure the compatibility of preferred working
modes across 8 categories:

| Category | Variants |
|---|---|
| 💬 Communication | Direct, Diplomatic, Reserved, Expressive |
| 🤝 Conflict Resolution | Competing, Collaborating, Compromising, Avoiding, Accommodating |
| 🧠 Decision Making | Analytical, Intuitive, Participatory, Autocratic, ConsensusDriven |
| 👥 Leadership | Visionary, Servant, Transactional, Transformational, Bureaucratic |
| ⏰ Time Orientation | PastOriented, PresentOriented, FutureOriented |
| 📜 Moral Framework | RuleBased, OutcomeBased, VirtueBased, Relativist |
| 🫂 Interpersonal Conduct | Opportunistic, Intrusive, Manipulative, PassiveAggressive, Controlling, Detached, Respectful, Empathetic, Supportive, Nurturing |
| 🔗 Trust Style | ExtendsTrustFreely, EarnsTrustGradually, VerifiesTrust, Guarded, RepairsTrustActively |

For each category where both persons have a style set:

```
sim_style(cat) = 1.0 if same variant
                 0.5 if different variant
styles_raw = average of sim_style over shared categories
             0.5 if no categories in common
```

#### 7. Historical Factor (blind-spot tracking)

If both persons have ≥ 3 resolved predictions, their **average accuracy**
(< 5/10) indicates unreliable self-assessment:

```
historical_penalty =
  0.05 if both have avg < 5
  0.03 if one has avg < 5
  0.00 otherwise
```

#### Final Aggregation (dynamic weights)

The base score (compatibility categories) and asymmetric scores use
the same fixed weights redistributed dynamically:

```
weight_OCEAN   = 0.17
weight_Rep     = 0.26
weight_Mot     = 0.19
weight_Patterns = 0.14
weight_Bias    = 0.13
weight_Styles  = 0.11
```

When a category lacks data → it is excluded and its weight is redistributed
proportionally to the remaining categories. Motivation (weight 0.19) is
**always active** — even without data, the 0.19 weight is kept (a scarcity
penalty applies instead, see §3).

#### Asymmetric Score (individual benefit)

Each person receives their own score (`a_score` / `b_score`) reflecting what
they *benefit* from the other, computed per category:

- **OCEAN**: partner quality weighted by similarity. For each trait,
  the contribution is `B_quality × sim(A, B)` where
  `sim(A, B) = 1 - |A/10 - B/10|`. Asymmetric because `B × sim ≠ A × sim`
  when trait levels differ. Result: average of 5 traits.

- **Reputation**: raw quality of the other
  (`base_rep_quality(P) = weighted average of scores / 10`).

- **Bias**: absence of biases in the other
  (`base_bias_quality(P) = 1 - bias_count / 10`).

- **Motivation / Patterns / Styles**: mutual synergy (identical for both).

```
active_weight = Σ(cat_weight) for each active category
a_raw = OCEAN_score_a × 0.17 + Rep_quality_B × 0.26 + Mot_synergy × 0.19
       + Patterns_synergy × 0.14 + Bias_quality_B × 0.13 + Styles_synergy × 0.11
b_raw = OCEAN_score_b × 0.17 + Rep_quality_A × 0.26 + Mot_synergy × 0.19
       + Patterns_synergy × 0.14 + Bias_quality_A × 0.13 + Styles_synergy × 0.11

a_score = round(a_raw / active_weight × 100) → clamp [0, 100]
b_score = round(b_raw / active_weight × 100) → clamp [0, 100]
```

The **total score** is the average of the two, reduced by danger penalties:

```
total = round((a_score + b_score) / 2) - danger_pts
danger_pts = round(danger / active_weight × 100)
```

`danger` is the weighted sum of OCEAN, Reputation, Patterns, and
historical penalties:

```
danger = OCEAN_penalty × 0.17 + Rep_penalty × 0.26
       + Patterns_penalty × 0.14 + historical_penalty × 0.10
```

The `danger` field in `SynergyBreakdown` exposes this value for transparency
(without double subtraction).

The UI displays three scores: `{A}% – {total}% – {B}%` with directional
arrows showing who benefits more.

---

## 📄 License

MIT — Free to use, modify, and distribute.

> 🧩 Built with Kotlin, HTML/CSS/JS vanilla, and GitHub Actions.
