# Single Instance Check Debugging (2025-12-12)

Implemented detailed error reporting for `kitchn` single-instance check failure.
- File modified: `crates/kitchn_cli/src/main.rs`
- Change: `acquire_lock` now includes the lock file path and OS error code in the panic/error message.
- Reason: User reported "Another instance... running" on Void Linux (musl) despite no other instance. The new error message will expose the underlying cause (permissions, path, or flock support).
