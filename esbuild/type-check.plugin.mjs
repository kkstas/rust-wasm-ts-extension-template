import { execSync } from 'child_process';

const log = (...args) => console.log('🔷 [type-check-plugin]', ...args);

/** Custom plugin to run tsc --noEmit before each build */
export const typeCheckPlugin = {
  name: 'type-check-plugin',
  setup(build) {
    build.onStart(() => {
      try {
        log('Running type-checking...');
        execSync('./node_modules/typescript/bin/tsc --noEmit', {
          stdio: 'inherit',
        });
        log('✅ Type-checking passed');
      } catch (err) {
        log('❌ Type-checking failed');
        return { errors: [{ text: 'Type-checking failed, build aborted' }] };
      }
    });
  },
};
