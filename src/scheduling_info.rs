use jni::{objects::JClass, sys::jlong, JNIEnv};

use crate::{card::Card, review_log::ReviewLog, to_raw};

pub(crate) struct SchedulingInfo {
    pub(crate) inner: fsrs::SchedulingInfo,
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
