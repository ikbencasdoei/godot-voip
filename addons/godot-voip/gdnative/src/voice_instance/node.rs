use gdnative::{
    export::hint::{FloatHint, RangeHint},
    prelude::*,
};

use super::*;

#[methods]
impl NativeVoiceInstance {
    pub fn new(_: &Node) -> Self {
        Self {
            custom_voice_audio_stream_player: NodePath::from_str(""),
            recording: false,
            listen: false,
            input_threshold: 0.005,
            playback: None,
            receive_buffer: Float32Array::new(),
            effect_capture: None,
            prev_frame_recording: false,
        }
    }

    pub fn register_class(builder: &ClassBuilder<Self>) {
        builder
            .signal("received_voice_data")
            .with_param("data", VariantType::Float32Array)
            .with_param("sender_id", VariantType::I64)
            .done();
        builder
            .signal("sent_voice_data")
            .with_param("data", VariantType::Float32Array)
            .done();

        builder
            .property::<NodePath>("custom_voice_audio_stream_player")
            .with_ref_getter(Self::get_custom_voice_audio_stream_player)
            .with_setter(Self::set_custom_voice_audio_stream_player)
            .with_default(NodePath::from_str(""))
            .done();
        builder
            .property::<bool>("recording")
            .with_ref_getter(Self::get_recording)
            .with_setter(Self::set_recording)
            .with_default(false)
            .done();
        builder
            .property::<bool>("listen")
            .with_ref_getter(Self::get_listen)
            .with_setter(Self::set_listen)
            .with_default(false)
            .done();
        builder
            .property::<f32>("input_threshold")
            .with_ref_getter(Self::get_input_threshold)
            .with_setter(Self::set_input_threshold)
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

    fn get_custom_voice_audio_stream_player(&self, _owner: TRef<Node>) -> &NodePath {
        &self.custom_voice_audio_stream_player
    }

    fn set_custom_voice_audio_stream_player(&mut self, owner: TRef<Node>, value: NodePath) {
        if !value.is_empty() {
            match owner.get_node(value.to_string()) {
                Some(player) => {
                    match self.audiostream_set_playback_generator(unsafe { &player.assume_safe() })
                    {
                        Err(err) => {
                            godot_error!("{}", err);
                            return;
                        }
                        _ => (),
                    }
                }
                None => {
                    godot_error!(
                        "VoiceInstance: Referenced custom AudioStreamPlayer does not exist."
                    );
                    return;
                }
            }
        }

        self.custom_voice_audio_stream_player = value;
    }

    fn get_recording(&self, _owner: TRef<Node>) -> &bool {
        &self.recording
    }
    fn set_recording(&mut self, _owner: TRef<Node>, value: bool) {
        self.recording = value;
    }

    fn get_listen(&self, _owner: TRef<Node>) -> &bool {
        &self.listen
    }
    fn set_listen(&mut self, _owner: TRef<Node>, value: bool) {
        self.listen = value;
    }

    fn get_input_threshold(&self, _owner: TRef<Node>) -> &f32 {
        &self.input_threshold
    }
    fn set_input_threshold(&mut self, _owner: TRef<Node>, value: f32) {
        self.input_threshold = value;
    }

    #[export]
    fn _process(&mut self, owner: &Node, _owner: f32) {
        self.process_voice();
        self.process_mic(owner);
    }

    #[export(rpc = "remote")]
    fn speak(&mut self, owner: &Node, data: PoolArray<f32>) {
        self.receive_voice(owner, data)
    }
}
