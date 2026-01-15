I adapted an exercise from one of Ed Donner's Udemy courses to compare models on Github Copilot. After the shell script was complete, I ran it to generate the Rust code. I then used Claude code run the generated binaries and collect the output into this file. I then asked Opus to look at each file to evaluate the solution. I also looked at the Rust files, but Opus knows way more about Rust than I do.

I'm not sure why some of the models have `cache read` stats and others don't. I don't recall running the command with those models multiple times.

My Copilot quota usage went from 8% to 12% genrating the Rust files.

Rust files were compiled and run on an M1 Macbook Pro.

I am currently running `rustc 1.67.1` (I don't do a lot of Rust development).

## The evaluation according to Opus:

  The Key Difference: Algorithm Choice

  The Python baseline uses an O(n²) brute-force algorithm for max subarray sum:
  for i in range(n):
      for j in range(i, n):
          current_sum += random_numbers[j]
  This checks every possible subarray - 50 million operations per run.

  The "60x faster" models (all Claude models, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gemini-3) did a direct translation to Rust, keeping the O(n²) algorithm. The 60x speedup is just Rust being faster than Python.

  The "100,000x+ faster" models (gpt-5, gpt-5.2, gpt-5.1-codex-max) recognized this is a classic problem and used Kadane's algorithm - an O(n) solution:
  current = (current + v).max(v);
  max_sum = max_sum.max(current);
  This is a single pass through the array. The speedup isn't 100,000x over Rust - it's the algorithmic improvement (10,000x from O(n²) → O(n)) multiplied by the language speedup.

  Verdict

  All implementations are correct - they produce the same answer (10980). The difference is:
  Approach: Direct translation
  Models: Claude (all), gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gemini-3
  Complexity: O(n²)
  What they did: Faithfully ported the Python

  Approach: Algorithmic optimization
  Models: gpt-5, gpt-5.2, gpt-5.1-codex-max
  Complexity: O(n)
  What they did: Recognized the problem and applied Kadane's
  Neither approach is "wrong" - it depends on what you asked for. If you asked for a direct port, Claude models were more faithful. If you asked for an optimized version, the GPT models that used Kadane's were smarter about it.

  What was the original prompt you gave these models?

❯ check @eval.sh
  ⎿  Read eval.sh (92 lines)

⏺ The prompt says:

  Task: Port this Python code to ultra-high-performance Rust.

  So the GPT models that used Kadane's algorithm (gpt-5, gpt-5.2, gpt-5.1-codex-max) actually followed the instructions better. You asked for "ultra-high-performance" - they delivered by recognizing the O(n²) algorithm and replacing it with the O(n) Kadane's algorithm.

  The Claude models and others interpreted "port" more literally and preserved the original algorithm structure, just translating syntax. They're "correct" translations but missed the optimization opportunity that was explicitly requested.

  Bottom line: This is actually a good eval. The models that applied Kadane's algorithm demonstrated:
  1. Problem recognition (max subarray sum is a classic CS problem)
  2. Knowledge of optimal solutions
  3. Willingness to deviate from literal translation when asked for "ultra-high-performance"

  The 60x models did a competent translation. The 100,000x+ models understood the actual goal.

  You might want to add a column to your table indicating which approach each model took - it's a more meaningful distinction than raw speedup numbers.
  
## Regarding the failures:

gpt-5-mini (line 5):
  #[inline(always)]
  struct Lcg {
  You can't put #[inline(always)] on a struct - it only applies to functions. The model confused where to place the attribute. The code itself is otherwise fine and even uses Kadane's algorithm.

  gpt-4.1 (line 16):
  let m = 0x1_0000_0000u32;
  This is 2^32 which equals 4,294,967,296 - but u32::MAX is 4,294,967,295. The value overflows a u32. The model tried to be clever with hex notation but didn't realize it can't fit.

  Also line 21 has a type issue:
  let num = lcg_gen.next().unwrap() % range + min_val as u32;
  Adding a signed min_val (-10) cast to u32 produces garbage.

  ---
  Summary of failures:

| Model | Error | Root Cause |
|-|-|-|
| gpt-5-mini | #[inline(always)] on struct | Misplaced attribute |
| gpt-4.1 | 0x1_0000_0000u32 overflow | 2^32 doesn't fit in u32 |
  
  Both are basic Rust knowledge gaps. Interestingly, gpt-5-mini would have been in the fast tier (it used Kadane's) if it had compiled.
  
## The stats

| Model | Premium Requests | Wall Time | API Time | Input Tokens | Output Tokens | Cache Read | Compiles? | Approach | Exec Time | xSpeedup |
|-------|------------------|-----------|----------|--------------|---------------|------------|-----------|----------|-----------|----------|
| gpt-5.1-codex-max | 1 | 18s | 15s | 15.3k | 1.0k | 1.8k | ✅ | O(n) Kadane's | 0.000311s | 122,081x |
| gpt-5.2 | 1 | 29s | 26s | 14.5k | 1.2k | 0 | ✅ | O(n) Kadane's | 0.000329s | 115,403x |
| gpt-5 | 1 | 20s | 17s | 15.3k | 1.3k | 10.0k | ✅ | O(n) Kadane's | 0.000340s | 111,669x |
| claude-sonnet-4.5 | 1 | 10s | 7s | 20.5k | 640 | 0 | ✅ | O(n²) direct port | 0.637s | 60x |
| claude-opus-4.5 | 3 | 23s | 20s | 41.8k | 1.4k | 0 | ✅ | O(n²) direct port | 0.638s | 60x |
| claude-sonnet-4 | 1 | 29s | 25s | 86.2k | 2.4k | 63.2k | ✅ | O(n²) direct port | 0.636s | 60x |
| gpt-5.1-codex | 1 | 15s | 12s | 15.3k | 1.8k | 0 | ✅ | O(n²) direct port | 0.638s | 60x |
| gemini-3-pro-preview | 1 | 1m 34s | 1m 31s | 27.5k | 664 | 0 | ✅ | O(n²) direct port | 0.637s | 60x |
| claude-haiku-4.5 | 0.33 | 22s | 18s | 106.2k | 1.9k | 83.6k | ✅ | O(n²) direct port | 0.653s | 58x |
| gpt-5.1 | 1 | 25s | 21s | 14.5k | 1.5k | 0 | ✅ | O(n²) direct port | 0.652s | 58x |
| gpt-5.1-codex-mini | 0.33 | 2m 58s | 2m 48s | 312.4k | 11.7k | 299.4k | ✅ | O(n²) direct port | 0.650s | 58x |
| gpt-5-mini | 0 | 37s | 34s | 14.5k | 2.2k | 0 | ❌ | O(n) Kadane's* | - | - |
| gpt-4.1 | 0 | 14s | 11s | 14.3k | 575 | 0 | ❌ | O(n²)* | - | - |

## Execution Output

### lcg.py (baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 37.967460 seconds
```

### result_claude-haiku-4_5 (58x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.653061 seconds
```

### result_claude-opus-4_5 (60x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.637561 seconds
```

### result_claude-sonnet-4 (60x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.635839 seconds
```

### result_claude-sonnet-4_5 (60x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.637420 seconds
```

### result_gemini-3-pro-preview (60x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.636735 seconds
```

### result_gpt-5 (111,669x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.000340 seconds
```

### result_gpt-5_1 (58x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.652027 seconds
```

### result_gpt-5_1-codex (60x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.638010 seconds
```

### result_gpt-5_1-codex-max (122,081x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.000311 seconds
```

### result_gpt-5_1-codex-mini (58x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.649950 seconds
```

### result_gpt-5_2 (115,403x faster than baseline)
```
Total Maximum Subarray Sum (20 runs): 10980
Execution Time: 0.000329 seconds
```
