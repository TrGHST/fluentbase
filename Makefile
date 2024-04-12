all: build

.PHONY: build
build:
	cd crates/contracts && $(MAKE)
	cd examples && $(MAKE)
	cd crates/genesis && $(MAKE)



.PHONY: clean
clean:
	cargo clean
	rm -rf examples/node_modules
	rm -rf examples/bin/*.wat


.PHONY: test
test:
	clear
	cargo test --no-fail-fast
