wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

pub struct Component;

impl Guest for Component {
    fn start(d:Delay,) -> () {
        let config = wasi::spi::general::SpiConfig {bus: String::from("SPI0"), cs_pin: Some("0".to_string()), frequency: 1000, transmission_delay: 0, word_size: 8, mode: wasi::spi::general::SpiMode::Spi0};
        let spi = wasi::spi::controller::SpiController::get(&config).unwrap();

        let buffer = spi.read_n_words(64).unwrap();
        
        buffer.iter().for_each(|e| print!("{}", *e as char));
        println!();
    }
}

export!(Component);