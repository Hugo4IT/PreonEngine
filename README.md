- [PreonEngine](#preonengine)
  - [Compatibility](#compatibility)
    - [Android](#android)
      - [Setup](#setup)
      - [Code](#code)
          - [File structure (optional)](#file-structure-optional)
          - [Cargo.toml](#cargotoml)
          - [main<area>.rs](#mainarears)
          - [app<area>.rs](#apparears)
          - [lib<area>.rs](#libarears)
      - [Running](#running)
      - [Debugging](#debugging)

# PreonEngine

> An extremely customizable, modular, fast, gpu-accelerated, cross-platform solution for creating user-interfaces minimal effort.

## Compatibility

Platform | Status | Rendering Backend(s)
--- | --- | ---
Windows 10+ | :heavy_check_mark: | DX12, Vulkan
Windows 8.1- | :warning: | Vulkan, experimental DX11
MacOS/iOS | :heavy_check_mark: | Metal
Linux X11/Wayland | :heavy_check_mark: | Vulkan, GLES3
Android | :heavy_check_mark: | Vulkan, GLES3

### Android

#### Setup

Download & install the [recommended version of the Android NDK](https://github.com/android/ndk/wiki#current-lts-release) ("Current LTS Release", avoid the beta version).

Install `cargo-apk`:

```bash
cargo install cargo-apk
```

Setting up your device (pick one, real hardware is easier):

- **Real hardware**
  - [Enable developer options and enable USB debugging](https://developer.android.com/studio/debug/dev-options#enable)
  - Connect your phone via USB cable to your pc
- **Emulator** (virtual phone):
  - Download & install [the official Android emulator from google](https://developer.android.com/studio/run/emulator#requirements)
  - Make sure everything is setup correctly with PATH variables etc.

#### Code

Recommended setup for single codebase applications supporting both mobile & desktop:

###### File structure (optional)
```
my_application
├──res
│   └──image.png
├──src
│   ├──app.rs
│   ├──main.rs
│   └──lib.rs
└──Cargo.toml
```

###### Cargo.toml
```toml
[features]
default = [] # 1
android = ["ndk-glue", "preon_module_wgpu/android"] # 2

[dependencies]
preon_engine = { path = "../preon_engine" }
ndk-glue = { version = "0.3.0", optional = true } # 3
preon_module_wgpu = { path = "../preon_module_wgpu" }

[lib]
crate-type = ["cdylib"] # 4
```

> 1. Don't enable any features by default
> 2. When the `android` feature is specified in `cargo`, it will enable the `ndk-glue` dependency and enable the `android` feature in `preon_module_wgpu`
> 3. Specify the `ndk-glue` dependency, but don't enable it by default. **Warning:** If you have a tool that checks for dependencies and their updates, ignore the update for `ndk-glue`, as [`winit`](https://github.com/rust-windowing/winit#android) depends on this specific version.
> 4. Android requires a dynamic library for native code.

###### main<area>.rs

```rs
mod app;

fn main() {
    app::app();
}
```

###### app<area>.rs

```rs
pub fn app() {
    // Regular preon_engine code here
}
```

###### lib<area>.rs

```rs
mod app;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    app::app();
}
```

> **Why so weird?**
>
> Android needs native code to be a dynamically linked library, so you simply cant use `fn main()`. And while we can very easily use [the official example](https://github.com/rust-windowing/android-ndk-rs#hello-world) to narrow it down to only 2 files, I've found that it can sometimes break IDE & `rust-analyzer` functionality due to the macro. This solution avoids that.

#### Running

To run your app on android you would run:

```bash
cargo apk run --features android
```

Running on desktop remains the same:

```bash
cargo run
```

#### Debugging

To view `stdout` (`println!`, `panic!`, etc.), run (with your phone connected of course):

```bash
adb logcat RustStdoutStderr:D *:S
```
