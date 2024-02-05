extern crate rand;
extern crate rand_distr;

use rand::prelude::*;
use rand_distr::Normal;

use crate::processes::Simulate;

pub struct HestonProcess {
    pub s0: f64,
    // Initial asset price
    pub v0: f64,
    // Initial variance
    pub r: f64,
    // Risk-free rate
    pub kappa: f64,
    // Rate of mean reversion
    pub theta: f64,
    // Long-term variance mean
    pub sigma: f64,
    // Volatility of volatility
    pub rho: f64,
    // Correlation between the two Brownian motions
    pub t: f64, // Time to maturity
}

impl HestonProcess {
    pub fn new(s0: f64, v0: f64, r: f64, kappa: f64, theta: f64, sigma: f64, rho: f64, t: f64) -> HestonProcess {
        HestonProcess { s0, v0, r, kappa, theta, sigma, rho, t }
    }
}

impl Simulate for HestonProcess
{
    fn generate_price_path(&self, number_of_steps: usize) -> Vec<f64> {
        let mut rng = thread_rng();
        let dt = self.t / number_of_steps as f64;
        let mut s_path: Vec<f64> = Vec::with_capacity(number_of_steps);
        let mut s = self.s0;
        let mut v = self.v0;

        let normal = Normal::new(0.0, 1.0).unwrap();

        for _ in 0..number_of_steps {
            let dw_s = normal.sample(&mut rng) * dt.sqrt();
            let dw_v = self.rho * dw_s + (1.0 - self.rho.powi(2)).sqrt() * normal.sample(&mut rng) * dt.sqrt();

            s += self.r * s * dt + s * v.sqrt() * dw_s;
            v += self.kappa * (self.theta - v) * dt + self.sigma * v.sqrt() * dw_v;

            v = v.max(0.0);

            s_path.push(s);
        }

        s_path
    }
}
