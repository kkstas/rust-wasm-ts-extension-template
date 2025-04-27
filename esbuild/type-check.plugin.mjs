import { execSync } from 'child_process';

/** Custom plugin to run tsc --noEmit before each build */
export const typeCheckPlugin = {
  name: 'type-check-plugin',
  setup(build) {
    build.onStart(async () => {
      try {
        console.log('🔍 Running type-checking...');
        execSync('./node_modules/typescript/bin/tsc --noEmit', {
          stdio: 'inherit',
        });
        console.log('✅ Type-checking passed');
      } catch (error) {
        return { errors: [{ text: 'Type-checking failed, build aborted' }] };
      }
    });
  },
};
