use jni::{
    objects::{JClass, JString},
    sys::jlong,
    JNIEnv,
};

pub struct ReviewLog {
    pub inner: fsrs::ReviewLog,
}
#[no_mangle]
pub extern "system" fn Java_com_example_fsrs_ReviewLog_toString<'a>(
    env: JNIEnv<'a>,
    _class: JClass<'a>,
    card: jlong,
) -> JString<'a> {
    let c = unsafe { &*(card as *const ReviewLog) };
    env.new_string(format!("{:?}", c.inner))
        .expect("string error")
}
#[no_mangle]
pub extern "system" fn Java_com_example_fsrs_ReviewLog_Rating(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
) -> i32 {
    let rl = unsafe { &*(review_log as *const ReviewLog) };
    rl.inner.rating as i32
}

#[no_mangle]
pub extern "system" fn Java_com_example_fsrs_ReviewLog_ElapsedDays(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
) -> jlong {
    let rl = unsafe { &*(review_log as *const ReviewLog) };
    rl.inner.elapsed_days as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_example_fsrs_ReviewLog_ScheduledDays(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
) -> jlong {
    let rl = unsafe { &*(review_log as *const ReviewLog) };
    rl.inner.scheduled_days as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_example_fsrs_ReviewLog_State(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
) -> i32 {
    let rl = unsafe { &*(review_log as *const ReviewLog) };
    rl.inner.state as i32
}

#[no_mangle]
pub extern "system" fn Java_com_example_fsrs_ReviewLog_ReviewedDate(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
) -> jlong {
    let rl = unsafe { &*(review_log as *const ReviewLog) };
    let reviewed_date_timestamp = rl.inner.reviewed_date.timestamp();
    reviewed_date_timestamp as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_example_fsrs_ReviewLog_SetReviewedDate(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
    reviewed_date_timestamp: jlong,
) {
    let rl = unsafe { &mut *(review_log as *mut ReviewLog) };
    rl.inner.reviewed_date =
        chrono::DateTime::from_timestamp(reviewed_date_timestamp, 0).expect("time error");
}
#[no_mangle]
pub extern "system" fn Java_com_example_fsrs_ReviewLog_SetState(
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
pub extern "system" fn Java_com_example_fsrs_ReviewLog_SetScheduledDays(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
    scheduled_days: jlong,
) {
    let rl = unsafe { &mut *(review_log as *mut ReviewLog) };
    rl.inner.scheduled_days = scheduled_days;
}
#[no_mangle]
pub extern "system" fn Java_com_example_fsrs_ReviewLog_SetElapsedDays(
    _env: JNIEnv,
    _class: JClass,
    review_log: jlong,
    elapsed_days: jlong,
) {
    let rl = unsafe { &mut *(review_log as *mut ReviewLog) };
    rl.inner.elapsed_days = elapsed_days;
}
#[no_mangle]
pub extern "system" fn Java_com_example_fsrs_ReviewLog_SetRating(
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
