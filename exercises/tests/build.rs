//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // 获取当前时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 将时间戳转换为字符串并设置为环境变量 TEST_FOO 的值
    let timestamp_str = timestamp.to_string();
    std::env::set_var("TEST_FOO", &timestamp_str);

    // 打印标准输出以告诉 Cargo 有关环境变量
    println!("cargo:rerun-if-changed=build.rs");

    // 添加 feature "pass" 的配置
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
