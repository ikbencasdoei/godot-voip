use gdnative::{
    api::{AudioEffectCapture, AudioStreamGeneratorPlayback},
    prelude::*,
};

mod mic;
mod node;
mod voice;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_class)]
pub struct NativeVoiceInstance {
    custom_voice_audio_stream_player: NodePath,
    recording: bool,
    listen: bool,
    input_threshold: f32,
    playback: Option<Ref<AudioStreamGeneratorPlayback>>,
    receive_buffer: Float32Array,
    effect_capture: Option<Ref<AudioEffectCapture>>,
    prev_frame_recording: bool,
}
