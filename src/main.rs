mod kimg;
mod kmath;
mod hydrasim;

use hydrasim::*;
use kimg::*;
use kmath::*;

fn main() {
    let w = 400;
    let h = 400;

    let mut hs = Hydrasim::default();
    hs.w = w;
    hs.h = h;
    hs.gen();
    for i in 0..2000000 {
        hs.do_drop(i)
    }
    hs.colour();

    // i guess we just iterate over backwards and paint
    let max_height = 400;


    let mut buf = kimg::ImageBuffer::new(w, h + max_height);

    for i in 0..w {
        for j in 0..h {
            let h = hs.height[j*w + i];
            let h = h * 1.0;
            let h = h.min(1.0);
            // let h = (-(h.ln())/9.0).min(1.0);
            let c = Vec4::new(0.0, 0.2, 0.0, 1.0).lerp(Vec4::new(0.7, 0.6, 0.4, 1.0), h*4.0);
            let h = (h * max_height as f32) as usize;

            // let c = hs.colour[j*w+i];
            let c = ((c.x * 255.0) as u8, (c.y * 255.0) as u8, (c.z * 255.0) as u8);

            for hi in 0..h {
                buf.set_px(i, max_height - hi + j, c)
            }
        }
    }
    let j = h - 1;
    for i in 0..w {
        let h = hs.height[j*w + i];
        let h = h * 1.0;
        let h = h.min(1.0);
        let h = (h * max_height as f32) as usize;
        for hi in 0..h {
            buf.set_px(i, max_height - hi + j, (0, 0, 0));
        }
    }

    buf.dump_to_file("out.png");

    let imw = w * 2;
    let imh = h * 2;
    let mut buf = kimg::ImageBuffer::new(imw, imh);
    for i in 0..imw {
        for j in 0..imh {
            let i = i as i32;
            let j = j as i32;
            let imw = imw as i32;
            let imh = imh as i32;
            let hmw = hs.w as i32;
            let hmh = hs.h as i32;

            // ray starting point as f of i, j, s.t. i/2, j/2 is also i/2 j/2 of image
            let d = 400;
            // plane s.t. x+y+z = -d
            let ii = i - w as i32;
            let jj = j - h as i32;
            let rx = -jj - ii + imw/2 + hmw/2 - d;
            let ry = -jj + ii + imh/2 + hmh/2 - d;
            let rz = d/2 - rx - ry;
            let mut ray = (rx, ry, rz);
            if i == imw/2 && j == imh/2 {
                println!("{:?}", ray);
            }
            let max_iters = 1000;
            let mut it = 0;
            while it < max_iters {
                it += 1;
                ray.0 += 1;
                ray.1 += 1;
                ray.2 -= 1;
                if i == imw/2 && j == imh/2 {
                    println!("{:?}", ray);
                }

                // make sure rays arent culled that are outside, nah that should be fine
                if ray.0 < 0 || ray.1 < 0 || ray.0 >= hmw as i32 || ray.1 >= hmh as i32 {
                    continue;
                }
                let h = hs.height[(ray.1 as usize)*w + (ray.0 as usize)] * 400.0;
                let hu = h as usize;
                if hu >= ray.2 as usize {
                    let c = Vec4::new(0.0, 0.2, 0.0, 1.0).lerp(Vec4::new(0.7, 0.6, 0.4, 1.0), h/100.0);
                    let c = ((c.x * 255.0) as u8, (c.y * 255.0) as u8, (c.z * 255.0) as u8);

                    buf.set_px(i as usize, (imh - j) as usize, c);
                    break;
                }
            }
            // buf.set_px(i as usize, j as usize, (255, 0, 255))
        }
    }

    buf.dump_to_file("iso.png");


    // for isometric
    // output image dimensions need to be what?
    // or a given x y z
    // a given pixel on the output image is what value from the input image


    // if z is 0
    // diamond

    // probably start by specifying an input format

}

fn xyz_to_wh_iso(x: i32, y: i32, z: i32) -> (i32, i32) {
    let x = x - y - z;
    let y = x + y - z;

    // maybe fractions

    (x, y)
}


fn ray_start(x: i32, y: i32, imw: i32, imh: i32, hmw: i32, hmh: i32) -> (i32, i32, i32) {
    // x + y - z = d;
    // x = x(i,j);   // decreasing with i, decreasing with j
    // y = y(i, j);   // decreasing with i, decreasing with j
    // z = x = y + d;
    (0,0,0)
}

#[test]
fn test_ray_start() {
    let imw = 100;
    let imh = 100;


}