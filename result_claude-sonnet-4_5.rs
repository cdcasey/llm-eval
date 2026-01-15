use std::time::Instant;

struct LCG {
    state: u32,
}

impl LCG {
    fn new(seed: u32) -> Self {
        LCG { state: seed }
    }

    #[inline(always)]
    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut lcg = LCG::new(seed);
    let range = (max_val - min_val + 1) as u32;

    let mut random_numbers = Vec::with_capacity(n);
    for _ in 0..n {
        random_numbers.push((lcg.next() % range) as i32 + min_val);
    }

    let mut max_sum = i64::MIN;
    for i in 0..n {
        let mut current_sum: i64 = 0;
        for j in i..n {
            current_sum += random_numbers[j] as i64;
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
    }
    max_sum
}

fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut total_sum: i64 = 0;
    let mut lcg = LCG::new(initial_seed);

    for _ in 0..20 {
        let seed = lcg.next();
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
    let elapsed = start_time.elapsed();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", elapsed.as_secs_f64());
}
