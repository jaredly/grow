'use strict';

var canv = document.querySelector('#canvas');
var ctx = canv.getContext('2d');

const half = 400;
const mx = 4;
const TOLERANCE = .001;
const m = 10;
const DAMP = 0.90;
const k = 0.01;

let x = [];
let y = [];

let tx = [];
let ty = [];

let vx = [];
let vy = [];

for (var i=0; i<10; i++) {
  x.push(Math.random() - .5);
  y.push(Math.random() - .5);
  tx.push(Math.random() - .5);
  ty.push(Math.random() - .5);
  vx.push(0);
  vy.push(0);
}

function draw() {
  ctx.clearRect(0, 0, half*2, half*2);
  for (var i=0; i<x.length; i++) {
    ctx.fillRect(half + x[i] * half - 4, half + y[i] * half - 4, 8, 8);
  }
}

function step() {
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
