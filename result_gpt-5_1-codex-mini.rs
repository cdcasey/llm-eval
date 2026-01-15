use std::time::Instant;

const N: usize = 10_000;
const INITIAL_SEED: u32 = 42;
const MIN_VAL: i32 = -10;
const MAX_VAL: i32 = 10;
const RUNS: usize = 20;
const LCG_A: u32 = 1_664_525;
const LCG_C: u32 = 1_013_904_223;

#[repr(transparent)]
struct Lcg(u32);

impl Lcg {
    #[inline(always)]
    fn new(seed: u32) -> Self {
        Self(seed)
    }

    #[inline(always)]
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(LCG_A).wrapping_add(LCG_C);
        self.0
    }
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i32, max_val: i32) -> i64 {
    let range = (max_val - min_val) as u32 + 1;
    let mut generator = Lcg::new(seed);
    let mut values = Vec::with_capacity(n);
    for _ in 0..n {
        let value = generator.next();
        values.push(((value % range) as i32) + min_val);
    }

    let mut max_sum = i64::MIN;
    for i in 0..n {
        let mut current_sum = 0i64;
        for &value in &values[i..] {
            current_sum += value as i64;
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
    }
    max_sum
}

fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut generator = Lcg::new(initial_seed);
    let mut total = 0i64;
    for _ in 0..RUNS {
        let seed = generator.next();
        total += max_subarray_sum(n, seed, min_val, max_val);
    }
    total
}

fn main() {
    let start = Instant::now();
    let result = total_max_subarray_sum(N, INITIAL_SEED, MIN_VAL, MAX_VAL);
    let duration = start.elapsed().as_secs_f64();
    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", duration);
}

