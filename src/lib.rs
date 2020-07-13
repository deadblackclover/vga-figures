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

    /// Draw pixel
    pub fn pixel(&self, x: isize, y: isize, color: Color16) {
        self.rectangle(x, y, x, y, color);
    }

    /// Draw rectangle
    pub fn rectangle(&self, x1: isize, y1: isize, x2: isize, y2: isize, color: Color16) {
        self.mode.draw_line((x1, y1), (x1, y2), color);
        self.mode.draw_line((x1, y1), (x2, y1), color);
        self.mode.draw_line((x2, y2), (x1, y2), color);
        self.mode.draw_line((x2, y2), (x2, y1), color);
    }

    /// Draw circle
    pub fn circle(&self, x0: isize, y0: isize, r: isize, color: Color16) {
        let mut f = 1 - r;
        let mut dd_f_x = 1;
        let mut dd_f_y = -2 * r;
        let mut x = 0;
        let mut y = r;

        self.pixel(x0, y0 + r, color);
        self.pixel(x0, y0 - r, color);
        self.pixel(x0 + r, y0, color);
        self.pixel(x0 - r, y0, color);

        while x < y {
            if f >= 0 {
                y -= 1;
                dd_f_y += 2;
                f += dd_f_y;
            }

            x += 1;
            dd_f_x += 2;
            f += dd_f_x;

            self.pixel(x0 + x, y0 + y, color);
            self.pixel(x0 - x, y0 + y, color);
            self.pixel(x0 + x, y0 - y, color);
            self.pixel(x0 - x, y0 - y, color);

            self.pixel(x0 + y, y0 + x, color);
            self.pixel(x0 - y, y0 + x, color);
            self.pixel(x0 + y, y0 - x, color);
            self.pixel(x0 - y, y0 - x, color);
        }
    }
}
