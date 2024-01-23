pub struct HestonModel {
    pub s0: f64, // Initial asset price
    pub v0: f64, // Initial variance
    pub r: f64, // Risk-free rate
    pub kappa: f64, // Rate of mean reversion
    pub theta: f64, // Long-term variance mean
    pub sigma: f64, // Volatility of volatility
    pub rho: f64, // Correlation between the two Brownian motions
    pub t: f64, // Time to maturity
}

impl HestonModel {
    // Initialize a new Heston model
    pub fn new(s0: f64, v0: f64, r: f64, kappa: f64, theta: f64, sigma: f64, rho: f64, t: f64) -> HestonModel {
        HestonModel { s0, v0, r, kappa, theta, sigma, rho, t }
    }
}


