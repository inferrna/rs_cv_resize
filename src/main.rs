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
    let (cols, rows) = imgrgb.dimensions();
    println!("Unsafe create CvMat");
    let cvmat = unsafe {cvCreateMatHeader(rows as i32, cols as i32, CV_8SC3) as *mut c_void};
    println!("Unsafe cast pixels to c_void");
    let mut buf = img.raw_pixels().into_boxed_slice();
    let data = buf.as_mut_ptr();
    let cvbytes: &mut c_void = unsafe {&mut std::mem::transmute::<u8, c_void>(*data)};
    let bytes_len = buf.len() as i32;
    println!("{} bytes", bytes_len);
    println!("Unsafe cvSetData");
    unsafe { cvSetData(cvmat, cvbytes, bytes_len) };

    /*let attrs = CvAttrList{attr: 0 as *mut *const c_char,
                           next: 0 as *mut CvAttrList};
    println!("Unsafe save file");
    unsafe { cvSave(as_ch_ptr!("/tmp/sample_fromcv.jpg"), cvmat, as_ch_ptr!("Sample"), as_ch_ptr!("Created by inferrna"), attrs) };*/
    println!("Unsafe create empty buf");
    let cvdest = unsafe {cvCreateMat(SZ, SZ, CV_8SC3) as *mut c_void};
    println!("Unsafe cvResize. CV_INTER_LINEAR = {}", CV_INTER_LINEAR);
    //Fails below on assertion 'func != 0'
    //According to https://github.com/opencv/opencv/blob/3.2.0/modules/imgproc/src/imgwarp.cpp#L3370 this mustn't happen
    unsafe {cvResize(cvmat, cvdest, CV_INTER_LINEAR as i32)};
    println!("Hello, world!");
}
