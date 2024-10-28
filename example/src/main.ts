import Worker from './worker.js?worker'
import HashWasmWorker from './hash_wasm_worker.js?worker'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
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
      </tbody>
    </table>
  </div>
`

const file = document.getElementById('file');
const worker = new Worker();
const hashWasmWorker = new HashWasmWorker();


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

if (file) {
  file.addEventListener('change', async (event) => {
    const target = event.target as HTMLInputElement;
    if (target && target.files) {
      worker.postMessage(target.files[0]);
    }
  });
};

if (file) {
  file.addEventListener('change', async (event) => {
    const target = event.target as HTMLInputElement;
    if (target && target.files) {
      hashWasmWorker.postMessage(target.files[0]);
    }
  });
};
