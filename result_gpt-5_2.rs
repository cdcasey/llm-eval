use std::time::Instant;

#[inline(always)]
fn lcg_step(value: u32) -> u32 {
    // Mod 2^32 via wrapping arithmetic
    value
        .wrapping_mul(1_664_525u32)
        .wrapping_add(1_013_904_223u32)
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i32, max_val: i32) -> i64 {
    let range = (max_val - min_val + 1) as u32;

    let mut v = seed;
    let mut best: i64 = i64::MIN;
    let mut cur: i64 = 0;

    for _ in 0..n {
        v = lcg_step(v);
        let x = ((v % range) as i32 + min_val) as i64;

        let cur_plus = cur + x;
        cur = if x > cur_plus { x } else { cur_plus };
        if cur > best {
            best = cur;
        }
    }

    best
}

#[inline(always)]
fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut total: i64 = 0;
    let mut v = initial_seed;

    for _ in 0..20 {
        v = lcg_step(v);
        total += max_subarray_sum(n, v, min_val, max_val);
    }

    total
}

fn main() {
    let n: usize = 10_000;
    let initial_seed: u32 = 42;
    let min_val: i32 = -10;
    let max_val: i32 = 10;

    let start = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let elapsed = start.elapsed().as_secs_f64();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", elapsed);
}

