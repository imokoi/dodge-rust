use gdnative::prelude::*;
use player::Player;
use tester::Tester;

mod player;
mod tester;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Tester>();
}

godot_init!(init);
