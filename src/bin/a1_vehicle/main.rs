mod vehicle;

use agents::app::{App, State};
use vehicle::{Bounds, Vehicle};

use anyhow::Result;
use draw2d::{
    camera::{default_camera_controls, OrthoCamera},
    graphics::{
        layer::{Batch, LayerHandle},
        vertex::Vertex2d,
        Graphics,
    },
};

use std::time::Duration;

struct Demo {
    background_layer: LayerHandle,
    foreground_layer: LayerHandle,
    vehicles: Vec<Vehicle>,
    camera: OrthoCamera,
}

impl Demo {
    fn new(window: &mut glfw::Window, graphics: &mut Graphics) -> Result<Self> {
        let (w, h) = window.get_size();

        let mut vehicles = vec![];

        let max = 10000;
        for i in 0..max {
            let norm = i as f32 / max as f32;
            let angle = norm * std::f32::consts::TAU;
            vehicles.push(Vehicle::new(
                [angle.cos() * 10.0, angle.sin() * 10.0],
                [angle.cos() * 2.0, angle.sin() * 2.0],
            ));
        }
        Ok(Self {
            background_layer: graphics.add_layer_to_bottom(),
            foreground_layer: graphics.add_layer_to_top(),
            vehicles,
            camera: OrthoCamera::with_viewport(20.0, w as f32 / h as f32),
        })
    }

    fn update_projection(&self, graphics: &mut Graphics) {
        graphics
            .get_layer_mut(&self.background_layer)
            .set_projection(self.camera.as_matrix());
        graphics
            .get_layer_mut(&self.foreground_layer)
            .set_projection(self.camera.as_matrix());
    }
}

impl State for Demo {
    fn init(
        &mut self,
        _window: &mut glfw::Window,
        graphics: &mut Graphics,
    ) -> Result<()> {
        self.update_projection(graphics);

        let mut background = Batch::empty();
        background.texture_handle =
            graphics.add_texture("./assets/GridCell.png")?;

        let size = 20.0;
        let grid_spacing = 2.0;
        let grid_size = (size * 2.0) / grid_spacing;
        background.vertices.extend_from_slice(&[
            // top left
            Vertex2d {
                pos: [-size, size],
                uv: [0.0, 0.0],
                rgba: [0.2, 0.2, 0.4, 1.0],
            },
            // top right
            Vertex2d {
                pos: [size, size],
                uv: [grid_size, 0.0],
                rgba: [0.2, 0.2, 0.4, 1.0],
            },
            // bottom right
            Vertex2d {
                pos: [size, -size],
                uv: [grid_size, grid_size],
                rgba: [0.2, 0.2, 0.4, 1.0],
            },
            // top left
            Vertex2d {
                pos: [-size, size],
                uv: [0.0, 0.0],
                rgba: [0.2, 0.2, 0.4, 1.0],
            },
            // bottom right
            Vertex2d {
                pos: [size, -size],
                uv: [grid_size, grid_size],
                rgba: [0.2, 0.2, 0.4, 1.0],
            },
            // bottom left
            Vertex2d {
                pos: [-size, -size],
                uv: [0.0, grid_size],
                rgba: [0.2, 0.2, 0.4, 1.0],
            },
        ]);

        graphics
            .get_layer_mut(&self.background_layer)
            .push_batch(background);

        Ok(())
    }

    fn update(
        &mut self,
        window: &mut glfw::Window,
        graphics: &mut Graphics,
        update_duration: Duration,
    ) -> Result<()> {
        let mut vehicle_batch = Batch::empty();
        let bounds = Bounds::create(window);
        let dt = update_duration.as_secs_f32();
        for vehicle in &mut self.vehicles {
            vehicle.enforce_bounds(&bounds);
            vehicle.integrate(dt);
            vehicle.draw(&mut vehicle_batch);
        }

        let layer = graphics.get_layer_mut(&self.foreground_layer);
        layer.clear();
        layer.push_batch(vehicle_batch);

        Ok(())
    }

    fn handle_event(
        &mut self,
        window_event: &glfw::WindowEvent,
        _window: &mut glfw::Window,
        graphics: &mut Graphics,
    ) -> Result<()> {
        if default_camera_controls(&mut self.camera, &window_event) {
            self.update_projection(graphics);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    App::new(Demo::new)?.main_loop()
}
