use rppal::spi::*;

fn main() {
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1000, Mode::Mode0).unwrap();
    
    let mut buffer : [u8; 1] = [0];
    spi.read(&mut buffer).unwrap();
    
    println!("{}", buffer[0] as char);
}
