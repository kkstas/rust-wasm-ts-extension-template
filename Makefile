RUST_DIR = ./rust-wasm

wasm-dev:
	wasm-pack build $(RUST_DIR) --out-dir $(RUST_DIR)/target/web --target web --dev

wasm-prod:
	wasm-pack build $(RUST_DIR) --out-dir $(RUST_DIR)/target/web --target web --release

chrome-dev: wasm-dev
	pnpm install
	node esbuild/esbuild.config.mjs --chrome

chrome-watch:
	pnpm install
	node esbuild/esbuild.config.mjs --watch --chrome

chrome-prod: wasm-prod
	pnpm install
	rm -rf dist
	node esbuild/esbuild.config.mjs --prod --chrome

firefox-dev: wasm-dev
	pnpm install
	node esbuild/esbuild.config.mjs --firefox

firefox-watch:
	pnpm install
	node esbuild/esbuild.config.mjs --watch --firefox

firefox-prod: wasm-prod
	pnpm install
	rm -rf dist
	node esbuild/esbuild.config.mjs --prod --firefox


test:
	jest

zip:
	cd dist && zip -r ../extension.zip * && cd ..

clean:
	rm -rf dist extension.zip $(RUST_DIR)/target/web

