extern crate hd44780_simulator;

fn main() {
    let mut driver = hd44780_simulator::Simulator::driver();
    driver.define_glyph(0, &[0b01110,
                             0b10001,
                             0b10000,
                             0b01000,
                             0b00100,
                             0b00010,
                             0b00001,
                             0]);
    driver.define_glyph(1, &[0b01110,
                             0b10001,
                             0b00001,
                             0b00010,
                             0b00100,
                             0b01000,
                             0b10000,
                             0]);
    driver.write_at((0, 0), b"\x00\x01Cool example\x00\x01");
    driver.write_at((1, 0), b"0123456789#@+-=*");
    loop {}
}
