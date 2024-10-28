import Worker from './worker.js?worker'
import HashWasmWorker from './hash_wasm_worker.js?worker'
import Md5Worker from './md5.js?worker'
import CryptoWorker from './crypto.js?worker'
import SuperFastMd5Worker from './super_fast_md5.js?worker'

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
const worker = new Worker();
const hashWasmWorker = new HashWasmWorker();
const md5Worker = new Md5Worker();
const cryptoWorker = new CryptoWorker();
const superFastMd5Worker = new SuperFastMd5Worker();


worker.onmessage = (e) => {
  console.log(e.data);
  document.getElementById('md5gen-wasm-latency')!.textContent = e.data[1];
  document.getElementById('md5gen-wasm-throughput')!.textContent = e.data[2];
};

hashWasmWorker.onmessage = (e) => {
  console.log(e.data);
  document.getElementById('hash-wasm-latency')!.textContent = e.data[1];
  document.getElementById('hash-wasm-throughput')!.textContent = e.data[2];
};

md5Worker.onmessage = (e) => {
  console.log(e.data);
  document.getElementById('md5-latency')!.textContent = e.data[1];
  document.getElementById('md5-throughput')!.textContent = e.data[2];
};

cryptoWorker.onmessage = (e) => {
  console.log(e.data);
  document.getElementById('crypto-latency')!.textContent = e.data[1];
  document.getElementById('crypto-throughput')!.textContent = e.data[2];
};

superFastMd5Worker.onmessage = (e) => {
  console.log(e.data);
  document.getElementById('super-fast-md5-latency')!.textContent = e.data[1];
  document.getElementById('super-fast-md5-throughput')!.textContent = e.data[2];
};

if (file) {
  file.addEventListener('change', async (event) => {
    const target = event.target as HTMLInputElement;
    if (target && target.files) {
      worker.postMessage(target.files[0]);
      hashWasmWorker.postMessage(target.files[0]);
      md5Worker.postMessage(target.files[0]);
      cryptoWorker.postMessage(target.files[0]);
      superFastMd5Worker.postMessage(target.files[0]);
    }
  });
};
