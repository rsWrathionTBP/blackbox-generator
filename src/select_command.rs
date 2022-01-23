use anyhow::Result;

use std::path::PathBuf;

use crate::init::{
    initialize_solana_working_folder,
    initialize_ethereum_working_folder,
};
use crate::verify::{
    verify_assets,
};
use crate::generate::{
    generate_sample,
    generate_nfts,
    generate_all,
};
use crate::opt::{
    InitSubcommands,
    GenerateSubcommands,
};

pub fn command_init(actual_path: PathBuf, subcommands: InitSubcommands ) -> Result<()> {
    match subcommands{
        InitSubcommands::Solana {name} => initialize_solana_working_folder(actual_path, name),
        InitSubcommands::Ethereum {name} => initialize_ethereum_working_folder(actual_path, name),
    }
}

pub fn command_verify(actual_path: PathBuf, assets_path: String) -> Result<()>{
    verify_assets(actual_path, assets_path)?;
    Ok(())
}

pub fn command_config() -> Result<()>{
    todo!();
}

pub fn command_edit() -> Result<()>{
    todo!();
}

pub fn command_generate(actual_path: PathBuf, subcommands: GenerateSubcommands) -> Result<()>{
    match subcommands{
        GenerateSubcommands::Sample {assets_path, config_path} => generate_sample(actual_path, assets_path, config_path),
        GenerateSubcommands::NFTs {sample_path} => generate_nfts(actual_path, sample_path),
        GenerateSubcommands::All {assets_path, config_path} => generate_all(actual_path, assets_path, config_path),
    }
}

pub fn command_rarity() -> Result<()>{
    todo!();
}
