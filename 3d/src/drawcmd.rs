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
use kiss3d::resource::FramebufferManager;
use std::f32::consts::PI;
use na::{Vec2, Pnt3, PerspMat3, Iso3, Vec3, ToHomogeneous};
use state::{State, DrawState};
use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::mem;
use std::ptr;
//use core::ptr;

pub fn makeit(outfile: String) {
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
    let img = image::ImageBuffer::from_raw(500, 500, my_data).unwrap();
    let mut fout = File::create(outfile).unwrap();
    image::ImageRgba8(img).save(&mut fout, image::PNG).unwrap();
}

pub fn draw(window: &mut Window, infile: String, outfile: String) {
    let mut state = util::load_state(infile);
    let mut camera = ArcBall::new(Pnt3::new(0.0f32, 0.0, -50.0), na::orig());

    window.draw_state(&mut state, 180.0);
    window.render_with_camera(&mut camera);

    for i in 0..10 {
        window.draw_state(&mut state, 180.0);
        window.render_with_camera(&mut camera);
        let Vec2{x: mut width, y: mut height} = window.size();
        width *= 2.0;
        height *= 2.0;
        let mut buf = Vec::new();
        window.snap_rect(&mut buf, 0, 0, width as usize, height as usize);
        let yaw = camera.yaw();
        camera.set_yaw(yaw + 0.004);

        vflip(&mut buf, (width * 3.0) as usize, height as usize);

        let img = image::ImageBuffer::from_raw(width as u32, height as u32, buf).unwrap();
        let mut fout = File::create(format!("{}.{}.png", outfile.clone(), i)).unwrap();
        image::ImageRgb8(img).save(&mut fout, image::PNG).unwrap();
    }
}

fn vflip(vec: &mut [u8], width: usize, height: usize) {
    for j in 0 .. height / 2 {
        for i in 0 .. width {
            vec.swap((height - j - 1) * width + i, j * width + i);
        }
    }
}
