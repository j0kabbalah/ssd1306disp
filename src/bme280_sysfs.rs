use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const TEMP_PATH: &str = "iio:device0/in_temp_input";
const HUMI_PATH: &str = "iio:device0/in_humidityrelative_input";
const PRES_PATH: &str = "iio:device0/in_pressure_input";

fn read_f(path: &str) -> io::Result<String> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buf: Vec<u8> = Vec::new();
    reader.read_until(16, &mut buf)?;
    let res = String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    return Ok(res);
}

pub struct BME280 {
    i2c: u8,
    addr: u8,
}

impl BME280 {
    pub fn new(i2c: u8, addr: u8) -> BME280 {
        BME280 { i2c, addr }
    }
    fn basepath(&self) -> String {
        format!("/sys/bus/i2c/devices/{:x}-{:04x}", self.i2c, self.addr)
    }
    pub fn temperature(&self) -> io::Result<f32> {
        let path = format!("{}/{}", self.basepath(), TEMP_PATH);
        let s = read_f(&path)?;
        // println!("temp raw: {}", s);
        let rawval: i32 = s
            .trim()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        // rawval is C * 1000
        let val: f32 = (rawval as f32) / 1000.0f32;
        Ok(val)
    }
    pub fn humidity(&self) -> io::Result<f32> {
        let path = format!("{}/{}", self.basepath(), HUMI_PATH);
        let s = read_f(&path)?;
        // println!("humi raw: {}", s);
        let rawval: i32 = s
            .trim()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        // rawval is RH% * 1000
        let val: f32 = (rawval as f32) / 1000.0f32;
        Ok(val)
    }
    pub fn pressure(&self) -> io::Result<f32> {
        let path = format!("{}/{}", self.basepath(), PRES_PATH);
        let s = read_f(&path)?;
        // println!("pres raw: {}", s);
        let rawval: f32 = s
            .trim()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        // rawval is kPa, conver to hPa
        let val: f32 = rawval * 10.0f32;
        Ok(val)
    }
}
