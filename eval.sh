#!/bin/bash

# 1. Configuration
MODELS=("claude-sonnet-4.5" "claude-haiku-4.5" "claude-opus-4.5" "claude-sonnet-4" "gpt-5.1-codex-max" "gpt-5.1-codex" "gpt-5.2" "gpt-5.1" "gpt-5" "gpt-5.1-codex-mini" "gpt-5-mini" "gpt-4.1" "gemini-3-pro-preview")
#MODELS=("gpt-5-mini" "gpt-4.1")
#MODELS=("gemini-3")
COMPILE_CMD="rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 -C lto=fat -C panic=abort"

# 2. Gather System Info (Auto-detected)
# This gets the OS version (Darwin 25.2.0) and CPU Brand (Apple M2 Pro)
OS_INFO=$(uname -sr)
CPU_INFO=$(sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "Unknown CPU")
SYS_INFO="OS: $OS_INFO | CPU: $CPU_INFO | Arch: $(uname -m)"

echo "Detected System: $SYS_INFO"

# 3. Define the Python Code
read -r -d '' PYTHON_CODE <<'EOF'
def lcg(seed, a=1664525, c=1013904223, m=2**32):
    value = seed
    while True:
        value = (a * value + c) % m
        yield value

def max_subarray_sum(n, seed, min_val, max_val):
    lcg_gen = lcg(seed)
    random_numbers = [
        next(lcg_gen) % (max_val - min_val + 1) + min_val for _ in range(n)
    ]
    max_sum = float("-inf")
    for i in range(n):
        current_sum = 0
        for j in range(i, n):
            current_sum += random_numbers[j]
            if current_sum > max_sum:
                max_sum = current_sum
    return max_sum

def total_max_subarray_sum(n, initial_seed, min_val, max_val):
    total_sum = 0
    lcg_gen = lcg(initial_seed)
    for _ in range(20):
        seed = next(lcg_gen)
        total_sum += max_subarray_sum(n, seed, min_val, max_val)
    return total_sum

import time

n = 10000
initial_seed = 42
min_val = -10
max_val = 10

start_time = time.time()
result = total_max_subarray_sum(n, initial_seed, min_val, max_val)
end_time = time.time()

print(f"Total Maximum Subarray Sum (20 runs): {result}")
print(f"Execution Time: {end_time - start_time:.6f} seconds")
EOF

# 4. The Prompt Template
# We inject variables here.
PROMPT="
Task: Port this Python code to ultra-high-performance Rust.
System Context: $SYS_INFO
Constraint: The code will be compiled with: '$COMPILE_CMD'
Requirements:
1. Output ONLY valid Rust code.
2. No markdown backticks. No chat. No explanations.
3. Optimize specifically for the detected CPU architecture.

Python Code:
$PYTHON_CODE
"

# 5. The Loop
echo "--------------------------------"
for model in "${MODELS[@]}"; do
    echo "🚀 Asking $model..."
    OUTFILE="result_${model}.rs"
    echo "=== Results for $model ===" >> benchmark_stats.log
    # Run Copilot with the enhanced prompt
    copilot -p "$PROMPT" --model "$model" > "$OUTFILE" 2>> benchmark_stats.log

    echo "   -> Saved to $OUTFILE"
done

echo "--------------------------------"
echo "Done. Run this to verify compilation:"
echo "for f in result_*.rs; do rustc \"\$f\" $COMPILE_CMD -o \"bin/\${f%.rs}\" && echo \"✅ \$f\"; done"
