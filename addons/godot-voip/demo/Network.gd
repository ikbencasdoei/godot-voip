extends Node
class_name Network

var server_port := 3000
var server_ip := "127.0.0.1"

enum ClientType {ENET, WEBSOCKET}
export(ClientType) var client_type

func start_client() -> int:
	if client_type == ClientType.ENET:
		var peer := NetworkedMultiplayerENet.new()

		var err = peer.create_client(server_ip, server_port)
		if err != OK:
			return err

		get_tree().set_network_peer(peer)

		return OK
	elif client_type == ClientType.WEBSOCKET:
		var peer := WebSocketClient.new()

		var err = peer.connect_to_url("ws://%s:%s" % [server_ip, server_port], PoolStringArray(), true)
		if err != OK:
			return err

		get_tree().set_network_peer(peer)

		return OK
	return FAILED

func start_server() -> int:
	if client_type == ClientType.ENET:
		var peer := NetworkedMultiplayerENet.new()

		var err := peer.create_server(server_port)

		if err != OK:
			return err

		get_tree().set_network_peer(peer)

		return OK
	elif client_type == ClientType.WEBSOCKET:
		var peer := WebSocketServer.new()

		var err := peer.listen(server_port, PoolStringArray(), true)

		if err != OK:
			return err

		get_tree().set_network_peer(peer)

		return OK
	return FAILED

func stop() -> void:
	if get_tree().network_peer != null:
		if get_tree().network_peer is WebSocketClient:
			get_tree().network_peer.disconnect_from_host()
		elif get_tree().network_peer is WebSocketServer:
			get_tree().network_peer.stop()
		elif get_tree().network_peer is NetworkedMultiplayerENet:
			get_tree().network_peer.close_connection()
		get_tree().set_network_peer(null)

