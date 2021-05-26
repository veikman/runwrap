use textwrap::unfill;

#[test]
fn unfill_trivial() {
    const IN: &str = "I had so perfectly expected that the return of my pupils would be marked by a
demonstration that I was freshly upset at having to take into account that they
were dumb about my absence.";
    const OUT: &str = "I had so perfectly expected that the return of my pupils would be marked by a demonstration that I was freshly upset at having to take into account that they were dumb about my absence.";
    assert_eq!(OUT, unfill(IN).0);
}

/// Characterize the way textwrap::unfill responds to a typical Markdown
/// heading.
#[test]
fn unfill_markdown_heading_default() {
    let (filled, options) = unfill("## History");
    assert_eq!("History", filled);
    assert_eq!("## ", options.initial_indent);
    assert_eq!("", options.subsequent_indent);
}
