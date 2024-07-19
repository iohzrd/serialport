#!/usr/bin/sh

export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk
# source <(cargo ndk-env)
# echo $CARGO_NDK_ANDROID_PLATFORM

if [ $1="clean" ]; then
    cargo clean
    gradle clean
fi

rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

cargo install cargo-ndk
# cargo ndk --platform 16 -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 -o serialport/src/main/jniLibs/ build
# cargo ndk --platform 16 -t armeabi-v7a -o serialport/src/main/jniLibs/ build
cargo ndk -t arm64-v8a -t armeabi-v7a -o serialport/src/main/jniLibs/  build # --release

# Build the AAR
gradle build