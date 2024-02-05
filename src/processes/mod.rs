pub mod heston_process;
pub mod black_scholes_process;

pub trait Simulate {
    fn generate_price_path(&self, number_of_steps: usize) -> Vec<f64>;
}