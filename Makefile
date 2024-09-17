.DEFAULT_GOAL := all

# using pip install cargo (via maturin via pip) doesn't get the tty handle
# so doesn't render color without some help
export CARGO_TERM_COLOR=$(shell (test -t 0 && echo "always") || echo "auto")

# maturin develop only makes sense inside a virtual env, is otherwise
# more or less equivalent to pip install -e just a little nicer
USE_MATURIN = $(shell [ "$$VIRTUAL_ENV" != "" ] && (which maturin))

.PHONY: build-dev-polars
build-dev-polars:
	@rm -f polars/polars_techr/*.so
	cd polars &&maturin develop


.PHONY: build-prod-polars
build-prod-polars:
	@rm -f polars/polars_techr/*.so
	cd polars && maturin build --release


.PHONY: test-polars
test-polars:
	cd polars && pytest
