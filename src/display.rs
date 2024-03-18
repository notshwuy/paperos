#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    White = 0x7,
    Gray = 0x8,
    Yellow = 0xE,

    BrightBlue = 0x9,
    BrightGreen = 0xA,
    BrightCyan = 0xB,
    BrightRed = 0xC,
    BrightMagenta = 0xD,
    BrightWhite = 0xF
}

pub const DEFAULT_COLOR: u8 = (Color::Brown as u8) | (Color::BrightWhite as u8) << 4; // (foreground, background)

pub struct Display {
    pub width: u8,
    pub height: u8,
            
    pub cursor_start: *mut u8,
    pub cursor_position: *mut u8,
}

impl Display {
    pub fn create(size: (u8, u8), buffer: *mut u8) -> Display {
        Display {
            width: size.0,
            height: size.1,
            
           
            cursor_start: buffer,
            cursor_position: buffer
        }
    }
}

unsafe fn write(buffer: *mut u8, color: u8, character: u8) {        
    *buffer = character;
    *buffer.offset(1) = color;
}

impl Display {
    pub fn clear(&mut self) {
        todo!()
    }
    
    pub fn append(&mut self, color: u8, text: &[u8]) -> Result<u8, ()> {
        let buffer = self.cursor_position.clone();
        let max_offset = unsafe { self.cursor_start.offset((self.width as isize * self.height as isize) * 2) };

        for (character_idx, &character_byte) in text.iter().enumerate() {
            if buffer > max_offset {
                return Err(());
            }
              
            unsafe {                
                write(buffer.offset(character_idx as isize * 2), color, character_byte);
                self.cursor_position = buffer.offset(character_idx as isize * 2);
            }
        }

        return Ok(text.len() as u8);
    }
}
