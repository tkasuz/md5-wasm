import {md5_from_file} from '../index';
import { expect, test } from 'vitest';
import {File} from '@web-std/file';

test('empty', async () => {
    const file = new File([], 'test.txt');
    expect(await md5_from_file(file)).toBe("d41d8cd98f00b204e9800998ecf8427e");
})

test('a', async () => {
    const file = new File(["a"], 'test.txt');
    expect(await md5_from_file(file)).toBe("0cc175b9c0f1b6a831c399e269772661");
})

test('abc', async () => {
    const file = new File(["abc"], 'test.txt');
    expect(await md5_from_file(file)).toBe("900150983cd24fb0d6963f7d28e17f72");
})

test('message digest', async () => {
    const file = new File(["message digest"], 'test.txt');
    expect(await md5_from_file(file)).toBe("f96b697d7cb7938d525a2f31aaf161d0");
})

test('short string', async () => {
    const file = new File(["abcdefghijklmnopqrstuvwxyz"], 'test.txt');
    expect(await md5_from_file(file)).toBe("c3fcd3d76192e4007dfb496cca67e13b");
})

test('long string1', async () => {
    const file = new File(["ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"], 'test.txt');
    expect(await md5_from_file(file)).toBe("d174ab98d277d9f5a5611c2c9f419d9f");
})

test('long string2', async () => {
    const file = new File(["12345678901234567890123456789012345678901234567890123456789012345678901234567890"], 'test.txt');
    expect(await md5_from_file(file)).toBe("57edf4a22be3c955ac49da2e2107b67a");
})

test('long string2', async () => {
    const file = new File(["12345678901234567890123456789012345678901234567890123456789012345678901234567890"], 'test.txt');
    expect(await md5_from_file(file)).toBe("57edf4a22be3c955ac49da2e2107b67a");
})
