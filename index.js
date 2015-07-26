'use strict';

var canv = document.querySelector('#canvas');
var ctx = canv.getContext('2d');

const half = 400;
const TOLERANCE = .001;
const DAMP = 0.95;
const k = 0.05;

let x = [];
let y = [];

let vx = [];
let vy = [];

let edges = [];
let edgelen = [];

var ipts = 10;

for (var i=0; i<ipts; i++) {
  x.push(Math.cos(Math.PI/ipts*2*i) * .1) // * (.2 + Math.random()*.1));
  y.push(Math.sin(Math.PI/ipts*2*i) * .1) // * (.2 + Math.random()*.1));
  vx.push(0);
  vy.push(0);
}

for (var i=0; i<ipts; i++) {
  edges.push([i, (i+1) % ipts]);
  edgelen.push(.05);
  /*
  edges.push([i, (i+2) % ipts]);
  edgelen.push(.4);
  edges.push([i, (i+3) % ipts]);
  edgelen.push(.6);
  edges.push([i, (i+4) % ipts]);
  edgelen.push(.7);
  */
}

function px_(x) {return half + x*half*2}

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

function push(a, b, min) {
  let dx = x[b] - x[a];
  let dy = y[b] - y[a];
  let dist = Math.sqrt(dx*dx + dy*dy);
  if (dist >= min) {
    return;
  }
  let theta = Math.atan2(dy, dx);
  let mag = (min - dist) / 2;
  let ax = Math.cos(theta) * mag;
  let ay = Math.sin(theta) * mag;
  vx[b] = vx[b] - -k * ax;
  vy[b] = vy[b] - -k * ay;
  vx[a] = vx[a] + -k * ax;
  vy[a] = vy[a] + -k * ay;
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
  pushAway();
}

function pushAway() {
  for (var i=0; i<x.length; i++) {
    let connected = {};
    for (var e=0; e<edges.length; e++) {
      if (edges[e][0] === i) connected[edges[e][1]] = true;
      else if (edges[e][1] === i) connected[edges[e][0]] = true;
    }
    for (var j=0; j<x.length; j++) {
      if (j === i || connected[j]) continue;
      push(i, j, .01);
    }
  }
}

function edgegrow() {
  for (var i=0; i<edgelen.length; i++) {
    edgelen[i] += .0003;
  }
}

function splitn(i, n) {
  let a = edges[i][0];
  let b = edges[i][1];
  let dx = x[b] - x[a];
  let dy = y[b] - y[a];
  /*
  var dcenter = Math.sqrt(dx*dx + dy*dy);
  if (dcenter > .15) {
    return;
  }
  */
  let ni = x.length;
  edgelen[i] /= n;
  for (var z=0; z<n-1; z++) {
    edgelen.push(edgelen[i]);
  }
  for (var z=1; z<n; z++) {
    x.push(x[a] + z * dx/n);
    y.push(y[a] + z * dy/n);
    vx.push(0);
    vy.push(0);
  }
  for (var z=0; z<n-2; z++) {
    edges.push([ni + z, ni + z + 1]);
  }
  // edgelen.push(edgelen[i] * 0.8);
  // edges.push([edges[i][0], edges[i][1]]);
  edges.push([ni + n-2, edges[i][1]]);
  edges[i][1] = ni;
}

//var snum = 2;

function edgesplit() {
  var olen = edgelen.length;
  var changed = false;
  // all new edges are added to the end, and we don't need to traverse them
  for (var i=0; i<olen; i++) {
    if (edgelen[i] < .1) {
      continue;
    }
    /*
    if (!changed && snum > 2) {
      console.log('down');
      snum -= 1;
    }
    changed = true;
    */
    splitn(i, 4);
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
