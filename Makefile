# Makefile for quick access development-only workflows.

# Deploy the project as a WebAssembly backed JavaScript module.
.PHONY: deploy
deploy: build
	@cargo build --features='deploy,debug' --release --target=wasm32-unknown-unknown

# Build the project's code to WebAssembly byte code.
.PHONY: build
build:
	@cargo build --features='debug' --release --target=wasm32-unknown-unknown

# Run the Web UI.
#
# Make sure to `deploy` before or afterwards to serve up-to-date data.
.PHONY: run
run:
	@cd www && python3 -m http.server
