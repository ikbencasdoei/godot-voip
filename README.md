# Godot VoIP 🎙️🌐
![logo](https://raw.githubusercontent.com/casbrugman/godot-voip/master/icon.svg "Logo")

Godot-voip is a Godot addon which makes it very easy to setup a real time voice chat system in your Godot game. This addon also includes a demo project.

## Engine Compatibility
* **2.x**: Godot 3.2
* **3.x**: Godot 3.3

## Setup

### Adding to a existing project

(A high-level multiplayer peer is required.)
1. Click on the AssetLib inside editor or go to the [Godot Asset Library](https://godotengine.org/asset-library/asset/425) to download the latest release, or you can clone/download this repository to get the latest commit.
2. Select the `addons/godot-voip` folder and move it into your Godot project. 
(**Note**: make sure the structure is still `res://addons/godot-voip`)
3. Go to project-settings/audio and enable audio input.
4. Now go to the plugins tab also inside project-settings and enable the godot-voip plugin.
5. Add either a VoiceInstance (max 2 participants) or a VoiceOrchestrator (more than 2 participants) node to your scene.
6. Set `$VoiceInstance.recording` or `$VoiceOrchestrator.recording` to `true` and it will send your microphone input to connected participants.

### Running demo
1. Go to the [Godot Asset Library](https://godotengine.org/asset-library/asset/425) to download the latest release, or you can clone/download this repository to get the latest commit.
2. Open downloaded project.
