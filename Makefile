# Run the server with auto-reload
dev:
	cargo watch -q -c -w src -w templates -w content -x run

# Build for production (optimized)
build:
	cargo build --release

# Run tests
test:
	cargo test

# Clean up build files
clean:
	rm -rf target/

# Help command
help:
	@echo "Available commands:"
	@echo "  make dev   - Start the server with auto-reload (requires cargo-watch)"
	@echo "  make build - Build a release binary"
	@echo "  make test  - Run Rust tests"
	@echo "  make clean - Remove the 'target' build folder"