struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("davocarli@gmail.com"),
        username: String::from("dcarli-arnold"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("davocarli");

    let user2 = build_user(
        String::from("david.carli-arnold@smartsheet.com"), 
        String::from("dcarli-arnold")
    );

    let user3 = User {
        email: String::from("david.carli-arnold@demo.mbfcorp.com"),
        username: String::from("david-demo"),
        ..user2
    };

    
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}