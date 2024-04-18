mod intervals;
mod plug;
mod settings;
mod stdout_styling;

use std::path::PathBuf;

use paws_install::{list_plugins, install_from_github, remove_plugin, get_kittypaws_home};
use plug::start_main_loop;
use paws_config::load_config;

use clap::{Parser, Subcommand};

const DEFAULT_CONFIG_FILE_NAME: &str = "paws.yml";

fn get_default_config_path() -> PathBuf {
    get_kittypaws_home().join(DEFAULT_CONFIG_FILE_NAME)
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Run {
        #[arg(long = "config")]
        config: Option<PathBuf>
    },

    List,

    Uninstall {
        name: String,
    },
    Install {
        name: String,
        branch: String,
        save_as: Option<String>,
    },
}

#[derive(Parser, Debug)]
#[command(version, about = "Kittypaws Destruction Executor", long_about = None)]
pub struct CliArguments {
    #[command(subcommand)]
    pub command: Command,
}


fn main() {
    let args = CliArguments::parse();

    match args.command {
        Command::Run { config } => {
            let config = load_config(config.unwrap_or(get_default_config_path()));
            start_main_loop(config);
        },
        Command::List => list_plugins().unwrap(),
        Command::Install { name, branch, save_as } => install_from_github(&name, &branch, save_as).unwrap(),
        Command::Uninstall { name } => remove_plugin(name).unwrap(),
    }
}
