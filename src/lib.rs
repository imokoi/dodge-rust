use gdnative::prelude::*;
use hud::Hud;
use main_sene::MainScene;
use mob::Mob;
use player::Player;

mod hud;
mod main_sene;
mod mob;
mod player;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Mob>();
    handle.add_class::<MainScene>();
    handle.add_class::<Hud>();
}

godot_init!(init);
