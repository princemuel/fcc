struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
    let mut user_01 = User {
        username: String::from("princemuel"),
        email: String::from("vansomecsam@gmail.com"),
        sign_in_count: 1,
        is_active: true,
    };

    let name = user_01.username;
    user_01.username = String::from("blackswan");

    let user_02 = build_user(
        String::from("rednovember@gmail.com"),
        String::from("rednovember"),
    );

    let user_03 = User {
        email: String::from("rednovember@gmail.com"),
        username: String::from("rednovember"),
        ..user_01
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        is_active: true,
    }
}
