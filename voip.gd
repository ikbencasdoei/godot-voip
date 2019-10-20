extends AudioStreamPlayer

const MIN_PACKET_SIZE = 50000

var mic : AudioEffectRecord
var record
var recording = false

func _ready():
	mic = AudioServer.get_bus_effect(AudioServer.get_bus_index("Record"), 0)
	
remote func _play(id, audioPacket, format, mix_rate, stereo):
	get_node("/root/Control/Log").text += "received audio from player with id: %s\n" % id
	
	var audioStream = AudioStreamSample.new()
	audioStream.data = audioPacket
	audioStream.set_format(format)
	audioStream.set_mix_rate(mix_rate)
	audioStream.set_stereo(stereo)
	stream = audioStream
	play()
			
func _process(delta: float) -> void:
	if recording:
		if mic.is_recording_active():
			record = mic.get_recording()
			if record.get_data().size() > MIN_PACKET_SIZE:
				mic.set_recording_active(false)
				rpc_unreliable("_play",get_tree().get_network_unique_id(), record.get_data(), record.get_format(), record.get_mix_rate(), record.is_stereo())
				get_node("/root/Control/Log").text += "send recording of size %s\n" % record.get_data().size()
				mic.set_recording_active(true)
		else:
			mic.set_recording_active(true)
	else:
		mic.set_recording_active(false)