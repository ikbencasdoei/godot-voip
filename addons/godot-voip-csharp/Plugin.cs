using Godot;
using System;

[Tool]
public class Plugin : EditorPlugin {

    private Script _voiceInstanceScript = 
        ResourceLoader.Load<Script>("res://addons/godot-voip-csharp/scripts/VoiceInstance.cs");
    private Texture _voiceInstanceTex = 
        ResourceLoader.Load<Texture>("res://addons/godot-voip-csharp/icons/VoiceInstance.svg");

    private Script _voiceOrchestratorScript = 
        ResourceLoader.Load<Script>("res://addons/godot-voip-csharp/scripts/VoiceOrchestrator.cs");
    private Texture _voiceOrchestratorTex = 
        ResourceLoader.Load<Texture>("res://addons/godot-voip-csharp/icons/VoiceOrchestrator.svg");

    public override void _EnterTree() {
    
        AddCustomType("VoiceInstance", "Node", _voiceInstanceScript, _voiceInstanceTex);
        AddCustomType("VoiceOrchestrator", "Node", _voiceOrchestratorScript, _voiceOrchestratorTex);
    }

    public override void _ExitTree() {
        RemoveCustomType("VoiceInstance");
        RemoveCustomType("VoiceOrchestrator");
    }
}
