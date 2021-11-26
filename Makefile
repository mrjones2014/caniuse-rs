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
	cp target/x86_64-apple-darwin/release/caniuse ./caniuse-macos-x86


.PHONY: build-macos-arm
build-macos-arm:
	cargo build --release --target aarch64-apple-darwin
	cp target/aarch64-apple-darwin/release/caniuse ./caniuse-macos-arm

.PHONY: build-linux
build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/release/caniuse ./caniuse-linux-x86

.PHONY: build
build:
	cargo build

.PHONY: alfred-workflow
alfred-workflow:
	@if [ "$(WORKFLOW_FILE_NAME)" = "" ]; then echo "WORKFLOW_FILE_NAME variable not set"; exit 1; fi
	@if [ "$(CANIUSE_BIN)" = "" ]; then echo "CANIUSE_BIN variable not set"; exit 1; fi
	rm -f caniuse-x86.alfredworkflow
	zip -j -D $(WORKFLOW_FILE_NAME) info.plist
	zip -j -D $(WORKFLOW_FILE_NAME) README.md
	zip -j -D $(WORKFLOW_FILE_NAME) LICENSE
	zip -j -D $(WORKFLOW_FILE_NAME) images/icon.png
	# temporarily rename so it has the right name inside the zip
	mv $(CANIUSE_BIN) caniuse
	zip -j -D $(WORKFLOW_FILE_NAME) caniuse
	mv caniuse $(CANIUSE_BIN)

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
