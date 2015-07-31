#![allow(dead_code)]
extern crate kiss3d;
extern crate nalgebra as na;
extern crate gl;

use std::f32;
use na::{Vec3, Pnt3, FloatPnt, Norm};
use kiss3d::window::Window;
use kiss3d::light::Light;

const TOLERANCE: f32 = 0.001;
const DAMP: f32 = 0.85;
const STICK_K: f32 = 0.09;
const AVOID_K: f32 = 0.01;

const MAX_LEN: f32 = 0.1;
const TOO_CROWDED: i32 = 25; // neighbors
const MIN_CROWD: i32 = 5;
const TOO_DEAD: i32 = 20;
const DEAD_MOTION: f32 = 0.0001;
const CLOSE_DIST: f32 = 0.35;
const PUSH_DIST: f32 = 0.2;
const GROW_SPEED: f32 = 0.001;
const MAX_SPEED: f32 = 0.004;


//let SHOW_POINTS = false;
//let COLOR_SCHEME = 'age';
//const RANDOM = false;

#[derive(Copy, Clone)]
struct Edge {
    a: i32,
    b: i32,
    len: f32,
    curlen: f32,
    age: i32,
}

struct Node {
    pos: Pnt3<f32>,
    vel: Vec3<f32>,
    nclose: i32,
    dead: i32,
}

struct State {
    time: i32,
    pts: Vec<Node>,// = [Pnt3{x: 0.0, y: 0.0, z:0.0}; 1000];
    edges: Vec<Edge>,// = [Edge{a: 0, b: 0}; 1000];
    // num_pts: i32,// = 0;
    // num_edges: i32,// = 0;
}

fn angle_between(p1: &Pnt3<f32>, p2: &Pnt3<f32>) -> f32 {
    let c = na::cross(p1.as_vec(), p2.as_vec());
    na::norm(&c).atan2(na::dot(p1.as_vec(), p2.as_vec()))
}

impl State {
    fn init() -> State {
        State{
            time: 0,
            pts: vec![],
            edges: vec![],
            // num_pts: 0,
            // num_edges: 0,
        }
    }

    fn start(&mut self, num: i32) {
        // self.num_pts = num;

        let fnum = num as f32;
        let scale = 2.0 * f32::consts::PI / fnum;
        let circumference = fnum * MAX_LEN * 0.2;
        let rad = circumference / 2.0 / f32::consts::PI;
        for i in 0..num {
            self.pts.push(Node {
                pos: Pnt3{
                    x: (i as f32 * scale).cos() * rad,
                    y: (i as f32 * scale).sin() * rad,
                    z: 0.0,
                },
                vel: Vec3::new(0.0, 0.0, 0.0),
                nclose: 0,
                dead: 0,
            });
        }

        for i in 0..num {
            self.edges.push(Edge{
                a: i,
                b: (i + 1) % num,
                len: MAX_LEN / 2.0,
                curlen: self.pts[i as usize].pos.dist(&self.pts[((i + 1) % num) as usize].pos),
                age: 0,
            });
        }
    }

    fn draw(&self, window: &mut Window) {
        let color = Pnt3::new(0.5, 0.1, 1.0);
        for i in 0..self.edges.len() {
            let Edge{a, b, ..} = self.edges[i];
            window.draw_line(&self.pts[a as usize].pos, &self.pts[b as usize].pos, &color);
        }
    }

    fn adjust(&mut self) {
        for i in 0..self.edges.len() {
            let Edge{a, b, len, ..} = self.edges[i];
            let p1 = self.pts[a as usize].pos;
            let p2 = self.pts[b as usize].pos;
            let mag = p1.dist(&p2);
            self.edges[i].curlen = mag;
            let diff = (p2 - p1).normalize();
            self.pts[a as usize].vel = self.pts[a as usize].vel + diff * (len - mag) / 2.0 *
                -STICK_K;
            self.pts[b as usize].vel = self.pts[b as usize].vel - diff * (len - mag) / 2.0 *
                -STICK_K;

            //let theta = na::angle_between(&p1, &p2); //p1.angle_to(&p2);
            //let df = self.pts[a as usize].dist(self.pts[b as usize]);
            //let an = self.pts[a as usize].
        }
    }

    fn edge_grow(&mut self) {
        if (self.time / 300) % 2 == 0 {
            for i in 0..self.edges.len() {
                self.edges[i].len += GROW_SPEED;
            }
        } else {
            for i in 0..self.edges.len() {
                if (self.edges[i].len > GROW_SPEED * 2.0) {
                    self.edges[i].len -= GROW_SPEED / 2.0;
                }
            }
        }
    }

    fn edge_split(&mut self) {
        let len = self.edges.len();
        for i in 0..len {
            if self.edges[i].len < MAX_LEN || self.edges[i].curlen < MAX_LEN {
                continue;
            }
            let Edge{a, b, len, ..} = self.edges[i];
            let npt = self.pts.len();
            let npos = self.pts[a as usize].pos + (self.pts[b as usize].pos - self.pts[a as usize].pos) / 2.0;
            self.pts.push(Node{
                pos: npos,
                vel: Vec3::new(0.0, 0.0, 0.0),
                nclose: 0,
                dead: 0,
            });
            let ob = self.edges[i].b;
            self.edges.push(Edge{
                len: len / 2.0,
                curlen: 0.0,
                age: 0,
                a: npt as i32,
                b: ob,
            });
            self.edges[i].len = len / 2.0;
            self.edges[i].b = npt as i32;
        }
    }

    fn tick(&mut self) {
        self.time += 1;
        self.adjust();
        //self.pushAway();
        self.edge_grow();
        self.edge_split();
        self.move_things();
    }

    fn move_things(&mut self) {
        for i in 0..self.pts.len() {
            self.pts[i].vel = self.pts[i].vel * DAMP;
            self.pts[i].pos = self.pts[i].pos + self.pts[i].vel;
        }
    }
}

fn main() {
    let mut state = State::init();
    state.start(10);

    let mut window = Window::new("Grow");
    unsafe{
        gl::LineWidth(1.0);
        gl::Enable(gl::LINE_SMOOTH);
        gl::Hint(gl::LINE_SMOOTH_HINT, gl::NICEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    window.set_background_color(1.0, 1.0, 1.0);

    window.set_light(Light::StickToCamera);

    while window.render() {
        state.tick();
        state.draw(&mut window);
    }
}
