use super::bindings;

pub fn get_bus(bus: &str) -> Result<rppal::spi::Bus, bindings::wasi::spi::general::SpiError> {
    let bus = match bus.to_lowercase().as_str() {
            "spi0" => rppal::spi::Bus::Spi0,
            "spi1" => rppal::spi::Bus::Spi1,
            "spi2" => rppal::spi::Bus::Spi2,
            "spi3" => rppal::spi::Bus::Spi3,
            "spi4" => rppal::spi::Bus::Spi4,
            "spi5" => rppal::spi::Bus::Spi5,
            "spi6" => rppal::spi::Bus::Spi6,
            _ => return Err(bindings::wasi::spi::general::SpiError::NotAllowed),
        };

    Ok(bus)
}

pub fn get_cs_pin(cs_pin: &str) -> Result<rppal::spi::SlaveSelect, bindings::wasi::spi::general::SpiError> {
    let cs_pin = match cs_pin {
            "0" => rppal::spi::SlaveSelect::Ss0,
            "1" => rppal::spi::SlaveSelect::Ss1,
            "2" => rppal::spi::SlaveSelect::Ss2,
            "3" => rppal::spi::SlaveSelect::Ss3,
            "4" => rppal::spi::SlaveSelect::Ss4,
            "5" => rppal::spi::SlaveSelect::Ss5,
            "6" => rppal::spi::SlaveSelect::Ss6,
            "7" => rppal::spi::SlaveSelect::Ss7,
            "8" => rppal::spi::SlaveSelect::Ss8,
            "9" => rppal::spi::SlaveSelect::Ss9,
            "10" => rppal::spi::SlaveSelect::Ss10,
            "11" => rppal::spi::SlaveSelect::Ss11,
            "12" => rppal::spi::SlaveSelect::Ss12,
            "13" => rppal::spi::SlaveSelect::Ss13,
            "14" => rppal::spi::SlaveSelect::Ss14,
            "15" => rppal::spi::SlaveSelect::Ss15,
            _ => return Err(bindings::wasi::spi::general::SpiError::NotAllowed),
        };
    
    Ok(cs_pin)
}

impl From<bindings::wasi::spi::general::SpiMode> for rppal::spi::Mode {
    fn from(value: bindings::wasi::spi::general::SpiMode) -> Self {
        match value {
            bindings::wasi::spi::general::SpiMode::Spi0 => Self::Mode0,
            bindings::wasi::spi::general::SpiMode::Spi1 => Self::Mode1,
            bindings::wasi::spi::general::SpiMode::Spi2 => Self::Mode2,
            bindings::wasi::spi::general::SpiMode::Spi3 => Self::Mode3,
        }
    }
}