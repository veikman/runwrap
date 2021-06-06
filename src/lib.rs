use std::ops::Range;
use textwrap::{unfill, fill, refill};
use textwrap::Options as TwOptions;
use textwrap::wrap_algorithms::FirstFit;
use textwrap::word_separators::AsciiSpace;
use textwrap::word_splitters::NoHyphenation;
use pulldown_cmark::{Event, Parser, Tag};
use pulldown_cmark::Options as CmarkOptions;
use partial_application::partial;


// Interface functions:

pub fn wrap(raw: &str, new_width: usize) -> String {
    zip(raw, partial!(fill => _, opts(new_width)))
}
pub fn rewrap(raw: &str, new_width: usize) -> String {
    zip(raw, partial!(refill => _, opts(new_width)))
}
pub fn unwrap(raw: &str) -> String {
    zip(raw, partial!(unwrap_prefixed => _))
}


// Internal functions:

/// Produce a textwrap configuration for reversible programmatic applications, not readability or
/// aesthetics.
fn opts<'a>(width: usize) -> TwOptions<'a> {
    // TODO: Expose the configuration interface of textwrap more fully, not just width.
    // TODO: Memoization for performance?
    // TODO: Context-sensitive indentation; cf. https://github.com/mgeisler/textwrap/issues/224.
    TwOptions {
        width,
        initial_indent: "",
        subsequent_indent: "",
        break_words: false,
        wrap_algorithm: Box::new(FirstFit),
        word_separator: Box::new(AsciiSpace),
        word_splitter: Box::new(NoHyphenation),
    }
}

/// Act as a predicate to identify paragraphs.
fn pred((e, _r): &(Event, Range<usize>)) -> bool {
    matches!(e, Event::Start(Tag::Paragraph))
}
/// Select ranges filtered by pred().
fn pick((_e, r): (Event, Range<usize>)) -> Range<usize> {
    r
}

/// Join together modified paragraphs and other content.
/// This uses a pulldown-cmark event stream and a closure acting upon each paragraph.
fn zip<F: Fn(&str) -> String>(raw: &str, pfn: F) -> String {
    // “pranges” is an iterable of Ranges describing the beginning and ending of every paragraph of
    // text in the original document.
    let pranges = Parser::new_ext(raw, CmarkOptions::empty()).into_offset_iter().filter(pred).map(pick);

    // Combine untouched and retouched strings like a zip fastener.
    let mut new = String::new();
    let mut lastoffset: usize = 0;
    for range in pranges {
        if lastoffset < range.start {
            new.push_str(&raw[lastoffset..range.start]);
        }
        new.push_str(&pfn(&raw[range.start..range.end]));
        lastoffset = range.end;
    }
    if lastoffset < raw.len() {
        // Retain whatever is left after the final paragraph.
        new.push_str(&raw[lastoffset..raw.len()]);
    }
    return new;
}

/// Preserve initial indentation on unwrapping.
/// This is a workaround for textwrap’s tendency to interpret non-alphanumeric leading characters
/// as indentation (e.g. comment syntax) and destroy it. What textwrap calls “subsequent_indent” is
/// destroyed without comment.
fn unwrap_prefixed(raw: &str) -> String {
    let (content, properties) = unfill(raw);
    return String::from(properties.initial_indent) + &content;
}
