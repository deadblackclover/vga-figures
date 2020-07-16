use vga::colors::Color16;
use vga::writers::GraphicsWriter;

pub struct Figures2D<T> {
    mode: T,
}

impl<T: GraphicsWriter<Color16>> Figures2D<T> {
    /// Creates a new `Figures2D`.
    pub const fn new(mode: T) -> Figures2D<T> {
        Figures2D { mode }
    }

    /// Draw pixel
    pub fn pixel(&self, x: isize, y: isize, color: Color16) {
        self.rectangle(x, y, x, y, color);
    }

    /// Draw line
    pub fn line(&self, x1: isize, y1: isize, x2: isize, y2: isize, color: Color16) {
        self.mode.draw_line((x1, y1), (x2, y2), color);
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

    /// Draw polygon
    pub fn polygon(&self, arr: &[isize], color: Color16) {
        let len = arr.len();
        if len % 2 == 0 && len >= 2 {
            let mut i = 0;
            while i <= len - 4 {
                self.mode
                    .draw_line((arr[i], arr[i + 1]), (arr[i + 2], arr[i + 3]), color);
                i += 2;
            }
            self.mode
                .draw_line((arr[0], arr[1]), (arr[len - 2], arr[len - 1]), color);
        }
    }

    /// Draw text
    pub fn text(&self, x: usize, y: usize, s: &str, color: Color16) {
        for (i, ch) in s.chars().enumerate() {
            // i * 8 - so that the text does not stick together
            self.mode.draw_character(x + i * 8, y, ch, color);
        }
    }
}
