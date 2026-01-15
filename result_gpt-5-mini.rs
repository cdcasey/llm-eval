use std::time::Instant;
use std::num::Wrapping;

#[inline(always)]
struct Lcg {
    state: Wrapping<u32>,
}

impl Lcg {
    #[inline(always)]
    fn new(seed: u32) -> Self {
        Lcg { state: Wrapping(seed) }
    }

    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        // Constants chosen to match the Python implementation; arithmetic wraps at 2^32
        const A: Wrapping<u32> = Wrapping(1664525);
        const C: Wrapping<u32> = Wrapping(1013904223);
        self.state = A * self.state + C;
        self.state.0
    }
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i64, max_val: i64) -> i64 {
    // Generate n pseudorandom values in [min_val, max_val] using an LCG seeded with `seed`
    let mut lcg = Lcg::new(seed);
    let range = (max_val - min_val + 1) as u64;
    let mut nums: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let r = lcg.next_u32() as u64;
        nums.push((r % range) as i64 + min_val);
    }

    // Kadane's algorithm (O(n)) for maximum subarray sum
    if nums.is_empty() {
        return 0;
    }
    let mut max_ending_here = nums[0];
    let mut max_so_far = nums[0];
    for &v in nums.iter().skip(1) {
        let candidate = max_ending_here + v;
        max_ending_here = if v > candidate { v } else { candidate };
        if max_ending_here > max_so_far {
            max_so_far = max_ending_here;
        }
    }
    max_so_far
}

#[inline(always)]
fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i64, max_val: i64) -> i64 {
    let mut total: i64 = 0;
    let mut lcg = Lcg::new(initial_seed);
    for _ in 0..20 {
        let seed = lcg.next_u32();
        total += max_subarray_sum(n, seed, min_val, max_val);
    }
    total
}

fn main() {
    // Parameters matching the provided Python code
    let n: usize = 10000;
    let initial_seed: u32 = 42;
    let min_val: i64 = -10;
    let max_val: i64 = 10;

    let start = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let duration = start.elapsed();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", duration.as_secs_f64());
}

