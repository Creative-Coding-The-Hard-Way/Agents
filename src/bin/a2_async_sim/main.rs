mod vehicle;

use agents::app::{App, State, UpdateTimer};
use vehicle::{Bounds, Vehicle};

use anyhow::Result;
use draw2d::{
    camera::{default_camera_controls, OrthoCamera},
    Graphics, LayerHandle, Vertex,
};
use std::{
    sync::mpsc::{self, Sender, TryRecvError},
    thread,
    time::{Duration, Instant},
};
use triple_buffer::{Output, TripleBuffer};

struct Sim {
    join_handle: Option<thread::JoinHandle<()>>,
    stop_sender: Sender<()>,
    output_buffer: Output<Vec<Vehicle>>,
}

impl Sim {
    fn new() -> Result<Self> {
        let (stop_sender, stop_receiver) = mpsc::channel();
        let vertex_buffer = TripleBuffer::new(vec![]);
        let (mut input_buffer, output_buffer) = vertex_buffer.split();
        let mut vehicles = vec![];
        let mut timer = UpdateTimer::new("Simulation Update Duration");

        let join_handle = thread::Builder::new()
            .name("simulation thread".to_owned())
            .spawn(move || {
                let max = 10000;
                for i in 0..max {
                    let norm = i as f32 / max as f32;
                    let angle = norm * std::f32::consts::TAU;
                    vehicles.push(Vehicle::new(
                        [angle.cos() * 10.0, angle.sin() * 10.0],
                        [angle.cos() * 2.0, angle.sin() * 2.0],
                    ));
                }
                input_buffer.input_buffer().clear();
                input_buffer.input_buffer().extend_from_slice(&vehicles);
                input_buffer.publish();

                loop {
                    let elapsed = timer.tick();
                    if elapsed < Duration::from_millis(8) {
                        let wait = Duration::from_millis(8) - elapsed;
                        thread::sleep(wait);
                    }

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
                    let dt = 0.008;
                    for vehicle in &mut vehicles {
                        vehicle.enforce_bounds(&bounds);
                        vehicle.integrate(dt);
                    }

                    input_buffer.input_buffer().clear();
                    input_buffer.input_buffer().extend_from_slice(&vehicles);
                    input_buffer.publish();
                }
            })?;

        Ok(Self {
            join_handle: Some(join_handle),
            stop_sender,
            output_buffer,
        })
    }

    fn vehicles(&mut self) -> &[Vehicle] {
        self.output_buffer.read()
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

        let layer = graphics.get_layer_mut(&self.layer).unwrap();
        layer.clear();
        for vehicle in self.sim.vehicles() {
            layer.push_vertices(&vehicle.draw());
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
