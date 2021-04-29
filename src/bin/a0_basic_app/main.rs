use agents::app::{App, State};

use anyhow::{Context, Result};
use draw2d::{
    camera::{default_camera_controls, OrthoCamera},
    Graphics, LayerHandle, TextureHandle, Vertex,
};
use glfw::Window;

struct Demo {
    texture: TextureHandle,
    layer: LayerHandle,
    camera: OrthoCamera,
}

impl Demo {
    fn new(window: &mut glfw::Window, graphics: &mut Graphics) -> Result<Self> {
        let (w, h) = window.get_size();
        Ok(Self {
            texture: graphics.add_texture("assets/example.png")?,
            layer: graphics.add_layer_to_top(),
            camera: OrthoCamera::with_viewport(w as f32, w as f32 / h as f32),
        })
    }
}

impl State for Demo {
    fn init(&mut self, _w: &mut Window, graphics: &mut Graphics) -> Result<()> {
        graphics.set_projection(&self.camera.as_matrix());

        let layer = graphics
            .get_layer_mut(&self.layer)
            .with_context(|| "invalid layer handle???")?;
        layer.set_texture(self.texture);

        let size = 200.0;
        layer.push_vertices(&[
            // top left
            Vertex {
                pos: [-size, size],
                uv: [0.0, 0.0],
                ..Default::default()
            },
            // top right
            Vertex {
                pos: [size, size],
                uv: [1.0, 0.0],
                ..Default::default()
            },
            // bottom right
            Vertex {
                pos: [size, -size],
                uv: [1.0, 1.0],
                ..Default::default()
            },
            // top left
            Vertex {
                pos: [-size, size],
                uv: [0.0, 0.0],
                ..Default::default()
            },
            // bottom right
            Vertex {
                pos: [size, -size],
                uv: [1.0, 1.0],
                ..Default::default()
            },
            // bottom left
            Vertex {
                pos: [-size, -size],
                uv: [0.0, 1.0],
                ..Default::default()
            },
        ]);

        Ok(())
    }

    fn handle_event(
        &mut self,
        window_event: &glfw::WindowEvent,
        _window: &mut glfw::Window,
        graphics: &mut draw2d::Graphics,
    ) -> Result<()> {
        if default_camera_controls(&mut self.camera, &window_event) {
            graphics.set_projection(&self.camera.as_matrix());
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    App::new(Demo::new)?.main_loop()
}
