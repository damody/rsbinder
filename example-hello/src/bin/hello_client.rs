// Copyright 2022 Jeff Kim <hiking90@gmail.com>
// SPDX-License-Identifier: Apache-2.0
#![allow(non_snake_case)]

use std::sync::Arc;
use env_logger::Env;
use rsbinder::*;
use example_hello::*;

// 动态导入不同版本的 IServiceCallback
#[cfg(target_os = "android")]
use hub::android::os::{
    v33::IServiceCallback as IServiceCallback33,
    v34::IServiceCallback as IServiceCallback34,
    v35::IServiceCallback as IServiceCallback35,
    v36::IServiceCallback as IServiceCallback36,
};

#[cfg(not(target_os = "android"))]
struct IServiceCallback;

struct MyServiceCallback {
}

impl Interface for MyServiceCallback {}

// 为所有版本实现 IServiceCallback
#[cfg(target_os = "android")]
impl IServiceCallback33::IServiceCallback for MyServiceCallback {
    fn onRegistration(&self, name: &str, _service: &SIBinder) -> rsbinder::status::Result<()> {
        println!("MyServiceCallback (v33): {name}");
        Ok(())
    }
}

#[cfg(target_os = "android")]
impl IServiceCallback34::IServiceCallback for MyServiceCallback {
    fn onRegistration(&self, name: &str, _service: &SIBinder) -> rsbinder::status::Result<()> {
        println!("MyServiceCallback (v34): {name}");
        Ok(())
    }
}

#[cfg(target_os = "android")]
impl IServiceCallback35::IServiceCallback for MyServiceCallback {
    fn onRegistration(&self, name: &str, _service: &SIBinder) -> rsbinder::status::Result<()> {
        println!("MyServiceCallback (v35): {name}");
        Ok(())
    }
}

#[cfg(target_os = "android")]
impl IServiceCallback36::IServiceCallback for MyServiceCallback {
    fn onRegistration(&self, name: &str, _service: &SIBinder) -> rsbinder::status::Result<()> {
        println!("MyServiceCallback (v36): {name}");
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
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    process_with_args();

    println!("🚀 Starting Hello Client...");

    // Initialize ProcessState with the default binder path and the default max threads.
    println!("📱 Initializing ProcessState...");
    ProcessState::init_default();

    // 检测 Android API 级别并初始化对应版本的 ServiceManager
    println!("🔍 Detecting Android API level...");
    match hub::get_android_api_level() {
        Ok(level) => {
            println!("✅ Detected Android API level: {}", level);
            
            // 根据 API 级别初始化对应版本的 ServiceManager
            match level {
                33 | 34 | 35 | 36 => {
                    println!("🔧 Initializing ServiceManager for API {}...", level);
                    hub::init_service_manager(level)?;
                },
                _ => {
                    println!("🔧 Using auto-detection for API level {}...", level);
                    hub::init_service_manager_auto()?;
                }
            }
        },
        Err(e) => {
            println!("⚠️  Failed to get Android API level: {:?}", e);
            println!("🔧 Falling back to auto-detection...");
            hub::init_service_manager_auto()?;
        }
    }

    println!("✅ ServiceManager initialized successfully");

    println!("📋 Listing services...");
    // This is an example of how to use service manager.
    let services = hub::list_services(hub::DUMP_FLAG_PRIORITY_DEFAULT);
    println!("Found {} services", services.len());
    
    if !services.is_empty() {
        println!("First few services:");
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
        let callback = MyServiceCallback{};
        
        // 根据当前初始化的版本注册通知
        let api_level = hub::get_android_api_level().unwrap_or(35);
        let result = match api_level {
            33 => {
                let service_callback = IServiceCallback33::BnServiceCallback::new_binder(callback);
                hub::register_for_notifications_aidl_33(SERVICE_NAME, &service_callback)
            },
            34 => {
                let service_callback = IServiceCallback34::BnServiceCallback::new_binder(callback);
                hub::register_for_notifications_aidl_34(SERVICE_NAME, &service_callback)
            },
            35 => {
                let service_callback = IServiceCallback35::BnServiceCallback::new_binder(callback);
                hub::register_for_notifications_aidl_35(SERVICE_NAME, &service_callback)
            },
            36 => {
                let service_callback = IServiceCallback36::BnServiceCallback::new_binder(callback);
                hub::register_for_notifications_aidl_36(SERVICE_NAME, &service_callback)
            },
            _ => {
                // 默认使用 v35
                let service_callback = IServiceCallback35::BnServiceCallback::new_binder(callback);
                hub::register_for_notifications_aidl_35(SERVICE_NAME, &service_callback)
            }
        };
        
        match result {
            Ok(_) => println!("✅ Service notification registered for API {}", api_level),
            Err(e) => println!("⚠️  Failed to register for notifications: {:?}", e),
        }
    }
    
    #[cfg(not(target_os = "android"))]
    println!("⚠️  ServiceCallback registration skipped on non-Android platform");

    // Create a Hello proxy from binder service manager.
    println!("🔍 Looking for Hello service: {}", SERVICE_NAME);
    let hello: rsbinder::Strong<dyn IHello> = hub::get_interface(SERVICE_NAME)
        .unwrap_or_else(|_| panic!("Can't find {SERVICE_NAME}"));

    println!("✅ Found Hello service!");

    let recipient = Arc::new(MyDeathRecipient{});
    hello.as_binder().link_to_death(Arc::downgrade(&(recipient as Arc<dyn DeathRecipient>)))?;

    // Call echo method of Hello proxy.
    println!("📞 Calling echo method...");
    let echo = hello.echo("Hello World!")?;

    println!("🎉 Result: {echo}");
    println!("✅ Hello client completed successfully!");

    Ok(ProcessState::join_thread_pool()?)
}