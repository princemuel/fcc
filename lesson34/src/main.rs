fn main() {
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
        French,
    }

    let lang = Language::English;

    match lang {
        Language::English => println!("Heloo World!"),
        lang => println!("Unsupported language {lang:?}"),
        // _ => println!("Unsupported language"),
    }

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {status}");
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }

    let mut stack = vec![];

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = vec!['a', 'b', 'c'];

    for (idx, char) in v.iter().enumerate() {
        println!("{char} is at index {idx}");
    }

    let x = 5;

    let (_, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    let x = 5; // irrefutable
    let x: Option<&str> = None; // refutable
    if let Some(x) = x {
        println!("{x}");
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: ({x}, {y})");
}
