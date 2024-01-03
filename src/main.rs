fn main() {
    let text = String::from("Olá José!");
    let texto_longo = "texto longo";
    let str_palavra = primeira_palavra(&texto_longo);
    let palavra = primeira_palavra(&text);

    println!("Primeira palavra: {}\n", palavra);
    println!("Str palavra: {}\n", str_palavra);
    let (first, last) = primeira_ultima_palavra(&text);

    println!("Primeira palavra: {}", first);
    println!("Ultima palavra: {}", last);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..=3];

    for i in slice.iter() {
        println!("{}", i);
    }
}

fn primeira_palavra(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn primeira_ultima_palavra(s: &String) -> (&str, &str) {
    let bytes = s.as_bytes();

    let mut first = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            first = i
        }
    }

    (&s[..first], &s[first + 1..])
}
