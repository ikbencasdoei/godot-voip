use gdnative::{
    api::{AudioEffectCapture, AudioServer, AudioStreamMicrophone, AudioStreamPlayer},
    prelude::*,
};

use super::*;

impl NativeVoiceInstance {
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
        audio_server.set_bus_mute(record_bus_index, true); //mute because this audiobus is for capturing microhphone audio
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

    pub fn process_mic(&mut self, owner: &Node) {
        if self.recording {
            if self.effect_capture.is_none() {
                self.create_mic(owner);
            }

            let effect_capture = unsafe { self.effect_capture.as_ref().unwrap().assume_safe() };

            if effect_capture.get_frames_available() > 0 {
                //if recording just started: clear buffer to prevent sending of unintended recorded sound.
                if !self.prev_frame_recording {
                    effect_capture.clear_buffer();
                }

                let vec_stereo_data = effect_capture
                    .get_buffer(effect_capture.get_frames_available())
                    .read()
                    .to_vec();

                //convert samples to mono + get amplitude
                let mut vec_mono_data: Vec<f32> = Vec::with_capacity(vec_stereo_data.len());
                let mut max_value = 0f32;
                for vector in vec_stereo_data.iter() {
                    let value = (vector.x + vector.y) / 2.0;

                    max_value = max_value.max(value);
                    vec_mono_data.push(value);
                }

                // continue if amplitude is high enough
                if max_value < self.input_threshold {
                    return;
                }

                let mono_data = PoolArray::from_vec(vec_mono_data);

                //send voice data
                if self.listen {
                    self.receive_voice(owner, mono_data.clone());
                }
                owner.rpc_unreliable("speak", &[mono_data.to_variant()]);
                owner.emit_signal("sent_voice_data", &[mono_data.to_variant()]);
            }
        }

        self.prev_frame_recording = self.recording;
    }
}
