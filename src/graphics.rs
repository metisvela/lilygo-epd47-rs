use embedded_graphics_core::{pixelcolor::BinaryColor, prelude::*};

use crate::{display::Display, Error};

impl<'a> DrawTarget for Display<'a> {
    type Color = BinaryColor;

    type Error = Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let result = self.set_pixel(coord.x as usize, coord.y as usize, color.is_on());
            if matches!(result, Err(Error::OutOfBounds)) {
                continue;
            }
            result?;
        }
        Ok(())
    }
}

impl<'a> OriginDimensions for Display<'a> {
    fn size(&self) -> Size {
        Size::new(Self::WIDTH as u32, Self::HEIGHT as u32)
    }
}
