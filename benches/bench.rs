use criterion::{criterion_group, criterion_main, Criterion};
use rdu::get_disk_usage;
use std::path::PathBuf;

/// Benchmark allocating space for disk usage on a single thread. Do not benchmark printing, since that
/// is dependent on the machine and not code optimizations.
fn get_disk_usage_single_thread(c: &mut Criterion) {
    c.bench_function("Disk Usage On Single Thread", |b| {
        b.iter(|| get_disk_usage(PathBuf::from("./"), u16::MAX))
    });
}

criterion_group!(benches, get_disk_usage_single_thread);
criterion_main!(benches);
