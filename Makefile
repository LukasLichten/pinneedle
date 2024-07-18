.phony: all run clean help docker-build docker-run

all:
	cargo build --release

run: all
	PIN_BLOG_REPO='https://github.com/LukasLichten/pinneedle-template-blog.git' ./target/release/pinneedle

clean:
	rm -rf ./blog

docker-build:
	docker build -t pinneedle .

docker-run:
	docker run -p 3000:3000 -e PIN_BLOG_REPO='https://github.com/LukasLichten/pinneedle-template-blog.git' pinneedle:latest

help:
	@echo "pinneedle makefile:"
	@echo "make:               Builds the server"
	@echo "make run:           Builds and runs the server"
	@echo "make clean:         Deletes the ./blog folder"
	@echo "make docker-build:  Builds the docker container"
	@echo "make docker-run:    Runs the container"
	@echo "make help:          Prints this info"
