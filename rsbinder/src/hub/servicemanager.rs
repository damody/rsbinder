// Copyright 2022 Jeff Kim <hiking90@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::Mutex;
use std::process::Command;
use std::option::Option;

use crate::*;

// 包含所有版本的 AIDL 生成代碼
#[cfg(target_os = "android")]
pub mod android_33 {
    include!(concat!(env!("OUT_DIR"), "/android_33_generated.rs"));
}

#[cfg(target_os = "android")]
pub mod android_34 {
    include!(concat!(env!("OUT_DIR"), "/android_34_generated.rs"));
}

#[cfg(target_os = "android")]
pub mod android_35 {
    include!(concat!(env!("OUT_DIR"), "/android_35_generated.rs"));
}

#[cfg(target_os = "android")]
pub mod android_36 {
    include!(concat!(env!("OUT_DIR"), "/android_36_generated.rs"));
}

// 导入必要的 trait
#[cfg(target_os = "android")]
use android_33::android_33::android::os::IServiceManager::IServiceManager as IServiceManager33;
#[cfg(target_os = "android")]
use android_34::android_34::android::os::IServiceManager::IServiceManager as IServiceManager34;
#[cfg(target_os = "android")]
use android_35::android_35::android::os::IServiceManager::IServiceManager as IServiceManager35;
#[cfg(target_os = "android")]
use android_36::android_36::android::os::IServiceManager::IServiceManager as IServiceManager36;

// 导入 IServiceCallback trait
#[cfg(target_os = "android")]
use android_33::android_33::android::os::IServiceCallback::IServiceCallback as IServiceCallback33;
#[cfg(target_os = "android")]
use android_34::android_34::android::os::IServiceCallback::IServiceCallback as IServiceCallback34;
#[cfg(target_os = "android")]
use android_35::android_35::android::os::IServiceCallback::IServiceCallback as IServiceCallback35;
#[cfg(target_os = "android")]
use android_36::android_36::android::os::IServiceCallback::IServiceCallback as IServiceCallback36;

// 定義常量
pub const DUMP_FLAG_PRIORITY_DEFAULT: i32 = 0;

// 定義通用的 ServiceCallback trait
pub trait ServiceCallback: Send + Sync {
    fn on_registration(&self, name: &str, binder: &SIBinder) -> Result<()>;
}

// 定義通用的 ServiceManager trait
pub trait ServiceManager: Send + Sync {
    fn get_service(&self, name: &str) -> Option<SIBinder>;
    fn check_service(&self, name: &str) -> Option<SIBinder>;
    fn add_service(&self, identifier: &str, binder: SIBinder, allow_isolated: bool, dump_priority: i32) -> Result<()>;
    fn list_services(&self, dump_priority: i32) -> Vec<String>;
    fn register_for_notifications(&self, name: &str, callback: &dyn ServiceCallback) -> Result<()>;
    fn unregister_for_notifications(&self, name: &str, callback: &dyn ServiceCallback) -> Result<()>;
    fn is_declared(&self, name: &str) -> bool;
    fn get_service_debug_info(&self) -> Result<Vec<ServiceDebugInfo>>;
    fn as_any(&self) -> &dyn std::any::Any;
}

// ServiceDebugInfo 結構體
#[derive(Debug, Default, Clone)]
pub struct ServiceDebugInfo {
    pub name: String,
    pub debuggable: bool,
}

// 為 Android 33 實現 ServiceManager trait
#[cfg(target_os = "android")]
impl ServiceManager for android_33::android_33::android::os::IServiceManager::BpServiceManager {
    fn get_service(&self, name: &str) -> Option<SIBinder> {
        IServiceManager33::r#getService(self, name).ok().flatten()
    }

    fn check_service(&self, name: &str) -> Option<SIBinder> {
        IServiceManager33::r#checkService(self, name).ok().flatten()
    }

    fn add_service(&self, identifier: &str, binder: SIBinder, allow_isolated: bool, dump_priority: i32) -> Result<()> {
        IServiceManager33::r#addService(self, identifier, &binder, allow_isolated, dump_priority)
            .map_err(|e| e.into())
    }

    fn list_services(&self, dump_priority: i32) -> Vec<String> {
        IServiceManager33::r#listServices(self, dump_priority).unwrap_or_default()
    }

    fn register_for_notifications(&self, _name: &str, _callback: &dyn ServiceCallback) -> Result<()> {
        // TODO: 實現 ServiceCallback 到 IServiceCallback 的適配器
        todo!("Implement adapter for ServiceCallback -> IServiceCallback")
    }

    fn unregister_for_notifications(&self, _name: &str, _callback: &dyn ServiceCallback) -> Result<()> {
        todo!("Implement adapter for ServiceCallback -> IServiceCallback")
    }

    fn is_declared(&self, name: &str) -> bool {
        IServiceManager33::r#isDeclared(self, name).unwrap_or(false)
    }

    fn get_service_debug_info(&self) -> Result<Vec<ServiceDebugInfo>> {
        IServiceManager33::r#getServiceDebugInfo(self)
            .map(|info_list| {
                info_list.into_iter()
                    .map(|info| ServiceDebugInfo {
                        name: info.r#name,
                        debuggable: info.r#debugPid != 0,
                    })
                    .collect()
            })
            .map_err(|e| e.into())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// 為 Android 34 實現 ServiceManager trait
#[cfg(target_os = "android")]
impl ServiceManager for android_34::android_34::android::os::IServiceManager::BpServiceManager {
    fn get_service(&self, name: &str) -> Option<SIBinder> {
        IServiceManager34::r#getService(self, name).ok().flatten()
    }

    fn check_service(&self, name: &str) -> Option<SIBinder> {
        IServiceManager34::r#checkService(self, name).ok().flatten()
    }

    fn add_service(&self, identifier: &str, binder: SIBinder, allow_isolated: bool, dump_priority: i32) -> Result<()> {
        IServiceManager34::r#addService(self, identifier, &binder, allow_isolated, dump_priority)
            .map_err(|e| e.into())
    }

    fn list_services(&self, dump_priority: i32) -> Vec<String> {
        IServiceManager34::r#listServices(self, dump_priority).unwrap_or_default()
    }

    fn register_for_notifications(&self, _name: &str, _callback: &dyn ServiceCallback) -> Result<()> {
        // TODO: 實現 ServiceCallback 到 IServiceCallback 的適配器
        todo!("Implement adapter for ServiceCallback -> IServiceCallback")
    }

    fn unregister_for_notifications(&self, _name: &str, _callback: &dyn ServiceCallback) -> Result<()> {
        todo!("Implement adapter for ServiceCallback -> IServiceCallback")
    }

    fn is_declared(&self, name: &str) -> bool {
        IServiceManager34::r#isDeclared(self, name).unwrap_or(false)
    }

    fn get_service_debug_info(&self) -> Result<Vec<ServiceDebugInfo>> {
        IServiceManager34::r#getServiceDebugInfo(self)
            .map(|info_list| {
                info_list.into_iter()
                    .map(|info| ServiceDebugInfo {
                        name: info.r#name,
                        debuggable: info.r#debugPid != 0,
                    })
                    .collect()
            })
            .map_err(|e| e.into())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// 為 Android 35 實現 ServiceManager trait
#[cfg(target_os = "android")]
impl ServiceManager for android_35::android_35::android::os::IServiceManager::BpServiceManager {
    fn get_service(&self, name: &str) -> Option<SIBinder> {
        IServiceManager35::r#getService(self, name).ok().flatten()
    }

    fn check_service(&self, name: &str) -> Option<SIBinder> {
        IServiceManager35::r#checkService(self, name).ok().flatten()
    }

    fn add_service(&self, identifier: &str, binder: SIBinder, allow_isolated: bool, dump_priority: i32) -> Result<()> {
        IServiceManager35::r#addService(self, identifier, &binder, allow_isolated, dump_priority)
            .map_err(|e| e.into())
    }

    fn list_services(&self, dump_priority: i32) -> Vec<String> {
        IServiceManager35::r#listServices(self, dump_priority).unwrap_or_default()
    }

    fn register_for_notifications(&self, _name: &str, _callback: &dyn ServiceCallback) -> Result<()> {
        // TODO: 實現 ServiceCallback 到 IServiceCallback 的適配器
        todo!("Implement adapter for ServiceCallback -> IServiceCallback")
    }

    fn unregister_for_notifications(&self, _name: &str, _callback: &dyn ServiceCallback) -> Result<()> {
        todo!("Implement adapter for ServiceCallback -> IServiceCallback")
    }

    fn is_declared(&self, name: &str) -> bool {
        IServiceManager35::r#isDeclared(self, name).unwrap_or(false)
    }

    fn get_service_debug_info(&self) -> Result<Vec<ServiceDebugInfo>> {
        IServiceManager35::r#getServiceDebugInfo(self)
            .map(|info_list| {
                info_list.into_iter()
                    .map(|info| ServiceDebugInfo {
                        name: info.r#name,
                        debuggable: info.r#debugPid != 0,
                    })
                    .collect()
            })
            .map_err(|e| e.into())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// 為 Android 36 實現 ServiceManager trait
#[cfg(target_os = "android")]
impl ServiceManager for android_36::android_36::android::os::IServiceManager::BpServiceManager {
    fn get_service(&self, name: &str) -> Option<SIBinder> {
        IServiceManager36::r#getService(self, name).ok().flatten()
    }

    fn check_service(&self, name: &str) -> Option<SIBinder> {
        IServiceManager36::r#checkService(self, name).ok().flatten()
    }

    fn add_service(&self, identifier: &str, binder: SIBinder, allow_isolated: bool, dump_priority: i32) -> Result<()> {
        IServiceManager36::r#addService(self, identifier, &binder, allow_isolated, dump_priority)
            .map_err(|e| e.into())
    }

    fn list_services(&self, dump_priority: i32) -> Vec<String> {
        IServiceManager36::r#listServices(self, dump_priority).unwrap_or_default()
    }

    fn register_for_notifications(&self, _name: &str, _callback: &dyn ServiceCallback) -> Result<()> {
        // TODO: 實現 ServiceCallback 到 IServiceCallback 的適配器
        todo!("Implement adapter for ServiceCallback -> IServiceCallback")
    }

    fn unregister_for_notifications(&self, _name: &str, _callback: &dyn ServiceCallback) -> Result<()> {
        todo!("Implement adapter for ServiceCallback -> IServiceCallback")
    }

    fn is_declared(&self, name: &str) -> bool {
        IServiceManager36::r#isDeclared(self, name).unwrap_or(false)
    }

    fn get_service_debug_info(&self) -> Result<Vec<ServiceDebugInfo>> {
        IServiceManager36::r#getServiceDebugInfo(self)
            .map(|info_list| {
                info_list.into_iter()
                    .map(|info| ServiceDebugInfo {
                        name: info.r#name,
                        debuggable: info.r#debugPid != 0,
                    })
                    .collect()
            })
            .map_err(|e| e.into())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// 全局 ServiceManager 實例 - 使用 Mutex<Option<>> 来支持重新初始化
static GLOBAL_SM: OnceLock<Mutex<Option<Arc<Mutex<Box<dyn ServiceManager>>>>>> = OnceLock::new();

/// 從系統獲取 Android API 級別
#[cfg(target_os = "android")]
pub fn get_android_api_level() -> Result<u32> {
    let output = Command::new("getprop")
        .arg("ro.build.version.sdk")
        .output()
        .map_err(|_| StatusCode::Unknown)?;

    if !output.status.success() {
        return Err(StatusCode::Unknown);
    }

    let sdk_string = String::from_utf8(output.stdout)
        .map_err(|_| StatusCode::Unknown)?
        .trim()
        .to_string();

    sdk_string.parse::<u32>()
        .map_err(|_| StatusCode::Unknown)
}

#[cfg(not(target_os = "android"))]
pub fn get_android_api_level() -> Result<u32> {
    // 在非 Android 平台返回默认值
    Ok(35)
}

// 初始化 ServiceManager
#[cfg(target_os = "android")]
pub fn init_service_manager(android_version: u32) -> Result<()> {
    let process = ProcessState::as_self();
    let context = process.context_object()?;
    log::info!("ServiceManager initialization API level: {}", android_version);
    let service_manager: Box<dyn ServiceManager> = match android_version {
        33 => {
            let sm = android_33::android_33::android::os::IServiceManager::BpServiceManager::from_binder(context)
                .ok_or(StatusCode::NameNotFound)?;
            Box::new(sm)
        },
        34 => {
            let sm = android_34::android_34::android::os::IServiceManager::BpServiceManager::from_binder(context)
                .ok_or(StatusCode::NameNotFound)?;
            Box::new(sm)
        },
        35 => {
            let sm = android_35::android_35::android::os::IServiceManager::BpServiceManager::from_binder(context)
                .ok_or(StatusCode::NameNotFound)?;
            Box::new(sm)
        },
        36 => {
            let sm = android_36::android_36::android::os::IServiceManager::BpServiceManager::from_binder(context)
                .ok_or(StatusCode::NameNotFound)?;
            Box::new(sm)
        },
        _ => return Err(StatusCode::BadValue),
    };

    // 获取或创建全局 Mutex
    let global_mutex = GLOBAL_SM.get_or_init(|| Mutex::new(None));
    
    // 设置新的 ServiceManager（支持重新初始化）
    let mut global_sm = global_mutex.lock().unwrap();
    *global_sm = Some(Arc::new(Mutex::new(service_manager)));

    Ok(())
}

#[cfg(not(target_os = "android"))]
pub fn init_service_manager(android_version: u32) -> Result<()> {
    // 在非 Android 平台的占位实现
    log::warn!("ServiceManager initialization skipped on non-Android platform");
    Ok(())
}

/// 自動初始化 ServiceManager，使用系統的 API 級別
pub fn init_service_manager_auto() -> Result<()> {
    let api_level = get_android_api_level()?;
    init_service_manager(api_level)
}

/// 獲取當前的 ServiceManager 實例
fn get_service_manager() -> Arc<Mutex<Box<dyn ServiceManager>>> {
    let global_mutex = GLOBAL_SM.get()
        .expect("ServiceManager not initialized. Call init_service_manager first");
    
    let global_sm = global_mutex.lock().unwrap();
    global_sm.as_ref()
        .expect("ServiceManager not initialized. Call init_service_manager first")
        .clone()
}

/// Retrieve an existing service, blocking for a few seconds if it doesn't yet exist.
pub fn get_service(name: &str) -> Option<SIBinder> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    sm.get_service(name)
}

/// Retrieve an existing service called @a name from the service manager.
/// Non-blocking. Returns null if the service does not exist.
pub fn check_service(name: &str) -> Option<SIBinder> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    sm.check_service(name)
}

/// Return a list of all currently running services.
pub fn list_services(dump_priority: i32) -> Vec<String> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    sm.list_services(dump_priority)
}

pub fn add_service(identifier: &str, binder: SIBinder, allow_isolated: bool, dump_priority: i32) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    sm.add_service(identifier, binder, allow_isolated, dump_priority)
}

/// Request a callback when a service is registered.
pub fn register_for_notifications(name: &str, callback: &dyn ServiceCallback) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    sm.register_for_notifications(name, callback)
}

/// Unregisters all requests for notifications for a specific callback.
pub fn unregister_for_notifications(name: &str, callback: &dyn ServiceCallback) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    sm.unregister_for_notifications(name, callback)
}

/// Returns whether a given interface is declared on the device
pub fn is_declared(name: &str) -> bool {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    sm.is_declared(name)
}

pub fn get_interface<T: FromIBinder + ?Sized>(name: &str) -> Result<crate::Strong<T>> {
    let binder = get_service(name).ok_or(StatusCode::NameNotFound)?;
    T::try_from(binder)
}

/// Get debug information about all services
pub fn get_service_debug_info() -> Result<Vec<ServiceDebugInfo>> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    sm.get_service_debug_info()
}

// 兼容性函数：接受 AIDL 生成的 IServiceCallback (Android 33)
#[cfg(target_os = "android")]
pub fn register_for_notifications_aidl_33(
    name: &str, 
    callback: &crate::Strong<dyn IServiceCallback33>
) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    
    if let Some(sm33) = sm.as_any().downcast_ref::<android_33::android_33::android::os::IServiceManager::BpServiceManager>() {
        return IServiceManager33::r#registerForNotifications(sm33, name, callback).map_err(|e| e.into());
    }
    
    Err(StatusCode::BadValue)
}

// 兼容性函数：接受 AIDL 生成的 IServiceCallback (Android 34)
#[cfg(target_os = "android")]
pub fn register_for_notifications_aidl_34(
    name: &str, 
    callback: &crate::Strong<dyn IServiceCallback34>
) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    
    if let Some(sm34) = sm.as_any().downcast_ref::<android_34::android_34::android::os::IServiceManager::BpServiceManager>() {
        return IServiceManager34::r#registerForNotifications(sm34, name, callback).map_err(|e| e.into());
    }
    
    Err(StatusCode::BadValue)
}

// 兼容性函数：接受 AIDL 生成的 IServiceCallback (Android 35)
#[cfg(target_os = "android")]
pub fn register_for_notifications_aidl_35(
    name: &str, 
    callback: &crate::Strong<dyn IServiceCallback35>
) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    
    if let Some(sm35) = sm.as_any().downcast_ref::<android_35::android_35::android::os::IServiceManager::BpServiceManager>() {
        return IServiceManager35::r#registerForNotifications(sm35, name, callback).map_err(|e| e.into());
    }
    
    Err(StatusCode::BadValue)
}

// 兼容性函数：接受 AIDL 生成的 IServiceCallback (Android 36)
#[cfg(target_os = "android")]
pub fn register_for_notifications_aidl_36(
    name: &str, 
    callback: &crate::Strong<dyn IServiceCallback36>
) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    
    if let Some(sm36) = sm.as_any().downcast_ref::<android_36::android_36::android::os::IServiceManager::BpServiceManager>() {
        return IServiceManager36::r#registerForNotifications(sm36, name, callback).map_err(|e| e.into());
    }
    
    Err(StatusCode::BadValue)
}

// 类似地为 unregister_for_notifications 添加版本特定的函数
#[cfg(target_os = "android")]
pub fn unregister_for_notifications_aidl_33(
    name: &str, 
    callback: &crate::Strong<dyn IServiceCallback33>
) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    
    if let Some(sm33) = sm.as_any().downcast_ref::<android_33::android_33::android::os::IServiceManager::BpServiceManager>() {
        return IServiceManager33::r#unregisterForNotifications(sm33, name, callback).map_err(|e| e.into());
    }
    
    Err(StatusCode::BadValue)
}

#[cfg(target_os = "android")]
pub fn unregister_for_notifications_aidl_34(
    name: &str, 
    callback: &crate::Strong<dyn IServiceCallback34>
) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    
    if let Some(sm34) = sm.as_any().downcast_ref::<android_34::android_34::android::os::IServiceManager::BpServiceManager>() {
        return IServiceManager34::r#unregisterForNotifications(sm34, name, callback).map_err(|e| e.into());
    }
    
    Err(StatusCode::BadValue)
}

#[cfg(target_os = "android")]
pub fn unregister_for_notifications_aidl_35(
    name: &str, 
    callback: &crate::Strong<dyn IServiceCallback35>
) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    
    if let Some(sm35) = sm.as_any().downcast_ref::<android_35::android_35::android::os::IServiceManager::BpServiceManager>() {
        return IServiceManager35::r#unregisterForNotifications(sm35, name, callback).map_err(|e| e.into());
    }
    
    Err(StatusCode::BadValue)
}

#[cfg(target_os = "android")]
pub fn unregister_for_notifications_aidl_36(
    name: &str, 
    callback: &crate::Strong<dyn IServiceCallback36>
) -> Result<()> {
    let sm = get_service_manager();
    let sm = sm.lock().unwrap();
    
    if let Some(sm36) = sm.as_any().downcast_ref::<android_36::android_36::android::os::IServiceManager::BpServiceManager>() {
        return IServiceManager36::r#unregisterForNotifications(sm36, name, callback).map_err(|e| e.into());
    }
    
    Err(StatusCode::BadValue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_prefix() {
        // 測試初始化不同版本的 ServiceManager
        assert!(init_service_manager(33).is_ok());
        assert!(init_service_manager(34).is_ok());
        assert!(init_service_manager(35).is_ok());
        assert!(init_service_manager(36).is_ok());
        assert!(init_service_manager(37).is_err());
    }
}