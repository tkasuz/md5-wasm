import {MD5, lib}from 'crypto-js'

onmessage = async (e) => {
    const t0 = performance.now();

    const wordArray = lib.WordArray.create(await e.data.arrayBuffer());
    const hex = MD5(wordArray).toString();
    const t1 = performance.now();
    const latency = t1 - t0
    const throughput = Math.floor(e.data.size / 1024 / 1024 / (t1 - t0) * 1000)
    console.log(hex)
    postMessage([hex, latency, throughput]);
};
