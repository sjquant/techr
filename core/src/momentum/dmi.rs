use crate::utils::get_true_ranges;

pub fn dmi(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = highs.len();
    let mut plus_di = vec![None; len];
    let mut minus_di = vec![None; len];

    let delta_highs: Vec<f64> = highs.windows(2).map(|w| (w[1] - w[0]).max(0.0)).collect();
    let delta_lows: Vec<f64> = lows.windows(2).map(|w| (w[0] - w[1]).max(0.0)).collect();
    let trs = get_true_ranges(highs, lows, closes);

    let plus_dm: Vec<f64> = delta_highs
        .iter()
        .zip(delta_lows.iter())
        .map(|(&dh, &dl)| if dh > dl && dh > 0.0 { dh } else { 0.0 })
        .collect();
    let minus_dm: Vec<f64> = delta_highs
        .iter()
        .zip(delta_lows.iter())
        .map(|(&dh, &dl)| if dl > dh && dl > 0.0 { dl } else { 0.0 })
        .collect();

    let plus_dm_sum = wilders_smoothing(&plus_dm, period);
    let minus_dm_sum = wilders_smoothing(&minus_dm, period);
    let tr_sum = wilders_smoothing(&trs, period);

    for i in period..len {
        if tr_sum[i - period] == 0.0 {
            plus_di[i] = Some(0.0);
            minus_di[i] = Some(0.0);
        } else {
            plus_di[i] = Some((plus_dm_sum[i - period] / tr_sum[i - period]) * 100.0);
            minus_di[i] = Some((minus_dm_sum[i - period] / tr_sum[i - period]) * 100.0);
        }
    }

    (plus_di, minus_di)
}

fn wilders_smoothing(data: &[f64], period: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(data.len() - period + 1);
    let mut partial_sum: f64 = data.iter().take(period - 1).sum();

    for i in period - 1..data.len() {
        partial_sum = partial_sum - (partial_sum / period as f64) + data[i];
        result.push(partial_sum);
    }

    result
}

pub fn dmi_plus_di(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Vec<Option<f64>> {
    dmi(highs, lows, closes, period).0
}

pub fn dmi_minus_di(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    period: usize,
) -> Vec<Option<f64>> {
    dmi(highs, lows, closes, period).1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_dmi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let (plus_di, minus_di) = dmi(&highs, &lows, &closes, 14);

            let expected_plus_di = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/dmi_plus_{}.json",
                symbol
            ));
            let expected_minus_di = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/dmi_minus_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(plus_di, 8),
                round_vec(expected_plus_di, 8),
                "DMI +DI test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(minus_di, 8),
                round_vec(expected_minus_di, 8),
                "DMI -DI test failed for symbol {}.",
                symbol
            );
        }
    }
}
