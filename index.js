'use strict';

var canv = document.querySelector('#canvas');
var ctx = canv.getContext('2d');

const half = 400;
const mx = 2;

let POINTS = false;
let xs = [];
let ys = [];
const lengths = [];
const growlengths = [];
const grown = [];
const r = .1;

const MAXLEN = .05;
const NORM_LENGTH = .03;

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

function drawPointsTo(n) {
  ctx.clearRect(0, 0, half*2, half*2);
  ctx.fillStyle = 'red';
  for (var i=0; i<n; i++) {
    ctx.fillRect(half + xs[i]*half, half + ys[i]*half, 2, 2);
  }
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

function drawTo(n) {
  ctx.clearRect(0, 0, half*2, half*2);
  ctx.beginPath();
  ctx.moveTo(half + xs[0]*half, half + ys[0]*half);
  for (var i=0; i<n; i++) {
    ctx.lineTo(half + xs[i]*half, half + ys[i]*half);
    // ctx.fillRect(half + xs[i]*half, half + ys[i]*half, 2, 2);
  }
  ctx.lineTo(half + xs[0]*half, half + ys[0]*half);
  ctx.lineWidth = 1;
  ctx.strokeStyle = 'red';
  ctx.stroke();
}

function dist(a, b) {
  const dx = xs[b] - xs[a];
  const dy = ys[b] - ys[a];
  return Math.sqrt(dx*dx + dy*dy);
}

function newPt(a) {
  let b;
  if (a === xs.length - 1) {
    b = 0;
  } else {
    b = a + 1;
  }
  const dx = xs[b] - xs[a];
  const dy = ys[b] - ys[a];
  xs.splice(b, 0, xs[a] + dx/2);
  ys.splice(b, 0, ys[a] + dy/2);
}

function maybeAdd() {
  // let biggest = xs.length - 1;
  // let blen = dist(xs.length - 1, 0);
  for (let i=0; i<xs.length - 1; i++) {
    let len = dist(i, i + 1);
    if (len > .06) {
      newPt(i, len);
    }
    /*
    if (len > blen) {
      blen = len;
      biggest = i;
    }
    console.log(blen);
    */
  }
}

function push(a, b) {
  const dx = xs[b] > xs[a] ? .001 : -.001;
  const dy = ys[b] > ys[a] ? .001 : -.001;
  // const dx = .002 * (1 - (xs[b] - xs[a]) / .05);
  // const dy = .002 * (1 - (ys[b] - ys[a]) / .05);
  xs[a] -= dx;
  ys[a] -= dy;
  xs[b] += dx;
  ys[b] += dy;
}

function moveAway() {
  for (let i=0; i<xs.length; i++) {
    for (let j=0; j<xs.length; j++) {
      if (j === i) continue;
      const d = dist(i, j);
      if (d < .05) {
        push(i, j);
      }
    }
  }
}

const CLOSE = .05;
const TOLERANCE = .0001;

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
      let xx = xs[n] - xs[i];
      let yy = ys[n] - ys[i];
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
    nx[i] = xs[i] + dx;
    ny[i] = ys[i] + dy;
  }
  xs = nx;
  ys = ny;
}

const ANGTOL = Math.PI * 3 / 2;

function expandThings() {
  let last = xs.length - 1;
  let theta = Math.atan2(ys[0] - ys[last], xs[0] - xs[last]);
  for (let i=1; i<xs.length; i++) {
    let ntheta = Math.atan2(ys[i] - ys[i-1], xs[i] - xs[i-1]);
    if (Math.abs(theta - ntheta) > ANGTOL &&
        Math.abs(lengths[i - 1] - growlengths[i - 1]) <= TOLERANCE) {
      // console.log(theta / Math.PI * 180, ntheta / Math.PI * 180);
      growlengths[i - 1] = growlengths[i - 1] + .0001;
      grown[i] = true;
    } else {
      grown[i] = false;
    }
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
  expandThings();
  splitThings();
  // maybeAdd();
  // moveAway();
  draw();
  drawPoints();
  // POINTS ? drawPoints() : draw();
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
