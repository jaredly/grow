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

fn hsl(h: f32, s: f32, l: f32) -> Pnt3<f32> {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    if h < 60.0 {
        return Pnt3::new(c + m, x + m, 0.0 + m);
    }
    if h < 120.0 {
        return Pnt3::new(x + m, c + m, 0.0 + m);
    }
    if h < 180.0 {
        return Pnt3::new(0.0 + m, c + m, x + m);
    }
    if h < 240.0 {
        return Pnt3::new(0.0 + m, x + m, c + m);
    }
    if h < 300.0 {
        return Pnt3::new(x + m, 0.0 + m, c + m);
    }
    Pnt3::new(c, 0.0, x)
}

fn main() {
    let mut state = State::init();
    state.start(25);

    let mut window = Window::new("Grow");
    unsafe{
        gl::LineWidth(15.0);
        gl::Enable(gl::LINE_SMOOTH);
        gl::Hint(gl::LINE_SMOOTH_HINT, gl::NICEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut camera = ArcBall::new(Pnt3::new(0.0f32, 0.0, -3.0), na::orig());

    window.set_background_color(1.0, 1.0, 1.0);

    window.set_light(Light::StickToCamera);

    while window.render_with_camera(&mut camera) {
        state.tick();
        window.draw_state(&mut state);
        let yaw = camera.yaw();
        camera.set_yaw(yaw + 0.004);
    }
}
