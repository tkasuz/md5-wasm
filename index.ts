import {Md5} from './rust/pkg/'

/**
 * Calculate MD5 hash from File or Blob
 * @param input File or Blob
 * @returns MD5 hash
 */
export async function md5_from_file(input: File | Blob): Promise<string> {
    const md5 = new Md5();
    const reader = input.stream().getReader();

    while (true) {
        const {value, done } = await reader.read();
        if (done) {
            md5.finalize();
            break;
        } else {
            md5.update(value);
        }
    }
    return md5.digest();
}
