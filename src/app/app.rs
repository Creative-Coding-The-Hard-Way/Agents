use crate::app::UpdateTimer;

use super::{App, State};

use anyhow::Result;
use draw2d::{graphics::Graphics, GlfwWindow};
use flexi_logger::{DeferredNow, Logger, Record};
use glfw::WindowEvent;
use std::fmt::Write as FmtWrite;
use textwrap::{termwidth, Options};

impl<S: State> App<S> {
    /// Build a new instance of the application.
    ///
    /// `build_state` is a function which constructs an instance of the
    /// application's state type.
    ///
    pub fn new(
        build_state: fn(&mut glfw::Window, &mut Graphics) -> Result<S>,
    ) -> Result<Self> {
        Logger::with_env_or_str("info")
            .format(multiline_format)
            .start()?;

        log::info!("adjust log level by setting the RUST_LOG='info'");

        let mut window_surface = GlfwWindow::windowed("Agents", 1366, 768)?;
        let mut graphics = Graphics::new(&window_surface)?;
        window_surface.window.set_resizable(true);
        window_surface.window.set_key_polling(true);
        window_surface.window.set_size_polling(true);
        window_surface.window.set_scroll_polling(true);

        let state = build_state(&mut window_surface.window, &mut graphics)?;

        Ok(Self {
            graphics,
            window_surface,
            update_timer: UpdateTimer::new("Render Duration"),
            state,
        })
    }

    /// The application's main loop.
    ///
    /// This method will block until the user kills the application or until
    /// the State implementation calls `window.set_should_close(true)`.
    pub fn main_loop(mut self) -> Result<()> {
        // initialize the app's state
        self.state
            .init(&mut self.window_surface.window, &mut self.graphics)?;

        while !self.window_surface.window.should_close() {
            for (_, event) in self.window_surface.poll_events() {
                self.handle_event(event)?;
            }
            let duration = self.update_timer.tick();
            self.state.update(
                &mut self.window_surface.window,
                &mut self.graphics,
                duration,
            )?;
            self.graphics.render(&self.window_surface)?;
        }
        Ok(())
    }

    /// Handle window events and update the application state as needed.
    fn handle_event(&mut self, event: glfw::WindowEvent) -> Result<()> {
        match event {
            WindowEvent::FramebufferSize(_, _) => {
                self.graphics.rebuild_swapchain(&self.window_surface)?;
            }
            _ => {}
        }
        self.state.handle_event(
            &event,
            &mut self.window_surface.window,
            &mut self.graphics,
        )
    }
}

fn multiline_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let size = termwidth().min(74);
    let wrap_options = Options::new(size)
        .initial_indent("┏ ")
        .subsequent_indent("┃ ");

    let mut full_line = String::new();
    writeln!(
        full_line,
        "{} [{}] [{}:{}]",
        record.level(),
        now.now().format("%H:%M:%S%.6f"),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
    )
    .expect("unable to format first log line");

    write!(&mut full_line, "{}", &record.args())
        .expect("unable to format log!");

    writeln!(w, "{}", textwrap::fill(&full_line, wrap_options))
}
