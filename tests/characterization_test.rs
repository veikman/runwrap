use textwrap::unfill;

#[test]
fn unfill_trivial() {
    const IN: &str = "I had so perfectly expected that the return of my pupils would be marked by a
demonstration that I was freshly upset at having to take into account that they
were dumb about my absence.";
    const OUT: &str = "I had so perfectly expected that the return of my pupils would be marked by a demonstration that I was freshly upset at having to take into account that they were dumb about my absence.";
    assert_eq!(OUT, unfill(IN).0);
}

#[test]
fn unfill_markdown_heading() {
    const IN: &str = "## History";
    assert_eq!(IN, unfill(IN).0);
}
