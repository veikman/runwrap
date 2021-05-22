use runwrap;

#[test]
fn unwrap_empty() {
    const VAL: &str = "";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_space() {
    const VAL: &str = " ";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_1newline() {
    const VAL: &str = "\n";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_2newline() {
    assert_eq!("\n\n", runwrap::unwrap("\n\n"));
}
