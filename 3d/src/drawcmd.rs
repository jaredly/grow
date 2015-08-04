extern crate nalgebra as na;
extern crate kiss3d;
extern crate image;
extern crate gl;
extern crate libc;

use state;
use util;
use aaline::DrawLine;

use kiss3d::camera::ArcBall;
use kiss3d::window::Window;
use std::f32::consts::PI;
use na::{Pnt3, PerspMat3, Iso3, Vec3, ToHomogeneous};
use state::{State, DrawState};
use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::mem;
use std::ptr;
//use core::ptr;

pub fn makeit(window: &mut Window, state: &mut State, outfile: String) {
    let num = 4 * 500 * 500 as usize;
    let my_data: Vec<u8> = unsafe {
        let raw: *mut libc::c_void = libc::malloc(num as u64) as *mut libc::c_void;
        gl::ReadPixels(0, 0, 500, 500, gl::RGBA, gl::UNSIGNED_BYTE, raw);
        // data.set_len(500 * 500 * 4);
        let mut dst = Vec::with_capacity(num);
        dst.set_len(num);
        ptr::copy_nonoverlapping(mem::transmute(raw), dst.as_mut_ptr(), num);
        libc::free(raw);
        dst
        //Vec::from_raw_buf(mem::transmute(raw), 500 * 500 * 4)
    };
    let mut img = image::ImageBuffer::from_raw(500, 500, my_data).unwrap();
    let mut fout = File::create(outfile).unwrap();
    image::ImageRgba8(img).save(&mut fout, image::PNG);
}

pub fn draw(window: &mut Window, infile: String, outfile: String) {
    let mut state = util::load_state(infile);
    let mut camera = ArcBall::new(Pnt3::new(0.0f32, 0.0, -50.0), na::orig());

    let mut off = 0.0;
    //let mut data: Vec<u8> = Vec::with_capacity(500 * 500 * 4);
    let mut time = 0;
    while window.render_with_camera(&mut camera) {
        off = (off + 0.1) % 360.0;
        time += 1;
        window.draw_state(&mut state, 180.0);
        if time == 100 {
            println!("Making");
            makeit(window, &mut state, outfile.clone());
        }
        let yaw = camera.yaw();
        camera.set_yaw(yaw + 0.004);
    }
}


