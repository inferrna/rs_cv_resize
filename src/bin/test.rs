#[macro_use] extern crate cvtry;
use cvtry::*;

const SZ: i32 = 100;

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

