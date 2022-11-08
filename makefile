all: format lint

format: 
	@echo "Format 🧹"
	@cargo fmt --all -- --check

lint: 
	@echo "Linting 🧹"
	@cargo clippy --all -- -D warnings

pedantic:
	@echo "Linting (pedantic) 🧹"
	@cargo clippy --all -- -D warnings -D clippy::pedantic