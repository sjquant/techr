from pathlib import Path

import polars as pl
from polars.plugins import register_plugin_function


from polars_techr.types import IntoExpr

LIB: Path = Path(__file__).parent


def sma(expr: IntoExpr, *, period: int) -> pl.Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=LIB,
        function_name="sma",
        is_elementwise=False,
        kwargs={"period": period},
    )
