extends Node
class_name VoiceOrchestrator

signal received_voice_data
signal send_voice_data

export var recording: bool = false setget _set_recording
export var listen: bool = false setget _set_listen
export(float, 0.0, 1.0) var input_threshold: = 0.005 setget _set_input_threshold

var _instances := {}
var _id = null

func _ready() -> void:
	get_tree().connect("connected_to_server", self, "_connected_ok")
	get_tree().connect("server_disconnected", self, "_server_disconnected")
	get_tree().connect("connection_failed", self, "_server_disconnected")

	get_tree().connect("network_peer_connected", self, "_player_connected")
	get_tree().connect("network_peer_disconnected", self, "_player_disconnected")

func _physics_process(delta: float) -> void:
	if get_tree().has_network_peer() && get_tree().is_network_server() && _id == null:
		create_instance(get_tree().get_network_unique_id())

	if (!get_tree().has_network_peer() || !get_tree().is_network_server()) && _id == 1:
		reset()

func create_instance(id: int) -> void:
	var instance := VoiceInstance.new()

	if id == get_tree().get_network_unique_id():
		instance.recording = recording
		instance.listen = listen
		instance.input_threshold = input_threshold

		instance.connect("send_voice_data", self, "_send_voice_data")

		_id = id

	instance.connect("received_voice_data", self, "_received_voice_data")

	instance.name = str(id)

	_instances[id] = instance

	add_child(instance)

func remove_instance(id: int) -> void:
	var instance: VoiceInstance = _instances[id]

	if id == _id:
		_id = null

	instance.queue_free()

	_instances.erase(id)

func reset() -> void:
	for id in _instances.keys():
		remove_instance(id)

func _set_recording(value) -> void:
	if _id != null:
		_instances[_id].recording = value

	recording = value

func _set_listen(value) -> void:
	if _id != null:
		_instances[_id].listen = value

	listen = value

func _set_input_threshold(value) -> void:
	if _id != null:
		_instances[_id].input_threshold = value

	input_threshold = value

func _connected_ok() -> void:
	if (!get_tree().has_network_peer() || !get_tree().is_network_server()) && _id == 1:
		reset()

	create_instance(get_tree().get_network_unique_id())

func _server_disconnected() -> void:
	reset()

func _player_connected(id) -> void:
	create_instance(id)

func _player_disconnected(id) -> void:
	remove_instance(id)

func _received_voice_data(data: PoolRealArray, id: int) -> void:
	emit_signal("received_voice_data", data, id)

func _send_voice_data(data: PoolRealArray) -> void:
	emit_signal("send_voice_data", data)
