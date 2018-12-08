#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    vec_basics();
    vec_with_enum();
}

fn vec_basics() {
    /* 
        Same type & next-to each other in mem :P
    */

    // ----- ----- Create ----- -----

    // <?> could hold any type
    let v_container: Vec<i32> = Vec::new();
    let v_give_vals: Vec<i32> = vec![1, 3, 5];

    // Rust could infer its type 
    let v_in_realcase = vec![1, 3, 5];


    // ----- ----- Update ----- -----

    let mut v_up = Vec::new();

    v_up.push(20);
    v_up.push(10);
    
    v_up.pop();


    // ----- ----- Access ----- -----

    let sec: &i32           = &v_up[2];
    let sec: Option<&i32>   = v_up.get(2);

    // What if it goes out-of-bound? 
    //  &V[idx]     =>  panic
    //  .get(idx)   =>  None 
    assert_eq!(v_up.get(50), None); 


    // ----- ----- Ownership ----- ----- 

    // Though we only ref the 1st element, 
    //  the ownership rule still need to be used (design pps)
    let mut von = vec![11, 22];
    let first = &von[0];

    // The rule still applies :P 
    von.push(30);   

    // Oh, the two vars: von, first 
    //  Unless you're trying to use the imt ref (`first`)
    //  It won't raiser error instantly (von's safe, for now)
    println!("{:?}", von);


    // ----- ----- Iterating ----- ----- 

    // slightly diff from imt & mt refs 
    let vit         = vec![3, 33, 333];
    let mut mvit    = vec![1, 2, 3];

    for v in &vit {
        println!("-> {}", v);
    }

    // Unless you're trying to modify sth, 
    //  the iterating ways for both ARE the same (no &mut)
    for m in &mut mvit {
        *m += 10;
        println!("{}", m);
    }

    println!("----- ----- ----- ");
}


fn vec_with_enum() {
    /* 
        Storing elements of a different type :> 

        Enum contains DIFFERENT types 
        Enum as ONE type (for vector) :P 
    */

    // Kinda a 'hack' from my perspective XD
    enum SpeardcheetCell {
        Int(i32),
        Float(f64),
        Text(String), 
    }

    let row = vec![
        SpeardcheetCell::Int(3),
        SpeardcheetCell::Float(3.5),
        SpeardcheetCell::Text(String::from("Ich bin alex.")),
    ];

    
}