ifeq ($(PREFIX),)
	PREFIX := /usr/local
endif

target/release/jbscheme:
	cargo build --release

install: target/release/jbscheme
	install -m 755 $< $(DESTDIR)$(PREFIX)/bin/

test:
	cargo test
	cargo run -- tests/tests.jbscm

clean:
	cargo clean

.PHONY: test clean