use std::time::Instant;

struct Lcg {
    state: u32,
}

impl Lcg {
    #[inline(always)]
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        const A: u32 = 1_664_525;
        const C: u32 = 1_013_904_223;
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut lcg = Lcg::new(seed);
    let range = (max_val - min_val + 1) as u32;

    let mut random_numbers: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let v = (lcg.next_u32() % range) as i32 + min_val;
        random_numbers.push(v);
    }

    let mut max_sum: i64 = i64::MIN;
    let slice = random_numbers.as_slice();

    for i in 0..n {
        let mut current_sum: i64 = 0;
        for &v in &slice[i..] {
            current_sum += v as i64;
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
    }

    max_sum
}

#[inline(always)]
fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut total_sum: i64 = 0;
    let mut lcg = Lcg::new(initial_seed);

    for _ in 0..20 {
        let seed = lcg.next_u32();
        total_sum += max_subarray_sum(n, seed, min_val, max_val);
    }

    total_sum
}

fn main() {
    let n: usize = 10_000;
    let initial_seed: u32 = 42;
    let min_val: i32 = -10;
    let max_val: i32 = 10;

    let start_time = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let elapsed = start_time.elapsed();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", elapsed.as_secs_f64());
}

