extern crate ffmpeg_sys;
use std::ffi::CString;
use std::ptr;

use ffmpeg_sys::AVFormatContext;

fn main() {
    let image_fp = CString::new("./sample.jpg").unwrap();
    let audio_fp = CString::new("./sample.mp3").unwrap();
    println!("Starting AV Codecs");
    unsafe {
        ffmpeg_sys::av_register_all();
    }
    let mut image_format_ctx: *mut AVFormatContext = ptr::null_mut();
    unsafe {
        let open_img_ok =  ffmpeg_sys::avformat_open_input(
            &mut image_format_ctx,
            image_fp.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut());
        println!("Open Image Returned {}", open_img_ok);
    }

    let mut audio_format_ctx: *mut AVFormatContext = ptr::null_mut();
    unsafe {
        let audio_img_ok =  ffmpeg_sys::avformat_open_input(
            &mut audio_format_ctx,
            audio_fp.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut());
        println!("Open Audio Returned {}", audio_img_ok);
    }
}
