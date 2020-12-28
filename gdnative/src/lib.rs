use gdnative::prelude::*;
type InheritType = Node;

#[derive(NativeClass)]
#[inherit(InheritType)]
struct HelloWorld;

#[gdnative::methods]
impl HelloWorld {
    fn new(_owner: &InheritType) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, _owner: &InheritType) {
        godot_print!("Hello from Rust")
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_init!(init);
