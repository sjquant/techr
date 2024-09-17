pub fn wma(data: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];

    if data.len() < period {
        return result;
    }

    let weight_sum = (period * (period + 1)) / 2;
    let mut weighted_sum = 0.0;

    // Initialize the first period
    for i in 0..period {
        weighted_sum += data[i] * (i + 1) as f64;
    }

    for i in period - 1..data.len() {
        result[i] = Some(weighted_sum / weight_sum as f64);
        if i + 1 < data.len() {
            weighted_sum = weighted_sum + data[i + 1] * period as f64
                - data[i + 1 - period..=i].iter().sum::<f64>();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::utils::round_vec;

    use super::*;

    #[test]
    fn test_wma() {
        let input = vec![
            100.0, 102.5, 99.8, 101.7, 103.2, 98.5, 100.9, 102.1, 104.3, 103.8, 105.2, 106.7,
            104.9, 107.3, 108.1,
        ];
        let period = 5;
        let result = wma(&input, period);
        assert_eq!(
            round_vec(result, 4),
            [
                None,
                None,
                None,
                None,
                Some(101.8133),
                Some(100.8333),
                Some(100.7533),
                Some(101.18),
                Some(102.1867),
                Some(102.8533),
                Some(103.9467),
                Some(105.0933),
                Some(105.2533),
                Some(106.0267),
                Some(106.8667)
            ]
        );
    }
}
