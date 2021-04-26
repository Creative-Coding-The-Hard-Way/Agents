mod vehicle;

use agents::app::{App, State};
use vehicle::Vehicle;

use anyhow::{Context, Result};
use draw2d::{Graphics, LayerHandle};
use glfw::Window;

struct Demo {
    layer: LayerHandle,
    vehicle: Vehicle,
}

impl Demo {
    fn new(_w: &mut glfw::Window, graphics: &mut Graphics) -> Result<Self> {
        Ok(Self {
            layer: graphics.add_layer_to_top(),
            vehicle: Vehicle::new(),
        })
    }
}

impl State for Demo {
    fn init(&mut self, _w: &mut Window, graphics: &mut Graphics) -> Result<()> {
        let layer = graphics
            .get_layer_mut(&self.layer)
            .with_context(|| "invalid layer handle???")?;
        self.vehicle.draw(layer);
        Ok(())
    }
}

fn main() -> Result<()> {
    App::new(Demo::new)?.main_loop()
}
