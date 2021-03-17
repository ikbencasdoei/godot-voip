extends Node
class_name Network

var server_port := 3000
var server_ip := "127.0.0.1"

const MAX_PLAYERS := 20

func start_client() -> int:
	var peer = NetworkedMultiplayerENet.new()

	var err = peer.create_client(server_ip, server_port)
	if err != OK:
		return err

	get_tree().set_network_peer(peer)

	return OK

func start_server() -> int:
	var peer = NetworkedMultiplayerENet.new()

	var err = peer.create_server(server_port, MAX_PLAYERS)

	if err != OK:
		return err

	get_tree().set_network_peer(peer)

	return OK

func stop():
	if get_tree().network_peer != null:
		get_tree().network_peer.close_connection()

