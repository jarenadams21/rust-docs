// IP Address Enumeration
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

// *Can't express varying values with a struct
enum VaryingIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // IpAddr::V4() is a function call that 
    // takes a String argument
    // and returns an instance 
    // of the IpAddr type.
    let home2 = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback2 = IpAddr2::V6(String::from("::1"));

    // Varying values with enums
    let home = VaryingIpAddr::V4(127, 0, 0, 1);

    let loopback = VaryingIpAddr::V6(String::from("::1"));

    dbg!(&home2);
    dbg!(&loopback2);
    println!("Hello, world!");
}
