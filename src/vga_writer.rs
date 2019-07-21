/**
 * writer.rs
 * 
 * Holds the declaration and implementation of the writer to the terminal screen.
 */
pub mod vga_buffer;

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

        let row = BUFFER_HIEGHT - 1;
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
        _ -> self.write_byte(0xfe);
      }
    }
  }

  fn new_line(&mut self) {
    return;
  }
}
