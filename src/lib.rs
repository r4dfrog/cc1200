#![no_std]

use embedded_hal::spi::SpiDevice;

struct CC1200<Spi: SpiDevice> {
    spi: Spi,
}

impl<Spi: SpiDevice> CC1200<Spi> {
    fn new(spi: Spi) -> Self {
        Self {
            spi,
        }
    }
}