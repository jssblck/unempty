#[test]
fn compiles_indexing() {
    let v = unempty::Vec::new("abcd");
    assert_eq!(v[0], "abcd");
}

#[test]
#[should_panic]
fn compiles_but_panics_slice_oob() {
    let v = unempty::Vec::new("abcd");
    let _ = v[1];
}
