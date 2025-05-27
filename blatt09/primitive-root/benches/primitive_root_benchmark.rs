use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use primitive_root::{primitive_root_brute_force, primitive_root_lagrange};

pub fn bench_primitive_roots(c: &mut Criterion) {
    let primes = [223, 1033, 2239, 5179, 7919, 22447, 73907, 111799, 153913];
    let mut group = c.benchmark_group("Primitive Roots");
    for p in primes {
        group.bench_with_input(BenchmarkId::new("Brute Force", p), &p, |b, p| {
            b.iter(|| primitive_root_brute_force(*p))
        });
        group.bench_with_input(BenchmarkId::new("Lagrange", p), &p, |b, p| {
            b.iter(|| primitive_root_lagrange(*p))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_primitive_roots);
criterion_main!(benches);
