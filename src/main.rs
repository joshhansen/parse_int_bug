fn main() {
    // example from parse_int readme works:
    assert_eq!(parse_int::parse::<i128>("00042").unwrap(), 34);

    // but this does not:
    assert_eq!(parse_int::parse::<u32>("770").unwrap(), 0o770);
}
