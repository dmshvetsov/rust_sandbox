pub fn run() {
    let strings = vec!["hello", "!quit", "abra cadabra", ":look"];

    for string in strings {
        let mut chars = string.chars();

        let res = match chars.nth(0) {
            Some('!') => Str::Bang { content: &string },
            Some(':') => Str::Colon { content: &string },
            Some(_) => Str::Message { content: &string },
            None => Str::None,
        };

        match res {
            Str::Bang { content } => println!("bang {}", content),
            Str::Colon {content} => println!("colon {}", content),
            Str::Message {content} => println!("message {}", content),
            _ => println!("no string"),
        }
    }
}

enum Str<'str> {
    Bang { content: &'str str },
    Colon { content: &'str str },
    Message { content: &'str str },
    None
}
