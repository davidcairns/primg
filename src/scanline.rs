use image::math::utils::clamp;

#[derive(Debug)]
pub struct Scanline {
    pub y: usize,
    pub x1: usize,
    pub x2: usize,
}

impl Scanline {
    pub fn empty() -> Scanline {
        Scanline { y: 0, x1: 0, x2: 0 }
    }

    pub fn validating_set(&mut self, w: usize, h: usize, y: i32, x1: i32, x2: i32) -> bool {
        let w = w as i32;
        let h = h as i32;
        if (y < 0 || y >= h) || x1 >= w || x2 < 0 {
            return false;
        } else {
            let x1 = clamp(x1, 0, w - 1);
            let x2 = clamp(x2, 0, w - 1);
            if x1 > x2 {
                return false;
            }
            self.y = y as usize;
            self.x1 = x1 as usize;
            self.x2 = x2 as usize;
            return true;
        }
    }
}

