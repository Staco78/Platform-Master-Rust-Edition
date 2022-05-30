use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::config::CHUNK_SIZE;

struct Noise {
    octaves: u8,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
    rnd: [u8; 512],
}

impl Noise {
    pub fn new(seed: u64, octaves: u8, frequency: f64, persistence: f64, lacunarity: f64) -> Self {
        let mut n = Noise {
            octaves,
            frequency,
            persistence,
            lacunarity,
            rnd: [0u8; 512],
        };
        ChaCha8Rng::seed_from_u64(seed).fill(&mut n.rnd);
        n
    }

    fn grad(hash: u8, x: f64) -> f64 {
        let h = hash & 15;
        let mut grad = 1. + (h & 7) as f64;
        if h & 8 != 0 {
            grad *= -1.;
        }
        grad * x
    }

    // from https://github.com/WardBenjamin/SimplexNoise/blob/2afa9a63483562cc4c0a95bbfa6b183fc256a790/SimplexNoise/Noise.cs
    fn _noise(&self, x: f64) -> f64 {
        let i0: i32 = x.floor() as i32;
        let i1: i32 = i0 + 1;
        let x0: f64 = x - i0 as f64;
        let x1: f64 = x0 - 1.;

        let mut t0: f64 = 1. - x0 * x0;
        t0 *= t0;
        let n0: f64 = t0 * t0 * Noise::grad(self.rnd[(i0 & 255) as usize], x0);

        let mut t1: f64 = 1. - x1 * x1;
        t1 *= t1;
        let n1: f64 = t1 * t1 * Noise::grad(self.rnd[(i1 & 255) as usize], x1);

        // The maximum value of this noise is 8*(3/4)^4 = 2.53125
        // A factor of 0.395 scales to fit exactly within [-1,1]
        0.395 * (n0 + n1)
    }

    // return normalized in [-1, 1]
    pub fn noise(&self, mut x: f64) -> f64 {
        x *= self.frequency;

        let mut sum: f64 = 0.;
        let mut p: f64 = 1.;
        let mut max = 0.;
        for _ in 0..self.octaves {
            sum += p * self._noise(x);
            max += p;
            x *= self.lacunarity;
            p *= self.persistence;
        }
        sum / max
    }
}

pub struct NoiseContext {
    height: Noise,
    exps: Noise,
}

impl NoiseContext {
    pub fn new(seed: u64) -> NoiseContext {
        NoiseContext {
            height: Noise::new(seed, 6, 0.003, 0.5, 2.0),
            exps: Noise::new(seed + 1, 1, 0.0009, 0.5, 2.0),
        }
    }

    pub fn noise(&self, x: i32) -> (Vec<u32>, u32) {
        let mut out = Vec::with_capacity(CHUNK_SIZE as usize);
        let mut max = 0;
        for i in 0..CHUNK_SIZE {
            let e = self.exps.noise(x as f64 * CHUNK_SIZE as f64 + i as f64);
            let h = self.height.noise(x as f64 * CHUNK_SIZE as f64 + i as f64);
            out.push(((h + 1.).powf((e + 1.) / 1.3) * 100.) as u32);
            if out[i as usize] > max {
                max = out[i as usize];
            }
        }
        (out, max as u32)
    }
}
