#![allow(unused_variables)]
#![allow(dead_code)]

use std::net::{Ipv4Addr, Ipv6Addr};

fn main() {
    enum_start();
    enum_with_struct();

    enum_def_with_type();
    enum_similarity_to_struct();

    option_enum_and_none_in_rust();
}

fn enum_start() {
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

fn enum_def_with_type() {
    #[derive(Debug)]
    enum MyIpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let v4 = MyIpAddr::V4(10, 115, 176, 11);
    let v6 = MyIpAddr::V6(String::from("::1"));

    // standard library also provided one :P
    let stdv4 = Ipv4Addr::new(10, 115, 30, 25);
    let stdv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0x01);

    println!("Mine: \n\t{:?}\n\t{:?}", v4, v6);
    println!("Std: \n\t{}\n\t{}", stdv4, stdv6);
}

fn enum_similarity_to_struct() {
    #[derive(Debug)]
    enum Message {
        Quit,                       // no data associated
        Move { x: i32, y: i32 },    // anonymous struct
        Write(String),              // enum item: 1 str
        ChangeColor(i32, i32, i32), // enum item: 3 int
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
            println!("{:?}", &self);
        }
    }

    // Just a string (in case u've forget it)
    let m = Message::Write(String::from("Hey"));

    m.greeting();
}

fn option_enum_and_none_in_rust() {
    /*
        Rust doesn't have 'None'.
        Instead, it provides another way
            | to indicate whether the val exists or not
            | which this is also the reason why 'None' exists

        It was impl as an `enum` (i.e. Option<T>).
            -> The "T" used in here is a new concept
            -> we'll talk about this later #TODO till cp10
    
        Btw, it was brought into scope automatically :P
    */

    #[derive(Debug)]
    enum MyOption<T> {
        Some(T), // The 'T' could be any type
        None,
    }

    let some_num = Some(5);
    let some_str = Some("a str");

    /*
        `None` in Rust is quite special,
            -> ya must specify its type (i.e. Option)

        How to use it 
            -> If the val is possibly none, then u should use it 
            -> It's kinda split 'null' & else into two worlds :P 
        
        Also, there's a quote backing my words 
            | This was a deliberate design decision for Rust 
                -> to limit null’s pervasiveness
                -> and increase the safety of Rust code.
    */
    let where_ru: Option<i32> = None;

    /*
        We barely touched the 
            surface of `enum` & related `Option`. 

        Umm, that's all. 
        Let's continue the next section! (`Match`)
    */
}
