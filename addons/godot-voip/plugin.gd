tool
extends EditorPlugin

func _enter_tree() -> void:
	add_custom_type(
		"VoiceInstance",
		"Node",
		preload("res://addons/godot-voip/scripts/voice_instance.gd"),
		preload("res://addons/godot-voip/icons/VoiceInstance.svg")
	)

func _exit_tree() -> void:
	remove_custom_type("VoiceInstance")
