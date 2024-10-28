import { defineConfig } from 'vite'

export default defineConfig({
    base: process.env.GITHUB_PAGES ? 'md5-wasm' : './',
    server: {
        headers: {
            "Cross-Origin-Opener-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "credentialless"
        }
    }
})
