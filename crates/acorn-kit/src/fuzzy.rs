//! Fuzzy matching for quick switchers and pickers.

/// Fuzzy match of `q` (lowercase) in `text`: a prefix, a word start, a substring, or the
/// initials of words ("ab" for Ada Byron); scattered letters don't count. Higher is better:
/// a prefix beats a word start beats the rest, runs of consecutive characters count extra,
/// and long names lose a little.
pub fn fuzzy(q: &str, text: &str) -> Option<i64> {
    if q.is_empty() {
        return Some(0);
    }
    let t: Vec<char> = text.to_lowercase().chars().collect();
    let qs: Vec<char> = q.chars().collect();
    let joined: String = t.iter().collect();
    let mut score = if joined.starts_with(q) {
        1000
    } else if joined.split(|c: char| !c.is_alphanumeric()).any(|w| w.starts_with(q)) {
        700
    } else if joined.contains(q) {
        400
    } else {
        0
    };
    if score == 0 {
        // Initials: each query character starts a word, in order.
        let initials: String = t.iter().enumerate().filter(|(i, c)| c.is_alphanumeric() && (*i == 0 || !t[i - 1].is_alphanumeric())).map(|(_, c)| *c).collect();
        let mut it = initials.chars();
        if !qs.iter().all(|q| it.any(|c| c == *q)) {
            return None;
        }
        score = 300;
    }
    let (mut qi, mut run, mut last) = (0, 0i64, usize::MAX);
    for (i, c) in t.iter().enumerate() {
        if qi < qs.len() && *c == qs[qi] {
            run = if last != usize::MAX && i == last + 1 { run + 1 } else { 0 };
            let word_start = i == 0 || !t[i - 1].is_alphanumeric();
            score += 10 + run * 15 + if word_start { 25 } else { 0 };
            last = i;
            qi += 1;
        }
    }
    Some(score - t.len() as i64)
}
