wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

pub struct Component;

impl Guest for Component {
    fn start() -> () {
        let config = wasi::spi::general::SpiConfig{bus: String::from("SPI0"), cs_pin: Some(String::from("0")), frequency: 1000, transmission_delay: 0, word_size: 8, mode: wasi::spi::general::SpiMode::Spi0};
        let spi = wasi::spi::controller::SpiController::get(&config).unwrap();
        
        spi.write_word(&[1]).unwrap();
    }
}

export!(Component);