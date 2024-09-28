// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping from the
// current local frame (which is the scope within which local (temporary)
// references to Java objects remain valid)
use jni::objects::{GlobalRef, JClass, JObject, JString};

use jni::sys::{jlong, jstring};

struct FSRS {
    inner: fsrs::FSRS,
    callback: GlobalRef,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_FsrsNew(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();
    Box::into_raw(Box::new(FSRS {
        inner: fsrs::FSRS::default(),
        callback: global_ref,
    })) as jlong
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
    Box::into_raw(Box::new(Card {
        inner: fsrs::Card::new(),
        callback: global_ref,
    })) as jlong
}

struct ScheduledCards {
    inner: fsrs::ScheduledCards,
    callback: GlobalRef,
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_schedule(
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

    Box::into_raw(Box::new(ScheduledCards {
        inner: f.inner.schedule(
            card.inner.clone(),
            chrono::DateTime::from_timestamp(n as i64, 0).expect("time error"),
        ),
        callback: global_ref,
    })) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_selectCard(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
    scheduled_cards: jlong,
    rating: jlong,
) -> jlong {
    let f = &*(scheduled_cards as *const ScheduledCards);
    let global_ref = env.new_global_ref(callback).unwrap();

    Box::into_raw(Box::new(Card {
        inner: f.inner.select_card(match rating {
            1 => fsrs::Rating::Again,
            2 => fsrs::Rating::Hard,
            3 => fsrs::Rating::Good,
            4 => fsrs::Rating::Easy,
            _ => unreachable!(),
        }),
        callback: global_ref,
    })) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_FSRS_getCardScheduledDays(
    env: JNIEnv,
    _class: JClass,
    card: jlong,
) -> jlong {
    let c = &*(card as *const Card);
    c.inner.scheduled_days as jlong
}
