use anyhow::Result;
use draw2d::graphics::{layer::Batch, vertex::Vertex2d, Graphics};

pub struct World {
    pub size: f32,
    pub max_vel: f32,
    pub max_accel: f32,
    pub damping: f32,
    pub grid_cell_size: f32,
    pub color: [f32; 4],
}

impl World {
    pub fn build_render_batch(&self, graphics: &mut Graphics) -> Result<Batch> {
        let mut batch = Batch::empty();
        batch.texture_handle = graphics.add_texture("./assets/GridCell.png")?;

        let half_size = self.size / 2.0;
        let half_uv = half_size / self.grid_cell_size;
        batch.vertices = vec![
            // top left
            Vertex2d {
                pos: [-half_size, half_size],
                uv: [-half_uv, half_uv],
                rgba: self.color,
            },
            // top right
            Vertex2d {
                pos: [half_size, half_size],
                uv: [half_uv, half_uv],
                rgba: self.color,
            },
            // bottom right
            Vertex2d {
                pos: [half_size, -half_size],
                uv: [half_uv, -half_uv],
                rgba: self.color,
            },
            // top left
            Vertex2d {
                pos: [-half_size, half_size],
                uv: [-half_uv, half_uv],
                rgba: self.color,
            },
            // bottom right
            Vertex2d {
                pos: [half_size, -half_size],
                uv: [half_uv, -half_uv],
                rgba: self.color,
            },
            // bottom left
            Vertex2d {
                pos: [-half_size, -half_size],
                uv: [-half_uv, -half_uv],
                rgba: self.color,
            },
        ];

        Ok(batch)
    }
}
