use crate::*;
use bayer;
use std::io::Cursor;
extern crate image;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub type BayerPattern = bayer::CFA;
pub type Demosaic = bayer::Demosaic;
pub type Depth = bayer::BayerDepth;
pub type DebayerBuf = Vec<u8>;
pub fn cfa_from_u32(idx: u32) -> BayerPattern {
    match idx {
        0 => BayerPattern::RGGB,
        1 => BayerPattern::BGGR,
        2 => BayerPattern::GRBG,
        3 => BayerPattern::GBRG,
        _ => panic!("Not exsit  bayer patten"),
    }
}
#[derive(Debug, Clone)]
pub struct Debayer {
    width: u32,        // output width of image
    height: u32,       // output height of image
    cfa: BayerPattern, // bayer pattern of using Camera
}
impl Debayer {
    pub fn new(width: u32, height: u32, bayer_pattern: BayerPattern) -> Self {
        Self {
            width,
            height,
            cfa: bayer_pattern,
        }
    }
    pub fn run_from_buf(
        &self,
        buf: BufType,
        depth: Depth,
        alg: Demosaic,
    ) -> Result<DebayerBuf, bayer::BayerError> {
        info!("Starting debayer from buffer");
        self.run(&mut Cursor::new(&buf[..]), depth, alg)
    }
    pub fn run_from_file(
        &self,
        path: &str,
        depth: Depth,
        alg: Demosaic,
    ) -> Result<DebayerBuf, bayer::BayerError> {
        info!("Starting debayer from file {}",path);
        let mut file = File::open(Path::new(path)).unwrap();
        self.run(&mut file, depth, alg)
    }

    fn run(
        &self,
        buf: &mut dyn Read,
        depth: Depth,
        alg: Demosaic,
    ) -> Result<DebayerBuf, bayer::BayerError> {
        let raster_depth = match depth {
            Depth::Depth8 => bayer::RasterDepth::Depth8,
            _ => bayer::RasterDepth::Depth16,
        };
        let mut debayer_buf = vec![0; (self.width * self.height * 3) as usize];
        let mut dst = bayer::RasterMut::new(
            self.width as usize,
            self.height as usize,
            raster_depth,
            &mut debayer_buf,
        );
        //let cfa = bayer::CFA::GRBG;
        match bayer::run_demosaic(buf, depth, self.cfa, alg, &mut dst) {
            Ok(()) => Ok(debayer_buf),
            Err(e) => Err(e),
        }
    }

    pub fn buffer_to_rgb_image(&self, buffer: &[u8]) -> Result<image::RgbImage, image::ImageError> {
        let img_w = self.width;
        let img_h = self.height;
        let mut rgb_image = image::RgbImage::new(img_w, img_h);
        for y in 0..img_h {
            for x in 0..img_w {
                let buffer_idx = (y * img_w + x) as usize * 3;
                let r = buffer[buffer_idx];
                let g = buffer[buffer_idx + 1];
                let b = buffer[buffer_idx + 2];
                let pixel = image::Rgb([r, g, b]);
                rgb_image.put_pixel(x, y, pixel);
            }
        }

        Ok(rgb_image)
    }
}
