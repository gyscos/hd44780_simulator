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

pub enum AddressCounter {
    Ddram((usize, usize)),
    Cgram(usize),
}

impl AddressCounter {
    pub fn shift(&mut self, direction: lcd_hd44780::commands::Direction) {
        if let &mut AddressCounter::Ddram((ref mut line, ref mut addr)) =
               self {
            match direction {
                lcd_hd44780::commands::Direction::Right => {
                    *addr += 1;
                    if *addr > 40 {
                        *addr = 0;
                        *line = 1 - *line;
                    }
                }
                lcd_hd44780::commands::Direction::Left => {
                    if *addr == 0 {
                        *addr = 40;
                        *line = 1 - *line;
                    }
                    *addr -= 1;
                }
            }
        }
    }
}

pub struct GraphicData {
    pub ddram: [[u8; 40]; 2],
    pub cgram: [[u8; 8]; 8],
    pub cgrom: [[u8; 8]; 96],

    pub characters: lcd_hd44780::commands::CharacterGrid,
    pub lines: lcd_hd44780::commands::LineCount,

    // Address Counter
    pub ac: AddressCounter,

    pub offset: usize,

    pub display: bool,
    pub cursor: bool,
    pub blink: bool,
}

impl GraphicData {
    pub fn new() -> Self {
        GraphicData {
            ddram: [[0x30u8; 40]; 2],
            cgram: [[0u8; 8]; 8],
            cgrom: include!("font.rs"),

            characters: lcd_hd44780::commands::CharacterGrid::C5x8,
            lines: lcd_hd44780::commands::LineCount::Two,

            ac: AddressCounter::Ddram((0, 0)),

            offset: 0,
            display: true,
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
    let mut window: PistonWindow = WindowSettings::new("hd44780 simulator",
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

    let pixel_fill = 3;
    let pixel_spacing = 1;
    let pixel_size = pixel_fill + pixel_spacing;

    let char_fill = Point {
        x: pixel_size * 5,
        y: pixel_size * 8,
    };
    let char_spacing = 3;
    let char_size = Point {
        x: char_fill.x + char_spacing,
        y: char_fill.y + char_spacing,
    };

    let color = [0.9, 0.9, 1.0, 1.0];

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            image(&texture, c.transform, g);

            // Draw the two lines
            let data = data.lock().unwrap();

            let first_line = &data.ddram[0];
            let second_line = &data.ddram[1];

            let mut draw_char = |character: &[u8; 8], offset: Point| {
                for (y, &line) in character.iter().enumerate() {
                    for x in 0..5 {
                        if (line & 1 << x) != 0 {
                            // The most significant bit is actually the left size
                            // So mirror it all
                            let x = 4 - x;
                            rectangle(color, [(offset.x + x * pixel_size) as f64, (offset.y + y * pixel_size) as f64,
                            pixel_fill as f64, pixel_fill as f64],
                            c.transform, g);
                        }
                    }
                }
            };

            let mut draw_line = |line: &[u8], offset: Point| {
                for (i, &code) in line.iter()
                    .skip(data.offset)
                        .chain(line.iter())
                        .take(16)
                        .enumerate() {

                            let character = if code < 8 {
                                &data.cgram[code as usize]
                            } else if code >= 32 {
                                &data.cgrom[code as usize - 32]
                            } else {
                                panic!("Bad character code: {}", code);
                            };

                            draw_char(character, Point { x: offset.x + i * char_size.x,
                                y: offset.y});

                        }
            };

            draw_line(first_line, offset);
            draw_line(second_line, Point { x: offset.x, y: offset.y + char_size.y });
        });
    }
}
