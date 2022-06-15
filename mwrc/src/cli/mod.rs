use self::commands::{
    build, clean, init, run, test
};

mod commands;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "mwrc", about = "Moholoholo Wildlife Rehabilitation Center", version)]
struct Opt {
    /// command to run
    #[clap(subcommand)]
    command: Mwrc,
}

#[derive(Debug, Parser)]
enum Mwrc {

}