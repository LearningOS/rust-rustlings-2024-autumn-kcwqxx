//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::env;
fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
       // 设置环境变量 TEST_FOO
       let test_foo_value = format!("Value at {}", timestamp);
       println!("cargo:rerun-if-env-changed=TEST_FOO"); // 让 Cargo 在环境变量变化时重新构建
       println!("cargo:TEST_FOO={}", test_foo_value); // 设置环境变量

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    
        // 设置 rustc 的配置标志
        println!("cargo:rustc-cfg=pass");
    
        // 如果需要，可以添加条件重新构建的指令
        println!("cargo:rerun-if-env-changed=PASS");
    

}
