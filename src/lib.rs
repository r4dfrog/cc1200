//! CC1200 radio transceiver IC driver, written on top of embedded-hal.

#![no_std]
#![forbid(unsafe_code)]
#![warn(
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    clippy::cargo,
    missing_docs
)]

mod ral;
mod registers;

use embedded_hal::spi::SpiDevice;
use ral::read_register;
use registers::{PARTNUMBER, PARTVERSION};

/// CC1200 Driver
pub struct CC1200<Spi: SpiDevice> {
    /// SPI device through which the `CC1200` IC is available
    spi: Spi,
}

impl<Spi: SpiDevice> CC1200<Spi> {
    /// Create a new CC1200 driver with a pre-existing spi device
    pub fn new(spi: Spi) -> Self {
        Self { spi }
    }

    /// Read the part number from the IC
    ///
    /// According to the datasheet, this can be either 0x20 for the CC1200 or
    /// the 0x21 for the CC1201.
    ///
    /// # Errors
    ///
    /// Propagates SPI errors from the underlying transaction.
    pub fn part_number(&mut self) -> Result<u8, Spi::Error> {
        Ok(read_register::<Spi, PARTNUMBER>(&mut self.spi)?.into())
    }

    /// Read the part version from the IC
    ///
    /// # Errors
    ///
    /// Propagates SPI errors from the underlying transaction.
    pub fn part_version(&mut self) -> Result<u8, Spi::Error> {
        Ok(read_register::<Spi, PARTVERSION>(&mut self.spi)?.into())
    }
}
