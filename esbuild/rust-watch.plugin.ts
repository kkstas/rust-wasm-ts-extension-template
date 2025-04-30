import { execSync } from 'child_process';
import * as path from 'path';
import { watch } from 'fs';
import type { Plugin } from 'esbuild';

const log = (...args: any[]): void =>
  console.log('🦀 [rust-watch-plugin]', ...args);

/**  Custom plugin for rebuilding WASM when Rust files change */
export const rustWatchPlugin: Plugin = {
  name: 'rust-watch-plugin',
  setup(_) {
    const isWatchMode = process.argv.includes('--watch');
    if (!isWatchMode) return;

    log('Building Rust WASM at startup...');
    try {
      execSync('make wasm-dev', { stdio: 'inherit' });
      log('✅ Rust WASM build at startup complete');
    } catch (_) {
      log('❌ Rust WASM build failed');
    }

    if (isWatchMode) {
      const rustDir = path.join(process.cwd(), 'rust-wasm/src');

      let debounceTimer: NodeJS.Timeout | null = null;
      let isBuilding = false;

      const watcher = watch(rustDir, { recursive: true }, (_, filename) => {
        if (filename && filename.endsWith('.rs') && !isBuilding) {
          if (debounceTimer) {
            clearTimeout(debounceTimer);
          }

          debounceTimer = setTimeout(() => {
            log(`Rust file changed: ${filename}, rebuilding WASM...`);
            isBuilding = true;

            try {
              execSync('make wasm-dev', { stdio: 'inherit' });
              log('✅ Rust WASM rebuild complete');
            } catch (_) {
              log('❌ Rust WASM rebuild failed');
            } finally {
              isBuilding = false;
            }

            debounceTimer = null;
          }, 300);
        }
      });

      process.on('SIGINT', () => {
        if (debounceTimer) {
          clearTimeout(debounceTimer);
        }
        watcher.close();
        process.exit(0);
      });
    }
  },
};
