fn main() {
    let mut text: String = String::from("Rust is Awesome");

    let x = &text;
    let y = &text;

    println!("{x}");
    println!("{y}");

    exclamation(&mut text);

    println!("{text}");
}

fn exclamation(s: &mut String) {
    s.push_str("!!!");
}
