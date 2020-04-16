
fn parse(v: &Vec<char>, start: usize, s: String) -> bool {
    let n = v.len();
    let mut start = start;
    let sv = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    while start < n && i < sv.len() {
        if v[start] != sv[i] { return false; }
        start += 1;
        i += 1;
    }
    i == sv.len()
}

fn entity_parser(text: String) -> String {
    let vt = text.chars().collect::<Vec<_>>();
    let mut res = Vec::new();
    let mut i = 0;
    while i < vt.len() {
        if vt[i] != '&' {
            res.push(vt[i]);
        } else if parse(&vt, i, "&quot;".to_string()) {
            res.push('"');
            i += 5;
        } else if parse(&vt, i, "&apos;".to_string()) {
            res.push('\'');
            i += 5;
        } else if parse(&vt, i, "&amp;".to_string()) {
            res.push('&');
            i += 4;
        } else if parse(&vt, i, "&gt;".to_string()) {
            res.push('>');
            i += 3;
        } else if parse(&vt, i, "&lt;".to_string()) {
            res.push('<');
            i += 3;
        } else if parse(&vt, i, "&frasl;".to_string()) {
            res.push('/');
            i += 6;
        } else {
            res.push(vt[i]);
        }
        i += 1;
    }
    res.iter().collect::<String>()
}