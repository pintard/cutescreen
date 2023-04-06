#![allow(unused)]

mod commands;
mod constants;
mod utils;

use commands::{config, help, init, reset, base, create};
use constants::command::Command::{Config, Help, Init, Reset, Create};
use constants::command::Command;

use strum::ParseError;
use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command_str: &str = args[1].as_str();
        let command: Result<Command, ParseError> = Command::from_str(command_str);
        match command {
            Ok(Config) => config::config_command(),
            Ok(Help) => help::help_command(),
            Ok(Init) => init::init_command(),
            Ok(Reset) => reset::reset_command(),
            Ok(Create) => create::create_command(),
            Err(err) => println!("{err}: '{command_str}'"),
        };
    } else {
        base::base_command();
    }
}
