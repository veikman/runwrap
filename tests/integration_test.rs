use runwrap;

const EMPTY: &str = "";

#[test]
fn unwrap_empty() {
    assert_eq!(EMPTY, runwrap::unwrap(EMPTY));
}
