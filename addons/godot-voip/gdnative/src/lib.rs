use gdnative::{
    init::{
        godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init, InitializeInfo,
        TerminateInfo,
    },
    prelude::*,
};

mod voice_instance;

fn init(handle: InitHandle) {
    handle.add_class::<voice_instance::NativeVoiceInstance>();
}

fn godot_gdnative_init_empty(_options: &InitializeInfo) {}
fn godot_gdnative_terminate_empty(_term_info: &TerminateInfo) {}

godot_gdnative_init!(godot_gdnative_init_empty as godot_voip_gdnative_init);
godot_nativescript_init!(init as godot_voip_nativescript_init);
godot_gdnative_terminate!(godot_gdnative_terminate_empty as godot_voip_gdnative_terminate);
