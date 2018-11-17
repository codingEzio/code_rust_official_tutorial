fn main() {

    /*
        Passing params just like Python (well, sort of).
    */
    let my_planet: String = "Jupyter".to_string();
    another_func(my_planet, 77);


    /*
        Statements
            => not returning values 
            => so like 'a = b = 5', the var 'a' will get nothing

        Expressions
            => Note the diff between < 5 > and < 5; > 
    */
    println!(
        "a = {}", 
         a = {let b = 1; b + 1});  // should be 2

    /*
        Two more examples about 'Expressions'. 
    */
    let f1 = func_return_num();
    let f2 = func_return_with_calc(100);

    println!("--- f1: {}, --- f2: {}", f1, f2);
}

fn another_func(planet: String, planet_id: i32) {
    println!("Hello from {} #{}", planet, planet_id);
}

fn func_return_num() -> i32 {
    100
}

fn func_return_with_calc(x: i32) -> i32 {
    x + 5
}