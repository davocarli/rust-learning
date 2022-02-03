enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Second implementation
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

// Third implementation
enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(u16, u16, u16, u16),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("Let's Get Rusty!");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let  localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // Second implementation
    let localhost2 = IpAddrKind2::V4(String::from("127.0.0.1"));

    let localhost3 = IpAddrKind3::V4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddrKind) {

}