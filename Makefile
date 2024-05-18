.PHONY: build run clean watch

# Default command
all: run

# Compile the project
build:
	cargo build

# Run the project
run:
	cargo run

# Clean up the project
clean:
	cargo clean

# Watch for changes and rebuild
watch:
	cargo watch -x run

# Run tests
test:
	cargo test

# View documentation
doc:
	cargo doc --open
