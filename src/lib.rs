//! This library provides, `current_crate_name()`, a const function that returns the name of the current crate.
//!
//! Most other libraries that provide this functionality rely on a macro and\or the `CARGO_PKG_NAME` environment
//! variable. This library relies on the built-in `module_path!()` macro instead to provide a simple const fn that
//! achieves the same result.
//!

/// Returns the current crate name.
///
/// # Examples
///
/// ```
/// use current_crate_name::current_crate_name;
///
/// assert_eq!(current_crate_name(), "current_crate_name");
/// ```
#[allow(non_upper_case_globals)]
pub const fn current_crate_name() -> &'static str {
    const module_path_separator: u8 = b':';
    const module_path_byte_slice: &[u8] = module_path!().as_bytes();
    const module_path_separator_index: usize = {
        let mut index = 0;
        loop {
            if index == module_path_byte_slice.len()
                || module_path_byte_slice[index] == module_path_separator
            {
                break index;
            }
            index += 1;
        }
    };
    const new_slice: [u8; module_path_separator_index] = {
        let mut arr = [0; module_path_separator_index];
        let mut i = 0;
        while i < module_path_separator_index {
            arr[i] = module_path_byte_slice[i];
            i += 1;
        }
        arr
    };

    // SAFETY: The original string was valid UTF-8 and we sliced it at the start of an ASCII byte which is a valid unicode boundary. Hence the new string is still valid UTF-8.
    unsafe { std::str::from_utf8_unchecked(&new_slice) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_name() {
        assert_eq!(current_crate_name(), "current_crate_name");
    }
}
