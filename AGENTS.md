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

- Keys are stringly-typed in `crate::i18n::tr(key, lang)`. Prefer adding keys to the
  canonical match tables (`app/src/i18n.rs`, `core/src/i18n.rs`) with both languages;
  an unmatched key silently falls through to `_ => "Unknown"`. If this grows past ~today's
  scale, migrate to a typed key enum.

## Verification

- `cargo check -p peoplemodeler-app`, `cargo clippy -p peoplemodeler-app --all-targets -- -D warnings`, `cargo fmt -p peoplemodeler-app`.
- Native unit tests: `cargo test -p peoplemodeler-core`.
- E2E: Playwright (`tests/`). Cover state machines, not only value entry: Discord/discard,
  accordion open-close, the Work/Base persona toggle.
- Pre-commit hooks (husky) run lint-staged, fmt, clippy, mutation-marker detection and a
  secrets scan; the mutation suite is also run as part of CI.