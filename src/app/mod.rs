//! The application structure and supporting traits.
//!
//! # Example
//!
//! ```rust,no_run
//! use agents::app::{App, State};
//! use anyhow::{Context, Result};
//! use draw2d::{Graphics, LayerHandle, TextureHandle, Vertex};
//! use glfw::Window;
//!
//! /// Every binary will have a structure of some sort.
//! struct Example {
//!   layer: LayerHandle
//! }
//!
//! /// The structure can have whatever implementation functions it needs.
//! impl Example {
//!
//!   /// The constructor has access to both the window and graphics.
//!   pub fn create(
//!     window: &mut glfw::Window,
//!     graphics: &mut draw2d::Graphics
//!   ) -> Result<Self> {
//!     Ok(Self {
//!       layer: graphics.add_layer_to_top()
//!     })
//!   }
//! }
//!
//! /// The example must implement State to be used with an application.
//! /// All methods have defaults, so none are *required* for a bare minimum
//! /// working example.
//! impl State for Example {
//!   fn update(&mut self, _: &mut Window, g: &mut Graphics) -> Result<()> {
//!     let layer = g.get_layer_mut(&self.layer).unwrap();
//!     layer.push_vertices(&[
//!         Vertex {
//!             pos: [-100.0, -100.0],
//!             ..Default::default()
//!         },
//!         Vertex {
//!             pos: [100.0, -100.0],
//!             ..Default::default()
//!         },
//!         Vertex {
//!             pos: [0.0, 100.0],
//!             rgba: [0.1, 0.2, 0.8, 1.0],
//!             ..Default::default()
//!         },
//!     ]);
//!     Ok(())
//!   }
//! }
//!
//! /// Finally, start the application and let it run until the user quits.
//! fn main() -> Result<()> {
//!   App::new(Example::create)?.main_loop()
//! }
//! ```

mod app;

use anyhow::Result;
use draw2d::{GlfwWindow, Graphics};
use glfw::{Action, Key, WindowEvent};

/// Each application maintains an instance of State which controls the actual
/// behavior and rendering.
pub trait State {
    /// Invoked by the application once just before the first update/render.
    ///
    /// This can be useful if any initialization needs to be done before
    /// rendering but *after* constructing the instance for some reason. Or if
    /// the application has need to setup some visuals which never
    /// change.
    #[allow(unused)]
    fn init(
        &mut self,
        window: &mut glfw::Window,
        graphics: &mut draw2d::Graphics,
    ) -> Result<()> {
        Ok(())
    }

    /// Called once each frame before presenting the framebuffer.
    #[allow(unused)]
    fn update(
        &mut self,
        window: &mut glfw::Window,
        graphics: &mut draw2d::Graphics,
    ) -> Result<()> {
        Ok(())
    }

    /// Handle a glfw window event.
    ///
    /// The application can be stopped by calling `window.set_should_close`.
    /// By default, the application will close when `esc` is pressed.
    #[allow(unused)]
    fn handle_event(
        &mut self,
        window_event: &glfw::WindowEvent,
        window: &mut glfw::Window,
        graphics: &mut draw2d::Graphics,
    ) -> Result<()> {
        match window_event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

/// The main application.
///
/// This struct owns the window, the graphics subsystem, and the state. It is
/// responsible for the main application loop, rebuilding the swapchain, and
/// invoking functions on the State.
pub struct App<S: State> {
    graphics: Graphics,
    window_surface: GlfwWindow,
    state: S,
}
