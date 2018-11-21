fn main() {
    /* Code as notes (splited into functions) here. */

    ref_basic_usage();
    ref_type_mutable_immutable();

    mutable_ref_bypass_limit();
    immutable_ref_must_stay_imtble();

    dangling_refs();
}


fn ref_basic_usage() {
    /*
        [ABBREVIATION]
            abt         about 
    */

    /*
        In short, reference does NOT <take ownership>. 
            | But they DO HAVE restrictions of their own :P

        What changed? 
            => arg we passed	->  &VARIABLE
            => func's param 	->  &TYPE     

        Note here 
            -> Apart from the ref, there's a opposite one 
            -> which was called 'deref' (using *), we'll take abt it later.
    */
    let s1     = String::from("Hmm");
    let s1_len = calc_len(&s1);     

    
    /*
        References are immutable by default. 
        
        Just keywords added actually 
            | var       let VAR         ->  let mut VAR 
            | arg       &VAR            ->  &mut VAR
            | param     param: &String  ->  param: &mut String 
    */
    let mut r1 = String::from("Hallo");
    add_greeting(&mut r1);

    println!("--- [{}], [{}]", s1, s1_len);
    println!("--- [{}]", r1);

}

/* Starting helper of: <ref_basic_usage> */
fn calc_len(s: &String) -> usize {
    s.len()
}
fn add_greeting(s: &mut String) {
    s.push_str(", wie geht es?");
}
/* Ending helper of: <ref_basic_usage> */


fn ref_type_mutable_immutable() {
    
    let mut ms  = String::from("Aha");
    let     ims = String::from("Woo");

    /* 
        Mutable ref could only be borrowed once. 
            | to be clear, 
            | this is valid only they're inside the same scope). 
    */
    let m = &mut ms; 
    
    // Immutable could be borrowed multiple times (no limit I assume). 
    let i1 = &ims;
    let i2 = &ims;
    let i3 = &ims;

    println!("--- [{}]"         , m);
    println!("--- [{}, {}, {}]" , i1, i2, i3);

}


fn mutable_ref_bypass_limit() {
    /*
        [ABBREVIATION]
            undef           undefined 
        
        [SYMBOL_IN_COMMENT]
            &               and 
            =>              become (became)
    */

    /*
        Kinda like the problems we want to solve before. 
    
        So .. why the heck that there is a LIMIT ?! 
        
        It helps to prevent <data races> at compile time, e.g. 
            -> Two-or-more pointers access the same data (& same time)
                => At lease one of them is bein' used to write to the data
                => and there's no mechanism bein' used to sync access to the data

            The <data races> might cause 
                | undef behaviour and can be difficult to diagnose & fix. 
                | So, Rust directly prevents u from drowning in such situations.
    */
    let us = String::from("Calm down.");
    

    /*
        But we could use tricks to bypass it 
            | well, sort of, the rule still appies! 
    */
    let mut haha = String::from("HaHa");

    {                           // ALL HAIL THE CURLY-BRACKETS! 
        let mt1 = &mut haha;
    }                           // 'mt1' goes out-of-scope here (& => invalid!)

    let mt2 = &mut haha;

}


fn immutable_ref_must_stay_imtble() {
    /*
        [ABBREVIATION]
            mtble           mutable 
            imtble          immutable 
    */

    /*  
        We cannot have a mutable ref 
            | which we have an immutable one. 

        Multiple imtble ref is fine, 
            -> but the mtble ref could 
            -> affect anyone else's readin' of the data.  
    */
    let mut _stimt = String::from("Wooo");

    let f1 = &_stimt;
    let f2 = &_stimt;
    // let f3 = &mut _stimt;

    println!("--- [{}, {}]" , f1, f2);
    // println!("--- [{}]" , f3);
}


fn dangling_refs() {

    /*
        Let's deconstruct these two funcs. 
        
        fn main() {
            /*
                Two steps happens here (both the first and the last).
                    -> Step[0]: func was invoked  (then [1], [2], [3])
                    -> Step[4]: exec finished, go back for assign 

                The last step was the problem
                    | after the step[3], the 's' have already been invalid 
                    
                    | so when you trying assign the val (which is the step[4])
                        -> what u assign is actually a <null pointer>,
                        -> which is a big deal (and bad!), 
                        
                        -> what u should do is directly return it. 
            */
            let ref_to_nothin = dangle_it();    
        }

        fn dangle_it() -> &String {             
            let s = String::from("Yo");         // Step[1]: 's' was created 

            &s                                  // Step[2]: return its ref
        }                                       // Step[3]: the 's' => invalid
    */
    
    let ref_to_thing = no_dangle();

    println!("--- [{}]", ref_to_thing);
}

/* Starting helper of: <dangling_refs> */
fn no_dangle() -> String {
    let s = String::from("Hello");

    s
}
/* Ending helper of: <dangling_refs> */