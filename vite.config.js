import { defineConfig } from "vite";
import dts from "vite-plugin-dts";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { resolve } from "path";

export default defineConfig({
  plugins: [
    dts(),
    wasm(),
    topLevelAwait()
  ],
  build: {
    lib: {
      entry: resolve(__dirname, "index.ts"),
      name: "md5gen-wasm", 
      fileName: "index",
      formats: ["es"],
    },
  },
});