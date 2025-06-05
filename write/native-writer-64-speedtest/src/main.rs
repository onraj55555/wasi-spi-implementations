use rppal::spi::*;

fn main() {
    let data = "a".repeat(64);
    
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1, Mode::Mode0).unwrap();
    
    for speed in 1..1000 {
        spi.set_clock_speed(speed * 10000).unwrap();
        println!("Speed: {} kHz", spi.clock_speed().unwrap()/1000);
        
        
        spi.write(&data.as_bytes()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
