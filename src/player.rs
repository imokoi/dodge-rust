use gdnative::{api::*, prelude::*};

#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_signal)]
pub struct Player {
    #[property(default = 200.0)]
    pub speed: f32,
    pub screen_size: Vector2,
}

#[methods]
impl Player {
    fn register_signal(builder: &ClassBuilder<Self>) {
        builder.signal("hit").done()
    }

    fn new(_owner: &Area2D) -> Self {
        Player {
            speed: 200.0,
            screen_size: Vector2::new(0.0, 0.0),
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Area2D>) {
        self.screen_size = base.get_viewport_rect().size;
        base.hide();
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

        let position = Vector2 {
            x: base.global_position().x + velocity.x * self.speed * delta,
            y: base.global_position().y + velocity.y * self.speed * delta,
        };
        let final_position = Vector2 {
            x: position.x.max(0.0).min(self.screen_size.x),
            y: position.y.max(0.0).min(self.screen_size.y),
        };
        base.set_global_position(final_position);
    }

    #[method]
    fn on_area2d_body_entered(&self, #[base] base: TRef<Area2D>, _body: Ref<RigidBody2D>) {
        base.hide();
        base.emit_signal("hit", &[]);

        let collision_shape = unsafe {
            base.get_node_as::<CollisionShape2D>("CollisionShape2D")
                .unwrap()
        };
        collision_shape.set_deferred("disable", true);
    }

    #[method]
    pub fn start(&self, #[base] base: TRef<Area2D>, position: Vector2) {
        base.set_global_position(position);
        base.show();
        let collision_shape = unsafe {
            base.get_node_as::<CollisionShape2D>("CollisionShape2D")
                .unwrap()
        };
        collision_shape.set_disabled(false);
    }
}
