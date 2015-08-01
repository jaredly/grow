#![allow(dead_code)]
extern crate kiss3d;
extern crate nalgebra as na;
extern crate gl;

use na::{Pnt3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use state::{State, DrawState};

mod state;

impl DrawState for Window {
    fn draw_state(&mut self, state: &mut State) {
        for i in 0..state.num_edges() {
            let (a, b) = state.edge_pts(i);
            //let color = hsl((state.edges[i].age as f32 / 4.0) % 180.0 + 180.0, 1.0, 0.6);
            let color = state.edge_color(i);
            self.draw_line(state.pos(a), state.pos(b), &color);
        }
    }
}

fn main() {
    let mut state = State::init();
    state.start(10);

    let mut window = Window::new("Grow");
    unsafe{
        gl::LineWidth(15.0);
        gl::Enable(gl::LINE_SMOOTH);
        gl::Hint(gl::LINE_SMOOTH_HINT, gl::NICEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut camera = ArcBall::new(Pnt3::new(0.0f32, 0.0, -7.0), na::orig());

    window.set_background_color(1.0, 1.0, 1.0);

    window.set_light(Light::StickToCamera);

    while window.render_with_camera(&mut camera) {
        if state.time < 900 {
            state.tick();
            let dist = camera.dist();
            camera.set_dist(dist + 0.04);
            let yaw = camera.yaw();
            camera.set_yaw(yaw + 0.004);
        }
        window.draw_state(&mut state);
    }
}
