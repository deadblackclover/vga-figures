# vga-figures
:art: Library for drawing various shapes

## Figures2D
```rust
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};
use vga_figures::figures2d::Figures2D;

let mode = Graphics640x480x16::new();
mode.set_mode();
mode.clear_screen(Color16::Black);

let figures = Figures2D::new(mode);
figures.line(10, 10, 10, 60, Color16::White);
figures.rectangle(25, 10, 75, 60, Color16::White);
figures.circle(115, 35, 25, Color16::White);

let arr = [150, 100, 200, 120, 240, 180, 210, 200, 150, 150, 100, 200];
figures.polygon(&arr, Color16::White);

figures.text(10, 80, "Test text", Color16::White);
```
