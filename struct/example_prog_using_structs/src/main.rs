#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    
    cal_mul_basic_way();
    cal_mul_using_tuple();
    cal_mul_using_strcuts();

    add_funci_with_de_trait();
}


/* ---- ---- ---- Territory of Structs ---- ---- ---- */
struct Rect {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct RectWithDebug {
    width: u32,
    height: u32,
}
/* ---- ---- ---- Territory of Structs ---- ---- ---- */ 


fn cal_mul_basic_way() {
    let ln = 25;
    let wd = 40;

    println!("- {}", _helper(ln, wd));

    fn _helper(a: u32, b: u32) -> u32 {
        a * b 
    } 
}


fn cal_mul_using_tuple() {
    let rect = (25, 40);

    println!("-- {}", _helper(rect));

    fn _helper(dimen: (u32, u32)) -> u32 {
        dimen.0 * dimen.1  // it's index! 
    } 
}


fn cal_mul_using_strcuts() {

    /*
        Normally, this should be outside the main func 
            i.e. splitting code and data (thus the same level)
    */

    let a_rect = Rect { width: 25, height: 40 };

    println!("--- {}", _helper(&a_rect));

    /*
        Just a reminder
            => rect     param
            => &Rect    struct's ref  (immutable, btw)
    */
    fn _helper(rect: &Rect) -> u32 {
        rect.width * rect.height  // so much better than tuple 
    }
}

fn add_funci_with_de_trait() {
    /* 
        By default, ya won't be 
            able to "print" the types that is NOT primitive.

        Why? (quote of related-expl)
            If u wanna "print" the '5', that's CAN ONLY BE '5'
            
        But for structs & else, 
            => 'there are more display possibilities'
                -> want a comma or not?
                -> include the brackets? 
                -> should all field be shown?  
                -> ... 
            
            => Rust just NOT trying to guess it 
                -> yet also the 'struct' does NOT impl it. 
    */

    let a_rect = RectWithDebug {width: 25, height: 40};

    /*
        Wanna "print" struct thing? 
            => struct   add `#[derive(Debug)]` above
            => print    with `{:?}`

        Note: 
            It's debug info, not exactly the "print" we're usin'.
    */
    println!("Rect1 is a inst of  {:?}", a_rect);
    println!("Rect1 is a inst of  {:#?}", a_rect);
}