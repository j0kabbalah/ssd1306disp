use std::{
    error::Error,
    thread::sleep,
    time::Duration,
};

use chrono::Local;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use rppal::i2c::I2c;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

mod simple_error;
use simple_error::SimpleError;

mod bme280_sysfs;
use bme280_sysfs::BME280;

fn main() -> Result<(), Box<dyn Error>> {
    let i2c = I2c::new()?;
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().map_err(|_e| SimpleError::new(None, ""))?;

    let bme280 = BME280::new(1, 0x76);

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    loop {
        let time = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
        let ipaddr = format!(
            "IP: {}",
            local_ipaddress::get().unwrap_or("???".to_string())
        );
        let temp = format!("Temperature: {:3.2}C", bme280.temperature()?);
        let humi = format!("Humidity: {:3.2}%RH", bme280.humidity()?);
        let pres = format!("Pressure: {:3.2}hPa", bme280.pressure()?);

        display.clear();

        Text::with_baseline(&time, Point::new(1, 1), text_style, Baseline::Top)
            .draw(&mut display)
            .map_err(|_e| SimpleError::new(None, ""))?;
        Text::with_baseline(&ipaddr, Point::new(1, 12), text_style, Baseline::Top)
            .draw(&mut display)
            .map_err(|_e| SimpleError::new(None, ""))?;
        Text::with_baseline(&temp, Point::new(1, 23), text_style, Baseline::Top)
            .draw(&mut display)
            .map_err(|_e| SimpleError::new(None, ""))?;
        Text::with_baseline(&humi, Point::new(1, 34), text_style, Baseline::Top)
            .draw(&mut display)
            .map_err(|_e| SimpleError::new(None, ""))?;
        Text::with_baseline(&pres, Point::new(1, 45), text_style, Baseline::Top)
            .draw(&mut display)
            .map_err(|_e| SimpleError::new(None, ""))?;

        display.flush().map_err(|_e| SimpleError::new(None, ""))?;

        sleep(Duration::from_secs(1u64));
    }
}
