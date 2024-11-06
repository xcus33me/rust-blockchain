use cli::Cli;
use support::Dispatch;

mod balances;
mod support;
mod system;
mod proof_of_existence;
mod block;
mod blockchain;
mod error;
mod cli;

use crate::error::Result;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    
    cli.run()?;

    Ok(())
}
