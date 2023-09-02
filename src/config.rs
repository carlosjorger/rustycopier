use clap::Parser;
#[derive(Parser)]
pub struct Config {
    #[clap(short, long, value_parser, num_args = 2..)]
    pub paths: Vec<String>,
}
