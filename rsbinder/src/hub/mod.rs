// Copyright 2022 Jeff Kim <hiking90@gmail.com>
// SPDX-License-Identifier: Apache-2.0

mod servicemanager;
pub use servicemanager::*;

// 重新导出 Android 版本特定的类型以保持向后兼容性
#[cfg(target_os = "android")]
pub mod android {
    pub mod os {
        // 重新导出最新版本的类型作为默认
        pub use crate::hub::servicemanager::android_36::android_36::android::os::*;
        
        // 也提供版本特定的访问
        pub mod v33 {
            pub use crate::hub::servicemanager::android_33::android_33::android::os::*;
        }
        
        pub mod v34 {
            pub use crate::hub::servicemanager::android_34::android_34::android::os::*;
        }
        
        pub mod v35 {
            pub use crate::hub::servicemanager::android_35::android_35::android::os::*;
        }
        
        pub mod v36 {
            pub use crate::hub::servicemanager::android_36::android_36::android::os::*;
        }
    }
}

// 为了向后兼容，重新导出一些常用类型
#[cfg(target_os = "android")]
pub use android::os::IServiceManager::IServiceManager;
#[cfg(target_os = "android")]
pub use android::os::IServiceManager::BnServiceManager;
#[cfg(target_os = "android")]
pub use android::os::IServiceCallback::IServiceCallback;
#[cfg(target_os = "android")]
pub use android::os::IServiceCallback::BnServiceCallback;