use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    colour_code: ColourCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_BUF_ADDR: usize = 0xb8000;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_pos: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

pub fn print_test() {
    let mut writer = Writer {
        column_pos: 0,
        colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
        buffer: unsafe { &mut *(VGA_BUF_ADDR as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}

impl ColourCode {
    fn new(fg: Colour, bg: Colour) -> Self {
        ColourCode((bg as u8) << 4 | (fg as u8))
    }
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        const UNPRINTABLE: u8 = 0xfe;

        s.bytes().for_each(|byte| match byte {
            0x20..=0x7e | b'\n' => self.write_byte(byte),
            _ => self.write_byte(UNPRINTABLE),
        })
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;

                let colour_code = self.colour_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    colour_code,
                });
                self.column_pos += 1;
            }
        }
    }

    fn new_line(&mut self) {
        todo!()
    }
}
