use core::time;

wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

pub struct Component;

impl Guest for Component {
    fn start(d:Delay,) -> () {
        let data = "a";

        for speed in 1..1000 {
            let config = wasi::spi::general::SpiConfig{bus: String::from("SPI0"), cs_pin: Some(String::from("0")), frequency: speed * 10000, transmission_delay: 0, word_size: 8, mode: wasi::spi::general::SpiMode::Spi0};
            let spi = wasi::spi::controller::SpiController::get(&config).unwrap();

            println!("Speed: {} kHz", spi.get_config().unwrap().frequency/1000);
            
            spi.write_word(&data.as_bytes()).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

export!(Component);