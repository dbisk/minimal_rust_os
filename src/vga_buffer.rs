/**
 * vga_buffer.rs
 * 
 * This file holds the driver for the vga output to the screen. 
 */
use core::fmt;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

// private constants
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH : usize = 80;

// a global, static interface
lazy_static! {
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0, 
    color_code: ColorCode::new(Color::Yellow, Color::Black), 
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
  });
}


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
  Black       = 0x0, 
  Blue        = 0x1,
  Green       = 0x2,
  Cyan        = 0x3, 
  Brown       = 0x6,
  Red         = 0x4,
  Magenta     = 0x5,
  LightGray   = 0x7,
  DarkGray    = 0x8,
  LightBlue   = 0x9,
  LightGreen  = 0xA,
  LightCyan   = 0xB,
  LightRed    = 0xC,
  Pink        = 0xD,
  Yellow      = 0xE,
  White       = 0xF
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
  ascii_character: u8, 
  color_code: ColorCode
}

#[repr(transparent)]
struct Buffer {
  chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
  column_position: usize,
  color_code: ColorCode,
  buffer: &'static mut Buffer
}

impl Writer {
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let color_code = self.color_code;
        self.buffer.chars[row][col].write(ScreenChar {
          ascii_character: byte,
          color_code
        });
        self.column_position += 1;
      }
    }
  }

  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        // printable ASCII
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // otherwise, we print a box character
        _ => self.write_byte(0xfe)
      }
    }
  }

  fn new_line(&mut self) {
    return;
  }
}

impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}

// Testing function for VGA text-mode
pub fn vga_text_mode_test(color1: Color, color2: Color) {
  // write a string to the screen using the vga_writer
  let mut writer = Writer {
      column_position: 0,
      color_code: ColorCode::new(color1, color2),
      buffer: unsafe { &mut *(0xb8000 as *mut Buffer)}
  };

  writer.write_byte(b'H');
  writer.write_string("ello ");
  writer.write_string("World!");
}
