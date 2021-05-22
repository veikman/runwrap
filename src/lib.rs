
pub fn unwrap(raw: &str) -> String {
    let old: Vec<&str> = raw.split("\n\n").collect();
    let mut new = String::with_capacity(raw.len());
    for p in old.iter() {
        new.push_str(p)
    }
    return new;
}
