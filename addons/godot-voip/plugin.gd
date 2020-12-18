tool
extends EditorPlugin

func _enter_tree() -> void:
	add_custom_type("VoipInstance", "Node", preload("res://addons/godot-voip/voip/voip_instance.gd"), preload("res://addons/godot-voip/icon.svg"))

func _exit_tree() -> void:
	pass
