import Md5GenWasmWorker from './workers/md5gen_wasm.js?worker'
import HashWasmWorker from './workers/hash_wasm.js?worker'
import Md5Worker from './workers/md5.js?worker'
import CryptoWorker from './workers/crypto.js?worker'
import SuperFastMd5Worker from './workers/super_fast_md5.js?worker'

const sleep = (ms: number) => new Promise((res) => setTimeout(res, ms));

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>MD5 Hash Benchmark</h1>
    <div>
      <a href="https://github.com/tkasuz/md5-wasm/tree/main/example">Source code</a>
    </div>
    <input type="file" id="file">
    <table>
      <thead>
        <tr>
          <th scope="col">Library</th>
          <th scope="col">Latency [ms]</th>
          <th scope="col">Throughput [MB/s]</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <th scope="row">md5gen-wasm</th>
          <td id="md5gen-wasm-latency"></td>
          <td id="md5gen-wasm-throughput"></td>
        </tr>
        <tr>
          <th scope="row">hash-wasm</th>
          <td id="hash-wasm-latency"></td>
          <td id="hash-wasm-throughput"></td>
        </tr>
        <tr>
          <th scope="row">md5</th>
          <td id="md5-latency"></td>
          <td id="md5-throughput"></td>
        </tr>
        <tr>
          <th scope="row">crypto-js</th>
          <td id="crypto-latency"></td>
          <td id="crypto-throughput"></td>
        </tr>
        <tr>
          <th scope="row">super-fast-md5</th>
          <td id="super-fast-md5-latency"></td>
          <td id="super-fast-md5-throughput"></td>
        </tr>
      </tbody>
    </table>
  </div>
`

const file = document.getElementById('file');

async function runWorker(worker: Worker, name: string, file: File) {
  await sleep(1000);
  worker.postMessage(file);
  return new Promise((resolve) => {
    worker.onmessage = (event) => {
      console.log(event.data);
      document.getElementById(`${name}-latency`)!.textContent = event.data[1];
      document.getElementById(`${name}-throughput`)!.textContent = event.data[2];
      worker.terminate()
      resolve(event.data);
    };
  });
}

if (file) {
  file.addEventListener('change', async (event) => {
    const target = event.target as HTMLInputElement;
    if (target && target.files) {
      await runWorker(new Md5GenWasmWorker(), "md5gen-wasm", target.files[0])
      await runWorker(new HashWasmWorker(), "hash-wasm", target.files[0])
      await runWorker(new Md5Worker(), "md5", target.files[0])
      await runWorker(new CryptoWorker(), "crypto", target.files[0])
      await runWorker(new SuperFastMd5Worker(), "super-fast-md5", target.files[0])
    }
  });
};
