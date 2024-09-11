pub fn sma(data: &[f64], window: usize) -> Vec<Option<f64>> {
    let mut sma = vec![None; data.len()];
    let mut sum = 0.0;

    if data.len() < window {
        return sma;
    }

    for i in 0..data.len() {
        sum += data[i];
        if i >= window {
            sum -= data[i - window];
        }
        if i >= window - 1 {
            sma[i] = Some(sum / window as f64);
        }
    }

    sma
}

#[cfg(test)]
mod tests {
    use crate::utils::round_vec;

    use super::*;

    #[test]
    fn test_sma() {
        let input = vec![
            100.0, 102.5, 99.8, 101.7, 103.2, 98.5, 100.9, 102.1, 104.3, 103.8, 105.2, 106.7,
            104.9, 107.3, 108.1,
        ];
        let result = sma(&input, 5);

        assert_eq!(
            round_vec(result, 4),
            [
                None,
                None,
                None,
                None,
                Some(101.44),
                Some(101.14),
                Some(100.82),
                Some(101.28),
                Some(101.8),
                Some(101.92),
                Some(103.26),
                Some(104.42),
                Some(104.98),
                Some(105.58),
                Some(106.44)
            ]
        );
    }
}
