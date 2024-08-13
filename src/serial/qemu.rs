use std::{io::{self, Read}, process::{self, Command, Stdio}};

use crate::serial::{Serial, Port};

pub struct QEMUSerial {
    bin: String,
    args: String
}

impl QEMUSerial {
    pub fn new(bin: String, args: String) -> Self {
        Self {
            bin,
            args
        }
    }
}

impl Serial for QEMUSerial {
    fn display(&self) -> &str {
        &self.args
    }

    fn open(&self) -> Result<Box<dyn Port>, String> {
        let process = Command::new(&self.bin).args(shellwords::split(&self.args).expect("--qemu-args should be formatted correctly")).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit()).spawn().expect("could not spawn qemu process");
        Ok(Box::new(QEMUPort::new(process)))
    }
}

struct QEMUPort {
    process: process::Child
}

impl QEMUPort {
    fn new(process: process::Child) -> Self {
        Self {
            process
        }
    }
}

impl Port for QEMUPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.process.stdout.as_mut() {
            Some(stdout) => stdout.read(buf),
            None => Err(io::Error::new(io::ErrorKind::Other, "stdout is not available"))
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        use std::io::Write;
        match self.process.stdin.as_mut() {
            Some(stdin) => stdin.write_all(buf),
            None => Err(io::Error::new(io::ErrorKind::Other, "stdin is not available"))
        }
    }
}

impl Drop for QEMUPort {
    fn drop(&mut self) {
        self.process.kill().expect("could not kill qemu process");
    }
}
