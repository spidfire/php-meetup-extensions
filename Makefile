build: 
	podman build -t my-rust-image .


run: build
	podman run -it -v $(shell pwd):/workspace --workdir=/workspace  my-rust-image /bin/bash