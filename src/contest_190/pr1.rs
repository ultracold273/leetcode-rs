
fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    let mut res = -1;
    for (idx, word) in sentence.split(" ").enumerate() {
        if word.starts_with(&search_word) {
            res = idx as i32 + 1;
            break;
        }
    }
    res
}