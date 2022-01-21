use anyhow::Result;
use std::{env, path::PathBuf};
use structopt::StructOpt;

pub fn get_input() -> Result<(Opt, PathBuf)>{
    let option = Opt::from_args();
    let path = env::current_dir()?;

    Ok((option, path))
}

#[derive(Debug, StructOpt)]
#[structopt(name = "BlackBox-Generator", about = "The fastest and simplest NFT Generator for Solana/Ethereum")]
pub struct Opt {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Creates a new working folder in the current directory.
    #[structopt(name = "init")]
    Init { 
        #[structopt(subcommand)]
        init_subcommands: InitSubcommands,
    },

    /// Verify if the provided assets follows the required standards.
    #[structopt(name = "verify")]
    Verify {
        #[structopt(short, long, default_value = "assets")]
        assets_name: String,
    },

    /// Starts the UI configuration process.
    #[structopt(name = "config")]
    Config {},

    /// Edit the actual configuration.
    #[structopt(name = "edit")]
    Edit {},

    /// Generate a collection following the configuration.
    #[structopt(name = "generate")]
    Generate {},

    /// Calculates the Rarity ranking of an already created NFT Colelction
    #[structopt(name = "rarity")]
    Rarity {},
}

#[derive(Debug, StructOpt)]
pub enum InitSubcommands {
    /// Initialize working folder with name *<name>* for Solana Blockchain  
    #[structopt(name = "solana")]
    Solana {
        /// Name of the working folder
        #[structopt(short, long)]
        name: String,
    },
    /// Initialize working folder with name *<name>* for Ethereum Blockchain  
    #[structopt(name = "ethereum")]
    Ethereum {
        /// Name of the working folder
        #[structopt(short, long)]
        name: String,
    },
}


