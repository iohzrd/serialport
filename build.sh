#!/usr/bin/sh

export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk

if [ $1="clean" ]; then
    cargo clean
    gradle clean
fi

rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

cargo install cargo-ndk
cargo ndk -t arm64-v8a -t armeabi-v7a -o serialport/src/main/jniLibs/  build # --release

# Build the AAR
gradle build