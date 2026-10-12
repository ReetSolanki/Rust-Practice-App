use std::thread;

struct Person {
    first_name: String,
}

pub fn test_thread_variable () {
    let age = 34;
    let person01 = Person{first_name: String::from("Trevor")};

    let print_age = || { // Move is used to take the reference of variable and use it into the child scope(needs not to be done if this closure is not a part of thread)
        println!("Your age is {age}");
        println!("Your name is {}", &person01.first_name);  // we removed move and created an immutable reference 
    };

    thread::scope(|scope| {  // Scope is used to manage ownership of the struct person01, as they 
        scope.spawn(print_age);
    });
    // let _result = thread::spawn(print_age).join(); // Join to wait until print_age has finished executing
    
    println!("Your age is {age}");
    println!("Your name is {}", person01.first_name);

    println!("Finished printing age");
}