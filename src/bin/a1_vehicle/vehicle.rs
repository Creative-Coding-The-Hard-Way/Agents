use draw2d::{Layer, Vertex};

type Vec2 = nalgebra::Vector2<f32>;

pub struct Vehicle {
    pos: Vec2,
    vel: Vec2,
    accel: Vec2,
}

impl Vehicle {
    pub fn new() -> Vehicle {
        Self {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(1.0, 0.0),
            accel: Vec2::new(0.0, 0.0),
        }
    }

    pub fn draw(&self, layer: &mut Layer) {
        const SIZE: f32 = 25.0;

        let look = self.vel.normalize();
        let look_right = Vec2::new(look.y, -look.x);
        let look_left = -look_right;

        let front = self.pos + look * SIZE * 0.5;
        let right = self.pos + (look_right * SIZE * 0.25) + (look * -0.5);
        let left = self.pos + (look_left * SIZE * 0.25) + (look * -0.5);

        log::info!("front {:?}, left {:?}, right {:?}", front, left, right);

        layer.push_vertices(&[
            Vertex {
                pos: front.into(),
                ..Default::default()
            },
            Vertex {
                pos: right.into(),
                ..Default::default()
            },
            Vertex {
                pos: left.into(),
                ..Default::default()
            },
        ]);
    }
}
