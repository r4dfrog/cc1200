#![no_std]
#![forbid(unsafe_code)]
#![warn(clippy::cargo)]

mod ral;
mod registers;

use embedded_hal::spi::SpiDevice;
use ral::read_register;
use registers::{PARTNUMBER, PARTVERSION};

pub struct CC1200<Spi: SpiDevice> {
    spi: Spi,
}

impl<Spi: SpiDevice> CC1200<Spi> {
    pub fn new(spi: Spi) -> Self {
        Self { spi }
    }

    pub fn part_number(&mut self) -> Result<u8, Spi::Error> {
        Ok(read_register::<Spi, PARTNUMBER>(&mut self.spi)?.into())
    }

    pub fn part_version(&mut self) -> Result<u8, Spi::Error> {
        Ok(read_register::<Spi, PARTVERSION>(&mut self.spi)?.into())
    }
}
