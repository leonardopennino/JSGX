RUST_SRC_DIR := $(shell find ./src/ -type f -regex ".*\.rs")

ARCH_LIBDIR ?= /lib/$(shell $(CC) -dumpmachine)

ifeq ($(DEBUG),1)
GRAMINE_LOG_LEVEL = debug
else
GRAMINE_LOG_LEVEL = error
endif

ifeq ($(RELEASE),1)
CARGO_CMD = --release
CARGO_TARGET = release
else
CARGO_TARGET = debug
endif

.PHONY: all
all: ra-tls ra-tls.manifest

run-dev: ra-lts
	./ra-tls


ra-tls: $(RUST_SRC_DIR)
	cargo build $(CARGO_CMD) \
		&& cp target/$(CARGO_TARGET)/ra-tls .


.PHONY: run-test
run-test: ra-tls
	./tee-server


ra-tls.manifest: ra-tls.manifest.template
	gramine-manifest \
		-Dlog_level=$(GRAMINE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		-Dexec_dir=$(shell pwd) \
		-Dentrypoint=./ra-tls \
		$< >$@


gramine-dev: ra-tls.manifest
	gramine-direct ra-tls

gramine-sgx: ra-tls.manifest.sgx
	gramine-sgx ra-tls

ra-tls.manifest.sgx rust.sig: sgx_sign
	@:

.INTERMEDIATE: sgx_sign
sgx_sign: ra-tls.manifest ra-tls
	gramine-sgx-sign \
		--manifest $< \
		--output $<.sgx

.PHONY: clean
clean:
	$(RM) *.token *.sig *.manifest.sgx *.manifest  ra-tls

.PHONY: distclean
distclean: clean
