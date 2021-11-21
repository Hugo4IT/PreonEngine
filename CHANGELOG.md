# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - Since 2021-11-17
### Added
- This file
- TextureSheet caching for slightly faster startup times
- Documentation about building shaders
- Logs for much easier debugging
- Text rendering

### Changed
- Window now waits until it is fully initialized before showing
- Simplified WGPU initialization code

### Fixed
- Incorrect image data parsing from `.preonc` format, resulting in misaligned StaticTextureShapes
- `env_logger` startup error on initialization when modules are attached
- Window not correct size on Window 10

[Unreleased]: https://github.com/Hugo4IT/PreonEngine/
