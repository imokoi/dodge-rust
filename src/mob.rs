use gdnative::api::*;
use gdnative::prelude::*;
use rand;
use rand::Rng;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Mob {
    #[property(default = 150.0)]
    pub min_speed: f32,
    #[property(default = 250.0)]
    pub max_speed: f32,
}

#[derive(Debug, Clone)]
enum MobType {
    Walk,
    Swim,
    Fly,
}

impl MobType {
    fn to_string(self) -> String {
        match self {
            Self::Walk => "walk".to_string(),
            Self::Swim => "swim".to_string(),
            Self::Fly => "fly".to_string(),
        }
    }
}

const MOB_TYPES: [MobType; 3] = [MobType::Walk, MobType::Swim, MobType::Fly];

#[methods]
impl Mob {
    fn new(_base: &RigidBody2D) -> Self {
        Mob {
            min_speed: 150.0,
            max_speed: 250.0,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<RigidBody2D>) {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..MOB_TYPES.len());
        let animated_sprite = unsafe {
            base.get_node_as::<AnimatedSprite>("AnimatedSprite")
                .unwrap()
        };
        animated_sprite.set_animation(MOB_TYPES[random_number].clone().to_string());
    }

    #[method]
    fn on_visiable_screen_exited(&self, #[base] base: TRef<RigidBody2D>) {
        unsafe {
            base.assume_unique().queue_free();
        }
    }

    #[method]
    fn on_start_game(&self, #[base] base: TRef<RigidBody2D>) {
        unsafe {
            base.assume_unique().queue_free();
        }
    }
}
