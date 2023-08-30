
build:
	cargo build

buildrun: build run

prod:
	cargo build --release
	php -d extension=$(shell pwd)/target/release/libhello.so test.php

run:
	php -d extension=$(shell pwd)/target/debug/libhello.so test.php
	