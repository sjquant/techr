use crate::TechnicalIndicator;

pub struct RSI {
    window: usize,
}

impl RSI {
    pub fn new(window: usize) -> Self {
        RSI { window }
    }
}

impl TechnicalIndicator for RSI {
    fn compute(&self, data: &Vec<f64>) -> Vec<Option<f64>> {
        let mut rsi = vec![None; data.len()];
        let mut total_up = 0.0;
        let mut total_down = 0.0;
        let mut avg_up;
        let mut avg_down;

        for i in 1..data.len() {
            let change = data[i] - data[i - 1];
            if change > 0.0 {
                total_up += change;
            } else {
                total_down += change.abs();
            }
        }

        avg_up = total_up / (self.window - 1) as f64;
        avg_down = total_down / (self.window - 1) as f64;

        for i in self.window..data.len() {
            let change = data[i] - data[i - 1];
            let up = change.max(0.0);
            let down = change.abs();
            avg_up = (avg_up * (self.window - 1) as f64 + up) / self.window as f64;
            avg_down = (avg_down * (self.window - 1) as f64 + down) / self.window as f64;

            let rsi_point = if avg_down == 0.0 {
                100.0
            } else if avg_up == 0.0 {
                0.0
            } else {
                (avg_up / (avg_up + avg_down)) * 100.0
            };

            rsi[i] = Some(rsi_point);
        }

        rsi
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::round_vec;

    #[test]
    fn test_rsi() {
        let input = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61, 46.28, 46.28,
        ];
        let rsi = RSI::new(14);
        let result = rsi.compute(&input);

        let expected = vec![
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(70.464135),
        ];

        assert_eq!(round_vec(result, 6), expected);
    }
}
