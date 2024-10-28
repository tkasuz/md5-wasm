# md5gen-wasm 🦀

![NPM Version](https://img.shields.io/npm/v/md5gen-wasm)
![NPM Unpacked Size](https://img.shields.io/npm/unpacked-size/md5gen-wasm)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/tkasuz/md5-wasm/pr-checks.yml)
[![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/dwyl/esta/issues)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`md5gen-wasm` is a lightning-fast ⚡️ WebAssembly library designed for generating MD5 hashes. Built with Rust 🦀 and Wasm, this project offers a highly efficient way to compute MD5 hashes directly in the browser or any Wasm-supported environment..

## Features

- **Fastest MD5 Hashing**:  Based on [benchmark tests](./example/) done in example applications, md5gen-wasm is the fastest MD5 hashing library available.
- **Efficient Memory Usage**: Automatically chunks the given file to manage memory consumption efficiently.
- **WebAssembly**: Run in any environment that supports WebAssembly, including modern web browsers.

## Installation

To use `md5gen-wasm` in your project, you can install it via npm:

```sh
npm install md5gen-wasm
```

## Usage
Here's a quick example of how to use md5gen-wasm to compute MD5 hashes in a web application:

```typescript
import { md5_from_file } from 'md5gen-wasm';

// Example usage with a file input
document.getElementById('fileInput')?.addEventListener('change', async (event: Event) => {
  const input = event.target as HTMLInputElement;
  if (input.files && input.files.length > 0) {
    const file = input.files[0];
    const result = await md5_from_file(file);
    console.log(`MD5 hash: ${result}`);
  }
});;
```

## Contributing
Contributions are welcome! Please open an issue or submit a pull request on GitHub.
