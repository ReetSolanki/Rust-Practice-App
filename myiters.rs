//In Rust, an iterator is a trait that allows you to process a sequence of elements one at a time.
#[allow(unused)] // Supresses warnings for unsed variables and functions

pub fn test_rust_iterators() {
    let fruit_list: Vec<&str> = vec!["Strawberry", "Blueberry", "Mango", "Orange", "Apple"];

    let nut_list: Vec<&str> = vec![ "Walnut", "Almonds", "Pecans", "Brazil Nuts"];

    let mut fruit_iter = fruit_list.iter();

    fruit_iter.next(); // Skips the currt value and moves to next
    fruit_iter.next(); 
    let item01: Option<&&str> = fruit_iter.next();
    println!("First item in Iterator is {}", item01.unwrap());

    let agregate_food = fruit_list.iter().chain(&nut_list); //Chains the two vectors one after other fruit_list then nut_list

    let all_food: Vec<&&str> = agregate_food.clone().collect(); // Converts iterator back to vec type, clone to not delete the agregate_food iterator
    
    for food in agregate_food{
        println!("{}", food);
    }

    let mut fruit_list_string = fruit_list.iter().map(|e| String ::from(*e)); //Converts &&str to String
    let new_fruit = fruit_list_string.map(|mut e| {e.push_str(" fruit"); return e;}); // Adding fruit at the end of each element of fruit_list and storing in new_fruit

    new_fruit.clone().for_each(|e| println!("{}", e));

    println!("Last fruit is {}", new_fruit.clone().last().unwrap());

    let mut stepby = new_fruit.clone().step_by(2);
    println!("Step {}", stepby.next().unwrap());
    println!("Step {}", stepby.next().unwrap());
    println!("Step {}", stepby.next().unwrap());

    let first_names = vec!["Trevor", "Shannon", "James", "Tahsa"];
    let first_names_string = first_names.iter().map(|e| String::from(*e));
    
    let last_names = vec!["Jones", "Sullivan", "Tanner","Redman"];
    let last_names_string = last_names.iter().map(|e| String::from(*e));

    let full_names = first_names_string.zip(last_names_string);
    // full_names.for_each(|e| println!("{} {}", e.0, e.1));

    // for (index, value) in full_names.enumerate(){ // enumerate converts it into tuple
    //     println!("Index: {0} Value: {1} {2}", index, value.0, value.1);
    // }

    //full_names.skip(2).take(1).for_each(|e| println!("Did not skip {}", e.0)); // Skips 1 element, take takes one element and skips the rest

    let foods = vec![("potatoes", 10), ("strawberries", 23), ("burgers", 31)];
    let food_quantity = foods.clone().iter().fold(0u32, |mut a, mut e| a + e.1);
    println!("{}", food_quantity);

    //Peak doesn;t pass the iterator to next item, it just shows you the current item
    let food_iter = foods.iter();
    let mut mypeekable = food_iter.peekable();
    mypeekable.next();
    let food = mypeekable.peek();
    println!("Peaking at {}", food.unwrap().0);
}