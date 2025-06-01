// Copyright 2022 Jeff Kim <hiking90@gmail.com>
// SPDX-License-Identifier: Apache-2.0
#![allow(non_snake_case)]

use std::sync::Arc;
use env_logger::Env;
use rsbinder::*;
use example_hello::*;

// 使用版本特定的 IServiceCallback
#[cfg(target_os = "android")]
use hub::android::os::v35::IServiceCallback::{IServiceCallback, BnServiceCallback};

#[cfg(not(target_os = "android"))]
struct IServiceCallback;
#[cfg(not(target_os = "android"))]
struct BnServiceCallback;

struct MyServiceCallback {
}

impl Interface for MyServiceCallback {}

#[cfg(target_os = "android")]
impl IServiceCallback for MyServiceCallback {
    fn onRegistration(&self, name: &str, _service: &SIBinder) -> rsbinder::status::Result<()> {
        println!("MyServiceCallback: {name}");
        Ok(())
    }
}

struct MyDeathRecipient {
}

impl DeathRecipient for MyDeathRecipient {
    fn binder_died(&self, _who: &WIBinder) {
        println!("MyDeathRecipient");
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    process_with_args();

    println!("🚀 Starting Hello Client Debug...");

    // Initialize ProcessState with the default binder path and the default max threads.
    println!("📱 Initializing ProcessState...");
    ProcessState::init_default();
    println!("✅ ProcessState initialized");

    // 检测 Android API 级别
    println!("🔍 Detecting Android API level...");
    match hub::get_android_api_level() {
        Ok(level) => println!("✅ Detected Android API level: {}", level),
        Err(e) => {
            println!("❌ Failed to get Android API level: {:?}", e);
            return Err(format!("Failed to get API level: {:?}", e).into());
        }
    }

    // Initialize ServiceManager
    println!("🔧 Initializing ServiceManager...");
    match hub::init_service_manager_auto() {
        Ok(_) => println!("✅ ServiceManager initialized successfully"),
        Err(e) => {
            println!("❌ Failed to initialize ServiceManager: {:?}", e);
            return Err(format!("ServiceManager init failed: {:?}", e).into());
        }
    }

    println!("📋 Listing services...");
    // This is an example of how to use service manager.
    let services = hub::list_services(hub::DUMP_FLAG_PRIORITY_DEFAULT);
    println!("✅ Found {} services", services.len());
    
    if services.is_empty() {
        println!("⚠️  No services found. This might be normal in some environments.");
    } else {
        println!("📝 First few services:");
        for (i, name) in services.iter().take(5).enumerate() {
            println!("  {}. {}", i + 1, name);
        }
        if services.len() > 5 {
            println!("  ... and {} more services", services.len() - 5);
        }
    }

    #[cfg(target_os = "android")]
    {
        println!("🔔 Registering for service notifications...");
        let service_callback = BnServiceCallback::new_binder(MyServiceCallback{});
        match hub::register_for_notifications_aidl_35(SERVICE_NAME, &service_callback) {
            Ok(_) => println!("✅ Service notification registered"),
            Err(e) => println!("⚠️  Failed to register for notifications: {:?}", e),
        }
    }
    
    #[cfg(not(target_os = "android"))]
    println!("⚠️  ServiceCallback registration skipped on non-Android platform");

    // 尝试获取 Hello 服务
    println!("🔍 Looking for Hello service: {}", SERVICE_NAME);
    match hub::get_interface::<dyn IHello>(SERVICE_NAME) {
        Ok(hello) => {
            println!("✅ Found Hello service!");
            
            let recipient = Arc::new(MyDeathRecipient{});
            match hello.as_binder().link_to_death(Arc::downgrade(&(recipient as Arc<dyn DeathRecipient>))) {
                Ok(_) => println!("✅ Death recipient linked"),
                Err(e) => println!("⚠️  Failed to link death recipient: {:?}", e),
            }

            // Call echo method of Hello proxy.
            println!("📞 Calling echo method...");
            match hello.echo("Hello World!") {
                Ok(echo) => {
                    println!("🎉 Result: {}", echo);
                    println!("✅ Hello client completed successfully!");
                },
                Err(e) => {
                    println!("❌ Echo method failed: {:?}", e);
                    return Err(format!("Echo failed: {:?}", e).into());
                }
            }
        },
        Err(e) => {
            println!("❌ Can't find Hello service '{}': {:?}", SERVICE_NAME, e);
            println!("💡 Make sure hello_service is running first!");
            return Err(format!("Service not found: {:?}", e).into());
        }
    }

    Ok(())
} 