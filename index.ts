import {md5_from_u8_array as compute} from './rust/pkg/'

type Input = File | Blob | string | ArrayBuffer | Uint8Array | object;

export function md5_from_u8_array(input: Uint8Array): string {
    return compute(input);
}

export function md5_from_string(input: string): string {
    return compute(new TextEncoder().encode(input));
}

export function md5_from_array_buffer(input: ArrayBuffer): string {
    return compute(new Uint8Array(input));
}

export async function md5_from_file(input: File): Promise<string> {
    const buffer = await input.arrayBuffer();
    return compute(new Uint8Array(buffer));
}

export async function md5(input: Input): Promise<string> {
    if (input instanceof File) {
        return md5_from_file(input);
    } else if (input instanceof ArrayBuffer) {
        return md5_from_array_buffer(input);
    } else if (input instanceof Uint8Array) {
        return md5_from_u8_array(input);
    } else if (typeof input === 'string') {
        return md5_from_string(input);
    }
    throw new Error(`Invalid input type: ${typeof input}. Expected File, string, ArrayBuffer, or Uint8Array.`)
}
