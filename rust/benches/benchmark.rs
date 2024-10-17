use md5::MD5;

#[macro_use]
extern crate criterion;

use criterion::{BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    static MB: usize = 1024 * 1024;
    let mut group = c.benchmark_group("data size");
    group.sample_size(10);
    for size in [MB, 10 * MB, 100 * MB].iter() {
        let data = Vec::from_iter(std::iter::repeat(0u8).take(*size));
        group.throughput(criterion::Throughput::Bytes(*size as u64)).sampling_mode(criterion::SamplingMode::Flat);
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            data.as_slice(),
            |b, data| {
                b.iter(|| {
                    let mut md5 = MD5::new();
                    md5.update(data);
                    md5.finalize();
                });
            },
        );
    }
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);