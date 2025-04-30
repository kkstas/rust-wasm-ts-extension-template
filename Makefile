RUST_DIR = ./rust-wasm
TS_NODE_BIN = ./node_modules/.bin/ts-node

wasm-dev:
	wasm-pack build $(RUST_DIR) --out-dir target/web --target web --dev

wasm-prod:
	wasm-pack build $(RUST_DIR) --out-dir target/web --target web --release

chrome-dev: wasm-dev
	pnpm install
	$(TS_NODE_BIN) --project ./esbuild/tsconfig.esbuild.json esbuild/esbuild.config.ts --chrome

chrome-watch:
	pnpm install
	$(TS_NODE_BIN) --project ./esbuild/tsconfig.esbuild.json esbuild/esbuild.config.ts --chrome --watch

chrome-prod: wasm-prod
	pnpm install
	rm -rf dist
	$(TS_NODE_BIN) --project ./esbuild/tsconfig.esbuild.json esbuild/esbuild.config.ts --chrome --prod

firefox-dev: wasm-dev
	pnpm install
	$(TS_NODE_BIN) --project ./esbuild/tsconfig.esbuild.json esbuild/esbuild.config.ts --firefox

firefox-watch:
	pnpm install
	$(TS_NODE_BIN) --project ./esbuild/tsconfig.esbuild.json esbuild/esbuild.config.ts --firefox --watch

firefox-prod: wasm-prod
	pnpm install
	rm -rf dist
	$(TS_NODE_BIN) --project ./esbuild/tsconfig.esbuild.json esbuild/esbuild.config.ts --firefox --prod

zip:
	cd dist && zip -r ../extension.zip * && cd ..

clean:
	rm -rf dist extension.zip $(RUST_DIR)/target/web

