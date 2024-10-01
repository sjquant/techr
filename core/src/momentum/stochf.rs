use crate::utils::{calc_mean, find_max, find_min};

pub fn stochf(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period_k: usize,
    period_d: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = close.len();
    let mut percent_k = vec![None; len];
    let mut percent_d = vec![None; len];

    if len < period_k {
        return (percent_k, percent_d);
    }

    for i in (period_k - 1)..len {
        let max_high = find_max(&high[i + 1 - period_k..=i]);
        let min_low = find_min(&low[i + 1 - period_k..=i]);

        let k = if max_high == min_low {
            None
        } else {
            Some(((close[i] - min_low) / (max_high - min_low)) * 100.0)
        };

        percent_k[i] = k;

        if period_d == 1 {
            percent_d[i] = k;
        } else if i >= period_k - 1 + (period_d - 1) {
            let slice = &percent_k[i + 1 - period_d..=i];
            let valid_values: Vec<f64> = slice.iter().filter_map(|&x| x).collect();
            let d = if valid_values.len() == period_d {
                Some(calc_mean(&valid_values))
            } else {
                None
            };
            percent_d[i] = d;
        }
    }

    (percent_k, percent_d)
}

pub fn stochf_k(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period_k: usize,
    period_d: usize,
) -> Vec<Option<f64>> {
    stochf(high, low, close, period_k, period_d).0
}

pub fn stochf_d(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period_k: usize,
    period_d: usize,
) -> Vec<Option<f64>> {
    stochf(high, low, close, period_k, period_d).1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_stochf() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");

            let (percent_k, percent_d) = stochf(&high, &low, &close, 14, 3);

            let expected_k = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/stochf_K_{}.json",
                symbol
            ));
            let expected_d = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/stochf_D_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(percent_k, 8),
                round_vec(expected_k, 8),
                "STOCHF %K test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(percent_d, 8),
                round_vec(expected_d, 8),
                "STOCHF %D test failed for symbol {}.",
                symbol
            );
        }
    }
}
