

#[cfg(target_os="android")]
pub mod android {
    extern crate jni;
    use self::jni::sys::{jlong};

    static mut COUNT: jlong = 0;

    #[no_mangle]
    pub unsafe extern fn Java_rakuten_test_technical_countup_Counter_countup() -> jlong {
        COUNT += 1;
        COUNT
    }

    #[no_mangle]
    pub unsafe extern fn Java_rakuten_test_technical_countup_Counter_reset() {
        COUNT = 0;
    }
}
