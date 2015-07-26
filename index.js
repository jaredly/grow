'use strict';

var canv = document.querySelector('#canvas');
var ctx = canv.getContext('2d');

const half = 400;
const mx = 4;

const TOLERANCE = .001;
const m = 10;

const DAMP = 0.90;

let x = 0;
let y = 0;

let tx = .5;
let ty = .2;

let vx = 0;
let vy = 0;

const k = 0.01;

function draw() {
  ctx.clearRect(0, 0, half*2, half*2);
  ctx.fillRect(half + x * half - 4, half + y * half - 4, 8, 8);
}

function step() {
  let dx = (x - tx)
  let dy = (y - ty)
  vx = DAMP * (vx + -k * dx);
  vy = DAMP * (vy + -k * dy);
  x += vx;
  y += vy;
  if (Math.abs(dx) < .0001 && Math.abs(dy) < .0001 && Math.abs(vx) < .0001 && Math.abs(vy) < .0001) {
    tx = Math.random() - .5;
    ty = Math.random() - .5;
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
