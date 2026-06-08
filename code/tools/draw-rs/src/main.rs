mod app;
mod position_traits;
mod prelude;
mod window_state;

use prelude::*;

use winit::event_loop::EventLoop;

pub fn tracing_init() {
    use tracing_subscriber::filter::{EnvFilter, LevelFilter};
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
}

fn main() -> Result<()> {
    tracing_init();

    let event_loop = EventLoop::with_user_event().build()?;
    let _event_loop_proxy = event_loop.create_proxy();

    // Wire the user event from another thread.
    std::thread::spawn(move || {
        // Wake up the `event_loop` once every second and dispatch a custom event
        // from a different thread.
        info!("Starting to send user event every second");
        loop {
            let _ = _event_loop_proxy.send_event(UserEvent::WakeUp);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let mut state = Application::new(&event_loop);

    event_loop.run_app(&mut state).map_err(Into::into)
}
