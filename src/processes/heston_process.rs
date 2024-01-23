extern crate rand;
extern crate rand_distr;

use rand::prelude::*;
use rand_distr::Normal;

use crate::models::heston_model::HestonModel;

pub struct HestonProcess {
    pub model: HestonModel, // Process parameters
    pub n: usize, // Number of time steps
}

impl HestonProcess {
    pub fn new(model: HestonModel, n: usize) -> HestonProcess {
        HestonProcess { model, n }
    }

    pub fn generate_path(&self) -> Vec<f64> {
        let mut rng = thread_rng();
        let dt = self.model.t / self.n as f64;
        let mut s_path: Vec<f64> = Vec::with_capacity(self.n);
        let mut s = self.model.s0;
        let mut v = self.model.v0;

        let normal = Normal::new(0.0, 1.0).unwrap();

        for _ in 0..self.n {
            let dw_s = normal.sample(&mut rng) * dt.sqrt();
            let dw_v = self.model.rho * dw_s + (1.0 - self.model.rho.powi(2)).sqrt() * normal.sample(&mut rng) * dt.sqrt();

            s += self.model.r * s * dt + s * v.sqrt() * dw_s;
            v += self.model.kappa * (self.model.theta - v) * dt + self.model.sigma * v.sqrt() * dw_v;

            v = v.max(0.0);

            s_path.push(s);
        }

        s_path
    }
}
