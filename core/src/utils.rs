pub fn round_scalar(value: f64, decimal_places: u32) -> f64 {
    let factor = 10.0f64.powi(decimal_places as i32);
    (value * factor).round() / factor
}

pub fn round_vec(vec: Vec<Option<f64>>, decimal_places: u32) -> Vec<Option<f64>> {
    vec.iter()
        .map(|&x| {
            x.map(|y| {
                let factor = 10.0f64.powi(decimal_places as i32);
                (y * factor).round() / factor
            })
        })
        .collect()
}

pub fn calc_mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    let count = data.len();
    sum / count as f64
}

pub fn find_max(data: &[f64]) -> f64 {
    data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}

pub fn find_min(data: &[f64]) -> f64 {
    data.iter().cloned().fold(f64::INFINITY, f64::min)
}

pub fn calc_clv(high: f64, low: f64, close: f64) -> f64 {
    if high == low {
        0.0
    } else {
        ((close - low) - (high - close)) / (high - low)
    }
}

pub fn get_true_ranges(highs: &[f64], lows: &[f64], closes: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(highs.len() - 1);

    for i in 1..highs.len() {
        let high = highs[i];
        let low = lows[i];
        let prev_close = closes[i - 1];
        result.push(calc_tr(high, low, prev_close));
    }

    result
}

fn calc_tr(high: f64, low: f64, prev_close: f64) -> f64 {
    let th = high.max(prev_close);
    let tl = low.min(prev_close);
    th - tl
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        let test_cases = vec![
            (1.23456, 2, 1.23),
            (7.891011, 2, 7.89),
            (12.345678, 2, 12.35),
            (3.14159, 3, 3.142),
            (100.0, 1, 100.0),
            (0.001, 1, 0.0),
        ];

        for (input, decimal_places, expected) in test_cases {
            let result = round_scalar(input, decimal_places);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_round_vec() {
        let test_cases = vec![
            (
                vec![Some(1.23456), Some(7.891011), None, Some(12.345678)], // input
                2,                                                          // decimal_places
                vec![Some(1.23), Some(7.89), None, Some(12.35)],            // expected
            ),
            (
                vec![Some(3.14159), Some(2.71828), Some(1.41421)],
                3,
                vec![Some(3.142), Some(2.718), Some(1.414)],
            ),
            (
                vec![Some(100.0), Some(0.001), Some(10.0)],
                1,
                vec![Some(100.0), Some(0.0), Some(10.0)],
            ),
        ];

        for (input, decimal_places, expected) in test_cases {
            let result = round_vec(input, decimal_places);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_calc_mean() {
        let result = calc_mean(&vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_find_max() {
        let result = find_max(&vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_find_min() {
        let result = find_min(&vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_calc_clv() {
        let test_cases = vec![
            (1.0, 2.0, 3.0, -3.0),
            (1.0, 2.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 0.0),
        ];
        for (high, low, close, expected) in test_cases {
            let result = calc_clv(high, low, close);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_get_true_ranges() {
        let highs = vec![
            10.0, 12.0, 11.5, 13.0, 14.5, 13.5, 15.0, 16.0, 15.5, 17.0, 18.0, 17.5, 19.0, 20.0,
            19.5, 21.0, 22.0, 21.5, 23.0, 24.0, 23.5,
        ];
        let lows = vec![
            9.0, 10.5, 10.0, 11.5, 13.0, 12.0, 13.5, 14.5, 14.0, 15.5, 16.5, 16.0, 17.5, 18.5,
            18.0, 19.5, 20.5, 20.0, 21.5, 22.5, 22.0,
        ];
        let closes = vec![
            9.5, 11.5, 10.5, 12.5, 14.0, 13.0, 14.5, 15.5, 15.0, 16.5, 17.5, 17.0, 18.5, 19.5,
            19.0, 20.5, 21.5, 21.0, 22.5, 23.5, 23.0,
        ];

        let expected = vec![
            2.5, 1.5, 2.5, 2.0, 2.0, 2.0, 1.5, 1.5, 2.0, 1.5, 1.5, 2.0, 1.5, 1.5, 2.0, 1.5, 1.5,
            2.0, 1.5, 1.5,
        ];

        let result = get_true_ranges(&highs, &lows, &closes);
        assert_eq!(result, expected, "Failed for dynamic input");
    }
}
