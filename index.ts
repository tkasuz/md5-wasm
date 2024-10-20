import {md5_from_file, md5_from_string, md5_from_array_buffer} from './rust/pkg/'

async function from_file(val: File): Promise<string> {
    return await md5_from_file(val);
}

async function from_string(val: string): Promise<string> {
    return md5_from_string(val);
}

async function from_array_buffer(val: ArrayBuffer): Promise<string> {
    return md5_from_array_buffer(val);
}

export {from_file, from_string, from_array_buffer};
