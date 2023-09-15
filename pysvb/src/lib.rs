#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use bayer;
use pyo3::prelude::*;
use svbony_camera_rs::camera::Camera;
use svbony_camera_rs::debayer;
use svbony_camera_rs::libsvb::{self, ROIFormat};
#[pyfunction]
fn get_num_of_camera() -> PyResult<i32> {
    Ok(libsvb::_get_num_of_connected_cameras())
}
#[pyclass]
#[derive(Debug, Clone)]
enum PyDemosaic {
    None,
    NearestNeighbour,
    Linear,
    Cubic,
}

#[pyclass]
#[derive(Debug, Clone)]
enum PyImgType {
    RAW8 = 0,
    RAW16 = 4,
    RGB24 = 10,
}
#[pyclass]
#[derive(Debug, Clone)]
enum PyControlType {
    GAIN,
    EXPOSURE,
    GAMMA,
    GAMMA_CONTRAST,
    WB_R,
    WB_G,
    WB_B,
    FLIP,             //reference: enum FLIP_STATUS
    FRAME_SPEED_MODE, //0:low speed, 1:medium speed, 2:high speed
    CONTRAST,
    SHARPNESS,
    SATURATION,

    AUTO_TARGET_BRIGHTNESS,
    BLACK_LEVEL,         //black level offset
    COOLER_ENABLE,       //0:disable, 1:enable
    TARGET_TEMPERATURE,  //unit is 0.1C
    CURRENT_TEMPERATURE, //unit is 0.1C
    COOLER_POWER,        //range: 0-100

    BAD_PIXEL_CORRECTION_ENABLE,
}
#[pyclass]
struct PyControlCaps{
    name: String,
    description: String,
    max_value: i64,
    min_value: i64,
    default_value: i64,
    is_auto_supported:u32,
    is_writable: u32,
    control_type: u32,
}


#[pymethods]
impl PyControlCaps{
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    #[getter]
    fn description(&self) -> &str {
        &self.description
    }

    #[getter]
    fn max_value(&self) -> i64 {
        self.max_value
    }

    #[getter]
    fn min_value(&self) -> i64 {
        self.min_value
    }

    #[getter]
    fn default_value(&self) -> i64 {
        self.default_value
    }

    #[getter]
    fn is_auto_supported(&self) -> u32 {
        self.is_auto_supported
    }

    #[getter]
    fn is_writable(&self) -> u32 {
        self.is_writable
    }

    #[getter]
    fn control_type(&self) -> u32 {
        self.control_type
    }

}
#[pyclass]
struct PyROIFormat {
    pub startx: i32,
    pub starty: i32,
    pub width: i32,
    pub height: i32,
    pub bin: i32,
}

#[pymethods]
impl PyROIFormat {
    #[getter]
    fn startx(&self) -> i32 {
        self.startx
    }

    #[getter]
    fn starty(&self) -> i32 {
        self.starty
    }

    #[getter]
    fn width(&self) -> i32 {
        self.width
    }

    #[getter]
    fn height(&self) -> i32 {
        self.height
    }

    #[getter]
    fn bin(&self) -> i32 {
        self.bin
    }
}

#[pyclass]
struct PyCameraInfo {
    friendly_name: String,
    camera_sn: String,
    port_type: String,
    device_id: u32,
    camera_id: i32,
}

#[pymethods]
impl PyCameraInfo {
    #[getter]
    fn friendly_name(&self) -> &str {
        &self.friendly_name
    }

    #[getter]
    fn camera_sn(&self) -> &str {
        &self.camera_sn
    }

    #[getter]
    fn port_type(&self) -> &str {
        &self.port_type
    }

    #[getter]
    fn device_id(&self) -> u32 {
        self.device_id
    }

    #[getter]
    fn camera_id(&self) -> i32 {
        self.camera_id
    }
}

#[pyclass]
struct PyCameraProperty {
    pub max_height: i64,
    pub max_width: i64,
    pub is_color_cam: u32,
    pub bayer_pattern: u32,
    pub supported_bins: [i32; 16],
    pub supported_video_formats: [i32; 8],
    pub max_bit_depth: i32,
    pub is_trigger_cam: u32,
}

#[pymethods]
impl PyCameraProperty {
    #[getter]
    fn max_height(&self) -> i64 {
        self.max_height
    }

    #[getter]
    fn max_width(&self) -> i64 {
        self.max_width
    }

    #[getter]
    fn is_color_cam(&self) -> u32 {
        self.is_color_cam
    }

    #[getter]
    fn bayer_pattern(&self) -> u32 {
        self.bayer_pattern
    }

    #[getter]
    fn supported_bins(&self) -> Vec<i32> {
        self.supported_bins.clone().to_vec()
    }

    #[getter]
    fn supported_video_formats(&self) -> Vec<i32> {
        self.supported_video_formats.clone().to_vec()
    }

    #[getter]
    fn max_bit_depth(&self) -> i32 {
        self.max_bit_depth
    }

    #[getter]
    fn is_trigger_cam(&self) -> u32 {
        self.is_trigger_cam
    }
}
#[pyclass]
pub struct PyCamera {
    pub inner: Camera,
}
#[pymethods]
impl PyCamera {
    #[new]
    fn new(camera_idx: i32) -> PyResult<PyCamera> {
        let mut camera = Camera::new(camera_idx);
        Ok(PyCamera { inner: camera })
    }
    fn init(&mut self) {
        self.inner.init();
    }
    fn close(&self) {
        self.inner.close().unwrap();
    }
    fn set_roi_format(&mut self, startx: i32, starty: i32, width: i32, height: i32, bin: i32) {
        self.inner
            .set_roi_format(startx, starty, width, height, bin)
            .unwrap();
        self.inner.roi = ROIFormat {
            startx,
            starty,
            width,
            height,
            bin,
        };
    }
    fn get_num_of_controls(&self,) -> PyResult<i32> {
        Ok(self.inner.get_num_of_controls().unwrap())
    }
    fn get_ctl_caps(&self, ctl_idx : i32) -> PyResult<PyControlCaps>{
        let caps = self.inner.get_ctl_caps_by_idx(ctl_idx).unwrap();
        let a: Vec<u8> = caps.Name.iter().map(|&x| x as u8).collect();
        let b: Vec<u8> = caps.Description.iter().map(|&x| x as u8).collect();
        Ok(PyControlCaps {
            name :String::from_utf8_lossy(&a).to_string(),
            description:String::from_utf8_lossy(&b).to_string(),
            max_value : caps.MaxValue,
            min_value : caps.MinValue,
            default_value : caps.DefaultValue,
            is_auto_supported : caps.IsAutoSupported,
            is_writable : caps.IsWritable,
            control_type: caps.ControlType,
            

        })

    }
    fn get_info(&self) -> PyResult<PyCameraInfo> {
        let info = self.inner.info;
        let a: Vec<u8> = info.FriendlyName.iter().map(|&x| x as u8).collect();
        let b: Vec<u8> = info.CameraSN.iter().map(|&x| x as u8).collect();
        let c: Vec<u8> = info.PortType.iter().map(|&x| x as u8).collect();
        Ok(PyCameraInfo {
            friendly_name: String::from_utf8_lossy(&a).to_string(),
            camera_sn: String::from_utf8_lossy(&b).to_string(),
            port_type: String::from_utf8_lossy(&c).to_string(),
            device_id: info.DeviceID,
            camera_id: info.CameraID,
        })
    }
    fn get_prop(&self) -> PyResult<PyCameraProperty> {
        let prop = self.inner.prop;
        Ok(PyCameraProperty {
            max_height: prop.MaxWidth,
            max_width: prop.MaxHeight,
            is_color_cam: prop.IsColorCam,
            bayer_pattern: prop.BayerPattern,
            supported_bins: prop.SupportedBins,
            supported_video_formats: prop.SupportedVideoFormat,
            max_bit_depth: prop.MaxBitDepth,
            is_trigger_cam: prop.IsTriggerCam,
        })
    }

    fn get_roi_format(&self) -> PyResult<PyROIFormat> {
        let roi = self.inner.get_roi_format().unwrap();
        Ok(PyROIFormat {
            startx: roi.startx,
            starty: roi.starty,
            width: roi.width,
            height: roi.height,
            bin: roi.bin,
        })
    }
    fn set_ctl_value(&self, ctl_type: libsvb::SVB_CONTROL_TYPE, value: i64, is_auto: u32) {
        match self
            .inner
            .set_ctl_value(ctl_type, value, is_auto as libsvb::SVB_BOOL)
        {
            Ok(()) => (),
            Err(e) => panic!("{}", e),
        }
    }
    fn get_ctl_value(&self, ctl_type: libsvb::SVB_CONTROL_TYPE) -> PyResult<Vec<i64>> {
        let state = self.inner.get_ctl_value(ctl_type).unwrap();
        Ok(vec![state.value, state.is_auto as i64])
    }
    fn get_property(&self) {
        let prop = libsvb::_get_camera_prop(self.inner.id);
    }
    fn start_video_capture(&self) {
        self.inner.start_video_capture().unwrap();
    }
    pub fn get_video_frame(&self, ) -> PyResult<Vec<u8>> {
        match self.inner.get_video_frame() {
            Ok(buf) => Ok(buf),
            Err(e) => Err(pyo3::exceptions::PyBufferError::new_err(e.to_string())),
        }
    }

    fn stop_video_capture(&self) {
        self.inner.stop_video_capture().unwrap();
    }
    fn get_img_type(&self) -> PyResult<i32> {
        Ok(self.inner.get_img_type().unwrap())
    }
    fn get_bayer_pattern(&self) -> PyResult<u32> {
        Ok(self.inner.prop.BayerPattern)
    }
    fn set_img_type(&self, img_type: i32) {
        self.inner.set_img_type(img_type).unwrap();
    }
}

#[pyfunction]
fn debayer_buffer(camera: &PyCamera, buffer: Vec<u8>, alg: PyDemosaic) -> PyResult<Vec<u8>> {
    let roi = camera.inner.roi;
    let width = roi.width as u32;
    let height = roi.height as u32;
    let img_type = camera.inner.get_img_type().unwrap();
    let bayer_idx = camera.inner.get_bayer_pattern();

    let bayer_pattern = debayer::cfa_from_u32(bayer_idx);
    let runtime = debayer::Debayer::new(width, height, bayer_pattern);

    let alg = match alg {
        PyDemosaic::None => bayer::Demosaic::None,
        PyDemosaic::Linear => bayer::Demosaic::Linear,
        PyDemosaic::Cubic => bayer::Demosaic::Cubic,
        PyDemosaic::NearestNeighbour => bayer::Demosaic::NearestNeighbour,
    };
    // convert to image by image type (RAW8,RAW16,RGB24,Y8)
    let debayer_buf = match img_type {
        libsvb::SVB_IMG_TYPE_SVB_IMG_RAW8 => {
            runtime.run_from_buf(buffer, debayer::Depth::Depth8, alg)
        }

        libsvb::SVB_IMG_TYPE_SVB_IMG_RAW16 => {
            runtime.run_from_buf(buffer, debayer::Depth::Depth16LE, alg)
        }

        _ => Err(bayer::BayerError::WrongDepth),
    };
    Ok(debayer_buf.unwrap())

}
/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_num_of_camera, m)?)?;
    m.add_function(wrap_pyfunction!(debayer_buffer, m)?)?;
    m.add_class::<PyCamera>()?;
    m.add_class::<PyROIFormat>()?;
    m.add_class::<PyControlType>()?;
    m.add_class::<PyImgType>()?;
    m.add_class::<PyCameraProperty>()?;
    m.add_class::<PyDemosaic>()?;
    m.add_class::<PyCameraInfo>()?;
    m.add_class::<PyControlCaps>()?;
    Ok(())
}
