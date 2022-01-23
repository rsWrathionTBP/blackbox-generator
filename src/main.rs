use anyhow::Result;

use blackbox_generator::opt::{
    get_input,
    Command
};
use blackbox_generator::select_command::{
    command_init,
    command_verify,
    command_config,
    command_edit,
    command_generate,
    command_rarity
};

fn main() -> Result<()> {
    let (option, actual_path) = get_input()?;

    match option.command {
        Command::Init {init_subcommands} => command_init(actual_path, init_subcommands)?,
        Command::Verify {assets_path} => command_verify(actual_path, assets_path)?,
        Command::Config {} => command_config()?,
        Command::Edit {} => command_edit()?,
        Command::Generate {generate_subcommands} => command_generate(actual_path, generate_subcommands)?,
        Command::Rarity {} => command_rarity()?,
    }

    Ok(())
}
