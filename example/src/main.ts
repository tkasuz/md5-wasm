import {from_blob, from_string} from 'md5gen-wasm'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <input type="file" id="file"> 
  </div>
`

const file = document.getElementById('file');
if (file) {
  file.addEventListener('change', async (event) => {
    const target = event.target as HTMLInputElement;
    if (target && target.files) {
      console.log(target.files[0]);
      console.log(await from_blob(target.files[0]));
      console.log(await from_string('hello world'));
    }
  });
};