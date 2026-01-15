use std::time::Instant;

#[inline(always)]
fn lcg(seed: u32, a: u32, c: u32, m: u32) -> impl Iterator<Item = u32> {
    let mut value = seed;
    std::iter::repeat_with(move || {
        value = value.wrapping_mul(a).wrapping_add(c) % m;
        value
    })
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i32, max_val: i32) -> i32 {
    let a = 1664525u32;
    let c = 1013904223u32;
    let m = 0x1_0000_0000u32;
    let mut lcg_gen = lcg(seed, a, c, m);
    let range = (max_val - min_val + 1) as u32;
    let mut random_numbers = Vec::with_capacity(n);
    for _ in 0..n {
        let num = lcg_gen.next().unwrap() % range + min_val as u32;
        random_numbers.push(num as i32);
    }
    let mut max_sum = i32::MIN;
    for i in 0..n {
        let mut current_sum = 0;
        for j in i..n {
            current_sum += random_numbers[j];
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
    }
    max_sum
}

#[inline(always)]
fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i32, max_val: i32) -> i32 {
    let a = 1664525u32;
    let c = 1013904223u32;
    let m = 0x1_0000_0000u32;
    let mut total_sum = 0;
    let mut lcg_gen = lcg(initial_seed, a, c, m);
    for _ in 0..20 {
        let seed = lcg_gen.next().unwrap();
        total_sum += max_subarray_sum(n, seed, min_val, max_val);
    }
    total_sum
}

fn main() {
    let n = 10000;
    let initial_seed = 42u32;
    let min_val = -10;
    let max_val = 10;
    let start_time = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let elapsed = start_time.elapsed().as_secs_f64();
    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", elapsed);
}

