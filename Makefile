.phony: all help

all:
	cargo build --release

run:
	cargo run --release

help:
	@echo "pinneedle makefile:"
	@echo "make:        Builds the server"
	@echo "make run:    Builds and runs the server"
	@echo "make help:   Prints this info"
