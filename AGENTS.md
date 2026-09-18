# People Modeler — project conventions

## Dioxus reactivity

- **Derive "session snapshots"/pending-state from `p`/props, never from live signals.**
  `let x = some_signal()` at the top of a big component silently re-tracks on every
  unrelated edit (whole-scope re-evaluation). Before the discard/save flows trust a
  value, re-read it from the props at snapshot time, not from a signal that the UI
  itself keeps writing. Reading a signal you write is how "Discard reverts to itself"
  bugs happen.
- Signal reads inside `use_effect`/handlers are explicit subscriptions; top-level reads
  on a big component are implicit ones. Prefer the former for derived state.

## Base/Work facet drift

- Every facet-aware field exists in **two** forms: Base and Work (often as
  `resilience`/`work_resilience`, plus a `has_*` override flag). They have drifted
  before (OCEAN, Resilience/Risk, header badges, section heights) and caused bugs.
- Review checklist — when editing one side, verify the sibling:
  - `rg -n "fn .*[Ff]ield|resilience|risk_appetite" app/src/pages/person_edit.rs`
  - confirm the save path (`Person` vs `WorkPersona`) and the discard snapshot both
    carry the mirrored field.
- Prefer one generic "facet-aware field" component/helper over hand-mirrored markup.
  New facet fields should not be added twice by hand if a shared form exists.

## CSS / flexbox

- Shrinkable flex children in a row need `min-width: 0` (or `overflow-wrap: break-word`)
  on narrow viewports, or they wrap/squeeze the row (title wrap, toolbar wrap, button
  squeeze have all shipped then been patched reactively). New `display: flex` rows
  should include it from the start.
- On mobile fixes, audit sibling flex rows before shipping.

## WASM-gated logic

- `#[cfg(target_arch = "wasm32")]` code is structurally untestable by native `cargo test`.
  Extract the pure decision logic behind the gate as an ordinary function, keep the
  web_sys call at the boundary, and unit-test the pure part (see the cascade-delete fix).

## i18n

- **Call sites with a literal key must use `crate::tr!("key", lang)` (the exported
  macro), not the raw `crate::i18n::tr` function.** The macro carries a `const`
  assert against `VALID_KEYS`, so a typo or a key missing from the list is a build
  error instead of a silent `_ => "Unknown"` fallthrough.
- Adding/sharing a key means touching **three** places: the `en`/`fr` match arms in
  `app/src/i18n.rs` (and `core/src/i18n.rs` for core keys) plus the
  `app/src/i18n.rs::VALID_KEYS` list. `VALID_KEYS` is enforced by the compile-time
  macro check and also drives the `all_keys_translate_*` tests, so it never drifts alone.
- Dynamic-key call sites (key held in a variable, e.g. `tr(band_keys[i], lang())`)
  keep using the unchecked `crate::i18n::tr` function. Keep these keys on the
  `IDENTITY_KEYS`/machine-key lists only when they are genuinely runtime values.
- Keys are still stringly-typed; if it grows past ~today's scale, migrate to a typed
  key enum as the roadmap suggested.

## Verification

- `cargo check -p peoplemodeler-app`, `cargo clippy -p peoplemodeler-app --all-targets -- -D warnings`, `cargo fmt -p peoplemodeler-app`.
- Native unit tests: `cargo test -p peoplemodeler-core`.
- E2E: Playwright (`tests/`). Cover state machines, not only value entry: Discord/discard,
  accordion open-close, the Work/Base persona toggle.
- Pre-commit hooks (husky) run lint-staged, fmt, clippy, mutation-marker detection and a
  secrets scan; the mutation suite is also run as part of CI.