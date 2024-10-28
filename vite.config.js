import { defineConfig } from "vite";
import dts from "vite-plugin-dts";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { resolve } from "path";
import codspeedPlugin from "@codspeed/vitest-plugin";

export default defineConfig({
  test: {
    include: ['tests/**/*.test.ts'],
    benchmark: {
      include: ['tests/**/*.bench.ts']
    },
  },
  plugins: [
    dts(),
    wasm(),
    topLevelAwait(),
    codspeedPlugin()
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
