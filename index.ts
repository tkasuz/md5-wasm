import {md5_from_file, md5_from_string} from './rust/pkg/'

async function from_file(file: File): Promise<string> {
    return await md5_from_file(file);
}

async function from_string(val: string): Promise<string> {
    return md5_from_string(val);
}

export {from_file, from_string};
