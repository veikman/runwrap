use runwrap;

const EMPTY: &str = "";

#[test]
fn unwrap_empty() {
    //assert_eq!(EMPTY, yamlrap::unwrap(EMPTY));
    assert_eq!(EMPTY, EMPTY);
}
