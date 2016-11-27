

use lcd_hd44780;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum PinState {
    Low,
    High,
}

pub struct BitPin {
    byte: Rc<Cell<u8>>,
    offset: u8,
}

impl BitPin {
    pub fn new(byte: Rc<Cell<u8>>, offset: u8) -> Self {
        BitPin {
            byte: byte,
            offset: offset,
        }
    }

    pub fn new_group(byte: Rc<Cell<u8>>) -> [BitPin; 8] {
        [BitPin::new(byte.clone(), 0),
         BitPin::new(byte.clone(), 1),
         BitPin::new(byte.clone(), 2),
         BitPin::new(byte.clone(), 3),
         BitPin::new(byte.clone(), 4),
         BitPin::new(byte.clone(), 5),
         BitPin::new(byte.clone(), 6),
         BitPin::new(byte.clone(), 7)]
    }
}

impl lcd_hd44780::gpio::Pin for BitPin {
    fn high(&mut self) {
        let byte = self.byte.get();
        self.byte.set(byte | (1 << self.offset));
    }

    fn low(&mut self) {
        let byte = self.byte.get();
        self.byte.set(byte & !(1 << self.offset));
    }
}

impl PinState {
    pub fn new() -> Rc<Cell<Self>> {
        Rc::new(Cell::new(PinState::Low))
    }
}

pub struct Pin {
    state: Rc<Cell<PinState>>,
}

impl Pin {
    pub fn new(state: Rc<Cell<PinState>>) -> Self {
        Pin { state: state }
    }
}

impl lcd_hd44780::gpio::Pin for Pin {
    fn high(&mut self) {
        self.state.set(PinState::High);
    }

    fn low(&mut self) {
        self.state.set(PinState::Low);
    }
}
