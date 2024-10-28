import {md5_from_file} from '../index';
import { bench, describe } from 'vitest';
import {File} from '@web-std/file';

describe('bench', async () => {
    const one_kb = new File(["a".repeat(1000)], 'test.txt');
    bench('1kb', async () => {
        await md5_from_file(one_kb)
    });

    const one_mb = new File(["a".repeat(1_000_000)], 'test.txt');
    bench('1mb', async () => {
        await md5_from_file(one_mb)
    });

    const handred_mb = new File(["a".repeat(100_000_000)], 'test.txt');
    bench('100mb', async () => {
        await md5_from_file(handred_mb)
    });
})
