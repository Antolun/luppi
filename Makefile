.PHONY: all build package run clean

all: build

build:
	cargo build --release

run:
	cargo run

package: build
	./build-luppo.sh

clean:
	cargo clean
