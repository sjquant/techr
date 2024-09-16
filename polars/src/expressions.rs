use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

use serde::Deserialize;
use techr::sma as techr_sma;

#[derive(Deserialize)]
struct SMAKwargs {
    window: u32,
}

#[polars_expr(output_type=Float64)]
fn sma(inputs: &[Series], kwargs: SMAKwargs) -> PolarsResult<Series> {
    let window = kwargs.window as usize;
    let input = inputs[0].f64()?.to_vec_null_aware().left().unwrap();
    let out = techr_sma(&input, window);
    let out_series: Series = out.into_iter().collect();
    Ok(out_series)
}
