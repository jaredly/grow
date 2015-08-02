#![allow(dead_code)]
extern crate kiss3d;
extern crate bincode;
extern crate nalgebra as na;
extern crate rustc_serialize;
extern crate gl;
extern crate time;

use na::{Pnt3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use state::{State, DrawState};
use bincode::SizeLimit;
use std::io::prelude::*;
use std::fs::File;

mod state;

extern crate docopt;

use docopt::Docopt;

static USAGE: &'static str = "
3d Growth and Awesomeness

Usage:
  grow show <maxtime> <outfile>
  grow make <maxtime> <outfile>
  grow display <infile>
  grow (-h | --help)
  grow --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_maxtime: Option<i32>,
    arg_outfile: Option<String>,
    arg_infile: Option<String>,
    cmd_display: bool,
    cmd_make: bool,
    cmd_show: bool,
}

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

fn write_out(state: &State, outfile: String) {
    let mut out = File::create(outfile).unwrap();
    let result = bincode::encode_into(&state, &mut out, SizeLimit::Infinite);
}

fn grow(window: &mut Window, max_time: i32, outfile: String) {
    let mut state = State::init();
    state.start(10);

    let mut camera = ArcBall::new(Pnt3::new(0.0f32, 0.0, -7.0), na::orig());

    while window.render_with_camera(&mut camera) {
        if state.time < max_time {
            state.tick();
            let dist = camera.dist();
            camera.set_dist(dist + 0.04);
            let yaw = camera.yaw();
            camera.set_yaw(yaw + 0.004);
        }
        if state.time == max_time {
            println!("Output");
            write_out(&state, outfile.clone());
            state.time += 1;
        }
        window.draw_state(&mut state, 180.0);
    }
}

fn display(window: &mut Window, infile: String) {
    let mut file = File::open(infile).unwrap();
    let mut state = bincode::decode_from(&mut file, SizeLimit::Infinite).unwrap();

    let mut camera = ArcBall::new(Pnt3::new(0.0f32, 0.0, -7.0), na::orig());

    //let mut off = 0.0;
    while window.render_with_camera(&mut camera) {
        //off = (off + 0.1) % 360.0;
        window.draw_state(&mut state, 180.0);
    }
}

fn make(max_time: i32, outfile: String) {
    let mut state = State::init();
    state.start(10);
    let start = time::get_time();

    for i in 0..max_time {
        state.tick();
        if i % 50 == 0 {
            write_out(&state, outfile.clone() + ".tmp");
            let diff = time::get_time() - start;
            println!("At {} : {}", i, diff);
        }
    }
    println!("Output");
    write_out(&state, outfile.clone());
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    if args.cmd_make {
        make(args.arg_maxtime.unwrap(), args.arg_outfile.unwrap());
        return;
    }

    let mut window = Window::new("Grow");
    unsafe{
        gl::LineWidth(15.0);
        gl::Enable(gl::LINE_SMOOTH);
        gl::Hint(gl::LINE_SMOOTH_HINT, gl::NICEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    window.set_background_color(1.0, 1.0, 1.0);

    window.set_light(Light::StickToCamera);

    if args.cmd_display {
        display(&mut window, args.arg_infile.unwrap());
    } else {
        grow(&mut window, args.arg_maxtime.unwrap(), args.arg_outfile.unwrap());
    }

}
