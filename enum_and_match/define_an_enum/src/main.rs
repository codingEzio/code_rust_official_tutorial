#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {

    /*
        Down below we're using "IP Address" to take examples.

        To be clear, there's already a impl at
            https://doc.rust-lang.org/std/net/enum.IpAddr.html
    */
}

fn enum_start() {
    /* Syntax, variants, param, arg */

    enum IPAddrKind {
        V4,
        V6,
    }

    let p4 = IPAddrKind::V4;
    let p6 = IPAddrKind::V6;

    fn route(ip_type: IPAddrKind) {
        // pass
    }

    route(IPAddrKind::V4);
    route(IPAddrKind::V6);
}

fn enum_with_struct() {
    /* If using existed knowledge for now */

    #[derive(Debug)]
    enum IPAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IPAddrKind,
        addr: String,
    }

    let home = IpAddr {
        kind: IPAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };
}

fn enum_type_contained() {
    /* This one no longer needs a struct! */

    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    enum IpAddrAwesome {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // Hmm
    let v4 = IpAddr::V4(String::from("1.1.1.1"));
    let v6_loopback = IpAddr::V6(String::from("::1"));

    // Ja
    let va4 = IpAddrAwesome::V4(10, 115, 176, 11);
}

fn enum_similarity_to_struct() {
    enum Message {
        Quit,                       // no data associated
        Move { x: i32, y: i32 },    // anonymous struct
        Write(String),              // one string val
        ChangeColor(i32, i32, i32), // three val (?param)
    }

    // if using `struct`
    struct QuitMsg;
    struct MoveMsg {
        x: i32,
        y: i32,
    }
    struct WriteMsg(String);
    struct ChangeColorMsg(i32, i32, i32);

    // define methods
    impl Message {
        fn greeting(&self) {
            // pass
        }
    }

    let em = Message::Write(String::from("Hey"));
    em.greeting();
}

fn option_enum_basic() {

    /* 
        https://doc.rust-lang.org/book/2018-edition/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values
    */
}
