NAME := trs
NAME_GEN := trs_gen
BIN := ./target/release/$(NAME)
SRC := $(shell find ./src -type f -name '*.rs')
PWD := $(shell pwd)

bin/$(NAME): Cargo.toml $(SRC) oxford-client
	docker build -t $(NAME) .
	docker run --rm -v `pwd`/target:/app/target -t $(NAME)
	cp target/release/$(NAME) bin/$(NAME)

oxford-client:
	docker run --rm -v $(PWD):/local swaggerapi/swagger-codegen-cli generate \
			-i https://developer.oxforddictionaries.com/swagger/spec/public_doc_guest.json \
			-l rust \
			-o /local/oxford-client

.PHONY: clean
clean:
	cargo clean
