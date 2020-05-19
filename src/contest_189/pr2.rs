fn arrange_words(text: String) -> String {
    let mut words = text.split_ascii_whitespace().collect::<Vec<_>>();
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    let mut new_words = Vec::new();
    for (idx, &w) in words.iter().enumerate() {
        let nw = if idx == 0 {
            w.chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect::<String>()
        } else {
            w.chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_lowercase() } else { c })
                .collect::<String>()
        };
        new_words.push(nw.clone())
    }
    new_words.join(" ")
}
