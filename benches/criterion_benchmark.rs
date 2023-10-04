use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Throughput, Criterion};

use double_buffer::DoubleBuffer;

pub fn criterion_benchmark(c: &mut Criterion) {
    static SIZES: &[usize] = &[
        8, 16, 32, 64, 128, 256, 512, 1024,
        2048, 4096, 8192, 16384, 32768, 65535,
        131072, 262144, 524288, 1048576,
        2097152, 4194304, 8388608, 16777216,
    ];

    criterion_benchmark_buffer(c, SIZES, "swap", |buffer| {
        buffer.swap();
    });

    criterion_benchmark_buffer(c, SIZES, "swap_with_default", |buffer| {
        buffer.swap_with_default();
    });

    criterion_benchmark_buffer(c, SIZES, "swap_with_clone", |buffer| {
        buffer.swap_with_clone();
    });
}

fn criterion_benchmark_buffer(c: &mut Criterion, sizes: &[usize], group_name: &str, iter_fn: fn(&mut DoubleBuffer<Vec<u8>>)) {
    let mut group = c.benchmark_group(group_name);

    for &size in sizes {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let dummy = black_box(vec![0u8; size]);
            let mut buffer = DoubleBuffer::new(
                dummy.clone(),
                dummy
            );

            b.iter(|| iter_fn(black_box(&mut buffer)));
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
