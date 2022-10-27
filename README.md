# Current crate name

This library provides, `current_crate_name()`, a const function that returns the name of the current crate.

Most other libraries that provide this functionality rely on a macro and\or the `CARGO_PKG_NAME` environment
variable. This library relies on the built-in `module_path!()` macro instead to provide a simple const fn that
achieves the same result.

# Usage

```rust
use current_crate_name::current_crate_name;

assert_eq!(current_crate_name(), "current_crate_name");
```
