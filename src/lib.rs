#[macro_use]
extern crate log;
extern crate env_logger;
pub mod camera;
pub mod debayer;
pub mod libsvb;
pub mod utils;
//pub mod capture_video;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub type BufType = Vec<u8>;
pub type BufSize = i64;
pub type ImgType = libsvb::SVB_IMG_TYPE;
pub type CtrlType = libsvb::SVB_CONTROL_TYPE;

#[derive(Debug, Clone, Copy)]
pub enum Endian {
    Big,
    Little,
}
