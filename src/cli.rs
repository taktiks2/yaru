use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List,
    Add {
        #[arg(short, long)]
        title: String,
    },
    Delete {
        #[arg(short, long)]
        id: u64,
    },
}
