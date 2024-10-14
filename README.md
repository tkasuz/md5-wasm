# md5gen-wasm 🦀

`md5gen-wasm` is a WebAssembly (Wasm) library for generating MD5 hashes. This project leverages Rust and Wasm to provide a fast and efficient way to compute MD5 hashes directly in the browser or other Wasm-supported environments.

## Features

- **Fast MD5 Hashing**: Compute MD5 hashes quickly using Rust's performance and Wasm's portability.
- **WebAssembly**: Run in any environment that supports WebAssembly, including modern web browsers.
- **Easy Integration**: Simple API for integrating MD5 hashing into your web applications.

## Installation

To use `md5gen-wasm` in your project, you can install it via npm:

```sh
npm install md5gen-wasm
```

## Usage
Here's a quick example of how to use md5gen-wasm to compute MD5 hashes in a web application:

```js
import { md5_from_file } from 'md5gen-wasm';

async function computeMD5(file) {
  const result = await md5_from_file(file);
  console.log(`MD5 hash: ${result}`);
}

// Example usage with a file input
document.getElementById('fileInput').addEventListener('change', (event) => {
  const file = event.target.files[0];
  computeMD5(file);
});
```