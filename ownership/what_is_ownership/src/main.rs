fn main() {

    /* Code as notes (splited into functions) here. */ 

    foreword();
    why_ownership();
    what_is_ownership();

    ownership_for_func_param();
    ownership_for_func_return();
    ownership_for_func_return_multi_val();

    /* 
        The end. 
        
        We're gonna introduce a new concept, 
            | which was called 'reference' (exists in another project folder)
    */
}

fn foreword() {
    /* 
        [TOPIC]
            commn-ds-heap-stack
    */

    /*
        It's kinda new concept, yet easy to understand. 

        Let's review the 'stack' & 'heap' data structure. 
            => Stack: LIFO  Last-In, First-Out
            => Heap : FIFO  First-In, Last-Out

        For 'stack', 
            -> <add>    pushing onto	
            -> <del>    poping  off	    ('stack' opt on top ONLY)

            Cuz it opt on top ONLY, 
                => fast     cuz u don't have search elsewhere
                => fast     fixed size (ya only add/del on top(end))

        For 'heap', 
            | data with a size 
            | unknown at compile time (or'll change)

            Unlike 'stack', ya had to 
                (pre) allocate (not) fixed space (toPrepare) for it. 

            The OS 'finds' somewhere that is big enough 
                => mark it as 'being in use'
                => returns a pointer (i.e. memory-addr)

            You CAN store the pointer on the 'stack', 
                ya still need to follow the pointer (which is on 'heap')

            Conceptually, reason for why 'heap' is slower is that 
                => allocating   Unlike 'stack', ya just put it on the top. 
                => accessing    Same idea. Plus ya need 'from pointer to data'.

        In short,
            => the 'heap' is slower than access data on the 'stack'
            => (might not be true for contem-processor, but, well..)

        So .. why we need 'ownership'? 

        Let's taking for an example. 
            -> Ya called a func, val passed into it (might pt to heap)
            -> Local var psh-on-stack | over | local var pp-off-stack

        ... Hmm? 

        Here's the concerns. 
            => Keeping track of what parts of code're using what data 
            => Minimizing the amount of duplicated data on the 'heap'
            => Cleaning up unused data on it (e.g. mem-leak if go bad)

        'Ownership' is here for help! 
    */

    println!("Hello, world!");
}

fn why_ownership() {
    /* 
        [TOPIC]
            mem-cleanup & related issues  
    */

    /* 
        Rules of ownership
            => value <=> variable, i.e. 'a=5', a is the owner of 5. 
            => one owner at a time (e.g. 'a=5; b=a;' -> 'a' is invalid)
            => out of scople (i.e. {}) -> its val was dropped 

        Well, in short, 
            Clean up /efficiently/ yet /intelligently/  (my-own-thoughts)
    */


    /*
        The type of variable 'sf' is "String",
            and the one was pushed is a "literal" (i.e. "!").

        Literals are faster. 
            | cuz it's DEFINITE, HARD-CODED, and IMMUTABLE. 
            | but .. we don't always know everythin' (i.e. users' input)

        So, we still need the 'String' type :P 
            | The type requests the memory it needs, 
            | which is pretty common in prog languages. 
    */
    let mut msf = String::from("Aha"); 
    msf.push_str("!"); 


    /*
        Well, 'allocate' is just the first step. 
            | Then, we need to think about how to 'free' the mem.
        
        Since Rust don't have a GC,  
            | which keeps track and 
            | cleans up mem that isn't being used anymore 

        So .. we need to find a way!  
        
        Rust took a different path, 
            -> the mem is automatically returned 
            -> once <the var that owns it> goes out of scope (i.e. {})

        Down below is (just) a simple example, 
            | which we might (often) meet 
            | a <not-that-intuitive> and <more compli> situations.
    */

    {   // STATUS: doesn't exist 
        
        let dp_thkin = String::from("Hmm");  // living between { 😅 }

    }   // STATUS: boom, invalid!  (called 'drop' by Rust)


    /*
        Simple vals (i.e. int here) 
            | with a known, fixed size,  
            | which could be push onto the 'stack'. 
            
        There's not much for us to optimize. 
    */
    let _i1 = 5; 
    let _i2 = _i1; 


    /*
        What did the 's1' do? 
            | String type => allocate enough space 
            | then store the 'yo ...' onto the 'heap'

            Three things were related to this: 
                => Pointer      starting addr on memory 
                => Length       how much mem it was using (in bytes)
                => Capacity     the total mem was allocated  

        When we assigning 's1' to 's2', 
            -> we're actually only 
                => copying (i.e. assgin) the 
                => pointer (i.e. mem-addr) to 's2'. 

        So .. when the variables goes out scope, 
            | Rust willl calls the 'drop' and cleans up the heap mem. 
        
        Bang! That's where's the problem occurs :p 

        Since the 's2' & 's1' both points to the same addr 
            => the same memory might be "freed twice" 
            => which could result in mem corruption, even security issues. 

        Now we came to the core concept of 'Ownership' !! 
    */

    let s1 = String::from("Yo, wut's up");
    // let s2 = _s1; 

}

fn what_is_ownership() {
    /* 
        [TOPIC]
            _  

        [ABBREVIATION]
            rntm            runtime          
            perfmce	        performance
            compl-error     compile error
            atmtcly	        automatically
            IMO	            In My Opinion
            bk18e	        Rust Prog Lang: 2018 edition 
    */


    /*
        Code like
            >> let w1 = String::from("hoo");  
            >> let w2 = w1;
        
        For mem-safety reasons (& else), 
            -> Rust choose (IMO) to <invalidate> the var 'w1' directly 
            -> so we don't have 'free' anythin when 'w1' goes out-of-scope

            As we've already invalidate the 'w1', 
                | therefore, at lease for the 
                | 'free-same-mem-twice', we don't need to care it anymore

        What Rust does is that 
            -> directly prevents u from using that 'var' (=> compl-error)
            -> also, what it does is NOT the same as "shallow copy". 

            What's the diff? 
                => Shallow      copy the [pnter, capac, len] of w1 to w2
                => RustStyle    1st part's same. but 'w1' was bein' invalidated

            In official (bk18e) terms, 
                | this operation is called < 'w1' was moved into 'w2' >

        Furthermore, 
            | there's a design choice that's implied by this
            | i.e. < Rust'll never (atmtcly) create 'deep-copies' of ur data. >

            Therefore, any 'automatic copying' 
                | could be assured to be <inexpensive> in-trms-of rntm perfmce.
    */


    /*
        Explicitly claiming ya wanna deeply copy that data 
            | which is to copy the data on the 'heap', 
            | rather than the default that invalidate the previous "owner".
    */
    let c1 = String::from("mozilla");
    let c2 = c1.clone();

    println!("--- [{}], [{}]", c1, c2);


    /* 
        Let's review the stack-only data,
            | which we've talked before (i.e. integer). 

            Here's the list of them: 
                => all the integer types    e.g. u32
                => all the float types      e.g. f64
                => boolean type             i.e. true, false 
                => char type 
                => tuples                   which only contains prev mentioned
        
        Features about this kind of data: 
            => fixed
            => quick-to-make 
            => have a known size at compl time  (for str, which is not)

            So, it could be stored entirely on the stack 
                -> as features above, there's NO NEED to 
                -> do some 'invalidate' kind of thing (i.e. different vars)

        For this, we need to bring a new concept called 'trait'
            -> It's kinda like "magic methods" in Python (dig more later)

        Here's some quotes from the book 
            -> put a 'Copy' trait like int that're stored on 'stack'
            
            -> If a type got a 'Copy' trait 
                | an older var is still usable after assignment 
            
            -> but if the type already got a 'Drop' trait 
                | we actually cannot a 'Copy' on it (which is ?conflict)
    */
    let x = 5;
    let y = x;  

    println!("--- [{}], [{}]", x, y);

}


fn ownership_for_func_param() {

    /*
        Ownership was transferred by operations. 
            | True for assignment, it does for function as well. 

        Down below I'll demostrate (sort of)
            | the lifecycles of the variable 'a_string' 
    */

    let a_string = String::from("assassin");    // [0] create 
    let an_int = 5; 

    take_ownership(a_string);                   // [1] passing  
    make_copy(an_int);

                                                // [5] 'a_string' no longer exists
}

/* Starting of <ownership_for_func_param> */
fn take_ownership(some_str: String) {           // [2] came in  
    println!("--- [{}]", some_str);             // [3] using it 
}                                               // [4] mem's freed (`drop` called)

fn make_copy(some_int: i32) {
    println!("--- [{}]", some_int);
}
/* Ending of <ownership_for_func_param> */


fn ownership_for_func_return() {

    let g = give_ownership();               // func => here 

    let t1 = String::from("Ha!");  
    let t2 = take_and_give_back(t1);        // above => func => here 
}

/* Starting of <ownership_for_func_return> */
fn give_ownership() -> String {
    let some_str = String::from("Hallo, ich bin Karl.");
    some_str
}

fn take_and_give_back(a_str: String) -> String {
    a_str
}
/* Ending of <ownership_for_func_return> */


fn ownership_for_func_return_multi_val() {
    /* Same mechanics as the prev function. */

    let s1        = String::from("Woo");
    let (s2, len) = calc_len(s1);

    println!("--- [{}] [{}]", s2, len); 
}

/* Starting of <ownership_for_func_return_multi_val> */
fn calc_len(s: String) -> (String, usize) {
    let length = s.len(); 

    (s, length)
}
/* Ending of <ownership_for_func_return_multi_val> */