fn main() {

    /*
        I found that 
            the error-msg in Rust is quite .. amazing.
            I couldn't help sayin "Damn!" each time I'm using it.

        Imagine here's a msg got 'rustc --explain E0384', or similar ones. 
            If u type it inside the terminal, 
            it literally got a few sentences which EXPLAINING it! 

        Also, when u run 'cargo run', 
            ya can add a param '--verbose' to get more details.

        ------------------------------ 

        Q: Why immutable by default? 
        A: Easy to track problems 
            -> esp. the "by default", literally. 
            -> that's HUGE if u think it's not a big deal
    */
    let mut x: i32 = 2;
    x = x.pow(10);

    println!("The value is\t: {}\n", x);

    /*
        Two things ya need to notice:
            => Annotating type is REQUIRED! 
            => The value can ONLY be a constant expr
                -> anything except that could NOT be used 

        Also, it is recommended (which I agree with, XD) that 
            => Define the vars as constants 
                -> like u do it in C 
                -> e.g. #DEFINE WIDTH 20
                -> esp. in the case that var is changed A LOT 
            
            => Then, if u wanna change a variable,
                -> all u need is to change the constant's value.
    */
    const MY_NAME: &'static str = "Alex";
    const FLS_AGE: i32          = 30;

    println!("My name is\t: {}", MY_NAME);
    println!("My age  is\t: {}", FLS_AGE);

    /*
        This is called <Variable-Shadowing>.
            detail: https://en.wikipedia.org/wiki/Variable_shadowing 
        
        It might able to be called as "?变量遮蔽" (in Chinese).

        It's quite different betweet 'mut' and the 'var-shadowing'
        Let me (author) demostrate for ya!
            
            suppose we got
                let sp4 = "    ";
                let sp4 = sp4.len(); 

            my thought is that
                -> suppose we need the str & its len,
                    yet we don't have to create a brand new variable. 
                
                -> The 'mut' could NOT achive this 
                    and with 'compile-time' error ... 
                    
                    #TODO expl-later (soon)
            
    */
    let var_shdow = "hello";
    let var_shdow = "hello world";

    println!("Greeting\t: {}", var_shdow);
}
