NAME := trs
NAME_GEN := trs_gen
SERVER := trs_server
BIN := ./target/release/$(NAME)
SRC := $(shell find ./src -type f -name '*.rs')
PWD := $(shell pwd)
GIT_HASH := $(shell git log  --pretty=format:"%H" | head -n1)
GCP_PJ_ID := trslt-165501
GCP_REPO := asia.gcr.io/$(GCP_PJ_ID)/github-kogai-trs:$(GIT_HASH)
GCP_FN_NAME := helloGET
GCP_FN_ENDPOINT := https://us-central1-$(GCP_PJ_ID).cloudfunctions.net/$(GCP_FN_NAME)
GOOGLE_CLOUD_PLATFORM_API_KEY := "${GOOGLE_CLOUD_PLATFORM_API_KEY}"
OXFORD_API_ID := "${OXFORD_API_ID}"
OXFORD_API_KEY := "${OXFORD_API_KEY}"
OS := $(shell uname)
VERSION := $(shell cat Cargo.toml | grep version | sed -e 's/version\ =\ \"\(.*\)\"/\1/')

bin/$(OS)/$(NAME): Cargo.toml $(SRC)
	GCP_FN_ENDPOINT=$(GCP_FN_ENDPOINT ) && \
	cargo build --release
	mkdir -p bin/$(OS)
	cp target/release/$(NAME) bin/$(OS)/$(NAME)

.PHONY: server/build
server/build:
	docker build \
		--build-arg \
			GOOGLE_CLOUD_PLATFORM_API_KEY=$(GOOGLE_CLOUD_PLATFORM_API_KEY) \
		--build-arg \
			OXFORD_API_ID=$(OXFORD_API_ID) \
		--build-arg \
			OXFORD_API_KEY=$(OXFORD_API_KEY) \
		-t $(SERVER) .

.PHONY: server/push
server/push:
	docker tag $(SERVER) $(GCP_REPO)
	gcloud docker -- push $(GCP_REPO)

.PHONY: server/run
server/run: server/build
	docker run -t $(SERVER) -p 3000:3000

.PHONY: hyper/login
hyper/login:
	hyper login \
		-e kogai0121@gmail.com \
		-u oauth2accesstoken \
		-p "$(shell gcloud --project=trs-command auth print-access-token)" \
		https://asia.gcr.io

.PHONY: hyper/create
hyper/create: hyper/login
	hyper pull $(GCP_REPO)
	hyper func create --name $(NAME) $(GCP_REPO)

.PHONY: hyper/rebuild
hyper/rebuild: server/build hyper/login
	hyper func rm $(NAME) $(GCP_REPO)
	hyper pull $(GCP_REPO)
	hyper func create --name $(NAME) $(GCP_REPO)

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

.PHONY: functions
functions:
	gcloud config set project trslt-165501
	gcloud beta functions deploy $(GCP_FN_NAME) --trigger-http
	curl -d "key=value" $(GCP_FN_ENDPOINT)

.PHONY: release
release:
	git tag -af "v${VERSION}" -m ""
	git push
	git push --follow-tags

.PHONY: perf.data
perf.data:
	perf record -g -o ./perf.data -- $(NAME) -t cat is cute
	perf report -g -G

secrets:
	mkdir -p secrets
	ssh-keygen -t rsa -b 4096 -C "kogai0121@gmail.com"
