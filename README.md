### rs_cv_resize
Rust FFI + OpenCV usage example

As for now it's just working. 
1. Create /tmp/sample.png before check it up.
2. Be sure you have libopencv_imgcodecs.so.3.2, libopencv_core.so.3.2 and libopencv_imgproc.so.3.2 installed in your system. (3.2 is the default OpenCV version in Ubuntu 18.04 repo)
3. `cargo run` in project directory
4. `/tmp/sample_fromcv.jpg` and `/tmp/resized_sample_fromcv.jpg` must be created.
