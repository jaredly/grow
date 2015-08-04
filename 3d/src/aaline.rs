extern crate image;
use image::{ImageBuffer, Rgba};
use image::Pixel;
use na::Pnt3;

const TABLE: [f32; 32] = [2.0965317582885168e-209, 3.6938830684872561e-196, 2.394254760948756e-183, 5.7090401058641008e-171, 5.0079657066127921e-159, 1.6160884138202511e-147, 1.918555668934785e-136, 8.3789425338193693e-126, 1.3461998461573202e-115, 7.9567438919514001e-106, 1.7300822096825899e-96, 1.3838965267367376e-87, 4.0723586257611754e-79, 4.4085313314632264e-71, 1.7556880978548265e-63, 2.5722093726424148e-56, 1.3863432936411706e-49, 2.7487850079102147e-43, 2.0050087819616541e-37, 5.3801861600211382e-32, 5.3110922496790952e-27, 1.9287498479639178e-22, 2.576757109154981e-18, 1.2664165549094176e-14, 2.289734845645553e-11, 1.5229979744712629e-08, 3.7266531720786709e-06, 0.00033546262790251185, 0.011108996538242306, 0.1353352832366127, 0.60653065971263342, 1.0];

pub trait Color {
    fn rgba(&self) -> Rgba<u8>;
    fn with_alpha(&self, alpha: f32) -> Rgba<u8>;
}

impl Color for Pnt3<f32> {
    fn rgba(&self) -> Rgba<u8> {
        let Pnt3{x, y, z} = *self;
        Rgba([(x * 255.0) as u8, (y * 255.0) as u8, (z * 255.0) as u8, 255])
    }

    fn with_alpha(&self, alpha: f32) -> Rgba<u8> {
        let Pnt3{x, y, z} = *self;
        Rgba([(x * 255.0) as u8, (y * 255.0) as u8, (z * 255.0) as u8, (alpha * 255.0) as u8])
    }
}

impl Color for (u8, u8, u8) {
    fn rgba(&self) -> Rgba<u8> {
        let (r, g, b) = *self;
        Rgba([r, g, b, 255])
    }

    fn with_alpha(&self, alpha: f32) -> Rgba<u8> {
        let (r, g, b) = *self;
        Rgba([r, g, b, (alpha * 255.0) as u8])
    }
}

impl Color for (u8, u8, u8, u8) {
    fn rgba(&self) -> Rgba<u8> {
        let (r, g, b, a) = *self;
        Rgba([r, g, b, a])
    }

    fn with_alpha(&self, alpha: f32) -> Rgba<u8> {
        let (r, g, b, a) = *self;
        let combined = (a as f32 * alpha) as u8;
        Rgba([r, g, b, combined])
    }
}

pub trait DrawLine {
    fn draw_line(&mut self, x: f32, y: f32, x2: f32, y2: f32, width: f32, color: &Color);
}

impl DrawLine for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn draw_line(&mut self, x: f32, y: f32, x2: f32, y2: f32, width: f32, color: &Color) {
        /*
        let dx = x1 - x0
        let dy = y1 - y0
        let theta = dy.atan2(dx);
        let half = width as f32 / 2.0;
        let m = dy / dx;
        let im = 1 / m;
        let bottomx = cos(theta + PI/2) * half
        let bottomy = sin(theta + PI/2) * half
        topx = cos(theta - PI/2) * half
        topy = sin(theta - PI/2) * half
        */

        rect(self, color, x, y, x2, y2, width);

        // draw_line(self, color, x, y, x2, y2); // , width);
    }
}

fn lookup(val: f32) -> f32 {
    // println!("Lookup {}", val);
    if val > 1.0 {
        return 1.0;
    }
    if val < 0.0 {
        return 0.0;
    }
    return val;
    // return if val > 0.5 { 1.0 } else { val }; 
    // return TABLE[(val * 31.0) as usize];
}

// http://http.developer.nvidia.com/GPUGems2/gpugems2_chapter22.html
fn rect(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: &Color, x0: f32, y0: f32, x1: f32, y1: f32, width: f32) {
    let r = 4.0;
    let w = width;

    let k = 2.0 / ((2.0 * r + w) * ((x0 - x1).powi(2) - (y0 - y1).powi(2)).abs().sqrt());
    let ky0 = k * (y0 - y1);
    let ky1 = -ky0;
    let kx0 = k * (x0 - x1);
    let kx1 = -kx0;
    let kx0y = k * (x0 * y1 - x1 * y0);
    let e0 = (ky0, kx1, 1.0 + kx0y);
    let e1 = (kx1, ky1, 1.0 + k * (x0.powi(2) + y0.powi(2) - x0 * x1 - y0 * y1));
    let e2 = (ky1, kx0, 1.0 - kx0y);
    let e3 = (kx0, ky0, 1.0 + k * (x1.powi(2) + y1.powi(2) - x0 * x1 - y0 * y1));

    let iw = img.width();
    let ih = img.height();

    for xi in (x0.min(x1) - w).max(0.0) as u32 .. (x0.max(x1) + w).min(iw as f32) as u32 {
        let x = xi as f32;
        for yi in (y0.min(y1) - w).max(0.0) as u32 .. (y0.max(y1) + w).min(ih as f32) as u32 {
            let y = yi as f32;
            let d0 = x * e0.0 + y * e0.1 + e0.2;
            if d0 <= 0.0 {continue}
            let d1 = x * e1.0 + y * e1.1 + e1.2;
            if d1 <= 0.0 {continue}
            let d2 = x * e2.0 + y * e2.1 + e2.2;
            if d2 <= 0.0 {continue}
            let d3 = x * e3.0 + y * e3.1 + e3.2;
            if d3 <= 0.0 {continue}
            let alpha = lookup(d0.min(d2).max(0.0)) * lookup(d1.min(d3).max(0.0));
            plot(img, color, x, y, alpha);
        }
    }
}

/*
fn iplot(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: &Color, x: u32, y: u32) {
    img.put_pixel(x, y, color.rgba());
}
*/

fn plot(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: &Color, x: f32, y: f32, c: f32) {
    let mut pixel = color.with_alpha(c);
    pixel.blend(img.get_pixel(x as u32, y as u32));
    img.put_pixel(x as u32, y as u32, pixel);
}

/*
fn fastLine(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: &Color, x0: u32, y0: u32, x1: u32, y1: u32) {
  let dx = x1 - x0;
  let dy = y1 - y0;

  let mut D = 2 * dy - dx;
  iplot(img, color, x0, y0);
  let mut y = y0;

  for x in x0 + 1 .. x1 {
    if D > 0 {
      y = y+1;
      iplot(img, color, x, y);
      D = D + (2*dy-2*dx);
    } else {
      iplot(img, color, x, y);
      D = D + (2*dy);
    }
  }
}
*/

// fractional part of x
fn fpart(x: f32) -> f32 {
    if x < 0.0 {
        1.0 - (x - x.floor())
    } else {
        x - x.floor()
    }
}

fn rfpart(x: f32) -> f32 {
    1.0 - fpart(x)
}

fn draw_line(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: &Color, x0: f32, y0: f32, x1: f32, y1: f32) {
    let steep = (y1 - y0).abs() > (x1 - x0).abs();

    let (x0, y0, x1, y1) = 
        if steep {
            (y0, x0, y1, x1)
        } else {
            (x0, y0, x1, y1)
        };
    let (x0, x1, y0, y1) = 
        if x0 > x1 {
            (x1, x0, y1, y0)
        } else {
            (x0, x1, y0, y1)
        };

    let dx = x1 - x0;
    let dy = y1 - y0;
    let gradient = dy / dx;

    // handle first endpoint
    let mut xend = x0.round();
    let mut yend = y0 + gradient * (xend - x0);
    let mut xgap = rfpart(x0 + 0.5);
    let xpxl1 = xend; // this will be used in the main loop
    let ypxl1 = yend.floor();
    if steep {
        plot(img, color, ypxl1,     xpxl1, rfpart(yend) * xgap);
        plot(img, color, ypxl1+1.0, xpxl1,  fpart(yend) * xgap);
    } else {
        plot(img, color, xpxl1, ypxl1    , rfpart(yend) * xgap);
        plot(img, color, xpxl1, ypxl1+1.0,  fpart(yend) * xgap);
    }
    let mut intery = yend + gradient; // first y-intersection for the main loop

    // handle second endpoint
    xend = x1.round();
    yend = y1 + gradient * (xend - x1);
    xgap = fpart(x1 + 0.5);
    let xpxl2 = xend; //this will be used in the main loop
    let ypxl2 = yend.floor();
    if steep {
        plot(img, color, ypxl2  , xpxl2, rfpart(yend) * xgap);
        plot(img, color, ypxl2+1.0, xpxl2,  fpart(yend) * xgap);
    } else {
        plot(img, color, xpxl2, ypxl2,  rfpart(yend) * xgap);
        plot(img, color, xpxl2, ypxl2+1.0, fpart(yend) * xgap);
    }

    // main loop
    for x in xpxl1 as u32 + 1 .. xpxl2 as u32 - 1 {
        if steep {
            plot(img, color, intery, x as f32, rfpart(intery));
            plot(img, color, intery + 1.0, x as f32,  fpart(intery));
        } else {
            plot(img, color, x as f32, intery,  rfpart(intery));
            plot(img, color, x as f32, intery + 1.0, fpart(intery));
        }
        intery = intery + gradient;
    }
}
