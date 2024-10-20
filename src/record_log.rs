use jni::objects::JClass;

use jni::sys::jlong;
use jni::JNIEnv;

use crate::{to_raw, SchedulingInfo};

pub struct RecordLog {
    pub(crate) inner: fsrs::RecordLog,
}
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_RecordLog_SchedulingInfo(
    _env: JNIEnv,
    _class: JClass,
    record_log: jlong,
    rating: jlong,
) -> jlong {
    let record_log = unsafe { &*(record_log as *const RecordLog) };

    to_raw(SchedulingInfo {
        inner: record_log
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
