use bencher::{benchmark_group, benchmark_main, Bencher};

use distance_metrics::{distance,FloatArray, distance_no_simd, distance_slice};
use rand::{rngs::StdRng, SeedableRng, Rng};

benchmark_main!(benches);
benchmark_group!(benches, distance_measurement, distance_measurement_no_simd, distance_measurement_slice);

fn distance_measurement(bench: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let point_a = FloatArray([rng.gen(); 300]);
    let point_b = FloatArray([rng.gen(); 300]);

    bench.iter(|| distance(&point_a, &point_b))
}

fn distance_measurement_no_simd(bench: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let point_a = FloatArray([rng.gen(); 300]);
    let point_b = FloatArray([rng.gen(); 300]);

    bench.iter(|| distance_no_simd(&point_a, &point_b))
}

fn distance_measurement_slice(bench: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(SEED);
    let point_a = [rng.gen(); 300];
    let point_b = [rng.gen(); 300];

    bench.iter(|| distance_slice(&point_a, &point_b))
}

const SEED: u64 = 123456789;