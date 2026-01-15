use std::time::Instant;

#[inline(always)]
fn lcg_next(state: &mut u32) -> u32 {
    const A: u32 = 1_664_525;
    const C: u32 = 1_013_904_223;
    *state = state.wrapping_mul(A).wrapping_add(C);
    *state
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut state = seed;
    let range = (max_val - min_val + 1) as u32;
    let mut current = i64::MIN;
    let mut best = i64::MIN;
    for _ in 0..n {
        let v = (lcg_next(&mut state) % range) as i32 + min_val;
        let v64 = v as i64;
        current = if current == i64::MIN { v64 } else { (current + v64).max(v64) };
        if current > best {
            best = current;
        }
    }
    best
}

fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i32, max_val: i32) -> i64 {
    let mut state = initial_seed;
    let mut total = 0i64;
    for _ in 0..20 {
        let seed = lcg_next(&mut state);
        total += max_subarray_sum(n, seed, min_val, max_val);
    }
    total
}

fn main() {
    let n = 10_000usize;
    let initial_seed = 42u32;
    let min_val = -10;
    let max_val = 10;

    let start = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let elapsed = start.elapsed().as_secs_f64();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", elapsed);
}

