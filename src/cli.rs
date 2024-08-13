use clap::{Parser, Subcommand};

use crate::style::CLAP_STYLING;

#[derive(Parser, Debug)]
#[command(name = "comet", version, about, long_about = None, styles = CLAP_STYLING)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Debug {
        #[arg(short, long)]
        port: String,

        #[arg(short, long)]
        quiet: bool,
    },
    Upload {
        #[arg(short, long)]
        port: String,

        #[arg(short, long)]
        file: String,

        #[arg(short, long)]
        quiet: bool,
    },
    Test {
        #[arg(short = 'a', long, allow_hyphen_values = true)]
        qemu_args: String,

        #[arg(short = 'b', long, default_value = "qemu-system-aarch64")]
        qemu_bin: String,

        #[arg(short, long)]
        upload_file: Option<String>,

        #[arg(short, long)]
        quiet: bool,
    }
    // Locate {
    //     #[arg(short, long)]
    //     verbose: bool
    // }
}
