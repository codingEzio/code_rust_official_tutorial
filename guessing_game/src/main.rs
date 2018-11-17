/*
    Even wonder how to use it?
        -> Type 'cargo doc --open' 
        -> then it'll build docs locally for you. 
    
    Note
        The content depends on what u declared in the .toml
*/
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    /* Do using the 'cargo check' all along! XD. */

    println!("Guess the num!");

    /*
        Rust could infer that it's a number.
        The type defaults to 'i32' (while we got other types)
    */
    let secret_num = rand::thread_rng().gen_range(1, 21);
    
    // Ya better comment this line, XD
    // println!("The secret num is: {}.", secret_num);
    
    loop {

        println!("\nPlease input ur guess.");

        /* 
            Var are immutable by default. 
            i.e. 
                let     foo = 5  // immutable 
                let mut foo = 5  //   mutable 

            Also, about the right-handed expr
            i.e. 
                String      A string type (there's 3 more)
                ::new       A static method of 'String' 

            In short, 'String::new()'
                => Create a instance of 'String' type 
                => then calling the String's "new" (static method) 
        */
        let mut guess = String::new();  

        /*
            Similar to the 'import' stmt in other langs,
            e.g. 
                use std;            =>  std::io::stdin
                use std::io         =>       io::stdin 
                use std::io::stdin  =>           stdin

            Verbose is accepted. 
                Having 'use std::io::stdin',
                Invoke 'std::io::stdin' is still ACCEPTED.

            Let's break down the rest of them. 
            i.e. 
                .read_line()   
                    => &mut     mutablize the var 'guess'
                    => guess    #TODO explain later (rust-18e-cp04)
                
                .expect()
                    => like a::b().c()
                        return a val which its type is "io::Result"
                        and itself is an 'Enum' contains 'Ok' and 'Err'.
                    
                    => So what u actually called 
                        is this one 'io::Result().expect()'
                    
                    => Is it 'error handling' ? 
                        not really, we'll see this later
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");  // (hardly) optional 

        /* 
            Convert its type ('guess') before comparing. (? reassign)
        
            <later-added>
                'match' stmt for variable assignment! 
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) =>  num,   

            /* 
                [0] The '_' is similar to 'except Exception'
                [1] The 'continue' let the code go into the next loop
            */ 
            Err(_)  =>  continue,   
        };

        println!("\nYou guessed: {}", guess);

        // Freaking elegant!
        match guess.cmp(&secret_num) {
            Ordering::Less      => println!("-> Too small"),
            Ordering::Greater   => println!("-> Too big"),
            Ordering::Equal     => {
                println!("-> You win!");
                break;
            }
        };

    }
}
