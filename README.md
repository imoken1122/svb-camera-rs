<div align=center>
    <h1>svb-camera-rs</h1>
</div>

 This repository is a simplified Rust binding of SVBONY Camera driver.
 
The SDK is distributed [here](https://www.svbony.jp/downloads).

Tested Deviced
- Camera
  - SV405CC
- OS
  - M2 Mac ( target arm64) 
  
## Run

First connect the SVBONY Camera using USB

```zsh
 RUST_LOG=debug cargo run -r --example capture_frame
```
## Use Rust

1. The number of camera connections must always be obtained first

2. camera.init() is used to retrieve information about the various cameras and open the camera. (not exposure)

3. The next step is to set the parameters before acquiring the video frames, such as setting the ControlType(Exposure,Gain,..etc) value, ROI(width,height..etc), ImageType(RAW8,RAW16), etc.

4. A separate thread initiates the capture and retrieves the video frames, where it is optional to create a buffer and pass a pointer to it, or to receive a buffer with the frame data stored in it.x

```rust

fn main() {

    // 1. get num of connected camera
    let num = get_num_of_connected_cameras();

    // create camera object
    let mut camera = Camera::new(0);

    //2. initialize, and camera open
    camera.init();

    //3. set image type ,
    camera.set_img_type(libsvb::SVB_IMG_TYPE_SVB_IMG_RAW8);

    // set ROI (startx,starty,width,height,bin)
    camera.set_roi_format(0, 0, 1980, 1080, 1);
    let roi = camera.get_roi_format().unwrap();

    // set control type value
    let exp_type = libsvb::SVB_CONTROL_TYPE_SVB_EXPOSURE;
    camera.set_ctl_value(exp_type, 5000000, libsvb::SVB_BOOL_SVB_FALSE);

    n=0
    //4. create thread and start capture
    thread::spawn(move || {
        camera.start_video_capture();
        while n < 2 {
            let buf : Vec<u8> = camera.get_video_frame().unwrap();
          
            let img = camera.buf_to_img(buf,Demosaic::Linear).unwrap();

            camera.save_img(img, "jpg");
            n += 1;
        }
        camera.stop_video_capture();
        camera.close();
    });

}


```
