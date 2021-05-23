use textwrap::{fill};

// lines: Vec<&str> = p.split("\n").collect();

pub fn unwrap(raw: &str) -> String {
    let old: Vec<&str> = raw.split("\n\n").collect();
    let ilast = old.len() - 1;
    let mut new = String::with_capacity(raw.len());
    for (i, p) in old.iter().enumerate() {
        new.push_str(&unwrap_paragraph(p));
        if i < ilast {
            new.push_str("\n\n");
        }
    }
    return new;
}

enum ParagraphType {
    Text,
}

fn unwrap_paragraph(p: &str) -> String {
    match classify(p) {
        Some(ParagraphType::Text) => String::from(fill(p, usize::MAX)),
        None => String::from(p),
    }
}

fn classify(subject: &str) -> Option<ParagraphType> {
    if subject.trim() == "" {
        // Extraneous, unclassifiable whitespace to be left untouched.
        return None
    } else {
        // Default to wrappable text.
        return Some(ParagraphType::Text)
    }
}

#[test]
fn classify_whitespace() {
    assert!(matches!(classify(""), None));
    assert!(matches!(classify(" "), None));
    assert!(matches!(classify(" \n"), None));
    assert!(matches!(classify("\n\n"), None));
    assert!(matches!(classify("\n \n\n"), None));
}
#[test]
fn classify_misc() {
    assert!(matches!(classify("a"), Some(ParagraphType::Text)));
    assert!(matches!(classify(" a"), Some(ParagraphType::Text)));
    assert!(matches!(classify("a "), Some(ParagraphType::Text)));
    assert!(matches!(classify("a\na"), Some(ParagraphType::Text)));
    assert!(matches!(classify("a\na\na"), Some(ParagraphType::Text)));
    assert!(matches!(classify("a\na\n\na"), Some(ParagraphType::Text)));
    assert!(matches!(classify("^a"), Some(ParagraphType::Text)));
    assert!(matches!(classify("`a`"), Some(ParagraphType::Text)));
}
