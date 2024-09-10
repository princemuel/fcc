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

    let x = 1;
    match x {
        1 => println!("one"),
        _ => println!("anything"),
    }

    let x = Some(5);

    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y:?}"),
        _ => println!("Default case, x = {x:?}"),
    }

    let x = 1;
    match x {
        1 | 2 | 3 => println!("one or two or three"),
        4 => println!("four"),
        _ => println!("anything"),
    }

    match x {
        1..=5 => println!("one through five"),
        6 => println!("six"),
        _ => println!("somthing else"),
    }
    let x = 'c';

    match x {
        'a'..='j' => println!("realy ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("somthing else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = point;

    assert_eq!(0, a);
    assert_eq!(7, b);

    let point = Point { x: 0, y: 7 };

    match point {
        Point { x, y: 0 } => {
            println!("On the x axis at {x}")
        },
        Point { x: 0, y } => {
            println!("On the y axis at {y}")
        },
        Point { x, y } => {
            println!("On neither axis at ({x}, {y})")
        },
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit")
        },
        Message::Move { x, y } => {
            println!("Move to x: {x} y: {y}")
        },
        Message::Write(text) => {
            println!("text message {text}")
        },
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red: {r} green: {g}, blue: {b}")
        },
    }

    enum Color {
        RGB(i32, i32, i32),
        HSV(i32, i32, i32),
    }
    enum Msg {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Msg::ChangeColor(Color::HSV(0, 160, 255));
    match msg {
        Msg::ChangeColor(Color::RGB(r, g, b)) => {
            println!("Change color to red: {r} green: {g}, blue: {b}")
        },
        Msg::ChangeColor(Color::HSV(h, s, v)) => {
            println!("Change color to hue: {h} saturation: {s}, value: {v}")
        },
        _ => (),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing cutomized value");
        },
        _ => {
            setting_value = new_setting_value;
        },
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        },
    }

    struct Coord {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Coord { x: 0, y: 0, z: 0 };

    match origin {
        Coord { x, .. } => println!("x is {x}"),
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        },
    }

    // * Match Guards
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than fixe: {x}"),
        Some(x) => println!("{x}"),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(n) if n == 5 => println!("Matched, n = {n}"),
        _ => println!("default case, x = {x:?}"),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum Greeting {
        Hello { id: i32 },
    }
    let msg = Greeting::Hello { id: 5 };

    match msg {
        Greeting::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in the range: {id_variable}")
        },
        Greeting::Hello { id: 10..=12 } => {
            println!("Found an id in another range:")
        },
        Greeting::Hello { id } => {
            println!("Found some other id: {id}")
        },
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: ({x}, {y})");
}
