extends Node
class_name VoipInstance

signal received_voice_data
signal send_voice_data
signal _updated_sample_format

export var custom_voice_audio_stream_player: NodePath

export var recording: bool = false
export var listen: bool = false

var _microphone: VoipMicrophone
var _voice
var _effect_capture: AudioEffectCapture
var _playback: AudioStreamGeneratorPlayback

var _receive_buffer := PoolRealArray()

func _ready() -> void:
	_microphone = VoipMicrophone.new()
	add_child(_microphone)

	if !custom_voice_audio_stream_player.is_empty():
		var player = get_node(custom_voice_audio_stream_player)
		if player != null:
			if player is AudioStreamPlayer || player is AudioStreamPlayer2D || player is AudioStreamPlayer3D:
				_voice = player
			else:
				push_error("node:'%s' is not any kind of AudioStreamPlayer!" % custom_voice_audio_stream_player)
		else:
			push_error("node:'%s' does not exist!" % custom_voice_audio_stream_player)
	else:
		_voice = AudioStreamPlayer.new()
		add_child(_voice)

	var record_bus_idx = AudioServer.get_bus_index(_microphone.bus)

	_effect_capture = AudioServer.get_bus_effect(record_bus_idx, 0)


	_voice.stream = AudioStreamGenerator.new()
	_playback = _voice.get_stream_playback()
	_voice.play()

remote func _speak(sample_data: PoolRealArray, id: int = -1):
	emit_signal("received_voice_data", sample_data, id)
	_receive_buffer.append_array(sample_data)

func _process_input():
	for i in range(_playback.get_frames_available()):
		if _receive_buffer.size() > 0:
			_playback.push_frame(Vector2(_receive_buffer[0], _receive_buffer[0]))
			_receive_buffer.remove(0)
		else:
			_playback.push_frame(Vector2.ZERO)

func _process(delta: float) -> void:
	if recording:
		var stereo_data = _effect_capture.get_buffer(_effect_capture.get_frames_available())
		if stereo_data.size() > 0:

			var data = PoolRealArray()
			data.resize(stereo_data.size())

			for i in range(stereo_data.size()):
				data[i] = (stereo_data[i].x + stereo_data[i].y) / 2.0

			if listen:
				_speak(data, get_tree().get_network_unique_id())

			rpc_unreliable("_speak", data,  get_tree().get_network_unique_id())
			emit_signal("send_voice_data", data)
	_process_input()




