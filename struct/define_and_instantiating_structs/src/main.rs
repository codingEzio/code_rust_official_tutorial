#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {

    // ----------- Part 01 --------------
    struct User {
        name:   String,
        email:  String,
        dead:   bool,
    }

    // order isn't required
    let mut user001 = User {
        email:  String::from("numberOne@example.com"),
        name:   String::from("John Doe"),
        dead:   false,
    };

    // access, modify
    user001.name;
    user001.name = String::from("Elizabeth Olsen");

    // as return val
    fn build_user(email: String, name: String) -> User {
        User {
            email:  email,
            name:   name,
            dead:   true,
        }
    }

    let another_user = build_user(
        String::from("aol@exp.com"), 
        String::from("Alisha")
    );

    another_user.name;
    another_user.email;

    // same name as param 
    fn build_user_shorthand(email: String, name: String) -> User {
        User {
            name,           // instead of 'name: name'
            email,
            dead: false,
        }
    }

    // using other instances' attrs 
    let user002 = User {
        email: String::from("aa@bb.com"),
        name: String::from("Eliot Alderson"),
        dead: user001.dead  
    };

    // a much better way using other inst' attrs 
    let user003 = User {
        email: String::from("cc@dd.com"),
        ..user002  // all the remaining attrs! 
    };

    // ----------- Part 02 --------------

    /*
        Tuple structs which only got types 
            -> tuple got a name 
            -> struct only got type 
    */
    struct ColorRGB(i32, i32, i32);
    struct AxisXY(i32, i32);

    let rand_color = ColorRGB(255, 255, 255);
    let rand_point = AxisXY(10, -10);

    let color_r = rand_color.0;  // aha, still 'dot' notation

    // ----------- Part 03 --------------
 
    /*
        #TODO More on Chapter 10
            1. unit-like structs without any fields :P
            2. lifetime about structs 
    */

}
