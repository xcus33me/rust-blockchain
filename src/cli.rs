use crate::blockchain::*;
use crate::error::*;

use clap::{Args, Command, arg};

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("rsb-demo")
            .version("1.0")
            .author("Eugenii 'xcus33me' Igorevic")
            .about("A Rich Rust-based blockchain project focused on secure, efficient, and decentralized data management.")
            .subcommand(Command::new("print").about("print all the blocks from blockchain"))
            .subcommand(
                Command::new("addblock")
                .about("add a block in the blockchain")
                .arg(arg!(<DATA>" 'the blockchain data'")),
            )   
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.addblock(String::from(c))?;
            } else {
                println!("Not printing testing lists...");
            }
        }

        if let Some(_) = matches.subcommand_matches("print") {
            self.printchain();
        }

        Ok(())
    } 

    fn addblock(&mut self, data: String) -> Result<()> {
        self.bc.add_block(data);
        Ok(())
    }

    fn printchain(&self) -> Result<()> {
        for item in &mut self.bc.iter() {
            dbg!(item);
        }
        Ok(())
    }
}