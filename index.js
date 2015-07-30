'use strict';

var canv = document.querySelector('#canvas');
var ctx = canv.getContext('2d');

const half = 400;
const TOLERANCE = .001;
const DAMP = 0.85;
const k = 0.05;

const MAX_LEN = .05;
const TOO_CLOSE = 20;
const TOO_DEAD = 20;
const DEAD_MOTION = .0001;
const CLOSE_DIST = .25;
const PUSH_DIST = .2;

const SHOW_POINTS = false;
const COLOR_SCHEME = 'age';

const xb = new ArrayBuffer(8 * 2000);
const yb = new ArrayBuffer(8 * 2000);
const vxb = new ArrayBuffer(8 * 2000);
const vyb = new ArrayBuffer(8 * 2000);
const ncloseb = new ArrayBuffer(1 * 2000);
const deadb = new ArrayBuffer(1 * 2000);

const x = new Float64Array(xb);
const y = new Float64Array(yb);

const vx = new Float64Array(vxb);
const vy = new Float64Array(vyb);

const nclose = new Uint8Array(ncloseb);
const dead = new Uint8Array(deadb);

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

  for (var i=0; i<ipts; i++) {
    x[i] = (Math.cos(Math.PI/ipts*2*i) * .1);//(.2 + Math.random()*.1));
    y[i] = (Math.sin(Math.PI/ipts*2*i) * .1);//(.2 + Math.random()*.1));
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
      ctx.strokeStyle = 'hsl(' + (age[i] % 360) + ',100%,60%)';
    } else {
      if (dead[edges[i][0]] > TOO_DEAD && dead[edges[i][1]] > TOO_DEAD) {
        ctx.strokeStyle = 'black';
      } else if (nclose[edges[i][0]] > TOO_CLOSE && nclose[edges[i][1]] > TOO_CLOSE) {
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
  /*
  if (Math.abs(dx) + Math.abs(dy) >= min) {
    return Math.abs(dx) + Math.abs(dy);
  }
  */
  let dist = Math.sqrt(dx*dx + dy*dy);
  if (dist >= min) {
    return dist;
  }
  let theta = Math.atan2(dy, dx);
  let mag = (min - dist) / 2;
  let ax = Math.cos(theta) * mag;
  let ay = Math.sin(theta) * mag;
  if (dead[a] > TOO_DEAD) {
    vx[b] = vx[b] - -k * ax;
    vy[b] = vy[b] - -k * ay;
  } else if (dead[b] > TOO_DEAD) {
    vx[a] = vx[a] + -k * ax;
    vy[a] = vy[a] + -k * ay;
  } else {
    vx[b] = vx[b] - -k * ax / 2;
    vy[b] = vy[b] - -k * ay / 2;
    vx[a] = vx[a] + -k * ax / 2;
    vy[a] = vy[a] + -k * ay / 2;
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
  let ax = Math.cos(theta) * mag;
  let ay = Math.sin(theta) * mag;
  vx[b] = vx[b] - -k * ax;
  vy[b] = vy[b] - -k * ay;
  vx[a] = vx[a] + -k * ax;
  vy[a] = vy[a] + -k * ay;
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
    if (Math.abs(vx[i]) < DEAD_MOTION && Math.abs(vy[i]) < DEAD_MOTION) {
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
      if (dead[i] > TOO_DEAD && dead[j] > TOO_DEAD) continue;
      var d = push(i, j, PUSH_DIST);
      if (d < CLOSE_DIST) {
        close += 1;
      }
    }
    nclose[i] = Math.max(close, nclose[i]);
  }
}

function edgegrow() {
  var edst = [];
  var esum = 0;
  var emax = 0;
  for (var i=0; i<edgelen.length; i++) {
    let a = edges[i][0];
    let b = edges[i][1];
    let dx = x[b] - x[a];
    let dy = y[b] - y[a];
    let cx = x[a] + dx/2;
    let cy = y[a] + dy/2;
    let dcenter = Math.sqrt(cx*cx + cy*cy);
    curlen[i] = Math.sqrt(dx*dx + dy*dy);
    edst.push(dcenter);
    esum += dcenter;
    if (dcenter > emax) {
      emax = dcenter;
    }
  }
  var eavg = esum / (edgelen.length + 1);
  for (var i=0; i<edgelen.length; i++) {
    if (age[i] > 100) continue;
    if (nclose[edges[i][0]] > TOO_CLOSE && nclose[edges[i][1]] > TOO_CLOSE) {
      continue;
    }
    edgelen[i] += .0008;
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
    if (nclose[edges[i][0]] > TOO_CLOSE && nclose[edges[i][1]] > TOO_CLOSE) {
      continue;
    }
    splitn(i, 2);
  }
}


function step() { adjust(); pushAway(); edgegrow(); edgesplit(); move(); }
function tick() { step(); draw(); }
function run(n) {
  var a = Date.now();
  tick();
  var diff = Date.now() - a;
  if (diff > 100) {
    console.log('Long time', n, diff);
  }
  if (n > 0) {
    return requestAnimationFrame(run.bind(null, n-1));
  }
}

function test(pts, n) {
  init(pts);
  draw();
  setTimeout(function () {
    let start = performance.now();
    for (var i=0; i<n; i++) {
      tick();
    }
    console.log(performance.now() - start);
    console.log('done');
  }, 100);
}

init(10);
draw();
setTimeout(function () {
  run(300);
}, 500);
//test(5, 300);
