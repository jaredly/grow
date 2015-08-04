extern crate nalgebra as na;
extern crate kiss3d;
extern crate time;
use util;

use na::{Pnt3};
use state::{State, DrawState};
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;

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

pub fn grow(window: &mut Window, max_time: i32, outfile: String, infile: Option<String>) {
    let mut state = util::load_maybe(infile, 10);

    let mut camera = ArcBall::new(Pnt3::new(0.0f32, 0.0, -7.0), na::orig());
    let start = time::get_time();

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
            util::write_out(&state, outfile.clone());
            state.time += 1;
        } else if state.time % 50 == 0 {
            util::write_out(&state, outfile.clone() + ".tmp");
            let diff = time::get_time() - start;
            println!("At {} : {}", state.time, diff);
        }
        window.draw_state(&mut state, 180.0);
    }
}

pub fn display(window: &mut Window, infile: String) {
    let mut state = util::load_state(infile);
    let mut camera = ArcBall::new(Pnt3::new(0.0f32, 0.0, -50.0), na::orig());

    let mut off = 0.0;
    while window.render_with_camera(&mut camera) {
        off = (off + 0.1) % 360.0;
        window.draw_state(&mut state, 180.0);
        let yaw = camera.yaw();
        camera.set_yaw(yaw + 0.004);
    }
}

