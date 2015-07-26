'use strict';

var canv = document.querySelector('#canvas');
var ctx = canv.getContext('2d');

const half = 400;
const mx = 4;

var xs = [0, 10, 5];
var ys = [0, 5, 10];
var vxs = [0, 0, 0];
var vys = [0, 0, 0];
// var ts = [0, 0, 0];
var lengths = [5, 5, 5];
var e1 = [0, 1, 2];
var e2 = [1, 2, 0];


var pts = [{
  x: 0,
  y: 0,
  edges: [0, 2],
}, {
  x: 10,
  y: 5,
  edges: [0, 1],
}, {
  x: 5,
  y: 10,
  edges: [1, 2],
}];
var edges = [{
  a: 0,
  b: 1,
  length: 5,
}, {
  a: 1,
  b: 2,
  length: 5,
}, {
  a: 2,
  b: 0,
  length: 5,
}];

const TOLERANCE = .001;
const m = 10;

function draw() {
  ctx.save();
  ctx.translate(400, 400);
  ctx.lineWidth = 1;
  ctx.strokeStyle = 'red';
  edges.forEach(function (edge) {
    ctx.beginPath();
    var p1 = pts[edge.a];
    var p2 = pts[edge.b];
    ctx.moveTo(p1.x * m, p1.y * m);
    ctx.lineTo(p2.x * m, p2.y * m);
    ctx.stroke();

    ///var theta = Math.atan2(dy, dx);
    ///var gx = Math.cos(theta) * edge.length;
    ///var gy = Math.sin(theta) * edge.length;
  });
  ctx.fillStyle = 'blue';
  pts.forEach(function (pt) {
    ctx.fillRect(pt.x * m - 1, pt.y * m - 1, 2, 2);
  });
  ctx.restore();
}

const k = 0.002;
const DAMP = 0.2;

function drawMore() {
  for (var i=0; i<pts.length; i++) {
    var dx = 0;
    var dy = 0;
    var pt = pts[i];
    for (var e=0; e<pt.edges.length; e++) {
      var edge = edges[pt.edges[e]];
      var opi = edge.a === i ? edge.b : edge.a;
      var opt = pts[opi];
      const xdiff = pts[opi].x - pts[i].x;
      const ydiff = pts[opi].y - pts[i].y;
      const d = Math.sqrt(xdiff*xdiff + ydiff*ydiff);
      if (Math.abs(d - edge.length) < TOLERANCE) {
        continue;
      }

      console.log(d - edge.length, edge.length, d);
      const theta = Math.atan2(ydiff, xdiff);
      var gx = Math.cos(theta) * edge.length;
      var gy = Math.sin(theta) * edge.length;
      var fx = -k * (pt.x - gx);
      vxs[i] = DAMP * (vxs[i] + fx);
      dx += vxs[i];
      var fy = -k * (pt.y - gy);
      vys[i] = DAMP * (vys[i] + fy);
      dy += vys[i];
    }
    npts.push({
      x: pt.x + dx,
      y: pt.y + dy,
      edges: pt.edges,
    });
  }
  pts = npts;
}


function adjust() {
  var npts = [];
  for (var i=0; i<pts.length; i++) {
    var dx = 0;
    var dy = 0;
    var pt = pts[i];
    for (var e=0; e<pt.edges.length; e++) {
      var edge = edges[pt.edges[e]];
      var opi = edge.a === i ? edge.b : edge.a;
      var opt = pts[opi];
      const xdiff = pts[opi].x - pts[i].x;
      const ydiff = pts[opi].y - pts[i].y;
      const d = Math.sqrt(xdiff*xdiff + ydiff*ydiff);
      if (Math.abs(d - edge.length) < TOLERANCE) {
        continue;
      }

      console.log(d - edge.length, edge.length, d);
      const theta = Math.atan2(ydiff, xdiff);
      var gx = Math.cos(theta) * edge.length;
      var gy = Math.sin(theta) * edge.length;
      var fx = -k * (pt.x - gx);
      vxs[i] = DAMP * (vxs[i] + fx);
      dx += vxs[i];
      var fy = -k * (pt.y - gy);
      vys[i] = DAMP * (vys[i] + fy);
      dy += vys[i];
    }
    npts.push({
      x: pt.x + dx,
      y: pt.y + dy,
      edges: pt.edges,
    });
  }
  pts = npts;
}

function dist(a, b) {
  const dx = pts[b].x - pts[a].x;
  const dy = pts[b].y - pts[a].y;
  return Math.sqrt(dx*dx + dy*dy);
}

draw();

function tick() {
  adjust();
  draw();
}

function run(n) {
  tick();
  if (n > 0) {
    return requestAnimationFrame(run.bind(null, n-1));
  }
  console.log('done');
}

/*
let POINTS = false;
let xs = [];
let ys = [];
const lengths = [];
const growlengths = [];
const grown = [];
const r = .1;

const MAXLEN = .05;
const NORM_LENGTH = .03;

const CLOSE = .05;
const TOLERANCE = .0001;

function setup() {
  const ln = 50;
  for (var i=0; i<ln; i++) {
    var w = r + Math.random()*.05;
    xs.push(Math.cos(Math.PI*2/ln*i)*w);
    ys.push(Math.sin(Math.PI*2/ln*i)*w);
    grown.push(true);
    if (i > 0) {
      lengths.push(dist(i-1, i));
      growlengths.push(dist(i-1, i));
    }
  }
  growlengths.push(dist(ln-1, 0));
}

function drawPoints() {
  // ctx.clearRect(0, 0, half*2, half*2);
  const SZ = 2;
  for (var i=0; i<xs.length; i++) {
    ctx.fillStyle = grown[i] ? 'green' : 'blue';
    ctx.fillRect(half + xs[i]*half*mx - SZ, half + ys[i]*half*mx - SZ, SZ * 2, SZ * 2);
  }
}

function draw() {
  ctx.fillStyle = 'orange';
  ctx.fillRect(half + xs[0]*half*mx - 3, half + ys[0]*half*mx - 3, 6, 6);
  ctx.clearRect(0, 0, half*2, half*2);
  ctx.beginPath();
  ctx.moveTo(half + xs[0]*half*mx, half + ys[0]*half*mx);
  for (var i=0; i<xs.length; i++) {
    ctx.lineTo(half + xs[i]*half*mx, half + ys[i]*half*mx);
    // ctx.fillRect(half + xs[i]*half, half + ys[i]*half, 2, 2);
  }
  ctx.lineTo(half + xs[0]*half*mx, half + ys[0]*half*mx);
  ctx.lineWidth = 1;
  ctx.strokeStyle = 'red';
  ctx.stroke();
}

function dist(a, b) {
  const dx = xs[b] - xs[a];
  const dy = ys[b] - ys[a];
  return Math.sqrt(dx*dx + dy*dy);
}

function moveThings() {
  let nx = new Array(xs.length);
  let ny = new Array(ys.length);
  for (let i=0; i<xs.length; i++) {
    let dx = 0;
    let dy = 0;
    for (let j=0; j<xs.length; j++) {
      if (j === i) continue;
      if (j === i + 1 || j === i - 1) continue;
      if (j === 0 && i === xs.length - 1) continue;
      if (i === 0 && j === xs.length - 1) continue;
      let xx = xs[j] - xs[i];
      let yy = ys[j] - ys[i];
      let d = Math.sqrt(xx*xx+yy*yy);
      if (d < CLOSE) {
        let a = Math.atan2(yy, xx);
        dx -= Math.cos(a) * (CLOSE - d) / 10;
        dy -= Math.sin(a) * (CLOSE - d) / 10;
      }
    }

    var p = i === 0 ? xs.length - 1 : i - 1;
    var n = i === xs.length - 1 ? 0 : i + 1;

    {
      let xx = xs[p] - xs[i];
      let yy = ys[p] - ys[i];
      let d = Math.sqrt(xx*xx+yy*yy);
      if (Math.abs(d - growlengths[p]) > TOLERANCE) {
        let a = Math.atan2(yy, xx);
        dx -= Math.cos(a) * (lengths[p] - d) / 10;
        dy -= Math.sin(a) * (lengths[p] - d) / 10;
      }
    }
    {
      let xx = xs[i] - xs[n];
      let yy = ys[i] - ys[n];
      let d = Math.sqrt(xx*xx+yy*yy);
      lengths[i] = d;
      if (Math.abs(d - growlengths[i]) > TOLERANCE) {
        let a = Math.atan2(yy, xx);
        dx -= Math.cos(a) * (lengths[n] - d) / 10;
        dy -= Math.sin(a) * (lengths[n] - d) / 10;
      }
    }

    if (isNaN(dx) || isNaN(dy)) {
      debugger;
    }
    nx[i] = xs[i] + dx * .8;
    ny[i] = ys[i] + dy * .8;
  }
  xs = nx;
  ys = ny;
}

const ANGTOL = Math.PI * 2 / 3;

function expandThings() {
  let last = xs.length - 1;
  let theta = Math.atan2(ys[0] - ys[last], xs[0] - xs[last]);
  for (let i=1; i<xs.length; i++) {
    let xx = xs[i] - xs[i - 1];
    let yy = ys[i] - ys[i - 1];

    let ntheta = Math.atan2(yy, xx);
    const dtheta = Math.abs(Math.PI - Math.abs(theta - ntheta));
    if (dtheta < ANGTOL) {// &&
        //Math.abs(lengths[i - 1] - growlengths[i - 1]) <= TOLERANCE) {
      // console.log(theta / Math.PI * 180, ntheta / Math.PI * 180);
      growlengths[i - 1] = growlengths[i - 1] + .001;
      grown[i] = true;
    } else {
      grown[i] = false;
    }
    const x = half + xs[i - 1]*half*mx
    const y = half + ys[i - 1]*half*mx
    const a1 = parseInt(dtheta / Math.PI * 180);
    const a2 = parseInt(ANGTOL / Math.PI * 180);
    // ctx.fillText(a1 + ':' + a2, x, y);
    theta = ntheta;
  }
}

function splitThings() {
  for (let i=0; i<lengths.length - 1; i++) {
    if (lengths[i] > MAXLEN) {
      lengths[i] /= 2;
      lengths.splice(i, 0, lengths[i]);
      growlengths[i] = NORM_LENGTH;
      growlengths.slice(i + 1, 0, NORM_LENGTH);
      grown.splice(i, 0, true);
      xs.splice(i + 1, 0, xs[i] + (xs[i + 1] - xs[i]) / 2);
      ys.splice(i + 1, 0, ys[i] + (ys[i + 1] - ys[i]) / 2);
    }
  }
}

function tick() {
  moveThings();
  draw();
  drawPoints();
  expandThings();
  splitThings();
}

function run(n) {
  tick();
  if (n > 0) {
    return requestAnimationFrame(run.bind(null, n-1));
  }
  console.log('done');
}

setup();
tick();
*/
