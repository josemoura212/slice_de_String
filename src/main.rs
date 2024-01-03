fn main() {
    let text = String::from("Olá mundo!");
    let first = primeira_palavra(&text);

    println!("Primeira palavra: {}", first);
}

fn primeira_palavra(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
