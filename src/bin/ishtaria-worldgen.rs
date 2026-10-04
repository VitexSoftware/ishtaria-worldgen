//! Render the six cube faces of a planet as a PGM heightmap strip.
//!
//! Usage: ishtaria-worldgen [seed] [face_size] > planet.pgm

use ishtaria_worldgen::{Face, Terrain};
use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let seed = args
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(Terrain::default().seed);
    let size: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(256);
    let terrain = Terrain {
        seed,
        ..Terrain::default()
    };

    let mut out = BufWriter::new(io::stdout().lock());
    writeln!(out, "P5\n{} {}\n255", size * 6, size)?;
    let mut row = vec![0u8; size * 6];
    for j in 0..size {
        for (f, face) in Face::ALL.iter().enumerate() {
            for i in 0..size {
                let u = (i as f64 + 0.5) / size as f64 * 2.0 - 1.0;
                let v = 1.0 - (j as f64 + 0.5) / size as f64 * 2.0;
                let e = terrain.elevation(face.sphere_point(u, v)) / terrain.amplitude_m;
                row[f * size + i] = ((e * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;
            }
        }
        out.write_all(&row)?;
    }
    out.flush()
}
