use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::task::JoinHandle;
use tokio::net::TcpStream;
use serde::{Serialize, Deserialize};
use std::error::Error;

/// Message sent to the signaling server to register a peer.
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterMessage {
    pub peer_addr: String,
}

/// Message received from the signaling server with peer information.
#[derive(Serialize, Deserialize, Debug)]
pub struct PeerListMessage {
    pub peers: Vec<String>,
}

/// `Discovery` is responsible for discovering and connecting to peers.
pub struct Discovery {
    signaling_server_url: String,
    peers: HashMap<SocketAddr, UnboundedSender<String>>, // Stores discovered peers
}

impl Discovery {
    /// Creates a new `Discovery` instance with the given signaling server URL.
    pub fn new(signaling_server_url: &str) -> Self {
        Self {
            signaling_server_url: signaling_server_url.to_string(),
            peers: HashMap::new(),
        }
    }

    /// Registers the current peer with the signaling server and retrieves the list of available peers.
    pub async fn register_peer(&mut self, local_addr: SocketAddr) -> Result<Vec<String>, Box<dyn Error>> {
        let register_message = RegisterMessage {
            peer_addr: local_addr.to_string(),
        };

        // Send the registration message to the signaling server
        let client = reqwest::Client::new();
        let response = client
            .post(&self.signaling_server_url)
            .json(&register_message)
            .send()
            .await?;

        // Deserialize the response into a PeerListMessage
        let peer_list: PeerListMessage = response.json().await?;
        
        Ok(peer_list.peers)
    }

    /// Connects to the discovered peers based on the information received from the signaling server.
    pub async fn connect_to_peers(
        &mut self,
        peer_addrs: Vec<String>,
        connection_handler: impl Fn(TcpStream, SocketAddr) -> JoinHandle<()>,
    ) -> Result<(), Box<dyn Error>> {
        for peer_addr in peer_addrs {
            if let Ok(socket_addr) = peer_addr.parse::<SocketAddr>() {
                // Attempt to establish a connection to the peer
                if let Ok(stream) = TcpStream::connect(socket_addr).await {
                    // Spawn a task to handle the peer connection
                    connection_handler(stream, socket_addr);
                }
            }
        }
        Ok(())
    }

    /// Starts the discovery process by registering the peer and connecting to discovered peers.
    pub async fn start_discovery(
        &mut self,
        local_addr: SocketAddr,
        connection_handler: impl Fn(TcpStream, SocketAddr) -> JoinHandle<()>,
    ) -> Result<(), Box<dyn Error>> {
        // Register the local peer with the signaling server
        let peer_addrs = self.register_peer(local_addr).await?;

        // Connect to discovered peers
        self.connect_to_peers(peer_addrs, connection_handler).await?;

        Ok(())
    }
}
