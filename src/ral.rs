use bitfield::bitfield;
use embedded_hal::spi::{Operation, SpiDevice};

use crate::registers::{Register, RegisterAddress};

const EXTENDED_REGISTER_MAGIC_ADDR: u8 = 0x2f;

bitfield! {
    struct Header(u8);

    _, set_write_bit: 7;
    _, set_burst_bit: 6;
    _, set_address: 5, 0;
}

pub fn read_register<Spi: SpiDevice, R: Register>(spi: &mut Spi) -> Result<R, Spi::Error> {
    let mut buf = [0_u8];
    match R::ADDRESS {
        RegisterAddress::Config(addr) => {
            let mut header = Header(0x00);
            header.set_address(addr & 0x3F);
            header.set_burst_bit(false);
            header.set_write_bit(false);
            spi.transaction(&mut [Operation::Write(&[header.0]), Operation::Read(&mut buf)])?;
        }
        RegisterAddress::Extended(addr) => {
            let mut header = Header(0x00);
            header.set_address(EXTENDED_REGISTER_MAGIC_ADDR);
            header.set_burst_bit(false);
            header.set_write_bit(false);
            spi.transaction(&mut [
                Operation::Write(&[header.0]),
                Operation::Write(&[addr]),
                Operation::Read(&mut buf),
            ])?;
        }
    };
    Ok::<R, Spi::Error>(buf[0].into())
}
