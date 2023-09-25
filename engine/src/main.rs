use winit::event_loop::EventLoop;

use wgpu_engine::{
    app::{App, World},
    entities::{ClearScreenEntity, Cube},
};

#[cfg(not(target_os = "android"))]
use wgpu_engine::log::log_init;

#[cfg(target_os = "android")]
use winit::{
    event_loop::EventLoopBuilder,
    platform::android::{activity::AndroidApp, EventLoopBuilderExtAndroid},
};

fn _main(event_loop: EventLoop<()>) {
    let mut world = World::new();

    world.add_entity(Box::new(Cube::default()));
    world.add_entity(Box::new(ClearScreenEntity {}));

    App::run("WGPU", event_loop, world).expect("App failed");
}

#[cfg(not(target_os = "android"))]
fn main() {
    log_init();

    let event_loop = EventLoop::new();
    _main(event_loop);
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use log::LevelFilter;

    #[cfg(debug_assertions)]
    android_logger::init_once(android_logger::Config::default().with_max_level(LevelFilter::Trace));
    #[cfg(not(debug_assertions))]
    android_logger::init_once(android_logger::Config::default().with_max_level(LevelFilter::Info));

    let event_loop = EventLoopBuilder::with_user_event()
        .with_android_app(app)
        .build();
    _main(event_loop);
}
