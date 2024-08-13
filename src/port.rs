use std::{fmt::Write, fs::File, io::{self, Read}, process};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::serial::{Port, Serial};

const COMMAND_PREFIX: &str = "#COMET:";

pub fn open_port(serial: Box<dyn Serial>, quiet: bool, file_to_send: &mut Option<File>) {
    if !quiet {
        eprintln!("[☄️ ] Opening serial port: {}", serial);
    }

    let mut port = serial.open().unwrap_or_else(|e| {
        if !quiet {
            eprintln!("[☄️ ] Could not open serial port! Error: {}", e);
        }

        process::exit(1);
    });

    if !quiet {
        eprintln!("[☄️ ] Connected to serial port! CTRL+C to exit.");
    }

    let mut new_line = true;
    let mut device_type = 0;

    let mut command_prefix_len = 0;

    loop {
        let byte = read_byte(&mut port);

        if let Ok(byte) = byte {
            if byte == COMMAND_PREFIX.chars().nth(command_prefix_len).unwrap() as u8 {
                command_prefix_len += 1;
                if command_prefix_len == COMMAND_PREFIX.len() {
                    command_prefix_len = 0;
                    let command = read_byte(&mut port).unwrap();
                    match command {
                        0x01 => {
                            device_type = read_byte(&mut port).unwrap();
                        },
                        0x02 => {
                            if let Some(file) = file_to_send {
                                port.write_all(COMMAND_PREFIX.as_bytes()).expect("could not write command prefix");
                                port.write_all(&[0x03]).expect("could not write command");

                                let mut buf = vec![0; 1];

                                let size = file.read_to_end(&mut buf).expect("file could not be read") as u32;
                                port.write_all(&size.to_le_bytes()).expect("could not write file size");

                                let pb = ProgressBar::new(size as u64);
                                pb.set_style(ProgressStyle::with_template("[☄️ ] {elapsed_precise} {wide_bar:.cyan/blue} {bytes}/{total_bytes} ({eta})")
                                             .unwrap()
                                            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w,"{:.1}s", state.eta().as_secs_f64()).unwrap())
                                            .progress_chars("#>-"));

                                for byte in buf {
                                    pb.set_position(pb.position() + 1);
                                    port.write_all(&[byte]).expect("could not write file");
                                }

                                pb.finish();
                                println!();
                            }
                        }
                        _ => ()
                    }
                    continue;
                }
            } else {
                let mut chars = COMMAND_PREFIX.chars();
                for _ in 0..command_prefix_len {
                    print!("{}", chars.next().unwrap());
                }
                command_prefix_len = 0;

                if new_line {
                    print!("[{}] ", get_device_emoji(device_type));
                    new_line = false;
                }
                print!("{}", byte as char);

                if byte == b'\n' {
                    new_line = true;
                }
            }
        } else {
            if !quiet {
                eprintln!("[☄️ ] Error reading from serial port: {:?}", byte);
            }

            process::exit(1);
        }
    }
}

fn get_device_emoji(device_type: u8) -> char {
    match device_type {
        1 => '🚀', // Starship
        2 => '🌟', // Starlight
        3 => '💫', // Starlight Mini
        _ => '❓'
    }
}

fn read_byte(port: &mut Box<dyn Port>) -> Result<u8, io::Error> {
        let mut buf: Vec<u8> = vec![0; 1];
        match port.read(&mut buf) {
            Ok(_) => Ok(buf[0]),
            Err(e) => Err(e)
        }
}
