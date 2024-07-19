mod java_glue;
pub use crate::java_glue::*;
use serialport;
use std::path::Path;
use std::time::Duration;

fn vec_i8_into_u8(v: Vec<i8>) -> Vec<u8> {
    let mut v = std::mem::ManuallyDrop::new(v);
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
}

fn vec_u8_into_i8(v: Vec<u8>) -> Vec<i8> {
    let mut v = std::mem::ManuallyDrop::new(v);
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
}

// ANCHOR: rust_code
struct SerialPort {
    path: Option<String>,
    port: Option<Box<dyn serialport::SerialPort>>,
    stop_bits: serialport::StopBits,
}

impl SerialPort {
    pub fn new(path: &str, baud_rate: i32, stop_bits: i32) -> SerialPort {
        SerialPort {
            path: match Path::new(path).exists() {
                false => None,
                true => Some(path.to_string()),
            },
            stop_bits: match stop_bits {
                1 => serialport::StopBits::One,
                2 => serialport::StopBits::Two,
                _ => serialport::StopBits::One,
            },
            port: match Path::new(path).exists() {
                false => None,
                true => {
                    let stop_bits = match stop_bits {
                        1 => serialport::StopBits::One,
                        2 => serialport::StopBits::Two,
                        _ => serialport::StopBits::One,
                    };
                    match serialport::new(path, baud_rate as u32)
                        .data_bits(serialport::DataBits::Eight)
                        .parity(serialport::Parity::None)
                        .stop_bits(stop_bits)
                        .timeout(Duration::from_millis(1000))
                        .open()
                    {
                        Err(_) => None,
                        Ok(p) => Some(p),
                    }
                }
            },
        }
    }

    pub fn open(&mut self) -> bool {
        match self.port {
            Some(_) => true,
            None => match self.path.clone() {
                None => false,
                Some(path) => match Path::new(&path).exists() {
                    false => return false,
                    true => {
                        let builder = serialport::new(&path, 9600)
                            .data_bits(serialport::DataBits::Eight)
                            .parity(serialport::Parity::None)
                            .stop_bits(self.stop_bits)
                            .timeout(Duration::from_millis(1000));
                        match builder.open() {
                            Ok(p) => {
                                self.port = Some(p);
                                true
                            }
                            _ => false,
                        }
                    }
                },
            },
        }
    }

    pub fn close(&mut self) -> bool {
        if self.port.is_some() {
            self.port = None
        };
        true
    }

    pub fn bytes_to_read(&mut self) -> i32 {
        match self.port.as_mut() {
            Some(p) => match p.bytes_to_read() {
                Ok(n) => n as i32,
                _ => 0,
            },
            _ => 0,
        }
    }

    pub fn read(&mut self, length: i32) -> Vec<i8> {
        let mut vec: Vec<u8> = vec![0; length as usize];
        match self.port.as_mut() {
            Some(p) => match p.read(vec.as_mut()) {
                _ => {}
            },
            _ => {}
        };
        vec_u8_into_i8(vec)
    }

    pub fn read_exact(&mut self, length: i32) -> Vec<i8> {
        let mut vec: Vec<u8> = vec![0; length as usize];
        match self.port.as_mut() {
            Some(p) => match p.read_exact(vec.as_mut()) {
                _ => {}
            },
            _ => {}
        };
        vec_u8_into_i8(vec)
    }

    pub fn read_to_eol(&mut self) -> Vec<i8> {
        let mut vec: Vec<u8> = Vec::new();
        match self.port.as_mut() {
            Some(p) => match p.read_to_end(vec.as_mut()) {
                _ => {}
            },
            _ => {}
        };
        vec_u8_into_i8(vec)
    }

    pub fn write(&mut self, buf: Vec<i8>) -> i32 {
        let u8_vec: Vec<u8> = vec_i8_into_u8(buf);
        match self.port.as_mut() {
            Some(p) => match p.write(&u8_vec) {
                Ok(r) => r as i32,
                Err(_) => -1,
            },
            _ => -1,
        }
    }
}
// ANCHOR_END: rust_code
