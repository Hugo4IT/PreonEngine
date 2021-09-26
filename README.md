# PreonEngine

> An extremely customizable, extendable, fast, gpu-accelerated solution for creating user-interfaces minimal effort.

## How this works

> NOTE: If you just want to use PreonEngine as a UI "framework", use the `preon` crate, it contains the opengl renderer and preon_core with some helper functions for quick&easy app development. If you need a custom renderer or wish to use it inside your game, look into this:

- `preon_renderer_*` - Rendering backend, use this in combination with `preon_core` if you want to use a specific backend or for integrating it into your game engine.
- `preon_core` - A component layout system and nothing more, you can place components and call the `layout()` function to automatically find the correct coordinates for those components. This is all backend, nothing will visually happen, no windows or rendering. You can use this if you want to use PreonEngine UI in your game. Also contains the PreonRenderer trait so you can implement your own custom renderer if you wish
- `preon` - Contains `preon_core` and `preon_renderer_opengl`

## Why PreonEngine?

PreonEngine is designed with fully custom brand designs in mind, no need to follow the usual Material, UWP or \**throws up*\* native ui. The only (free, open-source, permissively licensed) UI solution I could find that matches this level of customization was NW.js(aka Electron but without bullshit). I can give you a multitude of reasons to choose PreonEngine over those, and also some reasons why you shouldn't:

### Pros

- A feature-set specifically for UI, without an entire planet worth of completely useless features for UI.
- Single executable applications.
- Directly make UIs in Rust without having to learn 3 diffirent languages and things like specific file structure, CLI programs, build scripts and 700 extra frameworks with new ones added every day.
- The ability to make closed-source applications, without needing the source included to function.

### Cons

- Doesn't read HTML/CSS/JS files, so if you are just making a desktop app of your website skip this
- Currently, with the default OpenGL/GLFW3 renderer, there is no support for multiple windows. You can emulate it with WindowComponents if you want though
- Your users will miss the comedic loading screen tips you had for your Electron application
- Due to the focus on speed, creating a custom renderer for preon might be a chore

## Developer experience

While this crate is mainly focussed on a good and fast user experience, which means sacrifices must be made, PreonEngine also offers a stunning developer experience. PreonEngine is designed to fit any code-style, this is the reason why I call it the Preon**Engine**, unlike a framework which has a format or shiny new programming language you need to learn and adapt to, this engine is a functional-first solution to UI: Initialize at desired time <`let ui = preon::init()`>, add any wanted components wherever in your code you are with a single function <`preon::add_component(ui, ...)`>, and start the engine to open the window and start the main loop <`preon::start(ui)`> (optional, you can call the `update` and `render` functions manually if you dont want you program halted)

## Examples

### Creating a window, entering a render loop

```rs
let mut ui = preon::init();
preon::start(&mut ui);
```