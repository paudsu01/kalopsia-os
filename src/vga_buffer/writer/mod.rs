#[allow(unused)]
pub trait VGAByteWriter {
    fn write_byte(&mut self, byte: u8);

    fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_byte(byte);
        }
    }

    fn write_string(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }
}

pub mod datetime_writer;
pub mod main_writer;
pub mod timer_writer;
