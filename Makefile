ifeq ($(PREFIX),)
	PREFIX := /usr/local
endif

MANUALS = MANUAL.pdf MANUAL.html MANUAL.md

build:
	cargo build --release

docs: $(MANUALS)

install:
	install -m 755 target/release/jbscheme $(DESTDIR)$(PREFIX)/bin/

test:
	cargo test

clean:
	cargo clean
	rm -f $(MANUALS)

MANUAL.pdf: PANDOC_OPTS = -V documentclass=scrreprt
MANUAL.md: PANDOC_OPTS = -s -t gfm
MANUAL.html: PANDOC_OPTS = -s --metadata title="JB Scheme Manual" --toc --toc-depth 4
$(MANUALS): MANUAL.pandoc.md
	pandoc $(PANDOC_OPTS) -o $@ $<

.PHONY: build install test clean
