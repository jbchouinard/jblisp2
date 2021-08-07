ifeq ($(PREFIX),)
	PREFIX := /usr/local
endif

PDF_MANUAL = docs/JB\ Scheme\ Manual.pdf
MD_MANUAL = MANUAL.md
HTML_MANUAL = docs/jbscheme.manual.html
MANUALS = $(MD_MANUAL) $(HTML_MANUAL) $(PDF_MANUAL)

build:
	cargo build --release

docs: $(MANUALS)
	cargo doc --no-deps
	cp -r target/doc/* docs/
	cp docs.index.html docs/index.html

install:
	install -m 755 target/release/jbscheme $(DESTDIR)$(PREFIX)/bin/

test:
	cargo test

clean:
	cargo clean
	rm -rf docs

$(PDF_MANUAL): PANDOC_OPTS = -V documentclass=scrreprt
$(MD_MANUAL): PANDOC_OPTS = -s -t gfm
$(HTML_MANUAL): PANDOC_OPTS = -s --metadata title="JB Scheme Manual" --toc --toc-depth 4
$(MANUALS): MANUAL.pandoc.md
	mkdir -p docs
	pandoc $(PANDOC_OPTS) -o "$@" "$<"

.PHONY: build install test clean docs
