// Copyright 2022 Jeff Kim <hiking90@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

fn generate_version(api_level: u32) -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let aidl_dir = format!("aidl{}", api_level);
    
    // 检查 AIDL 目录是否存在
    if !PathBuf::from(&aidl_dir).exists() {
        println!("cargo:warning=AIDL directory {} does not exist, skipping", aidl_dir);
        return Ok(());
    }

    let mut builder = rsbinder_aidl::Builder::new();

    // 添加整个 AIDL 目录作为源
    builder = builder.source(PathBuf::from(&aidl_dir));

    builder
        .output(out_dir.join(format!("android_{}_generated.rs", api_level)))
        .set_android_api_level(api_level)
        .set_crate_support(true)
        .generate()?;

    println!("cargo:rerun-if-changed={}", aidl_dir);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 生成支持的所有版本
    for api_level in [33, 34, 35, 36] {
        if let Err(e) = generate_version(api_level) {
            println!("cargo:warning=Failed to generate AIDL for API {}: {}", api_level, e);
        }
    }
    Ok(())
}