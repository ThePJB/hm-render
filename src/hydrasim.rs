use crate::kmath::*;

fn fracnoise(seed: u32, x: f32, y: f32) -> f32 {
    1.000 * noise2d(x, y, seed) +
    0.500 * noise2d(x*2.0, y*2.0, seed*1238715) +
    0.250 * noise2d(x*4.0, y*4.0, seed*9148167) +
    0.125 * noise2d(x*8.0, y*8.0, seed*2442347) /
    1.875
}

fn hh(seed: u32, x: f32, y: f32) -> f32 {
    fracnoise(seed, 2.0*x, 2.0*y) * 0.1
}

fn hn(seed: u32, x: f32, y: f32) -> (f32, Vec3) {
    let d = 0.01;
    let h = hh(seed, x, y);
    let hgx = hh(seed, x + d, y);
    let hgy = hh(seed, x, y + d);
    let vx = Vec3::new(d, hgx - h, 0.0);
    let vz = Vec3::new(0.0, hgy - h, d);

    let norm = vz.cross(vx);

    (h, norm)
}

pub struct Hydrasim {
    pub stale: bool,
    pub w: usize,
    pub h: usize,
    pub height: Vec<f32>,
    pub colour: Vec<Vec4>,
    pub normal: Vec<Vec3>,
    pub seed: u32,
}

impl Hydrasim {
    pub fn gen(&mut self) {
        self.height.resize(self.w * self.h, 0.0);
        self.normal.resize(self.w * self.h, Vec3::new(0.0, 0.0, 0.0));
        self.colour.resize(self.w * self.h, Vec4::new(0.0, 0.0, 0.0, 1.0));
        for i in 0..self.w {
            for j in 0..self.h {
                let nx = 4.0 * i as f32 / self.w as f32;
                let ny = 4.0 * j as f32 / self.h as f32;

                let (h, norm) = hn(self.seed, nx, ny);
                let norm = norm.normalize();

                let slopeyness = remap(norm.y, 0.95, 1.0, 0.0, 1.0);





                let c1 = Vec4::new(1.0, 0.0, 0.0, 1.0);
                let c2 = Vec4::new(0.0, 1.0, 0.0, 1.0);


                self.colour[j*self.w + i] = c1.lerp(c2, slopeyness);

                // self.colour[j*self.w + i] = Vec4::new((norm.x + 1.0) / 2.0, (norm.y + 1.0)/2.0, (norm.z + 1.0) / 2.0, 1.0);
                self.normal[j*self.w + i] = norm;
                self.height[j*self.w + i] = h as f32;
            }
        }
        self.stale = true;
    }

    pub fn colour(&mut self) {
        for i in 0..self.w {
            for j in 0..self.h {
                let dx = if i > 0 {
                    self.height[self.w * j + i - 1] - self.height[self.w * j + i]
                } else {
                    0.0
                };
                let dy = if j > 0 {
                    self.height[self.w * (j-1) + i] - self.height[self.w * j + i]
                } else {
                    0.0
                };

                let gmag = dx*dx + dy*dy;


                let green = Vec4::new(0.0, 1.0, 0.0, 1.0);
                let grey = Vec4::new(0.6, 0.6, 0.6, 1.0);
                let c = green.lerp(grey, (1000000.0 * gmag).min(1.0));

                let seed = self.seed + i as u32 * 124712547 + j as u32 * 12341547;
                let noise_amount = 0.1;

                let mut r = c.x;
                let tr = krand(seed);
                r += tr * noise_amount;
                
                let mut g = c.y;
                let tg = krand(seed * 1598157);
                g += tg * noise_amount;

                let mut b = c.z;
                let tbb = krand(seed * 139857157);
                b += tbb * noise_amount;

                let max = r.max(g).max(b).max(1.0);
                r /= max;
                g /= max;
                b /= max;

                self.colour[j*self.w + i] = Vec4::new(r, g, b, 1.0);        
            }
        }
    }

    pub fn minimums(&self) {
        for i in 1..self.w-1 {
            for j in 1..self.h-1 {
                let h = self.height[j * self.w + i];
                let hl = self.height[j * self.w + (i+1)];
                let hr = self.height[j * self.w + (i-1)];
                let hu = self.height[(j-1) * self.w + i];
                let hd = self.height[(j+1) * self.w + i];

                if h < hl && h < hr && h < hu && h < hd {
                    println!("local minimum at {},{}", i,j);
                }
            }
        }
    }

    pub fn do_drop(&mut self, seed: u32) {
        // println!("new drop");
        let mut x = khash(seed) as usize % self.w;
        let mut y = khash(123124 + seed * 124717) as usize % self.h;

        let mut sediment = 0.0;

        let max_iters = 1000;

        'OUTER:
        for iter in 0..max_iters {
            // println!("iter {} x {} y {}", iter, x, y);
            // r d l u
            let idx = self.w * y + x;
            let h = self.height[idx];
            let edge = [x == self.w-1, y == self.h - 1, x == 0, y == 0];
            // println!("edge {:?}", edge);
            let xi = [x as i32 + 1, x as i32, x as i32 - 1, x as i32];
            // println!("xi {:?}", xi);
            let yi = [y as i32, y as i32 + 1, y as i32, y as i32 - 1];
            // println!("yi {:?}", yi);
            let mut w = [0.0f32; 4];
            let mut dh = [0.0f32; 4];
            for i in 0..4 {
                if edge[i] {
                    w[i] = 1.0;
                } else {
                    let other_h = self.height[self.w * yi[i] as usize + xi[i] as usize];
                    dh[i] = h - other_h;
                    // wi is a fn of dh such that if dh == 0, wi = 1.0, if dh negative, wi = 0, if all wi = 0, local minimum (stop)
                    if dh[i] < 0.0 {
                        w[i] = 0.0;
                    } else {
                        w[i] = dh[i] + 1.0;
                    }
                }
            }
            // println!("wi {:?}", w);
            // println!("dh {:?}", dh);

            // if x == 3 && y == 95 {
            //     panic!("asdf")
            // }

            if w[0] <= 0.0 && w[1] <= 0.0 && w[2] <= 0.0 && w[3] <= 0.0 {
                // local minimum
                // println!("minimum");
                self.height[idx] += sediment;
                return;
            }

            // select particle direction
            let sw = w[0] + w[1] + w[2] + w[3];
            let cumw = [w[0], w[0] + w[1], w[0] + w[1] + w[2], w[0] + w[1] + w[2] + w[3]];
            let wchoice = krand(seed + 12381723 * iter) as f32 * sw;
            for i in 0..4 {
                if wchoice <= cumw[i] {
                    if edge[i] {
                        // println!("off edge");
                        return;
                    }
                    let sediment_capacity = dh[i];
                    let scapd = sediment_capacity - sediment;
                    let take_amount = 0.01 * scapd;
                    self.height[idx] -= take_amount;
                    sediment += take_amount;
                    x = xi[i] as usize;
                    y = yi[i] as usize;
                    continue 'OUTER;
                }
            }
            println!("cumw: {:?}, wchoice: {:?}, sw: {:?}", cumw, wchoice, sw);
            panic!("unreachable");
        }
        println!("max iters reached");
    }
}

impl Default for Hydrasim {
    fn default() -> Self {
        let w = 400;
        let h = 400;

        let mut t = Hydrasim {
            stale: true,
            w,
            h,
            seed: 0,
            colour: vec![],
            height: vec![],
            normal: vec![],
        };
        t.gen();
        t.minimums();
        t
    }
}
