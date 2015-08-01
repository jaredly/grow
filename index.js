'use strict';

var canv = document.querySelector('#canvas');
var ctx = canv.getContext('2d');

const half = 400;
const TOLERANCE = .001;
const DAMP = 0.75;
const STICK_K = 0.09;
const AVOID_K = 0.01;

const MAX_LEN = .02;
const TOO_CROWDED = 35; // neighbors
const MIN_CROWD = 5;
const TOO_DEAD = 20;
const DEAD_MOTION = .0001;
const CLOSE_DIST = .35;
const PUSH_DIST = .2;
const GROW_SPEED = .0002;
const MAX_SPEED = .0046;

let SHOW_POINTS = false;
let COLOR_SCHEME = 'age';
const RANDOM = true;

ctx.lineWidth = 7;

const x = [];
const y = [];

const vx = [];
const vy = [];

const nclose = [];
const dead = [];

let edges = [];
let edgelen = [];
let curlen = [];
let age = [];

let num_points = 0;
let num_edges = 0;

function init(ipts) {
  num_points = 0;
  num_edges = 0;

  edges = [];
  edgelen = [];
  curlen = [];
  age = [];

  const circumference = ipts * MAX_LEN * .75;
  const radius = circumference / 2 / Math.PI;
  const scale = 2 * Math.PI/ipts
  for (var i=0; i<ipts; i++) {
    let rad;
    if (RANDOM) {
      rad = radius + Math.random() * .1;
    } else {
      rad = radius;
    }
    x[i] = (Math.cos(scale * i) * rad);
    y[i] = (Math.sin(scale * i) * rad);
    vx[i] = (0);
    vy[i] = (0);
    dead[i] = (0);
    nclose[i] = (0);
    num_points += 1;
  }

  for (var i=0; i<ipts; i++) {
    edges.push([i, (i+1) % ipts]);
    edgelen.push(.05);
    age.push(0);
    num_edges += 1;
  }
}

function px_(x) {return half + x*half*.3}

function draw() {
  ctx.clearRect(0, 0, half*2, half*2);
  if (SHOW_POINTS) {
    for (var i=0; i<num_points; i++) {
      if (dead[i] > TOO_DEAD) {
        ctx.fillStyle = 'red';
      } else {
        ctx.fillStyle = 'green';
      }
      ctx.fillRect(px_(x[i]) - 2, px_(y[i]) - 2, 4, 4);
    }
  }
  for (var i=0; i<num_edges; i++) {
    ctx.beginPath();
    let a = edges[i][0];
    let b = edges[i][1];
    ctx.moveTo(px_(x[a]), px_(y[a]));
    ctx.lineTo(px_(x[b]), px_(y[b]));
    age[i] += 1;
    if (COLOR_SCHEME === 'age') {
      ctx.strokeStyle = 'hsl(' + ((age[i] / 2) % 180 + 180) + ',100%,60%)';
    } else {
      if (dead[edges[i][0]] > TOO_DEAD && dead[edges[i][1]] > TOO_DEAD) {
        ctx.strokeStyle = 'black';
      } else if (nclose[edges[i][0]] > TOO_CROWDED && nclose[edges[i][1]] > TOO_CROWDED) {
        ctx.strokeStyle = 'red';
      } else {
        ctx.strokeStyle = 'green';
      }
    }
    ctx.stroke();
  }
}

function push(a, b, min) {
  let dx = x[b] - x[a];
  let dy = y[b] - y[a];
  let fx = Math.max(Math.abs(dx), Math.abs(dy));
  if (fx > min) {
    return fx;
  }
  let dist = Math.sqrt(dx*dx + dy*dy);
  if (dist >= min) {
    return dist;
  }
  let theta = Math.atan2(dy, dx);
  let mag = (min - dist) / 2;
  let ax = Math.cos(theta) * mag * -AVOID_K;
  let ay = Math.sin(theta) * mag * -AVOID_K;
  if (dead[a] > TOO_DEAD) {
    vx[b] -= ax;
    vy[b] -= ay;
  } else if (dead[b] > TOO_DEAD) {
    vx[a] += ax;
    vy[a] += ay;
  } else {
    vx[b] -= ax / 2;
    vy[b] -= ay / 2;
    vx[a] += ax / 2;
    vy[a] += ay / 2;
  }
  return dist;
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
  let ax = Math.cos(theta) * mag * -STICK_K;
  let ay = Math.sin(theta) * mag * -STICK_K;
  vx[b] -= ax;
  vy[b] -= ay;
  vx[a] += ax;
  vy[a] += ay;
}

function adjust() {
  for (var i=0; i<num_edges; i++) {
    let a = edges[i][0];
    let b = edges[i][1];
    match(a, b, edgelen[i]);
  }
}

function move() {
  for (var i=0; i<num_points; i++) {
    if (dead[i] > TOO_DEAD) {
      continue;
    }
    if (nclose[i] > TOO_CROWDED && Math.abs(vx[i]) < DEAD_MOTION && Math.abs(vy[i]) < DEAD_MOTION) {
      dead[i] += 1;
    } else {
      dead[i] = 0;
    }
    vx[i] *= DAMP;
    vy[i] *= DAMP;
    x[i] += vx[i];
    y[i] += vy[i];
  }
}

function pushAway() {
  for (var i=0; i<num_points; i++) {
    var connected = {};
    var close = 0;
    for (var e=0; e<num_edges; e++) {
      if (edges[e][0] === i) {
        connected[edges[e][1]] = true;
      } else if (edges[e][1] === i) {
        connected[edges[e][0]] = true;
      }
    }
    for (var j=0; j<num_points; j++) {
      if (j === i || connected[j]) continue;
      //if (dead[i] > TOO_DEAD && dead[j] > TOO_DEAD) continue;
      var d = push(i, j, PUSH_DIST);
      if (d < CLOSE_DIST) {
        close += 1;
      }
    }
    nclose[i] = close; // Math.max(close, nclose[i]);
  }
}

function edgegrow() {
  for (var i=0; i<edgelen.length; i++) {
    if (nclose[edges[i][0]] > TOO_CROWDED && nclose[edges[i][1]] > TOO_CROWDED) {
      continue;
    }
    let least = Math.min(nclose[edges[i][0]], nclose[edges[i][1]]);
    if (least <= MIN_CROWD) {
      edgelen[i] += MAX_SPEED;
    } else {
      edgelen[i] += GROW_SPEED + (MAX_SPEED - GROW_SPEED) * (least - MIN_CROWD) / (TOO_CROWDED - MIN_CROWD);
    }
  }
}

function splitn(i, n) {
  let a = edges[i][0];
  let b = edges[i][1];
  let dx = x[b] - x[a];
  let dy = y[b] - y[a];
  let ni = num_points;
  edgelen[i] /= n;
  age[i] = 0;
  for (var z=0; z<n-1; z++) {
    edgelen.push(edgelen[i]);
    age.push(0);
  }
  for (var z=1; z<n; z++) {
    x[ni - 1 + z] = (x[a] + z * dx/n);
    y[ni - 1 + z] = (y[a] + z * dy/n);
    vx[ni - 1 + z] = (0);
    vy[ni - 1 + z] = (0);
    dead[ni - 1 + z] = (0);
    nclose[ni - 1 + z] = (0);
    num_points += 1;
  }
  for (var z=0; z<n-2; z++) {
    edges.push([ni + z, ni + z + 1]);
    num_edges += 1;
  }
  // edgelen.push(edgelen[i] * 0.8);
  // edges.push([edges[i][0], edges[i][1]]);
  edges.push([ni + n-2, edges[i][1]]);
  num_edges += 1;
  edges[i][1] = ni;
}

function edgesplit() {
  var olen = edgelen.length;
  // all new edges are added to the end, and we don't need to traverse them
  for (var i=0; i<olen; i++) {
    if (curlen[i] < MAX_LEN || edgelen[i] < MAX_LEN) {
      continue;
    }
    if (nclose[edges[i][0]] > TOO_CROWDED && nclose[edges[i][1]] > TOO_CROWDED) {
      continue;
    }
    splitn(i, 2);
  }
}


function step() { adjust(); pushAway(); edgegrow(); edgesplit(); move(); }
function tick() { step(); draw(); }
function run(n) {
  if (STOP) return console.log('stopped');
  var a = Date.now();
  tick();
  var diff = Date.now() - a;
  if (diff > 100) {
    console.log('Long time', n, diff);
  }
  if (n > 0) {
    return requestAnimationFrame(run.bind(null, n-1));
  }
  console.log('done');
}

let STOP = false;

function test(pts, n) {
  init(pts);
  draw();
  setTimeout(function () {
    let start = performance.now();
    for (var i=0; i<n; i++) {
      step();
    }
    console.log(performance.now() - start);
    draw();
  }, 100);
}

const TEST = false;

if (TEST) {
  test(5, 300);
} else {
  init(6);
  draw();
  setTimeout(function () {
    run(1000);
  }, 500);
}
