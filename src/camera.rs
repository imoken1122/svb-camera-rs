use crate::utils;
use crate::*;
use crate::{
    debayer, libsvb,
    libsvb::{convert_err_code, ControlTypeState, ROIFormat, SVBError},
};

use image::{self};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
#[derive(Debug, Clone)]
pub struct Camera {
    pub id: i32,
    pub idx: i32,
    pub info: libsvb::SVB_CAMERA_INFO,
    pub prop: libsvb::SVB_CAMERA_PROPERTY,
    pub type2caps: HashMap<libsvb::SVB_CONTROL_TYPE, libsvb::SVB_CONTROL_CAPS>,
    pub roi : ROIFormat
}

pub trait ImageProcessor {
    fn save_img(&self, img :  image::RgbImage, extention: &str);
    fn save_raw(&self, buf: BufType);
    fn buf_to_img(&self, buffer: BufType, alg: debayer::Demosaic) -> Result<image::RgbImage,String>;
    fn buf_to_fits(&self, buf: BufType) -> BufType;
}

impl Camera {
    pub fn new(camera_idx: i32) -> Self {
        //camera_info.display_info();

        let mut camera = Camera {
            id: 0,
            idx: camera_idx,
            info: libsvb::SVB_CAMERA_INFO::new(),
            prop: libsvb::SVB_CAMERA_PROPERTY::new(),
            type2caps: HashMap::new(),
            roi : ROIFormat::new() 
        };
        camera
    }
    pub fn init(&mut self) {
        // get camera info
        self.info = libsvb::_get_camera_info(self.idx).unwrap();
        debug!("{}", self.info);
        self.id = self.info.CameraID;

        // camera open.
        self.open().unwrap();

        // get camera serial number .
        // Note: If you do not get the serial number first,
        // CameraID will not be recognized and you could not get the property.
        let sn = self.get_serial_number().unwrap();

        // get camera property
        self.prop = match libsvb::_get_camera_prop(self.id) {
            Ok(prop) => prop,
            Err(e) => {
                error!("{} got {}", e, self.id);
                self.prop
            }
        };
        debug!("{}", self.prop);

        self.roi = self.get_roi_format().unwrap();
        //get control capability and push to HashMap
        let num_of_ctls = self.get_num_of_controls().unwrap();
        for ctl_idx in 0..num_of_ctls {
            let ctl_caps = self.get_ctl_caps_by_idx(ctl_idx).unwrap();
            debug!("{}", ctl_caps);
            self.set_ctl_value( ctl_caps.ControlType, ctl_caps.DefaultValue,0);
            self.type2caps.insert(ctl_caps.ControlType, ctl_caps);
        }


    }
    pub fn open(&self) -> Result<(), SVBError> {
        match libsvb::_open_camera(self.id) {
            SVBError::Success => Ok(info!("Opened camera")),
            e => Err(e),
        }
    }
    pub fn close(&self) -> Result<(), SVBError> {
        match libsvb::_close_camera(self.id) {
            SVBError::Success => Ok(info!("Closed camera")),
            e => Err(e),
        }
    }
    pub fn get_info(&self) -> libsvb::SVB_CAMERA_INFO{
        self.info

    }
    pub fn get_property(&self) -> libsvb::SVB_CAMERA_PROPERTY{
        self.prop

    }

    pub fn get_num_of_controls(&self) -> Result<i32, SVBError> {
        let mut num_ctls = 0;
        match libsvb::_get_num_of_controls(self.id, &mut num_ctls) {

            0 => {
                debug!("Num of control types {}", num_ctls);
                Ok(num_ctls)},
            code => Err(convert_err_code(code)),
        }
    }

    pub fn get_ctl_caps_by_idx(&self, ctl_idx: i32) -> Result<libsvb::SVB_CONTROL_CAPS, SVBError> {
        let mut ctl_caps = libsvb::SVB_CONTROL_CAPS::new();
        match libsvb::_get_ctl_caps(self.id, ctl_idx, &mut ctl_caps) {
            0 => Ok(ctl_caps),
            code => Err(convert_err_code(code)),
        }
    }
    pub fn get_ctl_value(
        &self,
        ctl_type: libsvb::SVB_CONTROL_TYPE,
    ) -> Result<ControlTypeState, SVBError> {
        let mut value: libsvb::SVBControlValue = 0;
        let mut is_auto = 0;
        match libsvb::_get_ctl_value(self.id, ctl_type, &mut value, &mut is_auto) {
            SVBError::Success => {
                debug!("Get value {} of control type {}", value, ctl_type);
                Ok(ControlTypeState { value, is_auto })
            }
            e => Err(e),
        }
    }
    pub fn set_ctl_value(
        &self,
        ctl_type: libsvb::SVB_CONTROL_TYPE,
        value: libsvb::SVBControlValue,
        is_auto: u32,
    ) -> Result<(), SVBError> {
        match libsvb::_set_ctl_value(self.id, ctl_type, value, is_auto) {
            SVBError::Success => Ok(debug!("Set value {} of control type {}", value, ctl_type)),
            e => Err(e),
        }
    }
    pub fn start_video_capture(&self) -> Result<(), SVBError> {
        match libsvb::_start_video_capture(self.id) {
            SVBError::Success => Ok(info!("Starting video capture on camera_id {}", self.id)),
            e => Err(e),
        }
    }
    pub fn stop_video_capture(&self) -> Result<(), SVBError> {
        match libsvb::_stop_video_capture(self.id) {
            SVBError::Success => Ok(info!("Stopped video capture on camera_id {}", self.id)),
            e => Err(e),
        }
    }
    pub fn get_video_data(
        &self,
        pbuf: Option<*mut u8>,
        wait_ms: i32,
    ) -> Result<Option<BufType>, SVBError> {
        let buf_size = self.get_buffer_size();
        match pbuf {
            Some(pbuf) => match libsvb::_get_video_data(self.id, pbuf, buf_size, wait_ms) {
                SVBError::Success => Ok(None),
                e => Err(e),
            },
            None => {
                let mut buf = self.create_buffer(buf_size);
                let mut pbuf = buf.as_mut_ptr();
                match libsvb::_get_video_data(self.id, pbuf, buf_size, wait_ms) {
                    SVBError::Success => Ok(Some(buf)),
                    e => Err(e),
                }
            }
        }
    }
    pub fn get_video_frame(
        &self,
        wait_ms: i32
    ) -> Result<BufType, SVBError> {
        let buf_size = self.get_buffer_size();
        let mut buf = self.create_buffer(buf_size);
        let mut pbuf = buf.as_mut_ptr();
        match libsvb::_get_video_data(self.id, pbuf, buf_size, wait_ms) {
            SVBError::Success => Ok(buf),
            e => Err(e),
        }
    }
    pub fn get_roi_format(&self) -> Result<ROIFormat, SVBError> {
        let camera_id = self.id;

        let mut startx: i32 = 0;
        let mut starty: i32 = 0;
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut bin: i32 = 1;
        match libsvb::_get_roi_format(
            camera_id,
            &mut startx,
            &mut starty,
            &mut width,
            &mut height,
            &mut bin,
        ) {
            SVBError::Success => {
                let roi = ROIFormat {
                    startx,
                    starty,
                    width,
                    height,
                    bin,
                };
                debug!("Get ROI Format {}", roi);
                Ok(roi)
            }
            e => Err(e),
        }
    }
    pub fn set_roi_format(
        &mut self,
        startx: i32,
        starty: i32,
        width: i32,
        height: i32,
        bin: i32,
    ) -> Result<(), SVBError> {
        match libsvb::_set_roi_format(self.id, startx, starty, width, height, bin) {
            SVBError::Success => {
                self.roi = ROIFormat{ startx, starty, width, height, bin};
                Ok(debug!(
                "set ROI format startx : {}\nstarty:{}\nwidth:{}\nheight:{}\nbin:{}",
                startx, starty, width, height, bin
            ))},

            e => Err(e),
        }
    }
    pub fn get_img_type(&self) -> Result<libsvb::SVB_IMG_TYPE, SVBError> {
        let mut img_type = 0;
        match libsvb::_get_img_type(self.id, &mut img_type) {
            SVBError::Success => {
                debug!("Get image type {}", img_type);
                Ok(img_type)
            }
            e => Err(e),
        }
    }
    pub fn set_img_type(&self, img_type: libsvb::SVB_IMG_TYPE) -> Result<(), SVBError> {
        match libsvb::_set_img_type(self.id, img_type) {
            SVBError::Success => {
                debug!("Set image type {}", img_type);
                Ok(())
            }
            e => Err(e),
        }
    }
    pub fn get_serial_number(&self) -> Result<libsvb::SVB_SN, SVBError> {
        let mut sn = libsvb::SVB_SN::new();
        match libsvb::_get_serial_number(self.id, &mut sn) {
            SVBError::Success => Ok(sn),
            e => Err(e),
        }
    }
    fn create_buffer(&self, buf_size: BufSize) -> BufType {
        vec![0; buf_size as usize]
    }
    fn get_buffer_size(&self) -> BufSize {
        let roi = self.roi;
        let img_type = self.get_img_type().unwrap();
        let mut buf_size: i64 = roi.width as i64 * roi.height as i64;

        // IMG_RAW8 and IMG_Y8 is 1 byte, same size with w*h
        buf_size = match img_type {
            libsvb::SVB_IMG_TYPE_SVB_IMG_RAW16 => buf_size * 2,
            libsvb::SVB_IMG_TYPE_SVB_IMG_RGB24 => buf_size * 3,
            _ => buf_size,
        };
        buf_size
    }
    pub fn get_bayer_pattern(&self) -> u32 {
        self.prop.BayerPattern
    }
}

impl ImageProcessor for Camera {
    fn save_img(&self, img: image::RgbImage, extention: &str) {
        let ext = match extention {
            "jpg" => image::ImageFormat::Jpeg,
            "png" => image::ImageFormat::Png,
            "tiff" => image::ImageFormat::Tiff,
            _ => panic!("Not supported image extension"),
        };
        let output_path = utils::generate_filename(extention);
        match img.save_with_format(output_path.clone(), ext) {
            Ok(()) => debug!("Image saved to {}", output_path),
            Err(e) => panic!("Failed to save image : {}", e),
        }
    }
    fn save_raw(&self, buf: BufType) {
        let output_path = utils::generate_filename("raw");
        let mut file = match File::create(&output_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create file : {:?}", e);
                return;
            }
        };

        // バッファの内容をファイルに書き込む
        match file.write_all(&buf) {
            Ok(_) => debug!("Buffer saved to  {}", output_path),
            Err(e) => eprintln!("Failed to save buffer {:?}", e),
        }
    }
    fn buf_to_img(&self, buffer: BufType, alg: debayer::Demosaic) -> Result<image::RgbImage, String> {
        let roi = self.roi;
        let width = roi.width as u32;
        let height = roi.height as u32;
        let img_type = self.get_img_type().unwrap();
        let bayer_idx = self.get_bayer_pattern();
        let bayer_pattern = debayer::cfa_from_u32(bayer_idx);
        let runtime = debayer::Debayer::new(width, height, bayer_pattern);
        // convert to image by image type (RAW8,RAW16,RGB24,Y8)
        let debayer_buf = match img_type {
            libsvb::SVB_IMG_TYPE_SVB_IMG_RAW8 => {
                runtime.run_from_buf(buffer, debayer::Depth::Depth8, alg)
            }

            libsvb::SVB_IMG_TYPE_SVB_IMG_RAW16 => {
                runtime.run_from_buf(buffer, debayer::Depth::Depth16LE, alg)
            }

            _ => {error!("Not supoorted image type"); Err(bayer::BayerError::WrongDepth)}
        };

        Ok(runtime.buffer_to_rgb_image(&debayer_buf.unwrap()).unwrap())
    }

    /// buffer convert to fits format
    fn buf_to_fits(&self, buf: BufType) -> BufType {
        // amount of padding
        fn padding(n: usize) -> usize {
            match n % 2880 {
                0 => 0,
                a => 2880 - a,
            }
        }

        let mut fits = Vec::new();
        let roi = self.roi;
        let img_t = self.get_img_type().unwrap();

        let (w, h) = (
            if roi.width < 1000 {
                format!(" {}", roi.width)
            } else {
                format!("{}", roi.width)
            },
            if roi.height < 1000 {
                format!(" {}", roi.height)
            } else {
                format!("{}", roi.height)
            },
        );

        let bit = match img_t {
            libsvb::SVB_IMG_TYPE_SVB_IMG_RAW16 => "16",
            libsvb::SVB_IMG_TYPE_SVB_IMG_RAW8 => " 8",
            _ => panic!("Fits format is not supported RGB format"),
        };

        // section number of row is 80
        let header = [
            "SIMPLE  =                    T / FITS standard                                  ",
            &("BITPIX  =                   ".to_owned()
                + bit
                + " / bits per pixel                                 "),
            "NAXIS   =                    2 / number of axis                                 ",
            &("NAXIS1   =                ".to_owned()
                + w.as_str()
                + " / length of data axis 1                          "),
            &("NAXIS2   =                ".to_owned()
                + h.as_str()
                + " / length of data axis 2                          "),
            "END                                                                             ",
        ];

        // header section
        for (i, h) in header.into_iter().enumerate() {
            if h.len() != 80 {
                error!("length of header {} is {}, its must be 80.", i, h.len());
            }
            for b in h.as_bytes() {
                fits.push(*b);
            }
        }

        // length of header section and data section is 2880
        // padding using empty(32) to 2880
        for _ in 0..padding(fits.len()) {
            fits.push(32);
        }

        // data section
        for data in &buf {
            fits.push(*data)
        }

        fits
    }
}

mod test {

    use crate::libsvb;

    use super::Camera;
    use env_logger;

    #[test]
    fn test_new() {
        env_logger::init();
        let num = libsvb::_get_num_of_connected_cameras();
        if num == 0 {
            error!("not connected camera")
        } else {
            let camera = Camera::new(0);
            camera.close();
        }
    }
}
