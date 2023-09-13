use crate::*;
use bayer;
use std::io::Cursor;
extern crate image;
use std::time::{Duration, Instant};
use std::io::Read;
use std::fs::File;
use std::path::Path;

pub type BayerPattern = bayer::CFA;
pub type Demosaic = bayer::Demosaic;
pub type Depth = bayer::BayerDepth;
pub type DebayerBuf = Vec<u8>;
#[derive(Debug, Clone)]
pub struct Debayer {
    width: u32,              // output width of image
    height: u32,             // output height of image
    cfa: BayerPattern,       // bayer pattern of using Camera
    depth: Depth,            // such as RAW8 or RAW16
    endian: Endian, // The sequence of bits in which video frame data is stored in the buffer
}
impl Debayer {
    pub fn new(
        width: u32,
        height: u32,
        bayer_pattern: BayerPattern,
        depth: Depth,
        endian: Endian,
    ) -> Self {
        Self {
            width,
            height,
            cfa: bayer_pattern,
            depth,
            endian,
        }
    }
    pub fn run_from_buf(
        &mut self,
        buf: BufType,
        alg: Demosaic,
    ) -> Result<DebayerBuf, bayer::BayerError> {
        self.run( 
            &mut Cursor::new(&buf[..]),
            alg)
    }
    pub fn run_from_file(
        &mut self,
        path : &str,
        alg: Demosaic,
    ) -> Result<DebayerBuf, bayer::BayerError> {
    let mut file =
        File::open(Path::new(path)).unwrap();
        self.run(&mut file,alg)
    }
    
    fn run(&self, buf: &mut dyn Read, alg: Demosaic)-> Result<DebayerBuf, bayer::BayerError>{
        let raster_depth = match self.depth {
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
        match bayer::run_demosaic(
            buf,
            self.depth,
            self.cfa,
            alg,
            &mut dst,
        ) {
            Ok(()) => Ok(debayer_buf),
            Err(e) => Err(e),
        }
    }

    pub fn buffer_to_rgb_image(&self, buffer: &[u8]) -> Result<image::RgbImage, image::ImageError> {
        let img_w = self.width;
        let img_h = self.height;
        let mut rgb_image = image::RgbImage::new(img_w, img_h);
        let start_time = Instant::now();
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
        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time);
        let elapsed_ms = elapsed_time.as_secs() * 1000 + u64::from(elapsed_time.subsec_millis());

        debug!("elapsed time : {} ms", elapsed_ms);
        Ok(rgb_image)
    }
}
