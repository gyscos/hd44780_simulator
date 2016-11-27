extern crate hd44780_simulator;

fn main() {
    let driver = hd44780_simulator::Simulator::driver();
    loop {}
}
