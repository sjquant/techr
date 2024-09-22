pub fn willr(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if len < period {
        return result;
    }

    for i in period - 1..len {
        let slice_high = &highs[i + 1 - period..=i];
        let slice_low = &lows[i + 1 - period..=i];
        let (ll, hh) = get_array_extremes(slice_high, slice_low);

        let cc = closes[i];
        if hh == ll {
            result[i] = None;
        } else {
            result[i] = Some(((hh - cc) / (hh - ll)) * -100.0);
        }
    }

    result
}

#[inline]
pub fn get_array_extremes(high: &[f64], low: &[f64]) -> (f64, f64) {
    let ll = low.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let hh = high.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    (ll, hh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_willr() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = willr(&high, &low, &close, 14);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/willr_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "WILLR test failed for symbol {}.",
                symbol
            );
        }
    }
}
