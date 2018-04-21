NAME := trs
NAME_GEN := trs_gen
BIN := ./target/release/$(NAME)
SRC := $(shell find ./src -type f -name '*.rs')
PWD := $(shell pwd)
GOOGLE_CLOUD_PLATFORM_API_KEY := "${GOOGLE_CLOUD_PLATFORM_API_KEY}"
OXFORD_API_ID := "${OXFORD_API_ID}"
OXFORD_API_KEY := "${OXFORD_API_KEY}"

bin/$(NAME): Cargo.toml $(SRC) oxford_client
	docker build -t $(NAME) .
	docker run --rm -v `pwd`/target:/app/target \
		-e GOOGLE_CLOUD_PLATFORM_API_KEY=$(GOOGLE_CLOUD_PLATFORM_API_KEY) \
		-e OXFORD_API_ID=$(OXFORD_API_ID) \
		-e OXFORD_API_KEY=$(OXFORD_API_KEY) \
		-t $(NAME)
	cp target/release/$(NAME) bin/$(NAME)

.PHONY: oxford_client
oxford_client:
	docker run --rm -v $(PWD):/local swaggerapi/swagger-codegen-cli generate \
			-i https://developer.oxforddictionaries.com/swagger/spec/public_doc_guest.json \
			-l rust \
			-o /local/oxford_client

.PHONY: clean
clean:
	cargo clean
