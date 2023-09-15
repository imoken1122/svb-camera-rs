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
