use super::Demo;

use anyhow::Result;
use draw2d::{Graphics, Vertex};

impl Demo {
    /// Render the background grid geometry below all other geometry for the
    /// frame.
    pub fn build_background_layer(
        &self,
        graphics: &mut Graphics,
    ) -> Result<()> {
        let background = graphics.add_layer_to_bottom();
        let grid_cell = graphics.add_texture("./assets/GridCell.png")?;

        let bg = graphics.get_layer_mut(&background).unwrap();
        bg.set_texture(grid_cell);

        let size = 20.0;
        let grid_spacing = 4.0;
        let grid_size = (size * 2.0) / grid_spacing;
        bg.push_vertices(&[
            // top left
            Vertex {
                pos: [-size, size],
                uv: [0.0, 0.0],
                rgba: [0.2, 0.2, 0.4, 1.0],
                ..Default::default()
            },
            // top right
            Vertex {
                pos: [size, size],
                uv: [grid_size, 0.0],
                rgba: [0.2, 0.2, 0.4, 1.0],
                ..Default::default()
            },
            // bottom right
            Vertex {
                pos: [size, -size],
                uv: [grid_size, grid_size],
                rgba: [0.2, 0.2, 0.4, 1.0],
                ..Default::default()
            },
            // top left
            Vertex {
                pos: [-size, size],
                uv: [0.0, 0.0],
                rgba: [0.2, 0.2, 0.4, 1.0],
                ..Default::default()
            },
            // bottom right
            Vertex {
                pos: [size, -size],
                uv: [grid_size, grid_size],
                rgba: [0.2, 0.2, 0.4, 1.0],
                ..Default::default()
            },
            // bottom left
            Vertex {
                pos: [-size, -size],
                uv: [0.0, grid_size],
                rgba: [0.2, 0.2, 0.4, 1.0],
            },
        ]);

        Ok(())
    }
}
