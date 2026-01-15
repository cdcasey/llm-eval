use std::time::Instant;

struct Lcg {
    value: u32,
}

impl Lcg {
    #[inline(always)]
    fn new(seed: u32) -> Self {
        Self { value: seed }
    }
}

impl Iterator for Lcg {
    type Item = u32;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.value = self.value.wrapping_mul(1664525).wrapping_add(1013904223);
        Some(self.value)
    }
}

fn max_subarray_sum(n: usize, seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut lcg = Lcg::new(seed);
    let range = (max_val - min_val + 1) as u32;
    
    let mut random_numbers = Vec::with_capacity(n);
    for _ in 0..n {
        random_numbers.push((lcg.next().unwrap() % range) as i32 + min_val);
    }

    let mut max_sum = i64::MIN;

    for i in 0..n {
        let mut current_sum: i64 = 0;
        for x in &random_numbers[i..] {
            current_sum += *x as i64;
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
    }
    max_sum
}

fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut total_sum = 0;
    let mut lcg_gen = Lcg::new(initial_seed);
    for _ in 0..20 {
        let seed = lcg_gen.next().unwrap();
        total_sum += max_subarray_sum(n, seed, min_val, max_val);
    }
    total_sum
}

fn main() {
    let n = 10000;
    let initial_seed = 42;
    let min_val = -10;
    let max_val = 10;

    let start_time = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let duration = start_time.elapsed();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", duration.as_secs_f64());
}

