//! Cube-sphere mapping.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn scale(self, k: f64) -> Vec3 {
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl Face {
    pub const ALL: [Face; 6] = [
        Face::PosX,
        Face::NegX,
        Face::PosY,
        Face::NegY,
        Face::PosZ,
        Face::NegZ,
    ];

    /// Point on the unit cube for face coordinates `u, v` in `[-1, 1]`.
    pub fn cube_point(self, u: f64, v: f64) -> Vec3 {
        match self {
            Face::PosX => Vec3 {
                x: 1.0,
                y: v,
                z: -u,
            },
            Face::NegX => Vec3 {
                x: -1.0,
                y: v,
                z: u,
            },
            Face::PosY => Vec3 {
                x: u,
                y: 1.0,
                z: -v,
            },
            Face::NegY => Vec3 {
                x: u,
                y: -1.0,
                z: v,
            },
            Face::PosZ => Vec3 { x: u, y: v, z: 1.0 },
            Face::NegZ => Vec3 {
                x: -u,
                y: v,
                z: -1.0,
            },
        }
    }

    /// Point on the unit sphere. Uses the "spherified cube" mapping, which
    /// distributes area far more evenly than plain normalisation.
    pub fn sphere_point(self, u: f64, v: f64) -> Vec3 {
        let p = self.cube_point(u, v);
        let (x2, y2, z2) = (p.x * p.x, p.y * p.y, p.z * p.z);
        Vec3 {
            x: p.x * (1.0 - y2 / 2.0 - z2 / 2.0 + y2 * z2 / 3.0).sqrt(),
            y: p.y * (1.0 - z2 / 2.0 - x2 / 2.0 + z2 * x2 / 3.0).sqrt(),
            z: p.z * (1.0 - x2 / 2.0 - y2 / 2.0 + x2 * y2 / 3.0).sqrt(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_points_are_unit_length() {
        for face in Face::ALL {
            for i in 0..=10 {
                for j in 0..=10 {
                    let u = -1.0 + i as f64 * 0.2;
                    let v = -1.0 + j as f64 * 0.2;
                    let len = face.sphere_point(u, v).length();
                    assert!((len - 1.0).abs() < 1e-12, "{face:?} {u} {v}: {len}");
                }
            }
        }
    }

    #[test]
    fn face_edges_meet() {
        // Right edge of +Z equals left edge of +X.
        let a = Face::PosZ.sphere_point(1.0, 0.3);
        let b = Face::PosX.sphere_point(-1.0, 0.3);
        assert!(
            (a.x - b.x).abs() < 1e-12 && (a.y - b.y).abs() < 1e-12 && (a.z - b.z).abs() < 1e-12
        );
    }
}
