.PHONY: install-targets-mac-x86
install-targets-mac-x86:
	rustup target add x86_64-apple-darwin

.PHONY: install-targets-mac-arm
install-targets-mac-arm:
	rustup target add aarch64-apple-darwin

.PHONY: install-targets-linux
install-targets-linux:
	rustup target add x86_64-unknown-linux-gnu

.PHONY: build-macos-x86
build-macos-x86:
	cargo build --release --target x86_64-apple-darwin
	test -f target/x86_64-apple-darwin/release/caniuse


.PHONY: build-macos-arm
build-macos-arm:
	cargo build --release --target aarch64-apple-darwin
	test -f target/aarch64-apple-darwin/release/caniuse

.PHONY: build-linux
build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu
	test -f target/x86_64-unknown-linux-gnu/release/caniuse

.PHONY: build
build:
	cargo build

.PHONY: alfred-workflow-x86
alfred-workflow-x86:
	rm -f caniuse-x86.alfredworkflow
	zip -j -D caniuse-x86.alfredworkflow info.plist
	zip -j -D caniuse-x86.alfredworkflow README.md
	zip -j -D caniuse-x86.alfredworkflow LICENSE
	zip -j -D caniuse-x86.alfredworkflow images/icon.png
	zip -j -D caniuse-x86.alfredworkflow target/x86_64-apple-darwin/release/caniuse

.PHONY: alfred-workflow-arm
alfred-workflow-arm:
	rm -f caniuse-arm.alfredworkflow
	zip -j -D caniuse-arm.alfredworkflow info.plist
	zip -j -D caniuse-arm.alfredworkflow README.md
	zip -j -D caniuse-arm.alfredworkflow LICENSE
	zip -j -D caniuse-arm.alfredworkflow images/icon.png
	zip -j -D caniuse-arm.alfredworkflow target/aarch64-apple-darwin/release/caniuse

.PHONY: publish
publish:
	@if [ "$(CARGO_TOKEN)" = "" ]; then echo "CARGO_TOKEN variable not set"; exit 1; fi
	cargo login $(CARGO_TOKEN)
	cargo publish

.PHONY: clean
clean:
	cargo clean
	rm -f caniuse-x86.alfredworkflow
	rm -f caniuse-arm.alfredworkflow
