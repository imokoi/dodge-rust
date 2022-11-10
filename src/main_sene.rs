use std::f64::consts::PI;

use gdnative::api::*;
use gdnative::prelude::*;
use rand::Rng;

use crate::hud::Hud;
use crate::mob::Mob;
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
    fn game_over(&self, #[base] base: TRef<Node>) {
        let score_timer = unsafe { base.get_node_as::<Timer>("ScoreTimer").unwrap() };
        score_timer.stop();
        let timer = unsafe { base.get_node_as::<Timer>("MobTimer").unwrap() };
        timer.stop();

        let hud = unsafe { base.get_node_as_instance::<Hud>("HUD").unwrap() };
        hud.map(|hud, base| hud.show_game_over(base))
            .ok()
            .unwrap_or_else(|| {
                godot_error!("Failed to get HUD instance");
            });
    }

    #[method]
    fn new_game(&mut self, #[base] base: TRef<Node>) {
        let start_position = unsafe { base.get_node_as::<Position2D>("StartPosition").unwrap() };
        let player = unsafe { base.get_node_as_instance::<Player>("Player").unwrap() };
        player
            .map(|p, base| p.start(base, start_position.position()))
            .ok()
            .unwrap_or_else(|| godot_print!("failed to get palyer"));

        let start_timer = unsafe { base.get_node_as::<Timer>("StartTimer").unwrap() };
        start_timer.start(0.0);
        self.score = 0;

        let hud = unsafe { base.get_node_as_instance::<Hud>("HUD").unwrap() };
        hud.map(|h, base| {
            h.update_score(base, self.score);
            h.show_message(base, "Get Ready".into());
        })
        .ok()
        .unwrap_or_else(|| godot_print!("failed to update score"));
    }

    #[method]
    fn on_start_timer_timeout(&self, #[base] base: TRef<Node>) {
        // Start score time, so the score will be changed per second.
        let score_timer = unsafe { base.get_node_as::<Timer>("ScoreTimer").unwrap() };
        score_timer.start(0.0);

        // start the mob timer to spawn mobs.
        let mob_timer = unsafe { base.get_node_as::<Timer>("MobTimer").unwrap() };
        mob_timer.start(0.0);
    }

    #[method]
    fn on_score_timer_timeout(&mut self, #[base] base: TRef<Node>) {
        self.score += 1;
        let hud = unsafe { base.get_node_as_instance::<Hud>("HUD").unwrap() };
        hud.map(|h, base| {
            h.update_score(base, self.score);
        })
        .ok()
        .unwrap_or_else(|| godot_print!("failed to update score"));
    }

    #[method]
    fn on_mob_timer_timeout(&self, #[base] base: TRef<Node>) {
        let mob_spawn_location = unsafe {
            base.get_node_as::<PathFollow2D>("Path2D/MobSpawnLocation")
                .unwrap()
        };
        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(u32::MIN..u32::MAX);
        mob_spawn_location.set_offset(offset.into());

        let mob_scene: Ref<RigidBody2D, _> = instance_scene(&self.mob);
        let mut direction = mob_spawn_location.rotation() + PI / 2.0;
        mob_scene.set_position(mob_spawn_location.position());
        direction += rng.gen_range(-PI / 4.0..PI / 4.0);
        mob_scene.set_rotation(direction);
        let d = direction as f32;

        let mob_scene = unsafe { mob_scene.into_shared().assume_safe() };
        base.add_child(mob_scene, false);
        let mob = mob_scene.cast_instance::<Mob>().unwrap();

        mob.map(|m, base| {
            let velocity = Vector2::new(rng.gen_range(m.min_speed..m.max_speed), 0.0);
            base.set_linear_velocity(velocity);
            base.set_linear_velocity(base.linear_velocity().rotated(d));
        })
        .ok()
        .expect("faield");
    }
}

fn instance_scene<Root>(scene: &Ref<PackedScene, Shared>) -> Ref<Root, Unique>
where
    Root: GodotObject<Memory = ManuallyManaged> + SubClass<Node>,
{
    let scene = unsafe { scene.assume_safe() };
    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .expect("failed to instance a packed scene");
    let instance = unsafe { instance.assume_unique() };
    instance
        .try_cast::<Root>()
        .expect("root node is not correct.")
}
