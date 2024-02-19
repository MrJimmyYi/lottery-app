import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { resolve } from "path"; // 引入方法

const pathResolve = (path: string): string => resolve(process.cwd(), path)

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  resolve: {
    alias: {
      '@': pathResolve('src'), // 设置 `@` 指向 `src` 目录
    }
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
