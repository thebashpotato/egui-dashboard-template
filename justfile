fmt:
  @cargo fmt --all

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

dev: fmt lint test build-debug

release: fmt lint build-release
