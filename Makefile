NAME := trs
NAME_GEN := trs_gen
BIN := ./target/release/$(NAME)
SRC := $(shell find ./src -type f -name '*.rs')
PWD := $(shell pwd)
GOOGLE_CLOUD_PLATFORM_API_KEY := "${GOOGLE_CLOUD_PLATFORM_API_KEY}"
OXFORD_API_ID := "${OXFORD_API_ID}"
OXFORD_API_KEY := "${OXFORD_API_KEY}"
OS := $(shell uname)
VERSION := $(shell cat Cargo.toml | grep version | sed -e 's/version\ =\ \"\(.*\)\"/\1/')

bin/$(OS)/$(NAME): Cargo.toml $(SRC)
	GOOGLE_CLOUD_PLATFORM_API_KEY=$(GOOGLE_CLOUD_PLATFORM_API_KEY) && \
	OXFORD_API_ID=$(OXFORD_API_ID) && \
	OXFORD_API_KEY=$(OXFORD_API_KEY) && \
	cargo build --release
	mkdir -p bin/$(OS)
	cp target/release/$(NAME) bin/$(OS)/$(NAME)

bin/Docker/$(NAME): Cargo.toml $(SRC)
	docker build -t $(NAME) .
	docker run --rm -v `pwd`/target:/app/target \
		-e GOOGLE_CLOUD_PLATFORM_API_KEY=$(GOOGLE_CLOUD_PLATFORM_API_KEY) \
		-e OXFORD_API_ID=$(OXFORD_API_ID) \
		-e OXFORD_API_KEY=$(OXFORD_API_KEY) \
		-t $(NAME)
	mkdir -p bin/$(OS)
	cp target/release/$(NAME) bin/$(OS)/$(NAME)

.PHONY: cache
cache:
	rm -f .trs-cache
	GOOGLE_CLOUD_PLATFORM_API_KEY=$(GOOGLE_CLOUD_PLATFORM_API_KEY) && \
	OXFORD_API_ID=$(OXFORD_API_ID) && \
	OXFORD_API_KEY=$(OXFORD_API_KEY) && \
	cargo run -- -d dog
	ls -lh .trs-cache

.PHONY: clean
clean:
	rm -rf bin
	cargo clean

.PHONY: secret
secret:
	travis encrypt \
		GOOGLE_CLOUD_PLATFORM_API_KEY="${GOOGLE_CLOUD_PLATFORM_API_KEY}" \
		OXFORD_API_ID="${OXFORD_API_ID}" \
		OXFORD_API_KEY="${OXFORD_API_KEY}" \
		--add env
	travis encrypt \
		GITHUB_API_TOKEN="${GITHUB_API_TOKEN}" \
		--add deploy.api_key

.PHONY: deploy
deploy:
	git tag -a "v${VERSION}" -m ""
	git push --tags
	