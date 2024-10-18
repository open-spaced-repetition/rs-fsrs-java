#![deny(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping from the
// current local frame (which is the scope within which local (temporary)
// references to Java objects remain valid)
use jni::objects::{JClass, JDoubleArray, JString};

use jni::sys::{jboolean, jdouble, jint, jlong};

fn to_raw<T>(value: T) -> jlong {
    Box::into_raw(Box::new(value)) as jlong
}

struct FSRS {
    inner: fsrs::FSRS,
}

struct Parameter {
    inner: fsrs::Parameters,
}

struct ReviewLog {
    inner: fsrs::ReviewLog,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_FsrsDefault(
    env: JNIEnv,
    _class: JClass,
) -> jlong {
    to_raw(FSRS {
        inner: fsrs::FSRS::default(),
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ParameterNew(
    env: JNIEnv,
    _class: JClass,
    maximum_interval: jint,
    request_retention: jdouble,
    w: JDoubleArray,
    decay: jdouble,
    factor: jdouble,
    enable_short_term: jboolean,
) -> jlong {
    let mut buf = vec![0f64; 19];
    env.get_double_array_region(w, 0, &mut buf)
        .expect("too much information");
    to_raw(Parameter {
        inner: fsrs::Parameters {
            maximum_interval,
            request_retention,
            w: buf.try_into().unwrap(),
            decay,
            factor,
            enable_short_term: enable_short_term == (true as u8), // here is u8, 0 or 1
        },
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_FsrsNew(
    env: JNIEnv,
    _class: JClass,
    parameter: jlong,
) -> jlong {
    let parameter = unsafe { &*(parameter as *const Parameter) };
    to_raw(FSRS {
        inner: fsrs::FSRS::new(parameter.inner),
    })
}

struct Card {
    inner: fsrs::Card,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_CardNew(
    env: JNIEnv,
    _class: JClass,
) -> jlong {
    to_raw(Card {
        inner: fsrs::Card::new(),
    })
}

struct RecordLog {
    inner: fsrs::RecordLog,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_repeat(
    env: JNIEnv,
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
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_RecordLogGet(
    env: JNIEnv,
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
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_SchedulingInfoCard(
    env: JNIEnv,
    _class: JClass,
    scheduling_info: jlong,
) -> jlong {
    let f = unsafe { &*(scheduling_info as *const SchedulingInfo) };
    to_raw(Card {
        inner: f.inner.card.clone(),
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_SchedulingInfoReviewLog(
    env: JNIEnv,
    _class: JClass,
    scheduling_info: jlong,
) -> jlong {
    let f = unsafe { &*(scheduling_info as *const SchedulingInfo) };
    to_raw(ReviewLog {
        inner: f.inner.review_log.clone(),
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_CardScheduledDays(
    env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> jlong {
    let c = unsafe { &*(card as *const Card) };
    c.inner.scheduled_days as jlong
}
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_CardScheduledtoString<'a>(
    env: JNIEnv<'a>,
    _class: JClass<'a>,
    card: jlong,
) -> JString<'a> {
    let c = unsafe { &*(card as *const Card) };
    env.new_string(format!("{:?}", c.inner))
        .expect("string error")
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
