'use strict';

var canv = document.querySelector('#canvas');
var ctx = canv.getContext('2d');

const half = 400;
const TOLERANCE = .001;
const DAMP = 0.89;
const k = 0.1;

let x = [];
let y = [];

let vx = [];
let vy = [];

let edges = [];
let edgelen = [];

for (var i=0; i<10; i++) {
  x.push(Math.cos(Math.PI/5*i) * .1) // * (.2 + Math.random()*.1));
  y.push(Math.sin(Math.PI/5*i) * .1) // * (.2 + Math.random()*.1));
  vx.push(0);
  vy.push(0);
}

for (var i=0; i<10; i++) {
  edges.push([i, (i+1) % 10]);
  edgelen.push(.05);
  /*
  edges.push([i, (i+2) % 10]);
  edgelen.push(.4);
  edges.push([i, (i+3) % 10]);
  edgelen.push(.6);
  edges.push([i, (i+4) % 10]);
  edgelen.push(.7);
  */
}

function px_(x) {return half + x*half}

function draw() {
  ctx.clearRect(0, 0, half*2, half*2);
  /*
  for (var i=0; i<x.length; i++) {
    ctx.fillRect(half + x[i] * half - 4, half + y[i] * half - 4, 8, 8);
  }
  */
  for (var i=0; i<edges.length; i++) {
    ctx.beginPath();
    let a = edges[i][0];
    let b = edges[i][1];
    ctx.moveTo(px_(x[a]), px_(y[a]));
    ctx.lineTo(px_(x[b]), px_(y[b]));
    ctx.stroke();
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
  edgegrow();
  edgesplit();
  // grav();
}

function edgegrow() {
  for (var i=0; i<edgelen.length; i++) {
    edgelen[i] += .0005;
  }
}

function split2(i) {
  edgelen[i] /= 2;
  edgelen.push(edgelen[i]);
  let a = edges[i][0];
  let b = edges[i][1];
  let dx = x[b] - x[a];
  let dy = y[b] - y[a];
  let ni = x.length;
  x.push(x[a] + dx/2);
  y.push(y[a] + dy/2);
  vx.push(0);
  vy.push(0);
  edges.push([ni, edges[i][1]]);
  edges[i][1] = ni;
}

function split3(i) {
  edgelen[i] /= 3;
  edgelen.push(edgelen[i]);
  edgelen.push(edgelen[i]);
  let a = edges[i][0];
  let b = edges[i][1];
  let dx = x[b] - x[a];
  let dy = y[b] - y[a];
  let ni = x.length;
  x.push(x[a] + dx/3);
  y.push(y[a] + dy/3);
  vx.push(0);
  vy.push(0);
  let n2 = x.length;
  x.push(x[a] + 2*dx/3);
  y.push(y[a] + 2*dy/3);
  vx.push(0);
  vy.push(0);
  edges.push([ni, n2]);
  edges.push([n2, edges[i][1]]);
  edges[i][1] = ni;
}

function edgesplit() {
  var olen = edgelen.length;
  // all new edges are added to the end, and we don't need to traverse them
  for (var i=0; i<olen; i++) {
    if (edgelen[i] < .1) {
      continue;
    }
    split3(i);
  }
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
  run(500);
}, 500);
