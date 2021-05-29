use std::ops::Range;
use textwrap::unfill;
use pulldown_cmark::{Event, Options, Parser, Tag};

pub fn unwrap(raw: &str) -> String {
    let ret = zip(raw);
    return ret;
}

// Act as a predicate to identify paragraphs.
fn pred((e, _): &(Event, Range<usize>)) -> bool {
    matches!(e, Event::Start(Tag::Paragraph))
}
// Select ranges filtered by pred().
fn pick((_, r): (Event, Range<usize>)) -> Range<usize> {
    r
}

/// Join together modified paragraphs and other content using a pulldown-cmark event stream.
fn zip(raw: &str) -> String {
    let mut new = String::new();
    let mut lastoffset: usize = 0;

    // pranges is an iterable of Ranges describing the beginning and ending of every paragraph of
    // text in the original document.
    let pranges = Parser::new_ext(raw, Options::empty()).into_offset_iter().filter(pred).map(pick);

    // Combine untouched and retouched strings like a zip fastener.
    for range in pranges {
        if lastoffset < range.start {
            new.push_str(&raw[lastoffset..range.start]);
        }
        new.push_str(&unwrap_prefixed(&raw[range.start..range.end], false));
        lastoffset = range.end;
    }
    if lastoffset < raw.len() {
        new.push_str(&raw[lastoffset..raw.len()]);
    }

    return new;
}

/// Preserve initial indentation on unwrapping.
/// Check for subsequent indentation, either requiring it (for e.g. list items) or forbidding it
/// (in cases where it would not be recoverable on wrapping).
fn unwrap_prefixed(raw: &str, indented: bool) -> String {
    let (content, properties) = unfill(raw);
    if indented {
        assert!(!(properties.subsequent_indent.is_empty()));
    } else {
        assert!(properties.subsequent_indent.is_empty());
    }
    return String::from(properties.initial_indent) + &content;
}
