build: 
	docker build -t my-rust-image .


run: build
	docker run -it -v $(shell pwd):/workspace --workdir=/workspace  my-rust-image /bin/bash