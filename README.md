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


