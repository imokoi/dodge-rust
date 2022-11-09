use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Tester;

#[methods]
impl Tester {
    pub fn new(base: &Node) -> Self {
        Tester
    }

    #[method]
    pub fn say_hello(&self) {
        godot_print!("Hello hello from Node!");
    }
}
