mod cli;
mod port;
mod serial;
mod style;

use std::fs::File;

use cli::{Cli, Commands};
use serial::{device::DeviceSerial, qemu::QEMUSerial};

use clap::Parser;

fn main() {
    let command = Cli::parse().command;
    match command {
        Commands::Debug { port, quiet } => {
            port::open_port(Box::new(DeviceSerial::new(port)), quiet, &mut None)
        }
        Commands::Upload { port, file, quiet } => {
            let mut file = Some(File::open(file).expect("file not found"));
            port::open_port(Box::new(DeviceSerial::new(port)), quiet, &mut file)
        }
        Commands::Test {
            qemu_bin,
            qemu_args,
            upload_file,
            quiet,
        } => {
            if let Some(file) = upload_file {
                let mut file = Some(File::open(file).expect("file not found"));
                port::open_port(
                    Box::new(QEMUSerial::new(qemu_bin, qemu_args)),
                    quiet,
                    &mut file,
                )
            } else {
                port::open_port(
                    Box::new(QEMUSerial::new(qemu_bin, qemu_args)),
                    quiet,
                    &mut None,
                )
            }
        }
        // Commands::Locate { verbose } => {
        //     println!("Locating devices... Verbose: {}", verbose);
        // }
    }
}
