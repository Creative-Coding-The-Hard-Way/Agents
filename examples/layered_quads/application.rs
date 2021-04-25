//! The main application state.
//!
//! # Example
//!
//! ```
//! let mut app = Application::new()?;
//! app.run()?;
//! ```

use draw2d::{GlfwWindow, Graphics, Layer, LayerHandle, Vertex};

use anyhow::Result;

/// The main application.
///
/// The Application has a window, a render context, and one or more systems
/// which can render to a frame when presented by the render context.
pub struct Application {
    graphics: Graphics,
    window_surface: GlfwWindow,
    layer: Option<LayerHandle>,
}

impl Application {
    /// Build a new instance of the application.
    pub fn new() -> Result<Self> {
        let mut window_surface = GlfwWindow::windowed("Draw2D", 1366, 768)?;
        window_surface.window.set_resizable(true);
        window_surface.window.set_key_polling(true);
        window_surface.window.set_size_polling(true);
        Ok(Self {
            graphics: Graphics::new(&window_surface)?,
            window_surface,
            layer: None,
        })
    }

    fn init(&mut self) -> Result<()> {
        let texture_handle = self.graphics.add_texture("assets/example.png")?;

        // background
        {
            let layer_handle = self.graphics.add_layer_to_bottom();
            let layer = self.graphics.get_layer_mut(&layer_handle).unwrap();
            layer.set_texture(texture_handle);
            layer.add_square(200.0);
        }

        // foreground
        {
            let layer_handle = self.graphics.add_layer_to_top();
            let layer = self.graphics.get_layer_mut(&layer_handle).unwrap();
            layer.add_square(128.0);
        }

        // (even more) foreground
        {
            let layer_handle = self.graphics.add_layer_to_top();
            self.layer = Some(layer_handle);
            let layer = self.graphics.get_layer_mut(&layer_handle).unwrap();
            layer.set_texture(texture_handle);
            layer.add_square(40.0);
        }

        Ok(())
    }

    fn update(&mut self) {}

    /// Run the application, blocks until the main event loop exits.
    pub fn run(mut self) -> Result<()> {
        self.init()?;
        while !self.window_surface.window.should_close() {
            for (_, event) in self.window_surface.poll_events() {
                self.handle_event(event)?;
            }
            self.update();
            self.graphics.render(&self.window_surface)?;
        }
        Ok(())
    }

    /// Handle window events and update the application state as needed.
    fn handle_event(&mut self, event: glfw::WindowEvent) -> Result<()> {
        match event {
            glfw::WindowEvent::Key(
                glfw::Key::Escape,
                _,
                glfw::Action::Press,
                _,
            ) => {
                self.window_surface.window.set_should_close(true);
            }

            glfw::WindowEvent::FramebufferSize(_, _) => {
                self.graphics.rebuild_swapchain(&self.window_surface)?;
            }

            _ => {}
        }

        Ok(())
    }
}

trait Quads {
    fn add_square(&mut self, size: f32);
}

impl Quads for Layer {
    fn add_square(&mut self, size: f32) {
        self.push_vertices(&[
            // top left
            Vertex {
                pos: [-size, -size],
                uv: [0.0, 0.0],
                ..Default::default()
            },
            // top right
            Vertex {
                pos: [size, -size],
                uv: [1.0, 0.0],
                ..Default::default()
            },
            // bottom right
            Vertex {
                pos: [size, size],
                uv: [1.0, 1.0],
                ..Default::default()
            },
            // top left
            Vertex {
                pos: [-size, -size],
                uv: [0.0, 0.0],
                ..Default::default()
            },
            // bottom right
            Vertex {
                pos: [size, size],
                uv: [1.0, 1.0],
                ..Default::default()
            },
            // bottom left
            Vertex {
                pos: [-size, size],
                uv: [0.0, 1.0],
                ..Default::default()
            },
        ]);
    }
}
