mod background;
mod vehicle;

use agents::{
    app::{App, State},
    simulation::{Simulation, Worker},
};
use vehicle::{Bounds, Vehicle};

use anyhow::Result;
use draw2d::{
    camera::{default_camera_controls, OrthoCamera},
    graphics::{
        layer::{Batch, LayerHandle},
        Graphics,
    },
};
use std::time::Duration;
use triple_buffer::Input;

struct VehicleWorld {
    vehicles: Vec<Vehicle>,
}

impl VehicleWorld {
    fn new() -> Self {
        Self { vehicles: vec![] }
    }

    fn flush(&self, sync: &mut Input<Vec<Vehicle>>) {
        sync.input_buffer().clear();
        sync.input_buffer().extend_from_slice(&self.vehicles);
        sync.publish();
    }
}

impl Simulation for VehicleWorld {
    type SyncState = Vec<Vehicle>;

    fn setup(&mut self, sync: &mut Input<Self::SyncState>) {
        let max = 10000;
        for i in 0..max {
            let norm = i as f32 / max as f32;
            let angle = norm * std::f32::consts::TAU;
            self.vehicles.push(Vehicle::new(
                [angle.cos() * 10.0, angle.sin() * 10.0],
                [angle.cos() * 2.0, angle.sin() * 2.0],
            ));
        }
        self.flush(sync);
    }

    fn tick(&mut self, sync: &mut Input<Self::SyncState>, _: Duration) {
        let bounds = Bounds {
            left: -20.0,
            right: 20.0,
            bottom: -20.0,
            top: 20.0,
            margin: 0.5,
        };

        let dt = Self::TICK_THROTTLE.as_secs_f32();
        for vehicle in &mut self.vehicles {
            vehicle.enforce_bounds(&bounds);
            vehicle.integrate(dt);
        }
        self.flush(sync);
    }
}

struct Demo {
    background_layer: LayerHandle,
    foreground_layer: LayerHandle,
    camera: OrthoCamera,
    sim: Worker<VehicleWorld>,
}

impl Demo {
    fn new(window: &mut glfw::Window, graphics: &mut Graphics) -> Result<Self> {
        let (w, h) = window.get_size();
        Ok(Self {
            background_layer: graphics.add_layer_to_bottom(),
            foreground_layer: graphics.add_layer_to_top(),
            camera: OrthoCamera::with_viewport(20.0, w as f32 / h as f32),
            sim: Worker::new(VehicleWorld::new)?,
        })
    }

    fn update_projection(&self, graphics: &mut Graphics) {
        let matrix = self.camera.as_matrix();
        graphics
            .get_layer_mut(&self.background_layer)
            .set_projection(matrix);
        graphics
            .get_layer_mut(&self.foreground_layer)
            .set_projection(matrix);
    }
}

impl State for Demo {
    fn init(
        &mut self,
        _window: &mut glfw::Window,
        graphics: &mut Graphics,
    ) -> Result<()> {
        self.update_projection(graphics);
        let background = self.build_background(graphics)?;
        graphics
            .get_layer_mut(&self.background_layer)
            .push_batch(background);
        Ok(())
    }

    fn update(
        &mut self,
        _window: &mut glfw::Window,
        graphics: &mut Graphics,
        _update_duration: Duration,
    ) -> Result<()> {
        let vehicles = self.sim.state();

        let mut vehicle_batch = Batch::empty();
        vehicle_batch.vertices.reserve(vehicles.len() * 3);

        for vehicle in self.sim.state() {
            vehicle_batch.vertices.extend_from_slice(&vehicle.draw());
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
