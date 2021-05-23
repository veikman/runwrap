use textwrap::{fill};

// lines: Vec<&str> = p.split("\n").collect();

pub fn unwrap(raw: &str) -> String {
    let old: Vec<&str> = raw.split("\n\n").collect();
    let mut new = String::with_capacity(raw.len());
    for p in old.iter() {
        new.push_str(&unwrap_paragraph(p));
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
    Some(ParagraphType::Text)
}
