# parse_int_bug

Enabling feature implicit-octal does not seem to result in parsing strings as octals.

```rust
fn main() {
    let perms = parse_int::parse::<u32>("770").unwrap();

    assert_eq!(perms, 0o770);
}
```

```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/parse_int_bug`

thread 'main' (1791075) panicked at src/main.rs:4:5:
assertion `left == right` failed
  left: 770
 right: 504
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
