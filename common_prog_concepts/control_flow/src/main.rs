fn main() {

    // if-else -> Basic 
    simple_if_else();
    cond_mustbe_bool();

    // if-else -> Var-Assign 
    let_if_else_stmt();

    // loop-while-for -> Playin-with-Data 
    loop_using_break();
    while_with_cond();
    for_with_array_and_range();

}

fn simple_if_else() {
    let status = "Morning";

    if status == "Morning" {
        println!("Guten Morgen.");
    } else if status == "Evening" {
        println!("Guten Abend.");
    } else if status == "LateNight" {
        println!("Guten Nacht.");
    } else {
        println!("Guten Tag.");
    }
}

fn cond_mustbe_bool() {
    if true {
        println!("A bool is a must. Expr. like `if 3` is WRONG.");
    }
}

fn let_if_else_stmt() {

    /*
        Much stronger than we have in Python. 
            i.e. 'if-else-var-assign' done in Rustaceans' way. XD.

        One line or multiple lines are both FINE. 
        
        There's one thing ya need to notice 
            => The expr inside the 'if else' MUST be the same type 
    */
    let cond = true; 
    let cond_lang = String::from("German");
    
    let status = if cond { "day" } else { "night" };  
    
    let greeting = if cond_lang == "German" {
        let word = String::from("Guten Tag!");  
        word
    } else {
        let word = String::from("日安 !");
        word
    };

    println!("--- Status: {}", status);
    println!("--- Greet : {}", greeting);

}


fn loop_using_break() {

    /*
        Simple `loop { ... }` is a bloody infinite loop. 

        It could be also used in a var-assign statement :P 
    */

    let mut cnt = 0;

    let res = loop {
        cnt += 1;           // incr to 10 

        if cnt == 10 {
            break cnt * 2;  // then go into this -> break -> 10 * 2 
        }
    };

    println!("res = {}", res);
    // assert_eq!(res, 20);

}

fn while_with_cond() {

    /*
        Simple condition 
    */
    let mut num = 3;

    while num != 0 {
        print!("--- {}! ", num);

        num -= 1;
    }
    println!();


    /*
        Looping an array 
    */
    let     mth = ["October", "November", "December"];
    let mut idx = 0;

    while idx < mth.len() {
        println!("[{}]: {}", idx, mth[idx]);

        idx += 1;
    }

}

fn for_with_array_and_range() {

    /*  
        Iterating an array 
    */
    let fun = ["Rust tutorial", "Python stdlib", "Blog post"];

    for elem in fun.iter() {
        println!("Element: {}", elem);
    }


    /*
        Function `range` done in Rustaceans' way 
    */
    for even_num in (1 .. 9).rev() {
        
        if even_num % 2 == 0 {
            println!("Hmm: {}", even_num);  // should be '8, 6, 4, 2'
        }
    }

}