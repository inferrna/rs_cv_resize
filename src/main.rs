extern crate image;
mod imgproc_c;
use imgproc_c::*;
use std::ffi::c_void;

use image::DynamicImage;
const SZ: i32 = 100;

fn main() {
    let img: DynamicImage = image::open("/tmp/sample.png").unwrap();
    let imgrgb = img.to_rgb();
    let (cols, rows) = imgrgb.dimensions();
    println!("Unsafe create CvMat");
    let cvmat = unsafe {cvCreateMatHeader(rows as i32, cols as i32, CV_8SC3) as *mut c_void};
    println!("Unsafe cast pixels to c_void");
    let cvbytes: &mut c_void = unsafe {std::mem::transmute::<&mut Vec<u8>, &mut c_void>(&mut img.raw_pixels())};
    let bytes_len = (cols*rows*3) as i32;
    println!("{} bytes", bytes_len);
    println!("Unsafe cvSetData");
    unsafe { cvSetData(cvmat, cvbytes, bytes_len) };
    println!("Unsafe create empty buf");
    let cvdest = unsafe {cvCreateMat(SZ, SZ, CV_8SC3) as *mut c_void};
    println!("Unsafe cvResize. CV_INTER_LINEAR = {}", CV_INTER_LINEAR);
    //Fails below on assertion 'func != 0'
    //According to https://github.com/opencv/opencv/blob/3.2.0/modules/imgproc/src/imgwarp.cpp#L3370 this mustn't happen
    unsafe {cvResize(cvmat, cvdest, CV_INTER_LINEAR as i32)};
    println!("Hello, world!");
}
