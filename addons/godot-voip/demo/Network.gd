extends Node

const SERVER_PORT = 3000
const MAX_PLAYERS = 20
const SERVER_IP = "127.0.0.1"

func start_client():
	get_tree().connect("connected_to_server", self, "_connected_ok")
	get_tree().connect("server_disconnected", self, "_server_disconnected")
	get_tree().connect("connection_failed", self, "_connected_fail")

	var peer = NetworkedMultiplayerENet.new()

	var err = peer.create_client(SERVER_IP, SERVER_PORT)
	if err != OK:
		get_node("/root/Control/Status").text = "failed to create client!"
		return

	get_tree().set_network_peer(peer)

	get_node("/root/Control/Status").text = "connecting..."
	get_node("/root/Control/Button_voice").disabled = false

func _connected_ok():
	get_node("/root/Control/Status").text = "connected ok"


func _connected_fail():
	get_node("/root/Control/Status").text = "failed to connect to server!"

func _server_disconnected():
	get_node("/root/Control/Status").text = "server disconnected"


################################
#SERVER
################################

func start_server():
	get_tree().connect("network_peer_connected", self, "_player_connected")
	get_tree().connect("network_peer_disconnected", self, "_player_disconnected")

	var peer = NetworkedMultiplayerENet.new()

	var err = peer.create_server(SERVER_PORT, MAX_PLAYERS)

	if err != OK:
		get_node("/root/Control/Status").text = "Failed to create server!"
		return

	get_tree().set_network_peer(peer)

	get_node("/root/Control/Status").text = "server started"
	get_node("/root/Control/Button_voice").disabled = false


func _player_connected(_id):
	get_node("/root/Control/Log").text += "player with id: %s connected\n" % _id

func _player_disconnected(_id):
	get_node("/root/Control/Log").text += "player with id: %s disconnected\n" % _id

