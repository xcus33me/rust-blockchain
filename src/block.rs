use serde::{Deserialize, Serialize};
use bincode::serialize;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::time::SystemTime;
use log::info;

use crate::error::*;

pub const TARGET_HEX: usize = 4;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    timestamp: u128,
    transaction: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

impl Block {
    pub fn new_genesis_block() -> Block {
        Block::new_block(String::from("Genesis Block"), String::new(), 0).unwrap()
    }

    pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        let mut block = Block {
            timestamp: timestamp,
            transaction: data,
            prev_block_hash: prev_block_hash,
            hash: String::new(),
            height: height,
            nonce: 0,
        };
        block.run_proof_of_work()?;
        Ok(block)
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn run_proof_of_work(&mut self) -> Result<()> {
        info!("Mining the block");
        while !self.validate()? {
            self.nonce += 1;
        }

        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = vec![];
        vec1.resize(TARGET_HEX, '0' as u8);
        Ok(&hasher.result_str()[0..TARGET_HEX] == String::from_utf8(vec1)?)
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.transaction.clone(),
            self.timestamp,
            TARGET_HEX,
            self.nonce,
        );
        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }
}