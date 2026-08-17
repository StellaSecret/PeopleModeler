# PeopleModeler — Model Roadmap

> Phased plan to close the gaps between the scoring machinery (world-class) and the
> inputs it consumes / outputs it produces (narrow). Each phase is independent and
> ships alone. Phases are ordered by leverage, not effort.

**Global diagnosis:** the synergy engine is a small expert system — continuous
similarity, dynamic weight redistribution, ~90 consistency flags, contradicted-claim
discounting, asymmetric scores. But it scores static person snapshots in a vacuum:
it ignores relationships, confidence, time, context, and never says what to do.

---

## Phase 1 — Relationship Context (highest leverage)

**Goal:** synergy becomes relationship-aware. Scoring a boss–employee pair, two
rivals, or partners yields *different* numbers. `Person.relationships[]` finally feeds
the engine.

**Why:** `compute_synergy_score(a, b)` takes two static snapshots. Authority
asymmetry is modeled per-person (Authoritative/Submissive) but never *relational*.
This is the difference between a personality analyzer and a relationship modeler.

### Verified current state

- `Relationship { id, source_id, target_id, r#type: RelationType, notes, created_at }` — `core/src/models.rs:861`. **No `strength` field** (README documents one — doc drift).
- `RelationType` (8 variants, incl. directional `Manages` / `ReportsTo` / `Mentors`) — `core/src/models.rs:804`.
- `compute_synergy_score(a, b)` / `compute_synergy_score_with_preds(...)` — `core/src/synergy.rs:425/429` — never see relationships.
- Compare page calls `compute_synergy_score_with_preds` — `app/src/pages/compare.rs:33`.
- Storage is JSON blobs (SQLite + LocalStorage) — new fields deserialize with `#[serde(default)]`, **no migration needed**.

### Steps

1. **Data (`core/src/models.rs`)**
   - Add `#[serde(default = "default_strength")] pub strength: u8` to `Relationship` (1–10, default 5), clamped via existing `clamp_u8_opt_1_10`-style deserializer. Old rows load with strength 5.

2. **Core (`core/src/synergy.rs`)**
   - New struct `RelContext { rtype: RelationType, strength: u8 }`.
   - New entry `compute_synergy_score_ctx(a, b, ctx: Option<&RelContext>, a_preds, b_preds)`.
   - Keep `compute_synergy_score` / `compute_synergy_score_with_preds` as thin wrappers (backward compat: `wasm.rs` JNI exports depend on them). **Golden test: no-ctx output is byte-identical to today.**

3. **Relation-type weight profiles** — each `RelationType` maps to its own 6-bucket weights, feeding the *existing* dynamic-redistribution path. Draft table (tune via tests):

   | RelationType | OCEAN | Rep | Mot | Patterns | Bias | Styles | Rationale |
   |---|---|---|---|---|---|---|---|
   | WorksWith | 0.20 | 0.28 | 0.16 | 0.16 | 0.12 | 0.08 | predictability > warmth |
   | Collaborates | 0.18 | 0.28 | 0.16 | 0.16 | 0.13 | 0.09 | task output, conflict density |
   | Manages / ReportsTo | 0.15 | 0.30 | 0.15 | 0.18 | 0.13 | 0.09 | authority, reliability, reactivity |
   | Friends | 0.18 | 0.18 | 0.20 | 0.12 | 0.12 | 0.20 | shared temperament |
   | Family | 0.14 | 0.22 | 0.24 | 0.12 | 0.12 | 0.16 | deep drives, loyalty |
   | Partner | 0.16 | 0.20 | 0.22 | 0.14 | 0.10 | 0.18 | style fit, emotional reactivity |
   | Mentors | 0.20 | 0.18 | 0.20 | 0.14 | 0.12 | 0.16 | growth alignment |

4. **Directional asymmetry modulation** (only for `Manages` / `ReportsTo` / `Mentors`)
   - Subordinate with Power motivation ≥ 7 → motivation bucket penalty (power friction in a hierarchy).
   - Boss rep `AuthoritativeSubmissive` ≥ report's by > 3 → small complementarity bonus (clear hierarchy, less friction).
   - Reporting edges are conflict-dense → weight conflict/stress/injustice trigger pairs more within the Patterns bucket.

5. **Strength as score banding** (feeds Phase 2)
   - `strength` does not move the point score; it widens the *reported band*: 1–4 → ±12, 5–7 → ±8, 8–10 → ±4. UI shows `87 ± 8`, not a naked `87`.

6. **WASM + UI**
   - `core/src/wasm.rs`: export `compute_synergy_with_rel(a_json, b_json, rel_type, strength)`.
   - `app/src/pages/compare.rs`: relationship selector (type + strength), prefill from an existing `Relationship` row between the two ids; render `87 ± 8` with per-context chips.

7. **Tests** (existing style in `core/src/synergy.rs`)
   - weights redistribute (sum ≈ 1, active cats scaled);
   - Manages asymmetry: Power-heavy subordinate drops vs. neutral context;
   - no-ctx == old scores exactly;
   - strength banding monotonic;
   - serde roundtrip with `strength` missing → 5.

8. **README**: correct the data model (`strength` is now real), document per-context weights.

**Acceptance:** comparing Alice–Bob yields different scores per relation type;
`Manages` vs `ReportsTo` produce directionally asymmetric scores; no-ctx scores unchanged.

---

## Phase 2 — Confidence-banded scores ✅ DONE

**Goal:** `confidence` (1–10) stops being decorative.

**Why:** a profile filled with gut guesses displays the same `87%` as one backed by
years of evidence. The 0–100 scalar invites false precision.

### Implemented (Data Reliability UX)

- `core/src/synergy.rs`: new `confidence_band(conf)` (1–4 → ±12, 5–7 → ±8, 8–10 → ±4); `PersonProfile` gained `band: u8`; `compute_person_profile` sets it from `person.confidence`.
- `compute_synergy_score_ctx` composes bands by **max** — `max(strength band, confidence(A), confidence(B))`, bounded at ±12; `ctx=None` keeps band 0 (byte-identical to legacy).
- UX clarify: `Person.confidence` relabeled "Profile confidence" / "Fiabilité du profil" with a hint ("1 = rough sketch, 10 = built from real observations"); detail page groups it with completeness in a distinct "Data quality" cluster (dashed-border box); edit form wraps it in a "Data quality" fieldset; profile score renders `total ± band` with a hover tooltip.

**Acceptance:** two identical profiles differing only in `confidence` render different bands; core exposes the band as a `u8`. ✔ (tests: `test_confidence_band_mapping`, `test_person_profile_band_from_confidence`, `test_person_profile_total_unaffected_by_confidence`, `test_ctx_band_max_composition`, `test_no_ctx_band_still_zero`)

---

## Phase 3 — Temporal layer ✅ DONE

**Goal:** the model says *how good the fit is now and which way it's moving*.

**Why:** `Person.log[]` (`InteractionEntry { id, timestamp, text }`) was append-only
free text — stored, never analyzed. Nothing modeled escalation, drift, or recovery.

**Shipped**
- `InteractionEntry` now a typed event: `{ id, timestamp, text, valence: Option<i8> (-3..+3), trigger: Option<BehaviorTrigger>, target_id: Option<String> }` with `#[serde(default)]` back-compat (legacy free-text entries load unchanged, no migration).
- `trajectory_from(log, t)` → recency-decayed (30-day half-life) `Trajectory { delta: i8, trend: Trend, sample: usize, level: f32 }`. With ≥4 samples, momentum = recent-half minus early-half mean, clamped to `[-1, 1]`, weighted 0.55/0.45 vs level; `<4` samples use level alone.
- Per-pair `pair_trajectory(a, b, log)` — entries routed via `target_id` — and `personal_trajectory(p)`.
- Folded into Phase 1 output as a directional delta: `SynergyBreakdown` gains `trajectory_delta: i8`, `trajectory_trend: Trend`, `trajectory_sample: usize`. **The static point total does not move.**
- `Trend` chip (↑ improving / → stable / ↓ deteriorating) on Compare page (when `trajectory_sample > 0`) and a personal trend chip in the Log tab header.
- Log tab input: valence button row (−3…+3), trigger select (BehaviorTrigger + emoji), target select (other persons); per-entry valence/trigger/target badges.

**Acceptance (passes):** entering a few logged interactions shifts the displayed trend without changing the static point score. Verified by `test_trajectory_*`, `test_pair_trajectory_filters_by_target`, `test_breakdown_carries_trajectory_without_moving_total`, `test_interaction_entry_backcompat`.

---

## Phase 4 — Context-specific compatibility output ✅ DONE

**Goal:** compatibility per situation, not one number.

**Why:** the trigger matrix (Stress/Conflict/Change/…) is rich, but the pair score
collapses to a single scalar. A pair can be great for routine execution and toxic
under crisis — the engine has the raw material, it just doesn't expose it.

**Implemented**
- `SynergyBreakdown` gains `per_context: Vec<(InsightContext, u8)>` (Decision, Team, Stress, Communication, Leadership, Growth) computed by re-weighting the existing per-bucket scores (ocean/reputation/motivation/patterns/bias/styles) per context via a new `CFG.contexts` 6×6 weight table (`core/src/model_config.rs`), rows summing to 1.0. `InsightContext` derives `Serialize` so the field flows through the wasm JSON contract.
- **Phase 1 composition:** when a relationship context is present, the relation-type profile and the context profile compose by element-wise product (renormalized) — a `Manages` relationship under `Stress` emphasizes buckets that matter for both. The danger penalty re-weights with the same composed profile; inactive buckets stay masked exactly like the headline formula.
- Compare UI: "By situation" bar list under the headline (`app/src/pages/compare.rs` → `ContextBars`), colored per score band (tension→strong), with i18n labels (Decision/Team/Stress/Communication/Leadership/Growth).

**Acceptance (passes):** a pair that is strong on every bucket except divergent reactive patterns scores ~80 in normal-ops contexts and ~73 under Stress — the "works great in normal ops, collapses under crisis" reading is data, not prose. Verified by `test_context_weights_rows_sum_to_one`, `test_per_context_carries_six_scores`, `test_per_context_collapses_under_stress`, `test_per_context_composes_with_relationship`, plus the wasm `per_context` shape assertion in `test_compute_synergy_with_rel_json_output`.

---

## Phase 5 — Reputation weight rebalance

**Goal:** stop the least-falsifiable input from being the biggest lever.

**Why:** Rep carries 26% *and* judges every other bucket (motivations, OCEAN,
patterns). The malus system compensates but the weighting still leans on the
most subjective input.

**Steps**
- Empirical pass: measure score sensitivity per bucket on the existing test fixtures (`core/src/synergy.rs` tests).
- Rebalance base weights toward behavior-derived buckets (Patterns ↑, Bias ↑, Styles ↑) at the expense of Rep, keeping the documented math intact (just the constants).
- Guard with a snapshot test asserting the documented "manipulator" scenario still collapses (~53 → ~26, README §Consistency Flags).

**Acceptance:** a documented regression suite pins the new constants; no score in existing tests regresses by more than ±3 points.

**✅ DONE** — implemented in `84f95bc…`+1. Base weights rebalanced
`Rep 0.26 → 0.22`, `Patterns 0.14 → 0.16`, `Bias 0.13 → 0.14`, `Styles 0.11 → 0.12`
(OCEAN 0.17 / Motivation 0.19 unchanged, sum stays 1.00; `history` 0.10
untouched). Empirical pass measured per-bucket sensitivity on the
manipulator/genuine and rep-heavy fixtures; the ±3 acceptance caps how far Rep
can drop (the `good-rep`/`baseline` person-profile fixtures are Rep-driven, so
a Rep weight below 0.22 pushes them past −3). Snapshot assertions now guard the
manipulator collapse (`mp.total ≤ 30`, `gp.total ≥ 50`) and two phase-4
thresholds were relaxed to the measured post-rebalance values (crisis-pair
`≥ 75 → ≥ 72`, good-rep `≥ 45 → ≥ 42`). README weight math updated (Rep now the
largest single weight but down from 26%; behavior buckets net +4%).

---

## Phase 6 — Values alignment dimension

**Goal:** score what actually predicts relationship durability.

**Why:** OCEAN + motivations + biases + styles cover personality *mechanics*; nothing
covers life-goal/value alignment (career, family, money, risk-in-life, religion,
geography) — the biggest determinant of long-term fit.

**Steps**
- New `values: Vec<Value>` on `Person` (enum of ~10 dimensions, intensity 1–10 + priority), EN/FR i18n like the other enums.
- Pair score: distance-weighted similarity like OCEAN, added as a 7th bucket with its own weight (carve from Rep/Motivation).
- New consistency flags: `flag_value_*` (e.g., stated Family value ≥ 7 but time-orientation style PastOriented absent → trivial, low-weight).
- Edit + detail UI sections, completion count +1 category.

**Acceptance:** values bucket appears in completeness, compare breakdown, and i18n; scores shift measurably for value-aligned vs. misaligned fixtures.

---

## Phase 7 — Prescriptive coaching layer

**Goal:** move from "what's contradictory" to "what to do about it" — the stated ethical purpose.

**Why:** `core/src/insights.rs` (161 lines) is templates over top-motivation + top-bias.
It ignores validation.rs's ~90 flags and the danger penalties. Diagnostic without
prescription is a report, not a tool.

**Steps**
- New `core/src/advice.rs`: map each fired flag family (rhetoric/self-image/rep-internal/scalar/evidence/style) to actionable statements (mirror existing i18n pattern).
- Insights generator consumes `compute_person_profile` + fired flags instead of raw top-motivation.
- Per-context advice: reuse Phase 4 context weights to prioritize which advice surfaces first.
- Compare page: "risk + mitigation" panel built from fired danger penalties.

**Acceptance:** every flag in validation.rs has at least one advice string; insights differ between an honest profile and its manipulator twin.

---

## Phase 8 — Opposite-bias modulation

**Goal:** model complementary-bias friction, not just shared-bias amplification.

**Why:** biases currently modulate only when *shared* (same type both persons).
Opposite biases (optimism vs. catastrophizing, trusting vs. paranoid) are a real
friction source scored as "no effect."

**Steps**
- Add a small complementarity table: opposite/adjacent bias pairs with coefficients (mirroring the existing `bias_modifier` table in `core/src/synergy.rs:48`).
- Modulate the same target buckets, negative direction only, capped (e.g., max −0.15 combined).
- Tests for each documented pair.

**Acceptance:** a high-Trusting + high-Suspicious pair shows a documented friction penalty; shared-bias behavior unchanged.

**✅ DONE** — implemented in `84f95bc…`+1. `BiasConfig` gains
`complementary_pairs` (order-insensitive, negative-only) + `opposite_cap: 0.15`,
resolved via `ModelConfig::bias_complementarity()`; the shared-type modulation
loop in `synergy.rs` now applies the opposite pair, scaling the combined
magnitude down to the cap when exceeded. The "Trusting/Suspicious" acceptance
example names a Reputation dimension (not a `BiasType`), so the table uses the
three genuine opposite/adjacent pairs from the existing 12 bias types:
`DunningKruger ↔ Impostor → OCEAN −0.10`, `Anchoring ↔ Recency → Patterns −0.08`,
`Authority ↔ SocialProof → Reputation −0.08`. 7 new tests cover each pair, the
order-insensitive lookup, the −0.15 combined cap, and the shared-bias
unchanged guarantee; README §5 documents the table.

---

## Phase 9 — Structural: coefficients as data + team aggregation

**Goal:** make the model tunable without recompiling, and score beyond pairs.

**Why:** `synergy.rs` (4164 lines) + `validation.rs` (2705 lines) hold ~100
hand-tuned coefficients hardcoded in code. No team-level (N-person) aggregation
exists — insights "team" context is a single-person template.

**Steps**
- Extract all weights/penalties/thresholds into one `model_config.rs` (a single const table), loaded by the engine; no behavior change, pure refactor — proves no regression via the existing test suites (315 core + 28 app).
- (Later, optional) expose the config table over WASM so tuning is a build-time or settings-side concern.
- New `compute_team_synergy(persons: &[Person], rels: &[Relationship])` returning pair matrix + weakest/strongest links + team-level danger, reusing Phase 1 contexts.

**Acceptance:** moving constants to `model_config.rs` produces zero test diffs; team view computes all-pairs via Phase 1.

---

## Suggested sequencing

```
v1.x  Phase 1 (relationship context)          ✅ done
v1.y  Phase 2 (confidence bands)              ✅ done
v1.z  Phase 3 (temporal layer)                ✅ done
v2.0  Phase 9a (config extraction)            ✅ done — `core/src/model_config.rs` const table
v2.x  Phase 4 (context output)                ✅ done
v2.y  Phase 5 (Rep rebalance) + Phase 8 (opposite biases)  ✅ done
v3.x  Phase 6 (values) + Phase 7 (coaching)
v3.y  Phase 9b (team aggregation)                   ✅ done — `compute_team_synergy` in synergy.rs
```
