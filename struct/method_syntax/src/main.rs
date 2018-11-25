#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    /*
        Let's define a 'method'.

        Quite like 'function', but it
            => definied within the context of 'struct' (or enum, trait)
            => first param is `self` (like Python, I think), not all. 
    */

    let rc1 = Rect {width: 25, height: 40};
    let rc2 = Rect {width: 15, height: 30};
    let rc3 = Rect {width: 40, height: 50};
    
    // simple use 
    println!("rc1: {}", rc1.area());

    // passing inst 
    println!("{}", rc1.can_hold(&rc2));
    println!("{}", rc1.can_hold(&rc3));

    // "static method"
    println!("{:#?}", Rect::square(25));  // :? for one-line 


    /* 
        < Summary(edited) from the official tutorial > 

        Struct              keep associated pieces of data connected 
        Struct's method     define the behavior that struct has 
        Struct's function   let you namespace a functionality without inst 
    */
}

/* ----- ----- ----- structs ----- ----- ----- */
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
/* ----- ----- ----- structs EOF ----- ----- ----- */


/* ----- ----- ----- impl #1 ----- ----- ----- */
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        ( 
            self.width > other.width && // 'other' is just a inst of Rect
            self.height > other.height  // ... just don't overthink of it 
        )
    }

    fn square(size: u32) -> Rect {
        /*
            This one does NOT take 'self' as the 1st param. 

            It was called "associated functions". 
                Also, it was called 'static method' in other prog-langs. 

            How do ya call it?
                appeared before     String::from
                call ur own func	STRUCT::Function(param)
        */
        Rect {width: size, height: size}
    }
}
/* ----- ----- ----- impl EOF ----- ----- ----- */


/* ----- ----- ----- impl #2 ----- ----- ----- */

impl Rect {
    /* 
        'Each struct is allowed to have multiple impl blocks'. 
            -> So it's the same to 
            -> that defining these inside the initial one 
    */
    fn greeting() {
        println!("{}", "Hallo! ich bin Julia.");
    }
}

impl Rect {
    /*
        Ah.. for normal methods, this doesn't make any sense ... 
        
        But, "we" will see a case 
            in which this syntax is useful in Chapter 10 
    */
    fn goodbye() {
        println!("{}", "Auf Wiedersehen.")
    }
}

/* ----- ----- ----- impl EOF ----- ----- ----- */