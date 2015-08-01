#![allow(dead_code)]
extern crate kiss3d;
extern crate nalgebra as na;
extern crate gl;

use std::f32;
use na::{Vec3, Pnt3, FloatPnt, Norm};
use kiss3d::window::Window;
use kiss3d::light::Light;


const TOLERANCE: f32 = 0.001;
const DAMP: f32 = 0.15;
const STICK_K: f32 = 0.09;
const AVOID_K: f32 = 0.02;

const MAX_LEN: f32 = 0.005;
const TOO_CROWDED: usize = 25; // neighbors
const MIN_CROWD: i32 = 5;
const TOO_DEAD: i32 = 100;
const DEAD_MOTION: f32 = 0.0001;
const CLOSE_DIST: f32 = 4.0;
const PUSH_DIST: f32 = 2.0;
const GROW_SPEED: f32 = 0.00005;
const MAX_SPEED: f32  = 0.0001;


//let SHOW_POINTS = false;
//let COLOR_SCHEME = 'age';
//const RANDOM = false;

#[derive(Copy, Clone)]
struct Edge {
    a: usize,
    b: usize,
    len: f32,
    curlen: f32,
    age: usize,
}

struct Node {
    pos: Pnt3<f32>,
    vel: Vec3<f32>,
    nclose: usize,
    dead: i32,
    left: usize,
    right: usize,
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

    fn start(&mut self, num: usize) {
        // self.num_pts = num;

        let fnum = num as f32;
        let scale = 2.0 * f32::consts::PI / fnum;
        let circumference = fnum * MAX_LEN * 0.2;
        let rad = circumference / 2.0 / f32::consts::PI;
        for i in 0..num {
            let mrad = rad + (i as f32 / 20.0).sin();
            self.pts.push(Node {
                pos: Pnt3{
                    x: (i as f32 * scale).cos() * mrad,
                    y: (i as f32 * scale).sin() * mrad,
                    z: mrad - rad, // 0.0,
                },
                vel: Vec3::new(0.0, 0.0, 0.0),
                nclose: 0,
                dead: 0,
                left: if i == 0 {num - 1} else {i - 1},
                right: (i+1) % num,
            });
        }

        for i in 0..num {
            self.edges.push(Edge{
                a: i,
                b: (i + 1) % num,
                len: MAX_LEN / 2.0,
                curlen: self.pts[i].pos.dist(&self.pts[((i + 1) % num)].pos),
                age: 0,
            });
        }
    }

    fn draw(&mut self, window: &mut Window) {
        //let color = Pnt3::new(0.5, 0.1, 1.0);
        for i in 0..self.edges.len() {
            self.edges[i].age += 1;
            let Edge{a, b, ..} = self.edges[i];
            let color = hsl((self.edges[i].age as f32 / 4.0) % 180.0 + 180.0, 1.0, 0.6);
            window.draw_line(&self.pts[a].pos, &self.pts[b].pos, &color);
        }
    }

    fn adjust(&mut self) {
        for i in 0..self.edges.len() {
            let Edge{a, b, len, ..} = self.edges[i];
            let p1 = self.pts[a].pos;
            let p2 = self.pts[b].pos;
            let mag = p1.dist(&p2);
            self.edges[i].curlen = mag;
            let diff = (p2 - p1).normalize();
            let mdiff = diff * (len - mag) / 2.0 * -STICK_K;
            self.pts[a].vel = self.pts[a].vel + mdiff;
            self.pts[b].vel = self.pts[b].vel - mdiff;
        }
    }

    fn edge_grow(&mut self) {
        for i in 0..self.edges.len() {
            let Edge{a, b, len, ..} = self.edges[i];
            if len > MAX_LEN {
                continue;
            }
            if self.pts[a].nclose > TOO_CROWDED && self.pts[b].nclose > TOO_CROWDED {
                continue;
            }
            let least = (self.pts[a].nclose as f32).min(self.pts[b].nclose as f32);
            if least <= MIN_CROWD as f32 {
                self.edges[i].len += MAX_SPEED;
            } else {
                self.edges[i].len += GROW_SPEED + (MAX_SPEED - GROW_SPEED) * (least - MIN_CROWD as f32) / (TOO_CROWDED as f32 - MIN_CROWD as f32);
            }
        }
    }

    fn push_away(&mut self) {
        for i in 0..self.pts.len() {
            let mut close: usize = 0;
            for j in 0..self.pts.len() {
                if j == i || self.pts[i].left == j || self.pts[i].right == j {
                    continue;
                }
                let atob = self.pts[j].pos - self.pts[i].pos;
                let dist = atob.norm();
                if dist < CLOSE_DIST {
                    close += 1;
                }
                if dist > PUSH_DIST {
                    continue;
                }
                if self.pts[i].dead > TOO_DEAD && self.pts[j].dead > TOO_DEAD {
                    continue;
                }
                let diff = atob.normalize();
                let magdiff = diff * (PUSH_DIST - dist); // / 2.0;
                if self.pts[i].dead > TOO_DEAD {
                    self.pts[j].vel = self.pts[j].vel - magdiff * -AVOID_K ;
                } else if self.pts[j].dead > TOO_DEAD {
                    self.pts[i].vel = self.pts[i].vel + magdiff * -AVOID_K ;
                } else {
                    self.pts[i].vel = self.pts[i].vel + magdiff * -AVOID_K / 2.0;
                    self.pts[j].vel = self.pts[j].vel - magdiff * -AVOID_K / 2.0;
                }
            }
            self.pts[i].nclose = close;
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
            let npos = self.pts[a].pos + (self.pts[b].pos - self.pts[a].pos) / 2.0;
            let ob = self.edges[i].b;
            self.edges[i].age = 0;
            self.pts.push(Node{
                pos: npos,
                vel: Vec3::new(0.0, 0.0, 0.0),
                nclose: 0,
                dead: 0,
                left: self.edges[i].a,
                right: ob,
            });
            self.pts[a].right = npt;
            self.pts[b].left = npt;
            self.edges.push(Edge{
                len: len / 2.0,
                curlen: 0.0,
                age: 0,
                a: npt,
                b: ob,
            });
            self.edges[i].len = len / 2.0;
            self.edges[i].b = npt;
        }
    }

    fn tick(&mut self) {
        self.time += 1;
        self.adjust();
        self.push_away();
        self.edge_grow();
        self.edge_split();
        self.move_things();
    }

    fn move_things(&mut self) {
        for i in 0..self.pts.len() {
            if self.pts[i].dead > TOO_DEAD {
                continue;
            }
            if self.pts[i].nclose > TOO_CROWDED && self.pts[i].vel.norm() < DEAD_MOTION {
                self.pts[i].dead += 1;
            } else {
                self.pts[i].dead = 0;
            }
            self.pts[i].vel = self.pts[i].vel * DAMP;
            self.pts[i].pos = self.pts[i].pos + self.pts[i].vel;
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

    window.set_background_color(1.0, 1.0, 1.0);

    window.set_light(Light::StickToCamera);

    while window.render() {
        state.tick();
        state.draw(&mut window);
    }
}
