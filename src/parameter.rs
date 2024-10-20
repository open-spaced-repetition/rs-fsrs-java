use jni::objects::{JClass, JDoubleArray};

use jni::sys::{jboolean, jdouble, jint, jlong};
use jni::JNIEnv;

use crate::{to_raw, Parameter};

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_fsrs_Parameter_New(
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