using Godot;
using System;
using System.Collections.Generic;

public class VoiceInstance : Node {

    [Export]
    public NodePath customVoiceAudioStreamPlayer;
    [Export]
    public bool isRecording = false;
    [Export]
    public bool isListening = false;
    [Export(PropertyHint.Range, "0.0,1.0")]
    public float inputThreshold = 0.005f;

    private VoiceMic _mic;

    private AudioStreamPlayer _voice;
    private AudioStreamPlayer2D _voice2D;
    private AudioStreamPlayer3D _voice3D;
    private byte _voiceType = 0;

    private AudioEffectCapture _effectCapture;
    private AudioStreamGeneratorPlayback _playback;

    private List<float> _receiveBuffer = new List<float>();
    private bool _prevFrameRecording = false;

    public override void _Process(float delta) {
        if (!(_playback is null)) {
            ProcessVoice();
        }
        ProcessMic();
    }

    private void CreateMic() {
        _mic = new VoiceMic();
        AddChild(_mic);
        int recordBusIdx = AudioServer.GetBusIndex(_mic.BusName);
        _effectCapture = (AudioEffectCapture) AudioServer.GetBusEffect(recordBusIdx, 0);
    }

    private void CreateVoice() {
        if (!customVoiceAudioStreamPlayer.IsEmpty()) {
            var player = GetNodeOrNull(customVoiceAudioStreamPlayer);
            if (!(player is null)) {
                if (player is AudioStreamPlayer pl) {
                    _voiceType = 1;
                    _voice = pl;
                } else 
                if (player is AudioStreamPlayer2D pl2) {
                    _voiceType = 2;
                    _voice2D = pl2;
                } else 
                if (player is AudioStreamPlayer3D pl3) {
                    _voiceType = 3;
                    _voice3D = pl3;
                } else {
                    GD.PushError($"node:'{customVoiceAudioStreamPlayer.ToString()}' is not any kind of AudioStreamPlayer!");
                }
            } else {
                GD.PushError($"node:'{customVoiceAudioStreamPlayer.ToString()}' does not exist!");
            }
        } else {
            _voice = new AudioStreamPlayer();
            _voiceType = 1;
            AddChild(_voice);
        }

        AudioStreamGenerator generator = new AudioStreamGenerator();
        generator.BufferLength = 0.1f;

        switch (_voiceType) {
            case 1: 
                _voice.Stream = generator; 
                _playback = (AudioStreamGeneratorPlayback) _voice.GetStreamPlayback();
                _voice.Play();
            break;
            case 2: 
                _voice2D.Stream = generator; 
                _playback = (AudioStreamGeneratorPlayback) _voice2D.GetStreamPlayback();
                _voice2D.Play();
            break;
            case 3: 
                _voice3D.Stream = generator; 
                _playback = (AudioStreamGeneratorPlayback) _voice3D.GetStreamPlayback();
                _voice3D.Play();
            break;
            default: GD.PushError("VoiceInstance: AudioStreamPlayer is not set!"); break;
        }
    }

    [Remote]
    public void Speak(List<float> sample, int id) {
        if (_playback is null) CreateVoice();

        EmitSignal(nameof(VoiceOrchestrator.ReceivedVoiceData), sample, id);

        _receiveBuffer.AddRange(sample);
    }

    private void ProcessVoice() {
        if (_playback.GetFramesAvailable() < 1) {
            return;
        }

        for (int i = 0; i < Math.Min(_playback.GetFramesAvailable(), _receiveBuffer.Count); i ++) {
            _playback.PushFrame(new Vector2(_receiveBuffer[0], _receiveBuffer[0]));
            _receiveBuffer.RemoveAt(0);
        }
    }

    private void ProcessMic() {
        if (isRecording) {
            if (_effectCapture is null) CreateMic();
            if (_prevFrameRecording == false) _effectCapture.ClearBuffer();
            
            Vector2[] stereoData = _effectCapture.GetBuffer(_effectCapture.GetFramesAvailable());
            if (stereoData.Length > 0) {
                List<float> data = new List<float>();;
                
                float maxValue = 0.0f;

                for (int i = 0; i < stereoData.Length; i++) {
                    float value = (stereoData[i].x + stereoData[i].y) / 2f;
                    maxValue = Mathf.Max(value, maxValue);
                    data.Add(value);
                }

                if (maxValue < inputThreshold) {
                    return;
                }

                if (isListening) {
                    Speak(data, GetTree().GetNetworkUniqueId());
                }

                RpcUnreliable(nameof(Speak), GetTree().GetNetworkUniqueId());
                EmitSignal(nameof(VoiceOrchestrator.SentVoiceData), data);
            }
        }

        _prevFrameRecording = isRecording;
    }


}
