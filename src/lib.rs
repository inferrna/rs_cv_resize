extern crate image;
mod imgproc_c;

pub use imgproc_c::*;
pub use std::ffi::c_void;
pub use std::ffi::CString;
use std::{slice, mem};

pub use image::{DynamicImage, Rgb};

#[macro_export] macro_rules! as_ch_ptr {
    ($a:expr) => {CString::new($a).expect("CString::new failed").as_ptr()} 
}

pub fn resize_image_cv(cvimg: *mut IplImage, neww: i32, newh: i32) -> *mut IplImage {
    let cvsize_new = CvSize{width: neww, height: newh};
    let cvdest = unsafe {cvCreateImage(cvsize_new, (*cvimg).depth, (*cvimg).nChannels)};
    unsafe {cvResize(cvimg as *mut c_void, cvdest as *mut c_void, CV_INTER_LINEAR as i32)};
    return cvdest;
}

pub fn convert_image_to_cv(img: &DynamicImage) -> *mut IplImage {
    let imgrgb = img.to_rgb();
    let (width, height) = imgrgb.dimensions();
    let cvsize = CvSize{width: width as i32, height: height as i32};
    let cvimg = unsafe {cvCreateImageHeader(cvsize, IPL_DEPTH_8U as i32, 3)};
    let mut buf = imgrgb.into_vec().into_boxed_slice();
    let data = buf.as_mut_ptr();
    unsafe { cvSetData(cvimg as *mut c_void, data as *mut c_void, (*cvimg).widthStep) };
    return cvimg;
}

pub fn convert_image_from_cv(cvimg: *mut IplImage) -> DynamicImage {
    let (width, height) = unsafe {((*cvimg).width as u32, (*cvimg).height as u32)};
    let vbytes = unsafe { slice::from_raw_parts((*cvimg).imageDataOrigin, (3*width*height) as usize).to_vec()};
    let vuchar = unsafe {mem::transmute::<Vec<i8>, Vec<u8>>(vbytes)};
    let imgbuf = image::ImageBuffer::from_vec(width, height, vuchar).unwrap();
    return image::ImageRgb8(imgbuf);
}

