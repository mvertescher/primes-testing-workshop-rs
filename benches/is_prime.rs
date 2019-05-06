#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use primal;
use primes;
use primes_testing_workshop as my;

fn my_is_prime_benchmark(c: &mut Criterion) {
    c.bench_function("my::is_prime 999983", |b| b.iter(||
		my::is_prime(black_box(999983))
	));
}

fn primes_is_prime_benchmark(c: &mut Criterion) {
    c.bench_function("primes::is_prime 999983", |b| b.iter(||
		primes::is_prime(black_box(999983))
	));
}

// Below is fast because a deterministic version of the Miller-Rabin test is
// used.
fn primal_is_prime_benchmark(c: &mut Criterion) {
    c.bench_function("primal::is_prime 999983", |b| b.iter(||
		primal::is_prime(black_box(999983))
	));
}

criterion_group!(is_prime_benches,
	my_is_prime_benchmark,
	primes_is_prime_benchmark,
	primal_is_prime_benchmark,
);
criterion_main!(is_prime_benches);
