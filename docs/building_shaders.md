# Shader Compilation

> **Note:** Shaders come precompiled, this guide is only useful if you wish to contribute to- or edit the shaders

You might have noticed some weird shenanigans surrounding shaders and how they are set up. I was experimenting with WGPU (the primary render module) and its shaders and noticed a few things:

- Hand-written WGSL shaders compiled very slowly resulting in long initialization times.
- SPIR-V (sorta pre-compiled) shaders were even slower
- WGSL code generated by Naga compiles almost instantly
- I'm much more familiar with GLSL

The Ideal solution would be to convert my GLSL shader code to WGSL, right? Guess what, Naga doesn't have (primary) support for GLSL(-input), but it does, however, have SPIR-V(-input) binary support. The solution:

- Write shader code in GLSL
- Use the Vulkan SDK's `glslangValidator` to compile the shaders to SPIR-V binaries
- Run Naga on those binaries to convert them to WGSL

Everything can be found in the poorly made `makefile` in the root of the project.

## Setup

### Ubuntu

Install the Vulkan SDK

```bash
wget -qO - https://packages.lunarg.com/lunarg-signing-key-pub.asc | sudo apt-key add -
sudo wget -qO /etc/apt/sources.list.d/lunarg-vulkan-1.2.189-focal.list https://packages.lunarg.com/vulkan/1.2.189/lunarg-vulkan-1.2.189-focal.list
sudo apt update
sudo apt install vulkan-sdk
```

Then install Naga-cli

```bash
cargo install naga-cli
```

## Building

After Setting everything up, just run:

```bash
make shaders
```

for automatic building (unchanged shaders are skipped for faster builds, `-B` to force).