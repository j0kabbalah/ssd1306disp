use std::{
    sync::mpsc,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

use anyhow::Result;

mod error;
use error::Error;

use chrono::Local;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};
use rppal::i2c::I2c;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use procfs::{Current, LoadAverage};

mod bme280_sysfs;
use bme280_sysfs::BME280;

fn main() -> Result<()> {
    let i2c = I2c::new()?;
    let interface = I2CDisplayInterface::new(i2c);
    let bme280 = BME280::new(1, 0x76);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().map_err(|e| Error::from(e))?;
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let mut sigs = Signals::new(&[SIGINT, SIGTERM])?;
    let mut flag = true;

    clear_screen(&mut display)?;
    message_display(&mut display, text_style, "Welcome")?;
    sleep(Duration::from_secs(5));

    let period = Duration::from_millis(200);
    let timeout = Duration::from_millis(1);
    let mut ptime = Instant::now();

    // check signals
    let (tx, rx) = mpsc::channel::<bool>();
    spawn(move || {
        for _sig in sigs.forever() {
            tx.send(false).unwrap();
            break;
        }
    });

    while flag {
        flag = rx.recv_timeout(timeout).is_err();
        if ptime.elapsed() >= period {
            main_display(&bme280, &mut display, text_style)?;
            ptime = Instant::now();
        }
        sleep(Duration::from_millis(1));
    }

    message_display(&mut display, text_style, "Good Bye")?;
    sleep(Duration::from_secs(5));

    clear_screen(&mut display)?;
    Ok(())
}

/// 画面を消去する。
fn clear_screen(
    display: &mut Ssd1306<
        I2CInterface<I2c>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
    >,
) -> Result<()> {
    display.clear(BinaryColor::Off).map_err(|e| Error::from(e))?;
    display.flush().map_err(|e| Error::from(e))?;
    Ok(())
}

/// 起動画面
fn message_display(
    display: &mut Ssd1306<
        I2CInterface<I2c>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
    >,
    text_style: embedded_graphics::mono_font::MonoTextStyle<'_, BinaryColor>,
    text: &str,
) -> Result<()> {
    display.clear(BinaryColor::Off).map_err(|e| Error::from(e))?;

    let maxsize = Size::new(
        DisplaySize128x64::WIDTH as u32,
        DisplaySize128x64::HEIGHT as u32,
    );
    let textlen: u32 = text_style.font.character_size.width * text.chars().count() as u32;
    let mut x: i32 = ((maxsize.width - textlen) / 2).try_into()?;
    if x < 0 {
        x = 0;
    }
    let pos = Point::new(x, (maxsize.height / 2) as i32);

    Rectangle::new(Point::zero(), maxsize)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .map_err(|e| Error::from(e))?;
    Text::with_baseline(text, pos, text_style, Baseline::Middle)
        .draw(display)
        .map_err(|e| Error::from(e))?;
    display.flush().map_err(|e| Error::from(e))?;
    Ok(())
}

/// メイン画面
fn main_display(
    bme280: &BME280,
    display: &mut Ssd1306<
        I2CInterface<I2c>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
    >,
    text_style: embedded_graphics::mono_font::MonoTextStyle<'_, BinaryColor>,
) -> Result<()> {
    let time = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let ipaddr = format!(
        "IP: {}",
        local_ipaddress::get().unwrap_or("???".to_string())
    );
    let temp = format!("Temperature: {:3.2}C", bme280.temperature()?);
    let humi = format!("Humidity: {:3.2}%RH", bme280.humidity()?);
    let pres = format!("Pressure: {:3.2}hPa", bme280.pressure()?);

    let la_val = LoadAverage::current().map_err(|e| Error::from(e))?;
    let la = format!("LA: {:3.2}/{:3.2}/{:3.2}", la_val.one, la_val.five, la_val.fifteen);

    display.clear(BinaryColor::Off).map_err(|e| Error::from(e))?;

    Text::with_baseline(&time, Point::new(1, 1), text_style, Baseline::Top)
        .draw(display)
        .map_err(|e| Error::from(e))?;
    Text::with_baseline(&ipaddr, Point::new(1, 12), text_style, Baseline::Top)
        .draw(display)
        .map_err(|e| Error::from(e))?;
    Text::with_baseline(&temp, Point::new(1, 23), text_style, Baseline::Top)
        .draw(display)
        .map_err(|e| Error::from(e))?;
    Text::with_baseline(&humi, Point::new(1, 34), text_style, Baseline::Top)
        .draw(display)
        .map_err(|e| Error::from(e))?;
    Text::with_baseline(&pres, Point::new(1, 45), text_style, Baseline::Top)
        .draw(display)
        .map_err(|e| Error::from(e))?;
    Text::with_baseline(&la, Point::new(1, 56), text_style, Baseline::Top)
        .draw(display)
        .map_err(|e| Error::from(e))?;

    display.flush().map_err(|e| Error::from(e))?;

    Ok(())
}
