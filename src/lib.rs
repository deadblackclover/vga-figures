#![no_std]
#![feature(const_fn)]

use vga::colors::Color16;
use vga::writers::GraphicsWriter;

pub struct Figures<T> {
    mode: T,
}

impl<T: GraphicsWriter<Color16>> Figures<T> {
    /// Creates a new `Figures`.
    pub const fn new(mode: T) -> Figures<T> {
        Figures { mode }
    }

    /// Draw rectangle
    pub fn rectangle(&self, x: isize, y: isize, width: isize, height: isize, color: Color16) {
        self.mode.draw_line((x, y), (x + width, y), color);
        self.mode.draw_line((x, y), (x, y + width), color);
        self.mode
            .draw_line((x + width, y + height), (x, y + height), color);
        self.mode
            .draw_line((x + width, y + height), (x + width, y), color);
    }
}
