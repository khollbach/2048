#![no_std]

mod low_res;

use core::ptr;
use low_res::{Point, SCREEN_DIMS};

#[no_mangle]
fn main() {
    // ascii_table();

    // let board = Board::new();
    // board.render();

    test_print();

    loop {}
}

fn test_print() {
    let mut c = 0u8;
    for y in 0..24 {
        for x in 0..40 {
            print(Point { x, y }, &[c]);
            c = c.wrapping_add(1);
        }
    }
}

struct Board {
    cells: [[Cell; 4]; 4],
}

type Cell = Option<u16>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Board {
    fn new() -> Self {
        // todo: randomize initial state
        Self {
            cells: [
                [None, Some(2), None, None],
                [None, None, None, None],
                [None, None, Some(2), None],
                [None, None, None, None],
            ],
        }
    }

    fn swipe(&mut self, dir: Direction) {
        todo!()
    }

    fn render(&self) {
        let x_padding = 10;
        let y_padding = 8;

        for i in 0..4 {
            for j in 0..4 {
                let s = display_cell(self.cells[i as usize][j as usize]);
                let y = y_padding + i * 2;
                let x = x_padding + i * 5;

                print(Point { x, y }, &s);
            }
        }
    }
}

fn print(start: Point, s: &[u8]) {
    let mut coord = start;
    for &c in s {
        unsafe { ptr::write_volatile(coord.addr(), ascii_to_a2e(c)) };
        coord.x += 1;
    }
}

fn display_cell(cell: Cell) -> [u8; 4] {
    // let mut buf = Vec::<u8, 4>::new();
    // if let Some(mut value) = cell {
    //     debug_assert_ne!(value, 0);
    //     debug_assert!(value <= 9999);
    //     while value != 0 {
    //         let c = b'0' + (value % 10) as u8;
    //         buf.push(c).unwrap();
    //         value /= 10;
    //     }
    // }
    // buf.resize(4, b' ').unwrap();
    // buf.reverse();
    // buf.as_slice().try_into().unwrap()
    [b'2'; 4]
}

fn clear_screen() {
    for addr in 0x400..0x800 {
        unsafe { (addr as *mut u8).write_volatile(ascii_to_a2e(b' ')) };
    }
}

fn ascii_table() {
    clear_screen();

    for i in 0..16 {
        for j in 0..16 {
            let c = i << 4 | j;
            let p = Point {
                y: i as i8,
                x: j as i8,
            };
            unsafe { ptr::write_volatile(p.addr(), c) };
        }
    }

    for i in 0..16 {
        for j in 0..16 {
            let c = ascii_to_a2e(i << 4 | j);
            let p = Point {
                x: j as i8 + 20,
                y: i as i8,
            };
            unsafe { ptr::write_volatile(p.addr(), c) };
        }
    }
}

/// Convert an ASCII byte to the Apple IIe text mode glyphs.
fn ascii_to_a2e(c: u8) -> u8 {
    match c {
        // Control characters.
        0x00..=0x1f => 0xff,
        // Printable ASCII characters.
        0x20..=0x7f => c | 0x80,
        // High bit set -- that's not ASCII!
        0x80..=0xff => 0xff,
    }
}
