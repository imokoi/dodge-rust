use gdnative::api::*;
use gdnative::prelude::*;

use crate::player::Player;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct MainScene {
    #[property]
    mob: Ref<PackedScene>,
    score: i64,
}

#[methods]
impl MainScene {
    fn new(_base: &Node) -> Self {
        MainScene {
            mob: PackedScene::new().into_shared(),
            score: 0,
        }
    }

    #[method]
    fn new_game(&mut self, #[base] base: TRef<Node>) {
        let start_position = unsafe { base.get_node_as::<Position2D>("StartPosition").unwrap() };
        let player = unsafe { base.get_node_as_instance::<Player>("Player").unwrap() };
        player
            .map(|p, base| p.start(base, start_position.position()))
            .ok()
            .unwrap_or_else(|| godot_print!("failed to get palyer"));

        let start_timer = unsafe {
            base.get_node_as::<Timer>("StartTimer").unwrap()
        };
        start_timer.start(0.0);
        self.score = 0;
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Node>) {
        godot_print!("hello main");
        self.new_game(base);
    }
}
