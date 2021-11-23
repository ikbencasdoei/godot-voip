use gdnative::prelude::*;

mod voice_instance;

fn init(handle: InitHandle) {
    handle.add_class::<voice_instance::NativeVoiceInstance>();
}

godot_init!(init);
