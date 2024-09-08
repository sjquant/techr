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
