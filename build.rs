use std::{env, path::PathBuf};

fn main() {
    // Rebuild if any Vulkan header or library path changes

    println!("cargo:rerun-if-changed=Vulkan-Headers/include");

    // Rebuild if any of the environment variables used to find the Vulkan library changes

    println!("cargo:rerun-if-env-changed=VULKAN_SDK");
    println!("cargo:rerun-if-env-changed=PATH");
    println!("cargo:rerun-if-env-changed=LD_LIBRARY_PATH");
    println!("cargo:rerun-if-env-changed=DYLD_LIBRARY_PATH");

    // Vulkan bindings

    let bindings = bindgen::builder()
        .header("Vulkan-Headers/include/vulkan/vulkan.h")
        .clang_arg("-I Vulkan-Headers/include")
        .allowlist_function("vk.*")
        .allowlist_type("Vk.*")
        .allowlist_var("(VK_)|(vk).*")
        .prepend_enum_name(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");

    // Vulkan library search paths

    let vulkan_sdk_lib = env::var("VULKAN_SDK").map(|sdk| {
        PathBuf::from(sdk)
            .join("lib")
            .into_os_string()
            .into_string()
            .unwrap()
    });
    let path = env::var("PATH");
    let ld_library_path = env::var("LD_LIBRARY_PATH");
    let dyld_library_path = env::var("DYLD_LIBRARY_PATH");

    path.iter()
        .chain(ld_library_path.iter())
        .chain(dyld_library_path.iter())
        .flat_map(|paths| paths.split(':'))
        .chain(vulkan_sdk_lib.iter().map(|path| path.as_str()))
        .filter(|path| !path.is_empty())
        .for_each(|path| println!("cargo:rustc-link-search={path}"));

    // Vulkan library

    println!("cargo:rustc-link-lib=dylib=vulkan");
}
