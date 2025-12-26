# Code Cleanup Plan

**Date:** 2025-12-26
**Status:** Draft
**Priority:** Medium

## Overview

Clean up code issues found during quality analysis after platform refactoring.

## Issues to Fix

### High Priority

- [ ] **Duplicate GPU stub functions** - `linux.rs` and `windows.rs` have unused stub functions that reference non-existent `AppleGpuInfo`
- [ ] **Duplicate `cores()` function** in `lang.rs` - duplicates `cores_label()`

### Medium Priority

- [ ] **Hardcoded Vietnamese in RAM module** - Use `Text` from `lang.rs` instead
- [ ] **Magic number `10000`** in CPU stress test

### Low Priority

- [ ] **Replace `unwrap()` with `expect()`** for better error messages

## Progress

- [ ] Phase 1: Fix compilation errors (GPU stub functions)
- [ ] Phase 2: Remove duplicate code
- [ ] Phase 3: Fix hardcoded strings
- [ ] Phase 4: Test and verify

## Files

| Phase | Files |
|-------|-------|
| 1 | `src/stress/gpu/platform/linux.rs`, `windows.rs` |
| 2 | `src/lang.rs` |
| 3 | `src/stress/ram/mod.rs`, `src/stress/cpu/mod.rs` |
