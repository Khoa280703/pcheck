# Phase 3: Fix Hardcoded Strings

**Status:** Pending
**Priority:** MEDIUM

## Context

RAM module has hardcoded Vietnamese strings despite having a translation system.

## Files

- `src/stress/ram/mod.rs`

## Issues

Lines with hardcoded Vietnamese:
- Line 51: `"Đang kiểm tra RAM..."`
- Line 61: `"Lỗi khi đọc thông tin RAM"`
- Line 89: `"RAM"` (in display)

## Solution

Add these strings to `Text` struct in `lang.rs` and use them.

## Implementation Steps

1. Add methods to `Text` in `lang.rs`:
   - `ram_checking()` - "Checking RAM..." / "Đang kiểm tra RAM..."
   - `ram_error()` - "Error reading RAM info" / "Lỗi khi đọc thông tin RAM"
2. Update `stress/ram/mod.rs` to use `text.ram_checking()` etc.
3. Build and verify

## Additional: Magic Number

- `src/stress/cpu/mod.rs:77` - `calculate_primes(10000)`
- Make constant: `const PRIME_WORKLOAD: usize = 10000;`
