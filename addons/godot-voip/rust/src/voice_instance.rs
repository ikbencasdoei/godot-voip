use std::cmp::min;

use gdnative::{api::AudioStreamGeneratorPlayback, nativescript::property::*, prelude::*};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_class)]
pub struct NativeVoiceInstance {
    custom_voice_audio_stream_player: NodePath,
    recording: bool,
    listen: bool,
    input_threshold: f32,
    playback: Option<AudioStreamGeneratorPlayback>,
    receive_buffer: Float32Array,
}

#[methods]
impl NativeVoiceInstance {
    fn new(_: &Node) -> Self {
        NativeVoiceInstance {
            custom_voice_audio_stream_player: NodePath::from_str(""),
            recording: false,
            listen: false,
            input_threshold: 0.005,
            playback: None,
            receive_buffer: Float32Array::new(),
        }
    }

    fn register_class(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "received_voice_data",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "sent_voice_data",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "created_instance",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "removed_instance",
            args: &[],
        });

        builder
            .add_property::<NodePath>("custom_voice_audio_stream_player")
            .with_ref_getter(NativeVoiceInstance::get_custom_voice_audio_stream_player)
            .with_setter(NativeVoiceInstance::set_custom_voice_audio_stream_player)
            .with_default(NodePath::from_str(""))
            .done();
        builder
            .add_property::<bool>("recording")
            .with_ref_getter(NativeVoiceInstance::get_recording)
            .with_setter(NativeVoiceInstance::set_recording)
            .with_default(false)
            .done();
        builder
            .add_property::<bool>("listen")
            .with_ref_getter(NativeVoiceInstance::get_listen)
            .with_setter(NativeVoiceInstance::set_listen)
            .with_default(false)
            .done();
        builder
            .add_property::<f32>("input_threshold")
            .with_ref_getter(NativeVoiceInstance::get_input_threshold)
            .with_setter(NativeVoiceInstance::set_input_threshold)
            .with_default(0.005)
            .with_hint(FloatHint::Range(RangeHint {
                min: 0.0,
                max: 1.0,
                step: None,
                or_greater: false,
                or_lesser: false,
            }))
            .done();
    }

    fn get_custom_voice_audio_stream_player(&self, _: TRef<Node>) -> &NodePath {
        &self.custom_voice_audio_stream_player
    }
    fn set_custom_voice_audio_stream_player(&mut self, _: TRef<Node>, value: NodePath) {
        self.custom_voice_audio_stream_player = value;
    }

    fn get_recording(&self, _: TRef<Node>) -> &bool {
        &self.recording
    }
    fn set_recording(&mut self, _: TRef<Node>, value: bool) {
        self.recording = value;
    }

    fn get_listen(&self, _: TRef<Node>) -> &bool {
        &self.listen
    }
    fn set_listen(&mut self, _: TRef<Node>, value: bool) {
        self.listen = value;
    }

    fn get_input_threshold(&self, _: TRef<Node>) -> &f32 {
        &self.input_threshold
    }
    fn set_input_threshold(&mut self, _: TRef<Node>, value: f32) {
        self.input_threshold = value;
    }

    #[export]
    fn _process(&mut self, _: &Node, _: f32) {
        self.process_voice();
    }

    fn process_voice(&mut self) {
        match &self.playback {
            Some(playback) => {
                if playback.get_frames_available() < 1 {
                    return;
                }

                for _ in 0..min(
                    playback.get_frames_available(),
                    self.receive_buffer.len() as i64,
                ) {
                    playback.push_frame(Vector2::new(
                        self.receive_buffer.get(0),
                        self.receive_buffer.get(0),
                    ));

                    self.receive_buffer.remove(0);
                }

                if playback.get_frames_available() > 0 {
                    let mut buffer = Vector2Array::new();
                    buffer.resize(playback.get_frames_available() as i32);
                    playback.push_buffer(buffer);
                }
            }
            None => (),
        }
    }
}
