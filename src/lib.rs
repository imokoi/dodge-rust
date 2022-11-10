use gdnative::prelude::*;
use main_sene::MainScene;
use mob::Mob;
use player::Player;
use tester::Tester;

mod main_sene;
mod mob;
mod player;
mod tester;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Tester>();
    handle.add_class::<Mob>();
    handle.add_class::<MainScene>()
}

godot_init!(init);
