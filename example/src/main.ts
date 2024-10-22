import Worker from './worker.js?worker'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <input type="file" id="file">
  </div>
`

const file = document.getElementById('file');
const worker = new Worker();

worker.onmessage = (e) => {
  console.log(e.data);
};

if (file) {
  file.addEventListener('change', async (event) => {
    const target = event.target as HTMLInputElement;
    if (target && target.files) {
      worker.postMessage(target.files[0]);
    }
  });
};
