// #![warn(missing_docs)]
//! A modular, zero-dependency User Interface engine
//!
//! # Compatibility
//!
//! PreonEngine does not include a renderer by default, you need to attach a render module for that (e.g. preon_module_wgpu), so **compatibility depends on the render module of choice**. For now, only an official [WGPU](https://github.com/gfx-rs/wgpu/#supported-platforms) renderer exists, but making a renderer is extremely easy, as only 1 function is required. See [`rendering`] for more information and a tutorial.
//!
//! ### Render module?
//!
//! Rendering means drawing stuff to the screen, so "rendering a triangle" means drawing a triangle to the screen. PreonEngine only creates a list of shapes to render, you need a module (See ["Modules"](#modules) below) to take that list, and render all the shapes in the correct order.
//!
//! # Why "engine"?
//!
//! Recent UI solutions have opted for the word "framework", personally I hate this word as it indicates something pre-existing, with the developer having to adapt their code to fit the frames. I think code should adapt to the developer, not the other way around. PreonEngine can be used with just a single function: `engine.update(user_events)`, which returns a boolean indicating if any visual changes were present, to save on rerendering. For advanced interactivity, another function (`engine.events.pull(|event| {..})`) can be used, where `event` is an enum which you can `match` to get exactly what happened. Thats it!
//!
//! # Base features
//!
//! Without any modules attached, PreonEngine can get you:
//!
//! - A decently sized component library
//! - Custom components with a very easy to use rendering system that doesn't impact performance
//! - Full ownership of UI components in your mainloop
//! - Extremely optimized component reference system
//! - A super easy, clean way of designing your app inside of Rust
//!
//! # Modules
//!
//! PreonEngine is designed to have only the least amount of features by itself, not even rendering is included. You can add features by attaching modules like [`preon_module_wgpu`](https://crates.io/crates/preon_module_wgpu) for rendering or [`preon_module_locale`](https://crates.io/crates/preon_module_locale) for multi-language support. PreonEngine is also made to be fast by default, with minimal extra work. This also makes writing a custom renderer to, for example, include PreonEngine as the UI solution of choice for your custom game engine.
//!
//! > **Note:** A PreonEngine "module" is not the same as a rust "module", PreonEngine modules are just crates with extra functionality, rust modules are ways of organizing code.
//!
//! If you need something that isn't yet available, feel free to make you own module! If you feel like everyone can benefit from it, make a new [issue](https://github.com/Hugo4IT/PreonEngine-rs/issues) on GitHub so I can add it here.
//!
//! **Currently available modules are:**
//!
//! Official? | Crate name | Description
//! :--|:--|:--
//! Yes | preon_module_wgpu | Opens a window and draws the specified PreonRenderPass generated with `PreonEngine::render()`. Uses [WGPU](https://github.com/gfx-rs/wgpu/) as a rendering backend, so it is cross-platform with support for Vulkan, Metal, DirectX11, DirectX12 and OpenGLES3.
//!
//! **Modules currently in development (names may change) are:**
//!
//! Official? | Crate name | Description
//! :--|:--|:--
//! Yes | preon_module_locale | A fast language system, has PreonDesigner integration
//! Yes | preon_module_designer | Loads .preon files, created in PreonDesigner

use std::any::Any;

use components::{PreonComponent, PreonCustomComponentStack};
use events::{PreonEvent, PreonEventEmitter, PreonUserEvent};
use rendering::{PreonRenderPass, PreonStaticRenderData};

use self::types::PreonVector;

/// All default components.
pub mod components;

/// Traits and enums to make your own renderer.
pub mod rendering;

/// Mini event system.
pub mod events;

/// All Preon* utility structs like PreonVector<T>, PreonColor and PreonBox.
pub mod types;

/// Size flags shortcuts.
pub mod size {
    /// Only apply a specific size flag to the X axis.
    pub mod horizontal {
        /// Automatically resize to fit children horizontally.
        pub const FIT: u8 = 0b00000001;

        /// Expand to horizontally fill leftover space in parent.
        pub const EXPAND: u8 = 0b00000010;

        /// Resize to fit children, but expand to available space.
        pub const FIT_EXPAND: u8 = FIT + EXPAND;
    }

    /// Only apply a specific size flag to the Y axis.
    pub mod vertical {
        /// Automatically resize to fit children vertically.
        pub const FIT: u8 = 0b00000100;

        /// Expand to vertically fill leftover space in parent.
        pub const EXPAND: u8 = 0b00001000;

        /// Resize to fit children, but expand to available space.
        pub const FIT_EXPAND: u8 = FIT + EXPAND;
    }

    /// Automatically resize to fit children.
    pub const FIT: u8 = horizontal::FIT + vertical::FIT;

    /// Expand to fill leftover space in parent.
    pub const EXPAND: u8 = horizontal::EXPAND + vertical::EXPAND;

    /// Resize to fit children, but expand to available space.
    pub const FIT_EXPAND: u8 = FIT + EXPAND;
}

/// A container for all variables & functions needed for managing your UI at runtime.
///
/// # Ownership
///
/// PreonEngine tries to make managing ownership of important variables like UI components as easy as possible. This is why you see some variables wrapped in `Option<T>`, it makes it possible to swap the original variable with `None` temporarily, granting you full ownership of the variable until you decide to return it.
///
/// ![like this](https://c.tenor.com/eqLNYv0A9TQAAAAC/swap-indiana-jones.gif)
///
/// For an example of how this works, see [`PreonComponent::get_child()`].
///
/// # Writing modules
///
/// As you may have noticed, there is no `PreonModule` trait or anything like it. With PreonEngine, you have the freedom to implement a module in the way **you** like it. Some utilities have been put in place, as to not throw you completely in the dark by yourself. Communicating with PreonEngine (e.g. telling it the window has been resized) is done through events. There are 2 types of events in PreonEngine:
///
/// - [`PreonEvent`] - Written to by PreonEngine, read by the user.
/// - [`PreonUserEvent`] - Written by the user, read by PreonEngine.
///
/// If your module needs to communicate with PreonEngine, create a function like this:
///
/// ```rust
/// pub struct EpicModule {}
/// impl EpicModule {
/// #   pub fn new() -> Self { Self { } }
///     pub fn update(user_events: &mut PreonEventEmitter<PreonUserEvent>) {
/// #       let has_window_resized = true;
///         // -- snip --
///
///         if has_window_resized {
/// #           let new_size = PreonVector::new(800, 600);
///             user_events.push(PreonUserEvent::WindowResized(new_size));
///         }
///     }
/// }
/// ```
///
/// So the user can do this:
///
/// ```no_run
/// # fn main() {
/// # let mut engine = PreonEngine::<NoComponentStack>::new(
/// # PreonComponentBuilder::new()
/// #   .start_hbox()
/// #       .expand()
/// #       .start_panel()
/// #           .expand()
/// #           .panel_color("#f00")
/// #       .end()
/// #       .start_panel()
/// #           .expand()
/// #           .panel_color("#0f0")
/// #       .end()
/// #       .start_panel()
/// #           .expand()
/// #           .panel_color("#00f")
/// #       .end()
/// #   .end()
/// # .build()
/// # );
/// # let mut user_events = PreonEventEmitter::<PreonUserEvent>::new();
/// // After engine intialization
/// let epic_module = EpicModule::new();
///
/// // In the main loop
/// epic_module.update(&mut user_events);
/// # }
/// ```
pub struct PreonEngine<T: PreonCustomComponentStack> {
    /// The component tree
    pub tree: Option<PreonComponent<T>>,
    /// Will be filled with events after `engine.update()`. See [`PreonEventEmitter`] and [`PreonEvent`]
    pub events: PreonEventEmitter<PreonEvent>,
    /// The size of the viewport, title bar not included.
    pub window_inner_size: PreonVector<u32>,
    /// Pass this to your renderer module of choice after executing `engine.update()`. See [`PreonEventEmitter`] and [`PreonShape`](`rendering::PreonShape`)
    pub render_pass: PreonRenderPass,
    /// Data for StaticLabel and StaticTexture
    pub static_render_data: PreonStaticRenderData,
}

impl<T: PreonCustomComponentStack + Any + 'static> PreonEngine<T> {
    pub fn new(static_render_data: PreonStaticRenderData, tree: PreonComponent<T>) -> Self {
        Self {
            tree: Some(tree),
            events: PreonEventEmitter::new(),
            window_inner_size: PreonVector::zero(),
            render_pass: PreonRenderPass::new(),
            static_render_data,
        }
    }

    pub fn update(&mut self, user_events: &PreonEventEmitter<PreonUserEvent>) -> bool {
        let mut tree = self.tree.take().unwrap();

        let rerender = if user_events.len() > 0 || self.events.len() > 0 {
            let mut update_layout = false;

            user_events.pull(|f| match f {
                PreonUserEvent::WindowResized(s) => {
                    self.resize(s);
                    update_layout = true
                }
                PreonUserEvent::ForceUpdate => update_layout = true,
                PreonUserEvent::WindowOpened => {
                    self.events.push(PreonEvent::WindowOpened);
                    update_layout = true
                }
                PreonUserEvent::WindowClosed => {
                    self.events.push(PreonEvent::WindowClosed);
                }
                _ => {}
            });

            if update_layout {
                tree.set_outer_size(PreonVector::new(
                    self.window_inner_size.x as i32,
                    self.window_inner_size.y as i32,
                ));
                tree.set_outer_position(PreonVector::zero());
                T::layout(&mut tree);

                self.events.push(PreonEvent::LayoutUpdate);

                T::render(&mut tree, &mut self.render_pass);
                self.render_pass.flip();
            }

            self.events.push(PreonEvent::Update);
            self.events.flip();

            true
        } else {
            false
        };

        self.tree = Some(tree);
        rerender
    }

    pub fn resize(&mut self, new_size: PreonVector<u32>) {
        if new_size != self.window_inner_size {
            self.window_inner_size = new_size;
            self.events.push(PreonEvent::WindowResized(new_size));
        }
    }
}
