
fn reverse_words(s: String) -> String {
    let words = s.split(' ').filter(|a| a != &"").collect::<Vec<_>>();

    words.iter().rev().fold("".to_string(), 
                        |a, b| if a.len() > 0 { 
                            a + " " + &b.to_string()
                        } else { 
                            a + &b.to_string()
                        })
}