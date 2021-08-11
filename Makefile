ifeq ($(PREFIX),)
	PREFIX := /usr/local
endif

all: build mkdocs/site MANUAL.md

build:
	cargo build --release

doc:
	cargo doc --release --no-deps

install:
	install -m 755 target/release/jibi $(DESTDIR)$(PREFIX)/bin/

test:
	cargo test

clean:
	cargo clean
	rm -rf mkdocs/site MANUAL.md $(PDF_MANUAL)

PDF_MANUAL = Jibi\ Scheme\ Manual.pdf
MD_MANUAL = mkdocs/docs/index.md

$(PDF_MANUAL): PANDOC_OPTS = -V documentclass=scrreprt
$(MD_MANUAL): PANDOC_OPTS = -s -t gfm
$(PDF_MANUAL) $(MD_MANUAL): mkdocs/manual.pandoc.md
	mkdir -p mkdocs/docs
	pandoc $(PANDOC_OPTS) -o "$@" "$<"

mkdocs/site: doc $(PDF_MANUAL) $(MD_MANUAL)
	cd mkdocs && mkdocs build
	mkdir -p mkdocs/site/crate mkdocs/site
	cp -r target/doc/* mkdocs/site/crate
	cp $(PDF_MANUAL) mkdocs/site

MANUAL.md: $(MD_MANUAL)
	cp "$<" "$@"

.PHONY: default build doc install test clean
