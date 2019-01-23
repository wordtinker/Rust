// enums can hold structured info inside
// and be able to impl
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // will catch anything here
    }
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // lightweight equivalent
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // every arm must be listed
        None => None,
        Some(i) => Some(i + 1)
    }
}