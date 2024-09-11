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

#[cfg(test)]
mod tests {
    use super::*;

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
}
