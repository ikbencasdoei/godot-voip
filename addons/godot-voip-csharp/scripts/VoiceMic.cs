using Godot;
using System;

public class VoiceMic : AudioStreamPlayer {
    
    private string _bus;
    public string BusName { get => _bus; }
    
    public override void _Ready() {
        int currentNumber = 0;

        while (AudioServer.GetBusIndex("VoiceMicRecorder" + currentNumber.ToString()) != -1) {
            ++currentNumber;
        }

        string busName = "VoiceMicRecorder" + currentNumber.ToString();
        int idx = AudioServer.BusCount;

        AudioServer.AddBus(idx);
        AudioServer.SetBusName(idx, busName);

        AudioServer.AddBusEffect(idx, new AudioEffectCapture());

        AudioServer.SetBusMute(idx, true);

        _bus = busName;

        Stream = new AudioStreamMicrophone();

        Play();
    }
}
