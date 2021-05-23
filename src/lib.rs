use textwrap::{unfill};

const BLANK: &str = "\n\n";

pub fn unwrap(raw: &str) -> String {
    let old: Vec<&str> = raw.split(BLANK).collect();
    let ilast = old.len() - 1;
    let mut new = String::with_capacity(raw.len());
    let mut block = String::new();
    let mut type_last = ParagraphType::Whitespace;  // Ongoing block.
    for (i, p) in old.iter().enumerate() {
        let type_head = classify_head(p);
        let mut ends_previous_block = false;  // True if p shows that a previous block that does not include p is over.
        let mut ends_current_block = false;  // True if p is itself the end of a block.

        if type_last == ParagraphType::List {
            // Can only be ended by non-indented subsequent paragraph.
            if type_head != ParagraphType::Indented {
                ends_previous_block = true;
            }
            // Else maintain type_last.
        } else if type_head == ParagraphType::List {
            // New list. Wait and see.
        } else if type_last == ParagraphType::XML || type_head == ParagraphType::XML {
            let type_tail = classify_tail(p);
            if type_tail == ParagraphType::XML {
                // Unindented XML tag ends current paragraph.
                ends_current_block = true;
            } else {
                type_last = ParagraphType::XML;
            }
        } else {
            // There was no block-level structure in progress
            // and none has just started.
            ends_current_block = true;
        }

        if ends_previous_block {
            // Treat completed block. Append to output.
            new.push_str(&unwrap_paragraph(&block));
            if i < ilast {
                new.push_str(BLANK);
            }
            // Start new block.
            block = String::new();
            // Base future expectations on the current paragraph.
            type_last = type_head;
        }
        block.push_str(p);
        if ends_current_block {
            new.push_str(&unwrap_paragraph(&block));
            if i < ilast {
                new.push_str(BLANK);
            }
            block = String::new();
            type_last = ParagraphType::Whitespace;
        }
    }
    // Append any block not already finished, such as a Markdown list that ends the document, or an
    // XML block that is not properly terminated.
    if ! block.is_empty() {
        new.push_str(&unwrap_paragraph(&block));
    }
    return new;
}

#[derive(PartialEq)]
enum ParagraphType {
    // Continuation of e.g. XML or a Markdown list.
    // Identifiable at head only.
    // Indented text should end a wrappable block at end of document only.
    Indented,

    // Markdown-like list item.
    // Beginning of block is identifiable at head, end of block only by the start of a new
    // paragraph, outside the list, that is not indented.
    List,

    // General running text. One paragraph of this is always considered a complete block.
    Text,

    // Paragraph apparently composed entirely of whitespace.
    // Trash to be ignored.
    Whitespace,

    // XML/HTML block. Begins and ends with XML-style tags (<...>).
    XML,
}

fn unwrap_paragraph(p: &str) -> String {
    match classify_head(p) {
        ParagraphType::Indented => String::from(p),
        ParagraphType::List => String::from(unfill(p).0),
        ParagraphType::Text => String::from(unfill(p).0),
        ParagraphType::Whitespace => String::from(p),
        ParagraphType::XML => String::from(p),
    }
}

fn classify_head(subject: &str) -> ParagraphType {
    let trimmed: &str = subject.trim_start();
    if trimmed.is_empty() {
        // Extraneous, unclassifiable whitespace to be left untouched.
        return ParagraphType::Whitespace
    } else if trimmed != subject {
        // Starts with whitespace but is not all whitespace.
        // To be appended to a multi-paragraph block before other treatment.
        return ParagraphType::Indented
    } else if subject.starts_with("<") {
        // XML-like.
        return ParagraphType::XML
    } else if subject.starts_with("* ") {
        // Bullet list. Markdown-like.
        // TODO: More types: Leading numerals, dashes etc.
        return ParagraphType::List
    } else {
        // Default to wrappable text.
        return ParagraphType::Text
    }
}

fn classify_tail(subject: &str) -> ParagraphType {
    // Like classify_head but for the end of an apparent paragraph.
    if subject.trim().is_empty() {
        return ParagraphType::Whitespace
    } else if subject.ends_with(">") {
        return ParagraphType::XML
    } else {
        return ParagraphType::Text
    }
}

#[test]
fn classification_as_whitespace() {
    assert!(matches!(classify_head(""), ParagraphType::Whitespace));
    assert!(matches!(classify_head(" "), ParagraphType::Whitespace));
    assert!(matches!(classify_head(" \n"), ParagraphType::Whitespace));
    assert!(matches!(classify_head("\n\n"), ParagraphType::Whitespace));
    assert!(matches!(classify_head("\n \n\n"), ParagraphType::Whitespace));
}
#[test]
fn classification_as_indented() {
    assert!(matches!(classify_head(" a"), ParagraphType::Indented));
}
#[test]
fn classification_as_misc() {
    assert!(matches!(classify_head("a"), ParagraphType::Text));
    assert!(matches!(classify_head("a "), ParagraphType::Text));
    assert!(matches!(classify_head("a\na"), ParagraphType::Text));
    assert!(matches!(classify_head("a\na\na"), ParagraphType::Text));
    assert!(matches!(classify_head("a\na\n\na"), ParagraphType::Text));
    assert!(matches!(classify_head("^a"), ParagraphType::Text));
    assert!(matches!(classify_head("`a`"), ParagraphType::Text));
}
