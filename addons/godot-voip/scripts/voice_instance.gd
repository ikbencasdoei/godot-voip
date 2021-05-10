extends Node
class_name VoiceInstance

signal received_voice_data
signal sent_voice_data

export var custom_voice_audio_stream_player: NodePath
export var recording: bool = false
export var listen: bool = false
export(float, 0.0, 1.0) var input_threshold: = 0.005

var _mic: VoiceMic
var _voice
var _effect_capture: AudioEffectCapture
var _playback: AudioStreamGeneratorPlayback
var _receive_buffer := PoolRealArray()
var _prev_frame_recording = false

func _process(delta: float) -> void:
	if _playback != null:
		_process_voice()

	_process_mic()

func _create_mic():
	_mic = VoiceMic.new()
	add_child(_mic)
	var record_bus_idx := AudioServer.get_bus_index(_mic.bus)
	_effect_capture = AudioServer.get_bus_effect(record_bus_idx, 0)

func _create_voice():
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

	var generator := AudioStreamGenerator.new()
	generator.buffer_length = 0.1
	_voice.stream = generator

	_playback = _voice.get_stream_playback()
	_voice.play()

remote func _speak(sample_data: PoolRealArray, id: int):
	if _playback == null:
		_create_voice()

	emit_signal("received_voice_data", sample_data, id)

	_receive_buffer.append_array(sample_data)

func _process_voice():
	if _playback.get_frames_available() < 1:
		return

	for i in range(min(_playback.get_frames_available(), _receive_buffer.size())):
		_playback.push_frame(Vector2(_receive_buffer[0], _receive_buffer[0]))
		_receive_buffer.remove(0)

	if _playback.get_frames_available() > 0:
		var buffer = PoolVector2Array()
		buffer.resize(_playback.get_frames_available())
		_playback.push_buffer(buffer)

func _process_mic():
	if recording:
		if _effect_capture == null:
			_create_mic()

		if _prev_frame_recording == false:
			_effect_capture.clear_buffer()

		var stereo_data: PoolVector2Array = _effect_capture.get_buffer(_effect_capture.get_frames_available())
		if stereo_data.size() > 0:

			var data = PoolRealArray()
			data.resize(stereo_data.size())

			var max_value := 0.0
			for i in range(stereo_data.size()):
				var value := (stereo_data[i].x + stereo_data[i].y) / 2.0
				max_value = max(value, max_value)
				data[i] = value
			if max_value < input_threshold:
				return

			if listen:
				_speak(data, get_tree().get_network_unique_id())

			rpc_unreliable("_speak", data,  get_tree().get_network_unique_id())
			emit_signal("sent_voice_data", data)

	_prev_frame_recording = recording




