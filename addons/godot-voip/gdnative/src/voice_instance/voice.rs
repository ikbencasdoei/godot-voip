use std::cmp::min;

use gdnative::{
    api::{AudioStreamGenerator, AudioStreamPlayer, AudioStreamPlayer2D, AudioStreamPlayer3D},
    prelude::*,
};

use super::*;

impl NativeVoiceInstance {
    pub fn audiostream_set_playback_generator(
        &mut self,
        voice: &TRef<Node, Shared>,
    ) -> Result<(), &str> {
        let generator = AudioStreamGenerator::new();
        generator.set_buffer_length(0.1f64);

        //check what kind of audioplayer the given node is (if at all)
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
            return Ok(());
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
            return Ok(());
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
            return Ok(());
        }

        Err("VoiceInstance: Node is not any kind of AudioStreamPlayer.")
    }

    pub fn receive_voice(&mut self, owner: &Node, data: PoolArray<f32>) {
        if self.playback.is_none() {
            let voice = AudioStreamPlayer::new().into_shared();
            self.audiostream_set_playback_generator(unsafe {
                &voice.assume_safe().upcast::<Node>()
            })
            .unwrap();
            owner.add_child(voice, false);
        }

        let id = unsafe { owner.get_tree().unwrap().assume_safe() }.get_rpc_sender_id();

        owner.emit_signal("received_voice_data", &[data.to_variant(), id.to_variant()]);
        self.receive_buffer.append(&data);
    }

    pub fn process_voice(&mut self) {
        if let Some(playback) = &self.playback {
            let playback = unsafe { playback.assume_safe() };

            if playback.get_frames_available() < 1 {
                return;
            }

            for _ in 0..min(
                playback.get_frames_available(),
                self.receive_buffer.len() as i64,
            ) {
                //convert samples back to stereo
                playback.push_frame(Vector2::new(
                    self.receive_buffer.get(0),
                    self.receive_buffer.get(0),
                ));

                self.receive_buffer.remove(0);
            }
        }
    }
}
