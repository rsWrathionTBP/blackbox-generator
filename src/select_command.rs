use anyhow::Result;

use std::path::PathBuf;

use crate::init::{
    initialize_solana_working_folder,
    initialize_ethereum_working_folder,
};
use crate::opt::{
    InitSubcommands,
};

pub fn command_init(actual_path: PathBuf, subcommands: InitSubcommands ) -> Result<()> {
    match subcommands{
        InitSubcommands::Solana {name} => initialize_solana_working_folder(actual_path, name),
        InitSubcommands::Ethereum {name} => initialize_ethereum_working_folder(actual_path, name),
    }
}

pub fn command_verify() -> Result<()>{
    todo!();
}

pub fn command_config() -> Result<()>{
    todo!();
}

pub fn command_edit() -> Result<()>{
    todo!();
}

pub fn command_generate() -> Result<()>{
    todo!();
}

pub fn command_rarity() -> Result<()>{
    todo!();
}
