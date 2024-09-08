use crate::TechnicalIndicator;

pub struct SMA {
    window: usize,
}

impl SMA {
    pub fn new(window: usize) -> Self {
        SMA { window }
    }
}

impl TechnicalIndicator for SMA {
    fn compute(&self, data: &Vec<f64>) -> Vec<Option<f64>> {
        let mut sma = vec![None; data.len()];
        let mut sum = 0.0;

        for i in 0..data.len() {
            sum += data[i];
            if i >= self.window {
                sum -= data[i - self.window];
            }
            if i >= self.window - 1 {
                sma[i] = Some(sum / self.window as f64);
            }
        }

        sma
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sma = SMA::new(3);
        let result = sma.compute(&input);

        let expected = vec![None, None, Some(2.0), Some(3.0), Some(4.0)];
        assert_eq!(result, expected);
    }
}
