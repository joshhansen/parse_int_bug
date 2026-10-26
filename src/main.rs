fn main() {
    let perms = parse_int::parse::<u32>("770").unwrap();

    assert_eq!(perms, 0o770);
}
