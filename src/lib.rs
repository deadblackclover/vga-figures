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
    pub fn rectangle(&self, x1: isize, y1: isize, x2: isize, y2: isize, color: Color16) {
        self.mode.draw_line((x1, y1), (x1, y2), color);
        self.mode.draw_line((x1, y1), (x2, y1), color);
        self.mode.draw_line((x2, y2), (x1, y2), color);
        self.mode.draw_line((x2, y2), (x2, y1), color);
    }
}
