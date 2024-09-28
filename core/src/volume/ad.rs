pub fn ad(highs: &[f64], lows: &[f64], closes: &[f64], volumes: &[f64]) -> Vec<Option<f64>> {
    let mut ad = vec![None; highs.len()];

    let len = highs.len();

    if len == 0 || len != lows.len() || len != closes.len() || len != volumes.len() {
        return ad;
    }

    let mut ad_point = 0.0;
    for i in 0..len {
        let high = highs[i];
        let low = lows[i];
        let close = closes[i];
        let volume = volumes[i];

        if (2.0 * close - low - high).abs() < f64::EPSILON || (high - low).abs() < f64::EPSILON {
            ad_point += 0.0;
        } else {
            ad_point += ((2.0 * close - low - high) / (high - low)) * volume;
        }
        ad[i] = Some(ad_point);
    }

    ad
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_ad() {
        let test_cases = vec!["005930"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v");

            let result = ad(&high, &low, &close, &volume);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ad_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 6),
                round_vec(expected, 6),
                "AD test failed for symbol {}.",
                symbol
            );
        }
    }
}
