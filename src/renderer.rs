use crate::cpu::VRAM;
use crate::display::{Display, DisplayColor};

pub struct Renderer<'a> {
    display: &'a mut Display,
}

impl<'a> Renderer<'a> {
    pub fn new(display: &'a mut Display) -> Self {
        Renderer { display: display }
    }

    pub fn reset(&mut self) {
        self.display.clear_screen();
    }

    fn get_color(pixel: bool) -> DisplayColor {
        if pixel {
            DisplayColor::Black
        } else {
            DisplayColor::White
        }
    }

    pub fn render(&mut self, vram: &VRAM) {
        for (j, &line) in vram.iter().enumerate() {
            for (i, &pixel) in line.iter().enumerate() {
                self.display
                    .draw_pixel(i as i32, j as i32, Self::get_color(pixel))
            }
        }
        self.display.draw()
    }
}
