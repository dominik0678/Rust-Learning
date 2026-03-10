# second_practice_text

Simple Rust/C FFI starter with text utilities instead of math.

## C API (`c_lib/text_tools.h`)
- `txt_count_vowels(const char* input, int* out_count)`
- `txt_to_upper_ascii(char* buffer, size_t len)`
- `txt_starts_with(const char* input, const char* prefix, int* out_result)`

Status codes:
- `0` = success
- `-1` = null pointer
- `-2` = invalid length (e.g., buffer has no `\0` within `len`)

## Run the C-only demo
```bash
make -C c_lib run
```

## Run from Rust
```bash
cargo run
```

## Learning tasks
Open `src/student_ffi.rs` and implement:
- TODO 2: `to_upper_ascii`
- TODO 3: `starts_with`

Hints:
- For C strings from Rust, use `CString::new(...)`.
- For mutable C buffers, use a `Vec<u8>` with a trailing `0` byte.
- Keep `unsafe` inside wrapper functions only.

## Tests
```bash
cargo test
```

Two tests are marked `#[ignore]` until TODO 2 and TODO 3 are done.
