# Phase 2: Remove Duplicate Code

**Status:** Pending
**Priority:** MEDIUM

## Context

`src/lang.rs` has duplicate function `cores()` that does exactly the same as `cores_label()`.

## Files

- `src/lang.rs:68-73`

## Issue

```rust
pub fn cores(&self) -> &str {
    match self.lang {
        Language::Vietnamese => "nhân",
        Language::English => "cores",
    }
}
```

This is identical to `cores_label()` at lines 54-59.

## Solution

Remove `cores()` function and replace any usage with `cores_label()`.

## Implementation Steps

1. Search for `cores()` usage in codebase
2. Replace with `cores_label()`
3. Remove `cores()` from `lang.rs`
4. Build and verify
