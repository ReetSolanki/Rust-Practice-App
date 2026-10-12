// Closures aka anonymous functions can capture variables from their surrounding scope, without passing them as an argument
struct Person {
    first_name: String,
    last_name: String,
}

pub fn test_closures() {
    let add = |x: i32, y: i32| {
        println!("x: {} y: {}", x, y);
        x + y
    };
    let result: i32 = add(7, -18);
    let print_result = |x: i32| println!("The result is: {}", (result + x));
    print_result(32);

    let mut p1: Person = Person {
        first_name: "Reet".to_string(),
        last_name: "Solanki".to_string(),
    };
    let mut change_name = |new_last_name: &str| p1.last_name = new_last_name.to_string();
    change_name("Yadav");
    change_name("Joshi");
    println!("{} {}", p1.first_name, p1.last_name);
}
