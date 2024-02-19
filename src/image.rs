use crate::RGB;
use std::io::{Cursor, Result, Write};
use std::ops::{Index, IndexMut};

pub trait Image {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn save(&self, writer: &mut dyn Write) -> Result<()>;
}

pub struct PPM {
    width: usize,
    height: usize,
    samples_per_pixel: u32,
    data: Vec<RGB>,
}

impl Index<(usize, usize)> for PPM {
    type Output = RGB;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let (y, x) = idx;
        &self.data[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for PPM {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let (y, x) = idx;
        &mut self.data[y * self.width + x]
    }
}

impl PPM {
    pub fn new(w: usize, h: usize, samples: u32) -> Self {
        Self {
            width: w,
            height: h,
            samples_per_pixel: samples,
            data: vec![RGB::default(); w * h],
        }
    }
}

impl Image for PPM {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn save(&self, writer: &mut dyn Write) -> Result<()> {
        let mut contents = Cursor::new(vec![]);
        write!(contents, "P3\n{} {}\n255\n", self.width, self.height)?;
        for i in 0..self.height {
            for j in 0..self.width {
                let idx = (i * self.width + j) as usize;
                let px = self.data[idx];
                px.write(self.samples_per_pixel, &mut contents)?
            }
        }
        writer.write(&contents.into_inner()).map(|_| ())
    }
}
