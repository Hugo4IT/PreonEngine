# Todo

Things that need to be done:

- [ ] Update dependencies
    - [x] WGPU
        - [x] Rewrite GLSL shaders in WGSL
    - [ ] ndk-glue
- [x] Revise render system to function more like webdev
    > - Only one type of component with text, texture and shape
    > - Custom components are collections of said components
    > - Renderer will only have to render a single type of object, no custom rendering :)
- [x] Split component logic into sperate files per component
- [ ] Keyboard input
- [ ] Swap wgpu_glyph out with fontdue

Improvements:

- [x] Make PreonEngine core `#[no_std]`