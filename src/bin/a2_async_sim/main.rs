mod vehicle;

use agents::app::{App, State};
use vehicle::{Bounds, Vehicle};

use anyhow::Result;
use draw2d::{
    camera::{default_camera_controls, OrthoCamera},
    Graphics, LayerHandle, Vertex,
};

use std::{
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
    thread,
    time::{Duration, Instant},
};

struct Sim {
    join_handle: Option<thread::JoinHandle<()>>,
    stop_sender: Sender<()>,
    update_reciever: Receiver<Vec<Vehicle>>,
}

impl Sim {
    fn new() -> Result<Self> {
        let (stop_sender, stop_receiver) = mpsc::channel();
        let (update_sender, update_reciever) = mpsc::channel();

        let join_handle = thread::Builder::new()
            .name("simulation thread".to_owned())
            .spawn(move || {
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
                update_sender.send(vehicles.clone()).unwrap();

                let mut last_update = Instant::now();
                loop {
                    let wait = Duration::from_millis(16)
                        - (Instant::now() - last_update);
                    thread::sleep(wait);
                    last_update = Instant::now();

                    match stop_receiver.try_recv() {
                        Ok(_) | Err(TryRecvError::Disconnected) => {
                            log::info!("terminating the simulation loop");
                            break;
                        }
                        _ => {}
                    }

                    let bounds = Bounds {
                        left: -20.0,
                        right: 20.0,
                        bottom: -20.0,
                        top: 20.0,
                        margin: 0.5,
                    };
                    let dt = 0.1;
                    for vehicle in &mut vehicles {
                        vehicle.enforce_bounds(&bounds);
                        vehicle.integrate(dt);
                    }

                    update_sender.send(vehicles.clone()).unwrap();
                }
            })?;

        Ok(Self {
            join_handle: Some(join_handle),
            stop_sender,
            update_reciever,
        })
    }

    fn try_get_next_tick(&self) -> Result<Vec<Vehicle>, TryRecvError> {
        self.update_reciever.try_recv()
    }
}

impl Drop for Sim {
    fn drop(&mut self) {
        log::info!("waiting for sim to drop");
        self.stop_sender
            .send(())
            .expect("unable to send a stop signal to the simulation thread");
        self.join_handle
            .take()
            .unwrap()
            .join()
            .expect("unable to join the simulation thread");
    }
}

struct Demo {
    layer: LayerHandle,
    camera: OrthoCamera,
    sim: Sim,
}

impl Demo {
    fn new(window: &mut glfw::Window, graphics: &mut Graphics) -> Result<Self> {
        let (w, h) = window.get_size();
        Ok(Self {
            layer: graphics.add_layer_to_top(),
            camera: OrthoCamera::with_viewport(20.0, w as f32 / h as f32),
            sim: Sim::new()?,
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
        _window: &mut glfw::Window,
        graphics: &mut draw2d::Graphics,
        _update_duration: Duration,
    ) -> Result<()> {
        graphics.set_projection(&self.camera.as_matrix());

        match self.sim.try_get_next_tick() {
            Ok(vehicles) => {
                let layer = graphics.get_layer_mut(&self.layer).unwrap();
                layer.clear();
                for vehicle in vehicles {
                    vehicle.draw(layer);
                }
                Ok(())
            }

            Err(TryRecvError::Empty) => Ok(()),
            Err(TryRecvError::Disconnected) => {
                anyhow::bail!("simulation panicked!")
            }
        }
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
