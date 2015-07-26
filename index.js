'use strict';

var canv = document.querySelector('#canvas');
var ctx = canv.getContext('2d');

const half = 400;
const mx = 4;
const TOLERANCE = .001;
const m = 10;
const DAMP = 0.89;
const k = 0.1;

let x = [];
let y = [];

let vx = [];
let vy = [];

let edges = [];
let edgelen = [];

for (var i=0; i<10; i++) {
  x.push(Math.cos(Math.PI/5*i) * (.4 + Math.random()*.1));
  y.push(Math.sin(Math.PI/5*i) * (.4 + Math.random()*.1));
  vx.push(0);
  vy.push(0);
}

for (var i=0; i<10; i++) {
  edges.push([i, (i+1) % 10]);
  edgelen.push(.1);
  edges.push([i, (i+2) % 10]);
  edgelen.push(.4);
  edges.push([i, (i+3) % 10]);
  edgelen.push(.6);
  edges.push([i, (i+4) % 10]);
  edgelen.push(.7);
}

/*
for (var i=0; i<8; i+=2) {
  edges.push([i, i+2]);
  edgelen.push(.1);
}

for (var i=0; i<7; i+=1) {
  edges.push([i,i+3]);
  edgelen.push(Math.sqrt(3)*.05*2);
}
*/

/*
edges.push([0,2]);
edgelen.push(.1);
edges.push([1,3]);
edgelen.push(.1);
edges.push([2,4]);
edgelen.push(.1);
edges.push([3,5]);
edgelen.push(.1);
*/

function px_(x) {return half + x*half}

function draw() {
  ctx.clearRect(0, 0, half*2, half*2);
  for (var i=0; i<x.length; i++) {
    ctx.fillRect(half + x[i] * half - 4, half + y[i] * half - 4, 8, 8);
  }
  for (var i=0; i<edges.length; i++) {
    ctx.beginPath();
    let a = edges[i][0];
    let b = edges[i][1];
    ctx.moveTo(px_(x[a]), px_(y[a]));
    ctx.lineTo(px_(x[b]), px_(y[b]));
    ctx.stroke();
    let dx = x[b] - x[a];
    let dy = y[b] - y[a];
    let dist = Math.sqrt(dx*dx + dy*dy);
    if (Math.abs(dist - edgelen[i]) < TOLERANCE) {
      continue;
    }
    let theta = Math.atan2(dy, dx);
    let mag = (edgelen[i] - dist) / 2;
    let ax = Math.cos(theta) * mag;
    let ay = Math.sin(theta) * mag;
    /*
    vx[b] = DAMP * (vx[b] + -k * ax);
    vy[b] = DAMP * (vy[b] + -k * ay);
    vx[a] = DAMP * (vx[a] - -k * ax);
    vy[a] = DAMP * (vy[a] - -k * ay);
    */
  }
}

function match(a, b, length) {
  let dx = x[b] - x[a];
  let dy = y[b] - y[a];
  let dist = Math.sqrt(dx*dx + dy*dy);
  if (Math.abs(dist - length) < TOLERANCE) {
    return;
  }
  let theta = Math.atan2(dy, dx);
  let mag = (length - dist) / 2;
  let ax = Math.cos(theta) * mag;
  let ay = Math.sin(theta) * mag;
  vx[b] = vx[b] - -k * ax;
  vy[b] = vy[b] - -k * ay;
  vx[a] = vx[a] + -k * ax;
  vy[a] = vy[a] + -k * ay;
}

function adjust() {
  for (var i=0; i<edges.length; i++) {
    let a = edges[i][0];
    let b = edges[i][1];
    match(a, b, edgelen[i]);
  }
}

function move() {
  for (var i=0; i<x.length; i++) {
    vx[i] *= DAMP;
    vy[i] *= DAMP;
    x[i] += vx[i];
    y[i] += vy[i];
  }
}

function step() {
  adjust();
  move();
  grav();
}

let gx = .001;

function grav() {
  for (var i=0; i<x.length; i++) {
    vy[i] += gx;
    if (y[i] >= 1) {
      y[i] = 1;
      vy[i] = 0;
    }
  }
}

function step_() {
  for (var i=0; i<x.length; i++) {
    let dx = (x[i] - tx[i])
    let dy = (y[i] - ty[i])
    vx[i] = DAMP * (vx[i] + -k * dx);
    vy[i] = DAMP * (vy[i] + -k * dy);
    x[i] += vx[i];
    y[i] += vy[i];
    if (Math.abs(dx) < .0001 && Math.abs(dy) < .0001 && Math.abs(vx[i]) < .0001 && Math.abs(vy[i]) < .0001) {
      tx[i] = Math.random() - .5;
      ty[i] = Math.random() - .5;
    }
  }
}

function tick() {
  step();
  draw();
}

function run(n) {
  tick();
  if (n > 0) {
    return requestAnimationFrame(run.bind(null, n-1));
  }
  console.log('done');
}

draw();
setTimeout(function () {
  run(1000);
}, 500);
