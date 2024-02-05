extern crate rand;
extern crate rand_distr;

use rand::prelude::*;
use rand_distr::Normal;

use crate::processes::Simulate;

pub struct BlackScholesProcess {
    pub s0: f64, // Initial asset price
    pub r: f64,  // Risk-free rate
    pub sigma: f64, // Volatility of the asset
    pub t: f64,  // Time to maturity
}

impl BlackScholesProcess {
    pub fn new(s0: f64, r: f64, sigma: f64, t: f64) -> BlackScholesProcess {
        BlackScholesProcess { s0, r, sigma, t }
    }
}

impl Simulate for BlackScholesProcess {
    fn generate_price_path(&self, number_of_steps: usize) -> Vec<f64> {
        let mut rng = thread_rng();
        let dt = self.t / number_of_steps as f64;
        let mut s_path: Vec<f64> = Vec::with_capacity(number_of_steps);
        let mut s = self.s0;

        let normal = Normal::new(0.0, 1.0).unwrap();

        for _ in 0..number_of_steps {
            let dw = normal.sample(&mut rng) * dt.sqrt();
            s += self.r * s * dt + self.sigma * s * dw;

            s_path.push(s);
        }

        s_path
    }
}