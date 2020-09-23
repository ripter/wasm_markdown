.PHONY: all build test server share

all: test build server


test:
	cargo test
	wasm-pack test --node

build:
	wasm-pack build --target web
	cp pkg/gfm_bg.wasm public/gfm_bg.wasm
	cp pkg/gfm.js public/gfm.js

server:
	npx http-server

share:
	cp public/gfm_bg.wasm ~/dev/modern-frontend/public/lib/gfm_bg.wasm
	cp public/gfm.js ~/dev/modern-frontend/public/lib/gfm.js
