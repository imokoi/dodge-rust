use gdnative::{api::*, prelude::*};

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Player {
    #[property]
    pub speed: f32,
    pub screen_size: Vector2,
}

#[methods]
impl Player {
    fn new(_owner: &Area2D) -> Self {
        Player {
            speed: 250.0,
            screen_size: Vector2::new(0.0, 0.0),
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Area2D>) {
        godot_print!("Hello, base!");
        self.screen_size = base.get_viewport_rect().size;
    }

    #[method]
    fn _process(&self, #[base] base: TRef<Area2D>, delta: f32) {
        let mut velocity = Vector2::new(0.0, 0.0);

        if Input::godot_singleton().is_action_pressed("ui_right", true) {
            velocity.x += 1.0;
        }
        if Input::godot_singleton().is_action_pressed("ui_left", true) {
            velocity.x -= 1.0;
        }
        if Input::godot_singleton().is_action_pressed("ui_down", true) {
            velocity.y += 1.0;
        }
        if Input::godot_singleton().is_action_pressed("ui_up", true) {
            velocity.y -= 1.0;
        }

        let animated_sprite = unsafe {
            base.get_node_as::<AnimatedSprite>("AnimatedSprite")
                .unwrap()
        };
        if velocity.length() > 0.0 {
            let animation;
            if velocity.x != 0.0 {
                animation = "walk";
                animated_sprite.set_flip_h(velocity.x < 0.0);
            } else {
                animation = "up";
                animated_sprite.set_flip_v(velocity.y > 0.0);
            }
            animated_sprite.play(animation, false);
        } else {
            animated_sprite.stop();
        }

        base.set_global_position(Vector2 {
            x: base.global_position().x + velocity.x * self.speed * delta,
            y: base.global_position().y + velocity.y * self.speed * delta,
        })
    }
}
