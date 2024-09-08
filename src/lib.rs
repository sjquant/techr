pub mod momentum;
pub mod trend;
pub mod utils;
pub mod volatility;

pub trait TechnicalIndicator {
    fn compute(&self, data: &Vec<f64>) -> Vec<Option<f64>>;
}
