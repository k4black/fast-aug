use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand_pcg::Pcg64;
use rand_xorshift::XorShiftRng;
use instant::Instant;
use rand::seq::SliceRandom;


fn main() {
    let iterations = 10_000;
    let vector_size = 1_000;
    let sample_vector: Vec<u32> = (0..vector_size as u32).collect();

    benchmark_rng(&mut SmallRng::from_entropy(), "SmallRng", &sample_vector, iterations);
    benchmark_rng(&mut rand::thread_rng(), "ThreadRng", &sample_vector, iterations);
    benchmark_rng(&mut Pcg64::from_entropy(), "Pcg64", &sample_vector, iterations);
    benchmark_rng(&mut XorShiftRng::from_entropy(), "XorShiftRng", &sample_vector, iterations);
}

fn benchmark_rng(rng: &mut impl Rng, rng_name: &str, sample_vector: &Vec<u32>, iterations: u32) {
    let start_generation = Instant::now();
    for _ in 0..iterations {
        let _: u32 = rng.gen();
    }
    let duration_generation = start_generation.elapsed();

    let start_selection = Instant::now();
    for _ in 0..iterations {
        let _: Vec<u32> = sample_vector
            .as_slice()
            .choose_multiple(rng, sample_vector.len() * 30 / 100)
            .cloned()
            .collect();
    }
    let duration_selection = start_selection.elapsed();

    println!("{}:\n  Gen Time: {:?}\n  Vec Time: {:?}", rng_name, duration_generation, duration_selection);
}
