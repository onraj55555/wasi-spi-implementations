use wasi::spi::{controller::SpiController, general::SpiConfig};

wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

pub struct Component;

impl Guest for Component {
    fn start(d:Delay,) -> () {
        let config = SpiConfig{bus: String::from("SPI0"), cs_pin: Some(String::from("0")), frequency: 10000, transmission_delay: 0, word_size: 8, mode: wasi::spi::general::SpiMode::Spi0};
        let spi = SpiController::get(&config).unwrap();

        let word = "abcdef".as_bytes();
        spi.write_word(&word).unwrap();
        d.delay_ms(1);
        spi.write_n_words(&word).unwrap();
    }
}

export!(Component);