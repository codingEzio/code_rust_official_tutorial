use std::io;

fn main() {
    /* Do using the 'cargo check' all along! XD. */

    println!("Guess the num!");

    println!("Please input ur guess.");

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
            Typing 'use std::io::stdin'
            Invoke 'std::io::stdin' is still ACCEPTED.

        Let's break down the rest of them. 
        i.e. 
            .read_line()   
                => &mut     mutablize the variable 'guess' (it's a ref.) 
                => guess    #TODO explain later (rust-18e-Chapter04)
            
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
        .expect("Failed to read line");  // (hardly) optional 

    println!("You guessed: {}", guess);
}
