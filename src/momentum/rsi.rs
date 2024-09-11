pub fn rsi(data: &[f64], window: usize) -> Vec<Option<f64>> {
    let mut rsi = vec![None; data.len()];
    let mut total_up = 0.0;
    let mut total_down = 0.0;
    let mut avg_up;
    let mut avg_down;

    for i in 1..window {
        let change = data[i] - data[i - 1];
        if change > 0.0 {
            total_up += change;
        } else {
            total_down += change.abs();
        }
    }

    avg_up = total_up / (window - 1) as f64;
    avg_down = total_down / (window - 1) as f64;

    for i in window..data.len() {
        let change = data[i] - data[i - 1];
        let up = change.max(0.0);
        let down = change.abs();
        avg_up = (avg_up * (window - 1) as f64 + up) / window as f64;
        avg_down = (avg_down * (window - 1) as f64 + down) / window as f64;

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

#[cfg(test)]
mod tests {
    use crate::utils::round_vec;

    use super::*;

    #[test]
    fn test_rsi() {
        let input = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61, 46.28, 46.28, 46.00, 46.03, 46.41, 46.22, 45.64, 46.21,
        ];
        let result = rsi(&input, 14);

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
            Some(66.249619),
            Some(66.028387),
            Some(63.517888),
            Some(60.947772),
            Some(53.792335),
            Some(53.037494),
        ];

        assert_eq!(round_vec(result, 6), expected);
    }
}
