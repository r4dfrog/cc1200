//! Register definitions for the CC1200.

#![expect(non_camel_case_types, reason = "Register names should match datasheet")]

use bitfield::bitfield;

/// Address of a register in the CC1200 IC.
///
/// This contains two variants, since there are two register address spaces in
/// the CC1200: config, and extended.
pub enum RegisterAddress {
    /// Address to a config register.
    ///
    /// This address is only six bits wide, and has to be lower than `0x2f`
    Config(u8),
    Extended(u8),
}

/// The required shape of a valid register type.
///
/// Since all registers in the CC1200 are eight bits wide, we should be able to
/// convert the type back and forth between a `u8`.
pub(crate) trait Register: From<u8> + Into<u8> {
    /// Address of the register
    const ADDRESS: RegisterAddress;
}

/// Helper for creating a mapping between a register struct and it's address in
/// the device. This will create a suitable [`Register`] implementation on the
/// struct.
macro_rules! register_mapping {
    ($name:ident, $region:ident($address:literal)) => {
        impl From<u8> for $name {
            fn from(value: u8) -> Self {
                Self(value)
            }
        }

        impl From<$name> for u8 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl Register for $name {
            const ADDRESS: RegisterAddress = RegisterAddress::$region($address);
        }
    };
}

bitfield! {
    pub struct PKT_CFG2(u8);

    pkt_cfg2_not_used,  _:                      7;
    byte_swap_en,       set_byte_swap_en:       6;
    fg_mode_en,         set_fg_mode_en:         5;
    cca_mode,           set_cca_mode:           4, 2;
    pkt_format,         set_pkt_format:         1, 0;
}

bitfield! {
    pub struct PKT_CFG1(u8);

    fec_en,             set_fec_en:             7;
    white_data,         set_white_data:         6;
    pn9_swap_en,        set_pn9_swap_en:        5;
    addr_check_cfg,     set_addr_check_cfg:     4, 3;
    crc_cfg,            set_crc_cfg:            2, 1;
    append_status,      set_append_status:      0;
}

bitfield! {
    pub struct PKT_CFG0(u8);

    pkt_cfg0_reserved7, set_pkt_cfg0_reserved7: 7;
    length_config,      set_length_config:      6, 5;
    pkt_bit_len,        set_pkt_bit_len:        4, 2;
    uart_mode_en,       set_uart_mode_en:       1;
    uart_swap_en,       set_uart_swap_en:       0;
}

bitfield! {
    pub struct PARTNUMBER(u8);

    partnum,            _:                      7, 0;
}

bitfield! {
    pub struct PARTVERSION(u8);

    partver,            _:                      7, 0;
}

register_mapping!(PKT_CFG2, Config(0x26));
register_mapping!(PKT_CFG1, Config(0x27));
register_mapping!(PKT_CFG0, Config(0x28));
register_mapping!(PARTNUMBER, Extended(0x8f));
register_mapping!(PARTVERSION, Extended(0x90));
