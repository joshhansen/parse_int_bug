fn main() {
    // example from parse_int readme works:
    assert_eq!(parse_int::parse::<i128>("00042").unwrap(), 0o00042);

    // this works with leading 0:
    assert_eq!(parse_int::parse::<u32>("0770").unwrap(), 0o770);

    // but this does not:
    assert_eq!(parse_int::parse::<u32>("770").unwrap(), 0o770);
}
