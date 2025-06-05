use clap::Parser;

use rppal::gpio::{Gpio, InputPin, OutputPin, Level};

const CS_PIN: u8 = 2;     // GPIO8 = CE0
const SCLK_PIN: u8 = 3;  // GPIO11 = SCLK
const MISO_PIN: u8 = 17;   // GPIO9 = MISO

#[derive(Parser)]
struct Config {
    #[arg(short, long)]
    write: String
}

fn main() {
    let config = Config::parse();
    let data = config.write.as_bytes();
    let gpio = Gpio::new().expect("Failed to access GPIO");

    let cs = gpio.get(CS_PIN).unwrap().into_input();
    let sclk = gpio.get(SCLK_PIN).unwrap().into_input();
    let mut miso = gpio.get(MISO_PIN).unwrap().into_output();

    println!("Waiting for CS to go LOW (active)...");
    wait_for_level(&cs, Level::Low);

    println!("CS active, starting SPI transmission...");
    send_bytes(&sclk, &cs, &mut miso, &data);

    // Optionally reset the line
    miso.write(Level::Low);
    println!("Data sent. Done.");
}

fn send_bytes(clock: &InputPin, cs: &InputPin, miso: &mut OutputPin, bytes: &[u8]) {
    for byte in bytes {
        if let Err(()) = send_byte(clock, cs, miso, *byte) {
            return;
        }
    }
}

fn send_byte(clock: &InputPin, cs: &InputPin, miso: &mut OutputPin, byte: u8) -> Result<(), ()> {
    for i in (0..8).rev() {
        // Check if CS is still low; abort if high
        if cs.read() == Level::High {
            println!("CS went high, aborting.");
            return Err(());
        }

        // Prepare the bit on MISO
        let bit = (byte >> i) & 0x01;
        miso.write(if bit == 1 { Level::High } else { Level::Low });

        // Wait for SCLK rising edge (Mode 0)
        wait_for_rising_edge(clock);
    }
    
    Ok(())
}

fn wait_for_rising_edge(clock: &InputPin) {
    // Wait for clock to go LOW
    while clock.read() == Level::High {}

    // Wait for clock to go HIGH (rising edge)
    while clock.read() == Level::Low {}
}

fn wait_for_level(pin: &InputPin, level: Level) {
    while pin.read() != level {}
}