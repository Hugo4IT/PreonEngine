# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - Since 2021-11-17

### Notes

- My parents bought a Macbook (Air 2021, M1) for publishing iOS games, but this also means I can test PreonEngine on MacOS (I now have Window 10, MacOS, Android and Linux)

### Added

- This file
- TextureSheet caching for slightly faster startup times
- Documentation about building shaders
- Compatibility status in `README.md`
- Logs for much easier debugging
- Text rendering
- `PreonComponent::print_tree() -> String`
- `PreonColor::into_rgba8_tuple() -> (u8, u8, u8, u8)`
- `PreonAlignment::display()`
- `AddPanel::panel_color()` - Change panel color after `AddPanel::start_panel()`
- `preon_module_xml` - Use XML files to create an app ([gh](https://github.com/Hugo4IT/PreonEngine/milestone/1))

### Changed

- Window now waits until it is fully initialized before showing
- Unused `Copy` trait now no longer required for `PreonEventListener<E>`
- `render_pass` is now handled inside (and owned by) `ShapeManager`
- `PreonShape::StaticText` -> `PreonShape::Text`
- `PreonColor::as_linear` -> `PreonColor::into_linear`
- `AddPanel::start_panel` -> `AddPanel::start_panel_hex`
- `AddPanel::empty_panel` -> `AddPanel::empty_panel_hex`
- `PreonColor::display` now outputs in linear hex format for readability
- `PreonBorder::display` and `PreonCorner::display` now output without newlines for readability
- `PreonBox::display` now only shows non-default attributes, and much better formatted
- Simplified WGPU initialization code
- Code is now Clippy compliant
- Cleaned up `Texture` code
- Removed generics and `Any` from `PreonCustomComponentStack`, this greatly simplifies creating custom components
- Added default implementations for functions in `PreonCustomComponentStack`

### Fixed

- Incorrect image data parsing from `.preonc` format, resulting in misaligned StaticTextureShapes
- `env_logger` startup error on initialization when modules are attached
- Window not correct size on Window 10 & MacOS
- My incorrect usage of `env_logger` (again)

### Removed

- `preon_data` crate

[Unreleased]: https://github.com/Hugo4IT/PreonEngine/
