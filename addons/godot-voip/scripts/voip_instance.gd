extends Node
class_name VoipInstance

signal received_voice_data
signal send_voice_data
signal _updated_sample_format

export var min_packet_lenght_seconds: float = 1.0

var recording: bool = false

var voip_format: int = -1
var voip_mix_rate: int = -1
var voip_stereo: bool = true

var _microphone: VoipMicrophone
var _voice: AudioStreamPlayer
var _effect_record: AudioEffectRecord
var _latest_sample: AudioStreamSample
var _time_recording: float = 0

func _ready() -> void:
	_microphone = VoipMicrophone.new()
	add_child(_microphone)

	_voice = AudioStreamPlayer.new()
	add_child(_voice)

	var record_bus_idx = AudioServer.get_bus_index(_microphone.bus)
	_effect_record = AudioServer.get_bus_effect(record_bus_idx, 0)

remote func _receive_stream_format(_format: int, _mix_rate: int, _stereo: bool):
	voip_format = _format
	voip_mix_rate = _mix_rate
	voip_stereo = _stereo

	emit_signal("_updated_sample_format")

remote func _send_stream_format():
	rpc("_receive_stream_format", _latest_sample.format, _latest_sample.mix_rate, _latest_sample.stereo)

remote func _speak(sample_data: PoolByteArray, id: int = -1):
	emit_signal("received_voice_data", sample_data, id)

	var sample = AudioStreamSample.new()
	sample.data = sample_data

	if voip_format == -1:
		rpc("_send_stream_format")
		yield(self, "_updated_sample_format")

	sample.set_format(voip_format)
	sample.set_mix_rate(voip_mix_rate)
	sample.set_stereo(voip_stereo)

	_voice.stream = sample
	_voice.play()

func _process(delta: float) -> void:
	if recording:
		if _effect_record.is_recording_active():
			if _time_recording >= min_packet_lenght_seconds:

				_effect_record.set_recording_active(false)
				_latest_sample = _effect_record.get_recording()

				rpc_unreliable("_speak", _latest_sample.get_data(),  get_tree().get_network_unique_id())

				emit_signal("send_voice_data", _latest_sample.get_data())

				_effect_record.set_recording_active(true)
				_time_recording = 0

			_time_recording += delta
		else:
			_effect_record.set_recording_active(true)
	else:
		_effect_record.set_recording_active(false)





