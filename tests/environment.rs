use vulkan_sys::{
    vkCreateInstance, vkDestroyInstance, vkEnumerateInstanceVersion, vkGetInstanceProcAddr,
    VkApplicationInfo, VkInstanceCreateInfo, VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR,
    VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME, VK_MAKE_API_VERSION,
    VK_STRUCTURE_TYPE_APPLICATION_INFO, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, VK_SUCCESS,
    VK_VERSION_1_0,
};

use std::{os::raw::c_char, ptr};

#[test]
fn environment_test() {
    unsafe {
        let mut vulkan_version = 0;

        if vkGetInstanceProcAddr(
            ptr::null_mut(),
            b"vkEnumerateInstanceVersion\0".as_ptr() as *const c_char,
        ) == None
        {
            vulkan_version = VK_VERSION_1_0
        } else {
            let enumerate_instance_version_result = vkEnumerateInstanceVersion(&mut vulkan_version);
            assert_eq!(enumerate_instance_version_result, VK_SUCCESS, "Failed to enumerate the version of Vulkan (error code {enumerate_instance_version_result})");
        }

        let application_info = VkApplicationInfo {
            sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: ptr::null(),
            pApplicationName: b"Vulkan Environment Test\0".as_ptr() as *const c_char,
            applicationVersion: VK_MAKE_API_VERSION(0, 0, 1, 0),
            pEngineName: b"No engine\0".as_ptr() as *const c_char,
            engineVersion: 0,
            apiVersion: vulkan_version,
        };

        let enabled_extensions = vec![VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME];

        let instance_create_info = VkInstanceCreateInfo {
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR,
            pApplicationInfo: &application_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: enabled_extensions.len() as u32,
            ppEnabledExtensionNames: enabled_extensions.as_ptr() as *const *const i8,
        };

        let mut instance = ptr::null_mut();
        let instance_creation_result =
            vkCreateInstance(&instance_create_info, ptr::null(), &mut instance);

        assert_eq!(
            instance_creation_result, VK_SUCCESS,
            "Failed to create a Vulkan instance (error code {instance_creation_result})"
        );

        vkDestroyInstance(instance, ptr::null());
    }
}
