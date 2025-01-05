use criterion::{criterion_group, criterion_main, Criterion};
use utils::{run_both_benchmarks_bytes, switch_to_performance_core};

fn bench(c: &mut Criterion) {
    switch_to_performance_core();
    run_both_benchmarks_bytes(9, c, &day9::part1::<false>, &day9::part2::<false>);
}

criterion_group!(benches, bench);
criterion_main!(benches);
