# parse_int_bug

Enabling feature implicit-octal does not seem to always result in parsing strings as octals.

The first assertion succeeds, while the second fails:

```rust
fn main() {
    // example from parse_int readme works:
    assert_eq!(parse_int::parse::<i128>("00042").unwrap(), 0o00042);

    // this works with leading 0:
    assert_eq!(parse_int::parse::<u32>("0770").unwrap(), 0o770);

    // but this does not:
    assert_eq!(parse_int::parse::<u32>("770").unwrap(), 0o770);
}
```

```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/parse_int_bug`

thread 'main' (1798421) panicked at src/main.rs:9:5:
assertion `left == right` failed
  left: 770
 right: 504
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
