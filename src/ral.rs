//! Register access layer (RAL) for the CC1200 driver

use bitfield::bitfield;
use embedded_hal::spi::{Operation, SpiDevice};

use crate::registers::{Register, RegisterAddress};

/// If the address field in the first (header) byte of a register transaction is
/// set to this, the device switches into extended register access mode. The
/// subsequent byte is a register address in the extended register space.
const EXTENDED_REGISTER_MAGIC_ADDR: u8 = 0x2f;

bitfield! {
    struct Header(u8);
    impl new;
    u8;

    _, set_write_bit: 7;
    _, set_burst_bit: 6;
    _, set_address: 5, 0;
}

/// Read a register from the device using a single read.
pub fn read_register<Spi: SpiDevice, R: Register>(spi: &mut Spi) -> Result<R, Spi::Error> {
    let mut buf = [0_u8];
    match R::ADDRESS {
        RegisterAddress::Config(addr) => {
            let header = Header::new(false, false, addr & 0x3f).0;
            spi.transaction(&mut [Operation::Write(&[header]), Operation::Read(&mut buf)])?;
        }
        RegisterAddress::Extended(addr) => {
            let header = Header::new(false, false, EXTENDED_REGISTER_MAGIC_ADDR).0;
            spi.transaction(&mut [
                Operation::Write(&[header]),
                Operation::Write(&[addr]),
                Operation::Read(&mut buf),
            ])?;
        }
    }
    Ok::<R, Spi::Error>(buf[0].into())
}
