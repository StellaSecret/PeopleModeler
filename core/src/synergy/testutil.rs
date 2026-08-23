//! Shared test helpers for the synergy test modules.
#![allow(dead_code)]

#[cfg(test)]
use crate::models::*;

#[cfg(test)]
pub(crate) fn make_person(
    openness: Option<u8>,
    conscientiousness: Option<u8>,
    extraversion: Option<u8>,
    agreeableness: Option<u8>,
    neuroticism: Option<u8>,
) -> Person {
    Person {
        id: "test".into(),
        name: "Test".into(),
        role: String::new(),
        context: String::new(),
        avatar_emoji: "🧑".into(),
        tags: vec![],
        notes: String::new(),
        motivations: vec![],
        biases: vec![],
        rep_scores: RepScores::default(),
        behavioral_patterns: vec![],
        styles: vec![],
        values: vec![],
        ocean: OceanScores {
            openness,
            conscientiousness,
            extraversion,
            agreeableness,
            neuroticism,
        },
        resilience: None,
        risk_appetite: None,
        confidence: 5,
        log: Vec::new(),
        created_at: 0,
        updated_at: 0,
    }
}

#[cfg(test)]
pub(crate) fn full_profile() -> Person {
    let mut p = make_person(Some(10), Some(10), Some(10), Some(10), Some(1));
    p.motivations = vec![
        Motivation {
            r#type: MotivationType::Achievement,
            intensity: 8,
            notes: String::new(),
        },
        Motivation {
            r#type: MotivationType::Learning,
            intensity: 7,
            notes: String::new(),
        },
        Motivation {
            r#type: MotivationType::Helping,
            intensity: 6,
            notes: String::new(),
        },
    ];
    p.biases = vec![Bias {
        r#type: BiasType::Confirmation,
        intensity: 3,
        evidence: String::new(),
    }];
    // Full, identical reputation — every dimension filled so the missing
    // penalty never fires; authoritative stays at 7 to avoid the
    // "power struggle" danger rule (both >= 8).
    p.rep_scores = RepScores {
        hardworker_lazy: Some(8),
        authoritative_submissive: Some(7),
        honest_deceitful: Some(8),
        reliable_flaky: Some(8),
        humble_arrogant: Some(8),
        calm_reactive: Some(8),
        diplomatic_blunt: Some(8),
        generous_selfish: Some(8),
        fair_favoritism: Some(8),
        trusting_suspicious: Some(8),
        assertive_passive: Some(8),
        empathetic_detached: Some(8),
        adaptable_rigid: Some(8),
    };
    p.styles = vec![
        PersonalStyle {
            r#type: StyleType::Analytical,
            intensity: 8,
            notes: String::new(),
        },
        PersonalStyle {
            r#type: StyleType::DirectCommunicator,
            intensity: 8,
            notes: String::new(),
        },
        PersonalStyle {
            r#type: StyleType::Collaborating,
            intensity: 8,
            notes: String::new(),
        },
    ];
    p
}

/// A pair that's strong on every bucket except patterns: reactive,
/// divergent triggers (Stress→Panics vs Conflict→Escalates) drive the
/// patterns bucket to 0 while OCEAN/Rep/Mot/Bias/Styles stay near 1.
#[cfg(test)]
pub(crate) fn crisis_pair() -> (Person, Person) {
    let mut a = full_profile();
    let mut b = full_profile();
    a.behavioral_patterns = vec![BehavioralPattern {
        trigger: BehaviorTrigger::Stress,
        predicted_behavior: BehaviorResponse::Panics,
        notes: String::new(),
    }];
    b.behavioral_patterns = vec![BehavioralPattern {
        trigger: BehaviorTrigger::Conflict,
        predicted_behavior: BehaviorResponse::Escalates,
        notes: String::new(),
    }];
    (a, b)
}

#[cfg(test)]
pub(crate) fn one_bias(ty: BiasType) -> Bias {
    Bias {
        r#type: ty,
        intensity: 10,
        evidence: String::new(),
    }
}

#[cfg(test)]
pub(crate) fn one_positive_pattern() -> BehavioralPattern {
    BehavioralPattern {
        trigger: BehaviorTrigger::Change,
        predicted_behavior: BehaviorResponse::RemainsCalm,
        notes: String::new(),
    }
}

#[cfg(test)]
pub(crate) fn log_entry(ts: i64, valence: i8, target: Option<&str>) -> InteractionEntry {
    InteractionEntry {
        id: format!("e{ts}-{valence}"),
        timestamp: ts,
        text: String::new(),
        valence: Some(valence),
        trigger: None,
        target_id: target.map(|s| s.to_string()),
    }
}
