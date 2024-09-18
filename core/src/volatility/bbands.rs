use crate::{overlap::sma, utils::stddev_scalar};

pub fn bbands(
    data: &[f64],
    period: usize,
    multiplier: Option<f64>,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let center = sma(data, period);
    let mut upper_band = vec![None; data.len()];
    let mut lower_band = vec![None; data.len()];

    if data.len() < period {
        return (upper_band, center, lower_band);
    }

    let multiplier = multiplier.unwrap_or(2.0);

    for i in period - 1..data.len() {
        if center[i].is_some() {
            let stddev = stddev_scalar(&data[i + 1 - period..i]);
            upper_band[i] = Some(center[i].unwrap() + multiplier * stddev);
            lower_band[i] = Some(center[i].unwrap() - multiplier * stddev);
        }
    }

    (upper_band, center, lower_band)
}

#[cfg(test)]
mod tests {
    use crate::utils::round_vec;

    use super::*;

    #[test]
    fn test_bbands() {
        let data = vec![
            100.25, 101.50, 99.75, 102.00, 103.25, 101.75, 100.50, 99.00, 100.75, 102.50, 104.00,
            103.50, 102.75, 101.25, 100.00,
        ];

        let (up, center, down) = bbands(&data, 5, Some(2.0));
        assert_eq!(
            round_vec(up, 4),
            [
                None,
                None,
                None,
                None,
                Some(103.1700),
                Some(104.1625),
                Some(103.9594),
                Some(103.2526),
                Some(104.1825),
                Some(102.8685),
                Some(103.8343),
                Some(105.6979),
                Some(105.1843),
                Some(103.9924),
                Some(104.3767)
            ]
        );
        assert_eq!(
            round_vec(center, 4),
            vec![
                None,
                None,
                None,
                None,
                Some(101.35),
                Some(101.65),
                Some(101.45),
                Some(101.3),
                Some(101.05),
                Some(100.9),
                Some(101.35),
                Some(101.95),
                Some(102.7),
                Some(102.8),
                Some(102.3)
            ]
        );
        assert_eq!(
            round_vec(down, 4),
            [
                None,
                None,
                None,
                None,
                Some(99.53),
                Some(99.1375),
                Some(98.9406),
                Some(99.3474),
                Some(97.9175),
                Some(98.9315),
                Some(98.8657),
                Some(98.2021),
                Some(100.2157),
                Some(101.6076),
                Some(100.2233)
            ]
        );
    }
}
