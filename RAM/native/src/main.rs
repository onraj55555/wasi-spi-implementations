use rppal::spi::*;

fn main() {
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1000, Mode::Mode0).unwrap();
    
    spi.write(&[1]).unwrap();
}
