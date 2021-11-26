.PHONY: install-targets-mac-x86
install-targets-mac-x86:
	rustup target add x86_64-apple-darwin

.PHONY: install-targets-mac-m1
install-targets-mac-m1:
	rustup target add aarch64-apple-darwin

.PHONY: install-targets-linux
install-targets-linux:
	rustup target add x86_64-unknown-linux-gnu

.PHONY: build-macos-x86
build-macos-x86:
	cargo build --release --target x86_64-apple-darwin
	test -f target/x86_64-apple-darwin/release/caniuse


.PHONY: build-macos-m1
build-macos-m1:
	cargo build --release --target aarch64-apple-darwin
	test -f target/aarch64-apple-darwin/release/caniuse

.PHONY: build-linux
build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu
	test -f target/x86_64-unknown-linux-gnu/release/caniuse

.PHONY: build
build:
	cargo build

.PHONY: alfred-workflow
alfred-workflow: build-macos-x86
	rm -f caniuse.alfredworkflow
	zip -j -D caniuse.alfredworkflow info.plist
	zip -j -D caniuse.alfredworkflow README.md
	zip -j -D caniuse.alfredworkflow LICENSE
	zip -j -D caniuse.alfredworkflow images/icon.png
	zip -j -D caniuse.alfredworkflow target/x86_64-apple-darwin/release/caniuse

.PHONY: publish
publish:
	@if [ "$(CARGO_TOKEN)" = "" ]; then echo "CARGO_TOKEN variable not set"; exit 1; fi
	cargo login $(CARGO_TOKEN)
	cargo publish

.PHONY: clean
clean:
	cargo clean
	rm -f caniuse.alfredworkflow
