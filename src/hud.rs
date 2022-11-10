use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
#[register_with(Self::register_signal)]
pub struct Hud;

#[methods]
impl Hud {
    fn register_signal(builder: &ClassBuilder<Self>) {
        builder.signal("start_game").done();
    }

    fn new(_base: &CanvasLayer) -> Self {
        Hud
    }

    #[method]
    pub fn show_message(&self, #[base] base: TRef<CanvasLayer>, text: String) {
        let message_label = unsafe { base.get_node_as::<Label>("Message").unwrap() };
        message_label.set_text(text);
        message_label.show();

        let message_timer = unsafe { base.get_node_as::<Timer>("MessageTimer").unwrap() };
        message_timer.start(0.0);
    }

    #[method]
    pub fn show_game_over(&self, #[base] base: TRef<CanvasLayer>) {
        self.show_message(base, "Game Over".into());

        let message_label = unsafe { base.get_node_as::<Label>("Message").unwrap() };
        message_label.set_text("Dodge the\nCreeps!");
        message_label.show();

        let start_button = unsafe { base.get_node_as::<Button>("StartButton").unwrap() };
        start_button.show();
    }

    #[method]
    pub fn update_score(&self, #[base] base: TRef<CanvasLayer>, score: i64) {
        let score_label = unsafe { base.get_node_as::<Label>("ScoreLabel").unwrap() };
        score_label.set_text(score.to_string());
    }

    #[method]
    fn on_start_button_pressed(&self, #[base] base: TRef<CanvasLayer>) {
        let button = unsafe { base.get_node_as::<Button>("StartButton").unwrap() };
        button.hide();
        base.emit_signal("start_game", &[]);
    }

    #[method]
    fn on_message_timer_timeout(&self, #[base] base: TRef<CanvasLayer>) {
        let message_label = unsafe { base.get_node_as::<Label>("Message").unwrap() };
        message_label.hide();
    }
}
