# Steps

on linux,

1. install cargo xwin

- https://github.com/rust-cross/cargo-xwin

2. run with --frozen, `cargo xwin build --frozen --target x86_64-pc-windows-msvc --xwin-arch x86_64 --xwin-version 17`. it will build successfully.
3. update cc version to 1.0.91, then run it. it will fail to build.
