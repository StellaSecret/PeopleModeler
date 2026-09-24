use dioxus::prelude::*;
use peoplemodeler_core::models::{
    AVATAR_EMOJIS, BehaviorResponse, BehaviorTrigger, BehavioralPattern, Bias, BiasType, FacetKind,
    Motivation, MotivationType, OceanScores, Person, PersonaMask, PersonalStyle, RepDim, RepScores,
    StyleCategory, StyleType, Tag, Value, ValueType,
};

use crate::Route;
use crate::components::facet::FacetToggle;
use crate::db;
use crate::i18n::{Lang, core_lang};

/// `idx` is `hash % templates.len()`, so it can never equal `templates.len()`;
/// the `<=` mutant inside is behavior-equal to `<`.
#[cfg_attr(test, mutants::skip)]
fn person_from_template(
    idx: usize,
    templates: &[crate::templates::Archetype],
    blank: Person,
) -> Person {
    if idx < templates.len() {
        let t = &templates[idx];
        let mut person = blank;
        person.ocean = t.ocean.clone();
        person.motivations = t.motivations.clone();
        person.biases = t.biases.clone();
        person.rep_scores = t.rep_scores.clone();
        person
    } else {
        blank
    }
}

#[component]
pub fn PersonNew() -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut selected = use_signal(|| None::<usize>);
    let mut facet_choice = use_signal(|| None::<FacetKind>);
    let templates = crate::templates::all();
    let new_person_title = crate::tr!(FormNewTitle, lang());
    let template_title = crate::tr!(TemplateTitle, lang());
    let template_blank = crate::tr!(TemplateBlank, lang());
    let context_prompt = crate::tr!(PersonaContextPrompt, lang());
    let context_change = crate::tr!(PersonaContextChange, lang());
    let facet_base = crate::tr!(FacetBase, lang());
    let facet_work = crate::tr!(FacetWork, lang());
    let facet_online = crate::tr!(FacetOnline, lang());

    match facet_choice() {
        None => rsx! {
            div { class: "page",
                h2 { "{new_person_title}" }
                h3 { "{context_prompt}" }
                div { class: "template-grid",
                    button { class: "template-card", onclick: move |_| facet_choice.set(Some(FacetKind::Base)),
                        span { class: "template-emoji", "👤" }
                        strong { "{facet_base}" }
                    }
                    button { class: "template-card", onclick: move |_| facet_choice.set(Some(FacetKind::Work)),
                        span { class: "template-emoji", "💼" }
                        strong { "{facet_work}" }
                    }
                    button { class: "template-card", onclick: move |_| facet_choice.set(Some(FacetKind::Online)),
                        span { class: "template-emoji", "🎮" }
                        strong { "{facet_online}" }
                    }
                }
            }
        },
        Some(primary) => match selected() {
            Some(idx) => {
                let person =
                    person_from_template(idx, &templates, blank_person_with_facet(primary));
                rsx! { PersonEditForm { initial: person } }
            }
            None => rsx! {
                div { class: "page",
                    h2 { "{new_person_title}" }
                    h3 { "{template_title}" }
                    div { class: "template-grid",
                        for (i, tpl) in templates.iter().enumerate() {
                            button { class: "template-card", onclick: move |_| selected.set(Some(i)),
                                span { class: "template-emoji", "{tpl.emoji}" }
                                strong { "{tpl.name}" }
                            }
                        }
                    }
                    p { class: "template-skip",
                        button { class: "btn", onclick: move |_| { selected.set(None); facet_choice.set(None); }, "{context_change}" }
                        button { class: "btn", onclick: move |_| selected.set(Some(999)), "{template_blank}" }
                    }
                }
            },
        },
    }
}

#[component]
pub fn PersonEdit(id: String) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let existing = db::person(&id);
    let not_found = crate::tr!(PersonNotFound, lang());
    match existing {
        None => rsx! { div { class: "page", h2 { "{not_found}" } } },
        Some(p) => rsx! { PersonEditForm { initial: p } },
    }
}

fn blank_person() -> Person {
    Person {
        persona: None,
        online_persona: None,
        private_persona: None,
        primary_facet: FacetKind::Base,
        id: uuid::Uuid::new_v4().to_string(),
        name: String::new(),
        role: String::new(),
        context: String::new(),
        avatar_emoji: "👤".into(),
        tags: Vec::new(),
        notes: String::new(),
        motivations: Vec::new(),
        biases: Vec::new(),
        rep_scores: RepScores::default(),
        behavioral_patterns: Vec::new(),
        styles: Vec::new(),
        values: Vec::new(),
        ocean: OceanScores::default(),
        resilience: None,
        risk_appetite: None,
        confidence: 5,
        log: Vec::new(),
        created_at: chrono::Utc::now().timestamp_millis(),
        updated_at: chrono::Utc::now().timestamp_millis(),
    }
}

/// A blank person whose anchor profile is the given context, not private life
/// by default (a work-only person never needs a "Personal life" tab).
fn blank_person_with_facet(primary: FacetKind) -> Person {
    let mut p = blank_person();
    p.primary_facet = primary;
    p
}

/// Tab label of a context in the edit-mode facet toggle: the anchor context
/// (`kind == primary`) gets its plain name suffixed with "(main)"; every
/// other context wears its persona label ("Work Persona", "Personal life
/// Persona", "Online").
pub(crate) fn facet_tab_label(
    kind: FacetKind,
    primary: FacetKind,
    lang: crate::i18n::Lang,
) -> String {
    if kind == primary {
        format!(
            "{}{}",
            kind.label(crate::i18n::core_lang(lang)),
            crate::tr!(FacetMainSuffix, lang)
        )
    } else {
        match kind {
            FacetKind::Work => crate::tr!(PersonaSection, lang).to_string(),
            FacetKind::Online => crate::tr!(FacetOnline, lang).to_string(),
            FacetKind::Base => crate::tr!(PersonaBaseSection, lang).to_string(),
        }
    }
}

/// The mask a person wears in one arena. `None` on a bucket means "same as
/// base" (the core delta semantics — see `PersonaMask` in the core models).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FacetBucket {
    Ocean,
    Reputation,
    Motivations,
    Biases,
    Patterns,
    Styles,
    Values,
    Resilience,
    RiskAppetite,
}

impl FacetBucket {
    /// The base value this bucket overrides, cloned out of the base profile
    /// so a toggle can materialize it when the bucket turns on.
    fn base_value(&self, p: &Person) -> BucketValue {
        match self {
            Self::Ocean => BucketValue::Ocean(p.ocean.clone()),
            Self::Reputation => BucketValue::Rep(p.rep_scores.clone()),
            Self::Motivations => BucketValue::Motivations(p.motivations.clone()),
            Self::Biases => BucketValue::Biases(p.biases.clone()),
            Self::Patterns => BucketValue::Patterns(p.behavioral_patterns.clone()),
            Self::Styles => BucketValue::Styles(p.styles.clone()),
            Self::Values => BucketValue::Values(p.values.clone()),
            Self::Resilience => BucketValue::Resilience(p.resilience.unwrap_or(5)),
            Self::RiskAppetite => BucketValue::RiskAppetite(p.risk_appetite.unwrap_or(5)),
        }
    }

    fn is_defined(&self, m: &PersonaMask) -> bool {
        match self {
            Self::Ocean => m.ocean.is_some(),
            Self::Reputation => m.rep_scores.is_some(),
            Self::Motivations => m.motivations.is_some(),
            Self::Biases => m.biases.is_some(),
            Self::Patterns => m.behavioral_patterns.is_some(),
            Self::Styles => m.styles.is_some(),
            Self::Values => m.values.is_some(),
            Self::Resilience => m.resilience.is_some(),
            Self::RiskAppetite => m.risk_appetite.is_some(),
        }
    }

    /// Write the override state onto a persona mask: turning a bucket on
    /// materializes `base` (unless it is already defined); turning it off
    /// drops back to inheritance.
    fn set_on_mask(&self, m: &mut PersonaMask, on: bool, base: BucketValue) {
        match self {
            Self::Ocean => {
                if !on {
                    m.ocean = None;
                } else if m.ocean.is_none() {
                    m.ocean = Some(base.into_ocean());
                }
            }
            Self::Reputation => {
                if !on {
                    m.rep_scores = None;
                } else if m.rep_scores.is_none() {
                    m.rep_scores = Some(base.into_rep());
                }
            }
            Self::Motivations => {
                if !on {
                    m.motivations = None;
                } else if m.motivations.is_none() {
                    m.motivations = Some(base.into_motivations());
                }
            }
            Self::Biases => {
                if !on {
                    m.biases = None;
                } else if m.biases.is_none() {
                    m.biases = Some(base.into_biases());
                }
            }
            Self::Patterns => {
                if !on {
                    m.behavioral_patterns = None;
                } else if m.behavioral_patterns.is_none() {
                    m.behavioral_patterns = Some(base.into_patterns());
                }
            }
            Self::Styles => {
                if !on {
                    m.styles = None;
                } else if m.styles.is_none() {
                    m.styles = Some(base.into_styles());
                }
            }
            Self::Values => {
                if !on {
                    m.values = None;
                } else if m.values.is_none() {
                    m.values = Some(base.into_values());
                }
            }
            Self::Resilience => {
                if !on {
                    m.resilience = None;
                } else if m.resilience.is_none() {
                    m.resilience = Some(base.into_resilience());
                }
            }
            Self::RiskAppetite => {
                if !on {
                    m.risk_appetite = None;
                } else if m.risk_appetite.is_none() {
                    m.risk_appetite = Some(base.into_risk_appetite());
                }
            }
        }
    }
}

/// Owned copy of one base bucket, kept in an opaque enum so `FacetBucket` can
/// lift values out of a `Person` before the persona mask gets a mutable
/// borrow (the borrow checker would otherwise reject reading both at once).
#[derive(Debug, Clone, PartialEq)]
enum BucketValue {
    Ocean(OceanScores),
    Rep(RepScores),
    Motivations(Vec<Motivation>),
    Biases(Vec<Bias>),
    Patterns(Vec<BehavioralPattern>),
    Styles(Vec<PersonalStyle>),
    Values(Vec<Value>),
    Resilience(u8),
    RiskAppetite(u8),
}

impl BucketValue {
    fn into_ocean(self) -> OceanScores {
        match self {
            Self::Ocean(v) => v,
            _ => unreachable!("bucket kind mismatch"),
        }
    }
    fn into_rep(self) -> RepScores {
        match self {
            Self::Rep(v) => v,
            _ => unreachable!("bucket kind mismatch"),
        }
    }
    fn into_motivations(self) -> Vec<Motivation> {
        match self {
            Self::Motivations(v) => v,
            _ => unreachable!("bucket kind mismatch"),
        }
    }
    fn into_biases(self) -> Vec<Bias> {
        match self {
            Self::Biases(v) => v,
            _ => unreachable!("bucket kind mismatch"),
        }
    }
    fn into_patterns(self) -> Vec<BehavioralPattern> {
        match self {
            Self::Patterns(v) => v,
            _ => unreachable!("bucket kind mismatch"),
        }
    }
    fn into_styles(self) -> Vec<PersonalStyle> {
        match self {
            Self::Styles(v) => v,
            _ => unreachable!("bucket kind mismatch"),
        }
    }
    fn into_values(self) -> Vec<Value> {
        match self {
            Self::Values(v) => v,
            _ => unreachable!("bucket kind mismatch"),
        }
    }
    fn into_resilience(self) -> u8 {
        match self {
            Self::Resilience(v) => v,
            _ => unreachable!("bucket kind mismatch"),
        }
    }
    fn into_risk_appetite(self) -> u8 {
        match self {
            Self::RiskAppetite(v) => v,
            _ => unreachable!("bucket kind mismatch"),
        }
    }
}

/// The persona mask for `facet` — `None` for the context the inline anchor
/// fields represent (`person.primary_facet`), since that context is always
/// "base". Mirrors `Person::mask_for`, kept here so the form helpers stay
/// standalone/unit-testable.
fn mask_of(p: &Person, facet: FacetKind) -> Option<&PersonaMask> {
    if facet == p.primary_facet {
        return None;
    }
    match facet {
        FacetKind::Base => p.private_persona.as_ref(),
        FacetKind::Work => p.persona.as_ref(),
        FacetKind::Online => p.online_persona.as_ref(),
    }
}

fn mask_mut_of(p: &mut Person, facet: FacetKind) -> Option<&mut PersonaMask> {
    if facet == p.primary_facet {
        return None;
    }
    match facet {
        FacetKind::Base => p.private_persona.as_mut(),
        FacetKind::Work => p.persona.as_mut(),
        FacetKind::Online => p.online_persona.as_mut(),
    }
}

// ── Pure decision helpers ───────────────────────────────────────────────
// Every toggle/accessor on `PersonEditState` and the list/panel components
// is a one-line forward to one of these. They hold all the logic so a plain
// `#[test]` can reach it instead of cargo-mutants only ever seeing Dioxus
// `Signal`/`Memo` reads inside components (the whole reason the mutations
// below were invisible to the suite).

/// Whether a persona facet is enabled, given the resolved per-context flags.
/// The anchor context (`facet == primary`) never wears a mask, so it is never
/// "enabled" as a persona.
fn persona_active_for(
    primary: FacetKind,
    base: bool,
    work: bool,
    online: bool,
    facet: FacetKind,
) -> bool {
    if facet == primary {
        false
    } else {
        match facet {
            FacetKind::Base => base,
            FacetKind::Work => work,
            FacetKind::Online => online,
        }
    }
}

/// Whether a persona mask bucket is an explicit override for `facet` (the
/// anchor context has no mask, so it is never defined there).
fn bucket_defined_in(p: &Person, facet: FacetKind, bucket: FacetBucket) -> bool {
    mask_of(p, facet).is_some_and(|m| bucket.is_defined(m))
}

/// Enable/disable a persona facet on the draft: enabling materializes an
/// empty default mask only when none already exists; disabling drops the
/// mask entirely (back to pure anchor semantics). No-op for the anchor
/// context.
fn set_persona_state(p: &mut Person, facet: FacetKind, on: bool) {
    if facet == p.primary_facet {
        return;
    }
    match facet {
        FacetKind::Base => {
            if on {
                if p.private_persona.is_none() {
                    p.private_persona = Some(PersonaMask::default());
                }
            } else {
                p.private_persona = None;
            }
        }
        FacetKind::Work => {
            if on {
                if p.persona.is_none() {
                    p.persona = Some(PersonaMask::default());
                }
            } else {
                p.persona = None;
            }
        }
        FacetKind::Online => {
            if on {
                if p.online_persona.is_none() {
                    p.online_persona = Some(PersonaMask::default());
                }
            } else {
                p.online_persona = None;
            }
        }
    }
}

/// The "copy from base profile" mask: every bucket becomes an explicit
/// override seeded from the as-loaded base; unset scalars default to 5.
fn base_copy_mask(saved: &Person) -> PersonaMask {
    PersonaMask {
        ocean: Some(saved.ocean.clone()),
        rep_scores: Some(saved.rep_scores.clone()),
        motivations: Some(saved.motivations.clone()),
        biases: Some(saved.biases.clone()),
        behavioral_patterns: Some(saved.behavioral_patterns.clone()),
        styles: Some(saved.styles.clone()),
        values: Some(saved.values.clone()),
        resilience: Some(saved.resilience.unwrap_or(5)),
        risk_appetite: Some(saved.risk_appetite.unwrap_or(5)),
    }
}

/// Unconditionally assign `mask` as the persona for `facet` (anchor: no-op).
fn set_persona_mask(p: &mut Person, facet: FacetKind, mask: PersonaMask) {
    if facet == p.primary_facet {
        return;
    }
    p.set_persona_slot(facet, Some(mask));
}

/// Drop the persona for `facet` (anchor: no-op).
fn clear_persona_mask(p: &mut Person, facet: FacetKind) {
    if facet == p.primary_facet {
        return;
    }
    p.set_persona_slot(facet, None);
}

/// Toggle an override bucket on the draft for `facet`, materializing the
/// base value when the mask exists (base facet: no-op).
fn set_bucket_state(p: &mut Person, facet: FacetKind, bucket: FacetBucket, on: bool) {
    let base = bucket.base_value(p);
    if let Some(m) = mask_mut_of(p, facet) {
        bucket.set_on_mask(m, on, base);
    }
}

/// Row label of a persona mask (the "enable" checkbox + copy/clear buttons).
fn persona_mask_label(facet: FacetKind, lang: crate::i18n::Lang) -> &'static str {
    match facet {
        FacetKind::Work => crate::tr!(PersonaSection, lang),
        FacetKind::Online => crate::tr!(PersonaOnlineSection, lang),
        FacetKind::Base => crate::tr!(PersonaBaseSection, lang),
    }
}

/// Is a facet's action row hidden while editing `mode`? Rows stay mounted
/// (equal bar heights), just visually hidden when not the active facet or
/// when the row is the anchor context (which never wears a mask).
fn persona_row_hidden(primary: FacetKind, mode: FacetKind, facet: FacetKind) -> bool {
    facet == primary || mode != facet
}

/// Whether a persona section panel is editable: an explicit `active` memo
/// wins when present; otherwise the bucket override toggle decides.
fn section_panel_active(active: Option<bool>, bucket_defined: Option<bool>) -> bool {
    match (active, bucket_defined) {
        (Some(active), _) => active,
        (None, Some(defined)) => defined,
        _ => false,
    }
}

/// A fixed-cap list is complete once `filled` reaches `total`; an uncapped
/// list (`None`) is always complete.
fn progress_complete(filled: usize, total: Option<usize>) -> bool {
    total.is_none_or(|total| filled >= total)
}

/// The pattern editor clears its notes buffer only when the list shrank
/// since the last render (the first render sees `usize::MAX` as the last
/// length, so an empty list clears once on mount).
fn notes_clear_on_shrink(len: usize, last: usize) -> bool {
    len < last
}

/// Number of rows in a resolved list bucket: the mask's override when set,
/// otherwise the inherited base list. Pure + unit-testable.
fn list_len(p: &Person, facet: FacetKind, bucket: FacetBucket) -> usize {
    let m = mask_of(p, facet);
    match bucket {
        FacetBucket::Motivations => m
            .and_then(|m| m.motivations.as_deref())
            .unwrap_or(p.motivations.as_slice())
            .len(),
        FacetBucket::Biases => m
            .and_then(|m| m.biases.as_deref())
            .unwrap_or(p.biases.as_slice())
            .len(),
        FacetBucket::Patterns => m
            .and_then(|m| m.behavioral_patterns.as_deref())
            .unwrap_or(p.behavioral_patterns.as_slice())
            .len(),
        FacetBucket::Styles => m
            .and_then(|m| m.styles.as_deref())
            .unwrap_or(p.styles.as_slice())
            .len(),
        FacetBucket::Values => m
            .and_then(|m| m.values.as_deref())
            .unwrap_or(p.values.as_slice())
            .len(),
        _ => 0,
    }
}

/// Single source of truth for "what does Discard restore for this section in
/// this facet" — the same pure decision logic distilled from the old discard
/// closure, now unit-testable. Writes `saved`'s per-facet snapshot back onto
/// the draft for exactly one section.
fn discard_section(section: EditSectionId, facet: FacetKind, saved: &Person, draft: &mut Person) {
    if facet == draft.primary_facet {
        match section {
            EditSectionId::ResilienceRisk => {
                draft.resilience = saved.resilience;
                draft.risk_appetite = saved.risk_appetite;
            }
            EditSectionId::Ocean => draft.ocean = saved.ocean.clone(),
            EditSectionId::Motivations => draft.motivations = saved.motivations.clone(),
            EditSectionId::Biases => draft.biases = saved.biases.clone(),
            EditSectionId::Reputation => draft.rep_scores = saved.rep_scores.clone(),
            EditSectionId::Patterns => {
                draft.behavioral_patterns = saved.behavioral_patterns.clone();
            }
            EditSectionId::Styles => draft.styles = saved.styles.clone(),
            EditSectionId::Values => draft.values = saved.values.clone(),
        }
        return;
    }
    let Some(draft_mask) = mask_mut_of(draft, facet) else {
        return;
    };
    let saved_mask = mask_of(saved, facet);
    match section {
        EditSectionId::ResilienceRisk => {
            draft_mask.resilience = saved_mask.and_then(|m| m.resilience);
            draft_mask.risk_appetite = saved_mask.and_then(|m| m.risk_appetite);
        }
        EditSectionId::Ocean => {
            draft_mask.ocean = saved_mask.and_then(|m| m.ocean.clone());
        }
        EditSectionId::Motivations => {
            draft_mask.motivations = saved_mask.and_then(|m| m.motivations.clone());
        }
        EditSectionId::Biases => {
            draft_mask.biases = saved_mask.and_then(|m| m.biases.clone());
        }
        EditSectionId::Reputation => {
            draft_mask.rep_scores = saved_mask.and_then(|m| m.rep_scores.clone());
        }
        EditSectionId::Patterns => {
            draft_mask.behavioral_patterns = saved_mask.and_then(|m| m.behavioral_patterns.clone());
        }
        EditSectionId::Styles => {
            draft_mask.styles = saved_mask.and_then(|m| m.styles.clone());
        }
        EditSectionId::Values => {
            draft_mask.values = saved_mask.and_then(|m| m.values.clone());
        }
    }
}

/// The form's whole editable state in one place, shared with every scoped
/// child component via `use_context_provider`. `saved`, `work_active` and
/// `online_active` are derived memos (values change only when their real
/// dependencies change), so panels re-render in isolation instead of the
/// whole form re-running on every keystroke.
#[derive(Clone, Copy)]
struct PersonEditState {
    draft: Signal<Person>,
    mode: Signal<FacetKind>,
    open: Signal<Vec<EditSectionId>>,
    saved: Memo<Person>,
    base_active: Memo<bool>,
    work_active: Memo<bool>,
    online_active: Memo<bool>,
}

impl PersonEditState {
    // Handle accessors: `Signal`/`Memo` are `Copy` + callable, but Rust
    // parses `self.field()`/`ctx.field()` as a *method* call, so callable
    // fields get wrapped in same-named methods (field+method name can
    // legally coincide, and field access uses the paren-less form).
    fn draft(&self) -> Signal<Person> {
        self.draft
    }
    #[cfg_attr(test, mutants::skip)]
    fn mode(&self) -> FacetKind {
        (self.mode)()
    }
    #[cfg_attr(test, mutants::skip)]
    fn base_active(&self) -> bool {
        (self.base_active)()
    }
    #[cfg_attr(test, mutants::skip)]
    fn work_active(&self) -> bool {
        (self.work_active)()
    }
    #[cfg_attr(test, mutants::skip)]
    fn online_active(&self) -> bool {
        (self.online_active)()
    }
    fn saved(&self) -> Person {
        (self.saved)()
    }

    #[cfg_attr(test, mutants::skip)]
    fn persona_active(&self, facet: FacetKind) -> bool {
        let primary = self.draft().read().primary_facet;
        persona_active_for(
            primary,
            self.base_active(),
            self.work_active(),
            self.online_active(),
            facet,
        )
    }

    #[cfg_attr(test, mutants::skip)]
    fn bucket_defined(&self, facet: FacetKind, bucket: FacetBucket) -> bool {
        let draft = self.draft();
        let p = draft.read();
        bucket_defined_in(&p, facet, bucket)
    }

    #[cfg_attr(test, mutants::skip)]
    fn set_persona_enabled(&self, facet: FacetKind, on: bool) {
        let mut draft = self.draft();
        let mut p = draft.write();
        set_persona_state(&mut p, facet, on);
    }

    /// "Copy from base profile": every bucket becomes an explicit override
    /// seeded from the as-loaded base (the same snapshot Discard restores).
    #[cfg_attr(test, mutants::skip)]
    fn copy_base(&self, facet: FacetKind) {
        let mask = base_copy_mask(&self.saved());
        let mut draft = self.draft();
        let mut p = draft.write();
        set_persona_mask(&mut p, facet, mask);
    }

    #[cfg_attr(test, mutants::skip)]
    fn clear_persona(&self, facet: FacetKind) {
        let mut draft = self.draft();
        let mut p = draft.write();
        clear_persona_mask(&mut p, facet);
    }

    #[cfg_attr(test, mutants::skip)]
    fn set_bucket_defined(&self, facet: FacetKind, bucket: FacetBucket, on: bool) {
        let mut draft = self.draft();
        let mut p = draft.write();
        set_bucket_state(&mut p, facet, bucket, on);
    }

    #[cfg_attr(test, mutants::skip)]
    fn discard(&self, section: EditSectionId) {
        let facet = self.mode();
        let saved = self.saved();
        let mut binding = self.draft();
        let mut draft = binding.write();
        discard_section(section, facet, &saved, &mut draft);
    }
}

/// A facet-resolved list: `val` mirrors the mask bucket when overridden,
/// otherwise the inherited base list; every edit writes straight through to
/// the draft and materializes the bucket on first write.
#[derive(Clone, PartialEq)]
struct ListField<T: Clone + PartialEq + 'static> {
    val: Memo<Vec<T>>,
    set: EventHandler<ListOp<T>>,
}

#[derive(Clone)]
enum ListOp<T> {
    Push(T),
    Replace(usize, T),
    Remove(usize),
    Swap(usize, usize),
}

/// Apply one edit op to a resolved list. Extracted out of the Dioxus
/// `EventHandler` so the mutation behavior is unit-testable.
fn apply_list_op<T>(list: &mut Vec<T>, op: ListOp<T>) {
    match op {
        ListOp::Push(item) => list.push(item),
        ListOp::Replace(i, item) => {
            if let Some(slot) = list.get_mut(i) {
                *slot = item;
            }
        }
        ListOp::Remove(i) => {
            list.remove(i);
        }
        ListOp::Swap(i, j) => list.swap(i, j),
    }
}

impl<T: Clone + PartialEq + 'static> ListField<T> {
    #[cfg_attr(test, mutants::skip)]
    fn push(&self, item: T) {
        self.set.call(ListOp::Push(item));
    }
    #[cfg_attr(test, mutants::skip)]
    fn replace(&self, i: usize, item: T) {
        self.set.call(ListOp::Replace(i, item));
    }
    #[cfg_attr(test, mutants::skip)]
    fn remove(&self, i: usize) {
        self.set.call(ListOp::Remove(i));
    }
    #[cfg_attr(test, mutants::skip)]
    fn swap(&self, i: usize, j: usize) {
        self.set.call(ListOp::Swap(i, j));
    }
}

fn use_list_field<T: Clone + PartialEq + 'static>(
    ctx: PersonEditState,
    facet: FacetKind,
    base: fn(&Person) -> &Vec<T>,
    base_mut: fn(&mut Person) -> &mut Vec<T>,
    mask_bucket: fn(&PersonaMask) -> &Option<Vec<T>>,
    mask_bucket_mut: fn(&mut PersonaMask) -> &mut Option<Vec<T>>,
) -> ListField<T> {
    let draft = ctx.draft();
    let val = use_memo(move || {
        let p = draft.read();
        match mask_of(&p, facet) {
            Some(m) => mask_bucket(m).clone().unwrap_or_else(|| base(&p).clone()),
            None => base(&p).clone(),
        }
    });
    let set = EventHandler::new(move |op: ListOp<T>| {
        let mut binding = ctx.draft();
        let mut p = binding.write();
        let list: &mut Vec<T> = match mask_mut_of(&mut p, facet) {
            Some(m) => mask_bucket_mut(m).get_or_insert_with(Vec::new),
            None => base_mut(&mut p),
        };
        apply_list_op(list, op);
    });
    ListField { val, set }
}

/// A facet-resolved scalar field (OCEAN scores, reputation scores).
#[derive(Clone)]
struct MajorField<T: Clone + PartialEq + 'static> {
    val: Memo<T>,
    set: EventHandler<T>,
}

fn use_major_field<T: Clone + PartialEq + 'static>(
    ctx: PersonEditState,
    facet: FacetKind,
    base: fn(&Person) -> &T,
    base_mut: fn(&mut Person) -> &mut T,
    mask_bucket: fn(&PersonaMask) -> &Option<T>,
    mask_bucket_mut: fn(&mut PersonaMask) -> &mut Option<T>,
) -> MajorField<T> {
    let draft = ctx.draft();
    let val = use_memo(move || {
        let p = draft.read();
        match mask_of(&p, facet) {
            Some(m) => mask_bucket(m).clone().unwrap_or_else(|| base(&p).clone()),
            None => base(&p).clone(),
        }
    });
    let set = EventHandler::new(move |value: T| {
        let mut binding = ctx.draft();
        let mut p = binding.write();
        match mask_mut_of(&mut p, facet) {
            Some(m) => *mask_bucket_mut(m) = Some(value),
            None => *base_mut(&mut p) = value,
        }
    });
    MajorField { val, set }
}

/// Resilience/risk are `Option<u8>` internally but expose a plain `u8` that
/// defaults to 5 everywhere in the UI.
#[derive(Clone, Copy)]
struct ResilienceRiskField {
    resilience: Memo<u8>,
    risk_appetite: Memo<u8>,
    set_resilience: EventHandler<u8>,
    set_risk_appetite: EventHandler<u8>,
}

impl ResilienceRiskField {
    #[cfg_attr(test, mutants::skip)]
    fn resilience(&self) -> u8 {
        (self.resilience)()
    }
    #[cfg_attr(test, mutants::skip)]
    fn risk_appetite(&self) -> u8 {
        (self.risk_appetite)()
    }
}

fn use_resilience_risk(ctx: PersonEditState, facet: FacetKind) -> ResilienceRiskField {
    let draft = ctx.draft();
    let resilience = use_memo(move || {
        let p = draft.read();
        mask_of(&p, facet)
            .and_then(|m| m.resilience)
            .or(p.resilience)
            .unwrap_or(5)
    });
    let risk_appetite = use_memo(move || {
        let p = draft.read();
        mask_of(&p, facet)
            .and_then(|m| m.risk_appetite)
            .or(p.risk_appetite)
            .unwrap_or(5)
    });
    let set_resilience = EventHandler::new(move |v: u8| {
        let mut binding = ctx.draft();
        let mut p = binding.write();
        match mask_mut_of(&mut p, facet) {
            Some(m) => m.resilience = Some(v),
            None => p.resilience = Some(v),
        }
    });
    let set_risk_appetite = EventHandler::new(move |v: u8| {
        let mut binding = ctx.draft();
        let mut p = binding.write();
        match mask_mut_of(&mut p, facet) {
            Some(m) => m.risk_appetite = Some(v),
            None => p.risk_appetite = Some(v),
        }
    });
    ResilienceRiskField {
        resilience,
        risk_appetite,
        set_resilience,
        set_risk_appetite,
    }
}

#[component]
fn PersonEditForm(initial: Option<Person>) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let mut toast_sig = use_context::<Signal<Option<String>>>();
    let is_new = initial.is_none();
    let base = initial.unwrap_or_else(blank_person);
    let pers_id = base.id.clone();

    // Single draft: every field writes into this one Person. `saved` is a
    // memo derived from the prop (never from the live draft), so Discard
    // always restores the as-loaded state no matter how much has been typed.
    let draft = use_signal(|| base.clone());
    // The editor opens on the anchor context: the inline fields ARE that
    // context's data, so "main" is the first tab.
    let mode = use_signal(|| base.primary_facet);
    let open = use_signal(|| ALL_EDIT_SECTIONS.to_vec());
    let saved = use_memo(move || base.clone());
    let base_active = use_memo(move || draft.read().private_persona.is_some());
    let work_active = use_memo(move || draft.read().persona.is_some());
    let online_active = use_memo(move || draft.read().online_persona.is_some());

    let mut ctx = PersonEditState {
        draft,
        mode,
        open,
        saved,
        base_active,
        work_active,
        online_active,
    };
    use_context_provider(|| ctx);

    // Re-point the anchor context at another tab and jump straight to it. The
    // swap is draft-level, so Cancel/Discard restores the previous layout.
    let mut switch_primary = move |new: FacetKind| {
        ctx.draft.write().set_primary_facet(new);
        ctx.mode.set(new);
        ctx.open.set(ALL_EDIT_SECTIONS.to_vec());
    };
    // Derived label state: the tabs and the "main context" selector follow
    // the draft's anchor, but only when that anchor changes (never per
    // keystroke, so the whole form doesn't re-run on every edit).
    let primary = use_memo(move || ctx.draft().read().primary_facet);
    let main_context_label = crate::tr!(MainContextLabel, lang());

    let mut save = move || {
        let mut person = draft.read().clone();
        if is_new {
            person.created_at = chrono::Utc::now().timestamp_millis();
        }
        person.updated_at = chrono::Utc::now().timestamp_millis();
        if let Err(e) = db::save_person(&person) {
            toast_sig.set(Some(format!("{}: {e}", crate::tr!(ToastError, lang()))));
            return;
        }
        toast_sig.set(Some(crate::tr!(ToastSaved, lang()).into()));
        dioxus::prelude::navigator().push(Route::PersonDetail {
            id: pers_id.clone(),
        });
    };

    // Render counter, unconditionally instrumented: the E2E suite runs
    // against the release build, so this must be present there too. It
    // proves keystrokes never re-run the whole form (the counter only ticks
    // when the shell itself re-renders, which after mount is only on a
    // language switch).
    let render_count = use_hook(|| std::rc::Rc::new(std::cell::Cell::new(0u32)));
    let renders = next_render_count(render_count.get());
    render_count.set(renders);

    let form_new_title = crate::tr!(FormNewTitle, lang());
    let form_edit_title = crate::tr!(FormEditTitle, lang());
    let form_save = crate::tr!(FormSave, lang());
    let form_cancel = crate::tr!(FormCancel, lang());

    rsx! {
        div {
            class: "page page-edit",
            "data-edit-renders": renders,
            h2 { if is_new { "{form_new_title}" } else { "{form_edit_title}" } }
            div { class: "form",
                div { class: "facet-bar edit-mode-bar",
                    FacetToggle { facet: mode, primary: primary(), base_label: facet_tab_label(FacetKind::Base, primary(), lang()), work_label: facet_tab_label(FacetKind::Work, primary(), lang()), online_label: facet_tab_label(FacetKind::Online, primary(), lang()), group_label: Some(primary().label(crate::i18n::core_lang(lang())).to_string()) }
                }

                div { class: "main-context-bar",
                    span { class: "main-context-label", "{main_context_label}" }
                    for kind in FacetKind::ALL {
                        button {
                            class: if primary() == kind { "main-context-btn active" } else { "main-context-btn" },
                            role: "radio",
                            aria_checked: if primary() == kind { "true" } else { "false" },
                            onclick: move |_| switch_primary(kind),
                            "{kind.label(crate::i18n::core_lang(lang()))}"
                        }
                    }
                }

                div { class: "persona-actions-bar",
                    PersonaActions { facet: FacetKind::Base }
                    PersonaActions { facet: FacetKind::Work }
                    PersonaActions { facet: FacetKind::Online }
                }

                NameField {}
                RoleField {}
                ContextField {}
                EmojiField {}
                TagsField {}
                NotesField {}
                ConfidenceField {}

                FacetSections {}

                div { class: "edit-actions-bar form-actions",
                    button { class: "btn btn-primary", aria_label: "{form_save}", onclick: move |_| save(), "{form_save}" }
                    Link { to: Route::PeopleList {}, class: "btn", aria_label: "{form_cancel}", "{form_cancel}" }
                }
            }
        }
    }
}

/// One Base/Work/Online facet toggle row: always rendered in every facet so
/// the edit-mode bars keep identical heights; hidden (visibility, not
/// presence) when not the active facet or when the row is the anchor context
/// (the anchor never wears a persona mask).
#[component]
fn PersonaActions(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let hidden = use_memo(move || {
        let primary = ctx.draft().read().primary_facet;
        persona_row_hidden(primary, ctx.mode(), facet)
    });
    let enabled = use_memo(move || ctx.persona_active(facet));
    let section_label = persona_mask_label(facet, lang());
    let copy_label = crate::tr!(PersonaCopyBase, lang());
    let clear_label = crate::tr!(PersonaClear, lang());
    rsx! {
        div {
            class: "persona-actions",
            class: if hidden() { "persona-actions-hidden" },
            aria_hidden: if hidden() { "true" },
            label { class: "dim-toggle",
                input { r#type: "checkbox",
                    checked: enabled(),
                    oninput: move |e| ctx.set_persona_enabled(facet, e.value() == "true")
                }
                if enabled() { "✓ " } else { "✗ " }
                "{section_label}"
            }
            button { class: "btn btn-small", onclick: move |_| ctx.copy_base(facet), "{copy_label}" }
            button { class: "btn btn-small", onclick: move |_| ctx.clear_persona(facet), "{clear_label}" }
        }
    }
}

/// Renders the 8 edit sections for whichever facet is active (persona
/// sections only when that persona is enabled), all scoped children so a
/// keystroke inside one section never rerenders the others.
#[component]
fn FacetSections() -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let mode = use_memo(move || ctx.mode());
    let primary = use_memo(move || ctx.draft().read().primary_facet);
    let base_enabled = use_memo(move || ctx.base_active());
    let work_enabled = use_memo(move || ctx.work_active());
    let online_enabled = use_memo(move || ctx.online_active());

    let resilience_work = use_memo(move || {
        persona_panel_active(
            ctx.bucket_defined(FacetKind::Work, FacetBucket::Resilience),
            ctx.bucket_defined(FacetKind::Work, FacetBucket::RiskAppetite),
        )
    });
    let resilience_online = use_memo(move || {
        persona_panel_active(
            ctx.bucket_defined(FacetKind::Online, FacetBucket::Resilience),
            ctx.bucket_defined(FacetKind::Online, FacetBucket::RiskAppetite),
        )
    });
    let resilience_base = use_memo(move || {
        persona_panel_active(
            ctx.bucket_defined(FacetKind::Base, FacetBucket::Resilience),
            ctx.bucket_defined(FacetKind::Base, FacetBucket::RiskAppetite),
        )
    });

    let cf = peoplemodeler_core::model_config::CFG.completeness;
    let mot_cap = cf.motivation_cap;
    let bias_cap = cf.bias_cap;
    let pattern_cap = cf.pattern_cap;
    let style_cap = cf.style_cap;
    let values_cap = cf.values_cap;

    let persona_balance = crate::tr!(PersonaBalanceTitle, lang());
    let form_ocean_title = crate::tr!(FormOceanTitle, lang());
    let edit_motivations = crate::tr!(EditMotivations, lang());
    let edit_biases = crate::tr!(EditBiases, lang());
    let edit_reputation = crate::tr!(EditReputation, lang());
    let edit_patterns = crate::tr!(EditPatterns, lang());
    let edit_styles = crate::tr!(EditStyles, lang());
    let edit_values = crate::tr!(EditValues, lang());

    rsx! {
        if mode() == primary() {
            // Anchor context: the inline fields are this context's data, so
            // the sections render bare (no override toggles). `facet` is the
            // live mode so every scoped child reads the anchor inline values.
            FacetSection {
                facet: mode(),
                open: ctx.open,
                id: EditSectionId::ResilienceRisk,
                title: persona_balance,
                ResilienceRiskInputs { facet: mode() }
            }
            FacetSection {
                facet: mode(),
                open: ctx.open,
                id: EditSectionId::Ocean,
                title: form_ocean_title,
                header: Some(rsx! { OceanWarningBadge {} }),
                progress: Some(rsx! { OceanProgress { facet: mode() } }),
                OceanInputs { facet: mode() }
            }
            FacetSection {
                facet: mode(),
                open: ctx.open,
                id: EditSectionId::Motivations,
                title: edit_motivations,
                progress: Some(rsx! { ListProgress { facet: mode(), bucket: FacetBucket::Motivations, cap: mot_cap } }),
                MotEditPanel { facet: mode() }
            }
            FacetSection {
                facet: mode(),
                open: ctx.open,
                id: EditSectionId::Biases,
                title: edit_biases,
                progress: Some(rsx! { ListProgress { facet: mode(), bucket: FacetBucket::Biases, cap: bias_cap } }),
                BiasEditPanel { facet: mode() }
            }
            FacetSection {
                facet: mode(),
                open: ctx.open,
                id: EditSectionId::Reputation,
                title: edit_reputation,
                progress: Some(rsx! { RepProgress { facet: mode() } }),
                RepEditPanel { facet: mode() }
            }
            FacetSection {
                facet: mode(),
                open: ctx.open,
                id: EditSectionId::Patterns,
                title: edit_patterns,
                progress: Some(rsx! { ListProgress { facet: mode(), bucket: FacetBucket::Patterns, cap: pattern_cap } }),
                PatternEditPanel { facet: mode() }
            }
            FacetSection {
                facet: mode(),
                open: ctx.open,
                id: EditSectionId::Styles,
                title: edit_styles,
                progress: Some(rsx! { ListProgress { facet: mode(), bucket: FacetBucket::Styles, cap: style_cap } }),
                StyleEditPanel { facet: mode() }
            }
            FacetSection {
                facet: mode(),
                open: ctx.open,
                id: EditSectionId::Values,
                title: edit_values,
                progress: Some(rsx! { ListProgress { facet: mode(), bucket: FacetBucket::Values, cap: values_cap } }),
                ValEditPanel { facet: mode() }
            }
        }

        // Non-anchor contexts render as persona panels whose buckets are
        // explicit overrides over the anchor profile, shown only when the
        // mask exists (the "enable" row is in PersonaActions) and only while
        // that facet is the active tab.
        for (kind, enabled, resilience) in [
            (FacetKind::Base, base_enabled, resilience_base),
            (FacetKind::Work, work_enabled, resilience_work),
            (FacetKind::Online, online_enabled, resilience_online),
        ] {
            if mode() == kind && mode() != primary() && enabled() {
                FacetSection {
                    facet: kind,
                    open: ctx.open,
                    id: EditSectionId::ResilienceRisk,
                    title: persona_balance,
                    active: Some(resilience),
                    header: Some(rsx! {
                        BucketToggle { facet: kind, bucket: FacetBucket::Resilience }
                        BucketToggle { facet: kind, bucket: FacetBucket::RiskAppetite }
                    }),
                    ResilienceRiskInputs { facet: kind }
                }
                FacetSection {
                    facet: kind,
                    open: ctx.open,
                    id: EditSectionId::Ocean,
                    title: form_ocean_title,
                    bucket: FacetBucket::Ocean,
                    progress: Some(rsx! { OceanProgress { facet: kind } }),
                    OceanInputs { facet: kind }
                }
                FacetSection {
                    facet: kind,
                    open: ctx.open,
                    id: EditSectionId::Motivations,
                    title: edit_motivations,
                    bucket: FacetBucket::Motivations,
                    progress: Some(rsx! { ListProgress { facet: kind, bucket: FacetBucket::Motivations, cap: mot_cap } }),
                    MotEditPanel { facet: kind }
                }
                FacetSection {
                    facet: kind,
                    open: ctx.open,
                    id: EditSectionId::Biases,
                    title: edit_biases,
                    bucket: FacetBucket::Biases,
                    progress: Some(rsx! { ListProgress { facet: kind, bucket: FacetBucket::Biases, cap: bias_cap } }),
                    BiasEditPanel { facet: kind }
                }
                FacetSection {
                    facet: kind,
                    open: ctx.open,
                    id: EditSectionId::Reputation,
                    title: edit_reputation,
                    bucket: FacetBucket::Reputation,
                    progress: Some(rsx! { RepProgress { facet: kind } }),
                    RepEditPanel { facet: kind }
                }
                FacetSection {
                    facet: kind,
                    open: ctx.open,
                    id: EditSectionId::Patterns,
                    title: edit_patterns,
                    bucket: FacetBucket::Patterns,
                    progress: Some(rsx! { ListProgress { facet: kind, bucket: FacetBucket::Patterns, cap: pattern_cap } }),
                    PatternEditPanel { facet: kind }
                }
                FacetSection {
                    facet: kind,
                    open: ctx.open,
                    id: EditSectionId::Styles,
                    title: edit_styles,
                    bucket: FacetBucket::Styles,
                    progress: Some(rsx! { ListProgress { facet: kind, bucket: FacetBucket::Styles, cap: style_cap } }),
                    StyleEditPanel { facet: kind }
                }
                FacetSection {
                    facet: kind,
                    open: ctx.open,
                    id: EditSectionId::Values,
                    title: edit_values,
                    bucket: FacetBucket::Values,
                    progress: Some(rsx! { ListProgress { facet: kind, bucket: FacetBucket::Values, cap: values_cap } }),
                    ValEditPanel { facet: kind }
                }
            }
        }
    }
}

/// Section chrome shared by every facet: base facets get the bare section,
/// persona facets get the override bucket toggle header + the readonly
/// `persona-panel` body wrapper, all behind the same predicates so the two
/// sides can't drift.
#[component]
fn FacetSection(
    facet: FacetKind,
    open: Signal<Vec<EditSectionId>>,
    id: EditSectionId,
    title: &'static str,
    #[props(default)] bucket: Option<FacetBucket>,
    #[props(default)] active: Option<Memo<bool>>,
    #[props(default)] header: Option<Element>,
    #[props(default)] progress: Option<Element>,
    children: Element,
) -> Element {
    let ctx = use_context::<PersonEditState>();
    let on_discard = EventHandler::new(move |_| ctx.discard(id));
    let primary = ctx.draft().read().primary_facet;
    let is_persona = is_persona_facet(facet, primary);
    let panel_active = use_memo(move || {
        let active_memo = active.map(|a| a());
        let bucket_defined = bucket.map(|b| ctx.bucket_defined(facet, b));
        section_panel_active(active_memo, bucket_defined)
    });
    let header = if is_persona {
        match bucket {
            Some(b) => header.or_else(|| Some(rsx! { BucketToggle { facet, bucket: b } })),
            None => header,
        }
    } else {
        header
    };
    let children = if is_persona {
        let class = if panel_active() {
            "persona-panel"
        } else {
            "persona-panel readonly"
        };
        rsx! { div { class, {children} } }
    } else {
        children
    };
    rsx! {
        EditSection { open, id, title, header, progress, on_discard: Some(on_discard), {children} }
    }
}

#[component]
fn EditSection(
    open: Signal<Vec<EditSectionId>>,
    id: EditSectionId,
    title: &'static str,
    header: Option<Element>,
    #[props(default)] progress: Option<Element>,
    on_discard: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let is_open = open().contains(&id);
    let lang = use_context::<Signal<Lang>>();
    rsx! {
        div {
            class: "edit-section",
            class: if is_open { "open" },
            div { class: "edit-section-header",
                button {
                    class: "edit-section-toggle",
                    aria_label: "{title}",
                    aria_expanded: is_open,
                    onclick: move |_| open.set(toggle_section(open(), id)),
                    span { class: "chevron", if is_open { "▾" } else { "▸" } }
                    span { "{title}" }
                }
                // How much of this section is filled in — visible even
                // while collapsed, so a person can tell at a glance which
                // sections still need attention without opening each one.
                // Independent of `header` (the override checkbox / warning
                // badge) so the two never have to fight over one slot.
                if let Some(p) = progress {
                    div { class: "edit-section-progress", { p } }
                }
                if let Some(h) = header {
                    div { class: "edit-section-badge", { h } }
                }
                if let Some(d) = on_discard {
                    button {
                        class: "btn btn-small edit-section-discard",
                        r#type: "button",
                        aria_label: "{crate::tr!(AriaDiscardPrefix, lang())} {title}",
                        onclick: move |_| d.call(()),
                        "↺"
                    }
                }
            }
            // Kept mounted (just visually collapsed) rather than unmounted
            // when closed, so section content stays in the DOM for
            // scripts/tests/accessibility tools that query it directly.
            div { class: if is_open { "edit-section-body" } else { "edit-section-body collapsed" }, { children } }
        }
    }
}

#[component]
fn BucketToggle(facet: FacetKind, bucket: FacetBucket) -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let has = use_memo(move || ctx.bucket_defined(facet, bucket));
    rsx! {
        label { class: "dim-toggle bucket-toggle",
            input { r#type: "checkbox",
                checked: has(),
                oninput: move |e| ctx.set_bucket_defined(facet, bucket, e.value() == "true")
            }
            if has() { "{crate::tr!(BucketOverride, lang())}" } else { "{crate::tr!(BucketInheritsBase, lang())}" }
        }
    }
}

/// Conditional "⚠ N" consistency badge for the base OCEAN section. Empty when
/// there are no flagged gaps (a zero-size element, matching the pre-badge
/// layout).
#[component]
fn OceanWarningBadge() -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let flags = use_memo(move || {
        let p = ctx.draft.read();
        let mut flags = peoplemodeler_core::validation::ocean_rep_flags(&p.ocean, &p.rep_scores);
        flags.extend(peoplemodeler_core::validation::rhetoric_gap_flags(
            &p.ocean,
            &p.rep_scores,
            &p.motivations,
        ));
        if peoplemodeler_core::validation::pattern_calm_volatile_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_calm_volatile");
        }
        if peoplemodeler_core::validation::pattern_honest_exploiter_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_honest_exploiter");
        }
        if peoplemodeler_core::validation::bias_confirmation_open_gap(&p.biases, &p.ocean) {
            flags.push("flag_bias_confirmation_open");
        }
        if peoplemodeler_core::validation::bias_favoritism_fairness_gap(&p.biases, &p.motivations) {
            flags.push("flag_bias_favoritism_fairness");
        }
        if peoplemodeler_core::validation::authority_dominant_gap(&p.biases, &p.rep_scores) {
            flags.push("flag_authority_dominant");
        }
        if peoplemodeler_core::validation::social_proof_open_gap(&p.biases, &p.ocean) {
            flags.push("flag_social_proof_open");
        }
        if peoplemodeler_core::validation::sunk_cost_flexible_gap(&p.biases, &p.rep_scores) {
            flags.push("flag_sunk_cost_flexible");
        }
        if peoplemodeler_core::validation::loss_aversion_risky_gap(&p.biases, p.risk_appetite) {
            flags.push("flag_loss_aversion_risky");
        }
        if peoplemodeler_core::validation::dunning_kruger_humble_gap(&p.biases, &p.rep_scores) {
            flags.push("flag_dunning_kruger_humble");
        }
        if peoplemodeler_core::validation::impostor_arrogant_gap(&p.biases, &p.rep_scores) {
            flags.push("flag_impostor_arrogant");
        }
        if peoplemodeler_core::validation::recency_reliable_gap(&p.biases, &p.rep_scores) {
            flags.push("flag_recency_reliable");
        }
        if peoplemodeler_core::validation::pattern_diplomat_escalator_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_diplomat_escalator");
        }
        if peoplemodeler_core::validation::pattern_fair_exploiter_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_fair_exploiter");
        }
        if peoplemodeler_core::validation::pattern_humble_dismissive_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_humble_dismissive");
        }
        if peoplemodeler_core::validation::pattern_trusting_paranoid_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_trusting_paranoid");
        }
        if peoplemodeler_core::validation::pattern_reliable_shirker_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_reliable_shirker");
        }
        if peoplemodeler_core::validation::pattern_hardworker_complacent_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_hardworker_complacent");
        }
        if peoplemodeler_core::validation::pattern_passive_blowup_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_passive_blowup");
        }
        if peoplemodeler_core::validation::pattern_assertive_quiet_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_assertive_quiet");
        }
        if peoplemodeler_core::validation::security_risky_gap(&p.motivations, p.risk_appetite) {
            flags.push("flag_security_risky");
        }
        if peoplemodeler_core::validation::resilient_reactive_gap(p.resilience, &p.rep_scores) {
            flags.push("flag_resilient_reactive");
        }
        if peoplemodeler_core::validation::risk_appetite_ambition_gap(
            &p.motivations,
            p.risk_appetite,
        ) {
            flags.push("flag_risk_appetite_ambition");
        }
        if peoplemodeler_core::validation::resilient_hides_gap(p.resilience, &p.rep_scores) {
            flags.push("flag_resilient_hides");
        }
        if peoplemodeler_core::validation::pattern_generous_exploiter_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_generous_exploiter");
        }
        if peoplemodeler_core::validation::pattern_empath_dismissive_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_empath_dismissive");
        }
        if peoplemodeler_core::validation::pattern_flexible_resister_gap(
            &p.behavioral_patterns,
            &p.rep_scores,
        ) {
            flags.push("flag_pattern_flexible_resister");
        }
        if peoplemodeler_core::validation::anchoring_open_gap(&p.biases, &p.ocean) {
            flags.push("flag_anchoring_open");
        }
        if peoplemodeler_core::validation::pattern_helping_exploiter_gap(
            &p.behavioral_patterns,
            &p.motivations,
        ) {
            flags.push("flag_pattern_helping_exploiter");
        }
        if peoplemodeler_core::validation::pattern_warmth_dismissive_gap(
            &p.behavioral_patterns,
            &p.ocean,
        ) {
            flags.push("flag_pattern_warmth_dismissive");
        }
        if peoplemodeler_core::validation::pattern_discipline_shirker_gap(
            &p.behavioral_patterns,
            &p.ocean,
        ) {
            flags.push("flag_pattern_discipline_shirker");
        }
        if peoplemodeler_core::validation::pattern_claimed_calm_volatile_gap(
            &p.behavioral_patterns,
            &p.ocean,
        ) {
            flags.push("flag_pattern_claimed_calm_volatile");
        }
        if peoplemodeler_core::validation::pattern_fairness_exploiter_gap(
            &p.behavioral_patterns,
            &p.motivations,
        ) {
            flags.push("flag_pattern_fairness_exploiter");
        }
        if peoplemodeler_core::validation::pattern_achievement_complacent_gap(
            &p.behavioral_patterns,
            &p.motivations,
        ) {
            flags.push("flag_pattern_achievement_complacent");
        }
        if peoplemodeler_core::validation::pattern_learning_resister_gap(
            &p.behavioral_patterns,
            &p.motivations,
        ) {
            flags.push("flag_pattern_learning_resister");
        }
        if peoplemodeler_core::validation::pattern_extravert_quiet_gap(
            &p.behavioral_patterns,
            &p.ocean,
        ) {
            flags.push("flag_pattern_extravert_quiet");
        }
        if peoplemodeler_core::validation::pattern_open_resister_gap(
            &p.behavioral_patterns,
            &p.ocean,
        ) {
            flags.push("flag_pattern_open_resister");
        }
        if peoplemodeler_core::validation::pattern_recognition_dismissive_gap(
            &p.behavioral_patterns,
            &p.motivations,
        ) {
            flags.push("flag_pattern_recognition_dismissive");
        }
        if peoplemodeler_core::validation::availability_calm_gap(&p.biases, &p.rep_scores) {
            flags.push("flag_availability_calm");
        }
        flags.extend(peoplemodeler_core::validation::style_gap_flags(
            &p.styles,
            &p.rep_scores,
        ));
        flags
    });
    let flags_now = flags();
    if flags_now.is_empty() {
        return rsx! {};
    }
    let tooltip = flags_now
        .iter()
        .map(|k| crate::i18n::tr_str(k, lang()))
        .collect::<Vec<_>>()
        .join("\n");
    rsx! {
        span {
            class: "warning-badge",
            title: "{tooltip}",
            "⚠ {flags_now.len()}"
        }
    }
}

#[component]
fn OceanProgress(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let count = use_memo(move || {
        let p = ctx.draft.read();
        match mask_of(&p, facet) {
            Some(m) => {
                let o = m.ocean.clone().unwrap_or_else(|| p.ocean.clone());
                ocean_filled_count(&o)
            }
            None => ocean_filled_count(&p.ocean),
        }
    });
    progress_badge(count(), Some(5))
}

#[component]
fn RepProgress(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let count = use_memo(move || {
        let p = ctx.draft.read();
        match mask_of(&p, facet) {
            Some(m) => {
                let r = m.rep_scores.clone().unwrap_or_else(|| p.rep_scores.clone());
                rep_filled_count(&r)
            }
            None => rep_filled_count(&p.rep_scores),
        }
    });
    progress_badge(count(), Some(13))
}

#[component]
fn ListProgress(facet: FacetKind, bucket: FacetBucket, cap: usize) -> Element {
    let ctx = use_context::<PersonEditState>();
    let count = use_memo(move || {
        let p = ctx.draft.read();
        list_len(&p, facet, bucket)
    });
    progress_badge(count(), Some(cap))
}

/// Small "how much of this is filled in" badge for a section header —
/// visible even while the section is collapsed. `total: None` means an
/// open-ended list (just show the count); `Some(n)` means a fixed set of
/// slots (OCEAN's 5 traits, Reputation's 13 dimensions) and renders as
/// "filled/total".
fn progress_badge(filled: usize, total: Option<usize>) -> Element {
    let text = match total {
        Some(total) => format!("{filled}/{total}"),
        None => filled.to_string(),
    };
    let complete = progress_complete(filled, total);
    rsx! {
        span {
            class: if complete { "progress-badge complete" } else { "progress-badge" },
            "{text}"
        }
    }
}

fn ocean_filled_count(o: &OceanScores) -> usize {
    [
        o.openness,
        o.conscientiousness,
        o.extraversion,
        o.agreeableness,
        o.neuroticism,
    ]
    .iter()
    .filter(|v| v.is_some())
    .count()
}

fn rep_filled_count(r: &RepScores) -> usize {
    RepDim::ALL
        .iter()
        .filter(|d| r.score(**d).is_some())
        .count()
}

fn mot_helper(t: &MotivationType, lang: Lang) -> &'static str {
    t.i18n(core_lang(lang)).desc
}

fn bias_helper(t: &BiasType, lang: Lang) -> &'static str {
    t.i18n(core_lang(lang)).desc
}

fn style_helper(t: &StyleType, lang: Lang) -> &'static str {
    t.i18n_desc(core_lang(lang))
}

fn value_helper(t: &ValueType, lang: Lang) -> &'static str {
    t.i18n(core_lang(lang)).desc
}

fn pattern_helper(t: &BehaviorTrigger, lang: Lang) -> &'static str {
    match t {
        BehaviorTrigger::Stress => crate::tr!(PatternHelperStress, lang),
        BehaviorTrigger::Conflict => crate::tr!(PatternHelperConflict, lang),
        BehaviorTrigger::Success => crate::tr!(PatternHelperSuccess, lang),
        BehaviorTrigger::Uncertainty => crate::tr!(PatternHelperUncertainty, lang),
        BehaviorTrigger::Recognition => crate::tr!(PatternHelperRecognition, lang),
        BehaviorTrigger::Threatened => crate::tr!(PatternHelperThreat, lang),
        BehaviorTrigger::Change => crate::tr!(PatternHelperChange, lang),
        BehaviorTrigger::Feedback => crate::tr!(PatternHelperFeedback, lang),
        BehaviorTrigger::Injustice => crate::tr!(PatternHelperInjustice, lang),
    }
}

fn behavior_helper(t: &BehaviorResponse, lang: Lang) -> &'static str {
    t.desc(core_lang(lang))
}

fn persona_panel_active(a: bool, b: bool) -> bool {
    a || b
}

// Extracted so it's directly unit-testable: the discard closure that uses
// this lives inside a Dioxus component and needs a live render context, so
// cargo-mutants had no test able to exercise this comparison in isolation
/// Any non-anchor context is rendered as a persona panel whose buckets are
/// explicit overrides over the anchor profile (the anchor is the only "base").
fn is_persona_facet(facet: FacetKind, primary: FacetKind) -> bool {
    facet != primary
}

fn parse_tags(s: &str) -> Vec<Tag> {
    s.split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .map(|name| Tag { name, color: None })
        .collect()
}

/// Shared "add a new X, then browse/edit/reorder/delete existing Xs" list
/// section used by Motivations, Biases, Values, Patterns and Styles — the
/// five panels that manage a user-editable `Vec<T>` with reorder buttons.
/// (Reputation isn't included: its editor is a fixed set of sliders, not a
/// user-managed list of items, so it's a genuinely different shape and
/// keeps its own markup.)
///
/// This is a plain generic function, not a `#[component]`: it's called as
/// `{list_edit_section(...)}` inside another component's `rsx!`, not as an
/// `rsx!` tag, which sidesteps Dioxus's Props-derive machinery (and the
/// Clone/PartialEq bounds that come with it) entirely — the closures here
/// are just ordinary `Fn`/`FnMut`, nothing fancier.
///
/// `move_up_label`/`move_down_label`/`edit_label`/`delete_label` must
/// resolve, in English, to exactly the pre-existing aria-label text (e.g.
/// "Move motivation up", "Delete bias") — several Playwright tests locate
/// these buttons by that exact text — so callers pass fully-translated
/// phrases (via `crate::tr!`) rather than this function templating a raw
/// noun onto a suffix itself: the equivalent French phrases don't inflect
/// uniformly (grammatical gender differs per noun — "un biais" vs "une
/// motivation"), so a single template can't produce correct French for
/// all five callers.
///
/// The caller owns `edit_idx` and is expected to react to it (typically
/// via `use_effect`) to populate its own add-row fields when the user
/// clicks a row's edit button — this function only ever sets it to
/// `Some(i)`, it never reads item fields itself, which is what keeps it
/// generic over every item type without needing per-type closures wired
/// through every row.
#[allow(clippy::too_many_arguments)]
fn list_edit_section<T: Clone + PartialEq + 'static>(
    items: ListField<T>,
    mut edit_idx: Signal<Option<usize>>,
    move_up_label: &str,
    move_down_label: &str,
    edit_label: &str,
    delete_label: &str,
    legend_text: &str,
    add_row: Element,
    on_delete: impl FnMut() + 'static,
    render_row: impl Fn(usize, &T) -> Element,
) -> Element {
    let on_delete = std::rc::Rc::new(std::cell::RefCell::new(on_delete));
    let len = items.val.read().len();
    let rows = items
        .val
        .read()
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let row_on_delete = on_delete.clone();
            rsx! {
                div { class: "list-item",
                    button {
                        class: "reorder-btn",
                        aria_label: "{move_up_label}",
                        onclick: {
                            let row_items = items.clone();
                            move |_| {
                                if i > 0 {
                                    row_items.swap(i, i - 1);
                                }
                            }
                        },
                        "▲"
                    }
                    button {
                        class: "reorder-btn",
                        aria_label: "{move_down_label}",
                        onclick: {
                            let row_items = items.clone();
                            move |_| {
                                if i + 1 < len {
                                    row_items.swap(i, i + 1);
                                }
                            }
                        },
                        "▼"
                    }
                    button {
                        class: "btn btn-small",
                        aria_label: "{edit_label}",
                        onclick: move |_| { edit_idx.set(Some(i)); },
                        "✏"
                    }
                    {render_row(i, item)}
                    button {
                        class: "btn btn-small",
                        aria_label: "{delete_label}",
                        onclick: {
                            let row_items = items.clone();
                            move |_| { row_items.remove(i); (row_on_delete.borrow_mut())(); }
                        },
                        "✕"
                    }
                }
            }
        })
        .filter_map(|n| n.ok())
        .collect::<Vec<_>>();
    rsx! {
        fieldset { class: "section",
            legend { class: "sr-only", "{legend_text}" }
            {add_row}
            div { class: "section-items",
                {rows.into_iter()}
            }
        }
    }
}

#[component]
fn ResilienceRiskInputs(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let field = use_resilience_risk(ctx, facet);
    let form_resilience = crate::tr!(FormResilience, lang());
    let form_risk_appetite = crate::tr!(FormRiskAppetite, lang());
    rsx! {
        fieldset { class: "persona-balance",
            legend { class: "sr-only", "{crate::tr!(PersonaBalanceTitle, lang())}" }
            label { "{form_resilience}" }
            div { class: "ocean-slider",
                StepperSlider {
                    min: 1, max: 10, value: field.resilience(), display: format!("{}/10", field.resilience()),
                    onchange: move |v| field.set_resilience.call(v),
                }
            }
            label { "{form_risk_appetite}" }
            div { class: "ocean-slider",
                StepperSlider {
                    min: 1, max: 10, value: field.risk_appetite(), display: format!("{}/10", field.risk_appetite()),
                    onchange: move |v| field.set_risk_appetite.call(v),
                }
            }
        }
    }
}

#[component]
fn OceanInputs(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let field = use_major_field(
        ctx,
        facet,
        |p| &p.ocean,
        |p| &mut p.ocean,
        |m| &m.ocean,
        |m| &mut m.ocean,
    );
    rsx! {
        fieldset { class: "ocean-inputs",
            legend { class: "sr-only", "{crate::tr!(FormOceanTitle, lang())}" }
            OceanSlider {
                label: crate::tr!(OceanOpenness, lang()),
                val: field.val.read().openness,
                onchange: move |v| {
                    let mut o = field.val.read().clone();
                    o.openness = v;
                    field.set.call(o);
                },
                low_hint: Some(crate::tr!(OceanOLow, lang()).into()),
                high_hint: Some(crate::tr!(OceanOHigh, lang()).into()),
            }
            OceanSlider {
                label: crate::tr!(OceanConscientiousness, lang()),
                val: field.val.read().conscientiousness,
                onchange: move |v| {
                    let mut o = field.val.read().clone();
                    o.conscientiousness = v;
                    field.set.call(o);
                },
                low_hint: Some(crate::tr!(OceanCLow, lang()).into()),
                high_hint: Some(crate::tr!(OceanCHigh, lang()).into()),
            }
            OceanSlider {
                label: crate::tr!(OceanExtraversion, lang()),
                val: field.val.read().extraversion,
                onchange: move |v| {
                    let mut o = field.val.read().clone();
                    o.extraversion = v;
                    field.set.call(o);
                },
                low_hint: Some(crate::tr!(OceanELow, lang()).into()),
                high_hint: Some(crate::tr!(OceanEHigh, lang()).into()),
            }
            OceanSlider {
                label: crate::tr!(OceanAgreeableness, lang()),
                val: field.val.read().agreeableness,
                onchange: move |v| {
                    let mut o = field.val.read().clone();
                    o.agreeableness = v;
                    field.set.call(o);
                },
                low_hint: Some(crate::tr!(OceanALow, lang()).into()),
                high_hint: Some(crate::tr!(OceanAHigh, lang()).into()),
            }
            OceanSlider {
                label: crate::tr!(OceanNeuroticism, lang()),
                val: field.val.read().neuroticism,
                onchange: move |v| {
                    let mut o = field.val.read().clone();
                    o.neuroticism = v;
                    field.set.call(o);
                },
                low_hint: Some(crate::tr!(OceanNLow, lang()).into()),
                high_hint: Some(crate::tr!(OceanNHigh, lang()).into()),
            }
        }
    }
}

#[component]
fn RepEditPanel(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let edit_rep = crate::tr!(EditReputation, lang());
    let rep_undefined_warning = crate::tr!(RepUndefinedWarning, lang());
    let rep_scale_hint = crate::tr!(RepScaleHint, lang());
    let field = use_major_field(
        ctx,
        facet,
        |p| &p.rep_scores,
        |p| &mut p.rep_scores,
        |m| &m.rep_scores,
        |m| &mut m.rep_scores,
    );

    let rep_data: Vec<_> = RepDim::ALL
        .iter()
        .map(|dim| {
            let ri = dim.i18n(cl);
            (*dim, ri)
        })
        .collect();

    rsx! {
        fieldset { class: "ocean-inputs",
            legend { class: "sr-only", "{edit_rep}" }
            div { class: "helper-text", "{rep_undefined_warning}" }
            div { class: "helper-text", "{rep_scale_hint}" }
            div { class: "section-items",
                {rep_data.into_iter().map(|(dim, ri)| {
                    let cur = field.val.read().score(dim);
                    rsx! {
                        RepDimSlider {
                            dim,
                            label_a: ri.label_a,
                            label_b: ri.label_b,
                            desc: ri.desc,
                            val: cur,
                            onchange: move |new| {
                                let mut s = field.val.read().clone();
                                set_rep_dim(&mut s, dim, new);
                                field.set.call(s);
                            },
                        }
                    }
                })}
            }
        }
    }
}

fn set_rep_dim(r: &mut RepScores, dim: RepDim, new: Option<u8>) {
    match dim {
        RepDim::HardworkerLazy => r.hardworker_lazy = new,
        RepDim::AuthoritativeSubmissive => r.authoritative_submissive = new,
        RepDim::HonestDeceitful => r.honest_deceitful = new,
        RepDim::ReliableFlaky => r.reliable_flaky = new,
        RepDim::HumbleArrogant => r.humble_arrogant = new,
        RepDim::CalmReactive => r.calm_reactive = new,
        RepDim::DiplomaticBlunt => r.diplomatic_blunt = new,
        RepDim::GenerousSelfish => r.generous_selfish = new,
        RepDim::FairFavoritism => r.fair_favoritism = new,
        RepDim::TrustingSuspicious => r.trusting_suspicious = new,
        RepDim::AssertivePassive => r.assertive_passive = new,
        RepDim::EmpatheticDetached => r.empathetic_detached = new,
        RepDim::AdaptableRigid => r.adaptable_rigid = new,
    }
}

/// A reputation slider, fully controlled: its on/off state and value are
/// props derived from the facet-resolved RepScores, and every interaction
/// reports back through `onchange`. No local slider state survives an
/// external reset (discard/copy-from-base), fixing the "underrun out from
/// under it" remount hack (`rep_reset_gen`) for good.
#[component]
fn RepDimSlider(
    dim: RepDim,
    label_a: &'static str,
    label_b: &'static str,
    desc: &'static str,
    val: Option<u8>,
    onchange: EventHandler<Option<u8>>,
) -> Element {
    let on = val.is_some();
    let shown = val.unwrap_or(5);
    rsx! {
        div { class: "ocean-slider",
            div { class: "ocean-header",
                span { class: "ocean-label",
                    "{dim.emoji()} {label_b} ← → {label_a}"
                }
                label { class: "dim-toggle",
                    input { r#type: "checkbox",
                        checked: on,
                        oninput: move |e| {
                            let new = e.value() == "true";
                            onchange.call(if new { Some(shown) } else { None });
                        }
                    }
                    if on { "✓" } else { "✗" }
                }
            }
            if on {
                div { class: "rep-slider-bar",
                    span { class: "rep-pole-b", "{label_b}" }
                    input { r#type: "range", min: "0", max: "10", value: "{shown}",
                        oninput: move |e| {
                            let v = e.value().parse().unwrap_or(5);
                            onchange.call(Some(v));
                        }
                    }
                    span { class: "rep-pole-a", "{label_a}" }
                }
                div { class: "rep-dim-value",
                    strong { "{shown}/10" }
                    span { " — {desc}" }
                }
            }
        }
    }
}

#[component]
fn StepperSlider(
    min: u8,
    max: u8,
    value: u8,
    display: String,
    onchange: EventHandler<u8>,
) -> Element {
    rsx! {
        div { class: "stepper-slider",
            button {
                class: "step-btn step-minus",
                aria_label: "-1",
                onclick: move |_| onchange.call(value.saturating_sub(1).max(min)),
                "−"
            }
            span { class: "step-val", "{display}" }
            button {
                class: "step-btn step-plus",
                aria_label: "+1",
                onclick: move |_| onchange.call((value + 1).min(max)),
                "+"
            }
        }
    }
}

#[component]
fn OceanSlider(
    label: String,
    val: Option<u8>,
    onchange: EventHandler<Option<u8>>,
    low_hint: Option<String>,
    high_hint: Option<String>,
) -> Element {
    let current = val.unwrap_or(5);
    rsx! {
        div { class: "ocean-slider",
            label { "{label}" }
            StepperSlider {
                min: 1,
                max: 10,
                value: current,
                display: if val.is_some() { format!("{current}/10") } else { "—".to_string() },
                // `v` is StepperSlider's own +1/-1 computation, already
                // based on `current` (which defaults to 5 when val is
                // None) — so the first click away from "unset" should just
                // forward that computed value, the same as any other
                // click. The previous special case here discarded that
                // computation and hard-coded Some(5) instead, silently
                // dropping the first increment/decrement whenever a field
                // went from unset to set (e.g. 3 clicks of "+" from unset
                // only ever reached +2, one short of the target).
                onchange: move |v| {
                    onchange.call(Some(v));
                },
            }
            if let (Some(l), Some(h)) = (low_hint.as_ref(), high_hint.as_ref()) {
                div { class: "ocean-hint",
                    span { class: "hint-low", "↓ {l}" }
                    span { class: "hint-sep", "|" }
                    span { class: "hint-high", "↑ {h}" }
                }
            }
        }
    }
}

#[component]
fn NameField() -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let mut draft = ctx.draft();
    let form_name = crate::tr!(FormName, lang());
    let value = use_memo(move || draft.read().name.clone());
    rsx! {
        label { "{form_name}" }
        input { aria_label: "{form_name}", value: value(), oninput: move |e| draft.write().name = e.value() }
    }
}

#[component]
fn RoleField() -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let mut draft = ctx.draft();
    let form_role = crate::tr!(FormRole, lang());
    let value = use_memo(move || draft.read().role.clone());
    rsx! {
        label { "{form_role}" }
        input { aria_label: "{form_role}", value: value(), oninput: move |e| draft.write().role = e.value() }
    }
}

#[component]
fn ContextField() -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let mut draft = ctx.draft();
    let form_context = crate::tr!(FormContext, lang());
    let value = use_memo(move || draft.read().context.clone());
    rsx! {
        label { "{form_context}" }
        textarea { aria_label: "{form_context}", value: value(), oninput: move |e| draft.write().context = e.value() }
    }
}

#[component]
fn EmojiField() -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let mut draft = ctx.draft();
    let form_avatar = crate::tr!(FormAvatar, lang());
    let emoji = use_memo(move || draft.read().avatar_emoji.clone());
    rsx! {
        label { "{form_avatar}" }
        div { class: "emoji-picker", role: "radiogroup", aria_label: "{form_avatar}",
            for e in AVATAR_EMOJIS {
                button {
                    class: "emoji-btn",
                    class: if emoji() == *e { "selected" },
                    role: "radio",
                    aria_label: "{crate::tr!(AriaAvatarPrefix, lang())} {e}",
                    aria_checked: if emoji() == *e { "true" } else { "false" },
                    onclick: move |_| draft.write().avatar_emoji = e.to_string(),
                    "{e}"
                }
            }
        }
    }
}

#[component]
fn TagsField() -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let mut draft = ctx.draft();
    let form_tags = crate::tr!(FormTags, lang());
    let mut raw = use_signal(move || {
        draft
            .read()
            .tags
            .iter()
            .map(|t| t.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    });
    rsx! {
        label { "{form_tags}" }
        input { aria_label: "{form_tags}", value: "{raw}", oninput: move |e| {
            let v = e.value();
            raw.set(v.clone());
            draft.write().tags = parse_tags(&v);
        } }
    }
}

#[component]
fn NotesField() -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let mut draft = ctx.draft();
    let form_notes = crate::tr!(FormNotes, lang());
    let value = use_memo(move || draft.read().notes.clone());
    rsx! {
        label { "{form_notes}" }
        textarea { aria_label: "{form_notes}", value: value(), rows: "4", oninput: move |e| draft.write().notes = e.value() }
    }
}

#[component]
fn ConfidenceField() -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let mut draft = ctx.draft();
    let form_confidence = crate::tr!(FormConfidence, lang());
    let confidence_hint = crate::tr!(ConfidenceHint, lang());
    let reliability_title = crate::tr!(ReliabilityTitle, lang());
    let value = use_memo(move || draft.read().confidence);
    rsx! {
        fieldset { class: "reliability",
            legend { "{reliability_title}" }
            div { class: "reliability-hint", "{confidence_hint}" }
            label { "{form_confidence}" }
            div { class: "ocean-slider",
                StepperSlider {
                    min: 1, max: 10, value: value(), display: format!("{}/10", value()),
                    onchange: move |v| draft.write().confidence = v,
                }
            }
        }
    }
}

#[component]
fn MotEditPanel(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let items = use_list_field(
        ctx,
        facet,
        |p| &p.motivations,
        |p| &mut p.motivations,
        |m| &m.motivations,
        |m| &mut m.motivations,
    );
    let edit_idx = use_signal(|| None::<usize>);
    let edit_motivations = crate::tr!(EditMotivations, lang());
    let add_row = rsx! {
        MotAddRow { items: items.clone(), edit_idx }
    };
    list_edit_section(
        items.clone(),
        edit_idx,
        crate::tr!(AriaMoveMotivationUp, lang()),
        crate::tr!(AriaMoveMotivationDown, lang()),
        crate::tr!(AriaEditMotivation, lang()),
        crate::tr!(AriaDeleteMotivation, lang()),
        edit_motivations,
        add_row,
        || {},
        move |_i, m: &Motivation| {
            let m = m.clone();
            rsx! {
                strong { "{m.r#type.emoji()} {m.r#type.i18n(cl).label}" }
                span { " {m.intensity}/10" }
                span { " {m.notes}" }
            }
        },
    )
}

#[component]
fn MotAddRow(items: ListField<Motivation>, edit_idx: Signal<Option<usize>>) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let mut sel_type = use_signal(|| MotivationType::Achievement);
    let mut sel_intensity = use_signal(|| 5u8);
    let mut sel_notes = use_signal(String::new);
    let notes_pl = crate::tr!(EditNotesPlaceholder, lang());
    let add_btn = crate::tr!(AddBtn, lang());
    let update_btn = crate::tr!(EditUpdateBtn, lang());
    let mot_undefined_warning = crate::tr!(MotUndefinedWarning, lang());

    // Populate the add-row fields whenever the shared list section sets
    // edit_idx to a row (its ✏ button only ever does `edit_idx.set(Some(i))`
    // — it has no per-type knowledge of Motivation's fields, so syncing them
    // here, reactively, is what keeps list_edit_section generic over T).
    use_effect(move || {
        let Some(idx) = edit_idx() else {
            return;
        };
        let items_read = items.val.read();
        if let Some(item) = items_read.get(idx) {
            sel_type.set(item.r#type);
            sel_intensity.set(item.intensity);
            sel_notes.set(item.notes.clone());
        }
    });

    rsx! {
        div { class: "helper-text", "{mot_undefined_warning}" }
        div { class: "add-row",
            select { value: "{sel_type}",
                onchange: move |e| { sel_type.set(parse_mot_type(&e.value())); },
                for t in MotivationType::ALL {
                    option { value: "{t:?}", "{t.emoji()} {t.i18n(cl).label}" }
                }
            }
            StepperSlider {
                min: 1, max: 10, value: sel_intensity(), display: format!("{}", sel_intensity()),
                onchange: move |v| { sel_intensity.set(v); }
            }
            input { placeholder: "{notes_pl}", value: "{sel_notes}",
                oninput: move |e| { sel_notes.set(e.value()); }
            }
            button { class: "btn", aria_label: if edit_idx().is_some() { crate::tr!(AriaUpdateMotivation, lang()) } else { crate::tr!(AriaAddMotivation, lang()) }, onclick: move |_| {
                let new_item = Motivation { r#type: sel_type(), intensity: sel_intensity(), notes: sel_notes() };
                if let Some(idx) = edit_idx() {
                    items.replace(idx, new_item);
                    edit_idx.set(None);
                } else {
                    items.push(new_item);
                }
                sel_notes.set(String::new());
                sel_intensity.set(5);
            }, if edit_idx().is_some() { "{update_btn}" } else { "{add_btn}" } }
        }
        div { class: "helper-text", "{mot_helper(&sel_type(), lang())}" }
    }
}

#[component]
fn ValEditPanel(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let items = use_list_field(
        ctx,
        facet,
        |p| &p.values,
        |p| &mut p.values,
        |m| &m.values,
        |m| &mut m.values,
    );
    let edit_idx = use_signal(|| None::<usize>);
    let edit_values = crate::tr!(EditValues, lang());
    let add_row = rsx! {
        ValAddRow { items: items.clone(), edit_idx }
    };
    list_edit_section(
        items.clone(),
        edit_idx,
        crate::tr!(AriaMoveValueUp, lang()),
        crate::tr!(AriaMoveValueDown, lang()),
        crate::tr!(AriaEditValue, lang()),
        crate::tr!(AriaDeleteValue, lang()),
        edit_values,
        add_row,
        || {},
        move |_i, v: &Value| {
            let v = v.clone();
            rsx! {
                strong { "{v.r#type.emoji()} {v.r#type.i18n(cl).label}" }
                span { " I{v.intensity}/10 P{v.priority}/10" }
                span { " {v.notes}" }
            }
        },
    )
}

#[component]
fn ValAddRow(items: ListField<Value>, edit_idx: Signal<Option<usize>>) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let mut sel_type = use_signal(|| ValueType::Career);
    let mut sel_intensity = use_signal(|| 5u8);
    let mut sel_priority = use_signal(|| 5u8);
    let mut sel_notes = use_signal(String::new);
    let notes_pl = crate::tr!(EditNotesPlaceholder, lang());
    let priority_label = crate::tr!(EditPriority, lang());
    let value_intensity_helper = crate::tr!(ValueIntensityHelper, lang());
    let value_priority_helper = crate::tr!(ValuePriorityHelper, lang());
    let add_btn = crate::tr!(AddBtn, lang());
    let update_btn = crate::tr!(EditUpdateBtn, lang());

    use_effect(move || {
        let Some(idx) = edit_idx() else {
            return;
        };
        let items_read = items.val.read();
        if let Some(item) = items_read.get(idx) {
            sel_type.set(item.r#type);
            sel_intensity.set(item.intensity);
            sel_priority.set(item.priority);
            sel_notes.set(item.notes.clone());
        }
    });

    rsx! {
        div { class: "add-row",
            select { value: "{sel_type}",
                onchange: move |e| { sel_type.set(parse_val_type(&e.value())); },
                for t in ValueType::ALL {
                    option { value: "{t:?}", "{t.emoji()} {t.i18n(cl).label}" }
                }
            }
            div { class: "dual-range",
                StepperSlider {
                    min: 1, max: 10, value: sel_intensity(), display: format!("{}", sel_intensity()),
                    onchange: move |v| { sel_intensity.set(v); }
                }
                span { class: "range-label", "I" }
                StepperSlider {
                    min: 1, max: 10, value: sel_priority(), display: format!("{}", sel_priority()),
                    onchange: move |v| { sel_priority.set(v); }
                }
                span { class: "range-label", "{priority_label}" }
            }
            input { placeholder: "{notes_pl}", value: "{sel_notes}",
                oninput: move |e| { sel_notes.set(e.value()); }
            }
            button { class: "btn", aria_label: if edit_idx().is_some() { crate::tr!(AriaUpdateValue, lang()) } else { crate::tr!(AriaAddValue, lang()) }, onclick: move |_| {
                let new_item = Value { r#type: sel_type(), intensity: sel_intensity(), priority: sel_priority(), notes: sel_notes() };
                if let Some(idx) = edit_idx() {
                    items.replace(idx, new_item);
                    edit_idx.set(None);
                } else {
                    items.push(new_item);
                }
                sel_notes.set(String::new());
                sel_intensity.set(5);
                sel_priority.set(5);
            }, if edit_idx().is_some() { "{update_btn}" } else { "{add_btn}" } }
        }
        div { class: "helper-text",
            div { "{value_helper(&sel_type(), lang())}" }
            div { "{value_intensity_helper}" }
            div { "{value_priority_helper}" }
        }
    }
}

#[component]
fn BiasEditPanel(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let items = use_list_field(
        ctx,
        facet,
        |p| &p.biases,
        |p| &mut p.biases,
        |m| &m.biases,
        |m| &mut m.biases,
    );
    let edit_idx = use_signal(|| None::<usize>);
    let edit_biases = crate::tr!(EditBiases, lang());
    let add_row = rsx! {
        BiasAddRow { items: items.clone(), edit_idx }
    };
    list_edit_section(
        items.clone(),
        edit_idx,
        crate::tr!(AriaMoveBiasUp, lang()),
        crate::tr!(AriaMoveBiasDown, lang()),
        crate::tr!(AriaEditBias, lang()),
        crate::tr!(AriaDeleteBias, lang()),
        edit_biases,
        add_row,
        || {},
        move |_i, b: &Bias| {
            let b = b.clone();
            rsx! {
                strong { "{b.r#type.emoji()} {b.r#type.i18n(cl).label}" }
                span { " {b.intensity}/10" }
                span { " {b.evidence}" }
            }
        },
    )
}

#[component]
fn BiasAddRow(items: ListField<Bias>, edit_idx: Signal<Option<usize>>) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let mut sel_type = use_signal(|| BiasType::Confirmation);
    let mut sel_intensity = use_signal(|| 5u8);
    let mut sel_evidence = use_signal(String::new);
    let bias_undefined_warning = crate::tr!(BiasUndefinedWarning, lang());
    let bias_scale_hint = crate::tr!(BiasScaleHint, lang());
    let evidence_pl = crate::tr!(EditEvidencePlaceholder, lang());
    let add_btn = crate::tr!(AddBtn, lang());
    let update_btn = crate::tr!(EditUpdateBtn, lang());

    use_effect(move || {
        let Some(idx) = edit_idx() else {
            return;
        };
        let items_read = items.val.read();
        if let Some(item) = items_read.get(idx) {
            sel_type.set(item.r#type);
            sel_intensity.set(item.intensity);
            sel_evidence.set(item.evidence.clone());
        }
    });

    rsx! {
        div { class: "helper-text", "{bias_undefined_warning}" }
        div { class: "helper-text", "{bias_scale_hint}" }
        div { class: "add-row",
            select { value: "{sel_type}",
                onchange: move |e| { sel_type.set(parse_bias_type(&e.value())); },
                for t in BiasType::ALL {
                    option { value: "{t:?}", "{t.emoji()} {t.i18n(cl).label}" }
                }
            }
            StepperSlider {
                min: 0, max: 10, value: sel_intensity(), display: format!("{}/10", sel_intensity()),
                onchange: move |v| { sel_intensity.set(v); }
            }
            input { placeholder: "{evidence_pl}", value: "{sel_evidence}",
                oninput: move |e| { sel_evidence.set(e.value()); }
            }
            button { class: "btn", aria_label: if edit_idx().is_some() { crate::tr!(AriaUpdateBias, lang()) } else { crate::tr!(AriaAddBias, lang()) }, onclick: move |_| {
                let new_item = Bias { r#type: sel_type(), intensity: sel_intensity(), evidence: sel_evidence() };
                if let Some(idx) = edit_idx() {
                    items.replace(idx, new_item);
                    edit_idx.set(None);
                } else {
                    items.push(new_item);
                }
                sel_evidence.set(String::new());
                sel_intensity.set(5);
            }, if edit_idx().is_some() { "{update_btn}" } else { "{add_btn}" } }
        }
        div { class: "helper-text", "{bias_helper(&sel_type(), lang())}" }
    }
}

#[component]
fn PatternEditPanel(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let items = use_list_field(
        ctx,
        facet,
        |p| &p.behavioral_patterns,
        |p| &mut p.behavioral_patterns,
        |m| &m.behavioral_patterns,
        |m| &mut m.behavioral_patterns,
    );
    let edit_idx = use_signal(|| None::<usize>);
    let edit_patterns = crate::tr!(EditPatterns, lang());
    let ctx_stress = crate::tr!(CtxStress, lang());
    let ctx_conflict = crate::tr!(CtxConflict, lang());
    let ctx_success = crate::tr!(CtxSuccess, lang());
    let ctx_uncertainty = crate::tr!(CtxUncertainty, lang());
    let ctx_recognition = crate::tr!(CtxRecognition, lang());
    let ctx_threatened = crate::tr!(CtxThreatened, lang());
    let ctx_change = crate::tr!(CtxChange, lang());
    let ctx_feedback = crate::tr!(CtxFeedback, lang());
    let ctx_injustice = crate::tr!(CtxInjustice, lang());
    let trigger_label = move |t: BehaviorTrigger| -> &'static str {
        match t {
            BehaviorTrigger::Stress => ctx_stress,
            BehaviorTrigger::Conflict => ctx_conflict,
            BehaviorTrigger::Success => ctx_success,
            BehaviorTrigger::Uncertainty => ctx_uncertainty,
            BehaviorTrigger::Recognition => ctx_recognition,
            BehaviorTrigger::Threatened => ctx_threatened,
            BehaviorTrigger::Change => ctx_change,
            BehaviorTrigger::Feedback => ctx_feedback,
            BehaviorTrigger::Injustice => ctx_injustice,
        }
    };
    let add_row = rsx! {
        PatternAddRow { items: items.clone(), edit_idx }
    };
    list_edit_section(
        items.clone(),
        edit_idx,
        crate::tr!(AriaMovePatternUp, lang()),
        crate::tr!(AriaMovePatternDown, lang()),
        crate::tr!(AriaEditPattern, lang()),
        crate::tr!(AriaDeletePattern, lang()),
        edit_patterns,
        add_row,
        // Deleting a row clears the local notes buffer so stale text isn't
        // carried into the next "add" — mirrored inside PatternAddRow via
        // shrink-detection, since its copy of sel_notes lives there now.
        || {},
        move |_i, bp: &BehavioralPattern| {
            let bp = bp.clone();
            rsx! {
                strong { "{trigger_label(bp.trigger)}" }
                span { " {bp.predicted_behavior.label(cl)}" }
                if !bp.notes.is_empty() {
                    span { class: "item-notes", " — {bp.notes}" }
                }
            }
        },
    )
}

#[component]
fn PatternAddRow(items: ListField<BehavioralPattern>, edit_idx: Signal<Option<usize>>) -> Element {
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let ctx_stress = crate::tr!(CtxStress, lang());
    let ctx_conflict = crate::tr!(CtxConflict, lang());
    let ctx_success = crate::tr!(CtxSuccess, lang());
    let ctx_uncertainty = crate::tr!(CtxUncertainty, lang());
    let ctx_recognition = crate::tr!(CtxRecognition, lang());
    let ctx_threatened = crate::tr!(CtxThreatened, lang());
    let ctx_change = crate::tr!(CtxChange, lang());
    let ctx_feedback = crate::tr!(CtxFeedback, lang());
    let ctx_injustice = crate::tr!(CtxInjustice, lang());
    let mut sel_trigger = use_signal(|| BehaviorTrigger::Stress);
    let mut sel_behavior = use_signal(|| BehaviorResponse::SeeksSupport);
    let mut sel_notes = use_signal(String::new);

    let notes_pl = crate::tr!(EditNotesPlaceholder, lang());
    let add_btn = crate::tr!(AddBtn, lang());
    let update_btn = crate::tr!(EditUpdateBtn, lang());

    // The old editor cleared its notes buffer when a row was deleted; that
    // state now lives here, so the clear happens on list shrink (our own ✕
    // in list_edit_section drives the removal).
    let last_len = use_hook(|| std::rc::Rc::new(std::cell::Cell::new(usize::MAX)));
    use_effect(move || {
        let len = items.val.read().len();
        if notes_clear_on_shrink(len, last_len.get()) {
            sel_notes.set(String::new());
        }
        last_len.set(len);
    });

    use_effect(move || {
        let Some(idx) = edit_idx() else {
            return;
        };
        let items_read = items.val.read();
        if let Some(item) = items_read.get(idx) {
            sel_trigger.set(item.trigger);
            sel_behavior.set(item.predicted_behavior);
            sel_notes.set(item.notes.clone());
        }
    });

    rsx! {
        div { class: "add-row",
            select { value: "{sel_trigger}",
                onchange: move |e| { sel_trigger.set(parse_trigger(&e.value())); sel_behavior.set(BehaviorResponse::options_for(sel_trigger())[0]); },
                option { value: "Stress", "{ctx_stress}" }
                option { value: "Conflict", "{ctx_conflict}" }
                option { value: "Success", "{ctx_success}" }
                option { value: "Uncertainty", "{ctx_uncertainty}" }
                option { value: "Recognition", "{ctx_recognition}" }
                option { value: "Threatened", "{ctx_threatened}" }
                option { value: "Change", "{ctx_change}" }
                option { value: "Feedback", "{ctx_feedback}" }
                option { value: "Injustice", "{ctx_injustice}" }
            }
            select { value: "{sel_behavior().serde_name()}",
                onchange: move |e| { let _ = parse_response(&e.value()).map(|v| sel_behavior.set(v)); },
                for opt in BehaviorResponse::options_for(sel_trigger()) {
                    option { value: "{opt.serde_name()}", "{opt.label(cl)}" }
                }
            }
            input {
                r#type: "text",
                placeholder: "{notes_pl}",
                value: "{sel_notes()}",
                oninput: move |e| sel_notes.set(e.value()),
            }
            button { class: "btn", aria_label: if edit_idx().is_some() { crate::tr!(AriaUpdatePattern, lang()) } else { crate::tr!(AriaAddPattern, lang()) }, onclick: move |_| {
                let new_item = BehavioralPattern {
                    trigger: sel_trigger(),
                    predicted_behavior: sel_behavior(),
                    notes: sel_notes(),
                };
                if let Some(idx) = edit_idx() {
                    items.replace(idx, new_item);
                    edit_idx.set(None);
                } else {
                    items.push(new_item);
                }
                sel_behavior.set(BehaviorResponse::options_for(sel_trigger())[0]);
                sel_notes.set(String::new());
            }, if edit_idx().is_some() { "{update_btn}" } else { "{add_btn}" } }
        }
        div { class: "helper-text", "{pattern_helper(&sel_trigger(), lang())}" }
        div { class: "helper-text", "{behavior_helper(&sel_behavior(), lang())}" }
    }
}

#[component]
fn StyleEditPanel(facet: FacetKind) -> Element {
    let ctx = use_context::<PersonEditState>();
    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let items = use_list_field(
        ctx,
        facet,
        |p| &p.styles,
        |p| &mut p.styles,
        |m| &m.styles,
        |m| &mut m.styles,
    );
    let edit_idx = use_signal(|| None::<usize>);
    let panel_title = crate::tr!(EditStyles, lang());
    let add_row = rsx! {
        StyleAddRow { items: items.clone(), edit_idx }
    };
    list_edit_section(
        items.clone(),
        edit_idx,
        crate::tr!(AriaMoveStyleUp, lang()),
        crate::tr!(AriaMoveStyleDown, lang()),
        crate::tr!(AriaEditStyle, lang()),
        crate::tr!(AriaDeleteStyle, lang()),
        panel_title,
        add_row,
        || {},
        move |_i, s: &PersonalStyle| {
            let s = s.clone();
            rsx! {
                span { class: "style-cat-badge", "{s.r#type.category().i18n_label(cl)}" }
                strong { "{s.r#type.emoji()} {s.r#type.i18n_label(cl)}" }
                span { " {s.intensity}/10" }
                span { " {s.notes}" }
            }
        },
    )
}

#[component]
fn StyleAddRow(items: ListField<PersonalStyle>, edit_idx: Signal<Option<usize>>) -> Element {
    use peoplemodeler_core::models::StyleCategory;

    let lang = use_context::<Signal<Lang>>();
    let cl = core_lang(lang());
    let mut sel_category = use_signal(|| StyleCategory::Communication);
    let mut sel_type = use_signal(|| StyleType::DirectCommunicator);
    let mut sel_intensity = use_signal(|| 5u8);
    let mut sel_notes = use_signal(String::new);
    let notes_pl = crate::tr!(EditNotesPlaceholder, lang());
    let add_btn = crate::tr!(AddBtn, lang());
    let update_btn = crate::tr!(EditUpdateBtn, lang());

    use_effect(move || {
        let cat = sel_category();
        let current = sel_type();
        if let Some(coerced) = style_selection_reconcile(cat, current) {
            sel_type.set(coerced);
        }
    });

    use_effect(move || {
        let Some(idx) = edit_idx() else {
            return;
        };
        let items_read = items.val.read();
        if let Some(item) = items_read.get(idx) {
            sel_category.set(item.r#type.category());
            sel_type.set(item.r#type);
            sel_intensity.set(item.intensity);
            sel_notes.set(item.notes.clone());
        }
    });

    rsx! {
        div { class: "add-row",
            select {
                value: "{sel_category():?}",
                onchange: move |e| {
                    let cat = parse_style_category(&e.value());
                    sel_category.set(cat);
                },
                for cat in StyleCategory::ALL {
                    option { value: "{cat:?}", "{cat.i18n_label(cl)}" }
                }
            }
            select { value: "{sel_type()}",
                onchange: move |e| { sel_type.set(parse_style_type(&e.value())); },
                for t in StyleType::options_for(sel_category()) {
                    option { value: "{t:?}", "{t.emoji()} {t.i18n_label(cl)}" }
                }
            }
            StepperSlider {
                min: 1, max: 10, value: sel_intensity(), display: format!("{}", sel_intensity()),
                onchange: move |v| { sel_intensity.set(v); }
            }
            input { placeholder: "{notes_pl}", value: "{sel_notes}",
                oninput: move |e| { sel_notes.set(e.value()); }
            }
            button { class: "btn", aria_label: if edit_idx().is_some() { crate::tr!(AriaUpdateStyle, lang()) } else { crate::tr!(AriaAddStyle, lang()) }, onclick: move |_| {
                let new_item = PersonalStyle { r#type: sel_type(), intensity: sel_intensity(), notes: sel_notes() };
                if let Some(idx) = edit_idx() {
                    items.replace(idx, new_item);
                    edit_idx.set(None);
                } else {
                    items.push(new_item);
                }
                sel_notes.set(String::new());
                sel_intensity.set(5);
            }, if edit_idx().is_some() { "{update_btn}" } else { "{add_btn}" } }
        }
        div { class: "helper-text", "{style_helper(&sel_type(), lang())}" }
    }
}

fn parse_mot_type(s: &str) -> MotivationType {
    match s {
        "Power" => MotivationType::Power,
        "Affiliation" => MotivationType::Affiliation,
        "Security" => MotivationType::Security,
        "Autonomy" => MotivationType::Autonomy,
        "Recognition" => MotivationType::Recognition,
        "Learning" => MotivationType::Learning,
        "Helping" => MotivationType::Helping,
        "Creativity" => MotivationType::Creativity,
        "Fairness" => MotivationType::Fairness,
        _ => MotivationType::Achievement,
    }
}

fn parse_val_type(s: &str) -> ValueType {
    match s {
        "Family" => ValueType::Family,
        "Health" => ValueType::Health,
        "Wealth" => ValueType::Wealth,
        "Stability" => ValueType::Stability,
        "Adventure" => ValueType::Adventure,
        "Community" => ValueType::Community,
        "Knowledge" => ValueType::Knowledge,
        "Faith" => ValueType::Faith,
        "Loyalty" => ValueType::Loyalty,
        _ => ValueType::Career,
    }
}

fn parse_bias_type(s: &str) -> BiasType {
    match s {
        "Anchoring" => BiasType::Anchoring,
        "Availability" => BiasType::Availability,
        "SunkCost" => BiasType::SunkCost,
        "DunningKruger" => BiasType::DunningKruger,
        "Impostor" => BiasType::Impostor,
        "LossAversion" => BiasType::LossAversion,
        "SocialProof" => BiasType::SocialProof,
        "Authority" => BiasType::Authority,
        "Recency" => BiasType::Recency,
        "InGroup" => BiasType::InGroup,
        "Favoritism" => BiasType::Favoritism,
        _ => BiasType::Confirmation,
    }
}

fn parse_trigger(s: &str) -> BehaviorTrigger {
    match s {
        "Conflict" => BehaviorTrigger::Conflict,
        "Success" => BehaviorTrigger::Success,
        "Uncertainty" => BehaviorTrigger::Uncertainty,
        "Recognition" => BehaviorTrigger::Recognition,
        "Threatened" => BehaviorTrigger::Threatened,
        "Change" => BehaviorTrigger::Change,
        "Feedback" => BehaviorTrigger::Feedback,
        "Injustice" => BehaviorTrigger::Injustice,
        _ => BehaviorTrigger::Stress,
    }
}

fn parse_response(s: &str) -> Option<BehaviorResponse> {
    serde_json::from_str(&format!("\"{}\"", s)).ok()
}

fn parse_style_type(s: &str) -> StyleType {
    serde_json::from_str(&format!("\"{}\"", s)).unwrap_or(StyleType::DirectCommunicator)
}

fn parse_style_category(s: &str) -> peoplemodeler_core::models::StyleCategory {
    match s {
        "ConflictResolution" => peoplemodeler_core::models::StyleCategory::ConflictResolution,
        "DecisionMaking" => peoplemodeler_core::models::StyleCategory::DecisionMaking,
        "Leadership" => peoplemodeler_core::models::StyleCategory::Leadership,
        "TimeOrientation" => peoplemodeler_core::models::StyleCategory::TimeOrientation,
        "MoralFramework" => peoplemodeler_core::models::StyleCategory::MoralFramework,
        "InterpersonalConduct" => peoplemodeler_core::models::StyleCategory::InterpersonalConduct,
        "TrustStyle" => peoplemodeler_core::models::StyleCategory::TrustStyle,
        _ => peoplemodeler_core::models::StyleCategory::Communication,
    }
}

fn coerce_style_to_category(cat: StyleCategory, sel: StyleType) -> StyleType {
    let opts = StyleType::options_for(cat);
    if opts.contains(&sel) { sel } else { opts[0] }
}

/// If `sel` is valid for `cat`, returns `None` (no change needed); otherwise
/// returns `Some` with the coerced replacement.
fn style_selection_reconcile(cat: StyleCategory, sel: StyleType) -> Option<StyleType> {
    let coerced = coerce_style_to_category(cat, sel);
    if coerced == sel { None } else { Some(coerced) }
}

/// A tick of the render counter, extracted for a direct #[test] (the counting
/// itself lives in a Dioxus component body).
fn next_render_count(current: u32) -> u32 {
    current + 1
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EditSectionId {
    ResilienceRisk,
    Ocean,
    Motivations,
    Biases,
    Reputation,
    Patterns,
    Styles,
    Values,
}

const ALL_EDIT_SECTIONS: [EditSectionId; 8] = [
    EditSectionId::ResilienceRisk,
    EditSectionId::Ocean,
    EditSectionId::Motivations,
    EditSectionId::Biases,
    EditSectionId::Reputation,
    EditSectionId::Patterns,
    EditSectionId::Styles,
    EditSectionId::Values,
];

// Sections open independently (not a single-open accordion): toggling one
// section never hides the others, so the whole form stays usable/testable
// at once. Each id just gets added to / removed from the open set.
fn toggle_section(mut current: Vec<EditSectionId>, target: EditSectionId) -> Vec<EditSectionId> {
    if let Some(pos) = current.iter().position(|x| *x == target) {
        current.remove(pos);
    } else {
        current.push(target);
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_person() -> Person {
        let mut base = blank_person();
        base.id = "person-1".into();
        base.name = "Base Name".into();
        base.ocean.openness = Some(8);
        base.ocean.conscientiousness = Some(7);
        base.motivations.push(Motivation {
            r#type: MotivationType::Power,
            intensity: 9,
            notes: "base mot".into(),
        });
        base.persona = Some(PersonaMask {
            ocean: Some(OceanScores {
                openness: Some(5),
                ..OceanScores::default()
            }),
            motivations: Some(vec![Motivation {
                r#type: MotivationType::Learning,
                intensity: 3,
                notes: "work mot".into(),
            }]),
            ..PersonaMask::default()
        });
        base.online_persona = None;
        base
    }

    #[test]
    fn blank_person_with_facet_sets_primary_context() {
        assert_eq!(blank_person().primary_facet, FacetKind::Base);
        assert_eq!(
            blank_person_with_facet(FacetKind::Work).primary_facet,
            FacetKind::Work
        );
        assert_eq!(
            blank_person_with_facet(FacetKind::Online).primary_facet,
            FacetKind::Online
        );
    }

    #[test]
    fn tab_labels_anchor_primary_and_masks() {
        let en = crate::i18n::Lang::En;
        let suffix = crate::tr!(FacetMainSuffix, en);
        let base = FacetKind::Base;
        let work = FacetKind::Work;
        let online = FacetKind::Online;
        // Anchor context always shows its name + "(main)".
        assert_eq!(
            facet_tab_label(base, base, en),
            format!("Personal life{suffix}")
        );
        assert_eq!(facet_tab_label(work, work, en), format!("At work{suffix}"));
        assert_eq!(
            facet_tab_label(online, online, en),
            format!("Online{suffix}")
        );
        // Non-anchor contexts keep their persona labels.
        assert_eq!(facet_tab_label(work, base, en), "Work Persona");
        assert_eq!(facet_tab_label(online, base, en), "Online");
        assert_eq!(facet_tab_label(base, work, en), "Personal life Persona");
        assert_eq!(facet_tab_label(online, work, en), "Online");
        assert_eq!(facet_tab_label(base, online, en), "Personal life Persona");
        assert_eq!(facet_tab_label(work, online, en), "Work Persona");
    }

    #[test]
    fn persona_mask_label_localized() {
        for lang in [Lang::En, Lang::Fr] {
            let work = persona_mask_label(FacetKind::Work, lang);
            let online = persona_mask_label(FacetKind::Online, lang);
            let base = persona_mask_label(FacetKind::Base, lang);
            assert_eq!(work, crate::tr!(PersonaSection, lang));
            assert_eq!(online, crate::tr!(PersonaOnlineSection, lang));
            assert_eq!(base, crate::tr!(PersonaBaseSection, lang));
            assert_ne!(work, online, "work and online labels differ");
            assert_ne!(online, base, "online and base labels differ");
        }
    }

    #[test]
    fn mot_helper_all_variants() {
        let lang = Lang::En;
        assert!(!mot_helper(&MotivationType::Achievement, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Power, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Affiliation, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Security, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Autonomy, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Recognition, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Learning, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Helping, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Creativity, lang).is_empty());
        assert!(!mot_helper(&MotivationType::Fairness, lang).is_empty());
    }

    #[test]
    fn bias_helper_all_variants() {
        let lang = Lang::En;
        assert!(!bias_helper(&BiasType::Confirmation, lang).is_empty());
        assert!(!bias_helper(&BiasType::Anchoring, lang).is_empty());
        assert!(!bias_helper(&BiasType::Availability, lang).is_empty());
        assert!(!bias_helper(&BiasType::SunkCost, lang).is_empty());
        assert!(!bias_helper(&BiasType::DunningKruger, lang).is_empty());
        assert!(!bias_helper(&BiasType::Impostor, lang).is_empty());
        assert!(!bias_helper(&BiasType::LossAversion, lang).is_empty());
        assert!(!bias_helper(&BiasType::SocialProof, lang).is_empty());
        assert!(!bias_helper(&BiasType::Authority, lang).is_empty());
        assert!(!bias_helper(&BiasType::Recency, lang).is_empty());
        assert!(!bias_helper(&BiasType::InGroup, lang).is_empty());
        assert!(!bias_helper(&BiasType::Favoritism, lang).is_empty());
    }

    #[test]
    fn style_helper_not_empty() {
        let lang = Lang::En;
        for t in StyleType::ALL {
            let h = style_helper(&t, lang);
            assert!(!h.is_empty(), "style_helper empty for {t:?}");
        }
    }

    #[test]
    fn pattern_helper_all_variants() {
        let lang = Lang::En;
        assert!(!pattern_helper(&BehaviorTrigger::Stress, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Conflict, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Success, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Uncertainty, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Recognition, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Threatened, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Change, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Feedback, lang).is_empty());
        assert!(!pattern_helper(&BehaviorTrigger::Injustice, lang).is_empty());
    }

    #[test]
    fn parse_mot_type_all_variants() {
        assert_eq!(parse_mot_type("Power"), MotivationType::Power);
        assert_eq!(parse_mot_type("Achievement"), MotivationType::Achievement);
        assert_eq!(parse_mot_type("Affiliation"), MotivationType::Affiliation);
        assert_eq!(parse_mot_type("Security"), MotivationType::Security);
        assert_eq!(parse_mot_type("Autonomy"), MotivationType::Autonomy);
        assert_eq!(parse_mot_type("Recognition"), MotivationType::Recognition);
        assert_eq!(parse_mot_type("Learning"), MotivationType::Learning);
        assert_eq!(parse_mot_type("Helping"), MotivationType::Helping);
        assert_eq!(parse_mot_type("Creativity"), MotivationType::Creativity);
        assert_eq!(parse_mot_type("Fairness"), MotivationType::Fairness);
    }

    #[test]
    fn parse_mot_type_unknown() {
        assert_eq!(parse_mot_type("bogus"), MotivationType::Achievement);
    }

    #[test]
    fn parse_val_type_all_variants() {
        assert_eq!(parse_val_type("Career"), ValueType::Career);
        assert_eq!(parse_val_type("Family"), ValueType::Family);
        assert_eq!(parse_val_type("Health"), ValueType::Health);
        assert_eq!(parse_val_type("Wealth"), ValueType::Wealth);
        assert_eq!(parse_val_type("Stability"), ValueType::Stability);
        assert_eq!(parse_val_type("Adventure"), ValueType::Adventure);
        assert_eq!(parse_val_type("Community"), ValueType::Community);
        assert_eq!(parse_val_type("Knowledge"), ValueType::Knowledge);
        assert_eq!(parse_val_type("Faith"), ValueType::Faith);
        assert_eq!(parse_val_type("Loyalty"), ValueType::Loyalty);
    }

    #[test]
    fn parse_val_type_unknown() {
        assert_eq!(parse_val_type("bogus"), ValueType::Career);
    }

    #[test]
    fn parse_bias_type_all_variants() {
        assert_eq!(parse_bias_type("Confirmation"), BiasType::Confirmation);
        assert_eq!(parse_bias_type("Anchoring"), BiasType::Anchoring);
        assert_eq!(parse_bias_type("Availability"), BiasType::Availability);
        assert_eq!(parse_bias_type("SunkCost"), BiasType::SunkCost);
        assert_eq!(parse_bias_type("DunningKruger"), BiasType::DunningKruger);
        assert_eq!(parse_bias_type("Impostor"), BiasType::Impostor);
        assert_eq!(parse_bias_type("LossAversion"), BiasType::LossAversion);
        assert_eq!(parse_bias_type("SocialProof"), BiasType::SocialProof);
        assert_eq!(parse_bias_type("Authority"), BiasType::Authority);
        assert_eq!(parse_bias_type("Recency"), BiasType::Recency);
        assert_eq!(parse_bias_type("InGroup"), BiasType::InGroup);
        assert_eq!(parse_bias_type("Favoritism"), BiasType::Favoritism);
    }

    #[test]
    fn parse_bias_type_unknown() {
        assert_eq!(parse_bias_type("bogus"), BiasType::Confirmation);
    }

    #[test]
    fn parse_trigger_all_variants() {
        assert_eq!(parse_trigger("Stress"), BehaviorTrigger::Stress);
        assert_eq!(parse_trigger("Conflict"), BehaviorTrigger::Conflict);
        assert_eq!(parse_trigger("Success"), BehaviorTrigger::Success);
        assert_eq!(parse_trigger("Uncertainty"), BehaviorTrigger::Uncertainty);
        assert_eq!(parse_trigger("Recognition"), BehaviorTrigger::Recognition);
        assert_eq!(parse_trigger("Threatened"), BehaviorTrigger::Threatened);
        assert_eq!(parse_trigger("Change"), BehaviorTrigger::Change);
        assert_eq!(parse_trigger("Feedback"), BehaviorTrigger::Feedback);
        assert_eq!(parse_trigger("Injustice"), BehaviorTrigger::Injustice);
    }

    #[test]
    fn parse_trigger_unknown() {
        assert_eq!(parse_trigger("bogus"), BehaviorTrigger::Stress);
    }

    #[test]
    fn parse_response_valid() {
        assert!(parse_response("remains_calm").is_some());
        assert!(parse_response("facilitates_resolution").is_some());
        assert!(parse_response("seeks_support").is_some());
    }

    #[test]
    fn parse_response_invalid() {
        assert!(parse_response("bogus").is_none());
    }

    #[test]
    fn parse_style_category_all_variants() {
        use peoplemodeler_core::models::StyleCategory;
        assert_eq!(
            parse_style_category("Communication"),
            StyleCategory::Communication
        );
        assert_eq!(
            parse_style_category("ConflictResolution"),
            StyleCategory::ConflictResolution
        );
        assert_eq!(
            parse_style_category("DecisionMaking"),
            StyleCategory::DecisionMaking
        );
        assert_eq!(
            parse_style_category("Leadership"),
            StyleCategory::Leadership
        );
        assert_eq!(
            parse_style_category("TimeOrientation"),
            StyleCategory::TimeOrientation
        );
        assert_eq!(
            parse_style_category("MoralFramework"),
            StyleCategory::MoralFramework
        );
        assert_eq!(
            parse_style_category("InterpersonalConduct"),
            StyleCategory::InterpersonalConduct
        );
        assert_eq!(
            parse_style_category("TrustStyle"),
            StyleCategory::TrustStyle
        );
    }

    #[test]
    fn parse_style_category_unknown() {
        use peoplemodeler_core::models::StyleCategory;
        assert_eq!(parse_style_category("bogus"), StyleCategory::Communication);
    }

    #[test]
    fn parse_style_type_valid() {
        let st = parse_style_type("DirectCommunicator");
        assert_eq!(st, StyleType::DirectCommunicator);
    }

    #[test]
    fn parse_style_type_invalid() {
        let st = parse_style_type("bogus");
        assert_eq!(st, StyleType::DirectCommunicator);
    }

    #[test]
    fn mot_helper_returns_known_string() {
        let lang = Lang::En;
        assert_ne!(mot_helper(&MotivationType::Achievement, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Power, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Affiliation, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Security, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Autonomy, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Recognition, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Learning, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Helping, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Creativity, lang), "xyzzy");
        assert_ne!(mot_helper(&MotivationType::Fairness, lang), "xyzzy");
    }

    #[test]
    fn bias_helper_returns_known_string() {
        let lang = Lang::En;
        assert_ne!(bias_helper(&BiasType::Confirmation, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Anchoring, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Availability, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::SunkCost, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::DunningKruger, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Impostor, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::LossAversion, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::SocialProof, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Authority, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Recency, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::InGroup, lang), "xyzzy");
        assert_ne!(bias_helper(&BiasType::Favoritism, lang), "xyzzy");
    }

    #[test]
    fn style_helper_returns_known_string() {
        let lang = Lang::En;
        for t in StyleType::ALL {
            let h = style_helper(&t, lang);
            assert_ne!(h, "xyzzy", "style_helper returned xyzzy for {t:?}");
        }
    }

    #[test]
    fn pattern_helper_returns_known_string() {
        let lang = Lang::En;
        assert_ne!(pattern_helper(&BehaviorTrigger::Stress, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Conflict, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Success, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Uncertainty, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Recognition, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Threatened, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Change, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Feedback, lang), "xyzzy");
        assert_ne!(pattern_helper(&BehaviorTrigger::Injustice, lang), "xyzzy");
    }

    #[test]
    fn mot_helper_unique_per_variant() {
        let lang = Lang::En;
        let results: Vec<_> = MotivationType::ALL
            .iter()
            .map(|t| mot_helper(t, lang))
            .collect();
        let distinct: std::collections::HashSet<&str> = results.into_iter().collect();
        assert_eq!(distinct.len(), MotivationType::ALL.len());
    }

    #[test]
    fn bias_helper_unique_per_variant() {
        let lang = Lang::En;
        let results: Vec<_> = BiasType::ALL.iter().map(|t| bias_helper(t, lang)).collect();
        let distinct: std::collections::HashSet<&str> = results.into_iter().collect();
        assert_eq!(distinct.len(), BiasType::ALL.len());
    }

    #[test]
    fn style_helper_exact_en_value() {
        let desc = style_helper(&StyleType::DirectCommunicator, Lang::En);
        assert_eq!(
            desc,
            "Speaks frankly and goes straight to the point — at work: gives direct feedback in meetings; in everyday life: says what they think without softening"
        );
    }

    #[test]
    fn style_helper_exact_fr_value() {
        let desc = style_helper(&StyleType::DirectCommunicator, Lang::Fr);
        assert_eq!(
            desc,
            "Parle franchement et va droit au but — au travail : donne un retour direct en réunion ; dans la vie : dit ce qu'il pense sans adoucir"
        );
    }

    #[test]
    fn style_helper_en_differs_from_fr() {
        let en = style_helper(&StyleType::DirectCommunicator, Lang::En);
        let fr = style_helper(&StyleType::DirectCommunicator, Lang::Fr);
        assert_ne!(en, fr);
    }

    #[test]
    fn style_helper_unique_per_variant() {
        let lang = Lang::En;
        let results: Vec<_> = StyleType::ALL
            .iter()
            .map(|t| style_helper(t, lang))
            .collect();
        let distinct: std::collections::HashSet<&str> = results.into_iter().collect();
        assert_eq!(distinct.len(), StyleType::ALL.len());
    }

    #[test]
    fn pattern_helper_unique_per_variant() {
        let lang = Lang::En;
        let results: Vec<_> = BehaviorTrigger::ALL
            .iter()
            .map(|t| pattern_helper(t, lang))
            .collect();
        let distinct: std::collections::HashSet<&str> = results.into_iter().collect();
        assert_eq!(distinct.len(), BehaviorTrigger::ALL.len());
    }

    #[test]
    fn behavior_helper_unique_per_variant() {
        let lang = Lang::En;
        let pair: Vec<(String, &'static str)> = BehaviorTrigger::ALL
            .iter()
            .flat_map(|t| BehaviorResponse::options_for(*t).to_vec())
            .map(|b| (b.serde_name().to_string(), behavior_helper(&b, lang)))
            .collect();
        let mut by_name: Vec<String> = pair.iter().map(|(n, _)| n.clone()).collect();
        by_name.sort();
        by_name.dedup();
        let mut by_desc: Vec<&'static str> = pair.iter().map(|(_, d)| *d).collect();
        by_desc.sort();
        by_desc.dedup();
        assert_eq!(by_desc.len(), by_name.len());
    }

    #[test]
    fn value_helper_bilingual_not_empty() {
        for &v in &ValueType::ALL {
            let en = value_helper(&v, Lang::En);
            let fr = value_helper(&v, Lang::Fr);
            assert!(!en.is_empty(), "{v:?} en empty");
            assert!(!fr.is_empty(), "{v:?} fr empty");
            assert_ne!(en, "xyzzy");
            assert_ne!(fr, "xyzzy");
            assert_ne!(en, fr, "{v:?} langs identical");
        }
    }

    #[test]
    fn coerce_style_to_category_keeps_valid_choice() {
        for cat in StyleCategory::ALL {
            let opts = StyleType::options_for(cat);
            let valid = opts[1];
            assert_eq!(coerce_style_to_category(cat, valid), valid);
        }
    }

    #[test]
    fn coerce_style_to_category_falls_back_to_first() {
        let cat = peoplemodeler_core::models::StyleCategory::TrustStyle;
        let invalid = StyleType::DirectCommunicator;
        assert_eq!(
            coerce_style_to_category(cat, invalid),
            StyleType::options_for(cat)[0]
        );
    }

    #[test]
    fn style_selection_reconcile_keeps_valid_choice() {
        for cat in StyleCategory::ALL {
            if let Some(valid) = StyleType::options_for(cat).get(1) {
                assert_eq!(style_selection_reconcile(cat, *valid), None);
            }
        }
    }

    #[test]
    fn style_selection_reconcile_coerces_invalid_choice() {
        let cat = peoplemodeler_core::models::StyleCategory::TrustStyle;
        let invalid = StyleType::DirectCommunicator;
        assert_eq!(
            style_selection_reconcile(cat, invalid),
            Some(StyleType::options_for(cat)[0])
        );
    }

    #[test]
    fn parse_tags_drops_blank_entries_and_trims() {
        assert_eq!(
            parse_tags("  a ,, b , ")
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>(),
            vec!["a", "b"]
        );
        assert!(
            parse_tags(" ,, , ").is_empty(),
            "only blanks must produce no tags"
        );
        assert!(
            parse_tags("").is_empty(),
            "empty input must produce no tags"
        );
    }

    #[test]
    fn persona_panel_active_truth_table() {
        assert!(!persona_panel_active(false, false));
        assert!(persona_panel_active(true, false));
        assert!(persona_panel_active(false, true));
        assert!(persona_panel_active(true, true));
    }

    #[test]
    fn is_persona_facet_gates_by_primary() {
        let kinds = [FacetKind::Base, FacetKind::Work, FacetKind::Online];
        for &primary in &kinds {
            for &facet in &kinds {
                // The anchor context is the only non-persona arena; every
                // other context wears a mask over it.
                assert_eq!(
                    is_persona_facet(facet, primary),
                    facet != primary,
                    "facet={facet:?} primary={primary:?}"
                );
            }
        }
    }

    #[test]
    fn next_render_count_increments_by_one() {
        assert_eq!(next_render_count(0), 1);
        assert_eq!(next_render_count(41), 42);
    }

    #[test]
    fn toggle_section_opens_and_closes_independently() {
        let a = EditSectionId::Ocean;
        let b = EditSectionId::ResilienceRisk;
        let open = toggle_section(vec![], a);
        assert_eq!(open, vec![a], "opens a from nothing open");
        let open = toggle_section(open, b);
        assert_eq!(open, vec![a, b], "opening b leaves a open too");
        let open = toggle_section(open, a);
        assert_eq!(open, vec![b], "closing a leaves b open");
        let open = toggle_section(open, b);
        assert!(open.is_empty(), "closing the last one leaves nothing open");
    }

    #[test]
    fn mask_of_is_none_for_base() {
        let p = test_person();
        assert!(mask_of(&p, FacetKind::Base).is_none());
        assert!(mask_of(&p, FacetKind::Work).is_some());
        assert!(mask_of(&p, FacetKind::Online).is_none());
    }

    #[test]
    fn discard_base_section_restores_saved_value_only() {
        let saved = test_person();
        let mut draft = test_person();
        // Edit the base ocean and leave the persona untouched.
        draft.ocean.openness = Some(2);
        draft.ocean.conscientiousness = Some(9);
        draft.name = "edited".into();
        discard_section(EditSectionId::Ocean, FacetKind::Base, &saved, &mut draft);
        assert_eq!(draft.ocean, saved.ocean);
        assert_eq!(draft.name, "edited", "unrelated edits untouched");
    }

    #[test]
    fn discard_base_resilience_risk_restores_defaults() {
        let mut saved = test_person();
        saved.resilience = Some(9);
        saved.risk_appetite = Some(3);
        let mut draft = test_person();
        draft.resilience = Some(1);
        draft.risk_appetite = Some(10);
        discard_section(
            EditSectionId::ResilienceRisk,
            FacetKind::Base,
            &saved,
            &mut draft,
        );
        assert_eq!(draft.resilience, saved.resilience);
        assert_eq!(draft.risk_appetite, saved.risk_appetite);
    }

    #[test]
    fn discard_work_section_restores_saved_mask_bucket() {
        let saved = test_person();
        let mut draft = test_person();
        let m = draft.persona.as_mut().unwrap();
        m.ocean = Some(OceanScores {
            openness: Some(10),
            ..OceanScores::default()
        });
        draft.ocean.openness = Some(1);
        discard_section(EditSectionId::Ocean, FacetKind::Work, &saved, &mut draft);
        // Mask bucket reverts to the saved mask (openness 5), untouched by
        // the base edits.
        assert_eq!(
            draft
                .persona
                .as_ref()
                .unwrap()
                .ocean
                .as_ref()
                .unwrap()
                .openness,
            Some(5)
        );
        assert_eq!(draft.ocean.openness, Some(1), "base untouched");
    }

    #[test]
    fn discard_work_unset_bucket_falls_back_to_inherit() {
        let mut saved = test_person();
        saved.persona.as_mut().unwrap().biases = Some(vec![]);
        let mut draft = test_person();
        draft.persona.as_mut().unwrap().biases = Some(vec![Bias {
            r#type: BiasType::Confirmation,
            intensity: 8,
            evidence: "draft".into(),
        }]);
        discard_section(EditSectionId::Biases, FacetKind::Work, &saved, &mut draft);
        // Saved mask had biases SOME(empty) → restore that; values were unset
        // in saved → None.
        assert_eq!(
            draft
                .persona
                .as_ref()
                .unwrap()
                .biases
                .as_ref()
                .unwrap()
                .len(),
            0
        );
        assert_eq!(draft.persona.as_ref().unwrap().values, None);
    }

    #[test]
    fn discard_without_persona_is_noop() {
        let mut saved = test_person();
        saved.persona = None;
        let mut draft = test_person();
        draft.persona = None;
        discard_section(EditSectionId::Ocean, FacetKind::Work, &saved, &mut draft);
        assert!(draft.persona.is_none());
    }

    #[test]
    fn bucket_base_value_materializes_on_enable() {
        let mut p = test_person();
        p.persona = Some(PersonaMask::default());
        let bucket = FacetBucket::Ocean;
        let value = bucket.base_value(&p);
        bucket.set_on_mask(p.persona.as_mut().unwrap(), true, value);
        let m = p.persona.as_ref().unwrap();
        assert!(m.ocean.is_some(), "bucket materializes on enable");
        assert_eq!(m.ocean.as_ref().unwrap().openness, Some(8));
        let off_value = bucket.base_value(&p);
        bucket.set_on_mask(p.persona.as_mut().unwrap(), false, off_value);
        assert!(p.persona.as_ref().unwrap().ocean.is_none());
    }

    #[test]
    fn list_len_resolves_through_mask_and_base() {
        let p = test_person();
        // work mask overrides motivations (1) — base also has 1.
        assert_eq!(list_len(&p, FacetKind::Work, FacetBucket::Motivations), 1);
        // online has no mask → inherits base values (0 biassed).
        assert_eq!(list_len(&p, FacetKind::Online, FacetBucket::Biases), 0);
        // base list.
        assert_eq!(list_len(&p, FacetKind::Base, FacetBucket::Motivations), 1);
    }

    fn person_with_full_base_lists() -> Person {
        let mut p = test_person();
        p.persona = Some(PersonaMask::default());
        p.rep_scores.hardworker_lazy = Some(6);
        p.biases.push(Bias {
            r#type: BiasType::Confirmation,
            intensity: 5,
            evidence: "".into(),
        });
        p.behavioral_patterns.push(BehavioralPattern {
            trigger: BehaviorTrigger::Stress,
            predicted_behavior: BehaviorResponse::SeeksSupport,
            notes: "".into(),
        });
        p.styles.push(PersonalStyle {
            r#type: StyleType::DirectCommunicator,
            intensity: 5,
            notes: "".into(),
        });
        p.values.push(Value {
            r#type: ValueType::Career,
            intensity: 5,
            priority: 5,
            notes: "".into(),
        });
        p
    }

    #[test]
    fn list_len_covers_every_list_bucket_branch() {
        let p = person_with_full_base_lists();
        for bucket in [
            FacetBucket::Motivations,
            FacetBucket::Biases,
            FacetBucket::Patterns,
            FacetBucket::Styles,
            FacetBucket::Values,
        ] {
            assert_eq!(
                list_len(&p, FacetKind::Base, bucket),
                1,
                "{bucket:?} base list"
            );
            assert_eq!(
                list_len(&p, FacetKind::Online, bucket),
                1,
                "{bucket:?} online has no mask so it inherits base"
            );
            assert_eq!(
                list_len(&p, FacetKind::Work, bucket),
                1,
                "{bucket:?} an untouched work mask inherits base"
            );
        }
    }

    #[test]
    fn list_len_uses_mask_override_when_set() {
        let mut p = person_with_full_base_lists();
        p.persona.as_mut().unwrap().motivations = Some(vec![
            Motivation {
                r#type: MotivationType::Power,
                intensity: 9,
                notes: "a".into(),
            },
            Motivation {
                r#type: MotivationType::Learning,
                intensity: 6,
                notes: "b".into(),
            },
        ]);
        assert_eq!(list_len(&p, FacetKind::Work, FacetBucket::Motivations), 2);
        assert_eq!(
            list_len(&p, FacetKind::Base, FacetBucket::Motivations),
            1,
            "base list unchanged by the override"
        );
    }

    const ALL_BUCKETS: [FacetBucket; 9] = [
        FacetBucket::Ocean,
        FacetBucket::Reputation,
        FacetBucket::Motivations,
        FacetBucket::Biases,
        FacetBucket::Patterns,
        FacetBucket::Styles,
        FacetBucket::Values,
        FacetBucket::Resilience,
        FacetBucket::RiskAppetite,
    ];

    fn mask_bucket_value(m: &PersonaMask, bucket: FacetBucket) -> Option<BucketValue> {
        match bucket {
            FacetBucket::Ocean => m.ocean.clone().map(BucketValue::Ocean),
            FacetBucket::Reputation => m.rep_scores.clone().map(BucketValue::Rep),
            FacetBucket::Motivations => m.motivations.clone().map(BucketValue::Motivations),
            FacetBucket::Biases => m.biases.clone().map(BucketValue::Biases),
            FacetBucket::Patterns => m.behavioral_patterns.clone().map(BucketValue::Patterns),
            FacetBucket::Styles => m.styles.clone().map(BucketValue::Styles),
            FacetBucket::Values => m.values.clone().map(BucketValue::Values),
            FacetBucket::Resilience => m.resilience.map(BucketValue::Resilience),
            FacetBucket::RiskAppetite => m.risk_appetite.map(BucketValue::RiskAppetite),
        }
    }

    #[test]
    fn set_on_mask_clears_and_materializes_for_every_bucket() {
        let p = test_person();
        for bucket in ALL_BUCKETS {
            let mut m = PersonaMask::default();
            assert!(!bucket.is_defined(&m), "{bucket:?} starts undefined");
            let base = bucket.base_value(&p);
            bucket.set_on_mask(&mut m, true, base.clone());
            assert!(bucket.is_defined(&m), "{bucket:?} materializes on enable");
            bucket.set_on_mask(&mut m, true, base.clone());
            assert!(
                bucket.is_defined(&m),
                "{bucket:?} stays defined on re-enable"
            );
            bucket.set_on_mask(&mut m, false, base.clone());
            assert!(!bucket.is_defined(&m), "{bucket:?} clears on disable");
            bucket.set_on_mask(&mut m, false, base);
            assert!(
                !bucket.is_defined(&m),
                "{bucket:?} disabling an undefined bucket is a no-op"
            );
        }
    }

    #[test]
    fn set_on_mask_materializes_exact_base_values() {
        let p = person_with_full_base_lists();
        for bucket in ALL_BUCKETS {
            let mut m = PersonaMask::default();
            let base = bucket.base_value(&p);
            bucket.set_on_mask(&mut m, true, base.clone());
            let stored = mask_bucket_value(&m, bucket)
                .unwrap_or_else(|| panic!("{bucket:?} not defined after materialize"));
            assert_eq!(
                stored, base,
                "{bucket:?} materialized value must match the base"
            );
        }
    }

    #[test]
    fn set_rep_dim_writes_only_its_dimension() {
        for dim in RepDim::ALL {
            let mut r = RepScores::default();
            set_rep_dim(&mut r, dim, Some(7));
            assert_eq!(r.score(dim), Some(7), "{dim:?} written");
            for other in RepDim::ALL {
                if other != dim {
                    assert_eq!(r.score(other), None, "{dim:?} wrote a sibling {other:?}");
                }
            }
        }
    }

    #[test]
    fn persona_active_for_picks_flag_by_facet() {
        // Anchor context never wears a mask, whatever its flags say.
        assert!(!persona_active_for(
            FacetKind::Work,
            false,
            true,
            false,
            FacetKind::Work
        ));
        assert!(!persona_active_for(
            FacetKind::Base,
            true,
            false,
            false,
            FacetKind::Base
        ));
        assert!(persona_active_for(
            FacetKind::Work,
            true,
            false,
            false,
            FacetKind::Base
        ));
        assert!(persona_active_for(
            FacetKind::Base,
            false,
            true,
            false,
            FacetKind::Work
        ));
        assert!(!persona_active_for(
            FacetKind::Base,
            false,
            false,
            false,
            FacetKind::Work
        ));
        assert!(persona_active_for(
            FacetKind::Base,
            false,
            false,
            true,
            FacetKind::Online
        ));
        assert!(!persona_active_for(
            FacetKind::Base,
            false,
            false,
            false,
            FacetKind::Online
        ));
    }

    #[test]
    fn bucket_defined_in_reads_mask_override_only() {
        let p = test_person();
        assert!(bucket_defined_in(
            &p,
            FacetKind::Work,
            FacetBucket::Motivations
        ));
        assert!(bucket_defined_in(&p, FacetKind::Work, FacetBucket::Ocean));
        assert!(!bucket_defined_in(&p, FacetKind::Work, FacetBucket::Biases));
        assert!(!bucket_defined_in(
            &p,
            FacetKind::Work,
            FacetBucket::Reputation
        ));
        assert!(
            !bucket_defined_in(&p, FacetKind::Base, FacetBucket::Ocean),
            "the base facet has no mask to be defined in"
        );
        assert!(
            !bucket_defined_in(&p, FacetKind::Online, FacetBucket::Ocean),
            "no online persona means nothing is defined there either"
        );
    }

    #[test]
    fn set_persona_state_enable_is_idempotent_and_disable_clears() {
        let mut p = test_person();
        p.persona = None;
        p.online_persona = None;

        set_persona_state(&mut p, FacetKind::Work, true);
        assert!(p.persona.is_some(), "enabling work creates a default mask");

        let custom = PersonaMask {
            ocean: Some(OceanScores {
                openness: Some(9),
                ..OceanScores::default()
            }),
            ..PersonaMask::default()
        };
        p.persona = Some(custom.clone());
        set_persona_state(&mut p, FacetKind::Work, true);
        assert_eq!(
            p.persona,
            Some(custom),
            "re-enabling an existing persona must not clobber it"
        );

        set_persona_state(&mut p, FacetKind::Work, false);
        assert!(p.persona.is_none(), "disabling drops the work persona");

        set_persona_state(&mut p, FacetKind::Online, true);
        assert!(p.online_persona.is_some(), "enabling online creates a mask");
        set_persona_state(&mut p, FacetKind::Online, false);
        assert!(
            p.online_persona.is_none(),
            "disabling drops the online persona"
        );

        set_persona_state(&mut p, FacetKind::Base, true);
        assert!(p.persona.is_none(), "the base facet never gains a persona");
        assert!(p.online_persona.is_none());
    }

    #[test]
    fn set_persona_state_base_becomes_private_mask_when_primary_is_work() {
        let mut p = blank_person_with_facet(FacetKind::Work);
        assert_eq!(p.primary_facet, FacetKind::Work);
        assert!(p.private_persona.is_none());

        set_persona_state(&mut p, FacetKind::Base, true);
        assert!(
            p.private_persona.is_some(),
            "enabling the Base facet under a Work primary creates the private mask"
        );
        assert!(
            mask_of(&p, FacetKind::Base).is_some(),
            "mask_of(Base) reads the private persona"
        );

        set_persona_state(&mut p, FacetKind::Base, false);
        assert!(
            p.private_persona.is_none(),
            "disabling drops the private persona"
        );
        assert!(mask_of(&p, FacetKind::Base).is_none());
    }

    #[test]
    fn mask_of_gates_by_primary_facet() {
        let mut p = blank_person_with_facet(FacetKind::Work);
        p.persona = Some(PersonaMask::default());
        p.online_persona = Some(PersonaMask::default());
        p.private_persona = Some(PersonaMask::default());

        assert!(
            mask_of(&p, FacetKind::Work).is_none(),
            "the anchor context never reads a mask"
        );
        assert!(mask_of(&p, FacetKind::Base).is_some());
        assert!(mask_of(&p, FacetKind::Online).is_some());
    }

    #[test]
    fn base_copy_mask_seeds_every_bucket_and_defaults_unset_scalars() {
        let mut saved = test_person();
        saved.resilience = None;
        saved.risk_appetite = None;
        let mask = base_copy_mask(&saved);
        assert_eq!(mask.ocean, Some(saved.ocean.clone()));
        assert_eq!(mask.rep_scores, Some(saved.rep_scores.clone()));
        assert_eq!(mask.motivations, Some(saved.motivations.clone()));
        assert_eq!(mask.biases, Some(saved.biases.clone()));
        assert_eq!(
            mask.behavioral_patterns,
            Some(saved.behavioral_patterns.clone())
        );
        assert_eq!(mask.styles, Some(saved.styles.clone()));
        assert_eq!(mask.values, Some(saved.values.clone()));
        assert_eq!(
            mask.resilience,
            Some(5),
            "unset base resilience copies as 5"
        );
        assert_eq!(
            mask.risk_appetite,
            Some(5),
            "unset base risk appetite copies as 5"
        );
    }

    #[test]
    fn base_copy_mask_keeps_explicit_scalar_values() {
        let mut saved = test_person();
        saved.resilience = Some(8);
        saved.risk_appetite = Some(2);
        let mask = base_copy_mask(&saved);
        assert_eq!(mask.resilience, Some(8));
        assert_eq!(mask.risk_appetite, Some(2));
    }

    #[test]
    fn set_persona_mask_writes_only_its_facet() {
        let mut p = test_person();
        let mask = PersonaMask::default();
        set_persona_mask(&mut p, FacetKind::Work, mask.clone());
        assert_eq!(p.persona, Some(mask));
        set_persona_mask(&mut p, FacetKind::Online, PersonaMask::default());
        assert!(p.online_persona.is_some(), "online persona gets assigned");
        let kept = p.persona.clone();
        set_persona_mask(
            &mut p,
            FacetKind::Base,
            PersonaMask {
                ocean: Some(OceanScores::default()),
                ..PersonaMask::default()
            },
        );
        assert_eq!(p.persona, kept, "base facet is a no-op");
    }

    #[test]
    fn clear_persona_mask_drops_only_the_target_facet() {
        let mut p = test_person();
        clear_persona_mask(&mut p, FacetKind::Work);
        assert!(p.persona.is_none());
        assert!(p.online_persona.is_none(), "online persona untouched");
        let mut q = test_person();
        clear_persona_mask(&mut q, FacetKind::Base);
        assert!(q.persona.is_some(), "base facet is a no-op");
        let kept = q.persona.clone();
        clear_persona_mask(&mut q, FacetKind::Online);
        assert!(q.online_persona.is_none());
        assert_eq!(q.persona, kept, "work persona untouched by online clear");
    }

    #[test]
    fn set_bucket_state_toggles_mask_bucket_and_ignores_base() {
        let mut p = test_person();
        p.online_persona = Some(PersonaMask::default());
        set_bucket_state(&mut p, FacetKind::Work, FacetBucket::Biases, true);
        assert!(bucket_defined_in(&p, FacetKind::Work, FacetBucket::Biases));
        set_bucket_state(&mut p, FacetKind::Work, FacetBucket::Biases, false);
        assert!(!bucket_defined_in(&p, FacetKind::Work, FacetBucket::Biases));
        set_bucket_state(&mut p, FacetKind::Online, FacetBucket::Ocean, true);
        assert!(bucket_defined_in(&p, FacetKind::Online, FacetBucket::Ocean));
        let work_ocean = p.persona.as_ref().unwrap().ocean.clone();
        let online_ocean = p.online_persona.as_ref().unwrap().ocean.clone();
        set_bucket_state(&mut p, FacetKind::Base, FacetBucket::Ocean, true);
        assert_eq!(
            p.persona.as_ref().unwrap().ocean,
            work_ocean,
            "base facet must not write into the work mask"
        );
        assert_eq!(
            p.online_persona.as_ref().unwrap().ocean,
            online_ocean,
            "base facet must not write into the online mask"
        );

        // A facet with no persona has no mask to write into: the toggle is
        // a genuine no-op there.
        let mut q = test_person();
        set_bucket_state(&mut q, FacetKind::Online, FacetBucket::Ocean, true);
        assert!(
            !bucket_defined_in(&q, FacetKind::Online, FacetBucket::Ocean),
            "no online persona, no bucket to define"
        );
    }

    #[test]
    fn persona_row_hidden_matches_active_facet_except_anchor() {
        // Visible only for the active tab, and never for the anchor context
        // (the anchor has no mask row).
        assert!(!persona_row_hidden(
            FacetKind::Base,
            FacetKind::Work,
            FacetKind::Work
        ));
        assert!(persona_row_hidden(
            FacetKind::Base,
            FacetKind::Work,
            FacetKind::Base
        ));
        assert!(persona_row_hidden(
            FacetKind::Base,
            FacetKind::Work,
            FacetKind::Online
        ));
        assert!(persona_row_hidden(
            FacetKind::Base,
            FacetKind::Base,
            FacetKind::Base
        ));
        assert!(persona_row_hidden(
            FacetKind::Work,
            FacetKind::Work,
            FacetKind::Work
        ));
    }

    #[test]
    fn section_panel_active_prefers_explicit_and_falls_back_to_bucket() {
        assert!(section_panel_active(Some(true), None));
        assert!(
            !section_panel_active(Some(false), Some(true)),
            "an explicit active memo wins over the bucket toggle"
        );
        assert!(section_panel_active(None, Some(true)));
        assert!(!section_panel_active(None, Some(false)));
        assert!(!section_panel_active(None, None));
    }

    #[test]
    fn progress_complete_handles_cap_and_exact_fill() {
        assert!(
            progress_complete(0, None),
            "uncapped list is always complete"
        );
        assert!(progress_complete(0, Some(0)), "0/0 is complete");
        assert!(!progress_complete(4, Some(5)));
        assert!(progress_complete(5, Some(5)), "exact fill is complete");
        assert!(progress_complete(6, Some(5)));
    }

    #[test]
    fn notes_clear_on_shrink_only_on_actual_shrink() {
        assert!(notes_clear_on_shrink(3, 5), "shrinking clears");
        assert!(!notes_clear_on_shrink(5, 5), "stable length does not clear");
        assert!(!notes_clear_on_shrink(6, 5), "growing does not clear");
        assert!(
            notes_clear_on_shrink(0, usize::MAX),
            "the first render sees an empty list as shrunk"
        );
    }

    #[test]
    fn ocean_filled_count_counts_only_set_traits() {
        let mut o = OceanScores::default();
        assert_eq!(ocean_filled_count(&o), 0);
        o.openness = Some(8);
        o.extraversion = Some(6);
        assert_eq!(ocean_filled_count(&o), 2);
    }

    #[test]
    fn rep_filled_count_counts_only_scored_dimensions() {
        let mut r = RepScores::default();
        assert_eq!(rep_filled_count(&r), 0);
        r.hardworker_lazy = Some(5);
        r.diplomatic_blunt = Some(8);
        assert_eq!(rep_filled_count(&r), 2);
    }

    #[test]
    fn apply_list_op_push_replace_remove_swap() {
        let mut v: Vec<u8> = vec![1, 2, 3];
        apply_list_op(&mut v, ListOp::Push(4));
        assert_eq!(v, vec![1, 2, 3, 4]);
        apply_list_op(&mut v, ListOp::Replace(1, 9));
        assert_eq!(v, vec![1, 9, 3, 4]);
        apply_list_op(&mut v, ListOp::Replace(100, 7));
        assert_eq!(v, vec![1, 9, 3, 4], "out-of-range replace is a no-op");
        apply_list_op(&mut v, ListOp::Swap(0, 2));
        assert_eq!(v, vec![3, 9, 1, 4]);
        apply_list_op(&mut v, ListOp::Remove(1));
        assert_eq!(v, vec![3, 1, 4]);
    }

    #[test]
    #[should_panic]
    fn apply_list_op_remove_out_of_range_panics() {
        let mut v: Vec<u8> = vec![1];
        apply_list_op(&mut v, ListOp::Remove(5));
    }
}
