extends Node

const SERVER_PORT = 3000
const MAX_PLAYERS = 20
const SERVER_IP = "127.0.0.1"

func start_client() -> int:
	var peer = NetworkedMultiplayerENet.new()

	var err = peer.create_client(SERVER_IP, SERVER_PORT)
	if err != OK:
		return err

	get_tree().set_network_peer(peer)

	return OK

func start_server() -> int:
	var peer = NetworkedMultiplayerENet.new()

	var err = peer.create_server(SERVER_PORT, MAX_PLAYERS)

	if err != OK:
		return err

	get_tree().set_network_peer(peer)

	return OK

