extern crate image;
mod imgproc_c;

use imgproc_c::*;
use std::ffi::c_void;
use std::ffi::CString;
use std::{slice, mem};

use image::{DynamicImage, Rgb};
const SZ: i32 = 100;

macro_rules! as_ch_ptr {
    ($a:expr) => {CString::new($a).expect("CString::new failed").as_ptr()} 
}

fn resize_image_with_cv(img: DynamicImage){

}

fn resize_image_cv(cvimg: *mut IplImage, neww: i32, newh: i32) -> *mut IplImage {
    let cvsize_new = CvSize{width: neww, height: newh};
    let cvdest = unsafe {cvCreateImage(cvsize_new, (*cvimg).depth, (*cvimg).nChannels)};
    unsafe {cvResize(cvimg as *mut c_void, cvdest as *mut c_void, CV_INTER_LINEAR as i32)};
    return cvdest;
}

fn convert_image_to_cv(img: &DynamicImage) -> *mut IplImage {
    let imgrgb = img.to_rgb();
    let (width, height) = imgrgb.dimensions();
    let cvsize = CvSize{width: width as i32, height: height as i32};
    let cvimg = unsafe {cvCreateImageHeader(cvsize, IPL_DEPTH_8U as i32, 3)};
    let mut buf = imgrgb.into_vec().into_boxed_slice();
    let data = buf.as_mut_ptr();
    unsafe { cvSetData(cvimg as *mut c_void, data as *mut c_void, (*cvimg).widthStep) };
    return cvimg;
}

fn convert_image_from_cv(cvimg: *mut IplImage) -> DynamicImage {
    let (width, height) = unsafe {((*cvimg).width as u32, (*cvimg).height as u32)};
    let vbytes = unsafe { slice::from_raw_parts((*cvimg).imageDataOrigin, (3*width*height) as usize).to_vec()};
    let vuchar = unsafe {mem::transmute::<Vec<i8>, Vec<u8>>(vbytes)};
    let imgbuf = image::ImageBuffer::from_vec(width, height, vuchar).unwrap();
    return image::ImageRgb8(imgbuf);
}

fn main() {
    let img: DynamicImage = image::open("/tmp/sample.png").unwrap();
    println!("Unsafe create cv image");
    let cvimg = convert_image_to_cv(&img);
    
    /*println!("Unsafe save original file");
    unsafe { cvSaveImage(as_ch_ptr!("/tmp/sample_fromcv.jpg"), cvimg as *mut c_void, &0i32) };*/
    
    println!("Unsafe resize image");
    let cvdest = resize_image_cv(cvimg, SZ, SZ);
    
    println!("Unsafe save resized image");
    unsafe { cvSaveImage(as_ch_ptr!("/tmp/resized_sample_fromcv.jpg"), cvdest as *mut c_void, &0i32) };
    
    println!("Unsafe convert resized image back");
    let imgdest = convert_image_from_cv(cvdest);
    imgdest.save("/tmp/resized_sample_fromimg.jpg").unwrap();

    println!("Hello, world!");
}
