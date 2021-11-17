# .preonc File Format

> To optimize runtime performance, PreonEngine optimizes all textures in the provided StaticRenderData struct. This optimization can take some time in the initialization stage, which could scare away the user. To avoid this, PreonEngine caches the resulting textures in the `.preonc` format as described below, so it only has to optimized the textures once.

Offset | Type | Name | Description
---|---|---|---
`0` | `u32` | `ic` | Amount of textures in cache
`4` | `[[f32; 4]; ic]` | `locs` | The locations of the individual textures in the TextureSheet
`4+16*ic` | `u32` | `tx` | Width of the TextureSheet (pixels)
`8+16*ic` | `u32` | `ty` | Height of the TextureSheet (pixels)
`12+16*ic` | `[[[u8; 4]; tx]; ty]` | `data` | Pixel data of the TextureSheet, stored as a byte (0-255) array: [r, g, b, a]
