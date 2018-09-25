static mut COUNT: i64 = 0;

#[no_mangle]
pub unsafe extern fn countup() -> i64 {
    COUNT += 1;
    COUNT
}

#[no_mangle]
pub unsafe extern fn reset() {
    COUNT = 0
}

#[cfg(target_os="android")]
pub mod android {
    extern crate jni;
    use self::jni::sys::{jlong};
    use countup;
    use reset;

    #[no_mangle]
    pub unsafe extern fn Java_rakuten_test_technical_countup_Counter_countup() -> jlong {
       countup()
    }

    #[no_mangle]
    pub unsafe extern fn Java_rakuten_test_technical_countup_Counter_reset() {
        reset()
    }
}
