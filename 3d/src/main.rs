#![allow(dead_code)]
#![allow(unused_imports)]
extern crate kiss3d;
extern crate bincode;
extern crate nalgebra as na;
extern crate rustc_serialize;
extern crate gl;
extern crate time;
extern crate image;
extern crate glfw;

mod state;
mod util;
mod glcmd;
mod imgcmd;
mod aaline;
mod drawcmd;
mod shaded;

use kiss3d::window::Window;
use kiss3d::light::Light;
use state::{State};
use na::Pnt3;

extern crate docopt;
use docopt::Docopt;

static USAGE: &'static str = "
3d Growth and Awesomeness

Usage:
  grow show <maxtime> <outfile> [--start=<path>] [--hollow] [--record]
  grow make <maxtime> <outfile> [--start=<path>]
  grow draw <infile> <outfile>
  grow once
  grow info <infile>
  grow display <infile> [--hollow]
  grow (-h | --help)
  grow --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --start=<path>   The file to use as a base
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_maxtime: Option<i32>,
    arg_outfile: Option<String>,
    arg_infile: Option<String>,
    flag_start: Option<String>,
    flag_hollow: bool,
    flag_record: bool,
    cmd_display: bool,
    cmd_info: bool,
    cmd_make: bool,
    cmd_show: bool,
    cmd_once: bool,
    cmd_draw: bool,
}

fn make(max_time: i32, outfile: String, infile: Option<String>) {
    let mut state = util::load_maybe(infile, 10);
    let start = time::get_time();

    for i in state.time..max_time {
        state.tick();
        if i % 50 == 0 {
            util::write_out(&state, outfile.clone() + ".tmp");
            let diff = time::get_time() - start;
            println!("At {} : {}", i, diff);
        }
    }
    println!("Output");
    util::write_out(&state, outfile.clone());
}

fn info(infile: String) {
    let state: State = util::load_state(infile);

    state.print_info();
}

fn just_once() {
    let mut state = State::init();
    state.start(10);
    for _ in 0..100 {
        state.tick();
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    if args.cmd_once {
        just_once();
        return;
    }
    if args.cmd_make {
        make(args.arg_maxtime.unwrap(), args.arg_outfile.unwrap(), args.flag_start);
        return;
    }
    if args.cmd_info {
        println!("Info");
        info(args.arg_infile.unwrap());
        return;
    }

    let mut window = Window::new("Grow");
    unsafe {
        if args.flag_hollow {
            gl::LineWidth(1.0);
            gl::Enable(gl::LINE_SMOOTH);
            gl::Hint(gl::LINE_SMOOTH_HINT, gl::NICEST);
        }
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);
    // window.set_light(Light::Absolute(Pnt3::new(10.0, 1.0, 0.0)));

    if args.cmd_draw {
        drawcmd::draw(&mut window, args.arg_infile.unwrap(), args.arg_outfile.unwrap());
    } else if args.cmd_display {
        glcmd::display(&mut window, args.arg_infile.unwrap(), args.flag_hollow);
    } else {
        glcmd::grow(&mut window, args.arg_maxtime.unwrap(), args.arg_outfile.unwrap(), args.flag_start, args.flag_hollow, args.flag_record);
    }
}
