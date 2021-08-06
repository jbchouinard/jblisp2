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

MANUAL.pdf: pandoc/manual.md
	pandoc -V geometry:margin="1.5in" -V title="JB Scheme Manual" --toc --toc-depth 4 -o $@ $<

MANUAL.md: pandoc/manual.md
	pandoc -t gfm --toc --toc-depth 3 -s -o $@ $<

.PHONY: build install test clean
