import md5 from 'md5'

onmessage = async (e) => {
    const t0 = performance.now();
    const hex = md5(new Uint8Array(await e.data.arrayBuffer()));
    const t1 = performance.now();
    const latency = t1 - t0
    const throughput = Math.floor(e.data.size / 1024 / 1024 / (t1 - t0) * 1000)
    console.log(hex)
    postMessage([hex, latency, throughput]);
};
