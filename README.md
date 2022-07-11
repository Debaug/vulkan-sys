# vulkan-sys

Direct Rust bindings for the Khronos Vulkan API.

All functions are marked `unsafe`, and have zero overhead.

## Note

This crate is currently not available on [crates.io](https://www.crates.io) as
there exists [a different crate](https://crates.io/crates/vulkan-sys) with the
same name and goal. Unfortunately, its author has left GitHub and its repository
is no longer available.

## Requirements

To use this crate, you need:
 - The [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/) installed.
 - At least one of:
   - The `VULKAN_SDK` environment variable set to the location of the Vulkan SDK
   - One of the `PATH`, `LD_LIBRARY_PATH` or `DYLD_LIBRARY_PATH` set to the
     location of the Vulkan library.

See
[here](https://vulkan.lunarg.com/doc/view/latest/windows/getting_started.html) 
(Windows),
[here](https://vulkan.lunarg.com/doc/view/latest/linux/getting_started.html)
(Linux) or
[here](https://vulkan.lunarg.com/doc/sdk/latest/mac/getting_started.html)
(macOS) for more information.

## License

This crate is published under the MIT License and the Apache 2.0 License. See
[LICENSE-MIT](https://github.com/Debaug/vulkan-sys/blob/master/LICENSE-MIT) and
[LICENSE-APACHE](https://github.com/Debaug/vulkan-sys/blob/master/LICENSE-APACHE)
for more information.
