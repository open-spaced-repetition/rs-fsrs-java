// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping from the
// current local frame (which is the scope within which local (temporary)
// references to Java objects remain valid)
use jni::objects::{GlobalRef, JClass, JObject};

use jni::sys::{jboolean, jdouble, jfloat, jfloatArray, jint, jlong};

fn to_raw<T>(value: T) -> jlong {
    Box::into_raw(Box::new(value)) as jlong
}

struct FSRS {
    inner: fsrs::FSRS,
    callback: GlobalRef,
}

struct Parameter {
    inner: fsrs::Parameters,
    callback: GlobalRef,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_FsrsDefault(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();
    to_raw(FSRS {
        inner: fsrs::FSRS::default(),
        callback: global_ref,
    })
}

#[no_mangle]
/// TODO: pass array instead of 19 val
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_ParameterNew(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
    maximum_interval: jint,
    request_retention: jdouble,
    w1: jdouble,
    w2: jdouble,
    w3: jdouble,
    w4: jdouble,
    w5: jdouble,
    w6: jdouble,
    w7: jdouble,
    w8: jdouble,
    w9: jdouble,
    w10: jdouble,
    w11: jdouble,
    w12: jdouble,
    w13: jdouble,
    w14: jdouble,
    w15: jdouble,
    w16: jdouble,
    w17: jdouble,
    w18: jdouble,
    w19: jdouble,
    decay: jdouble,
    factor: jdouble,
    enable_short_term: jboolean,
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();
    let w = [
        w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14, w15, w16, w17, w18, w19,
    ];
    to_raw(Parameter {
        inner: fsrs::Parameters {
            maximum_interval,
            request_retention,
            w,
            decay,
            factor,
            enable_short_term: enable_short_term == 0, // here is u8, 0 or 1
        },
        callback: global_ref,
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_FsrsNew(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
    parameter: jlong,
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();
    let parameter = &*(parameter as *const Parameter);
    to_raw(FSRS {
        inner: fsrs::FSRS::new(parameter.inner),
        callback: global_ref,
    })
}

struct Card {
    inner: fsrs::Card,
    callback: GlobalRef,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_CardNew(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();
    to_raw(Card {
        inner: fsrs::Card::new(),
        callback: global_ref,
    })
}

struct RecordLog {
    inner: fsrs::RecordLog,
    callback: GlobalRef,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_repeat(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
    fsrs_: jlong,
    card: jlong,
    n: jlong,
) -> jlong {
    let f = &*(fsrs_ as *const FSRS);
    let card = &*(card as *const Card);
    let global_ref = env.new_global_ref(callback).unwrap();

    to_raw(RecordLog {
        inner: f.inner.repeat(
            card.inner.clone(),
            chrono::DateTime::from_timestamp(n as i64, 0).expect("time error"),
        ),
        callback: global_ref,
    })
}

struct SchedulingInfo {
    inner: fsrs::SchedulingInfo,
    callback: GlobalRef,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_RecordLogGet(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
    scheduling_info: jlong,
    rating: jlong,
) -> jlong {
    let f = &*(scheduling_info as *const RecordLog);
    let global_ref = env.new_global_ref(callback).unwrap();

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
        callback: global_ref,
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_SchedulingInfoCard(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
    scheduling_info: jlong,
) -> jlong {
    let f = &*(scheduling_info as *const SchedulingInfo);
    let global_ref = env.new_global_ref(callback).unwrap();
    to_raw(Card {
        inner: f.inner.card.clone(),
        callback: global_ref,
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_getCardScheduledDays(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
    card: jlong,
) -> jlong {
    let c = &*(card as *const Card);
    c.inner.scheduled_days as jlong
}
