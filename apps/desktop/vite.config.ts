import { defineConfig } from 'vite';
import tsconfigPaths from 'vite-tsconfig-paths';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    port: 8001
  },
  plugins: [
    react({
      jsxRuntime: 'classic'
    })
  ],
  root: 'src',
  publicDir: '../../packages/interface/src/assets',
  build: {
    outDir: '../dist',
    assetsDir: '.'
  }
});
