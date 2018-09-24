# rakuten-count-up-ndk

Require Rust installed on your system (including rustc/cargo/rustup)  
Make sure your Cargo configured properly for Android before compile.  


##cargo/rustup config

[target.aarch64-linux-android]  
ar = "PATH_TO_YOUR_STANDALONE_NDK/arm64/bin/aarch64-linux-android-ar"  
linker = "PATH_TO_YOUR_STANDALONE_NDK/arm64/bin/aarch64-linux-android-clang"  

[target.armv7-linux-androideabi]  
ar = "PATH_TO_YOUR_STANDALONE_NDK/arm/bin/arm-linux-androideabi-ar"  
linker = "PATH_TO_YOUR_STANDALONE_NDK/arm/bin/arm-linux-androideabi-clang"  

[target.i686-linux-android]  
ar = "PATH_TO_YOUR_STANDALONE_NDK/x86/bin/i686-linux-android-ar"  
linker = "PATH_TO_YOUR_STANDALONE_NDK/x86/bin/i686-linux-android-clang"  


## Compile instructions
cargo build --target aarch64-linux-android --release  
cargo build --target armv7-linux-androideabi --release    
cargo build --target i686-linux-android --release  


