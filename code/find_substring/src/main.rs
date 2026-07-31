fn main() {
    println!("Hello, world!");

    let s = String::from("Hello, world");
    let w = String::from("world");

    let result = find_first_occurance(&s, &w);

    if let Some(matched) = result {
        println!("{}", matched);
    }
}

fn find_first_occurance<'a>(s: &'a str, w: &str) -> Option<&'a str> {
    s.find(w).map(|start| &s[start .. start + w.len()])
}