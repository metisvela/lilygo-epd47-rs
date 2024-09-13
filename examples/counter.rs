#![no_std]
#![no_main]

extern crate alloc;
extern crate lilygo_epd47;

use alloc::format;

use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::{prelude::*, text::Text};
use embedded_graphics_core::pixelcolor::BinaryColor;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::Io,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use lilygo_epd47::{pin_config, Display};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);
    let mut display = Display::new(
        pin_config!(io),
        peripherals.DMA,
        peripherals.LCD_CAM,
        peripherals.RMT,
        &clocks,
    );

    display.power_on();
    delay.delay_millis(10);

    let style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(200, 400))
        .digit_spacing(50)
        .segment_width(50)
        .segment_color(BinaryColor::On)
        .build();

    let mut counter = 0.0;
    loop {
        Text::new(
            format!("{:.1}", counter).as_str(),
            Point::new(50, 450),
            style,
        )
        .draw(&mut display)
        .unwrap();
        display.flush().unwrap();
        counter += 0.1;
    }
}
