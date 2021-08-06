ifeq ($(PREFIX),)
	PREFIX := /usr/local
endif

build:
	cargo build --release

docs: MANUAL.pdf MANUAL.md

install:
	install -m 755 target/release/jbscheme $(DESTDIR)$(PREFIX)/bin/

test:
	cargo test

clean:
	cargo clean
	rm -f MANUAL.pdf MANUAL.md

MANUAL.pdf: MANUAL.pandoc.md
	pandoc -V geometry:margin="1.5in" -V title="JB Scheme Manual" --toc --toc-depth 4 -o $@ $<

MANUAL.md: MANUAL.pandoc.md
	pandoc -t gfm -s -o $@ $<

.PHONY: build install test clean
