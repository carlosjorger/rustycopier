use std::path::PathBuf;

use clap::{ArgAction, Parser};
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, value_parser, num_args = 2..)]
    pub paths: Vec<PathBuf>,
    #[clap(long, short, action=ArgAction::SetFalse)]
    pub disable_logging: bool,
}
