use bitvec::{bitbox, prelude::*};
use esp_hal::{clock::Clocks, peripheral::Peripheral, peripherals};

use crate::{
    ed047tc1,
    ed047tc1::{DMA_BUFFER_SIZE, ED047TC1},
    Error,
    Result,
};

const FRAMEBUFFER_SIZE: usize = Display::WIDTH * Display::HEIGHT * 2;

pub struct Display<'a> {
    epd: ED047TC1<'a>,
    framebuffer: BitBox<u8>,
}

impl<'a> Display<'a> {
    /// Width of the screen.
    pub const WIDTH: usize = 960;
    /// Height of the screen
    pub const HEIGHT: usize = 540;

    pub fn new(
        pins: ed047tc1::PinConfig,
        dma: impl Peripheral<P = peripherals::DMA> + 'a,
        lcd_cam: impl Peripheral<P = peripherals::LCD_CAM> + 'a,
        rmt: impl Peripheral<P = peripherals::RMT> + 'a,
        clocks: &'a Clocks,
    ) -> Self {
        let mut framebuffer = bitbox![u8, Lsb0; 0; FRAMEBUFFER_SIZE];
        framebuffer.fill_with(|i| i % 2 != 0);
        Display {
            epd: ED047TC1::new(pins, dma, lcd_cam, rmt, clocks),
            framebuffer,
        }
    }

    /// Turn the display on.
    pub fn power_on(&mut self) {
        self.epd.power_on()
    }

    /// Turn the display off.
    pub fn power_off(&mut self) {
        self.epd.power_off()
    }

    /// Sets a single pixel in the framebuffer without updating the display.
    ///
    /// If the provided coordinates are outside the screen, this method returns
    /// [Error::OutOfBounds].
    pub fn set_pixel(&mut self, x: usize, y: usize, color: bool) -> Result<()> {
        if x > Self::WIDTH || y > Self::HEIGHT {
            return Err(Error::OutOfBounds);
        }
        // Calculate the index in the framebuffer.
        let index: usize = x + y * Self::WIDTH;
        self.framebuffer.set(index * 2, color);
        self.framebuffer.set(index * 2 + 1, !color);
        Ok(())
    }

    /// Flush updates the display with the contents of the framebuffer. The
    /// method clears the framebuffer. The provided mode should match the
    /// contents of your framebuffer.
    pub fn flush(&mut self) -> Result<()> {
        self.epd.frame_start()?;
        for chunk in self.framebuffer.as_raw_slice().chunks(DMA_BUFFER_SIZE) {
            self.epd.set_buffer(chunk);
            self.epd.output_row(300)?;
        }
        self.epd.frame_end()?;
        self.clear_framebuffer();
        Ok(())
    }

    /// Clears the screen.
    pub fn clear(&mut self) -> Result<()> {
        self.clear_framebuffer();
        self.flush()
    }

    /// Clears the framebuffer.
    fn clear_framebuffer(&mut self) {
        self.framebuffer.fill_with(|i| i % 2 != 0);
    }
}
