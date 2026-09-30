JAVA_SRC_DIR := $(shell find ./java-ta/src/ -type f -regex ".*\.java")
USE_CASES_DIR := $(shell find ./use_cases/ -type f -regex ".*\.java")
RUST_SRC_DIR := $(shell find ./rust-ta/src/ -type f -regex ".*\.rs")

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
all: jar tee-server tee-client rust.manifest

run-dev: jar tee-server tee-client
	cp ./java-tee.jar rust-ta/ && cd rust-ta && ./target/debug/server


java-tee.jar: $(JAVA_SRC_DIR)
	cd java-ta && \
		mvn clean compile install && \
		cp ./target/java-ta-1.0-SNAPSHOT-jar-with-dependencies.jar ../java-tee.jar

.PHONY: jar
jar : java-tee.jar

blacklist-ta.jar :  $(USE_CASES_DIR)
	cd use_cases/blacklist/java && \
		mvn clean compile package && \
		cp ./target/*-with-dependencies.jar ../../../blacklist-ta.jar

.PHONY: blacklist
blacklist: tee-server tee-client blacklist-ta.jar
	cp blacklist-ta.jar rust-ta/java-tee.jar
	cd rust-ta && ./target/debug/server

tee-server: $(RUST_SRC_DIR)
	cd ./rust-ta && cargo build $(CARGO_CMD) --bin server  \
		&& cp target/$(CARGO_TARGET)/server ../tee-server \

tee-client: $(RUST_SRC_DIR)
	cd ./rust-ta/ && cargo build $(CARGO_RELEASE) --bin client \
		&& cp target/$(CARGO_TARGET)/client ../tee-client

.PHONY: run-server
run-server: java-tee.jar tee-server tee-client
	./tee-server

.PHONY: run-client
run-client: tee-client
	./tee-client

rust.manifest: rust.manifest.template
	gramine-manifest \
		-Dlog_level=$(GRAMINE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		-Dexec_dir=$(shell pwd) \
		-Djava_path=$(shell readlink -f $(shell which java)) \
		-Dentrypoint=./tee-server \
		$< >$@

release:
	gramine-manifest \
		-Dlog_level=$(GRAMINE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		-Dexec_dir=$(shell pwd) \
		-Djava_path=$(shell readlink -f $(shell which java)) \
		-Dentrypoint=./server \
		$< >$@
	gramine-sgx-sign \
		--manifest $< \
		--output $<.sgx


gramine-dev: rust.manifest
	gramine-direct rust

gramine-sgx: rust.manifest.sgx
	gramine-sgx rust

rust.manifest.sgx rust.sig: sgx_sign
	@:

.INTERMEDIATE: sgx_sign
sgx_sign: rust.manifest tee-server java-tee.jar
	gramine-sgx-sign \
		--manifest $< \
		--output $<.sgx

.PHONY: clean
clean:
	$(RM) *.token *.sig *.manifest.sgx *.manifest *.class tee-client tee-server *.jar

.PHONY: distclean
distclean: clean
