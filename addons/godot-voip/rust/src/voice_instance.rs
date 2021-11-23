use gdnative::{nativescript::property::*, prelude::*};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_class)]
pub struct NativeVoiceInstance {
    custom_voice_audio_stream_player: NodePath,
    recording: bool,
    listen: bool,
    input_threshold: f32,
}

#[methods]
impl NativeVoiceInstance {
    fn new(_: &Node) -> Self {
        NativeVoiceInstance {
            custom_voice_audio_stream_player: NodePath::from_str(""),
            recording: false,
            listen: false,
            input_threshold: 0.005,
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
}
