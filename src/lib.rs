extern crate lcd_hd44780;
extern crate piston_window;
extern crate image;

use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub mod graphics;
pub mod pin;

use pin::{Pin, PinState};

pub struct Sleep;

impl lcd_hd44780::gpio::Sleep for Sleep {
    fn sleep(&self, ms: usize) {
        let millis = std::time::Duration::from_millis(ms as u64);
        std::thread::sleep(millis);
    }
}

pub struct Simulator {
    graphics: Arc<Mutex<graphics::GraphicData>>,
    text_direction: lcd_hd44780::commands::TextDirection,
    auto_shift: bool,

    enable: bool,
    rs: Rc<Cell<PinState>>,
    rw: Rc<Cell<PinState>>,
    data: [Rc<Cell<PinState>>; 8],
}

impl lcd_hd44780::gpio::Pin for Simulator {
    fn low(&mut self) {
        self.enable = false;
    }

    fn high(&mut self) {
        if self.enable {
            return;
        }

        self.enable = true;

        // Do stuff here
    }
}

impl Simulator {
    pub fn new() -> Self {
        Simulator {
            graphics: Arc::new(Mutex::new(graphics::GraphicData::new())),
            text_direction: lcd_hd44780::commands::TextDirection::LeftToRight,
            auto_shift: false,

            enable: false,
            rs: PinState::new(),
            rw: PinState::new(),
            data: [PinState::new(),
                   PinState::new(),
                   PinState::new(),
                   PinState::new(),
                   PinState::new(),
                   PinState::new(),
                   PinState::new(),
                   PinState::new()],
        }
    }

    pub fn driver
        ()
        -> lcd_hd44780::driver::Driver<Pin, Pin, ([Pin; 8], Self), Sleep>
    {
        let simulator = Simulator::new();

        let rs = Pin::new(simulator.rs.clone());
        let rw = Pin::new(simulator.rw.clone());
        let data = [Pin::new(simulator.data[0].clone()),
                    Pin::new(simulator.data[1].clone()),
                    Pin::new(simulator.data[2].clone()),
                    Pin::new(simulator.data[3].clone()),
                    Pin::new(simulator.data[4].clone()),
                    Pin::new(simulator.data[5].clone()),
                    Pin::new(simulator.data[6].clone()),
                    Pin::new(simulator.data[7].clone())];

        graphics::start_graphics(simulator.graphics.clone());

        lcd_hd44780::driver::Driver::new(rs, rw, (data, simulator), Sleep)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
