mod java_glue;
pub use crate::java_glue::*;
use serialport::{self, FlowControl};
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
    baud_rate: u32,
    data_bits: serialport::DataBits,
    flow_control: serialport::FlowControl,
    parity: serialport::Parity,
    stop_bits: serialport::StopBits,
    timeout: u64
}

impl SerialPort {
    pub fn new(
        path: &str,
        baud_rate: i32,
        data_bits: i32,
        flow_control: i32,
        parity: i32,
        stop_bits: i32,
        timeout: i32,
    ) -> SerialPort {
        SerialPort {
            path: match Path::new(path).exists() {
                false => None,
                true => Some(path.to_string()),
            },
            baud_rate: baud_rate as u32,
            data_bits: match data_bits {
                5 => serialport::DataBits::Five,
                6 => serialport::DataBits::Six,
                7 => serialport::DataBits::Seven,
                8 => serialport::DataBits::Eight,
                _ => serialport::DataBits::Eight,
            },
            flow_control: match flow_control {
                1 => serialport::FlowControl::Software,
                2 => serialport::FlowControl::Hardware,
                _ => serialport::FlowControl::None,
            },
            parity: match parity {
                1 => serialport::Parity::Odd,
                2 => serialport::Parity::Even,
                _ => serialport::Parity::None,
            },
            stop_bits: match stop_bits {
                1 => serialport::StopBits::One,
                2 => serialport::StopBits::Two,
                _ => serialport::StopBits::One,
            },
            timeout: timeout as u64,
            port: None,
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
                        let builder = serialport::new(&path, self.baud_rate)
                            .data_bits(self.data_bits)
                            .flow_control(self.flow_control)
                            .parity(self.parity)
                            .stop_bits(self.stop_bits)
                            .timeout(Duration::from_millis(self.timeout));
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

    pub fn flow_control(&mut self) -> i32 {
        match self.port.as_mut() {
            Some(p) => match p.flow_control() {
                Ok(n) => match n {
                    FlowControl::None => 0,
                    FlowControl::Software => 1,
                    FlowControl::Hardware => 2,
                },
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

    pub fn read_carrier_detect(&mut self) -> bool {
        match self.port.as_mut() {
            Some(p) => match p.read_carrier_detect() {
                val => val.unwrap_or(false),
            },
            _ => false,
        }
    }

    pub fn read_clear_to_send(&mut self) -> bool {
        match self.port.as_mut() {
            Some(p) => match p.read_clear_to_send() {
                val => val.unwrap_or(false),
            },
            _ => false,
        }
    }

    pub fn read_data_set_ready(&mut self) -> bool {
        match self.port.as_mut() {
            Some(p) => match p.read_data_set_ready() {
                val => val.unwrap_or(false),
            },
            _ => false,
        }
    }

    pub fn read_ring_indicator(&mut self) -> bool {
        match self.port.as_mut() {
            Some(p) => match p.read_ring_indicator() {
                val => val.unwrap_or(false),
            },
            _ => false,
        }
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
