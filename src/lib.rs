extern crate lcd_hd44780;
extern crate piston_window;
extern crate image;
extern crate gpio_traits;

use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
pub mod graphics;
pub mod pin;

use gpio_traits::pin::PinState;
use pin::{Pin, BitPin};

pub struct Sleep;

pub type Driver = lcd_hd44780::driver::Driver<Pin, Pin, ([BitPin; 8], Simulator), Sleep>;

impl lcd_hd44780::gpio::Sleep for Sleep {
    fn sleep(&self, ms: usize) {
        let millis = std::time::Duration::new(0, 1000 * ms as u32);
        std::thread::sleep(millis);
    }
}

enum BitMode {
    EightBits,
    FourBits,
    FourBits2 { buffer: u8 },
}

pub struct Simulator {
    graphics: Arc<Mutex<graphics::GraphicData>>,

    enable: bool,
    rs: Rc<Cell<PinState>>,
    rw: Rc<Cell<PinState>>,

    bit_mode: BitMode,
    data: Rc<Cell<u8>>,
}

impl gpio_traits::pin::Output for Simulator {
    fn low(&mut self) {
        self.enable = false;
    }

    fn high(&mut self) {
        if self.enable {
            return;
        }

        self.enable = true;

        let data = match self.bit_mode {
            BitMode::EightBits => self.data.get(),
            BitMode::FourBits => {
                self.bit_mode = BitMode::FourBits2 { buffer: self.data.get() };
                return;
            }
            BitMode::FourBits2 { buffer } => buffer | self.data.get() >> 4,
        };

        // Do stuff here
        match self.rs.get() {
            PinState::Low => {
                // Instruction
                match data {
                    0 => {
                        // NOOP
                    }
                    0b00000001 => {
                        // Clear display
                        let mut graphics = self.graphics.lock().unwrap();
                        graphics.ddram = [[0x20; 40]; 2];
                        graphics.ac = graphics::AddressCounter::Ddram { line: 0, addr: 0 };
                        graphics.offset = 0;
                    }
                    0b00000010...0b00000011 => {
                        // Return home
                        let mut graphics = self.graphics.lock().unwrap();
                        graphics.ac = graphics::AddressCounter::Ddram { line: 0, addr: 0 };
                        graphics.offset = 0;
                    }
                    data @ 0b00000100...0b00000111 => {
                        // Set Entry Mode
                        let mut graphics = self.graphics.lock().unwrap();
                        graphics.text_direction =
                            lcd_hd44780::commands::TextDirection::from_u8(data);
                        graphics.auto_shift = (data & 1) != 0;
                    }
                    data @ 0b00001000...0b00001111 => {
                        // Display control
                        let mut graphics = self.graphics.lock().unwrap();
                        graphics.display = (data & 1 << 2) != 0;
                        graphics.cursor = (data & 1 << 1) != 0;
                        graphics.blink = (data & 1) != 0;
                    }
                    data @ 0b00010000...0b00010111 => {
                        // Cursor shift = AC shift
                        let mut graphics = self.graphics.lock().unwrap();
                        let direction = lcd_hd44780::commands::Direction::from_u8(data);
                        graphics.ac.shift(direction);

                    }
                    data @ 0b00011000...0b00011111 => {
                        // Display shift
                        //
                        let mut graphics = self.graphics.lock().unwrap();
                        let direction = lcd_hd44780::commands::Direction::from_u8(data);
                        graphics::shift_offset(&mut graphics.offset, 40, direction.switch());

                        // TODO: apply direction to offset
                    }
                    data @ 0b00100000...0b00111111 => {
                        // Function set
                        self.bit_mode = if (data & 1 << 4) != 0 {
                            BitMode::EightBits
                        } else {
                            BitMode::FourBits
                        };
                        // For now, ignore lines / font settings
                    }
                    data @ 0b01000000...0b01111111 => {
                        // Set CGRAM address
                        let mut graphics = self.graphics.lock().unwrap();
                        let cell = (data & 0b00111000) >> 3;
                        let addr = data & 0b00000111;
                        graphics.ac = graphics::AddressCounter::Cgram {
                            cell: cell,
                            addr: addr,
                        };
                    }
                    data @ 0b10000000...0b11111111 => {
                        // Set DRAM address
                        let mut graphics = self.graphics.lock().unwrap();
                        let mut addr = data & 0b01111111;
                        let line = if addr >= 0x40 {
                            addr -= 0x40;
                            1
                        } else {
                            0
                        };
                        graphics.ac = graphics::AddressCounter::Ddram {
                            line: line,
                            addr: addr,
                        };
                    }
                    _ => unreachable!(),
                }
            }
            PinState::High => {
                // Data
                let mut graphics = self.graphics.lock().unwrap();
                graphics.write(data);
            }
        }
    }
}

impl Simulator {
    pub fn new() -> Self {
        Simulator {
            graphics: Arc::new(Mutex::new(graphics::GraphicData::new())),

            enable: false,
            bit_mode: BitMode::EightBits,
            rs: pin::new_state(),
            rw: pin::new_state(),
            data: Rc::new(Cell::new(0)),
        }
    }

    pub fn driver() -> Driver {
        let simulator = Simulator::new();

        let rs = Pin::new(simulator.rs.clone());
        let rw = Pin::new(simulator.rw.clone());
        let data = BitPin::new_group(simulator.data.clone());

        graphics::start_graphics(simulator.graphics.clone());

        lcd_hd44780::driver::Driver::new(rs, rw, (data, simulator), Sleep)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
