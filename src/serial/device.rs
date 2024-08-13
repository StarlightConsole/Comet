use std::{io, time::Duration};

use crate::serial::{Serial, Port};

const STARLIGHT_BAUD_RATE: u32 = 921_600;

pub struct DeviceSerial {
    port: String
}

impl DeviceSerial {
    pub fn new(port: String) -> Self {
        Self {
            port
        }
    }
}

impl Serial for DeviceSerial {
    fn display(&self) -> &str {
        &self.port
    }

    fn open(&self) -> Result<Box<dyn Port>, String> {
        let port = serialport::new(&self.port, STARLIGHT_BAUD_RATE).timeout(Duration::MAX).open();

        if port.is_err() {
            Err(port.unwrap_err().description)
        } else {
            Ok(Box::new(DevicePort::new(port.unwrap())))
        }
    }
}

struct DevicePort {
    port: Box<dyn serialport::SerialPort>
}

impl DevicePort {
    fn new(port: Box<dyn serialport::SerialPort>) -> Self {
        Self {
            port
        }
    }
}

impl Port for DevicePort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.port.read(buf)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.port.write_all(buf)
    }
}
