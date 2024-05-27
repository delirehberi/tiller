build:
	cargo build --release

install:
	sudo cp target/release/tiller /usr/local/bin/
	sudo chmod +x /usr/local/bin/tiller 

bi: build install

