const WIDTH: usize = 69;

pub fn leftpad(s: &str) -> String {
    match s.len() {
        0..=WIDTH => format!("{}{}", "-".repeat(WIDTH - s.len()), s),
        _ => s.to_string(),
    }
}

/// Wrap lines of text, but not lines with URLs.
pub fn selective_wrap(s: &str) -> String {
    s.split('\n')
        .map(|l| match l.contains("http") {
            true => l.to_string(),
            false => textwrap::fill(l, WIDTH),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {}
