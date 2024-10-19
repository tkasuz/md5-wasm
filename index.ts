import {md5_from_blob, md5_from_string, md5_from_array_buffer} from './rust/pkg/'

async function from_blob(val: Blob): Promise<string> {
    return await md5_from_blob(val);
}

async function from_string(val: string): Promise<string> {
    return md5_from_string(val);
}

async function from_array_buffer(val: ArrayBuffer): Promise<string> {
    return md5_from_array_buffer(val);
}

export {from_blob, from_string, from_array_buffer};
