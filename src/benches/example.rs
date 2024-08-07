#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn a(bench: &mut Bencher) {
    bench.iter(|| (0..1000).sum::<isize>());
}

fn b(bench: &mut Bencher) {
    const N: usize = 1024;
    bench.iter(|| vec![0_u8; N]);

    bench.bytes = N as u64;
}

benchmark_group!(benches, a, b);
benchmark_main!(benches);
