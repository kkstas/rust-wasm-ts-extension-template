import * as esbuild from 'esbuild';
import copy from 'esbuild-plugin-copy';
import { typeCheckPlugin } from './type-check.plugin.mjs';

const isWatchMode = process.argv.includes('--watch');
const isProd = process.argv.includes('--prod');

const isFirefox = process.argv.includes('--firefox');
const isChrome = process.argv.includes('--chrome');

const manifestFrom = (() => {
  if (isFirefox) return 'manifests/firefox.json';
  if (isChrome) return 'manifests/chrome.json';
  throw new Error(
    'Browser target not specified. Use flag (e.g. --chrome) to specify browser target.',
  );
})();

const context = await esbuild.context({
  entryPoints: [
    'src/background/main.ts',
    'src/content/main.ts',
    'src/popup/main.ts',
    'src/popup/popup.html',
  ],
  outdir: 'dist',
  bundle: true,
  minify: isProd,
  sourcemap: !isProd,
  treeShaking: true,
  target: 'chrome110',
  supported: {
    'top-level-await': true,
  },
  platform: 'browser',
  format: 'esm',
  tsconfig: 'tsconfig.json',
  loader: {
    '.ts': 'ts',
    '.json': 'json',
    '.html': 'copy',
    '.wasm': 'copy',
  },
  plugins: [
    typeCheckPlugin,

    copy({
      resolveFrom: 'cwd',
      assets: {
        from: [manifestFrom],
        to: ['dist/manifest.json'],
      },
      watch: true,
    }),
    copy({
      resolveFrom: 'cwd',
      assets: {
        from: ['src/assets/**/*'],
        to: ['dist/assets'],
      },
      watch: true,
    }),
    copy({
      resolveFrom: 'cwd',
      assets: {
        from: ['rust-wasm/target/web/rust_wasm_bg.wasm'],
        to: ['dist/wasm/rust_wasm_bg.wasm'],
      },
      watch: true,
    }),
  ],
});

console.log(`🔧 Build mode: ${isProd ? 'Production' : 'Development'}`);
console.log(`👀 Watching: ${isWatchMode ? 'Yes' : 'No'}`);

if (isWatchMode) {
  console.log('👁️  Watching for file changes...');
  await context.watch();
} else {
  console.log('📦 Building...');
  await context.rebuild();
  await context.dispose();
}
