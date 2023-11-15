/// A position on the screen, in text or low-res mode.
#[derive(Clone, Copy)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

/// Assuming full-screen mode.
pub const SCREEN_DIMS: Point = Point { x: 40, y: 24 };

impl Point {
    fn in_bounds(self) -> bool {
        0 <= self.x && self.x < SCREEN_DIMS.x && 0 <= self.y && self.y < SCREEN_DIMS.y
    }

    pub fn addr(self) -> *mut u8 {
        debug_assert!(self.in_bounds());
        let x = self.x as u16;
        let y = self.y as u16;

        let base = match y / 8 {
            0 => 0x400,
            1 => 0x428,
            2 => 0x450,
            _ => unreachable!(),
        };
        let offset = y % 8 * 0x80;

        let addr = base + offset + x;
        addr as *mut u8
    }
}
