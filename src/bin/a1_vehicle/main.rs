mod vehicle;

use agents::app::{App, State};
use vehicle::{Bounds, Vehicle};

use anyhow::Result;
use draw2d::{
    camera::{default_camera_controls, OrthoCamera},
    Graphics, LayerHandle, Vertex,
};

use std::time::Duration;

struct Demo {
    layer: LayerHandle,
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
            layer: graphics.add_layer_to_top(),
            vehicles,
            camera: OrthoCamera::with_viewport(20.0, w as f32 / h as f32),
        })
    }
}

impl State for Demo {
    fn init(
        &mut self,
        _window: &mut glfw::Window,
        graphics: &mut draw2d::Graphics,
    ) -> Result<()> {
        graphics.set_projection(&self.camera.as_matrix());

        let background = graphics.add_layer_to_bottom();
        let grid_cell = graphics.add_texture("./assets/GridCell.png")?;

        {
            let bg = graphics.get_layer_mut(&background).unwrap();
            bg.set_texture(grid_cell);

            let size = 20.0;
            let grid_spacing = 2.0;
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
        }

        Ok(())
    }

    fn update(
        &mut self,
        window: &mut glfw::Window,
        graphics: &mut draw2d::Graphics,
        update_duration: Duration,
    ) -> Result<()> {
        graphics.set_projection(&self.camera.as_matrix());
        let layer = graphics.get_layer_mut(&self.layer).unwrap();
        layer.clear();

        let bounds = Bounds::create(window);
        let dt = update_duration.as_secs_f32();
        for vehicle in &mut self.vehicles {
            vehicle.enforce_bounds(&bounds);
            vehicle.integrate(dt);
            vehicle.draw(layer);
        }

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
