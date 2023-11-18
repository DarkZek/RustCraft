
use noise::{NoiseFn, OpenSimplex};


pub struct SimplexNoise {
    simplex: OpenSimplex,
    scale: f64,
}

impl SimplexNoise {
    pub fn new(seed: u32) -> SimplexNoise {
        SimplexNoise {
            simplex: OpenSimplex::new(seed),
            scale: 1.0,
        }
    }

    pub fn with_scale(mut self, scale: f32) -> SimplexNoise {
        self.scale = scale as f64;
        self
    }

    /// Returns an index with the range 0-1
    pub fn sample_2d<T: Into<f64>>(&self, x: T, y: T) -> f64 {
        self.simplex
            .get([x.into() / self.scale, y.into() / self.scale])
            + 0.5
    }
}
