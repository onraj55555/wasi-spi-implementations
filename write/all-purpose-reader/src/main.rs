use rppal::gpio::*;
use quanta::Clock;
use clap::Parser;

#[derive(Parser)]
struct Config {
    #[arg(short, long)]
    to_match: String
}

fn read_bytes(sck_pin: &InputPin, mosi_pin: &InputPin, cs_pin: &InputPin, clock: &Clock, to_match: &[u8]) {
    let mut count = 0;
    let mut byte = 0u8;
    let mut last_sck = false;
    
    let mut data : Vec<u8> = Vec::new();
    // To prevent delay when resizing as long as data transmitted is lower than 100 bytes
    data.reserve(100);
    
    let start = clock.now();
    
    while cs_pin.is_low() {
        let current_sck = sck_pin.is_high();
        
        // Rising edge
        if !last_sck && current_sck {
            let bit = match mosi_pin.read() {
                Level::Low => 0,
                Level::High => 1,
            };
            
            byte = (byte << 1) | bit;
            count += 1;
            
            if count == 8 {
                data.push(byte);
                byte = 0;
                count = 0;
            }
        }
        
        last_sck = current_sck;
    }
    let duration = start.elapsed();
    
    let mut ok = true;
    for i in 0..data.len() {
        if to_match[i] != data[i] { ok = false; break; }
    }
    
    match ok {
        true => print!("OK"),
        false => print!("ERR"),
    }
    
    println!(",{},{}", duration.as_nanos(), duration.as_nanos() / 8);
}

fn main() {
    let config = Config::parse();
    
    let gpio = Gpio::new().unwrap();
    let cs_pin = gpio.get(2).unwrap().into_input_pulldown();
    let sck_pin = gpio.get(3).unwrap().into_input_pulldown();
    let mosi_pin = gpio.get(4).unwrap().into_input_pulldown();

    let clock = quanta::Clock::new();

    eprintln!("Starting loop");

    loop {
        while cs_pin.is_high() {}

        read_bytes(&sck_pin, &mosi_pin, &cs_pin, &clock, config.to_match.as_bytes());
        
        while cs_pin.is_low() {}
    }
}
