import {MD5, md5_from_file} from "md5gen-wasm";

const file = document.getElementById('file');
file.addEventListener('change', (event) => {
   md5_from_file(event.target.files[0]).then((hex) => {
        console.log(hex); 
   }) 
});
const text = document.getElementById('text');
text.addEventListener('change', (event) => {
    const md5 = new MD5()
    md5.update(event.target.value)
    md5.finalize()
    console.log(md5.digest())
});
