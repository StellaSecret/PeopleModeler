use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};

use crate::models::{OceanScores, Person, Prediction};
use crate::insights::{self, InsightContext};
use crate::ocean;
use crate::predictions;

fn jstring_to_string(env: &mut JNIEnv, input: &JString) -> String {
    env.get_string(input)
        .map(|s| s.into())
        .unwrap_or_default()
}

fn string_to_jstring(env: &mut JNIEnv, output: &str) -> jstring {
    env.new_string(output)
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_core_PeopleModeler_analyzeOcean(
    mut env: JNIEnv,
    _class: JClass,
    json: JString,
) -> jstring {
    let input = jstring_to_string(&mut env, &json);
    let scores: OceanScores = serde_json::from_str(&input).unwrap_or_default();
    let result = ocean::interpret_all(&scores);
    string_to_jstring(&mut env, &result)
}

#[no_mangle]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_core_PeopleModeler_generateInsight(
    mut env: JNIEnv,
    _class: JClass,
    ctx: JString,
    person_json: JString,
) -> jstring {
    let context = jstring_to_string(&mut env, &ctx);
    let input = jstring_to_string(&mut env, &person_json);
    let p: Person = serde_json::from_str(&input).unwrap();
    let ic = match context.as_str() {
        "decision" => InsightContext::Decision,
        "team" => InsightContext::Team,
        "stress" => InsightContext::Stress,
        "communication" => InsightContext::Communication,
        "leadership" => InsightContext::Leadership,
        "growth" => InsightContext::Growth,
        _ => InsightContext::Decision,
    };
    let result = insights::generate_insight(ic, &p);
    string_to_jstring(&mut env, &result)
}

#[no_mangle]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_core_PeopleModeler_suggestPrediction(
    mut env: JNIEnv,
    _class: JClass,
    person_json: JString,
    context: JString,
) -> jstring {
    let pj = jstring_to_string(&mut env, &person_json);
    let ctx = jstring_to_string(&mut env, &context);
    let p: Person = serde_json::from_str(&pj).unwrap();
    let result = predictions::suggest_outcome(&p, &ctx);
    string_to_jstring(&mut env, &result)
}

#[no_mangle]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_core_PeopleModeler_calcAccuracy(
    mut env: JNIEnv,
    _class: JClass,
    predictions_json: JString,
) -> jni::sys::jdouble {
    let input = jstring_to_string(&mut env, &predictions_json);
    let preds: Vec<Prediction> = serde_json::from_str(&input).unwrap_or_default();
    predictions::prediction_accuracy_score(&preds)
}

#[no_mangle]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_core_PeopleModeler_createPrediction(
    mut env: JNIEnv,
    _class: JClass,
    person_id: JString,
    context: JString,
    predicted_outcome: JString,
) -> jstring {
    let pid = jstring_to_string(&mut env, &person_id);
    let ctx = jstring_to_string(&mut env, &context);
    let out = jstring_to_string(&mut env, &predicted_outcome);
    let p = predictions::create_prediction(&pid, &ctx, &out);
    let json = serde_json::to_string(&p).unwrap_or_default();
    string_to_jstring(&mut env, &json)
}

#[no_mangle]
pub extern "system" fn Java_com_stellasecret_peoplemodeler_core_PeopleModeler_resolvePrediction(
    mut env: JNIEnv,
    _class: JClass,
    prediction_json: JString,
    actual_outcome: JString,
    accuracy: jint,
) -> jstring {
    let input = jstring_to_string(&mut env, &prediction_json);
    let actual = jstring_to_string(&mut env, &actual_outcome);
    let mut p: Prediction = serde_json::from_str(&input).unwrap();
    predictions::resolve_prediction(&mut p, &actual, accuracy as u8);
    let json = serde_json::to_string(&p).unwrap_or_default();
    string_to_jstring(&mut env, &json)
}
