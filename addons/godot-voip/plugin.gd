tool
extends EditorPlugin

func _enter_tree() -> void:
	add_custom_type("VoipInstance", "Node", preload("res://addons/godot-voip/scripts/voip_instance.gd"), preload("res://addons/godot-voip/icons/VoipInstance.svg"))

func _exit_tree() -> void:
	remove_custom_type("VoipInstance")
