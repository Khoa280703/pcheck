# Phase 1: Fix Compilation Errors

**Status:** Pending
**Priority:** HIGH

## Context

The GPU platform stub files for Linux and Windows reference `AppleGpuInfo` which was made private in `macos.rs`. This causes compilation errors on non-macOS platforms.

## Files

- `src/stress/gpu/platform/linux.rs`
- `src/stress/gpu/platform/windows.rs`

## Issues

Both files have unused stub functions:
- `get_apple_gpu_info()` - references `AppleGpuInfo`
- `get_smc_temperature()` - duplicated from stub module

## Solution

The `mod.rs` already has a stub module that handles non-macOS platforms. The individual platform files should only contain what's actually different for each platform.

**For Linux/Windows:**
- Remove unused functions (already handled by stub)
- Keep files minimal or remove entirely if just returning `None`

## Implementation Steps

1. Remove unused stub functions from `linux.rs`
2. Remove unused stub functions from `windows.rs`
3. Build and verify no errors
