pub mod vanilla_option;

pub enum OptionType {
    Call,
    Put,
}

pub trait Payoff{
    fn payoff(&self, process_path: Vec<(f64, f64)>) -> f64;
}