#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    match_intro();
    match_nested_func_call();

    match_with_option();
    match_with_option_omit();

    match_udline_placeholder();
}

fn match_intro() {
    enum ZhCurrency {
        YiYuan,
        ShiYuan,
        WuShiYuan,
        YiBaiYuan,
    }

    fn curr_in_num(currency: ZhCurrency) -> u32 {
        /*
            A much better `if`
                if      =>  boolean only
                match   =>  any type (ala it's an expr)

            My understanding is that it is
                kinda an advanced `switch` with `break` (byDef)
        */

        /*
            Do keep in mind
                it's just a GODDAMN simple `if`!

            match ARG { //  val passed in
                X -> Y	//  if ARG == X, exec Y ( {Y} == Y )
            }           //  Y required a val being returned
        */
        match currency {
            ZhCurrency::YiYuan => 1,
            ZhCurrency::ShiYuan => 10,
            ZhCurrency::WuShiYuan => 50,

            ZhCurrency::YiBaiYuan => {
                println!("Ah, you lucky dog!");
                100
            }
        }
    }

    println!("{}", curr_in_num(ZhCurrency::YiBaiYuan))
}

fn match_nested_func_call() {
    #[derive(Debug)]
    enum UsState {
        NewYork,
        California,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // In my perspective,
    //  it's just a nested func call :P
    fn val_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quatr from {:?}.", state);
                25
            }
        }
    }

    val_in_cents(Coin::Quarter(UsState::NewYork));
}

fn match_with_option() {
    /*
        If I'd use this, the reasons could be
            the arg might be `None` (either none, or else)
    */

    fn plus_one(arg: Option<i32>) -> Option<i32> {
        /*
            It's just "regular func, regular match" (sort-of)

            The param being passed in must be `Option<TYPE>`.
            Yet the match only got two cases
                | None  ->  `None` (literally)
                | Some  ->  anything but `None`

            You can add multiple lines of code,
                => just like before (i.e. `{ CODE; RET_VAL }`)
        */

        match arg {
            None => None,
            Some(i) => {
                println!("Just testin the 'i'!");
                Some(i + 1)
            }
        }
    }

    let four = Some(5);
    let five = plus_one(four);

    let none = plus_one(None);

    // It's an enum (`Option`),
    //   so apparently you need to add ':?' for printing
    println!("{:?}, {:?}, {:?}", four, five, none);
}

fn match_with_option_omit() {
    fn helper(arg: Option<i32>) -> Option<i32> {
        /*
            Either of them MUST NOT be omitted :P
            cuz it's `Option` type (which the impl requires)
        */
        match arg {
            None => None,
            Some(i) => Some(i + 2),
        }
    }
}

fn match_udline_placeholder() {

    let valid_wkday = 5; 

    // Exec started from here :P
    match valid_wkday {
        1 => println!("yep"), 
        2 => println!("yep"), 
        3 => println!("yep"), 
        4 => println!("yep"), 
        5 => println!("yep"), 
        6 => println!("yep"), 
        7 => println!("yep"), 
        _ => println!("Not valid"), 
    }
}

