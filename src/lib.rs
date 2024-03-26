//! # TODO
//!    * Docs
//!    * Tests
//!    * Examples
//!    * Github CI
//!    * Cleanup
//!    * Settable I2C address
//!    * Settable sensitivities
//!    * More?

#![no_std]
#![allow(unused)]

pub struct Als31300<I2C> {
    i2c: I2C,
}

impl<I2C> Als31300<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    pub fn new(i2c: I2C) -> Self {
        Als31300 { i2c }
    }

    pub fn setup(&mut self) -> Result<(), I2C::Error> {
        let mut tx_buf = [0; 5];
        tx_buf[0] = C3DHALL9_REG_EEPROM_02;
        tx_buf[1] = ((C3DHALL9_EEPROM_02_ENABLE_XYZ >> 24) & 0xFF) as u8;
        tx_buf[2] = ((C3DHALL9_EEPROM_02_ENABLE_XYZ >> 16) & 0xFF) as u8;
        tx_buf[3] = ((C3DHALL9_EEPROM_02_ENABLE_XYZ >> 8) & 0xFF) as u8;
        tx_buf[4] = (C3DHALL9_EEPROM_02_ENABLE_XYZ & 0xFF) as u8;
        self.i2c.write(ADDRESS, &tx_buf).unwrap();
        tx_buf[0] = C3DHALL9_REG_VOLATILE_27;
        tx_buf[1] = ((C3DHALL9_VOLATILE_27_ACTIVE_MODE | C3DHALL9_VOLATILE_27_I2C_SINGLE >> 24)
            & 0xFF) as u8;
        tx_buf[2] = ((C3DHALL9_VOLATILE_27_ACTIVE_MODE | C3DHALL9_VOLATILE_27_I2C_SINGLE >> 16)
            & 0xFF) as u8;
        tx_buf[3] = ((C3DHALL9_VOLATILE_27_ACTIVE_MODE | C3DHALL9_VOLATILE_27_I2C_SINGLE >> 8)
            & 0xFF) as u8;
        tx_buf[4] =
            (C3DHALL9_VOLATILE_27_ACTIVE_MODE | C3DHALL9_VOLATILE_27_I2C_SINGLE & 0xFF) as u8;

        Ok(())
    }

    pub fn read_data(&mut self) -> Result<Data, I2C::Error> {
        let mut buf = [0; 8];
        self.i2c
            .write_read(ADDRESS, &[C3DHALL9_REG_VOLATILE_28], &mut buf)?;
        Ok(to_data(&buf))
    }
}

const ADDRESS: u8 = 0x60;

const C3DHALL9_REG_EEPROM_02: u8 = 0x02;
const C3DHALL9_REG_EEPROM_03: u8 = 0x03;
const C3DHALL9_REG_EEPROM_0D: u8 = 0x0D;
const C3DHALL9_REG_EEPROM_0E: u8 = 0x0E;
const C3DHALL9_REG_EEPROM_0F: u8 = 0x0F;
const C3DHALL9_REG_VOLATILE_27: u8 = 0x27;
const C3DHALL9_REG_VOLATILE_28: u8 = 0x28;
const C3DHALL9_REG_VOLATILE_29: u8 = 0x29;
const C3DHALL9_EEPROM_02_ENABLE_XYZ: u32 = 0x40 | 0x80 | 0x100;
const C3DHALL9_VOLATILE_27_ACTIVE_MODE: u32 = 0x00000000;
const C3DHALL9_VOLATILE_27_SLEEP_MODE: u32 = 0x00000001;
const C3DHALL9_VOLATILE_27_LOW_POWER_MODE: u32 = 0x00000002;
const C3DHALL9_VOLATILE_27_I2C_SINGLE: u32 = 0x00000000;
const C3DHALL9_VOLATILE_27_I2C_FAST_LOOP: u32 = 0x00000004;
const C3DHALL9_VOLATILE_27_I2C_FULL_LOOP: u32 = 0x00000008;
const C3DHALL9_SIGN_BIT: i16 = 0x0800;
// Resultion (LSB/Gauss) for ALS31300EEJASR-2000 [currently used]
// const C3DHALL9_GAUSS_RESOLUTION: u8 = 1;
// Resultion (LSB/Gauss) for ALS31300EEJASR-1000
// const C3DHALL9_GAUSS_RESOLUTION: u8 = 2;
// Resultion (LSB/Gauss) for ALS31300EEJASR-500 [Demoboard]
const C3DHALL9_GAUSS_RESOLUTION: i16 = 4;

fn to_data(buf: &[u8; 8]) -> Data {
    let mut raw_x: i16 = (buf[0] as i16) << 4 | ((buf[5] & 0x0F) as i16);
    let mut raw_y: i16 = (buf[1] as i16) << 4 | ((buf[6] >> 4) as i16);
    let mut raw_z: i16 = (buf[2] as i16) << 4 | ((buf[6] & 0x0F) as i16);
    let raw_temp: i32 = ((buf[3] & 0x3F) as i32) << 6 | (buf[3] & 0x3F) as i32;

    raw_x = (raw_x ^ C3DHALL9_SIGN_BIT) - C3DHALL9_SIGN_BIT;
    raw_y = (raw_y ^ C3DHALL9_SIGN_BIT) - C3DHALL9_SIGN_BIT;
    raw_z = (raw_z ^ C3DHALL9_SIGN_BIT) - C3DHALL9_SIGN_BIT;

    let data: Data = Data {
        x: raw_x / C3DHALL9_GAUSS_RESOLUTION,
        y: raw_y / C3DHALL9_GAUSS_RESOLUTION,
        z: raw_z / C3DHALL9_GAUSS_RESOLUTION,
        temp: (302 * (raw_temp - 1708) / 4096) as i16,
    };

    return data;
}

#[derive(Debug)]
pub struct Data {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub temp: i16,
}
