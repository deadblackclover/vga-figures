use vga::colors::Color16;
use vga::writers::GraphicsWriter;

/// 2D shapes
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

    /// Draw ellipse
    pub fn ellipse(&self, x: isize, y: isize, a: isize, b: isize, color: Color16) {
        let asq = a * a;
        let bsq = b * b;

        self.pixel(x, y + b, color);
        self.pixel(x, y - b, color);

        let mut wx = 0;
        let mut wy = b;
        let mut xa = 0;
        let mut ya = asq * 2 * b;
        let mut thresh = asq / 4 - asq * b;

        loop {
            thresh += xa + bsq;

            if thresh >= 0 {
                ya -= asq * 2;
                thresh -= ya;
                wy -= 1;
            }

            xa += bsq * 2;
            wx += 1;

            if xa >= ya {
                break;
            }

            self.pixel(x + wx, y - wy, color);
            self.pixel(x - wx, y - wy, color);
            self.pixel(x + wx, y + wy, color);
            self.pixel(x - wx, y + wy, color);
        }

        self.pixel(x + a, y, color);
        self.pixel(x - a, y, color);

        wx = a;
        wy = 0;
        xa = bsq * 2 * a;

        ya = 0;
        thresh = bsq / 4 - bsq * a;

        loop {
            thresh += ya + asq;

            if thresh >= 0 {
                xa -= bsq * 2;
                thresh = thresh - xa;
                wx -= 1;
            }

            ya += asq * 2;
            wy += 1;

            if ya > xa {
                break;
            }

            self.pixel(x + wx, y - wy, color);
            self.pixel(x - wx, y - wy, color);
            self.pixel(x + wx, y + wy, color);
            self.pixel(x - wx, y + wy, color);
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
