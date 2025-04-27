RUST_DIR = ./rust-wasm

wasm-dev:
	cd $(RUST_DIR) && wasm-pack build --out-dir target/web --target web --dev

wasm-prod:
	cd $(RUST_DIR) && wasm-pack build --out-dir target/web --target web --release

chrome-dev: wasm-dev
	pnpm install
	node esbuild/esbuild.config.mjs --chrome

chrome-watch: wasm-dev
	pnpm install
	node esbuild/esbuild.config.mjs --watch --chrome

chrome-prod: wasm-prod
	pnpm install
	rm -rf dist
	node esbuild/esbuild.config.mjs --prod --chrome

firefox-dev: wasm-dev
	pnpm install
	node esbuild/esbuild.config.mjs --firefox

firefox-watch: wasm-dev
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

