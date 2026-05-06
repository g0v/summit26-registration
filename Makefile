CONFIG ?= config.toml
SERVER_BIND_HOST := $(shell awk -F' *= *' 'BEGIN { section = "" } /^\[server\]/ { section = "server"; next } /^\[/ { section = ""; next } section == "server" && ($$1 == "bind_host" || $$1 == "host") { gsub(/"/, "", $$2); print $$2; exit }' $(CONFIG))
SERVER_PORT := $(shell awk -F' *= *' 'BEGIN { section = "" } /^\[server\]/ { section = "server"; next } /^\[/ { section = ""; next } section == "server" && $$1 == "port" { gsub(/"/, "", $$2); print $$2; exit }' $(CONFIG))
LOCAL_BACKEND_HOST := $(shell printf '%s\n' "$(SERVER_BIND_HOST)" | awk '{ if ($$0 == "" || $$0 == "0.0.0.0" || $$0 == "::") print "127.0.0.1"; else print $$0 }')
LOCAL_BACKEND_URL ?= http://$(LOCAL_BACKEND_HOST):$(SERVER_PORT)

.PHONY: build deploy-build serve run-server check fmt-check check-server check-wasm print-config

build:
	trunk build

deploy-build:
	trunk build

serve:
	APP_CONFIG=$(CONFIG) APP__FRONTEND__BACKEND_PUBLIC_URL=$(LOCAL_BACKEND_URL) trunk serve --address 127.0.0.1 --port 8080

run-server:
	APP_CONFIG=$(CONFIG) cargo run --features server --bin server

check:
	cargo fmt --check
	cargo check --features server --bin server
	cargo check --target wasm32-unknown-unknown

fmt-check:
	cargo fmt --check

check-server:
	cargo check --features server --bin server

check-wasm:
	cargo check --target wasm32-unknown-unknown

print-config:
	@echo "CONFIG=$(CONFIG)"
	@echo "SERVER_BIND_HOST=$(SERVER_BIND_HOST)"
	@echo "SERVER_PORT=$(SERVER_PORT)"
	@echo "LOCAL_BACKEND_URL=$(LOCAL_BACKEND_URL)"
