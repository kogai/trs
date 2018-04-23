NAME := trs
NAME_GEN := trs_gen
BIN := ./target/release/$(NAME)
SRC := $(shell find ./src -type f -name '*.rs')
PWD := $(shell pwd)
GOOGLE_CLOUD_PLATFORM_API_KEY := "${GOOGLE_CLOUD_PLATFORM_API_KEY}"
OXFORD_API_ID := "${OXFORD_API_ID}"
OXFORD_API_KEY := "${OXFORD_API_KEY}"

all: bin/Darwin/$(NAME) bin/Linux/$(NAME)

bin/Darwin/$(NAME): Cargo.toml $(SRC)
	cargo build --release
	mkdir -p bin/Darwin
	cp target/release/$(NAME) bin/Darwin/$(NAME)

bin/Linux/$(NAME): Cargo.toml $(SRC)
	docker build -t $(NAME) .
	docker run --rm -v `pwd`/target:/app/target \
		-e GOOGLE_CLOUD_PLATFORM_API_KEY=$(GOOGLE_CLOUD_PLATFORM_API_KEY) \
		-e OXFORD_API_ID=$(OXFORD_API_ID) \
		-e OXFORD_API_KEY=$(OXFORD_API_KEY) \
		-t $(NAME)
	mkdir -p bin/Linux
	cp target/release/$(NAME) bin/Linux/$(NAME)

.PHONY: clean
clean:
	cargo clean
