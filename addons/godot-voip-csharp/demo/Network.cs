using Godot;
using System;

public class Network : Node { 
    public int serverPort = 3000;
    public string serverIp = "127.0.0.1";

    public enum ClientType {enet, websocket}
    
    [Export(PropertyHint.Enum)]
    public ClientType clientType = ClientType.enet;

    public int StartClient() {
        if (clientType == ClientType.enet) {
            NetworkedMultiplayerENet peer = new NetworkedMultiplayerENet();

            Error err = peer.CreateClient(serverIp, serverPort);
            if (err != Error.Ok) {
                return (int) err;
            }

            GetTree().NetworkPeer = peer;
        } else {
            WebSocketClient peer = new WebSocketClient();

            Error err = peer.ConnectToUrl($"ws://{serverIp}:{serverPort}", new string[]{}, true);
            if (err != Error.Ok) {
                return (int) err;
            }

            GetTree().NetworkPeer = peer;
        }
        return (int) Error.Ok;
    }

    public int StartServer() {
        if (clientType == ClientType.enet) {
            NetworkedMultiplayerENet peer = new NetworkedMultiplayerENet();

            Error err = peer.CreateServer(serverPort);
            if (err != Error.Ok) {
                return (int) err;
            }

            GetTree().NetworkPeer = peer;
        } else {
            WebSocketServer peer = new WebSocketServer();

            Error err = peer.Listen(serverPort, new string[]{}, true);
            if (err != Error.Ok) {
                return (int) err;
            }

            GetTree().NetworkPeer = peer;
        }
        return (int) Error.Ok;
    }

    public void Stop() {
        if (!(GetTree().NetworkPeer is null)) {
            if (GetTree().NetworkPeer is WebSocketClient wpeer) {
                wpeer.DisconnectFromHost();
            } else
            if (GetTree().NetworkPeer is WebSocketServer wserv) {
                wserv.Stop();
            } else
            if (GetTree().NetworkPeer is NetworkedMultiplayerENet wenet) {
                wenet.CloseConnection();
            } 
            GetTree().NetworkPeer = null;
        }
    }
}