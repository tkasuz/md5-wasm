import {from_file} from 'md5gen-wasm'

onmessage = async (e) => {
    const t0 = performance.now();
    const hex = await from_file(e.data)
    const t1 = performance.now();
    const latency = t1 - t0
    const throughput = Math.floor(e.data.size / 1024 / 1024 / (t1 - t0) * 1000)
    postMessage([hex, latency, throughput]);
};
