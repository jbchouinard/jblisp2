ifeq ($(PREFIX),)
	PREFIX := /usr/local
endif

build:
	cargo build --release

install:
	install -m 755 target/release/jbscheme $(DESTDIR)$(PREFIX)/bin/

test:
	cargo test

clean:
	cargo clean

.PHONY: build install test clean
