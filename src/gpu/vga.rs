pub const VGA_BUFFER: u32 = 0xb8000;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub struct VGA {
    buffer: *mut u8
}

impl VGA {
    pub fn init() -> Self {
        VGA {
            buffer: VGA_BUFFER as *mut u8,
        }
    }

    pub fn println(&mut self, text: &[u8]) {
        for (i, &byte) in text.iter().enumerate() {
            unsafe {
                *self.buffer.offset(i as isize * 2) = byte;
                *self.buffer.offset(i as isize * 2 + 1) = 0xb;
            }
        }
    }
}
