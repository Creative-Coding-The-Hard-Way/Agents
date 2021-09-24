use super::Demo;

use anyhow::Result;
use draw2d::graphics::{
    ext::TextureLoader, layer::Batch, texture_atlas::TextureAtlas,
    vertex::Vertex2d, Graphics,
};

impl Demo {
    /// Render the background grid geometry below all other geometry for the
    /// frame.
    pub fn build_background(&self, graphics: &mut Graphics) -> Result<Batch> {
        let mut background = Batch::empty();
        let grid_cell_texture =
            graphics.read_texture_file("./assets/GridCell.png")?;
        background.texture_handle = graphics.add_texture(grid_cell_texture)?;

        let size = 20.0;
        let grid_spacing = 4.0;
        let grid_size = (size * 2.0) / grid_spacing;
        background.vertices.extend_from_slice(&[
            // top left
            Vertex2d {
                pos: [-size, size],
                uv: [0.0, 0.0],
                rgba: [0.2, 0.2, 0.4, 1.0],
                ..Default::default()
            },
            // top right
            Vertex2d {
                pos: [size, size],
                uv: [grid_size, 0.0],
                rgba: [0.2, 0.2, 0.4, 1.0],
                ..Default::default()
            },
            // bottom right
            Vertex2d {
                pos: [size, -size],
                uv: [grid_size, grid_size],
                rgba: [0.2, 0.2, 0.4, 1.0],
                ..Default::default()
            },
            // top left
            Vertex2d {
                pos: [-size, size],
                uv: [0.0, 0.0],
                rgba: [0.2, 0.2, 0.4, 1.0],
                ..Default::default()
            },
            // bottom right
            Vertex2d {
                pos: [size, -size],
                uv: [grid_size, grid_size],
                rgba: [0.2, 0.2, 0.4, 1.0],
                ..Default::default()
            },
            // bottom left
            Vertex2d {
                pos: [-size, -size],
                uv: [0.0, grid_size],
                rgba: [0.2, 0.2, 0.4, 1.0],
            },
        ]);
        Ok(background)
    }
}
