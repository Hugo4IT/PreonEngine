# PreonEngine

### How this works

- `preon_renderer_*` - Rendering backend, use this in combination with `preon_core` if you already have a rendering engine.
- `preon_core` - A component layout system and nothing more, you can place components and call the `layout()` function to automatically find the correct coordinates for those components. This is all backend, nothing will visually happen, no windows or rendering. You can use this if you want to use PreonEngine UI in your game. All you need now is a renderer.