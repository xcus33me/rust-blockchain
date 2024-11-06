use crate::block::{Block, TARGET_HEX};
use crate::error::*;

use bincode::{serialize, deserialize};
use log::info;
use sled::Db;

#[derive(Debug, Clone)]
pub struct Blockchain {
    current_hash: String,
    db: sled::Db,
}

pub struct BlockchainIter<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        info!("Opening database");

        let db = sled::open("data/blocks")?;
        match db.get("LAST")? {
            Some(hash) => {
                let last_hash = String::from_utf8(hash.to_vec())?;
                Ok(Blockchain {
                    current_hash: last_hash,
                    db,
                })
            },
            None => {
                let block = Block::new_genesis_block();
                db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                db.insert("LAST", block.get_hash().as_bytes())?;
                let bc = Blockchain {
                    current_hash:block.get_hash(),
                    db,
                };
                bc.db.flush()?;
                Ok(bc)
            }
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev_hash = self.db.get("LAST")?.unwrap();
        let new_block = Block::new_block(data, String::from_utf8(prev_hash.to_vec())?, TARGET_HEX)?;
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        self.db.flush()?;
        Ok(())
    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter {
            current_hash: self.current_hash.clone(),
            bc: &self,
        }
    }
}

impl<'a> Iterator for BlockchainIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option::<Self::Item> {
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash) {
            return match encode_block {
                Some(b) => {
                    if let Ok(block) = deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_block() {
        let mut bc = Blockchain::new().unwrap();
        bc.add_block("data".to_string()).unwrap();
        bc.add_block("data1".to_string()).unwrap();
        bc.add_block("data2".to_string()).unwrap();
        
        for item in bc.iter() {
            dbg!(item);
        }
    }
}