use std::{fmt::Display, io};

pub mod device;
pub mod qemu;

pub trait Serial {
    fn display(&self) -> &str;
    fn open(&self) -> Result<Box<dyn Port>, String>;
}

pub trait Port {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error>;
}

impl Display for dyn Serial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
