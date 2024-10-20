#![deny(warnings)]
pub mod card;
pub mod parameter;
pub mod review_log;
// This is the interface to the JVM that we'll
// call the majority of our methods on.
use crate::card::Card;
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping from the
// current local frame (which is the scope within which local (temporary)
// references to Java objects remain valid)
use jni::objects::JClass;

use crate::review_log::ReviewLog;
use jni::sys::jlong;

fn to_raw<T>(value: T) -> jlong {
    Box::into_raw(Box::new(value)) as jlong
}

struct FSRS {
    inner: fsrs::FSRS,
}

struct Parameter {
    inner: fsrs::Parameters,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_FsrsDefault(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    to_raw(FSRS {
        inner: fsrs::FSRS::default(),
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_FsrsNew(
    _env: JNIEnv,
    _class: JClass,
    parameter: jlong,
) -> jlong {
    let parameter = unsafe { &*(parameter as *const Parameter) };
    to_raw(FSRS {
        inner: fsrs::FSRS::new(parameter.inner),
    })
}

struct RecordLog {
    inner: fsrs::RecordLog,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_FsrsRepeat(
    __env: JNIEnv,
    _class: JClass,
    fsrs_: jlong,
    card: jlong,
    n: jlong,
) -> jlong {
    let f = unsafe { &*(fsrs_ as *const FSRS) };
    let card = unsafe { &*(card as *const Card) };

    to_raw(RecordLog {
        inner: f.inner.repeat(
            card.inner.clone(),
            chrono::DateTime::from_timestamp(n as i64, 0).expect("time error"),
        ),
    })
}

struct SchedulingInfo {
    inner: fsrs::SchedulingInfo,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Card_RecordLogGet(
    _env: JNIEnv,
    _class: JClass,
    scheduling_info: jlong,
    rating: jlong,
) -> jlong {
    let f = unsafe { &*(scheduling_info as *const RecordLog) };

    to_raw(SchedulingInfo {
        inner: f
            .inner
            .get(&match rating {
                1 => fsrs::Rating::Again,
                2 => fsrs::Rating::Hard,
                3 => fsrs::Rating::Good,
                4 => fsrs::Rating::Easy,
                _ => unreachable!(),
            })
            .unwrap()
            .to_owned(),
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_SchedulingInfo_Card(
    _env: JNIEnv,
    _class: JClass,
    scheduling_info: jlong,
) -> jlong {
    let f = unsafe { &*(scheduling_info as *const SchedulingInfo) };
    to_raw(Card {
        inner: f.inner.card.clone(),
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_SchedulingInfo_ReviewLog(
    _env: JNIEnv,
    _class: JClass,
    scheduling_info: jlong,
) -> jlong {
    let f = unsafe { &*(scheduling_info as *const SchedulingInfo) };
    to_raw(ReviewLog {
        inner: f.inner.review_log.clone(),
    })
}
