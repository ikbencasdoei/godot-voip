using Godot;
using System;
using System.Collections.Generic;

public class VoiceOrchestrator : Node {
    [Signal]
    public delegate void ReceivedVoiceData(List<float> sample, int id);
    [Signal]
    public delegate void SentVoiceData(List<float> data);
    [Signal]
    public delegate void CreatedInstance(int id);
    [Signal]
    public delegate void RemovedInstance(int id);

    [Export]
    private bool _isRecording = false;
    public bool IsRecording {set => _isRecording = value;}

    [Export]
    private bool _isListening = false;
    public bool IsListening {set => _isListening = value;}

    [Export(PropertyHint.Range, "0.0,1.0")]
    private float _inputThreshold = 0.005f;
    public float InputThreshold {set => _inputThreshold = value;}

    private Dictionary<int, VoiceInstance> _instances;
    private int? _id = null;

    public override void _Ready() {
        GetTree().Connect("connected_to_server", this, nameof(ConnectedOk));
        GetTree().Connect("server_disconnected", this, nameof(ServerDisconnected));
        GetTree().Connect("connection_failed", this, nameof(ServerDisconnected));
     
        GetTree().Connect("network_peer_connected", this, nameof(PlayerConnected));
        GetTree().Connect("network_peer_disconnected", this, nameof(PlayerDisconnected));
    }

    public override void _PhysicsProcess(float delta) {
        if (GetTree().HasNetworkPeer() && GetTree().IsNetworkServer() && _id is null) {
            CreateInstance(GetTree().GetNetworkUniqueId());
        }

        if ((GetTree().HasNetworkPeer() || GetTree().IsNetworkServer()) == false && _id == 1) {
            Reset();
        }
    }

    private void CreateInstance(int id) {
        VoiceInstance instance = new VoiceInstance();

        if (id == GetTree().GetNetworkUniqueId()) {
            instance.isRecording = _isRecording;
            instance.isListening = _isListening;
            instance.inputThreshold = _inputThreshold;

            instance.Connect(nameof(SentVoiceData), this, nameof(_SentVoiceData));

            _id = id;
        }

        instance.Connect(nameof(ReceivedVoiceData), this, nameof(_ReceivedVoiceData));

        instance.Name = id.ToString();

        _instances[id] = instance;

        AddChild(instance);

        EmitSignal(nameof(CreatedInstance), id);
    }

    private void RemoveInstance(int id) {
        VoiceInstance instance = _instances[id];

        if (id == _id) {
            _id = null;
        }

        instance.QueueFree();
        _instances.Remove(id);

        EmitSignal(nameof(RemovedInstance), id);
    }

    private void Reset() {
        foreach (int i in _instances.Keys) {
            RemoveInstance(i);
        }
    }

    private void SetRecording(bool val) {
        if (!(_id is null)) _instances[(int) _id].isRecording = val;

        _isRecording = val;
    }

    private void SetListen(bool val) {
        if (!(_id is null)) _instances[(int) _id].isListening = val;

        _isListening = val;

    }

    private void SetInputThreshold(float val) {
        if (!(_id is null)) _instances[(int) _id].inputThreshold = val;

        _inputThreshold = val;

    }

    private void ConnectedOk() {
        if ((GetTree().HasNetworkPeer() || GetTree().IsNetworkServer()) == false && _id == 1) {
            Reset();
        }

        CreateInstance(GetTree().GetNetworkUniqueId());
    }

    private void ServerDisconnected() {
        Reset();
    }

    private void PlayerConnected(int id) {
        CreateInstance(id);
    }

    private void PlayerDisconnected(int id) {
        RemoveInstance(id);
    }

    private void _ReceivedVoiceData(List<float> data, int id) {
        EmitSignal(nameof(ReceivedVoiceData), data, id);
    }

    private void _SentVoiceData(List<float> data) {
        EmitSignal(nameof(SentVoiceData), data);
    }
}
