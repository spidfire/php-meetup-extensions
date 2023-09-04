
build:
	cargo build
	@echo "\nBuild complete.\n"

buildrun: build run

FILE ?= test1.php


prod:
	cargo build --release
	php -d extension=$(shell pwd)/target/release/libhello.so $(FILE)


run:
	@echo "\nRunning $(FILE):"
	@php -d extension=$(shell pwd)/target/debug/libhello.so $(FILE)
	@echo "\n"
