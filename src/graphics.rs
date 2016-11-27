
use lcd_hd44780;
use piston_window::*;

use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

enum AddressCounter {
    Ddram(usize),
    Cgram(usize),
}

pub struct GraphicData {
    ddram: [u8; 80],
    cgram: [u8; 64],

    characters: lcd_hd44780::commands::CharacterGrid,
    lines: lcd_hd44780::commands::LineCount,

    // Address Counter
    ac: AddressCounter,

    offset: usize,

    cursor: bool,
    blink: bool,
}

impl GraphicData {
    pub fn new() -> Self {
        GraphicData {
            ddram: [20u8; 80],
            cgram: [0u8; 64],

            characters: lcd_hd44780::commands::CharacterGrid::C5x8,
            lines: lcd_hd44780::commands::LineCount::Two,

            ac: AddressCounter::Ddram(0),

            offset: 0,
            cursor: false,
            blink: false,
        }
    }
}

pub fn start_graphics(data: Arc<Mutex<GraphicData>>) {
    thread::spawn(|| run_graphics(data));
}

fn run_graphics(data: Arc<Mutex<GraphicData>>) {
    let w = 483;
    let h = 206;
    let mut window: PistonWindow = WindowSettings::new("ili9163c simulator",
                                                       [w, h])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let image_data = include_bytes!("../assets/background.png");
    let img = ::image::load(Cursor::new(&image_data[..]), ::image::PNG)
        .unwrap();
    let texture = Texture::from_image(&mut window.factory,
                                      img.as_rgba8().unwrap(),
                                      &TextureSettings::new())
        .unwrap();
    let offset = Point { x: 60, y: 66 };

    let cellSize = 3;
    let spacing = 1;

    let char_spacing = 3;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            image(&texture, c.transform, g);
        });
    }

}
