pub fn sma(data: &[f64], window: usize) -> Vec<Option<f64>> {
    let mut sma = vec![None; data.len()];
    let mut sum = 0.0;

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
    use super::*;

    #[test]
    fn test_sma() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(&input, 3);

        let expected = vec![None, None, Some(2.0), Some(3.0), Some(4.0)];
        assert_eq!(result, expected);
    }
}
