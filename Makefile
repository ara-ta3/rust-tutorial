CARGO=cargo
RUSTUP=rustup


run:
	$(CARGO) run

build:
	$(CARGO) build

check:
	$(CARGO) check

components:
	$(RUSTUP) component add rls rust-analysis rust-src
