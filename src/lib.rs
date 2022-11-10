use gdnative::prelude::*;
use mob::Mob;
use player::Player;
use tester::Tester;

mod mob;
mod player;
mod tester;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Tester>();
    handle.add_class::<Mob>();
}

godot_init!(init);
