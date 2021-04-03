extends AudioStreamPlayer
class_name VoiceMic

func _ready() -> void:
	var current_number = 0
	while AudioServer.get_bus_index("VoiceMicRecorder" + str(current_number)) != -1:
		current_number += 1

	var bus_name = "VoiceMicRecorder" + str(current_number)
	var idx = AudioServer.bus_count

	AudioServer.add_bus(idx)
	AudioServer.set_bus_name(idx, bus_name)

	AudioServer.add_bus_effect(idx, AudioEffectCapture.new())

	AudioServer.set_bus_mute(idx, true)

	bus = bus_name

	stream = AudioStreamMicrophone.new()
	play()
