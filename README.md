[⮩View in Godot Asset Library](https://godotengine.org/asset-library/asset/425)
>previously godot-voip-demo
# Godot VoIP 🎤📡
![logo](https://raw.githubusercontent.com/casbrugman/godot-voip/master/icon.png "Logo")

Godot-voip is a Godot addon (currently only 3.2) which makes it very easy to setup a voip system in your Godot game. This addon also includes a demo project.

## Setup

### Adding to a existing project

(A high-level multiplayer peer is required.)
1. Click on the AssetLib inside editor or go to the [Godot Asset Library](https://godotengine.org/asset-library/asset/425) to download the latest release, or you can clone/download this repository to get the latest commit.
2. Select the `addons/godot-voip` folder and move it into your Godot project. 
(**Note**: make sure the structure is still `res://addons/godot-voip`)
3. Go to project-settings/audio and enable audio input.
4. Now go to the plugins tab also inside project-settings and enable the godot-voip plugin.
5. Add a VoipInstance node to your scene.
6. Set `$VoipInstance.recording` to `true` and it will send your microphone input to other connected players.

### Running demo
1. Go to the [Godot Asset Library](https://godotengine.org/asset-library/asset/425) to download the latest release, or you can clone/download this repository to get the latest commit.
2. Open downloaded project.
