use std::io;
use crate::utils::{calculate_hash, hash_to_binary};
use chrono::Utc;
use serde::{Serialize, Deserialize};

mod utils;
mod p2p;

const DIFFICULTY_PREFIX: &str = "00";

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    // Proof of work
    pub pow: u64,
    // TODO: Transaction struct
    transactions: Vec<String>,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            id: 0,
            hash: "".to_string(),
            previous_hash: "".to_string(),
            timestamp: 0,
            data: "".to_string(),
            pow: 0,
            transactions: vec![],
        }
    }
}

impl Block {
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();

        let (pow, hash) = Block::mine_block(id, now.timestamp(), previous_hash.as_str(), data.as_str());

        Self {
            id,
            hash,
            previous_hash,
            timestamp: now.timestamp(),
            data,
            pow,
            transactions: vec![],
        }
    }


    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        let mut pow = 0;

        loop {
            let hash = calculate_hash(id, timestamp, previous_hash, data, pow);
            let binary_hash = hash_to_binary(&hash);

            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                println!("mined! pow: {} hash: {}", pow, binary_hash);

                return (pow, hex::encode(binary_hash));
            }

            pow += 1;
        }
    }
}

impl Chain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: "genesis".to_string(),
            data: "genesis!".to_string(),
            pow: 0,
            hash: "6166746572206461726b".to_string(),
            transactions: vec![],
        };

        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("there is no block");

        if self.is_block_valid(&block, &latest_block) {
            self.blocks.push(block);
        }
    }

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            println!("Not the same with previous_block hash");
            return false;
        } else if !hash_to_binary(&block.hash.as_bytes()).starts_with(DIFFICULTY_PREFIX) {
            println!("difficulty prefix");
            return false;
        } else if block.id != previous_block.id + 1 {
            println!("id");
            return false;
        } else if hex::encode(calculate_hash(block.id, block.timestamp, &block.previous_hash, &block.data, block.pow)) != block.hash {
            println!("hash is not correct");
            return false;
        }

        true
    }

    fn is_chain_valid(&self, chain: &Vec<Block>) -> (bool, i32) {
        let mut valid_block_count = 0;

        if chain.len() == 1 {
            return (false, 0);
        }

        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }

            let first = chain.get(i - 1).expect("not exist");
            let second = chain.get(i).expect("not exist");

            if !self.is_block_valid(&second, &first) {
                return (false, 0);
            }

            valid_block_count += 1;
        }

        (true, valid_block_count)
    }


    fn choose_chain(&self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
        let (local_validation, _) = self.is_chain_valid(&local);
        let (remote_validation, _) = self.is_chain_valid(&remote);

        if local_validation && remote_validation {
            if local.len() > remote.len() {
                local
            } else {
                remote
            }
        } else if !local_validation && remote_validation {
            remote
        } else if local_validation && !remote_validation {
            local
        } else {
            panic!("Both chain is valid")
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    Ok(())
}

