use std::time::Instant;

struct Lcg {
    value: u32,
}

impl Lcg {
    #[inline(always)]
    fn new(seed: u32) -> Self {
        Self { value: seed }
    }

    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.value = self
            .value
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        self.value
    }
}

fn max_subarray_sum(n: usize, seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut lcg = Lcg::new(seed);
    let range = (max_val - min_val + 1) as u32;
    let mut random_numbers = Vec::with_capacity(n);
    for _ in 0..n {
        let value = (lcg.next_u32() % range) as i32 + min_val;
        random_numbers.push(value);
    }

    let mut max_sum = i64::MIN;
    for i in 0..n {
        let mut current_sum = 0i64;
        for &val in &random_numbers[i..] {
            current_sum += val as i64;
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
    }
    max_sum
}

fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut lcg = Lcg::new(initial_seed);
    let mut total_sum = 0i64;
    for _ in 0..20 {
        let seed = lcg.next_u32();
        total_sum += max_subarray_sum(n, seed, min_val, max_val);
    }
    total_sum
}

fn main() {
    let n = 10_000usize;
    let initial_seed = 42u32;
    let min_val = -10i32;
    let max_val = 10i32;

    let start = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let duration = start.elapsed();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", duration.as_secs_f64());
}

