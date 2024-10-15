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

## Benchmark
<a href="https://bencher.dev/perf/md5gen-wasm?key=true&reports_per_page=4&branches_per_page=8&testbeds_per_page=8&benchmarks_per_page=8&plots_per_page=8&reports_page=1&branches_page=1&testbeds_page=1&benchmarks_page=1&plots_page=1&report=61ebaffc-dd67-45e2-a33a-38805cb3154a&branches=666f4d69-6bf0-40e0-8650-36f1846fbd2a&heads=0c9312d4-0094-4c10-a565-6486b40df986&testbeds=07f21df8-5f71-4ea2-a330-6f992adaaca4&benchmarks=e1540e1d-c537-4efd-9fc7-12fc88fcd0f2%2C54c2efa0-b09f-4e86-b3bd-823ee0365afe%2C8b893af9-4e16-43d7-aadf-21521ab4af7e%2C68edef88-ab1f-4ae6-a71c-fbd124553a04&measures=a04eb8a7-c6bd-4fa5-825e-af7cdbdb9e21&start_time=1726363759000&end_time=1728955815000&lower_boundary=false&upper_boundary=false&clear=true"><img src="https://api.bencher.dev/v0/projects/md5gen-wasm/perf/img?branches=666f4d69-6bf0-40e0-8650-36f1846fbd2a&heads=0c9312d4-0094-4c10-a565-6486b40df986&testbeds=07f21df8-5f71-4ea2-a330-6f992adaaca4&benchmarks=e1540e1d-c537-4efd-9fc7-12fc88fcd0f2%2C54c2efa0-b09f-4e86-b3bd-823ee0365afe%2C8b893af9-4e16-43d7-aadf-21521ab4af7e%2C68edef88-ab1f-4ae6-a71c-fbd124553a04&measures=a04eb8a7-c6bd-4fa5-825e-af7cdbdb9e21&start_time=1726363759000&end_time=1728955815000" title="md5gen-wasm" alt="md5gen-wasm - Bencher" /></a>