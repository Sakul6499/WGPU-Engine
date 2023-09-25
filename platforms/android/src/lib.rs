use log::LevelFilter;
use winit::{
    event_loop::{EventLoop, EventLoopBuilder},
    platform::android::{
        activity::{AndroidApp, MainEvent, PollEvent},
        EventLoopBuilderExtAndroid,
    },
    window::{self, Window},
};

use game::game_init;

#[no_mangle]
fn android_main(app: AndroidApp) {
    #[cfg(debug_assertions)]
    android_logger::init_once(android_logger::Config::default().with_max_level(LevelFilter::Trace));
    #[cfg(not(debug_assertions))]
    android_logger::init_once(android_logger::Config::default().with_max_level(LevelFilter::Info));

    let event_loop: EventLoop<()> = EventLoopBuilder::with_user_event()
        .with_android_app(app)
        .build();

    // let window = Window::new(&event_loop).expect("Window failure");

    // event_loop.run(|x, y, z| {
    //     log::debug!("Event: {:?}", x);
    // });

    game_init(event_loop);
}
