# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	@rustc --version              # Rust compiler
	@cargo --version              # Rust package manager
	@rustfmt --version            # Rust code formatter
	@rustup --version             # Rust toolchain manager
	@clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	@cargo fmt --quiet

# Run clippy for linting
lint:
	@cargo clippy --quiet

# Run tests
test:
	@cargo test --quiet

# Build and run the project
run:
	@cargo run

# Build release version
release:
	@cargo build --release

# Run all formatting, linting, and testing tasks
all: format lint test run

# Custom tasks for database operations
extract:
	@cargo run --bin chris_moreira_rust_database_individual_2 -- extract

create:
	@cargo run --bin chris_moreira_rust_database_individual_2 -- create_table

load:
	@cargo run --bin chris_moreira_rust_database_individual_2 -- load_data_from_csv

query-create:
	@cargo run --bin chris_moreira_rust_database_individual_2 -- query_create

query-read:
	@cargo run --bin chris_moreira_rust_database_individual_2 -- query_read

query-update:
	@cargo run --bin chris_moreira_rust_database_individual_2 -- query_update

query-delete:
	@cargo run --bin chris_moreira_rust_database_individual_2 -- query_delete
