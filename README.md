<div align=center>
    <h1>svb-camera-rs</h1>
</div>

 This repository is a simplified Rust binding of SVBONY Camera driver.
 
The SDK is distributed [here]().

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

    // get num of connected camera
    let num = get_num_of_connected_cameras();

    // creta camera object
    let mut camera = Camera::new(0);

    // initialize, and camera open
    camera.init();



}


```
## Python 
