use spinning_top::Spinlock;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static::lazy_static! {
    pub static ref WRITER: Spinlock<Writer<'static>> = Spinlock::new(Writer {
        current_column: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub struct Writer<'b> {
    current_column: usize,
    color_code: ColorCode,
    buffer: &'b mut Buffer,
}

impl Writer<'_> {
    fn write_byte(&mut self, b: u8) {
        match b {
            b'\n' => self.write_newline(),
            c => {
                if self.current_column >= BUFFER_WIDTH {
                    self.write_newline();
                }
                self.buffer[BUFFER_HEIGHT - 1][self.current_column].write(ScreenChar {
                    ascii_character: c,
                    color_code: self.color_code,
                });
                self.current_column += 1;
            }
        }
    }

    fn write_newline(&mut self) {
        for y in 1..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                // SAFETY: We know that both y and x are within the buffer, this is to prevent
                // repeated bounds checks.
                unsafe {
                    let value = self.buffer.get_unchecked(y).get_unchecked(x).read();
                    self.buffer
                        .get_unchecked_mut(y - 1)
                        .get_unchecked_mut(x)
                        .write(value);
                }
            }
        }
        self.clear_line(BUFFER_HEIGHT - 1);
        self.current_column = 0;
    }

    fn clear_line(&mut self, row: usize) {
        for c in self.buffer[row].iter_mut() {
            c.write(ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            });
        }
    }
}

impl core::fmt::Write for Writer<'_> {
    fn write_str(&mut self, s: &str) -> core::result::Result<(), core::fmt::Error> {
        for b in s.bytes() {
            match b {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(b),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
        Ok(())
    }
}

type Buffer = [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

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
