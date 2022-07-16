use libp2p::{identity, PeerId, NetworkBehaviour};
use libp2p::floodsub::{Floodsub, FloodsubEvent, Topic};
use libp2p::identity::Keypair;
use libp2p::mdns::{Mdns, MdnsConfig, MdnsEvent};
use libp2p::swarm::NetworkBehaviourEventProcess;
use once_cell::sync::Lazy;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use crate::{Block, Chain};

pub static KEYS: Lazy<Keypair> = Lazy::new(identity::Keypair::generate_ed25519);
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
pub static CHAIN_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("chains"));
pub static BLOCK_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("blocks"));

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainResponse {
    pub blocks: Vec<Block>,
    pub receiver: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalChainRequest {
    pub from_peer_id: String,
}

pub enum EventType {
    LocalChainResponse(ChainResponse),
    Input(String),
    Init,
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "EventType")]
pub struct AppBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub response_sender: mpsc::UnboundedSender<ChainResponse>,
    #[behaviour(ignore)]
    pub init_sender: mpsc::UnboundedSender<bool>,
    #[behaviour(ignore)]
    pub app: Chain,
}


impl From<MdnsEvent> for EventType {
    fn from(_: MdnsEvent) -> Self {
        Self::Init
    }
}

impl AppBehaviour {
    async fn new(app: Chain, response_sender: mpsc::UnboundedSender<ChainResponse>, init_sender: mpsc::UnboundedSender<bool>) -> Self {
        let mut behaviour = Self {
            app,
            floodsub: Floodsub::new(*PEER_ID),
            mdns: Mdns::new(MdnsConfig::default())
                .await.expect("cant create mdns"),
            response_sender,
            init_sender,
        };

        behaviour.floodsub.subscribe(CHAIN_TOPIC.clone());
        behaviour.floodsub.subscribe(BLOCK_TOPIC.clone());

        behaviour
    }
}

impl From<FloodsubEvent> for EventType {
    fn from(_: FloodsubEvent) -> Self {
        Self::Init
    }
}