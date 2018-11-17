fn main() {

    let greeting = String::from("Hallo, ich bin Alexander!");

    /*
        Variants 
            Type    => i8, u8
            Option  =>  8, 16, 32, 64, 128
        
        Two more thing 
            -> The upper/lower limit still applies like C. 
            -> The val of 'isize' is dpding on ur PC's arch (i32, i64)
    */
    let s1: i8;     
    let s2: i128;   
    let s3: isize;  


    /* 
        The types which Rust 'guessed out' 
            are usually good choices (and it's FAST)

        Number values end with sth like 'i32' 
            is totally accepted (surely the b'' cannot add this)
    */
    let i1 = 1_000_000_000i32;  // 1 billion 
    let i2 = 0xff + 0o77;       // 318
    let i3 = 0b1111_1111;       // 255
    let i4 = b'R';              // 82 (hmm)

    println!("\n{}", greeting);
    println!("[1]: {} [2]: {} [3]: {} [4]: {}",
                   i1,     i2,     i3,     i4 );


    // Eazy peasy
    let ep1         = 2.0;  // f64
    let ep2: f32    = 2.0;  // f32
    let ep3	        = true;
    let ep4: bool   = false;


    // Opts! 
    let o1 = 10 / 3;
    let o2 = 10 % 3;
    println!("(10, 3) => Div: {}, Mod: {}", o1, o2);


    /*
        The '' and "" is quite different (e.g. Type) in Rust. 

        It use the 'UTF-16' by default I assume. 
            Related info: https://zh.wikipedia.org/wiki/UTF-16
    */
    let c1 = 'ä';
    let c2 = '😇';
    println!("--- {}, --- {}", c1, c2);


    // The annotations here are OPTIONAL. 
    let tuple: (bool, i32, f64)  = (true, 1, 2.0);

    let (t1, t2, t3) = tuple;

    let p1 = tuple.0;  // index, sort of
    let p2 = tuple.1;
    let p3 = tuple.2;

    println!("--- {}, --- {}, --- {}", t1, t2, t3);
    println!("--- {}, --- {}, --- {}", p1, p2, p3);


    /*
        Some cmp between 'Tuple' & 'Array'
            => Both are <fixed> length!
            => The former could contain any types, the latter is NOT.

        Oh, there's a "vector" type BTW. 
            It could be expanded (or shrinked).
    */
    let ary_mth             = ["Jan.", "Feb.", "Mar."];
    let ary_num: [i32; 6]   = [1, 2, 3, 4, 5, 6];
    
    // Access the elems
    let m_jan = ary_mth[0];
    let m_feb = ary_mth[1];

    /* 
        Invalid index 
            -> checking by Rust 
            -> then produce 'panic' (Exit)
    */
    // let m_dec = ary_mth[11];
}
