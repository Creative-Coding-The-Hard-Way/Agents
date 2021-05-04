use draw2d::graphics::{layer::Batch, vertex::Vertex2d};

type Vec2 = nalgebra::Vector2<f32>;

pub struct Bounds {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    margin: f32,
}

impl Bounds {
    pub fn create(_window: &glfw::Window) -> Self {
        let half_width = 20.0;
        let half_height = 20.0;

        Self {
            left: -half_width,
            right: half_width,
            bottom: -half_height,
            top: half_height,
            margin: f32::min(half_height, half_width) * 0.05,
        }
    }
}

pub struct Vehicle {
    pos: Vec2,
    vel: Vec2,
    accel: Vec2,
}

impl Vehicle {
    const MAX_FORCE: f32 = 400.0;
    const MAX_VEL: f32 = 15.0;

    pub fn new(pos: [f32; 2], vel: [f32; 2]) -> Vehicle {
        Self {
            pos: pos.into(),
            vel: vel.into(),
            accel: Vec2::new(0.0, 0.0),
        }
    }

    /// Step a vehicle forward in time with an euler integrator.
    ///
    /// # Params
    ///
    /// `dt` is the timestep used for integration. Behavior can become unstable
    /// for large `dt` or for large velocities and accelerations.
    pub fn integrate(&mut self, dt: f32) {
        self.vel = (self.vel + self.accel * dt).limit(Self::MAX_VEL);
        self.pos += self.vel * dt;
        self.accel *= 0.0;
    }

    /// Enforce bounds, don't let the vehicles escape the screen.
    pub fn enforce_bounds(&mut self, bounds: &Bounds) {
        const TIME_TO_ENFORCE: f32 = 0.25;

        if self.pos.x < bounds.left + bounds.margin {
            self.seek_vel(
                Vec2::new(Self::MAX_VEL, self.vel.y).limit(Self::MAX_VEL),
                TIME_TO_ENFORCE,
            );
        } else if self.pos.x > bounds.right - bounds.margin {
            self.seek_vel(
                Vec2::new(-Self::MAX_VEL, self.vel.y).limit(Self::MAX_VEL),
                TIME_TO_ENFORCE,
            );
        }

        if self.pos.y < bounds.bottom + bounds.margin {
            self.seek_vel(
                Vec2::new(self.vel.x, Self::MAX_VEL).limit(Self::MAX_VEL),
                TIME_TO_ENFORCE,
            );
        } else if self.pos.y > bounds.top - bounds.margin {
            self.seek_vel(
                Vec2::new(self.vel.x, -Self::MAX_VEL).limit(Self::MAX_VEL),
                TIME_TO_ENFORCE,
            );
        }
    }

    /// Seek a target velocity.
    ///
    /// # Params
    ///
    /// - `target_vel` is the desired velocity, if not otherwise effected the
    ///   vehicle will eventually reach the target velocity.
    /// - `secs_to_target` is how many seconds (floating point) it should take
    ///   for the vehicle to reach the target velocity.
    pub fn seek_vel(&mut self, target_vel: Vec2, secs_to_target: f32) {
        let error: Vec2 = (target_vel - self.vel).scale(1.0 / secs_to_target);
        self.apply_force(error);
    }

    /// Apply a force to a vehicle.
    ///
    /// Forces are reset to zero after each integration.
    pub fn apply_force(&mut self, force: Vec2) {
        self.accel += force.limit(Self::MAX_FORCE);
    }

    /// Draw the vehicle as a triangle to a single graphics layer.
    pub fn draw(&self, batch: &mut Batch) {
        const SIZE: f32 = 1.0;

        let look = self.vel.normalize();
        let look_right = Vec2::new(look.y, -look.x);
        let look_left = -look_right;

        let front = self.pos + look * SIZE * 0.5;
        let right = self.pos + (look_right * SIZE * 0.15) + (look * -0.5);
        let left = self.pos + (look_left * SIZE * 0.15) + (look * -0.5);

        batch.vertices.extend_from_slice(&[
            Vertex2d {
                pos: front.into(),
                ..Default::default()
            },
            Vertex2d {
                pos: right.into(),
                ..Default::default()
            },
            Vertex2d {
                pos: left.into(),
                ..Default::default()
            },
        ]);
    }
}

trait VecTools {
    fn limit(&self, max: f32) -> Self;
}

impl VecTools for Vec2 {
    fn limit(&self, max: f32) -> Self {
        if self.norm_squared() > max * max {
            self.normalize().scale(max)
        } else {
            *self
        }
    }
}
