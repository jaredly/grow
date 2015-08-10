#![allow(dead_code)]

extern crate nalgebra as na;
use std::f32;
use na::{Pnt2, Vec3, Pnt3, FloatPnt, Norm};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::mem;

const TOLERANCE: f32 = 0.001;
const DAMP: f32 = 0.75;
const STICK_K: f32 = 0.09;
const AVOID_K: f32 = 0.02;

const MAX_LEN: f32 = 0.5;
const TOO_CROWDED: usize = 34;
const MIN_CROWD: i32 = 5;
const TOO_DEAD: i32 = 100;
const DEAD_MOTION: f32 = 0.0001;
const CLOSE_DIST: f32 = 2.0;
const CLOSE_DIST_SQ: f32 = 2.0 * 2.0;
const PUSH_DIST: f32 = 0.8;
const PUSH_DIST_SQ: f32 = 0.8 * 0.8;
const GROW_SPEED: f32 = 0.01;
const MAX_SPEED: f32  = 0.02;
const GRAVITY: f32 = 0.01;
const GRAV_TOP: f32 = 10.0;
const GRAV_BOTTOM: f32 = 8.0;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Clone)]
struct Edge {
    pub a: usize,
    pub b: usize,
    age: usize,
    len: f32,
    curlen: f32,
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Clone)]
struct Node {
    pos: Pnt3<f32>,
    nclose: usize,
    siblings: usize,
    age: usize,
    dead: i32,
    left: usize,
    right: usize,
    trunk: bool,
}

type Bins = HashMap<(usize, usize, usize), Vec<usize>>;

macro_rules! check_bin {
    ($bins:expr, $pos:expr, $me:expr, $first:expr, $second:ident) => {
        $bins.get($pos).map::<usize, _>(
            |arr| arr.iter().map(|j|{push_two(&$me.pts, $first, *j, &mut $me.vels)}).fold(0, |a, i| a + i)
        ).map(|val| $second += val);
        /* An alternate way that seems slower
        match $bins.get($pos) {
            Some(arr) => {
                for j in arr {$second += $me.push_two($first, *j);}
            },
            None => {}
        }
        */
    }
}

macro_rules! check_bin_mm {
    ($bins:expr, $pos:expr, $pts:expr, $sender:expr, $first:expr, $second:ident) => {
        $bins.get($pos).map::<usize, _>(
            |arr| arr.iter().map(|j|{push_two_mm($pts, $first, *j, $sender)}).fold(0, |a, i| a + i)
        ).map(|val| $second += val);
        /* An alternate way that seems slower
        match $bins.get($pos) {
            Some(arr) => {
                for j in arr {$second += $me.push_two($first, *j);}
            },
            None => {}
        }
        */
    }
}

fn calc_one((minx, miny, minz): (f32, f32, f32), pts: &Vec<Node>, i: usize, bins: &Bins, sender: &mpsc::Sender<(usize, Vec3<f32>)>, close_sender: &mpsc::Sender<(usize, usize)>) {
    let mut close: usize = 0;
    let Pnt3{x, y, z} = pts[i].pos;
    let xp = (x - minx) / CLOSE_DIST / 2.0;
    let yp = (y - miny) / CLOSE_DIST / 2.0;
    let zp = (z - minz) / CLOSE_DIST / 2.0;
    let xn = xp as usize;
    let yn = yp as usize;
    let zn = zp as usize;
    let nx = if xp.round() > xp {xn + 1} else if xn > 0 {xn - 1} else {xn};
    let ny = if yp.round() > yp {yn + 1} else if yn > 0 {yn - 1} else {yn};
    let nz = if zp.round() > zp {zn + 1} else if zn > 0 {zn - 1} else {zn};

    check_bin_mm!(bins, &(xn, yn, zn), pts, sender, i, close);
    if nx != xn {
        check_bin_mm!(bins, &(nx, yn, zn), pts, sender, i, close);
        if ny != yn {
            check_bin_mm!(bins, &(nx, ny, zn), pts, sender, i, close);
            if nz != zn {
                check_bin_mm!(bins, &(nx, ny, nz), pts, sender, i, close);
            }
        }
        if nz != zn {
            check_bin_mm!(bins, &(nx, yn, nz), pts, sender, i, close);
        }
    }
    if ny != yn {
        check_bin_mm!(bins, &(xn, ny, zn), pts, sender, i, close);
        if nz != zn {
            check_bin_mm!(bins, &(xn, ny, nz), pts, sender, i, close);
        }
    }
    if nz != zn {
        check_bin_mm!(bins, &(xn, yn, nz), pts, sender, i, close);
    }
    close_sender.send((i, close));
}

fn push_two_mm(pts: &Vec<Node>, i: usize, j: usize, sender: &mpsc::Sender<(usize, Vec3<f32>)>) -> usize {
    if j == i || pts[i].left == j || pts[i].right == j {
        return 0;
    }
    let atob = pts[j].pos - pts[i].pos;
    let sqdist = atob.sqnorm();
    if sqdist > PUSH_DIST_SQ {
        return if sqdist < CLOSE_DIST_SQ {1} else {0}
    }
    if pts[i].dead > TOO_DEAD && pts[j].dead > TOO_DEAD {
        return 1;
    }
    let dist = atob.norm();
    let diff = atob.normalize();
    let magdiff = diff * (PUSH_DIST - dist); // / 2.0;
    if pts[i].dead > TOO_DEAD {
        sender.send((j, -magdiff * -AVOID_K));
    } else if pts[j].dead > TOO_DEAD {
        sender.send((i, magdiff * -AVOID_K));
    } else {
        sender.send((i, magdiff * -AVOID_K / 2.0));
        sender.send((j, -magdiff * -AVOID_K / 2.0));
    }
    return 1;
}


fn push_two(pts: &Vec<Node>, i: usize, j: usize, vels: &mut Vec<Vec3<f32>>) -> usize {
    if j == i || pts[i].left == j || pts[i].right == j {
        return 0;
    }
    let atob = pts[j].pos - pts[i].pos;
    let sqdist = atob.sqnorm();
    if sqdist > PUSH_DIST_SQ {
        return if sqdist < CLOSE_DIST_SQ {1} else {0}
    }
    if pts[i].dead > TOO_DEAD && pts[j].dead > TOO_DEAD {
        return 1;
    }
    let dist = atob.norm();
    let diff = atob.normalize();
    let magdiff = diff * (PUSH_DIST - dist); // / 2.0;
    if pts[i].dead > TOO_DEAD {
        vels[j] = vels[j] - magdiff * -AVOID_K ;
    } else if pts[j].dead > TOO_DEAD {
        vels[i] = vels[i] + magdiff * -AVOID_K ;
    } else {
        vels[i] = vels[i] + magdiff * -AVOID_K / 2.0;
        vels[j] = vels[j] - magdiff * -AVOID_K / 2.0;
    }
    return 1;
}

fn get_mins(pts: &[Node]) -> (f32, f32, f32) {
    let mut minx = 0.0;
    let mut miny = 0.0;
    let mut minz = 0.0;
    for pnt in pts {
        if pnt.pos.x < minx {minx = pnt.pos.x;}
        if pnt.pos.y < miny {miny = pnt.pos.y;}
        if pnt.pos.z < minz {minz = pnt.pos.z;}
    }
    (minx, miny, minz)
}

fn make_bins(pts: &[Node], minx: f32, miny: f32, minz: f32) -> Bins {
    let mut bins = HashMap::new();
    for (i, &ref pnt) in pts.iter().enumerate() {
        let pos = (
            ((pnt.pos.x - minx) / CLOSE_DIST / 2.0).floor() as usize,
            ((pnt.pos.y - miny) / CLOSE_DIST / 2.0).floor() as usize,
            ((pnt.pos.z - minz) / CLOSE_DIST / 2.0).floor() as usize,
        );
        let val = bins.entry(pos).or_insert(vec![]);
        val.push(i);
    }
    bins
}

impl Node {
    fn new(pos: Pnt3<f32>, left: usize, right: usize, trunk: bool) -> Node {
        Node {
            pos: pos,
            siblings: 2,
            age: 0,
            trunk: trunk,
            nclose: 0,
            dead: 0,
            left: left,
            right: right,
        }
    }

    fn radial(theta: f32, radius: f32, left: usize, right: usize, cx: f32, cz: f32) -> Node {
        Node {
            pos: Pnt3 {
                x: cx + theta.sin() * radius,
                z: cz + theta.cos() * radius,
                y: 0.0,
            },
            siblings: 2,
            age: 0,
            trunk: true,
            nclose: 0,
            dead: 0,
            left: left,
            right: right,
        }
    }
}

pub trait DrawState {
    fn draw_state(&mut self, state: &mut State, off: f32);
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Clone)]
pub struct State {
    pub time: i32,
    pts: Vec<Node>,
    vels: Vec<Vec3<f32>>,
    edges: Vec<Edge>,
    pub tris: Vec<Pnt3<u32>>,
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

impl State {

    pub fn init() -> State {
        State{
            time: 0,
            pts: vec![],
            vels: vec![],
            edges: vec![],
            tris: vec![],
        }
    }

    pub fn print_info(&self) {
        println!("Edges: {}, Points: {}, time: {}", self.edges.len(), self.pts.len(), self.time);
    }

    #[inline]
    pub fn edge_color(&self, i: usize, off: f32) -> Pnt3<f32> {
        hsl(((1.8 - self.edges[i].age as f32 / self.time as f32) * 180.0 + off) % 360.0, 1.0, 0.3)
    }

    #[inline]
    pub fn pos(&self, i: usize) -> &Pnt3<f32> {
        &self.pts[i].pos
    }

    #[inline]
    pub fn edge_pts(&self, e: usize) -> (usize, usize) {
        (self.edges[e].a, self.edges[e].b)
    }

    #[inline]
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn coords(&self) -> Vec<Pnt3<f32>> {
        self.pts.iter().map(|n| n.pos).collect()
    }

    pub fn coord_colors(&self, off: f32) -> Vec<Pnt2<f32>> {
        self.pts.iter().map(|n| 
            Pnt2::new(
                1.0 - n.age as f32 / self.time as f32,
                if n.trunk {1.0} else {0.0}
            )
        ).collect()
    }

    pub fn add_triangle(&mut self, rad: f32, x: f32, z: f32) {

        let n0 = self.pts.len();
        self.pts.push(Node::radial(0.0, rad * 2.0, n0 + 1, n0 + 2, x, z));
        self.vels.push(Vec3::new(0.0, 0.0, 0.0));
        self.pts.push(Node::radial(f32::consts::PI / 3.0 * 2.0, rad, n0 + 2, n0 + 0, x, z));
        self.vels.push(Vec3::new(0.0, 0.0, 0.0));
        self.pts.push(Node::radial(f32::consts::PI / 3.0 * 4.0, rad, n0 + 0, n0 + 1, x, z));
        self.vels.push(Vec3::new(0.0, 0.0, 0.0));

        self.tris.push(Pnt3::new(n0 as u32, n0 as u32 + 1, n0 as u32 + 2));

        let len = MAX_LEN / 4.0;
        let curlen = self.pts[0].pos.dist(&self.pts[1].pos);

        self.edges.push(Edge {
            a: n0,
            b: n0 + 1,
            len: len,
            curlen: curlen,
            age: 0,
        });

        self.edges.push(Edge {
            a: n0 + 1,
            b: n0 + 2,
            len: len,
            curlen: curlen,
            age: 0,
        });

        self.edges.push(Edge {
            a: n0 + 0,
            b: n0 + 2,
            len: len,
            curlen: curlen,
            age: 0,
        });
    }

    pub fn start(&mut self, num: usize) {
        let rad = 0.2;

        self.add_triangle(rad, 0.0, 0.0);
        self.add_triangle(rad, 0.4, 0.4);
        self.add_triangle(rad, -0.4, 0.4);
    }

    pub fn tick(&mut self) {
        self.time += 1;
        self.adjust();
        self.push_multi();
        self.edge_grow();
        self.edge_split();
        self.move_things();
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
            self.vels[a] = self.vels[a] + mdiff;
            self.vels[b] = self.vels[b] - mdiff;
        }
    }

    fn edge_grow(&mut self) {
        for i in 0..self.edges.len() {
            self.edges[i].age += 1;
            let Edge{a, b, len, ..} = self.edges[i];
            if len > MAX_LEN {
                continue;
            }
            let max_crowd = if self.pts[a].pos.y.max(self.pts[b].pos.y) > GRAV_TOP {TOO_CROWDED + 20} else {TOO_CROWDED};
            if self.pts[a].nclose > max_crowd && self.pts[b].nclose > max_crowd {
                continue;
            }
            let least = (self.pts[a].nclose as f32).min(self.pts[b].nclose as f32);
            if least <= MIN_CROWD as f32 {
                self.edges[i].len += MAX_SPEED;
            } else {
                self.edges[i].len += GROW_SPEED + (MAX_SPEED - GROW_SPEED) * (least - MIN_CROWD as f32) / (max_crowd as f32 - MIN_CROWD as f32);
            }
        }
    }

    fn push_multi(&mut self) {
        if self.pts.len() < 40 {
            return self.push_away();
        }
        let num_chunks = 10;
        let (minx, miny, minz) = get_mins(&self.pts);
        let bins = make_bins(&self.pts, minx, miny, minz);
        let parts = self.pts.len() / num_chunks;
        let (sender, receiver) = mpsc::channel();
        let (close_sender, close_receiver) = mpsc::channel();
        let chunk = self.pts.len() / num_chunks;
        let len = self.pts.len();
        // throw away the safety... so we can escape lifetime checking, which *we* know is safe,
        // but the checker doesn't understand.
        // It's **only** safe b/c we make sure all threads have joined before this function
        // returns, by iterating on the receiver, which will only terminate once all `sender`s have
        // been dropped.
        let ptsref = unsafe {mem::transmute(&self.pts)};
        let binsref = unsafe {mem::transmute(&bins)};
        for i in 0..num_chunks {
            let sender = sender.clone();
            let close_sender = close_sender.clone();
            thread::spawn(move || {
                let max = if i == num_chunks - 1 {len} else {(i + 1) * chunk};
                for z in i*chunk..max {
                    calc_one((minx, miny, minz), ptsref, z, binsref, &sender, &close_sender);
                }
            });
        }

        // if we don't do this, everything will hang :)
        drop(sender);
        drop(close_sender);
        for (i, vel) in receiver {
            self.vels[i] = self.vels[i] + vel;
        }
        // we only start reading these once we *know* that all threads are done
        for (i, close) in close_receiver {
            self.pts[i].nclose = close;
        }
    }

    fn push_away(&mut self) {
        let (minx, miny, minz) = get_mins(&self.pts);
        let bins = make_bins(&self.pts, minx, miny, minz);
        // TODO I'm double-calculating all of this... I need to remove it, but also still calculate
        // nclose correctly.
        for i in 0..self.pts.len() {
            let mut close: usize = 0;
            let Pnt3{x, y, z} = self.pts[i].pos;
            let xp = (x - minx) / CLOSE_DIST / 2.0;
            let yp = (y - miny) / CLOSE_DIST / 2.0;
            let zp = (z - minz) / CLOSE_DIST / 2.0;
            let xn = xp as usize;
            let yn = yp as usize;
            let zn = zp as usize;
            let nx = if xp.round() > xp {xn + 1} else if xn > 0 {xn - 1} else {xn};
            let ny = if yp.round() > yp {yn + 1} else if yn > 0 {yn - 1} else {yn};
            let nz = if zp.round() > zp {zn + 1} else if zn > 0 {zn - 1} else {zn};

            check_bin!(bins, &(xn, yn, zn), self, i, close);
            if nx != xn {
                check_bin!(bins, &(nx, yn, zn), self, i, close);
                if ny != yn {
                    check_bin!(bins, &(nx, ny, zn), self, i, close);
                    if nz != zn {
                        check_bin!(bins, &(nx, ny, nz), self, i, close);
                    }
                }
                if nz != zn {
                    check_bin!(bins, &(nx, yn, nz), self, i, close);
                }
            }
            if ny != yn {
                check_bin!(bins, &(xn, ny, zn), self, i, close);
                if nz != zn {
                    check_bin!(bins, &(xn, ny, nz), self, i, close);
                }
            }
            if nz != zn {
                check_bin!(bins, &(xn, yn, nz), self, i, close);
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
            let trunk = self.pts[a].trunk || self.pts[b].trunk;
            self.edges[i].age = 0;
            self.pts.push(Node::new(npos, self.edges[i].a, ob, trunk));
            self.vels.push(Vec3::new(0.0, 0.0, 0.0));
            self.tris.push(Pnt3::new(npt as u32, a as u32, b as u32));
            self.pts[a].siblings += 1;
            self.pts[b].siblings += 1;
            self.pts[a].right = npt;
            self.pts[b].left = npt;
            self.edges.push(Edge{
                len: len / 2.0,
                curlen: 0.0,
                age: 0,
                a: npt,
                b: ob,
            });
            let oa = self.edges[i].a;
            self.edges.push(Edge{
                len: len / 2.0,
                curlen: 0.0,
                age: 0,
                a: oa,
                b: ob,
            });
            self.edges[i].len = len / 2.0;
            self.edges[i].b = npt;
        }
    }

    fn move_things(&mut self) {
        for i in 0..self.pts.len() {
            if i >= 10 {
                if self.pts[i].pos.y > GRAV_TOP {
                    self.pts[i].trunk = false;
                }
                if self.pts[i].trunk {
                    self.vels[i].y += GRAVITY;
                } /* BUSHY else if self.pts[i].pos.y > GRAV_BOTTOM {
                    self.pts[i].vel.y -= GRAVITY / 4.0;// * (GRAV_TOP - self.pts[i].pos.y) / GRAV_TOP;
                } */
            } else {
                self.vels[i].y = 0.0;
            }
            self.vels[i] = self.vels[i] * DAMP;
            self.pts[i].pos = self.pts[i].pos + self.vels[i];
            self.pts[i].age += 1;
        }
    }
}

