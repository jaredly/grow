extern crate nalgebra as na;
extern crate kiss3d;
extern crate time;
extern crate glfw;
extern crate image;
use image::{ImageBuffer, Rgba};
use std::fs::File;

use shaded;
use util;

use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use std::rc::Rc;
use std::cell::RefCell;
use na::{Pnt3, Vec2};
use state::{State, DrawState};
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use kiss3d::resource::{Shader, ShaderAttribute, ShaderUniform, Material, Mesh, FramebufferManager};
use kiss3d::builtin::UvsMaterial;
use glfw::{Action, Key, WindowEvent};
use std::thread;
use std::path::Path;

impl DrawState for Window {
    fn draw_state(&mut self, state: &mut State, off: f32) {
        for i in 0..state.num_edges() {
            let (a, b) = state.edge_pts(i);
            //let color = hsl((state.edges[i].age as f32 / 4.0) % 180.0 + 180.0, 1.0, 0.6);
            let color = state.edge_color(i, off);
            self.draw_line(state.pos(a), state.pos(b), &color);
        }
    }
}

fn shoot(window: &mut Window, outfile: String) {
    let Vec2{x: mut width, y: mut height} = window.size();
    width *= 2.0;
    height *= 2.0;
    let mut buf = Vec::new();
    window.snap_rect(&mut buf, 0, 0, width as usize, height as usize);

    thread::spawn(move || {
        vflip(&mut buf, (width * 3.0) as usize, height as usize);
        let img = image::ImageBuffer::from_raw(width as u32, height as u32, buf).expect("Opening image for writing");
        let mut fout = File::create(outfile.clone()).ok().expect("Open file");
        image::ImageRgb8(img).save(&mut fout, image::PNG).ok().expect("Saving image");
        println!("Wrote {}", outfile);
    });
}

fn shoot_at(window: &mut Window, outfile: String, sender: Sender<(String, Box<Vec<u8>>, usize, usize)>) {
    let Vec2{x: mut width, y: mut height} = window.size();
    width *= 2.0;
    height *= 2.0;
    let mut buf = Vec::new();
    window.snap_rect(&mut buf, 0, 0, width as usize, height as usize);
    sender.send((outfile, Box::new(buf), width as usize, height as usize)).ok().expect("Sending to channel");
}

fn vflip(vec: &mut [u8], width: usize, height: usize) {
    for j in 0 .. height / 2 {
        for i in 0 .. width {
            vec.swap((height - j - 1) * width + i, j * width + i);
        }
    }
}

pub fn grow(window: &mut Window, max_time: i32, outfile: String, infile: Option<String>, hollow: bool, record: bool) {
    let mut state = util::load_maybe(infile.clone(), 10);
    let mut camera = ArcBall::new(Pnt3::new(0.0f32, 0.0, -7.0), Pnt3::new(0.0, 1.5, 0.0));
    let start = time::get_time();

    let (sender, receiver): (Sender<(String, Box<Vec<u8>>, usize, usize)>, Receiver<_>) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let (outfile, mut buf, width, height) = match receiver.recv() {
                Ok(x) => x,
                Err(_) => {return},
            };
            vflip(&mut *buf, width * 3, height);
            let img = image::ImageBuffer::from_raw(width as u32, height as u32, *buf).expect("Create image");
            let mut fout = File::create(outfile.clone()).ok().expect("Open file");
            image::ImageRgb8(img).save(&mut fout, image::PNG).ok().expect("Save image");
            println!("Wrote {}", outfile);
        }
    });

    if state.time == 0 {
        while state.tris.len() == 0 {
            state.tick();
        }
    }
    let vertices = state.coords();
    let indices = state.tris.clone();
    let texture_idx = state.coord_colors(0.0);
    let mesh  = Rc::new(RefCell::new(Mesh::new(vertices, indices, None, Some(texture_idx), false)));
    let material   = Rc::new(RefCell::new(Box::new(shaded::ShaderMaterial::default()) as Box<Material + 'static>));
    let mut obj = window.add_mesh(mesh, na::one());
    obj.set_color(0.0, 1.0, 0.0);
    obj.enable_backface_culling(false);
    // obj.set_surface_rendering_activation(false);
    obj.set_lines_width(15.0);
    obj.set_material(material);

    let mut running = true;
    let mut recording = record;
    while window.render_with_camera(&mut camera) {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(code, _, Action::Press, _) => {
                    match code {
                        Key::X => {
                            state = util::load_maybe(infile.clone(), 10);
                        },
                        Key::R => {
                            recording = !recording;
                        },
                        Key::S => {
                            shoot_at(window, format!("gen/{}-{:04}.png", outfile.clone(), state.time), sender.clone());
                        },
                        Key::P => {
                            running = !running;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        if running && state.time < max_time {
            if recording {
                shoot_at(window, format!("gen/{}-{:04}.png", outfile.clone(), state.time), sender.clone());
            }

            state.tick();

            // update stuff
            let vertices = state.coords();
            let indices = state.tris.clone();
            let texture_idx = state.coord_colors(0.0);
            obj.modify_vertices(&mut move |current| {
                for i in 0..current.len() {
                    current[i] = vertices[i];
                }
                let _: Vec<usize> = vertices[current.len()..].iter().map(|i| {current.push(*i); 0usize}).collect();
            });
            obj.modify_faces(&mut move |current| {
                let _: Vec<usize> = indices[current.len()..].iter().map(|i| {current.push(*i); 0usize}).collect();
            });
            obj.modify_uvs(&mut move |current| {
                for i in 0..current.len() {
                    current[i] = texture_idx[i];
                }
                let _: Vec<usize> = texture_idx[current.len()..].iter().map(|i| {current.push(*i); 0usize}).collect();
            });
            //obj.modify_faces(&move |_| indices);
            //obj.modify_uvs(&move |_| texture_idx);

            // move camera
            let dist = camera.dist();
            camera.set_dist(dist + 0.03);
            let yaw = camera.yaw();
            camera.set_yaw(yaw + 0.002);
            let at = camera.at_mut();
            at.y += 0.010;
        }
        if state.time == max_time {
            println!("Output");
            util::write_out(&state, outfile.clone());
            state.time += 1;
        } else if state.time % 50 == 0 {
            util::write_out(&state, outfile.clone() + ".tmp");
            let diff = time::get_time() - start;
            println!("At {} : {}", state.time, diff);
        }
        // window.draw_state(&mut state, 180.0);
    }
}

pub fn display(window: &mut Window, infile: String, hollow: bool) {
    let mut state = util::load_state(infile);
    let mut camera = ArcBall::new(Pnt3::new(0.0f32, 20.0, -50.0), na::orig());

    let vertices = state.coords();
    let indices = state.tris.clone();
    let texture_idx = state.coord_colors(0.0);
    let mesh  = Rc::new(RefCell::new(Mesh::new(vertices, indices, None, Some(texture_idx), false)));
    let material   = Rc::new(RefCell::new(Box::new(shaded::ShaderMaterial::default()) as Box<Material + 'static>));
    if !hollow {
        let mut obj = window.add_mesh(mesh, na::one());
        obj.set_color(0.0, 1.0, 0.0);
        obj.enable_backface_culling(false);

        // obj.set_surface_rendering_activation(false);
        obj.set_lines_width(15.0);

        //obj.set_texture_from_file(&Path::new("media/kitten.png"), "kitten");
        obj.set_material(material);
    }

    let mut off = 0.0;
    while window.render_with_camera(&mut camera) {
        off = (off + 0.1) % 360.0;
        // material.inc_time();
        if hollow {
            window.draw_state(&mut state, 180.0);
        }
        let yaw = camera.yaw();
        camera.set_yaw(yaw + 0.004);
    }
}

