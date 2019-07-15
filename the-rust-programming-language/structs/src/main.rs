fn main() {
    let mut user = User {
        email: String::from("some@email.com"),
        username: String::from("some-user-name"),
        sign_in_count: 0,
        active: true,
    };

    user.email = String::from("some-other@email.com");

    let _built_user = build_user(user.email, user.username);

    let _user2 = User {
        email: String::from("another@email.com"),
        username: String::from("another-user-name"),
        ..user
    };

    //Tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    let _red_component = black.0;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
