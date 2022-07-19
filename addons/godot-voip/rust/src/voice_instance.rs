use std::cmp::min;

use gdnative::{
    api::{
        AudioEffectCapture, AudioServer, AudioStreamGenerator, AudioStreamGeneratorPlayback,
        AudioStreamMicrophone, AudioStreamPlayer, AudioStreamPlayer2D, AudioStreamPlayer3D,
    },
    export::hint::{FloatHint, RangeHint},
    prelude::*,
};

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

#[methods]
impl NativeVoiceInstance {
    fn new(_: &Node) -> Self {
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

    fn do_playback(&mut self, voice: &TRef<Node, Shared>) {
        let generator = AudioStreamGenerator::new();
        generator.set_buffer_length(0.1f64);

        if let Some(voice) = voice.cast::<AudioStreamPlayer>() {
            voice.set_stream(generator);
            self.playback = Some(
                voice
                    .get_stream_playback()
                    .unwrap()
                    .try_cast::<AudioStreamGeneratorPlayback>()
                    .unwrap(),
            );
            voice.play(0.0);
            return;
        }

        if let Some(voice) = voice.cast::<AudioStreamPlayer2D>() {
            voice.set_stream(generator);
            self.playback = Some(
                voice
                    .get_stream_playback()
                    .unwrap()
                    .try_cast::<AudioStreamGeneratorPlayback>()
                    .unwrap(),
            );
            voice.play(0.0);
            return;
        }

        if let Some(voice) = voice.cast::<AudioStreamPlayer3D>() {
            voice.set_stream(generator);
            self.playback = Some(
                voice
                    .get_stream_playback()
                    .unwrap()
                    .try_cast::<AudioStreamGeneratorPlayback>()
                    .unwrap(),
            );
            voice.play(0.0);
            return;
        }

        godot_error!("VoiceInstance: Node is not any kind of AudioStreamPlayer.")
    }

    fn register_class(builder: &ClassBuilder<Self>) {
        builder.signal("received_voice_data").done();
        builder.signal("sent_voice_data").done();
        builder.signal("created_instance").done();
        builder.signal("removed_instance").done();

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

    fn get_custom_voice_audio_stream_player(&self, _: TRef<Node>) -> &NodePath {
        &self.custom_voice_audio_stream_player
    }

    fn set_custom_voice_audio_stream_player(&mut self, owner: TRef<Node>, value: NodePath) {
        if !value.is_empty() {
            match owner.get_node(value.to_string()) {
                Some(player) => {
                    self.do_playback(unsafe { &player.assume_safe() });
                }
                None => godot_error!(
                    "VoiceInstance: Referenced custom AudioStreamPlayer does not exist."
                ),
            }
        }

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
    fn _process(&mut self, owner: &Node, _: f32) {
        self.process_voice();
        self.process_mic(owner);
    }

    #[export(rpc = "remote")]
    fn speak(&mut self, owner: &Node, data: PoolArray<f32>) {
        if self.playback.is_none() {
            let voice = AudioStreamPlayer::new().into_shared();
            self.do_playback(unsafe { &voice.assume_safe().upcast::<Node>() });
            owner.add_child(voice, false);
        }

        let id = unsafe { owner.get_tree().unwrap().assume_safe() }.get_rpc_sender_id();

        owner.emit_signal("received_voice_data", &[data.to_variant(), id.to_variant()]);

        self.receive_buffer.append(&data);
    }

    fn process_voice(&mut self) {
        if let Some(playback) = &self.playback {
            let playback = unsafe { playback.assume_safe() };

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
        }
    }

    fn create_mic(&mut self, owner: &Node) {
        let audio_server = AudioServer::godot_singleton();

        //add audio bus with record effect
        let record_bus_index = audio_server.bus_count();
        audio_server.add_bus(record_bus_index);
        audio_server.set_bus_name(record_bus_index, {
            const BUS_TITLE: &str = "VoiceMicRecorder";
            let mut i = 0;
            loop {
                let name = format!("{}{}", BUS_TITLE, i);
                if audio_server.get_bus_index(&name) == -1 {
                    break name;
                } else {
                    i += 1
                }
            }
        });
        audio_server.add_bus_effect(record_bus_index, AudioEffectCapture::new(), 0);
        audio_server.set_bus_mute(record_bus_index, true);
        self.effect_capture = Some(
            audio_server
                .get_bus_effect(record_bus_index, 0)
                .unwrap()
                .cast::<AudioEffectCapture>()
                .unwrap(),
        );

        //add adioplayer with microphone playing on the previously created bus
        let mic_player = AudioStreamPlayer::new();
        mic_player.set_stream(AudioStreamMicrophone::new());
        mic_player.set_bus(audio_server.get_bus_name(record_bus_index));
        mic_player.play(0.0);
        owner.add_child(mic_player, false);
    }

    fn process_mic(&mut self, owner: &Node) {
        if self.recording {
            if self.effect_capture.is_none() {
                self.create_mic(owner);
            }

            let effect_capture = unsafe { self.effect_capture.as_ref().unwrap().assume_safe() };

            if effect_capture.get_frames_available() > 0 {
                if !self.prev_frame_recording {
                    effect_capture.clear_buffer();
                }


                let vec_stereo_data = effect_capture
                    .get_buffer(effect_capture.get_frames_available())
                    .read()
                    .to_vec();

                let mut vec_mono_data: Vec<f32> = Vec::with_capacity(vec_stereo_data.len());

                let mut max_value = 0f32;

                for vector in vec_stereo_data.iter() {
                    let value = (vector.x + vector.y) / 2.0;

                    max_value = max_value.max(value);
                    vec_mono_data.push(value);
                }

                if max_value < self.input_threshold {
                    return;
                }

                let mono_data = PoolArray::from_vec(vec_mono_data);

                if self.listen {
                    self.speak(owner, mono_data.clone());
                }

                owner.rpc_unreliable("speak", &[mono_data.to_variant()]);

                owner.emit_signal("sent_voice_data", &[mono_data.to_variant()]);
            }
        }

        self.prev_frame_recording = self.recording;
    }
}
