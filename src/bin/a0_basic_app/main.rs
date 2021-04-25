use agents::{App, State};

use anyhow::{Context, Result};
use draw2d::{Graphics, LayerHandle, TextureHandle, Vertex};
use glfw::Window;

struct Demo {
    texture: TextureHandle,
    layer: LayerHandle,
}

impl Demo {
    fn new(_w: &mut glfw::Window, graphics: &mut Graphics) -> Result<Self> {
        Ok(Self {
            texture: graphics.add_texture("assets/example.png")?,
            layer: graphics.add_layer_to_top(),
        })
    }
}

impl State for Demo {
    fn init(&mut self, _w: &mut Window, graphics: &mut Graphics) -> Result<()> {
        let layer = graphics
            .get_layer_mut(&self.layer)
            .with_context(|| "invalid layer handle???")?;
        layer.set_texture(self.texture);

        let size = 200.0;
        layer.push_vertices(&[
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

        Ok(())
    }
}

fn main() -> Result<()> {
    App::new(Demo::new)?.run()
}
