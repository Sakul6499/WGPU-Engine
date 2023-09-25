use engine::{
    app::{App, World},
    entities::{ClearScreenEntity, Cube},
};
use winit::event_loop::EventLoop;

pub fn game_init(event_loop: EventLoop<()>) {
    let mut world = World::new();

    world.add_entity(Box::new(Cube::default()));
    world.add_entity(Box::new(ClearScreenEntity {}));

    App::run("Game", event_loop, world).expect("App failed");
}
