use rppal::spi::*;

fn main() {
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1000, Mode::Mode0).unwrap();
    
    let mut buffer = vec![0; 64];
    spi.read(&mut buffer).unwrap();
    
    buffer.iter().for_each(|e| print!("{}", *e as char));
    println!();
}