### rs_cv_resize
Rust FFI + OpenCV usage example

As for now it's just working. It needs OpenCV 3.2 installed in your system, so run
```
sudo apt install libopencv-core3.2 libopencv-imgcodecs3.2 libopencv-imgproc3.2
```
to intall it. Otherwise be sure you have libopencv_imgcodecs.so.3.2, libopencv_core.so.3.2 and libopencv_imgproc.so.3.2 installed in your system.
1. Create /tmp/sample.png before check it up.
2. `cargo run --bin test` in project directory
3. `/tmp/sample_fromcv.jpg` and `/tmp/resized_sample_fromcv.jpg` must be created.
