mod intervals;
mod plug;
mod settings;
mod stdout_styling;

use paws_rest::run_rest_api;
use paws_install::{list_plugins, install_from_github, remove_plugin};
use plug::start_main_loop;
use paws_config::load_config;

use clap::{Parser, Subcommand};

const DEFAULT_CONFIG_PATH: &str = "paws.yml";

#[derive(Subcommand, Debug)]
pub enum Command {
    Serve,
    Run {
        #[arg(long = "config", default_value_t = DEFAULT_CONFIG_PATH.to_string())]
        config: String
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


#[tokio::main]
async fn main() {
    let args = CliArguments::parse();

    match args.command {
        Command::Serve => run_rest_api().await,
        Command::Run { config } => {
            let config = load_config(&config);
            start_main_loop(config);
        },
        Command::List => list_plugins().unwrap(),
        Command::Install { name, branch, save_as } => install_from_github(&name, &branch, save_as).unwrap(),
        Command::Uninstall { name } => remove_plugin(name).unwrap(),
    }
}
