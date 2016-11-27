extern crate hd44780_simulator;

fn main() {
    let mut driver = hd44780_simulator::Simulator::driver();
    driver.write_slice(b" !Cool example! ");
    driver.set_ddram_address(0x40);
    driver.write_slice(b"0123456789#@+-=*");
    loop {}
}
