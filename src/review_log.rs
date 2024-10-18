use jni::{
    objects::{JClass, JString},
    sys::jlong,
    JNIEnv,
};

pub struct ReviewLog {
    pub inner: fsrs::ReviewLog,
}
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogtoString<'a>(
    env: JNIEnv<'a>,
    _class: JClass<'a>,
    card: jlong,
) -> JString<'a> {
    let c = unsafe { &*(card as *const ReviewLog) };
    env.new_string(format!("{:?}", c.inner))
        .expect("string error")
}
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogRating(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
) -> i32 {
    let rl = unsafe { &*(review_log as *const ReviewLog) };
    rl.inner.rating as i32
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogElapsedDays(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
) -> jlong {
    let rl = unsafe { &*(review_log as *const ReviewLog) };
    rl.inner.elapsed_days as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogScheduledDays(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
) -> jlong {
    let rl = unsafe { &*(review_log as *const ReviewLog) };
    rl.inner.scheduled_days as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogState(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
) -> i32 {
    let rl = unsafe { &*(review_log as *const ReviewLog) };
    rl.inner.state as i32
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogReviewedDate(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
) -> jlong {
    let rl = unsafe { &*(review_log as *const ReviewLog) };
    let reviewed_date_timestamp = rl.inner.reviewed_date.timestamp();
    reviewed_date_timestamp as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogSetReviewedDate(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
    reviewed_date_timestamp: jlong,
) {
    let rl = unsafe { &mut *(review_log as *mut ReviewLog) };
    rl.inner.reviewed_date =
        chrono::DateTime::from_timestamp(reviewed_date_timestamp as i64, 0).expect("time error");
}
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogSetState(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
    state: i32,
) {
    let rl = unsafe { &mut *(review_log as *mut ReviewLog) };
    rl.inner.state = match state {
        0 => fsrs::State::New,
        1 => fsrs::State::Learning,
        2 => fsrs::State::Review,
        3 => fsrs::State::Relearning,
        _ => unreachable!(),
    };
}
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogSetScheduledDays(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
    scheduled_days: jlong,
) {
    let rl = unsafe { &mut *(review_log as *mut ReviewLog) };
    rl.inner.scheduled_days = scheduled_days as i64;
}
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogSetElapsedDays(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
    elapsed_days: jlong,
) {
    let rl = unsafe { &mut *(review_log as *mut ReviewLog) };
    rl.inner.elapsed_days = elapsed_days as i64;
}
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ReviewLogSetRating(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
    rating: i32,
) {
    let rl = unsafe { &mut *(review_log as *mut ReviewLog) };
    rl.inner.rating = match rating {
        1 => fsrs::Rating::Again,
        2 => fsrs::Rating::Hard,
        3 => fsrs::Rating::Good,
        4 => fsrs::Rating::Easy,
        _ => unreachable!(),
    };
}
