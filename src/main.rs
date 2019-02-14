extern crate image;
mod imgproc_c;

use imgproc_c::*;
use std::ffi::c_void;
use std::os::raw::c_char;
use std::ffi::CString;

use image::DynamicImage;
const SZ: i32 = 100;

macro_rules! as_ch_ptr {
    ($a:expr) => {CString::new($a).expect("CString::new failed").as_ptr()} 
}

fn main() {
    let img: DynamicImage = image::open("/tmp/sample.png").unwrap();
    let imgrgb = img.to_rgb();
    let (width, height) = imgrgb.dimensions();
    println!("Unsafe create ImageHeader with width={} and height={}", width, height);
    let cvsize = CvSize{width: width as i32, height: height as i32};
    let cvimg = unsafe {cvCreateImageHeader(cvsize, IPL_DEPTH_8U as i32, 3)};
    println!("Unsafe cast pixels to c_void");
    /*let mut buf = img.raw_pixels().into_boxed_slice();
    let data = buf.as_mut_ptr();*/
    let mut buf = imgrgb.into_vec().into_boxed_slice();
    let data = buf.as_mut_ptr();
    let cvbytes: &mut c_void = unsafe {&mut std::mem::transmute::<u8, c_void>(*data)};
    let bytes_len = buf.len() as i32;
    println!("{} bytes", bytes_len);
    println!("Unsafe cvSetData");
    unsafe { cvSetData(cvimg as *mut c_void, data as *mut c_void, (*cvimg).widthStep) };
    
    println!("Unsafe save original file");
    unsafe { cvSaveImage(as_ch_ptr!("/tmp/sample_fromcv.jpg"), cvimg as *mut c_void, &0i32) };
    
    let cvsize_new = CvSize{width: SZ, height: SZ};
    println!("Unsafe create empty buf");
    let cvdest = unsafe {cvCreateImage(cvsize_new, IPL_DEPTH_8U as i32, 3)};
    println!("Unsafe cvResize. CV_INTER_LINEAR = {}", CV_INTER_LINEAR);
    unsafe {cvResize(cvimg as *mut c_void, cvdest as *mut c_void, CV_INTER_LINEAR as i32)};
    
    println!("Unsafe save resized file");
    unsafe { cvSaveImage(as_ch_ptr!("/tmp/resized_sample_fromcv.jpg"), cvdest as *mut c_void, &0i32) };

    println!("Hello, world!");
}
