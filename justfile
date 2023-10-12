format:
  @cargo fmt

lint:
  @cargo clippy --all-targets --all-features -- -D warnings

clean:
  @cargo clean

build-debug:
  @cargo build

build-release:
  @cargo build --release

test: clean
  cargo test

dev: format lint test build-debug

release: format lint build-release
