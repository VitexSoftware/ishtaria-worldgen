//! Deterministic 3D value noise with fractal octaves.
//!
//! Sampling on the sphere surface (not in face UV space) avoids seams at
//! cube edges. Placeholder quality – to be replaced by a proper
//! gradient/simplex noise with erosion passes.

use crate::cubesphere::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Terrain {
    pub seed: u64,
    pub octaves: u32,
    /// Peak amplitude in metres.
    pub amplitude_m: f64,
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain {
            seed: 0x1517_A21A,
            octaves: 8,
            amplitude_m: 8_000.0,
        }
    }
}

fn hash(seed: u64, x: i64, y: i64, z: i64) -> f64 {
    // SplitMix64-style mixing; stable across platforms and releases.
    let mut h = seed
        ^ (x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
        ^ (z as u64).wrapping_mul(0x1656_67B1_9E37_79F9);
    h = (h ^ (h >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h = (h ^ (h >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    h ^= h >> 31;
    (h >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
}

fn smooth(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

fn value_noise(seed: u64, p: Vec3) -> f64 {
    let (xi, yi, zi) = (p.x.floor() as i64, p.y.floor() as i64, p.z.floor() as i64);
    let (tx, ty, tz) = (
        smooth(p.x - xi as f64),
        smooth(p.y - yi as f64),
        smooth(p.z - zi as f64),
    );
    let c = |dx, dy, dz| hash(seed, xi + dx, yi + dy, zi + dz);
    let x00 = lerp(c(0, 0, 0), c(1, 0, 0), tx);
    let x10 = lerp(c(0, 1, 0), c(1, 1, 0), tx);
    let x01 = lerp(c(0, 0, 1), c(1, 0, 1), tx);
    let x11 = lerp(c(0, 1, 1), c(1, 1, 1), tx);
    lerp(lerp(x00, x10, ty), lerp(x01, x11, ty), tz)
}

impl Terrain {
    /// Elevation in metres for a point on the unit sphere.
    pub fn elevation(&self, unit: Vec3) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;
        let mut freq = 2.0;
        let mut norm = 0.0;
        for o in 0..self.octaves {
            sum += amp * value_noise(self.seed.wrapping_add(o as u64), unit.scale(freq));
            norm += amp;
            amp *= 0.5;
            freq *= 2.0;
        }
        sum / norm * self.amplitude_m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Face;

    #[test]
    fn deterministic() {
        let t = Terrain::default();
        let p = Face::PosZ.sphere_point(0.1, 0.2);
        assert_eq!(t.elevation(p), t.elevation(p));
    }

    #[test]
    fn seed_changes_terrain() {
        let p = Face::PosZ.sphere_point(0.1, 0.2);
        let a = Terrain {
            seed: 1,
            ..Terrain::default()
        }
        .elevation(p);
        let b = Terrain {
            seed: 2,
            ..Terrain::default()
        }
        .elevation(p);
        assert_ne!(a, b);
    }
}
