fn main() {
    /* Code as notes (splited into functions) here. */

    slice_basic_usage();

    detect_first_space_ie_word();
    detect_first_space_ie_word_then_return();

    string_slices_as_param();

    other_slices_basic();
}

// ----- ----- ----- ----- --- FIRST --- ----- ----- ----- -----
fn detect_first_space_ie_word() {
    /* TAG: main */

    let word                = String::from("ab cd");
    let word_without_space  = String::from("abcd");

    println!(
        "| {}\n| {}",
        return_index_of_first_space(&word),
        return_index_of_first_space(&word_without_space)
    );
}

fn return_index_of_first_space(s: &String) -> usize {
    /* TAG: helper */

    let bytes = s.as_bytes();  // int <=> char

    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return idx;
        }
    }

    s.len()
}
// ----- ----- ----- ----- --- FIRST --- ----- ----- ----- -----


// ----- ----- ----- ----- --- SECOND --- ----- ----- ----- -----
fn slice_basic_usage() {
    /* TAG: main */

    let a_string = String::from("hey,golang");

    /* 
        Internally, the slice stores 
            => the starting position    1st byte of the pointer 
            => the length of slice      = (end - start)
    */
    let grt = &a_string[..=2];  // => hey
    let obj = &a_string[4..];   // => golang

    let usin_var_as_idx = a_string.len();

    println!("|| {}, {}",   grt, obj);
    println!("|| {}",       &a_string[4..usin_var_as_idx]);
}
// ----- ----- ----- ----- --- SECOND --- ----- ----- ----- -----


// ----- ----- ----- ----- --- THIRD --- ----- ----- ----- -----
fn detect_first_space_ie_word_then_return() {
    /* TAG: main */

    let word                = String::from("ab cd");
    let word_without_space  = String::from("abcd");

    println!(
        "||| {}\n||| {}",
        combine_prev2func_return_1st_word(&word),
        combine_prev2func_return_1st_word(&word_without_space)
    );
}

fn combine_prev2func_return_1st_word(s: &str) -> &str {
    /* TAG: helper */

    /* 
        There's basically two cases: 
            Find a space    ->  FROM zero To that-idx   thus first word
            Didn't find it  ->  FROM zero TO the-end    equiv to one word
    */

    let bytes = s.as_bytes();

    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..idx];  // 0..idx (no idx), exactly wat-we-need
        }
    }

    &s[..] 
}
// ----- ----- ----- ----- --- THIRD --- ----- ----- ----- -----


// ----- ----- ----- ----- --- FOURTH --- ----- ----- ----- -----
fn string_slices_as_param() {
    /* TAG: main */

    /* 
        These are notes that not categorized well,
            -> my mind is such a FUSS right now, sorry :) 

        String literals     being stored in the binary (quite ?definite)
        Slices              definite literals which < points to specifc addr >

        'hello'                 -> literal -> ?definite addr -> immutable 
        String::from("hello")   -> uncertain -> could be mut/imutable 

        That is, we can use 'slices' as "index". 
    */
    let aha_str = "world";

    println!(
        "|||| {}\n|||| {}",
        combine_prev2func_return_1st_word(&aha_str[..]),
        combine_prev2func_return_1st_word(&aha_str),
    )
}
// ----- ----- ----- ----- --- FOURTH --- ----- ----- ----- -----


// ----- ----- ----- ----- --- FIFTH --- ----- ----- ----- -----
fn other_slices() {
    // #TODO Vectors in Chapter08
}
// ----- ----- ----- ----- --- FIFTH --- ----- ----- ----- -----