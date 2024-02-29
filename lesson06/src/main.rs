// enum IpAddressKind {
//     V4,
//     V6,
// }
// enum IpAddressKind {
//     V4(String),
//     V6(String),
// }
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddress {
    __kind: IpAddressKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn boo() {
        println!("boo! Message::boo() was called!");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

fn main() {
    // let ip_v4 = IpAddressKind::V4;
    // let ip_v6 = IpAddressKind::V6;

    // let local_host = IpAddress {
    //     __kind: IpAddressKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let local_host = IpAddressKind::V4(String::from("127.0.0.1"));
    // let local_host = IpAddressKind::V4(127, 0, 0, 1);

    // let some_number = Some(5);
    // let some_string = Some("a string");

    // let absent_number = None::<i32>;

    let x = 5;
    let y = Some(5);

    // let sum = x + y.unwrap_or(0);
    value_in_cents(Coin::Quarter(State::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    // let none = plus_one(None);

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    if let Some(3) = some_value {
        println!("three");
    }
}

fn route(ip_kind: IpAddressKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None,
        Some(y) => Some(y + 1),
        _ => None,
    }
}
