use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;

use crate::to_raw;
pub(crate) struct Card {
    pub(crate) inner: fsrs::Card,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_New(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    to_raw(Card {
        inner: fsrs::Card::new(),
    })
}
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_ScheduledDays(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> jlong {
    let c = unsafe { &*(card as *const Card) };
    c.inner.scheduled_days as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_toString<'a>(
    env: JNIEnv<'a>,
    _class: JClass<'a>,
    card: jlong,
) -> JString<'a> {
    let c = unsafe { &*(card as *const Card) };
    env.new_string(format!("{:?}", c.inner))
        .expect("string error")
}

// Due DateTime (Getter and Setter)
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_Due(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> jlong {
    let c = unsafe { &*(card as *const Card) };
    let due_timestamp = c.inner.due.timestamp();
    due_timestamp as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_SetDue(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
    due_timestamp: jlong,
) {
    let c = unsafe { &mut *(card as *mut Card) };
    c.inner.due = chrono::DateTime::from_timestamp(due_timestamp as i64, 0).expect("time error");
}

// Stability (Getter and Setter)
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_Stability(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> f64 {
    let c = unsafe { &*(card as *const Card) };
    c.inner.stability
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_SetStability(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
    stability: f64,
) {
    let c = unsafe { &mut *(card as *mut Card) };
    c.inner.stability = stability;
}

// Difficulty (Getter and Setter)
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_Difficulty(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> f64 {
    let c = unsafe { &*(card as *const Card) };
    c.inner.difficulty
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_SetDifficulty(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
    difficulty: f64,
) {
    let c = unsafe { &mut *(card as *mut Card) };
    c.inner.difficulty = difficulty;
}

// Elapsed Days (Getter and Setter)
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_ElapsedDays(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> jlong {
    let c = unsafe { &*(card as *const Card) };
    c.inner.elapsed_days as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_SetElapsedDays(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
    elapsed_days: jlong,
) {
    let c = unsafe { &mut *(card as *mut Card) };
    c.inner.elapsed_days = elapsed_days as i64;
}

// Scheduled Days (Getter and Setter)
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_SetScheduledDays(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
    days: jlong,
) {
    let c = unsafe { &mut *(card as *mut Card) };
    c.inner.scheduled_days = days as i64;
}

// Reps (Getter and Setter)
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_Reps(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> i32 {
    let c = unsafe { &*(card as *const Card) };
    c.inner.reps
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_SetReps(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
    reps: i32,
) {
    let c = unsafe { &mut *(card as *mut Card) };
    c.inner.reps = reps;
}

// Lapses (Getter and Setter)
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_Lapses(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> i32 {
    let c = unsafe { &*(card as *const Card) };
    c.inner.lapses
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_SetLapses(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
    lapses: i32,
) {
    let c = unsafe { &mut *(card as *mut Card) };
    c.inner.lapses = lapses;
}

// State (Getter and Setter)
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_State(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> i32 {
    let c = unsafe { &*(card as *const Card) };
    c.inner.state as i32
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_SetState(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
    state: i32,
) {
    let c = unsafe { &mut *(card as *mut Card) };
    c.inner.state = match state {
        0 => fsrs::State::New,
        1 => fsrs::State::Learning,
        2 => fsrs::State::Review,
        3 => fsrs::State::Relearning,
        _ => unreachable!(),
    };
}

// Last Review (Getter and Setter)
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_LastReview(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> jlong {
    let c = unsafe { &*(card as *const Card) };
    let last_review_timestamp = c.inner.last_review.timestamp();
    last_review_timestamp as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_SetLastReview(
    _env: JNIEnv,
    _class: JClass,
    card: jlong,
    last_review_timestamp: jlong,
) {
    let c = unsafe { &mut *(card as *mut Card) };
    c.inner.last_review =
        chrono::DateTime::from_timestamp(last_review_timestamp as i64, 0).expect("time error");
}
