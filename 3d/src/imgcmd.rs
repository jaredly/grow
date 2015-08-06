extern crate nalgebra as na;
extern crate kiss3d;
extern crate image;

use util;
use aaline::DrawLine;

use std::f32::consts::PI;
use na::{Pnt3, PerspMat3, Iso3, Vec3, ToHomogeneous};
use state::{State, DrawState};
use image::{ImageBuffer, Rgba};
use std::fs::File;

impl DrawState for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn draw_state(&mut self, state: &mut State, off: f32) {
        let fov = PI / 4.0;
        let znear = 0.1;
        let zfar = 1024.0;
        let eye = Pnt3::new(0.0f32, 0.0, -50.0);
        let at: Pnt3<f32> = na::orig();
        let projection = PerspMat3::new(800.0 / 600.0, fov, znear, zfar);

        // let dist = na::norm(&(eye - at));
        // let pitch = ((eye.y - at.y) / dist).acos();
        // let yaw = (eye.z - at.z).atan2(eye.x - at.x);

        // let px = at.x + dist * yaw.cos() * pitch.sin();
        // let py = at.y + dist * pitch.cos();
        // let pz = at.z + dist * yaw.sin() * pitch.sin();

        //let neye = Pnt3::new(px, py, pz);

        let mut view_transform: Iso3<f32> = na::one();
        // TODO do I need to call the eye function?
        view_transform.look_at_z(&eye, &at, &Vec3::y());

        let proj_view = *projection.as_mat() * na::to_homogeneous(&na::inv(&view_transform).unwrap());
        //let inv_proj_view = na::inv(&proj_view).unwrap();

        for i in 0..state.num_edges() {
            let (a, b) = state.edge_pts(i);
            //let color = hsl((state.edges[i].age as f32 / 4.0) % 180.0 + 180.0, 1.0, 0.6);
            let color = state.edge_color(i, off);

            let p1 = proj_view * state.pos(a).to_homogeneous();
            let p2 = proj_view * state.pos(b).to_homogeneous();
            self.draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, &color);
            // self.draw_line(state.pos(a), state.pos(b), &color);
        }
    }
}

pub fn draw(infile: String, outfile: String) {
    println!("Ready");
    let mut state = util::load_state(infile);
    println!("Loaded state");
    let mut img = image::ImageBuffer::new(200, 200);
    img.draw_state(&mut state, 0.0);
    /*
    for x in 0..100 {
        img.put_pixel(100 - x, x, image::Rgba([255, 0, 0, 255]));
    }
    img.draw_line(10.0, 10.0, 120.0, 730.0, 2.0, &(255, 0, 255));
    img.draw_line(10.0, 10.0, 120.0, 130.0, 2.0, &(255, 0, 255));
    img.draw_line(10.0, 10.0, 120.0, 20.0, 2.0, &(255, 0, 255));
    img.draw_line(10.0, 10.0, 140.0, 120.0, 2.0, &(0, 0, 255));
    */
    let mut fout = File::create(outfile).unwrap();
    image::ImageRgba8(img).save(&mut fout, image::PNG).unwrap();
}

