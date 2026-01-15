use std::time::Instant;

#[inline(always)]
fn lcg_next(value: u32) -> u32 {
    value.wrapping_mul(1664525).wrapping_add(1013904223)
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i64, max_val: i64) -> i64 {
    let range = (max_val - min_val + 1) as u32;
    let mut random_numbers: Vec<i64> = Vec::with_capacity(n);
    let mut value = seed;

    for _ in 0..n {
        value = lcg_next(value);
        random_numbers.push((value % range) as i64 + min_val);
    }

    let mut max_sum = i64::MIN;
    let mut i = 0;
    while i < n {
        let mut current_sum: i64 = 0;
        let mut j = i;
        while j < n {
            current_sum += unsafe { *random_numbers.get_unchecked(j) };
            if current_sum > max_sum {
                max_sum = current_sum;
            }
            j += 1;
        }
        i += 1;
    }
    max_sum
}

fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i64, max_val: i64) -> i64 {
    let mut total_sum: i64 = 0;
    let mut value = initial_seed;

    for _ in 0..20 {
        value = lcg_next(value);
        total_sum += max_subarray_sum(n, value, min_val, max_val);
    }
    total_sum
}

fn main() {
    let n: usize = 10000;
    let initial_seed: u32 = 42;
    let min_val: i64 = -10;
    let max_val: i64 = 10;

    let start_time = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let elapsed = start_time.elapsed();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", elapsed.as_secs_f64());
}
