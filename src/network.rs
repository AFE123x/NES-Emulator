use std::error::Error;

use iroh::{NodeAddr, NodeId};
use iroh_gossip::proto::TopicId;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub topic: TopicId,
    pub nodes: Vec<NodeAddr>,
}

impl Ticket {
    /// Deserialize from a slice of bytes to a Ticket.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    /// Serialize from a `Ticket` to a `Vec` of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}


// First, let's define a new message type for controller states
#[derive(Debug, Serialize, Deserialize)]
pub enum EmulatorMessage {
    ControllerState {
        player_num: u8,  // 1 or 2
        controller_bits: u8,
    },
    // We can keep the existing chat messages
    AboutMe { from: NodeId, name: String },
    Message { from: NodeId, text: String },
}

impl EmulatorMessage {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}

// In your main.rs, add this struct to track player roles
pub struct MultiplayerState {
    is_host: bool,
    sender: Option<iroh_gossip::net::GossipSender>,
    remote_controller_state: u8,
    last_sent_state: u8,
}

impl MultiplayerState {
    pub fn new(is_host: bool) -> Self {
        Self {
            is_host,
            sender: None,
            remote_controller_state: 0,
            last_sent_state: 0,
        }
    }
    
    pub fn set_sender(&mut self, sender: iroh_gossip::net::GossipSender) {
        self.sender = Some(sender);
    }
    
    pub async fn send_controller_state(&mut self, controller_bits: u8) -> Result<(), Box<dyn Error>> {
        // Only send if state changed to reduce network traffic
        if controller_bits != self.last_sent_state {
            if let Some(sender) = &self.sender {
                let player_num = if self.is_host { 1 } else { 2 };
                let message = EmulatorMessage::ControllerState {
                    player_num,
                    controller_bits,
                };
                sender.broadcast(message.to_vec().into()).await?;
                self.last_sent_state = controller_bits;
            }
        }
        Ok(())
    }
}