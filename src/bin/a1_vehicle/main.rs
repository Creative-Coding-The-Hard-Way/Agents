mod vehicle;

use agents::app::{App, State};
use vehicle::{Bounds, Vehicle};

use anyhow::Result;
use draw2d::{Graphics, LayerHandle};

use std::time::Duration;

struct Demo {
    layer: LayerHandle,
    vehicles: Vec<Vehicle>,
}

impl Demo {
    fn new(_w: &mut glfw::Window, graphics: &mut Graphics) -> Result<Self> {
        let mut vehicles = vec![];

        let max = 2000;
        for i in 0..max {
            let norm = i as f32 / max as f32;
            let angle = norm * std::f32::consts::TAU;
            vehicles.push(Vehicle::new(
                [angle.cos() * 200.0, angle.sin() * 200.0],
                [
                    -angle.sin() * (200.0 * (angle * 4.0).cos()),
                    angle.cos() * (200.0 * (angle * 4.0).cos()),
                ],
            ));
        }
        Ok(Self {
            layer: graphics.add_layer_to_top(),
            vehicles,
        })
    }
}

impl State for Demo {
    fn update(
        &mut self,
        window: &mut glfw::Window,
        graphics: &mut draw2d::Graphics,
        update_duration: Duration,
    ) -> Result<()> {
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
}

fn main() -> Result<()> {
    App::new(Demo::new)?.main_loop()
}
