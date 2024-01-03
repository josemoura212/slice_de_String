fn main() {
    let text = String::from("Olá José!");
    let (first, last) = primeira_palavra(&text);

    println!("Primeira palavra: {}", first);
    println!("Ultima palavra: {}", last);
}

fn primeira_palavra(s: &String) -> (&str, &str) {
    let bytes = s.as_bytes();

    let mut first = 0;
    let mut last = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if first == 0 {
                first = i;
            }
            last = i;
        }
    }

    (&s[..first], &s[last + 1..])
}
